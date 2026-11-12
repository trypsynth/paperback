//! A RAR archive of books, read as the zip copy [`crate::util::rar`] makes of it. The desktop app opens that copy itself, so it can offer the book picker a zip gets; this is for everywhere else, such as `pb` and the phone apps.

use std::path::Path;

use anyhow::Result;

use crate::{
	document::{Document, ParserContext},
	parser::{Parser, parse_document},
	util::rar,
};

pub struct RarParser;

impl Parser for RarParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing rar archive");
		let copy = rar::zip_copy(Path::new(&context.file_path), context.password.as_deref(), &rar::cache_root())?;
		// The copy holds no password of its own, and takes whatever format claims `.zip`.
		parse_document(&ParserContext {
			file_path: copy.to_string_lossy().into_owned(),
			password: None,
			forced_extension: None,
			..context.clone()
		})
	}
}

#[cfg(test)]
mod tests {
	use std::fs;

	use rars::{ArchiveVersion, Builder};

	use super::*;
	use crate::util::test_support::TempDir;

	#[test]
	fn reads_the_book_a_rar_holds() {
		let dir = TempDir::new("rar-parser");
		let mut builder = Builder::new(ArchiveVersion::Rar50);
		let fb2 = r#"<?xml version="1.0" encoding="utf-8"?><FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0"><description><title-info><book-title>Packed Away</book-title></title-info></description><body><section><title><p>One</p></title><p>Inside a RAR.</p></section></body></FictionBook>"#;
		builder.add_bytes(b"book.fb2".to_vec(), fb2.as_bytes().to_vec(), None, None).expect("add the book");
		let path = dir.join_str("book.rar");
		fs::write(&path, builder.to_bytes().expect("build the RAR")).expect("write the RAR");
		let document = parse_document(&ParserContext::new(path)).expect("the book inside is read");
		assert!(document.buffer.content.contains("Inside a RAR."), "{}", document.buffer.content);
	}
}
