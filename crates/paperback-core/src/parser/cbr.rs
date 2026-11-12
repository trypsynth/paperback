//! A real RAR comic is read from the `.cbz` copy [`crate::util::rar`] makes of it.

use std::path::Path;

use anyhow::Result;

use super::cbz;
use crate::{
	document::{Document, ParserContext},
	parser::Parser,
	util::rar,
};

pub struct CbrParser;

impl Parser for CbrParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing cbr file");
		// A .cbr that is really a zip is common enough that people barely notice they have one.
		if cbz::is_zip(&context.file_path) {
			tracing::debug!(path = %context.file_path, "cbr file is really a zip, reading it as one");
			return cbz::CbzParser.parse(context);
		}
		let copy = rar::zip_copy(Path::new(&context.file_path), context.password.as_deref(), &rar::cache_root())?;
		cbz::CbzParser.parse(&ParserContext { file_path: copy.to_string_lossy().into_owned(), ..context.clone() })
	}
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

	#[test]
	fn a_real_rar_reads_from_its_copy() {
		let dir = TempDir::new("cbr-rar");
		let mut builder = rars::Builder::new(rars::ArchiveVersion::Rar50);
		for name in ["001.png", "002.png"] {
			builder.add_bytes(name.as_bytes().to_vec(), PIXEL.to_vec(), None, None).expect("add a page");
		}
		let path = write_comic(&dir, "book.cbr", &builder.to_bytes().expect("build the RAR"));
		let document = CbrParser.parse(&ParserContext::new(path)).expect("a RAR comic is read");
		assert_eq!(document.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::PageBreak).count(), 2);
	}

	#[test]
	fn a_zip_is_told_from_a_rar_by_what_is_in_it() {
		let dir = TempDir::new("cbr-magic");
		let rar = write_comic(&dir, "rar.cbr", &MARKER);
		assert!(!cbz::is_zip(&rar));
	}
}
