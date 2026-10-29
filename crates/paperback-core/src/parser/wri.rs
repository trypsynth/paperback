//! Windows Write (`.wri`). Most real `.wri` files are RTF or plain text under that name, so this routes on the first bytes and reads only the genuine Write binary itself.

use std::fs;

use anyhow::{Context, Result};

use super::{
	Parser,
	rtf::{RtfParser, looks_like_rtf},
	text::TextParser,
	util::path::extract_title_from_path,
};
use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	util::encoding::convert_to_utf8,
};

/// Stored little-endian, so the bytes on disk are `BE 31` / `BE 32`.
const WRITE_MAGIC_PLAIN: u16 = 0x31BE;
const WRITE_MAGIC_OLE: u16 = 0x32BE;
const WRITE_TEXT_START: usize = 128;
/// `fcMac` is the file position just past the end of the text.
const WRITE_FCMAC_OFFSET: usize = 0x0E;

pub struct WriParser;

impl Parser for WriParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing wri file");
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open WRI file '{}'", context.file_path))?;
		if looks_like_rtf(&bytes) {
			tracing::debug!(path = %context.file_path, "wri file is really rtf, routing to the rtf parser");
			return RtfParser.parse(context);
		}
		if let Some(text) = write_binary_text(&bytes) {
			tracing::debug!(path = %context.file_path, "wri file is the windows write binary format");
			let title = extract_title_from_path(&context.file_path);
			let mut document = Document::new().with_title(title);
			document.set_buffer(DocumentBuffer::with_content(text));
			return Ok(document);
		}
		tracing::debug!(path = %context.file_path, "wri file is plain text, routing to the text parser");
		TextParser.parse(context)
	}
}

/// A nonsense `fcMac` means the file isn't Write, so the caller reads it as text rather than emitting the paragraph and formatting tables past the run.
fn write_binary_text(bytes: &[u8]) -> Option<String> {
	if bytes.len() < WRITE_TEXT_START {
		return None;
	}
	let magic = u16::from_le_bytes([bytes[0], bytes[1]]);
	if magic != WRITE_MAGIC_PLAIN && magic != WRITE_MAGIC_OLE {
		return None;
	}
	let fc_mac = u32::from_le_bytes(bytes[WRITE_FCMAC_OFFSET..WRITE_FCMAC_OFFSET + 4].try_into().ok()?);
	let end = usize::try_from(fc_mac).ok()?;
	if end <= WRITE_TEXT_START || end > bytes.len() {
		return None;
	}
	Some(convert_to_utf8(&bytes[WRITE_TEXT_START..end]))
}

#[cfg(test)]
mod tests;
