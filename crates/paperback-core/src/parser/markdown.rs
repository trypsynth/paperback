use std::fs;

use anyhow::{Context, Result};
use pulldown_cmark::{Event, Options, Parser as MarkdownParserImpl, Tag, html::push_html};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	parser::{
		Parser, add_converter_markers,
		html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	util::encoding::convert_to_utf8,
};

/// Converts Markdown to HTML with an empty `<span id="pb-block-N"></span>` before each block.
///
/// The anchors produce no text but give every block a stable id, so a position
/// in the converted text can be mapped back to a `#fragment` when the document
/// is shown in a web view.
#[must_use]
pub fn markdown_to_html(markdown_text: &str) -> String {
	let mut options = Options::empty();
	options.insert(Options::ENABLE_TABLES);
	let parser = MarkdownParserImpl::new_ext(markdown_text, options);
	let mut block_counter = 0usize;
	let events = parser.flat_map(|event| {
		let anchor = match &event {
			Event::Start(Tag::Paragraph | Tag::Heading { .. } | Tag::Item | Tag::BlockQuote(_) | Tag::CodeBlock(_)) => {
				block_counter += 1;
				Some(Event::Html(format!("<span id=\"pb-block-{block_counter}\"></span>").into()))
			}
			_ => None,
		};
		anchor.into_iter().chain(std::iter::once(event))
	});
	let mut html_content = String::new();
	push_html(&mut html_content, events);
	html_content
}

pub struct MarkdownParser;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn markdown_to_html_injects_block_anchors() {
		let html = markdown_to_html("# Title\n\nFirst paragraph.\n\nSecond paragraph.\n");
		let anchor_2 = html.find(r#"<span id="pb-block-2"></span>"#).expect("second anchor present");
		let anchor_3 = html.find(r#"<span id="pb-block-3"></span>"#).expect("third anchor present");
		let first_para = html.find("<p>First paragraph.</p>").expect("first paragraph present");
		let second_para = html.find("<p>Second paragraph.</p>").expect("second paragraph present");
		assert!(html.contains(r#"<span id="pb-block-1"></span>"#), "got: {html}");
		assert!(anchor_2 < first_para && first_para < anchor_3 && anchor_3 < second_para, "got: {html}");
	}

	#[test]
	fn markdown_block_anchors_reach_id_positions_without_changing_text() {
		let source = "# Title\n\nFirst paragraph.\n\nSecond paragraph.\n";
		let html = markdown_to_html(source);
		let mut converter = HtmlToText::new();
		assert!(converter.convert(&html, HtmlSourceMode::Markdown));
		let text = converter.get_text();
		let ids = converter.get_id_positions();
		assert_eq!(ids.get("pb-block-1"), Some(&text.find("Title").unwrap()), "ids: {ids:?} text: {text:?}");
		assert_eq!(ids.get("pb-block-3"), Some(&text.find("Second").unwrap()), "ids: {ids:?} text: {text:?}");
		assert!(!text.contains("pb-block"), "anchors must not leak into text: {text:?}");
	}
}

impl Parser for MarkdownParser {
	fn name(&self) -> &'static str {
		"Markdown Files"
	}

	fn extensions(&self) -> &[&str] {
		&["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open Markdown file '{}'", context.file_path))?;
		let markdown_content = convert_to_utf8(&bytes);
		let html_content = markdown_to_html(&markdown_content);
		let mut converter = HtmlToText::new();
		if !converter.convert(&html_content, HtmlSourceMode::Markdown) {
			anyhow::bail!("Failed to convert Markdown to text: {}", context.file_path);
		}
		let title = extract_title_from_path(&context.file_path);
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
