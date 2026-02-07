use std::fs;

use anyhow::{Context, Result};
use pulldown_cmark::{Options, Parser as MarkdownParserImpl, html::push_html};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	encoding::convert_to_utf8,
	html_to_text::{HtmlSourceMode, HtmlToText},
	parser::{Parser, add_converter_markers, path::extract_title_from_path, toc::build_toc_from_headings},
};

pub struct MarkdownParser;

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
		let mut options = Options::empty();
		options.insert(Options::ENABLE_TABLES);
		let parser = MarkdownParserImpl::new_ext(&markdown_content, options);
		let mut html_content = String::new();
		push_html(&mut html_content, parser);
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
