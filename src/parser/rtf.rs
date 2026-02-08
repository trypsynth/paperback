use std::fs;

use anyhow::{Context, Result};
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
		let tokens = Lexer::scan(content_str).map_err(|e| anyhow::anyhow!("Failed to parse RTF document: {e}"))?;
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

/// Maps Windows-1252 bytes 0x80-0x9F to their Unicode code points.
/// The `rtf_parser` crate represents both `\uN` (Unicode) and `\'xx` (codepage hex) escapes
/// as `ControlWord::Unicode`. For `\'xx`, the value is a raw codepage byte, not a Unicode
/// code point. In the 0x80-0x9F range, Windows-1252 maps to typographic characters (smart
/// quotes, dashes, etc.) while Unicode has invisible C1 control characters. Since C1 controls
/// never appear in document text, values in this range are always codepage bytes.
const fn win1252_to_unicode(byte: u32) -> u32 {
	match byte {
		0x80 => 0x20AC, // €
		0x82 => 0x201A, // ‚
		0x83 => 0x0192, // ƒ
		0x84 => 0x201E, // „
		0x85 => 0x2026, // …
		0x86 => 0x2020, // †
		0x87 => 0x2021, // ‡
		0x88 => 0x02C6, // ˆ
		0x89 => 0x2030, // ‰
		0x8A => 0x0160, // Š
		0x8B => 0x2039, // ‹
		0x8C => 0x0152, // Œ
		0x8E => 0x017D, // Ž
		0x91 => 0x2018, // '
		0x92 => 0x2019, // '
		0x93 => 0x201C, // "
		0x94 => 0x201D, // "
		0x95 => 0x2022, // •
		0x96 => 0x2013, // –
		0x97 => 0x2014, // —
		0x98 => 0x02DC, // ˜
		0x99 => 0x2122, // ™
		0x9A => 0x0161, // š
		0x9B => 0x203A, // ›
		0x9C => 0x0153, // œ
		0x9E => 0x017E, // ž
		0x9F => 0x0178, // Ÿ
		other => other, // 0x81, 0x8D, 0x8F, 0x90 are undefined; pass through everything else
	}
}

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
									// Regular BMP character
									pending_high_surrogate = None;
									let codepoint = win1252_to_unicode(u32::from(code));
									if let Some(ch) = char::from_u32(codepoint) {
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
