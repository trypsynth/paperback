use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	parser::{Parser, utils::extract_title_from_path},
	utils::{encoding::convert_to_utf8, text::remove_soft_hyphens},
};

pub struct TextParser;

impl Parser for TextParser {
	fn name(&self) -> &'static str {
		"Text Files"
	}

	fn extensions(&self) -> &[&str] {
		&["txt", "log"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::NONE
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open text file '{}'", context.file_path))?;
		let utf8_content = convert_to_utf8(&bytes);
		let processed = remove_soft_hyphens(&utf8_content);
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(DocumentBuffer::with_content(processed));
		Ok(doc)
	}
}
