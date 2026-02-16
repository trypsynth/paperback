use std::fs;

use anyhow::{Context, Result};
use encoding_rs::Encoding;
use rtf_parser::{
	lexer::Lexer,
	tokens::{ControlWord, Property, Token},
};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{Parser, path::extract_title_from_path},
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
		ParserFlags::NONE
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open RTF file '{}'", context.file_path))?;
		let content_str = String::from_utf8_lossy(&bytes);
		// Some RTF files have garbage at the end
		let content_str = content_str.trim_end_matches(|c: char| c == '\0' || c.is_whitespace());
		let encoding = extract_codepage(content_str);
		let content_str = resolve_hex_escapes(content_str, encoding);
		// Strip \r so that \r\n line endings don't leave stray carriage returns in text tokens
		let content_str = content_str.replace('\r', "");
		let tokens = Lexer::scan(&content_str).map_err(|e| anyhow::anyhow!("Failed to parse RTF document: {e}"))?;
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

/// Pre-processes RTF text by replacing `\'xx` hex escapes (bytes >= 0x80) with
/// their correctly decoded UTF-8 characters. This resolves the ambiguity between
/// `\'xx` (codepage byte) and `\uN` (Unicode) escapes before the lexer sees them,
/// since the `rtf_parser` crate conflates both into `ControlWord::Unicode`.
///
/// Bytes < 0x80 are left as escapes since they're ASCII (identical across all
/// codepages) and some (`\'7b`, `\'7d`, `\'5c`) are structural RTF characters
/// that the lexer must process as escapes.
fn resolve_hex_escapes(rtf: &str, encoding: &'static Encoding) -> String {
	let mut result = String::with_capacity(rtf.len());
	let bytes = rtf.as_bytes();
	let len = bytes.len();
	let mut i = 0;
	while i < len {
		if bytes[i] == b'\\' && i + 3 < len && bytes[i + 1] == b'\'' {
			let h1 = bytes[i + 2];
			let h2 = bytes[i + 3];
			if let Some(byte) = parse_hex_pair(h1, h2) {
				if byte >= 0x80 {
					let buf = [byte];
					let (decoded, _, _) = encoding.decode(&buf);
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

#[allow(clippy::too_many_lines)]
fn extract_content_from_tokens(tokens: &[Token]) -> DocumentBuffer {
	let mut buffer = DocumentBuffer::new();
	let mut in_header = true;
	let mut pending_high_surrogate: Option<u16> = None;
	let mut pending_link: Option<PendingLink> = None;
	for token in tokens {
		match token {
			Token::ControlSymbol((ctrl, property)) => {
				match ctrl {
					ControlWord::Pard => in_header = false,
					ControlWord::Par => {
						if !in_header {
							buffer.append("\n");
						}
					}
					ControlWord::Unicode => {
						if !in_header {
							if let Property::Value(code) = property {
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
					}
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
			Token::CRLF => {
				if !in_header {
					buffer.append("\n");
				}
			}
			_ => {}
		}
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
	result
}

#[cfg(test)]
mod tests {
	use encoding_rs::Encoding;
	use rstest::rstest;

	use super::{encoding_for_codepage, extract_codepage, hex_digit, parse_hex_pair, resolve_hex_escapes};

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
	fn resolve_hex_escapes_decodes_high_bytes_only() {
		let input = "Cafe\\'e9 and plain";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252);
		assert_eq!(output, "Cafeé and plain");
	}

	#[test]
	fn resolve_hex_escapes_keeps_ascii_escape_sequences() {
		let input = "Escaped brace: \\'7b and slash: \\'5c";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252);
		assert_eq!(output, input);
	}

	#[test]
	fn resolve_hex_escapes_ignores_invalid_hex_sequences() {
		let input = "Broken: \\'zz and mixed: \\'G1";
		let output = resolve_hex_escapes(input, encoding_rs::WINDOWS_1252);
		assert_eq!(output, input);
	}
}
