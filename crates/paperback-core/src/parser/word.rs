use std::{
	fs::{self, File},
	io::Read,
	path::Path,
};

use anyhow::{Context, Result};

use crate::{
	document::{Document, ParserContext},
	parser::{
		Parser,
		rtf::{RtfParser, looks_like_rtf},
		util::ole::apply_summary_information,
		wri::{WriParser, is_write_magic},
	},
	t,
};

mod legacy;
mod numbering;
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
			// A file that can't be read here fails properly in the OLE path below.
			let mut head = Vec::with_capacity(64);
			if let Ok(file) = File::open(&context.file_path) {
				let _ = file.take(64).read_to_end(&mut head);
			}
			let magic = head.get(..2).map_or(0, |bytes| u16::from_le_bytes([bytes[0], bytes[1]]));
			if legacy::is_winword_magic(magic) {
				let data = fs::read(&context.file_path)
					.with_context(|| format!("Failed to read DOC file '{}'", context.file_path))?;
				return legacy::parse_winword_doc(context, &data);
			}
			if is_write_magic(magic) {
				return WriParser.parse(context);
			}
			// Word saves RTF under a `.doc` name when asked to, and opens it back as RTF.
			if looks_like_rtf(&head) {
				return RtfParser.parse(context);
			}
			match legacy::parse_legacy_doc(context) {
				Ok(mut document) => {
					apply_summary_information(&mut document, &context.file_path);
					return Ok(document);
				}
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

#[cfg(test)]
mod tests;
