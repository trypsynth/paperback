use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	parser::{
		Parser, add_converter_markers,
		html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::encoding::convert_to_utf8,
};

pub struct HtmlParser;

impl Parser for HtmlParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open HTML file '{}'", context.file_path))?;
		if bytes.is_empty() {
			// TRANSLATORS: Error shown when an HTML file has no content; {} is the file path
			anyhow::bail!(t("HTML file is empty: {}").replace("{}", &context.file_path));
		}
		let html_content = convert_to_utf8(&bytes);
		let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
		if !converter.convert(&html_content, HtmlSourceMode::NativeHtml) {
			// TRANSLATORS: Error shown when an HTML file fails to convert to plain text; {} is the file path
			anyhow::bail!(t("Failed to convert HTML to text: {}").replace("{}", &context.file_path));
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	fn parse_html(name: &str, contents: &str) -> Result<Document> {
		let dir = TempDir::new("html-parser");
		let path = dir.write_str(name, contents);
		HtmlParser.parse(&ParserContext::new(path))
	}

	fn parse_ok(contents: &str) -> Document {
		parse_html("page.html", contents).expect("parse html document")
	}

	#[test]
	fn extracts_visible_text_without_markup() {
		let doc = parse_ok("<html><body><p>Hello there</p><p>Second paragraph</p></body></html>");
		assert!(doc.buffer.content.contains("Hello there"), "text: {:?}", doc.buffer.content);
		assert!(doc.buffer.content.contains("Second paragraph"));
		assert!(!doc.buffer.content.contains('<'), "markup leaked into text: {:?}", doc.buffer.content);
	}

	#[test]
	fn prefers_the_title_element_over_the_file_name() {
		let doc = parse_ok("<html><head><title>Document Title</title></head><body><p>x</p></body></html>");
		assert_eq!(doc.title, "Document Title");
	}

	#[test]
	fn falls_back_to_the_file_name_when_there_is_no_title() {
		let doc = parse_html("Fallback Name.html", "<html><body><p>x</p></body></html>").expect("parse");
		assert_eq!(doc.title, "Fallback Name");
	}

	#[test]
	fn records_headings_as_markers_and_toc_entries() {
		let doc = parse_ok("<html><body><h1>Chapter One</h1><p>body</p><h2>Section</h2></body></html>");
		let headings: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| matches!(marker.mtype, MarkerType::Heading1 | MarkerType::Heading2))
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(headings, vec!["Chapter One", "Section"]);
		// The TOC is a tree, so the h2 hangs off the h1 rather than sitting beside it.
		assert_eq!(doc.toc_items.len(), 1);
		assert_eq!(doc.toc_items[0].name, "Chapter One");
		assert_eq!(doc.toc_items[0].children.len(), 1);
		assert_eq!(doc.toc_items[0].children[0].name, "Section");
	}

	#[test]
	fn records_links_with_their_targets() {
		let doc = parse_ok(r#"<html><body><p><a href="https://example.com">click me</a></p></body></html>"#);
		let link = doc.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Link).expect("link marker");
		assert_eq!(link.text, "click me");
		assert_eq!(link.reference, "https://example.com");
	}

	#[test]
	fn rejects_an_empty_file() {
		let err = parse_html("page.html", "").expect_err("empty html must fail");
		assert!(err.to_string().contains("empty"), "unexpected error: {err}");
	}
}
