//! Hand-written because `rst_parser`/`rst_renderer` panic via `unimplemented!()` on broken inline markup and Sphinx substitution references, so everything here degrades rather than fails.
//! Unsupported: docutils' section nesting (adornments map to levels in first-seen order), grid and simple tables (shown as source), and footnote bodies (`[1]_` renders as an unlinked `[1]`).

use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{DocPos, Document, DocumentBuffer, ParserContext},
	parser::{
		Parser, add_converter_markers,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	util::encoding::convert_to_utf8,
};

pub struct RstParser;

impl Parser for RstParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing reStructuredText file");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open reStructuredText file '{}'", context.file_path))?;
		let source = convert_to_utf8(&bytes);
		let html_content = translate::rst_to_html(&source);
		let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
		converter.convert(&html_content, HtmlSourceMode::NativeHtml);
		// A title opening the document is its title, as docutils reads it.
		let title = converter
			.get_headings()
			.first()
			.filter(|heading| heading.offset == DocPos::ZERO)
			.map_or_else(|| extract_title_from_path(&context.file_path), |heading| heading.text.clone());
		let mut buffer = DocumentBuffer::with_content(converter.take_text());
		add_converter_markers(&mut buffer, &converter, DocPos::ZERO);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title).with_author(author(&source));
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions.clone_from(converter.get_id_positions());
		tracing::debug!(path = %context.file_path, chars = doc.buffer.content.chars().count(), "parsed reStructuredText file");
		Ok(doc)
	}
}

fn author(source: &str) -> String {
	source
		.lines()
		.find_map(|line| {
			let (name, value) = line.strip_prefix(':')?.split_once(':')?;
			(name.eq_ignore_ascii_case("author") || name.eq_ignore_ascii_case("authors"))
				.then(|| value.trim().to_string())
		})
		.unwrap_or_default()
}

pub mod translate;

#[cfg(test)]
mod tests;
