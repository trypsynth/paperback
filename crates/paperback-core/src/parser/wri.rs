//! Windows Write (`.wri`).
//!
//! The `.wri` extension has meant three different things over the years, and a real one on disk is
//! almost never the format the name suggests. Of the files people actually keep, the overwhelming
//! majority are Rich Text Format or plain text saved with a `.wri` name (a WordPad or Write "save
//! as" habit from the 1990s), and only a rare few are the genuine Windows 3.x Write binary. So
//! this parser reads the first bytes and routes: an RTF file goes to [`super::rtf`], the Write
//! binary is read here, and anything else is read as plain text by [`super::text`]. That covers
//! every `.wri` seen in a wide sample and turns a hard "cannot read" into a document either way.
//!
//! The Write binary format is a small header followed by the text, laid out much like the Word 6
//! documents in [`super::word`]: a byte run whose end the header records, in the codepage of the
//! machine that wrote it. It is read the same way, through the shared encoding detector.

use std::fs;

use anyhow::{Context, Result};

use super::{Parser, rtf::RtfParser, text::TextParser, util::path::extract_title_from_path};
use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	util::encoding::convert_to_utf8,
};

/// Windows Write binary signatures: `0x31BE` for a plain document, `0x32BE` for one that also
/// carries OLE objects. Stored little-endian, so the bytes on disk are `BE 31` / `BE 32`.
const WRITE_MAGIC_PLAIN: u16 = 0x31BE;
const WRITE_MAGIC_OLE: u16 = 0x32BE;
/// The Write header is 128 bytes; the document text begins right after it.
const WRITE_TEXT_START: usize = 128;
/// `fcMac`, the file position just past the end of the text, is a `u32` at offset 14 of the header.
const WRITE_FCMAC_OFFSET: usize = 0x0E;

pub struct WriParser;

impl Parser for WriParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing wri file");
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open WRI file '{}'", context.file_path))?;
		if is_rtf(&bytes) {
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

/// Whether the bytes are Rich Text Format: `{\rtf` at the start, past a byte order mark or leading
/// whitespace, which is how a real RTF-in-`.wri` opens.
fn is_rtf(bytes: &[u8]) -> bool {
	let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
	let start = bytes.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(bytes.len());
	bytes[start..].starts_with(br"{\rtf")
}

/// The text of a genuine Windows Write binary document, or `None` when the bytes are not one.
///
/// The header records `fcMac`, the file position just past the text, and the text starts at a
/// fixed offset after the header, so the run between them is the document. A file whose `fcMac`
/// makes no sense is not treated as Write at all, so the caller falls back to reading it as text
/// rather than emitting the paragraph and formatting tables that follow the run.
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
