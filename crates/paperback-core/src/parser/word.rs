use std::path::Path;

use anyhow::Result;

use crate::{
	document::{Document, ParserContext},
	parser::Parser,
	t,
};

mod legacy;
pub mod ooxml;

pub struct WordParser;

impl Parser for WordParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let extension = context.forced_extension.as_ref().map_or_else(
			|| {
				Path::new(&context.file_path)
					.extension()
					.and_then(|ext| ext.to_str())
					.unwrap_or_default()
					.to_ascii_lowercase()
			},
			|ext| ext.to_ascii_lowercase(),
		);
		let render_tables_inline = context.render_tables_inline;
		tracing::debug!(path = %context.file_path, extension = %extension, "resolved word parser branch");
		if extension == "zip" {
			tracing::debug!(path = %context.file_path, "treating file as a batch zip of embedded docx documents");
			return ooxml::parse_word_zip(context, render_tables_inline);
		}
		if extension == "doc" {
			match legacy::parse_legacy_doc(context) {
				Ok(document) => return Ok(document),
				Err(legacy_err) => {
					tracing::warn!(path = %context.file_path, error = %legacy_err, "legacy doc parsing failed, falling back to ooxml parsing");
					match ooxml::parse_ooxml_doc(context, render_tables_inline) {
						Ok(document) => return Ok(document),
						Err(ooxml_err) => {
							tracing::warn!(path = %context.file_path, error = %ooxml_err, "ooxml fallback parsing failed, falling back to text-like parsing");
							if let Ok(document) = legacy::parse_text_like_doc(context) {
								return Ok(document);
							}
							// TRANSLATORS: Error shown when both DOC parsing strategies fail; the two {} are the underlying error details
							let msg = t("Legacy DOC parse failed: {}. OOXML fallback failed: {}")
								.replacen("{}", &legacy_err.to_string(), 1)
								.replacen("{}", &ooxml_err.to_string(), 1);
							return Err(anyhow::anyhow!(msg));
						}
					}
				}
			}
		}
		ooxml::parse_ooxml_doc(context, render_tables_inline)
	}
}
