use std::path::Path;

use anyhow::Result;

use crate::{
	document::{Document, ParserContext},
	parser::{Parser, util::ole::apply_summary_information},
};

mod legacy;
mod persist;
mod pp95;
mod pptx;

pub struct PowerpointParser;

impl Parser for PowerpointParser {
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
		if extension == "ppt" {
			tracing::debug!(path = %context.file_path, "parsing powerpoint file as legacy ppt");
			let mut document = legacy::parse_legacy_ppt(context)?;
			apply_summary_information(&mut document, &context.file_path);
			return Ok(document);
		}
		tracing::debug!(path = %context.file_path, "parsing powerpoint file as pptx");
		pptx::parse_pptx(context)
	}
}
