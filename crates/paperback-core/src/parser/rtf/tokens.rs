use rtf_parser::tokens::{ControlWord, Property, Token};

use crate::document::{DocLen, DocPos, DocumentBuffer, Marker, MarkerType};

struct PendingLink {
	url: String,
	start_position: DocPos,
}

fn apply_format_toggle(
	on: &mut bool,
	start: &mut Option<DocPos>,
	want_on: bool,
	position: DocPos,
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

fn starts_body(token: &Token) -> bool {
	match token {
		Token::PlainText(text) => !text.trim().is_empty(),
		Token::ControlSymbol((ctrl, _)) => matches!(
			ctrl,
			ControlWord::Par | ControlWord::Line | ControlWord::Bold | ControlWord::Italic | ControlWord::Underline
		),
		_ => false,
	}
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
	let mut bold_start: Option<DocPos> = None;
	let mut italic_start: Option<DocPos> = None;
	let mut underline_start: Option<DocPos> = None;
	let mut format_stack: Vec<(bool, bool, bool)> = Vec::new();
	for token in tokens {
		match token {
			Token::OpeningBracket => {
				depth += 1;
				// RTF reverts formatting on group close; push and pop unconditionally to stay in lockstep with `depth`.
				format_stack.push((bold_on, italic_on, underline_on));
			}
			Token::ClosingBracket => {
				depth -= 1;
				if let Some((want_bold, want_italic, want_underline)) = format_stack.pop() {
					let pos = buffer.end();
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
				// Keep the outermost depth, so a nested destination (\picprop inside \pict inside \shppict) can't end the skip early when its own group closes.
				let candidate = depth - 1;
				skip_until_depth = Some(skip_until_depth.map_or(candidate, |sd| sd.min(candidate)));
				continue;
			}
			_ => {}
		}
		if skip_until_depth.is_some() {
			continue;
		}
		// The header tables are groups of their own, so text directly in the document's group is body even without \pard; WordPad's minimal files and many generators write text straight after the font table.
		if in_header && depth == 1 && starts_body(token) {
			in_header = false;
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
							buffer.end(),
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
							buffer.end(),
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
							buffer.end(),
							MarkerType::Underline,
							&mut buffer,
						);
					}
					ControlWord::UnderlineNone if !in_header => {
						apply_format_toggle(
							&mut underline_on,
							&mut underline_start,
							false,
							buffer.end(),
							MarkerType::Underline,
							&mut buffer,
						);
					}
					// rtf_parser 0.4.3 gave these their own variants, so they no longer reach the `Unknown` arm below, whose copies should now be unreachable.
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
							buffer.add_marker(Marker::new(MarkerType::PageBreak, buffer.end()));
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
						pending_link = Some(PendingLink { url, start_position: buffer.end() });
					} else if let Some(link) = pending_link.take() {
						let display_text = text.to_string();
						let text_len = DocLen::of(&display_text);
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
	// Only truncated RTF with unbalanced braces leaves anything open here; the outermost group close reverts everything otherwise.
	let final_pos = buffer.end();
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
	let leading_trim = DocLen::of(&buffer.content[..buffer.content.len() - buffer.content.trim_start().len()]);
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
	let has_start_page = result.markers.iter().any(|m| m.mtype == MarkerType::PageBreak && m.position == DocPos::ZERO);
	if has_pages && !has_start_page {
		result.add_marker(Marker::new(MarkerType::PageBreak, DocPos::ZERO));
	}
	result
}

#[cfg(test)]
mod tests;
