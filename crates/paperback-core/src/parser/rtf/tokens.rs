//! Walking the lexed RTF token stream into a [`DocumentBuffer`]: tracking group nesting and
//! formatting scope (RTF reverts bold/italic/underline to their pre-group state on `}`),
//! resolving `\uN` surrogate pairs, skipping ignorable destinations and `\pict` groups, and
//! turning `HYPERLINK` field instructions into link markers.

use rtf_parser::tokens::{ControlWord, Property, Token};

use crate::document::{DocumentBuffer, Marker, MarkerType};

struct PendingLink {
	url: String,
	start_position: usize,
}

/// Applies a formatting toggle for a single marker kind, recording spans as they
/// open and close. No-op when the requested state already matches the current one
/// (handles redundant toggles and group-close reverts that don't change this kind).
/// Turning ON records the start position; turning OFF emits the span, guarded by
/// `position > s` to skip degenerate zero-length spans.
fn apply_format_toggle(
	on: &mut bool,
	start: &mut Option<usize>,
	want_on: bool,
	position: usize,
	kind: MarkerType,
	buffer: &mut DocumentBuffer,
) {
	if want_on == *on {
		return;
	}
	if want_on {
		*start = Some(position);
	} else if let Some(s) = start.take()
		&& position > s
	{
		buffer.add_marker(Marker::new(kind, s).with_length(position - s));
	}
	*on = want_on;
}

#[allow(clippy::too_many_lines)]
pub(super) fn extract_content_from_tokens(tokens: &[Token]) -> DocumentBuffer {
	let mut buffer = DocumentBuffer::new();
	let mut in_header = true;
	let mut pending_high_surrogate: Option<u16> = None;
	let mut pending_link: Option<PendingLink> = None;
	let mut depth: i32 = 0;
	let mut skip_until_depth: Option<i32> = None;
	let mut bold_on = false;
	let mut italic_on = false;
	let mut underline_on = false;
	let mut bold_start: Option<usize> = None;
	let mut italic_start: Option<usize> = None;
	let mut underline_start: Option<usize> = None;
	let mut format_stack: Vec<(bool, bool, bool)> = Vec::new();
	for token in tokens {
		// Depth tracking for group nesting (needed for IgnorableDestination skipping).
		match token {
			Token::OpeningBracket => {
				depth += 1;
				// Snapshot formatting state so a group close reverts to it (RTF group scoping).
				// Push/pop happen unconditionally to stay in lockstep with `depth`.
				format_stack.push((bold_on, italic_on, underline_on));
			}
			Token::ClosingBracket => {
				depth -= 1;
				if let Some((want_bold, want_italic, want_underline)) = format_stack.pop() {
					let pos = buffer.current_position();
					apply_format_toggle(&mut bold_on, &mut bold_start, want_bold, pos, MarkerType::Bold, &mut buffer);
					apply_format_toggle(
						&mut italic_on,
						&mut italic_start,
						want_italic,
						pos,
						MarkerType::Italic,
						&mut buffer,
					);
					apply_format_toggle(
						&mut underline_on,
						&mut underline_start,
						want_underline,
						pos,
						MarkerType::Underline,
						&mut buffer,
					);
				}
				if skip_until_depth.is_some_and(|sd| depth <= sd) {
					skip_until_depth = None;
				}
				continue;
			}
			Token::IgnorableDestination => {
				// {\* \keyword content} — skip the entire enclosing group.
				skip_until_depth = Some(depth - 1);
				continue;
			}
			_ => {}
		}
		if skip_until_depth.is_some() {
			continue;
		}
		match token {
			Token::ControlSymbol((ctrl, property)) => {
				match ctrl {
					ControlWord::Pard => in_header = false,
					ControlWord::Par | ControlWord::Line => {
						if !in_header {
							buffer.append("\n");
						}
					}
					ControlWord::Tab => {
						if !in_header {
							buffer.append("\t");
						}
					}
					ControlWord::Unicode => {
						if !in_header && let Property::Value(code) = property {
							let code = if *code < 0 {
								let adjusted = i64::from(*code) + 0x10000;
								let adjusted = u64::try_from(adjusted).unwrap_or(0) & 0xFFFF;
								u16::try_from(adjusted).unwrap_or(0)
							} else {
								u16::try_from(*code).unwrap_or(0)
							};
							// Check for surrogate pairs
							if (0xD800..=0xDBFF).contains(&code) {
								pending_high_surrogate = Some(code);
							} else if (0xDC00..=0xDFFF).contains(&code) {
								if let Some(high) = pending_high_surrogate.take() {
									let codepoint =
										0x10000 + ((u32::from(high) - 0xD800) << 10) + (u32::from(code) - 0xDC00);
									if let Some(ch) = char::from_u32(codepoint) {
										buffer.append(&ch.to_string());
									}
								}
							} else {
								pending_high_surrogate = None;
								if let Some(ch) = char::from_u32(u32::from(code)) {
									buffer.append(&ch.to_string());
								}
							}
						}
					}
					ControlWord::Bold if !in_header => {
						let want_on = !matches!(property, Property::Value(0));
						apply_format_toggle(
							&mut bold_on,
							&mut bold_start,
							want_on,
							buffer.current_position(),
							MarkerType::Bold,
							&mut buffer,
						);
					}
					ControlWord::Italic if !in_header => {
						let want_on = !matches!(property, Property::Value(0));
						apply_format_toggle(
							&mut italic_on,
							&mut italic_start,
							want_on,
							buffer.current_position(),
							MarkerType::Italic,
							&mut buffer,
						);
					}
					ControlWord::Underline if !in_header => {
						let want_on = !matches!(property, Property::Value(0));
						apply_format_toggle(
							&mut underline_on,
							&mut underline_start,
							want_on,
							buffer.current_position(),
							MarkerType::Underline,
							&mut buffer,
						);
					}
					ControlWord::UnderlineNone if !in_header => {
						apply_format_toggle(
							&mut underline_on,
							&mut underline_start,
							false,
							buffer.current_position(),
							MarkerType::Underline,
							&mut buffer,
						);
					}
					// rtf_parser 0.4.3 promoted these from `Unknown(name)` to their own dedicated
					// variants; handled directly here since they no longer reach the `Unknown`
					// arm below (kept for older/unrecognized spellings, but should be unreachable
					// against the pinned rtf_parser version).
					ControlWord::RightSingleQuote if !in_header => buffer.append("\u{2019}"),
					ControlWord::LeftSingleQuote if !in_header => buffer.append("\u{2018}"),
					ControlWord::RightDoubleQuote if !in_header => buffer.append("\u{201D}"),
					ControlWord::LeftDoubleQuote if !in_header => buffer.append("\u{201C}"),
					ControlWord::Emdash if !in_header => buffer.append("\u{2014}"),
					ControlWord::Endash if !in_header => buffer.append("\u{2013}"),
					ControlWord::Bullet if !in_header => buffer.append("\u{2022}"),
					ControlWord::Unknown(name) if !in_header => match *name {
						r"\page" => {
							let ends_with_ws = buffer.content.chars().next_back().is_some_and(char::is_whitespace);
							if !ends_with_ws && !buffer.content.is_empty() {
								buffer.append(" ");
							}
							buffer.add_marker(Marker::new(MarkerType::PageBreak, buffer.current_position()));
						}
						r"\rquote" => buffer.append("\u{2019}"),
						r"\lquote" => buffer.append("\u{2018}"),
						r"\rdblquote" => buffer.append("\u{201D}"),
						r"\ldblquote" => buffer.append("\u{201C}"),
						r"\emdash" => buffer.append("\u{2014}"),
						r"\endash" => buffer.append("\u{2013}"),
						r"\pict" => skip_until_depth = Some(depth - 1),
						_ => {}
					},
					_ => {}
				}
			}
			Token::PlainText(text) => {
				if !in_header {
					if let Some(url) = text.strip_prefix("HYPERLINK ") {
						let url = url.trim().trim_matches('"').to_string();
						pending_link = Some(PendingLink { url, start_position: buffer.current_position() });
					} else if let Some(link) = pending_link.take() {
						let display_text = text.to_string();
						let text_len = display_text.chars().count();
						buffer.append(&display_text);
						buffer.add_marker(
							Marker::new(MarkerType::Link, link.start_position)
								.with_text(display_text)
								.with_reference(link.url)
								.with_length(text_len),
						);
					} else {
						buffer.append(text);
					}
				}
			}
			Token::CRLF if !in_header => {
				buffer.append("\n");
			}
			_ => {}
		}
	}
	// Defensive flush for malformed/truncated RTF with unbalanced braces. In a
	// well-formed document the outermost group close already reverted all three to
	// `false` (step 3), so this is a no-op in the common case.
	let final_pos = buffer.current_position();
	let bold_was_open = bold_on;
	let italic_was_open = italic_on;
	let underline_was_open = underline_on;
	apply_format_toggle(&mut bold_on, &mut bold_start, false, final_pos, MarkerType::Bold, &mut buffer);
	apply_format_toggle(&mut italic_on, &mut italic_start, false, final_pos, MarkerType::Italic, &mut buffer);
	apply_format_toggle(&mut underline_on, &mut underline_start, false, final_pos, MarkerType::Underline, &mut buffer);
	if bold_was_open || italic_was_open || underline_was_open {
		tracing::warn!(
			bold = bold_was_open,
			italic = italic_was_open,
			underline = underline_was_open,
			"rtf document had an unclosed formatting span at end of document, likely unbalanced braces"
		);
	}
	let trimmed = buffer.content.trim().to_string();
	let mut result = DocumentBuffer::with_content(trimmed);
	let leading_trim = buffer.content.len() - buffer.content.trim_start().len();
	for marker in buffer.markers {
		let adjusted_pos = marker.position.saturating_sub(leading_trim);
		result.add_marker(
			Marker::new(marker.mtype, adjusted_pos)
				.with_text(marker.text)
				.with_reference(marker.reference)
				.with_length(marker.length)
				.with_level(marker.level),
		);
	}
	let has_pages = result.markers.iter().any(|m| m.mtype == MarkerType::PageBreak);
	let has_start_page = result.markers.iter().any(|m| m.mtype == MarkerType::PageBreak && m.position == 0);
	if has_pages && !has_start_page {
		result.add_marker(Marker::new(MarkerType::PageBreak, 0));
	}
	result
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use rtf_parser::lexer::Lexer;

	use super::*;
	use crate::parser::rtf::escapes::normalize_escapes;

	#[test]
	fn extract_content_maps_quote_unknown_control_words_to_typographic_quotes() {
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::ControlSymbol((ControlWord::Unknown(r"\ldblquote"), Property::None)),
			Token::PlainText("ship"),
			Token::ControlSymbol((ControlWord::Unknown(r"\rquote"), Property::None)),
			Token::PlainText("s"),
			Token::ControlSymbol((ControlWord::Unknown(r"\rdblquote"), Property::None)),
			Token::PlainText(" and "),
			Token::ControlSymbol((ControlWord::Unknown(r"\lquote"), Property::None)),
			Token::PlainText("captain"),
			Token::ControlSymbol((ControlWord::Unknown(r"\rquote"), Property::None)),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "\u{201C}ship\u{2019}s\u{201D} and \u{2018}captain\u{2019}");
	}

	#[test]
	fn extract_content_preserves_line_and_tab_unknown_controls() {
		let rtf = r"{\rtf1\ansi\pard delay.\line \tab next}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "delay.\n\tnext");
	}

	#[test]
	fn extract_content_keeps_text_around_word_style_unicode_escapes() {
		// Word writes curly quotes as \uN followed by a bare `?` fallback. Because the
		// lexer ends a control word at the first space, every character between the
		// digits and that space used to be swallowed along with the escape.
		let rtf = "{\\rtf1\\ansi\\pard\t\n\\u8216?Yes!\\u8217? shrieked Salem Rews, quartermaster of his August \
		           Majesty\\u8217?s First Regiment. \\u8216?Give \\u8217?em hell!\\u8217?\\par}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(
			buffer.content,
			"\u{2018}Yes!\u{2019} shrieked Salem Rews, quartermaster of his August Majesty\u{2019}s First Regiment. \
			 \u{2018}Give \u{2019}em hell!\u{2019}"
		);
	}

	#[test]
	fn extract_content_keeps_literal_tab_indentation() {
		// This writer opens each paragraph with a literal tab rather than \tab.
		let rtf = "{\\rtf1\\ansi\\pard first\\par}{\t\nFor Mum and Dad, Couldn\\u8217?t have done it.\\par}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "first\n\tFor Mum and Dad, Couldn\u{2019}t have done it.");
	}

	#[test]
	fn extract_content_maps_page_control_to_marker_and_separator() {
		let rtf = r"{\rtf1\ansi\pard chapter one\page chapter two}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "chapter one chapter two");
		let page_markers: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).collect();
		assert_eq!(page_markers.len(), 2);
		assert_eq!(page_markers[0].position, "chapter one ".chars().count());
		assert_eq!(page_markers[1].position, 0);
	}

	#[test]
	fn extract_content_skips_pict_groups() {
		// A bare {\pict...} group (no leading \*) still carries binary image data as
		// hex text, which must not leak into the document body.
		let rtf = r"{\rtf1\ansi\pard before{\pict\wmetafile8\picw100\pich100 010009000003}after}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "beforeafter");
		assert!(!buffer.content.contains("010009000003"));
	}

	#[test]
	fn extract_content_skips_ignorable_destination_groups() {
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::PlainText("before"),
			Token::OpeningBracket,
			Token::IgnorableDestination,
			Token::PlainText("504b0304themedata"),
			Token::ClosingBracket,
			Token::PlainText("after"),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "beforeafter");
		assert!(!buffer.content.contains("504b0304"));
	}

	#[test]
	fn extract_content_handles_libreoffice_unicode_fallback_and_nbsp_symbols() {
		let rtf = r"{\rtf1\ansi\pard AGRAVANTE:\~ Pedro da Silva\par O Ju\u237\'edzo da Vara, pela decis\u227\'e3o e execu\u231\'e7\u227\'e3o contra a 2\u170\'aa executada\par}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert!(buffer.content.contains("AGRAVANTE:"));
		assert!(buffer.content.contains("Pedro da Silva"));
		assert!(buffer.content.contains("Juízo"));
		assert!(buffer.content.contains("decisão"));
		assert!(buffer.content.contains("execução"));
		assert!(buffer.content.contains("2ª executada"));
	}

	#[test]
	fn extract_content_maps_bold_toggle_to_marker() {
		// \b bold \b0 not-bold  → one Bold marker spanning exactly "bold ".
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::ControlSymbol((ControlWord::Bold, Property::None)),
			Token::PlainText("bold "),
			Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
			Token::PlainText("not-bold"),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "bold not-bold");
		let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
		assert_eq!(bold.len(), 1);
		assert_eq!(bold[0].position, 0);
		assert_eq!(bold[0].length, "bold ".chars().count());
	}

	#[test]
	fn extract_content_scopes_nested_group_formatting() {
		// \b bold {\i more} still-bold \b0
		// Bold spans the whole "bold more still-bold " (uninterrupted by the group);
		// Italic spans only "more" (opened and closed within the group).
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::ControlSymbol((ControlWord::Bold, Property::None)),
			Token::PlainText("bold "),
			Token::OpeningBracket,
			Token::ControlSymbol((ControlWord::Italic, Property::None)),
			Token::PlainText("more"),
			Token::ClosingBracket,
			Token::PlainText(" still-bold "),
			Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "bold more still-bold");

		let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
		assert_eq!(bold.len(), 1);
		assert_eq!(bold[0].position, 0);
		assert_eq!(bold[0].length, "bold more still-bold ".chars().count());

		let italic: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Italic).collect();
		assert_eq!(italic.len(), 1);
		assert_eq!(italic[0].position, "bold ".chars().count());
		assert_eq!(italic[0].length, "more".chars().count());
	}

	#[test]
	fn extract_content_maps_underline_off_via_ulnone() {
		// \ul under \ulnone plain → one Underline marker spanning "under ".
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::ControlSymbol((ControlWord::Underline, Property::None)),
			Token::PlainText("under "),
			Token::ControlSymbol((ControlWord::UnderlineNone, Property::None)),
			Token::PlainText("plain"),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "under plain");
		let underline: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Underline).collect();
		assert_eq!(underline.len(), 1);
		assert_eq!(underline[0].position, 0);
		assert_eq!(underline[0].length, "under ".chars().count());
	}

	#[test]
	fn extract_content_reverts_bold_on_group_close() {
		// \b before {\b0 middle} after \b0
		// Two separate Bold markers ("before " and " after "), "middle" in neither.
		let tokens = vec![
			Token::ControlSymbol((ControlWord::Pard, Property::None)),
			Token::ControlSymbol((ControlWord::Bold, Property::None)),
			Token::PlainText("before "),
			Token::OpeningBracket,
			Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
			Token::PlainText("middle"),
			Token::ClosingBracket,
			Token::PlainText(" after "),
			Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
		];
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "before middle after");

		let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
		assert_eq!(bold.len(), 2, "expected two Bold spans, got {bold:?}");
		// "before "
		assert_eq!(bold[0].position, 0);
		assert_eq!(bold[0].length, "before ".chars().count());
		// " after " starts right after "before middle"
		assert_eq!(bold[1].position, "before middle".chars().count());
		assert_eq!(bold[1].length, " after ".chars().count());
		// "middle" is covered by neither span.
		let middle_start = "before ".chars().count();
		let middle_end = "before middle".chars().count();
		for m in &bold {
			let span_end = m.position + m.length;
			assert!(
				m.position >= middle_end || span_end <= middle_start,
				"Bold span {m:?} should not overlap \"middle\""
			);
		}
	}

	#[test]
	fn extract_content_renders_bold_from_real_rtf_string() {
		// Round-trip through the real lexer to confirm \b / \b0 map to Bold markers.
		let rtf = r"{\rtf1\ansi\pard normal \b bold\b0  normal}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "normal bold normal");
		let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
		assert_eq!(bold.len(), 1);
		assert_eq!(bold[0].position, "normal ".chars().count());
		assert_eq!(bold[0].length, "bold".chars().count());
	}

	/// Round-trip through the real lexer (unlike
	/// `extract_content_maps_quote_unknown_control_words_to_typographic_quotes` above, which
	/// hand-builds `ControlWord::Unknown` tokens and so can't catch this): rtf_parser 0.4.3
	/// promoted `\rquote`/`\lquote`/`\rdblquote`/`\ldblquote`/`\emdash`/`\endash`/`\bullet` from
	/// `Unknown(name)` to their own dedicated `ControlWord` variants, which silently fell
	/// through the `_ => {}` catch-all until dedicated match arms were added for them.
	#[test]
	fn extract_content_maps_special_char_control_words_from_real_rtf_string() {
		let rtf = r"{\rtf1\ansi\pard Earth\rquote s \lquote quoted\rquote \ldblquote double\rdblquote em\emdash dash en\endash dash \bullet bullet}";
		let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(
			buffer.content,
			"Earth\u{2019}s \u{2018}quoted\u{2019}\u{201C}double\u{201D}em\u{2014}dash en\u{2013}dash \u{2022}bullet"
		);
	}
}
