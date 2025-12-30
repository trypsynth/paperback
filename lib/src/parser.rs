use std::{collections::HashMap, sync::OnceLock};

use anyhow::Result;

use crate::document::{Document, ParserContext, ParserFlags};

pub mod chm;
pub mod docx;
pub mod epub;
pub mod fb2;
pub mod html;
pub mod markdown;
pub mod odp;
pub mod odt;
pub mod pdf;
pub mod pptx;
pub mod text;
pub(crate) mod utils;

pub const PASSWORD_REQUIRED_ERROR_PREFIX: &str = "[password_required]";

pub trait Parser: Send + Sync {
	fn name(&self) -> &str;
	fn extensions(&self) -> &[&str];
	fn supported_flags(&self) -> ParserFlags;
	/// Parse a document from the given context.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be read or parsed.
	fn parse(&self, context: &ParserContext) -> Result<Document>;
}

#[derive(Clone)]
pub struct ParserInfo {
	pub name: String,
	pub extensions: Vec<String>,
	pub flags: ParserFlags,
}

pub struct ParserRegistry {
	parsers: HashMap<String, Box<dyn Parser>>,
	extension_map: HashMap<String, String>,
}

impl ParserRegistry {
	fn new() -> Self {
		Self { parsers: HashMap::new(), extension_map: HashMap::new() }
	}

	pub fn register<P: Parser + 'static>(&mut self, parser: P) {
		let name = parser.name().to_string();
		for ext in parser.extensions() {
			self.extension_map.insert(ext.to_ascii_lowercase(), name.clone());
		}
		self.parsers.insert(name, Box::new(parser));
	}

	#[must_use]
	pub fn get_parser(&self, name: &str) -> Option<&dyn Parser> {
		self.parsers.get(name).map(|p| &**p)
	}

	#[must_use]
	pub fn get_parser_for_extension(&self, extension: &str) -> Option<&dyn Parser> {
		let ext = extension.to_ascii_lowercase();
		self.extension_map.get(&ext).and_then(|name| self.parsers.get(name)).map(|parser| &**parser)
	}

	#[must_use]
	pub fn all_parsers(&self) -> Vec<ParserInfo> {
		self.parsers
			.values()
			.map(|p| ParserInfo {
				name: p.name().to_string(),
				extensions: p.extensions().iter().map(|s| (*s).to_string()).collect(),
				flags: p.supported_flags(),
			})
			.collect()
	}

	pub fn global() -> &'static Self {
		static REGISTRY: OnceLock<ParserRegistry> = OnceLock::new();
		REGISTRY.get_or_init(|| {
			let mut registry = Self::new();
			registry.register(chm::ChmParser);
			registry.register(docx::DocxParser);
			registry.register(epub::EpubParser);
			registry.register(fb2::Fb2Parser);
			registry.register(html::HtmlParser);
			registry.register(pdf::PdfParser);
			registry.register(markdown::MarkdownParser);
			registry.register(odp::OdpParser);
			registry.register(odt::OdtParser);
			registry.register(pptx::PptxParser);
			registry.register(text::TextParser);
			registry
		})
	}
}

/// Parse a document from the given context.
///
/// # Errors
///
/// Returns an error if:
/// - No file extension is found
/// - No parser is available for the file extension
/// - The parser fails to parse the file
pub fn parse_document(context: &ParserContext) -> Result<Document> {
	let path = std::path::Path::new(&context.file_path);
	let extension = if let Some(ext) = &context.forced_extension {
		ext.as_str()
	} else {
		path.extension()
			.and_then(|e| e.to_str())
			.ok_or_else(|| anyhow::anyhow!("No file extension found for: {}", context.file_path))?
	};
	let parser = ParserRegistry::global()
		.get_parser_for_extension(extension)
		.ok_or_else(|| anyhow::anyhow!("No parser found for extension: .{extension}"))?;
	let mut doc = parser.parse(context)?;
	doc.compute_stats();
	Ok(doc)
}

#[must_use]
pub fn get_all_parsers() -> Vec<ParserInfo> {
	ParserRegistry::global().all_parsers()
}

#[must_use]
pub fn get_parser_name_for_extension(extension: &str) -> Option<String> {
	ParserRegistry::global().get_parser_for_extension(extension).map(|p| p.name().to_string())
}

#[must_use]
pub fn get_parser_flags_for_context(context: &ParserContext) -> ParserFlags {
	let path = std::path::Path::new(&context.file_path);
	let extension = if let Some(ext) = &context.forced_extension {
		ext.as_str()
	} else {
		path.extension().and_then(|e| e.to_str()).unwrap_or("")
	};
	ParserRegistry::global()
		.get_parser_for_extension(extension)
		.map(|p| p.supported_flags())
		.unwrap_or(ParserFlags::NONE)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn registry_has_parsers() {
		let parsers = get_all_parsers();
		assert!(!parsers.is_empty(), "Registry should have parsers");
	}

	#[test]
	fn test_extension_lookup() {
		assert!(get_parser_name_for_extension("epub").is_some());
		assert!(get_parser_name_for_extension("html").is_some());
		assert!(get_parser_name_for_extension("md").is_some());
		assert!(get_parser_name_for_extension("txt").is_some());
		assert!(get_parser_name_for_extension("zizzy").is_none());
		assert!(get_parser_name_for_extension("").is_none());
	}
}
