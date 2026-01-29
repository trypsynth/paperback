use std::{
	collections::{BTreeSet, HashMap},
	sync::OnceLock,
};

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

pub struct ParserInfo {
	pub name: String,
	pub extensions: Vec<String>,
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
	let extension = context.forced_extension.as_ref().map_or_else(
		|| {
			path.extension()
				.and_then(|e| e.to_str())
				.ok_or_else(|| anyhow::anyhow!("No file extension found for: {}", context.file_path))
		},
		|ext| Ok(ext.as_str()),
	)?;
	let parser = ParserRegistry::global()
		.get_parser_for_extension(extension)
		.ok_or_else(|| anyhow::anyhow!("No parser found for extension: .{extension}"))?;
	let mut doc = parser.parse(context)?;
	doc.compute_stats();
	Ok(doc)
}

#[must_use]
pub fn get_parser_flags_for_context(context: &ParserContext) -> ParserFlags {
	let path = std::path::Path::new(&context.file_path);
	let extension = context
		.forced_extension
		.as_ref()
		.map_or_else(|| path.extension().and_then(|e| e.to_str()).unwrap_or(""), |ext| ext.as_str());
	ParserRegistry::global().get_parser_for_extension(extension).map_or(ParserFlags::NONE, Parser::supported_flags)
}

#[must_use]
pub fn parser_supports_extension(extension: &str) -> bool {
	if extension.is_empty() {
		return false;
	}
	let normalized = extension.trim_start_matches('.').to_ascii_lowercase();
	if normalized.is_empty() {
		return false;
	}
	ParserRegistry::global().get_parser_for_extension(&normalized).is_some()
}

fn join_extensions<'a, I>(exts: I) -> String
where
	I: IntoIterator<Item = &'a str>,
{
	exts.into_iter().filter(|ext| !ext.is_empty()).map(|ext| format!("*.{ext}")).collect::<Vec<_>>().join(";")
}

#[must_use]
pub fn build_file_filter_string() -> String {
	let parsers = ParserRegistry::global().all_parsers();
	if parsers.is_empty() {
		return "All Files (*.*)|*.*".to_string();
	}
	let mut all_extensions = BTreeSet::new();
	for parser in &parsers {
		for ext in &parser.extensions {
			if !ext.is_empty() {
				all_extensions.insert(ext.clone());
			}
		}
	}
	let mut parts = String::new();
	let all_ext_part = join_extensions(all_extensions.iter().map(std::string::String::as_str));
	if !all_ext_part.is_empty() {
		parts.push_str("All Supported Files (");
		parts.push_str(&all_ext_part);
		parts.push_str(")|");
		parts.push_str(&all_ext_part);
		parts.push('|');
	}
	for parser in &parsers {
		if parser.extensions.is_empty() {
			continue;
		}
		let ext_part = join_extensions(parser.extensions.iter().map(std::string::String::as_str));
		if ext_part.is_empty() {
			continue;
		}
		parts.push_str(&parser.name);
		parts.push_str(" (");
		parts.push_str(&ext_part);
		parts.push_str(")|");
		parts.push_str(&ext_part);
		parts.push('|');
	}
	parts.push_str("All Files (*.*)|*.*");
	parts
}
