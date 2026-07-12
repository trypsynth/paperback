use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use encoding_rs::Encoding;
use rtf_parser::{
	lexer::Lexer,
	tokens::{ControlWord, Property, Token},
};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

pub struct RtfParser;

impl Parser for RtfParser {
	fn name(&self) -> &'static str {
		"RTF Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["rtf"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_PAGES
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open RTF file '{}'", context.file_path))?;
		let content_str = String::from_utf8_lossy(&bytes);
		// Some RTF files have garbage at the end
		let content_str = content_str.trim_end_matches(|c: char| c == '\0' || c.is_whitespace());
		let content_str = normalize_wrapped_space_lines(content_str);
		let encoding = extract_codepage(&content_str);
		let font_table = extract_font_table(&content_str, encoding);
		let content_str = resolve_hex_escapes(&content_str, encoding, &font_table);
		// Strip \r so that \r\n line endings don't leave stray carriage returns in text tokens
		let content_str = content_str.replace('\r', "");
		// TRANSLATORS: Error shown when an RTF document's tokens fail to parse; {} is the underlying lexer error
		let tokens = Lexer::scan(&content_str)
			.map_err(|e| anyhow::anyhow!(t("Failed to parse RTF document: {}").replace("{}", &e.to_string())))?;
		let buffer = extract_content_from_tokens(&tokens);
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		Ok(doc)
	}
}

struct PendingLink {
	url: String,
	start_position: usize,
}

/// Some writers hard-wrap lines and occasionally place an inter-word space on
/// its own line (`word\r\n \r\nnext`). Preserve that as a single space so words
/// don't get merged by downstream tokenization.
fn normalize_wrapped_space_lines(input: &str) -> String {
	let mut out = String::with_capacity(input.len());
	let bytes = input.as_bytes();
	let mut i = 0;
	while i < bytes.len() {
		let mut j = i;
		if consume_line_break(bytes, &mut j) {
			while j < bytes.len() && matches!(bytes[j], b' ' | b'\t') {
				j += 1;
			}
			let mut k = j;
			if consume_line_break(bytes, &mut k) {
				let left =
					out.chars().next_back().is_some_and(|ch| !ch.is_whitespace() && !matches!(ch, '\\' | '{' | '}'));
				let right = bytes
					.get(k)
					.copied()
					.is_some_and(|b| !b.is_ascii_whitespace() && !matches!(b, b'\\' | b'{' | b'}'));
				if left && right && !out.ends_with(' ') {
					out.push(' ');
				}
				i = k;
				continue;
			}
		}
		out.push(bytes[i] as char);
		i += 1;
	}
	out
}

fn consume_line_break(bytes: &[u8], idx: &mut usize) -> bool {
	if *idx >= bytes.len() {
		return false;
	}
	match bytes[*idx] {
		b'\r' => {
			*idx += 1;
			if *idx < bytes.len() && bytes[*idx] == b'\n' {
				*idx += 1;
			}
			true
		}
		b'\n' => {
			*idx += 1;
			true
		}
		_ => false,
	}
}

/// Resolves the `encoding_rs` encoding for an RTF `\ansicpg` codepage number.
fn encoding_for_codepage(cpg: i32) -> &'static Encoding {
	match cpg {
		874 => encoding_rs::WINDOWS_874,
		1250 => encoding_rs::WINDOWS_1250,
		1251 => encoding_rs::WINDOWS_1251,
		1253 => encoding_rs::WINDOWS_1253,
		1254 => encoding_rs::WINDOWS_1254,
		1255 => encoding_rs::WINDOWS_1255,
		1256 => encoding_rs::WINDOWS_1256,
		1257 => encoding_rs::WINDOWS_1257,
		1258 => encoding_rs::WINDOWS_1258,
		_ => encoding_rs::WINDOWS_1252, // Default per RTF spec
	}
}

/// Extracts the `\ansicpg` codepage number from the raw RTF text and returns
/// the corresponding encoding. Defaults to Windows-1252 if not found.
fn extract_codepage(rtf: &str) -> &'static Encoding {
	if let Some(pos) = rtf.find("\\ansicpg") {
		let after = &rtf[pos + 8..];
		let num_str: String = after.chars().take_while(char::is_ascii_digit).collect();
		if let Ok(cpg) = num_str.parse::<i32>() {
			return encoding_for_codepage(cpg);
		}
	}
	encoding_rs::WINDOWS_1252
}

/// Maps an RTF `\fcharsetN` number to the corresponding encoding.
fn encoding_for_fcharset(charset: i32, default: &'static Encoding) -> &'static Encoding {
	match charset {
		0 | 2 => default,                 // ANSI / Symbol — use document default
		161 => encoding_rs::WINDOWS_1253, // Greek
		162 => encoding_rs::WINDOWS_1254, // Turkish
		163 => encoding_rs::WINDOWS_1258, // Vietnamese
		177 => encoding_rs::WINDOWS_1255, // Hebrew
		178 => encoding_rs::WINDOWS_1256, // Arabic
		186 => encoding_rs::WINDOWS_1257, // Baltic
		204 => encoding_rs::WINDOWS_1251, // Cyrillic
		238 => encoding_rs::WINDOWS_1250, // Central/Eastern European
		222 => encoding_rs::WINDOWS_874,  // Thai
		_ => default,
	}
}

/// Parses the `{\fonttbl}` group and returns a map from font number to encoding,
/// so that `resolve_hex_escapes` can use the right charset per `\fN` switch.
fn extract_font_table(rtf: &str, default_encoding: &'static Encoding) -> HashMap<u32, &'static Encoding> {
	let mut map = HashMap::new();
	let Some(start) = rtf.find("{\\fonttbl") else { return map };

	// Find the matching closing brace for the {\fonttbl} group.
	let bytes = rtf.as_bytes();
	let mut depth = 0usize;
	let mut fonttbl_end = start;
	for (i, &b) in bytes[start..].iter().enumerate() {
		match b {
			b'{' => depth += 1,
			b'}' => {
				depth = depth.saturating_sub(1);
				if depth == 0 {
					fonttbl_end = start + i + 1;
					break;
				}
			}
			_ => {}
		}
	}

	let fonttbl = &rtf[start..fonttbl_end];
	let fb = fonttbl.as_bytes();
	// Start at 1 to skip the outer '{' of {\fonttbl} itself; inner {\fN...} entries follow.
	let mut j = 1;

	while j < fb.len() {
		if fb[j] != b'{' {
			j += 1;
			continue;
		}
		// Find the matching close for this font entry group.
		let entry_start = j;
		let mut d = 0usize;
		let mut entry_end = j;
		let mut k = j;
		while k < fb.len() {
			match fb[k] {
				b'{' => d += 1,
				b'}' => {
					d = d.saturating_sub(1);
					if d == 0 {
						entry_end = k + 1;
						break;
					}
				}
				_ => {}
			}
			k += 1;
		}

		let entry = &fonttbl[entry_start..entry_end];
		let eb = entry.as_bytes();

		// Find the first \fN (font number selection) in this entry.
		// \fcharset, \fbidi, \froman, etc. all start with \f + non-digit so they won't match.
		let mut font_num: Option<u32> = None;
		let mut ei = 0;
		while ei + 2 < eb.len() {
			if eb[ei] == b'\\' && eb[ei + 1] == b'f' && eb[ei + 2].is_ascii_digit() {
				let num_start = ei + 2;
				let mut num_end = num_start;
				while num_end < eb.len() && eb[num_end].is_ascii_digit() {
					num_end += 1;
				}
				if let Some(n) = std::str::from_utf8(&eb[num_start..num_end]).ok().and_then(|s| s.parse::<u32>().ok()) {
					font_num = Some(n);
					break;
				}
			}
			ei += 1;
		}

		if let Some(fnum) = font_num
			&& let Some(cs_pos) = entry.find("\\fcharset")
		{
			let after = &entry[cs_pos + 9..];
			let num_str: String = after.chars().take_while(char::is_ascii_digit).collect();
			if let Ok(cs) = num_str.parse::<i32>() {
				map.insert(fnum, encoding_for_fcharset(cs, default_encoding));
			}
		}

		j = entry_end.max(j + 1);
	}

	map
}

/// Pre-processes RTF text by replacing `\'xx` hex escapes with their correctly
/// decoded UTF-8 characters. This resolves the ambiguity between
/// `\'xx` (codepage byte) and `\uN` (Unicode) escapes before the lexer sees them,
/// since the `rtf_parser` crate conflates both into `ControlWord::Unicode`.
///
/// Structural ASCII escapes (`\'7b`, `\'7d`, `\'5c`) are left intact so the lexer
/// still handles escaped `{`, `}`, and `\` correctly.
///
/// Tracks `\fN` control words to use the charset declared for each font in the
/// font table, so that Central-European or other non-Latin characters encoded as
/// `\'xx` bytes decode correctly even when the document-level `\ansicpg` differs.
fn resolve_hex_escapes(rtf: &str, encoding: &'static Encoding, font_table: &HashMap<u32, &'static Encoding>) -> String {
	let mut result = String::with_capacity(rtf.len());
	let bytes = rtf.as_bytes();
	let len = bytes.len();
	let mut i = 0;
	let mut current_encoding = encoding;

	while i < len {
		// Track \fN font switches to use the right charset for subsequent \'xx escapes.
		// \fcharset, \fbidi, \froman, etc. start with \f + non-digit so won't match.
		if bytes[i] == b'\\' && i + 2 < len && bytes[i + 1] == b'f' && bytes[i + 2].is_ascii_digit() {
			let num_start = i + 2;
			let mut num_end = num_start;
			while num_end < len && bytes[num_end].is_ascii_digit() {
				num_end += 1;
			}
			if let Ok(s) = std::str::from_utf8(&bytes[num_start..num_end])
				&& let Ok(font_num) = s.parse::<u32>()
			{
				current_encoding = font_table.get(&font_num).copied().unwrap_or(encoding);
			}
			// Fall through — emit the control word bytes as-is for the lexer.
		}

		if bytes[i] == b'\\' && i + 1 < len {
			match bytes[i + 1] {
				// RTF non-breaking space
				b'~' => {
					result.push(' ');
					i += 2;
					continue;
				}
				// Optional / non-breaking hyphen
				b'-' | b'_' => {
					result.push('-');
					i += 2;
					continue;
				}
				_ => {}
			}
		}

		if bytes[i] == b'\\' && i + 3 < len && bytes[i + 1] == b'\'' {
			let h1 = bytes[i + 2];
			let h2 = bytes[i + 3];
			if let Some(byte) = parse_hex_pair(h1, h2) {
				// Normalize fallback bytes after \uN (e.g. \u237\'ed) so token
				// boundaries remain valid and characters are not duplicated.
				if is_unicode_fallback_escape(bytes, i) {
					// Drop fallback bytes and inject a delimiter so `\uN` remains a
					// valid standalone control word for the lexer.
					if !result.ends_with(' ') {
						result.push(' ');
					}
					i += 4;
					continue;
				}
				if !matches!(byte, 0x7B | 0x7D | 0x5C) {
					let buf = [byte];
					let (decoded, _, _) = current_encoding.decode(&buf);
					result.push_str(&decoded);
					i += 4;
					continue;
				}
			}
		}
		result.push(bytes[i] as char);
		i += 1;
	}
	result
}

fn is_unicode_fallback_escape(bytes: &[u8], index: usize) -> bool {
	if index == 0 {
		return false;
	}

	let mut j = index;
	while j > 0 && bytes[j - 1] == b' ' {
		j -= 1;
	}

	let digit_end = j;
	while j > 0 && bytes[j - 1].is_ascii_digit() {
		j -= 1;
	}
	if j == digit_end {
		return false;
	}
	if j > 0 && bytes[j - 1] == b'-' {
		j -= 1;
	}

	j >= 2 && bytes[j - 1] == b'u' && bytes[j - 2] == b'\\'
}

/// Parses two ASCII hex digit bytes into a `u8`.
fn parse_hex_pair(h1: u8, h2: u8) -> Option<u8> {
	let d1 = hex_digit(h1)?;
	let d2 = hex_digit(h2)?;
	Some(d1 << 4 | d2)
}

const fn hex_digit(b: u8) -> Option<u8> {
	match b {
		b'0'..=b'9' => Some(b - b'0'),
		b'a'..=b'f' => Some(b - b'a' + 10),
		b'A'..=b'F' => Some(b - b'A' + 10),
		_ => None,
	}
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
fn extract_content_from_tokens(tokens: &[Token]) -> DocumentBuffer {
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
					ControlWord::Par => {
						if !in_header {
							buffer.append("\n");
						}
					}
					ControlWord::Line => {
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
	apply_format_toggle(&mut bold_on, &mut bold_start, false, final_pos, MarkerType::Bold, &mut buffer);
	apply_format_toggle(&mut italic_on, &mut italic_start, false, final_pos, MarkerType::Italic, &mut buffer);
	apply_format_toggle(&mut underline_on, &mut underline_start, false, final_pos, MarkerType::Underline, &mut buffer);
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

	use encoding_rs::Encoding;
	use rstest::rstest;
	use rtf_parser::{
		lexer::Lexer,
		tokens::{ControlWord, Property, Token},
	};

	use super::{
		encoding_for_codepage, extract_codepage, extract_content_from_tokens, extract_font_table, hex_digit,
		is_unicode_fallback_escape, normalize_wrapped_space_lines, parse_hex_pair, resolve_hex_escapes,
	};
	use crate::document::MarkerType;

	fn enc_name(enc: &'static Encoding) -> &'static str {
		enc.name()
	}

	#[rstest]
	#[case(1252, "windows-1252")]
	#[case(1251, "windows-1251")]
	#[case(1258, "windows-1258")]
	#[case(874, "windows-874")]
	#[case(9999, "windows-1252")]
	fn encoding_for_codepage_maps_supported_and_defaults(#[case] codepage: i32, #[case] expected: &str) {
		assert_eq!(enc_name(encoding_for_codepage(codepage)), expected);
	}

	#[rstest]
	#[case("{\\rtf1\\ansi\\ansicpg1251 hello}", "windows-1251")]
	#[case("{\\rtf1\\ansi\\ansicpg1258 hello}", "windows-1258")]
	#[case("{\\rtf1\\ansi\\ansicpgNOTNUM hello}", "windows-1252")]
	#[case("{\\rtf1\\ansi hello}", "windows-1252")]
	fn extract_codepage_reads_ansicpg_when_present(#[case] rtf: &str, #[case] expected: &str) {
		assert_eq!(enc_name(extract_codepage(rtf)), expected);
	}

	#[rstest]
	#[case(b'0', Some(0))]
	#[case(b'9', Some(9))]
	#[case(b'a', Some(10))]
	#[case(b'f', Some(15))]
	#[case(b'A', Some(10))]
	#[case(b'F', Some(15))]
	#[case(b'g', None)]
	#[case(b'/', None)]
	fn hex_digit_classifies_ascii_hex(#[case] input: u8, #[case] expected: Option<u8>) {
		assert_eq!(hex_digit(input), expected);
	}

	#[rstest]
	#[case(b'4', b'1', Some(0x41))]
	#[case(b'e', b'9', Some(0xE9))]
	#[case(b'E', b'9', Some(0xE9))]
	#[case(b'Z', b'9', None)]
	#[case(b'1', b'X', None)]
	fn parse_hex_pair_parses_and_rejects_invalid(#[case] h1: u8, #[case] h2: u8, #[case] expected: Option<u8>) {
		assert_eq!(parse_hex_pair(h1, h2), expected);
	}

	#[test]
	fn resolve_hex_escapes_decodes_non_structural_escapes() {
		let input = "Don\\'27t say Caf\\'e9";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
		assert_eq!(output, "Don't say Café");
	}

	#[test]
	fn resolve_hex_escapes_keeps_ascii_escape_sequences() {
		let input = "Escaped brace: \\'7b and slash: \\'5c";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
		assert_eq!(output, input);
	}

	#[test]
	fn resolve_hex_escapes_ignores_invalid_hex_sequences() {
		let input = "Broken: \\'zz and mixed: \\'G1";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
		assert_eq!(output, input);
	}
	#[test]
	fn resolve_hex_escapes_keeps_u_fallback_hex_sequences() {
		let input = "Ju\\u237\\'edzo";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
		assert_eq!(output, "Ju\\u237 zo");
	}

	#[test]
	fn resolve_hex_escapes_maps_nonbreaking_space_and_hyphen_symbols() {
		let input = "A\\~B C\\_D E\\-F";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
		assert_eq!(output, "A B C-D E-F");
	}

	#[test]
	fn is_unicode_fallback_escape_detects_after_u_control_word() {
		let bytes = br"Ju\\u237\\'edzo";
		assert!(is_unicode_fallback_escape(bytes, 7));
	}

	#[test]
	fn is_unicode_fallback_escape_rejects_plain_hex_escape() {
		let bytes = br"Don\\'27t";
		assert!(!is_unicode_fallback_escape(bytes, 3));
	}

	#[test]
	fn normalize_wrapped_space_lines_preserves_inter_word_space_on_its_own_line() {
		let input = "The older man was\r\n \r\nwordless";
		assert_eq!(normalize_wrapped_space_lines(input), "The older man was wordless");
	}

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
		let normalized = resolve_hex_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "delay.\n\tnext");
	}

	#[test]
	fn extract_content_maps_page_control_to_marker_and_separator() {
		let rtf = r"{\rtf1\ansi\pard chapter one\page chapter two}";
		let normalized = resolve_hex_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "chapter one chapter two");
		let page_markers: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).collect();
		assert_eq!(page_markers.len(), 2);
		assert_eq!(page_markers[0].position, "chapter one ".chars().count());
		assert_eq!(page_markers[1].position, 0);
	}

	#[test]
	fn extract_font_table_maps_fcharset_to_encoding() {
		// Font 1 = ANSI (charset 0 → default), font 2 = CE (charset 238 → Windows-1250)
		let rtf =
			r"{\rtf1\ansi\ansicpg1252{\fonttbl{\f1\fcharset0 Arial;}{\f2\fcharset238 Times New Roman CE;}}\pard hello}";
		let default_enc = encoding_rs::WINDOWS_1252;
		let map = extract_font_table(rtf, default_enc);
		assert_eq!(map.get(&1).map(|e| e.name()), Some("windows-1252"));
		assert_eq!(map.get(&2).map(|e| e.name()), Some("windows-1250"));
	}

	#[test]
	fn resolve_hex_escapes_uses_font_charset_for_encoding() {
		// \f2 switches to charset 238 (Windows-1250); \'c6 = Ć in that encoding, Æ in 1252.
		let rtf = r"{\rtf1\ansi\ansicpg1252{\fonttbl{\f1\fcharset0 Arial;}{\f2\fcharset238 CE;}}\pard\f2 \'c6ao}";
		let default_enc = encoding_rs::WINDOWS_1252;
		let font_table = extract_font_table(rtf, default_enc);
		let out = resolve_hex_escapes(rtf, default_enc, &font_table);
		assert!(out.contains('Ć'), "expected Ć (Windows-1250 0xC6), got: {}", &out);
		assert!(!out.contains('Æ'), "should not contain Æ (Windows-1252 0xC6)");
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
		let normalized = resolve_hex_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
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
		let normalized = resolve_hex_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
		let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
		let buffer = extract_content_from_tokens(&tokens);
		assert_eq!(buffer.content, "normal bold normal");
		let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
		assert_eq!(bold.len(), 1);
		assert_eq!(bold[0].position, "normal ".chars().count());
		assert_eq!(bold[0].length, "bold".chars().count());
	}
}
