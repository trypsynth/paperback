use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	encoding::convert_to_utf8,
	parser::{Parser, add_converter_markers, path::extract_title_from_path, toc::build_toc_from_headings},
	xml_to_text::XmlToText,
};

pub struct XmlParser;

impl Parser for XmlParser {
	fn name(&self) -> &'static str {
		"XML Files"
	}

	fn extensions(&self) -> &[&str] {
		&["xml"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open XML file '{}'", context.file_path))?;
		if bytes.is_empty() {
			anyhow::bail!("XML file is empty: {}", context.file_path);
		}
		let xml_content = convert_to_utf8(&bytes);
		let mut converter = XmlToText::new();
		if !converter.convert(&xml_content) {
			anyhow::bail!("Failed to convert XML to text: {}", context.file_path);
		}
		let text = converter.get_text();
		let mut buffer = DocumentBuffer::with_content(text);
		let id_positions = converter.get_id_positions().clone();
		add_converter_markers(&mut buffer, &converter, 0);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}
