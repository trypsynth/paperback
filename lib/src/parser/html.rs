use std::fs;

use anyhow::{Context, Result};

use super::utils::{build_toc_from_headings, extract_title_from_path, heading_level_to_marker_type};
use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	html_to_text::{HtmlSourceMode, HtmlToText},
	parser::Parser,
	utils::encoding::convert_to_utf8,
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
		for heading in converter.get_headings() {
			let marker_type = heading_level_to_marker_type(heading.level);
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
					.with_text(table.placeholder.clone())
					.with_reference(table.html_content.clone()),
			);
		}
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}
