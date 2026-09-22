//! reStructuredText: section titles marked by underlining (and optionally overlining) with a
//! repeated punctuation character, directives (`.. name:: argument`), and a small set of inline
//! markup (`**strong**`, `*emphasis*`, ` ``literal`` `, hyperlink references).
//!
//! This is a hand-written translator to HTML rather than a wrapper around a crate, because the
//! available Rust implementation (`rst_parser`/`rst_renderer`) panics via `unimplemented!()` on
//! constructs real documents actually contain: broken inline markup and Sphinx substitution
//! references both hit it. Everything here is written to degrade instead of failing outright: a
//! directive it doesn't recognize still shows its body, a reference to an undefined target is
//! shown as plain text, and no input should ever panic it. The produced HTML is fed through the
//! same [`HtmlToText`](super::convert::html_to_text::HtmlToText) converter Markdown uses, which is
//! where headings, links, lists and formatting turn into the document's markers.
//!
//! Left unsupported: the docutils section-nesting algorithm proper (adornment characters are
//! mapped to heading levels in first-seen order instead, which matches the common convention of
//! using one character per level consistently), grid and simple tables (rendered as their literal
//! source lines rather than parsed), and footnote/citation bodies (a `[1]_` reference renders as
//! `[1]` with no link to where it's defined).

use std::fs;

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext},
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
		let title = extract_title_from_path(&context.file_path);
		let mut buffer = DocumentBuffer::with_content(converter.get_text());
		add_converter_markers(&mut buffer, &converter, 0);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions.clone_from(converter.get_id_positions());
		tracing::debug!(path = %context.file_path, chars = doc.buffer.content.chars().count(), "parsed reStructuredText file");
		Ok(doc)
	}
}

pub mod translate;

#[cfg(test)]
mod tests;
