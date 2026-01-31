use std::fs;

use anyhow::{Context, Result};
use rtf_parser::{
	lexer::Lexer,
	tokens::{ControlWord, Property, Token},
};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{Parser, utils::extract_title_from_path},
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
		let content = String::from_utf8_lossy(&bytes);
		// Some RTF files have garbage at the end
		let content = content.trim_end_matches(|c: char| c == '\0' || c.is_whitespace());
		let tokens = Lexer::scan(content).map_err(|e| anyhow::anyhow!("Failed to parse RTF document: {e}"))?;
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
								let code = if *code < 0 { (*code + 65536) as u16 } else { *code as u16 };
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
			Marker::new(marker.marker_type, adjusted_pos)
				.with_text(marker.text)
				.with_reference(marker.reference)
				.with_length(marker.length)
				.with_level(marker.level),
		);
	}
	result
}
