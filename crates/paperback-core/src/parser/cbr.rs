//! Comic book archives packed with RAR (`.cbr`): the same book as a [`super::cbz`], in the
//! other archive format the scene settled on.
//!
//! Nothing here decides what a page is or how the pages are ordered; that all comes from
//! [`super::cbz`], so the two formats read the same book the same way. What is different is
//! getting at the entries: a RAR is read from the front, one member at a time, rather than
//! opened at any member the way a zip is.

use anyhow::{Context, Result};

use super::cbz;
use crate::{
	document::{Document, ParserContext},
	parser::Parser,
	t,
};

pub struct CbrParser;

impl Parser for CbrParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing cbr file");
		// A .cbr that is really a zip is common enough that people barely notice they have
		// one, and the reader has no reason to be the first to tell them.
		if cbz::is_zip(&context.file_path) {
			tracing::debug!(path = %context.file_path, "cbr file is really a zip, reading it as one");
			return cbz::CbzParser.parse(context);
		}
		let pages = page_names(&context.file_path)?;
		tracing::debug!(path = %context.file_path, pages = pages.len(), "cbr structure read");
		cbz::comic_document(&pages, &context.file_path)
	}
}

/// The archive's page images, in reading order.
///
/// Shared with [`crate::ocr`], which reopens the archive to render a page and has to number
/// the pages exactly as this parser did.
///
/// # Errors
///
/// Returns an error if the archive cannot be opened or its entries cannot be listed.
pub fn page_names(file_path: &str) -> Result<Vec<String>> {
	let list = unrar::Archive::new(file_path).open_for_listing().map_err(|e| {
		tracing::warn!(path = %file_path, error = %e, "failed to open comic archive");
		// TRANSLATORS: Error shown when a comic book archive (.cbr) cannot be opened; {} is the underlying error
		anyhow::anyhow!(t("Failed to parse comic archive: {}").replace("{}", &e.to_string()))
	})?;
	let mut names = Vec::new();
	for header in list {
		let header = header.with_context(|| format!("Failed to read an entry of '{file_path}'"))?;
		if header.is_directory() {
			continue;
		}
		let name = header.filename.to_string_lossy().to_string();
		if cbz::is_page(&name) {
			names.push(name);
		}
	}
	names.sort_by(|a, b| cbz::natural_cmp(a, b));
	Ok(names)
}

/// The bytes of one page, read out of the archive.
///
/// A RAR is read from the front: reaching a page means stepping over every member before it,
/// which the library does without unpacking them.
///
/// # Errors
///
/// Returns an error if the archive cannot be opened or read, or if it holds no member of that
/// name.
pub fn page_bytes(file_path: &str, name: &str) -> Result<Vec<u8>> {
	let mut archive = Some(
		unrar::Archive::new(file_path)
			.open_for_processing()
			.with_context(|| format!("Failed to open comic archive '{file_path}'"))?,
	);
	while let Some(open) = archive.take() {
		let Some(header) = open.read_header().with_context(|| format!("Failed to read '{file_path}'"))? else {
			break;
		};
		if header.entry().filename.to_string_lossy() == name {
			let (bytes, _) = header.read().with_context(|| format!("Failed to read '{name}'"))?;
			return Ok(bytes);
		}
		archive = Some(header.skip().with_context(|| format!("Failed to read '{file_path}'"))?);
	}
	anyhow::bail!("comic archive '{file_path}' has no entry named '{name}'")
}

#[cfg(test)]
mod tests {
	use std::{fs, io::Write};

	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	/// The seven bytes every RAR starts with.
	const MARKER: [u8; 7] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00];

	fn crc32(data: &[u8]) -> u32 {
		let mut crc = 0xFFFF_FFFFu32;
		for byte in data {
			crc ^= u32::from(*byte);
			for _ in 0..8 {
				let carry = crc & 1;
				crc >>= 1;
				if carry != 0 {
					crc ^= 0xEDB8_8320;
				}
			}
		}
		!crc
	}

	/// A header's checksum is the bottom half of its CRC32.
	fn low_word(crc: u32) -> u16 {
		u16::try_from(crc & 0xFFFF).expect("sixteen bits fit in a u16")
	}

	/// Builds a stored (uncompressed) RAR of `members`, which is what a comic archive is:
	/// artwork does not compress, so the packers leave it alone.
	///
	/// Written by hand because nothing packs a RAR: the format's compression is proprietary
	/// and every library that reads one, ours included, only reads. Storing needs none of it.
	/// <https://codedread.github.io/bitjs/docs/unrar.html>
	fn stored_rar(members: &[(&str, &[u8])]) -> Vec<u8> {
		let mut out = Vec::from(MARKER);
		// The archive header: its type, no flags, and the 13 bytes it takes up.
		let mut main = vec![0x73u8];
		main.extend_from_slice(&0x0000u16.to_le_bytes());
		main.extend_from_slice(&13u16.to_le_bytes());
		main.extend_from_slice(&[0u8; 6]);
		out.extend_from_slice(&low_word(crc32(&main)).to_le_bytes());
		out.extend_from_slice(&main);
		for (name, data) in members {
			let name = name.as_bytes();
			let mut header = vec![0x74u8];
			// The one flag that matters here says the packed size field is present.
			header.extend_from_slice(&0x8000u16.to_le_bytes());
			header.extend_from_slice(&(32 + u16::try_from(name.len()).expect("a short name")).to_le_bytes());
			header.extend_from_slice(&u32::try_from(data.len()).expect("a small file").to_le_bytes());
			header.extend_from_slice(&u32::try_from(data.len()).expect("a small file").to_le_bytes());
			header.push(0x02);
			header.extend_from_slice(&crc32(data).to_le_bytes());
			header.extend_from_slice(&0x5000_0000u32.to_le_bytes());
			header.push(20);
			header.push(0x30);
			header.extend_from_slice(&u16::try_from(name.len()).expect("a short name").to_le_bytes());
			header.extend_from_slice(&0x20u32.to_le_bytes());
			header.extend_from_slice(name);
			out.extend_from_slice(&low_word(crc32(&header)).to_le_bytes());
			out.extend_from_slice(&header);
			out.extend_from_slice(data);
		}
		out
	}

	/// A one-pixel PNG, so that a page is a picture something could actually decode.
	const PIXEL: [u8; 67] = [
		0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00,
		0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00,
		0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D,
		0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
	];

	fn write_comic(dir: &TempDir, name: &str, bytes: &[u8]) -> String {
		let path = dir.join_str(name);
		fs::write(&path, bytes).expect("write the comic");
		path
	}

	#[test]
	fn a_rar_comic_reads_its_pages_in_the_order_they_are_numbered() {
		let dir = TempDir::new("cbr-pages");
		let comic = stored_rar(&[("002.png", &PIXEL), ("010.png", &PIXEL), ("001.png", &PIXEL)]);
		let path = write_comic(&dir, "book.cbr", &comic);
		let document = CbrParser.parse(&ParserContext::new(path)).expect("read the comic");
		let names: Vec<&str> = document
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::Image)
			.map(|marker| marker.reference.as_str())
			.collect();
		assert_eq!(names, ["001.png", "002.png", "010.png"]);
	}

	/// The metadata a comic archive carries beside its artwork is not a page, whichever
	/// archive format it was packed in.
	#[test]
	fn a_rar_of_nothing_but_metadata_has_no_pages() {
		let dir = TempDir::new("cbr-empty");
		let comic = stored_rar(&[("ComicInfo.xml", b"<ComicInfo/>"), ("readme.txt", b"hi")]);
		let path = write_comic(&dir, "book.cbr", &comic);
		let error = CbrParser.parse(&ParserContext::new(path)).expect_err("no pages in it");
		assert!(error.to_string().contains("no pages"), "{error}");
	}

	/// People rename these by hand, so a .cbr is a zip about as often as anyone bothers to
	/// check. Reading it is one line, and refusing it would only puzzle the reader.
	#[test]
	fn a_cbr_that_is_really_a_zip_reads_as_one() {
		let dir = TempDir::new("cbr-zip");
		let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
		for name in ["001.png", "002.png"] {
			zip.start_file(name, zip::write::SimpleFileOptions::default()).expect("start");
			zip.write_all(&PIXEL).expect("write");
		}
		let bytes = zip.finish().expect("finish").into_inner();
		let path = write_comic(&dir, "book.cbr", &bytes);
		let document = CbrParser.parse(&ParserContext::new(path)).expect("read the comic");
		assert_eq!(document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).count(), 2);
	}

	/// What the OCR flow calls to put a page in front of the engine.
	#[test]
	fn a_page_comes_back_byte_for_byte() {
		let dir = TempDir::new("cbr-bytes");
		let comic = stored_rar(&[("001.png", &PIXEL), ("002.png", b"the second one")]);
		let path = write_comic(&dir, "book.cbr", &comic);
		assert_eq!(page_bytes(&path, "002.png").expect("read the page"), b"the second one");
		assert_eq!(page_bytes(&path, "001.png").expect("read the page"), PIXEL);
	}

	/// A zip is a zip whatever it is called, and a RAR is not one.
	#[test]
	fn a_zip_is_told_from_a_rar_by_what_is_in_it() {
		let dir = TempDir::new("cbr-magic");
		let rar = write_comic(&dir, "rar.cbr", &stored_rar(&[("001.png", &PIXEL)]));
		assert!(!cbz::is_zip(&rar));
	}
}
