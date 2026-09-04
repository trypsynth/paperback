use std::fs;

use anyhow::{Context, Result};
use rtf_parser::lexer::Lexer;

use crate::{
	document::{Document, ParserContext},
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

mod encoding;
mod escapes;
mod tokens;

use encoding::{extract_codepage, extract_font_table};
use escapes::{normalize_escapes, normalize_wrapped_space_lines};
use tokens::extract_content_from_tokens;

pub struct RtfParser;

impl Parser for RtfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing rtf document");
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open RTF file '{}'", context.file_path))?;
		let content_str = String::from_utf8_lossy(&bytes);
		// Some RTF files have garbage at the end
		let content_str = content_str.trim_end_matches(|c: char| c == '\0' || c.is_whitespace());
		let content_str = normalize_wrapped_space_lines(content_str);
		let encoding = extract_codepage(&content_str);
		tracing::debug!(path = %context.file_path, encoding = %encoding.name(), "resolved rtf document encoding");
		let font_table = extract_font_table(&content_str, encoding);
		let content_str = normalize_escapes(&content_str, encoding, &font_table);
		// Strip \r so that \r\n line endings don't leave stray carriage returns in text tokens
		let content_str = content_str.replace('\r', "");
		let tokens = Lexer::scan(&content_str).map_err(|e| {
			tracing::warn!(path = %context.file_path, error = %e, "failed to scan rtf document tokens");
			// TRANSLATORS: Error shown when an RTF document's tokens fail to parse; {} is the underlying lexer error
			anyhow::anyhow!(t("Failed to parse RTF document: {}").replace("{}", &e.to_string()))
		})?;
		let buffer = extract_content_from_tokens(&tokens);
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		tracing::debug!(path = %context.file_path, "parsed rtf document successfully");
		Ok(doc)
	}
}
