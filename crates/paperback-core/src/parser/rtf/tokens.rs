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
				// {\* \keyword content}: skip the entire enclosing group. A destination
				// nested inside another skipped destination (e.g. \picprop inside \pict
				// inside \shppict) must not shrink the skip window: keep the outermost
				// (smallest) depth so skipping doesn't end early when the inner group closes.
				let candidate = depth - 1;
				skip_until_depth = Some(skip_until_depth.map_or(candidate, |sd| sd.min(candidate)));
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
mod tests;
