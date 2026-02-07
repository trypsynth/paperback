use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	encoding::convert_to_utf8,
	html_to_text::{HtmlSourceMode, HtmlToText},
	parser::{Parser, add_converter_markers, path::extract_title_from_path, toc::build_toc_from_headings},
};

pub struct HtmlParser;

impl Parser for HtmlParser {
	fn name(&self) -> &'static str {
		"HTML Files"
	}

	fn extensions(&self) -> &[&str] {
		&["htm", "html", "xhtml"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open HTML file '{}'", context.file_path))?;
		if bytes.is_empty() {
			anyhow::bail!("HTML file is empty: {}", context.file_path);
		}
		let html_content = convert_to_utf8(&bytes);
		let mut converter = HtmlToText::new();
		if !converter.convert(&html_content, HtmlSourceMode::NativeHtml) {
			anyhow::bail!("Failed to convert HTML to text: {}", context.file_path);
		}
		let extracted_title = converter.get_title();
		let title = if extracted_title.is_empty() {
			extract_title_from_path(&context.file_path)
		} else {
			extracted_title.to_string()
		};
		let text = converter.get_text();
		let mut buffer = DocumentBuffer::with_content(text);
		let id_positions = converter.get_id_positions().clone();
		add_converter_markers(&mut buffer, &converter, 0);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}
