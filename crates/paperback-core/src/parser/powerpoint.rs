use std::path::Path;

use anyhow::Result;

use crate::{
	document::{Document, ParserContext},
	parser::Parser,
};

mod legacy;
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
			return legacy::parse_legacy_ppt(context);
		}
		tracing::debug!(path = %context.file_path, "parsing powerpoint file as pptx");
		pptx::parse_pptx(context)
	}
}
