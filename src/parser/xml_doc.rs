use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	encoding::convert_to_utf8,
	parser::{Parser, path::extract_title_from_path, toc::build_toc_from_headings},
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
		for heading in converter.get_headings() {
			let marker_type = crate::parser::toc::heading_level_to_marker_type(heading.level);
			buffer.add_marker(
				Marker::new(marker_type, heading.offset).with_text(heading.text.clone()).with_level(heading.level),
			);
		}
		for link in converter.get_links() {
			buffer.add_marker(
				Marker::new(MarkerType::Link, link.offset)
					.with_text(link.text.clone())
					.with_reference(link.reference.clone()),
			);
		}
		for list in converter.get_lists() {
			buffer.add_marker(Marker::new(MarkerType::List, list.offset).with_level(list.item_count));
		}
		for list_item in converter.get_list_items() {
			buffer.add_marker(
				Marker::new(MarkerType::ListItem, list_item.offset)
					.with_text(list_item.text.clone())
					.with_level(list_item.level),
			);
		}
		for table in converter.get_tables() {
			buffer.add_marker(
				Marker::new(MarkerType::Table, table.offset)
					.with_text(table.text.clone())
					.with_reference(table.html_content.clone())
					.with_length(table.length),
			);
		}
		for separator in converter.get_separators() {
			buffer.add_marker(
				Marker::new(MarkerType::Separator, separator.offset)
					.with_text("Separator".to_string())
					.with_length(separator.length),
			);
		}
		let toc_items = build_toc_from_headings(converter.get_headings());
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}
