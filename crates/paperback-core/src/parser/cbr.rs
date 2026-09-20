//! Comic book archives packed with RAR (`.cbr`).
//!
//! Only the ones that are secretly zips. RAR's decoder is RARLAB's C++, carried by
//! `unrar_sys`, which picks the sources it compiles from the operating system doing the
//! compiling rather than the one being compiled for. That breaks every cross build we do:
//! Android gets the Unix set and calls `lutimes`, which bionic does not have, and iOS gets
//! objects built against the host SDK that the linker then refuses at our deployment target.
//! Both are changes to `unrar_sys`, not to anything here, so the reader says so instead.
//!
//! A `.cbr` is a zip often enough that people barely notice they have one, and those still
//! open: [`super::cbz`] decides what a page is and how pages are ordered, for both formats.

use anyhow::Result;

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
		Err(unsupported())
	}
}

/// What a reader is told when the archive really is a RAR.
fn unsupported() -> anyhow::Error {
	// TRANSLATORS: Error shown when opening a comic book archive packed with RAR (.cbr), which Paperback cannot read
	anyhow::anyhow!(t("Paperback cannot read RAR comic archives. Repack it as a .cbz to read it here."))
}

/// The archive's page images, in reading order.
///
/// Shared with [`crate::ocr`], which reopens the archive to render a page and has to number
/// the pages exactly as this parser did.
///
/// # Errors
///
/// Always, for a RAR: see the module note.
pub fn page_names(_file_path: &str) -> Result<Vec<String>> {
	Err(unsupported())
}

/// One page's bytes, by the name [`page_names`] gave it.
///
/// # Errors
///
/// Always, for a RAR: see the module note.
pub fn page_bytes(_file_path: &str, _name: &str) -> Result<Vec<u8>> {
	Err(unsupported())
}

#[cfg(test)]
mod tests {
	use std::{fs, io::Write};

	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	/// The seven bytes every RAR starts with.
	const MARKER: [u8; 7] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00];

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

	/// An actual RAR is refused, and says why rather than failing as a broken zip would.
	#[test]
	fn a_real_rar_is_refused_with_advice() {
		let dir = TempDir::new("cbr-rar");
		let path = write_comic(&dir, "book.cbr", &MARKER);
		let error = CbrParser.parse(&ParserContext::new(path)).expect_err("a RAR is not read");
		assert!(error.to_string().contains(".cbz"), "{error}");
	}

	/// A zip is a zip whatever it is called, and a RAR is not one.
	#[test]
	fn a_zip_is_told_from_a_rar_by_what_is_in_it() {
		let dir = TempDir::new("cbr-magic");
		let rar = write_comic(&dir, "rar.cbr", &MARKER);
		assert!(!cbz::is_zip(&rar));
	}
}
