use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	parser::{Parser, util::path::extract_title_from_path},
	util::{encoding::convert_to_utf8, text::remove_soft_hyphens},
};

pub struct TextParser;

impl Parser for TextParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing text file");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open text file '{}'", context.file_path))?;
		let utf8_content = convert_to_utf8(&bytes);
		let processed = remove_soft_hyphens(&utf8_content);
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		let char_len = processed.chars().count();
		doc.set_buffer(DocumentBuffer::with_content(processed));
		tracing::debug!(path = %context.file_path, bytes = bytes.len(), chars = char_len, "parsed text file");
		Ok(doc)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::test_support::TempDir;

	fn parse_bytes(name: &str, contents: impl AsRef<[u8]>) -> Document {
		let dir = TempDir::new("text-parser");
		let path = dir.write_str(name, contents);
		TextParser.parse(&ParserContext::new(path)).expect("parse text document")
	}

	#[test]
	fn reads_utf8_content_verbatim() {
		let doc = parse_bytes("book.txt", "first line\nsecond line\n");
		assert_eq!(doc.buffer.content, "first line\nsecond line\n");
	}

	#[test]
	fn takes_the_title_from_the_file_name() {
		let doc = parse_bytes("The Great Novel.txt", "content");
		assert_eq!(doc.title, "The Great Novel");
	}

	/// UTF-16 files are common enough from Windows editors that the parser decodes them by BOM
	/// rather than mangling them into replacement characters.
	#[test]
	fn decodes_utf16_by_byte_order_mark() {
		let mut bytes = vec![0xFF, 0xFE];
		for unit in "héllo".encode_utf16() {
			bytes.extend_from_slice(&unit.to_le_bytes());
		}
		let doc = parse_bytes("book.txt", bytes);
		assert_eq!(doc.buffer.content, "héllo");
	}

	/// Soft hyphens are invisible line-break hints; leaving them in makes searching and
	/// screen-reader output wrong, so the parser strips them.
	#[test]
	fn strips_soft_hyphens() {
		let doc = parse_bytes("book.txt", "hy\u{00AD}phen\u{00AD}ated");
		assert_eq!(doc.buffer.content, "hyphenated");
	}

	#[test]
	fn accepts_an_empty_file() {
		let doc = parse_bytes("book.txt", "");
		assert_eq!(doc.buffer.content, "");
	}

	#[test]
	fn reports_the_path_when_the_file_is_missing() {
		let dir = TempDir::new("text-parser");
		let missing = dir.join_str("nope.txt");
		let err = TextParser.parse(&ParserContext::new(missing.clone())).expect_err("missing file must fail");
		assert!(err.to_string().contains(&missing), "error should name the file: {err}");
	}
}
