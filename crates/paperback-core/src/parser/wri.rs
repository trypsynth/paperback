//! Windows Write (`.wri`), and the RTF or plain text sometimes saved under its name, told apart by the first bytes.

use std::fs;

use anyhow::{Context, Result};

use super::{
	Parser,
	convert::image_text,
	rtf::{RtfParser, looks_like_rtf},
	text::TextParser,
	util::path::extract_title_from_path,
};
use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	util::{encoding::decode_ansi, text::normalize_line_endings},
};

/// Stored little-endian, so the bytes on disk are `31 BE` / `32 BE`.
const WRITE_MAGIC_PLAIN: u16 = 0xBE31;
const WRITE_MAGIC_OLE: u16 = 0xBE32;
const WRITE_TEXT_START: usize = 128;
/// `fcMac` is the file position just past the end of the text.
const WRITE_FCMAC_OFFSET: usize = 0x0E;
/// The 128-byte page where the paragraph property pages start.
const WRITE_PNPARA_OFFSET: usize = 0x12;
const WRITE_PAGE: usize = 128;
/// A paragraph's stored properties skip the PAP's first byte, so `rhc` is at 16; its 0x10 bit marks a picture or OLE object held inside the text run.
const WRITE_RHC_INDEX: usize = 16;
const WRITE_RHC_GRAPHICS: u8 = 0x10;

pub struct WriParser;

/// Word for DOS shares Write's layout, so a `.doc` with this magic is read here too.
pub(super) const fn is_write_magic(magic: u16) -> bool {
	matches!(magic, WRITE_MAGIC_PLAIN | WRITE_MAGIC_OLE)
}

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
	let mut kept = Vec::with_capacity(end - WRITE_TEXT_START);
	let mut from = WRITE_TEXT_START;
	for (start, stop) in picture_paragraphs(bytes, end) {
		kept.extend_from_slice(&bytes[from..start]);
		// A byte Write text never holds stands in for the picture until the text is decoded.
		kept.push(0);
		from = stop;
	}
	kept.extend_from_slice(&bytes[from..end]);
	let image = format!("{}\n", image_text::placeholder(false, ""));
	let mut text = String::with_capacity(kept.len());
	for ch in normalize_line_endings(decode_ansi(&kept)).chars() {
		match ch {
			'\0' => text.push_str(&image),
			// Page and line breaks; the rest of Write's controls are optional hyphens, page-number fields and stray bytes, none of them read.
			'\u{0B}' | '\u{0C}' => text.push('\n'),
			'\t' | '\n' => text.push(ch),
			_ if ch.is_control() => {}
			_ => text.push(ch),
		}
	}
	Some(text)
}

/// The text-run spans of picture paragraphs, in order; a property page that doesn't make sense ends the walk, leaving the rest as text.
fn picture_paragraphs(bytes: &[u8], end: usize) -> Vec<(usize, usize)> {
	let mut pictures = Vec::new();
	let first_page = usize::from(u16::from_le_bytes([bytes[WRITE_PNPARA_OFFSET], bytes[WRITE_PNPARA_OFFSET + 1]]));
	let mut paragraph_start = WRITE_TEXT_START;
	for page in bytes.as_chunks::<WRITE_PAGE>().0.iter().skip(first_page) {
		let runs = usize::from(page[WRITE_PAGE - 1]);
		if usize::try_from(u32::from_le_bytes([page[0], page[1], page[2], page[3]])).ok() != Some(paragraph_start)
			|| runs == 0
			|| 4 + runs * 6 > WRITE_PAGE - 1
		{
			break;
		}
		for run in page[4..4 + runs * 6].as_chunks::<6>().0 {
			let Ok(limit) = usize::try_from(u32::from_le_bytes([run[0], run[1], run[2], run[3]])) else {
				return pictures;
			};
			if limit <= paragraph_start || limit > end + 2 {
				return pictures;
			}
			let property = usize::from(u16::from_le_bytes([run[4], run[5]]));
			let rhc = (property != 0xFFFF)
				.then(|| {
					page.get(4 + property..)
						.filter(|stored| stored.first().is_some_and(|&len| usize::from(len) > WRITE_RHC_INDEX))
						.and_then(|stored| stored.get(1 + WRITE_RHC_INDEX))
				})
				.flatten();
			if rhc.is_some_and(|&rhc| rhc & WRITE_RHC_GRAPHICS != 0) {
				pictures.push((paragraph_start, limit.min(end)));
			}
			paragraph_start = limit;
			if paragraph_start >= end {
				return pictures;
			}
		}
	}
	pictures
}

#[cfg(test)]
mod tests;
