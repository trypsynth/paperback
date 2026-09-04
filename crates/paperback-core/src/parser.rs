use std::{
	collections::{BTreeSet, HashMap},
	path::Path,
	string::String,
	sync::OnceLock,
};

use anyhow::Result;
use paperback_formats::FormatMeta;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	t,
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
};

pub mod chm;
pub mod convert;
pub mod daisy;
pub mod epub;
pub mod fb2;
pub mod html;
pub mod m4b;
pub mod markdown;
pub mod mobi;
pub mod odp;
pub mod odt;
pub mod pdf;
pub mod powerpoint;
pub mod rtf;
pub mod text;
pub mod util;
pub mod word;

pub const PASSWORD_REQUIRED_ERROR_PREFIX: &str = "[password_required]";

pub trait Parser: Send + Sync {
	/// Parse a document from the given context.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be read or parsed.
	fn parse(&self, context: &ParserContext) -> Result<Document>;
}

/// A parser paired with the format it was registered for.
///
/// Every fact about a format (its name, extensions and supported navigation features) lives
/// in `paperback-formats` and is attached here at registration time, so parsers themselves hold
/// no metadata that could drift from that table.
pub struct RegisteredParser {
	format: &'static FormatMeta,
	parser: Box<dyn Parser>,
}

impl RegisteredParser {
	#[must_use]
	pub const fn format(&self) -> &'static FormatMeta {
		self.format
	}

	#[must_use]
	pub const fn name(&self) -> &'static str {
		self.format.name
	}

	#[must_use]
	pub const fn extensions(&self) -> &'static [&'static str] {
		self.format.extensions
	}

	#[must_use]
	pub const fn supported_flags(&self) -> ParserFlags {
		self.format.flags
	}

	/// Parse a document from the given context.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be read or parsed.
	pub fn parse(&self, context: &ParserContext) -> Result<Document> {
		self.parser.parse(context)
	}
}

/// Builds a registry from a `FORMAT => parser` table, where `FORMAT` names a static in
/// `paperback-formats`. Registration order is the order the entries appear in: it decides
/// which parser is tried first for an extension more than one format claims, so it should
/// match the declaration order of the format table.
macro_rules! parser_registry {
	($($format:ident => $parser:expr),+ $(,)?) => {{
		let mut registry = ParserRegistry::new();
		$(registry.register(&paperback_formats::$format, $parser);)+
		registry
	}};
}

pub struct ParserRegistry {
	/// Registered parsers in registration order, which is the order they're offered to the
	/// user (file dialog filters) and tried in (extensions claimed by more than one format).
	parsers: Vec<RegisteredParser>,
	/// Lowercase extension to indices into `parsers`, preserving registration order.
	extension_map: HashMap<String, Vec<usize>>,
}

impl ParserRegistry {
	fn new() -> Self {
		Self { parsers: Vec::new(), extension_map: HashMap::new() }
	}

	pub fn register<P: Parser + 'static>(&mut self, format: &'static FormatMeta, parser: P) {
		let index = self.parsers.len();
		for ext in format.extensions {
			self.extension_map.entry(ext.to_ascii_lowercase()).or_default().push(index);
		}
		self.parsers.push(RegisteredParser { format, parser: Box::new(parser) });
	}

	#[must_use]
	pub fn get_parsers_for_extension(&self, extension: &str) -> Vec<&RegisteredParser> {
		let ext = extension.to_ascii_lowercase();
		self.extension_map
			.get(&ext)
			.map(|indices| indices.iter().map(|&index| &self.parsers[index]).collect())
			.unwrap_or_default()
	}

	#[must_use]
	pub fn all_parsers(&self) -> &[RegisteredParser] {
		&self.parsers
	}

	pub fn global() -> &'static Self {
		static REGISTRY: OnceLock<ParserRegistry> = OnceLock::new();
		REGISTRY.get_or_init(|| {
			parser_registry! {
				CHM => chm::ChmParser,
				DAISY => daisy::DaisyParser,
				WORD => word::WordParser,
				EPUB => epub::EpubParser,
				FB2 => fb2::Fb2Parser,
				HTML => html::HtmlParser,
				PDF => pdf::PdfParser,
				MARKDOWN => markdown::MarkdownParser,
				M4B => m4b::M4bParser,
				MOBI => mobi::MobiParser,
				FODP => odp::FodpParser,
				ODP => odp::OdpParser,
				FODT => odt::FodtParser,
				ODT => odt::OdtParser,
				POWERPOINT => powerpoint::PowerpointParser,
				RTF => rtf::RtfParser,
				TEXT => text::TextParser,
			}
		})
	}
}

/// The extension that selects a file's parser: ordinarily just `path`'s own extension, except a
/// loose DAISY 2.02 book's master file is named `ncc.html`, which would otherwise route to the
/// HTML parser since DAISY only claims `opf`/`zip` (an `.html` extension can't be reserved for
/// DAISY without also capturing every ordinary HTML file).
fn resolve_extension(path: &Path) -> Option<&str> {
	if path.file_name().is_some_and(|n| n.eq_ignore_ascii_case("ncc.html")) {
		return Some("opf");
	}
	path.extension().and_then(|e| e.to_str())
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
	let path = Path::new(&context.file_path);
	let extension = context.forced_extension.as_ref().map_or_else(
		|| {
			resolve_extension(path).ok_or_else(|| {
				// TRANSLATORS: Error shown when a file has no extension to determine its format; {} is the file path
				anyhow::anyhow!(t("No file extension found for: {}").replace("{}", &context.file_path))
			})
		},
		|ext| Ok(ext.as_str()),
	)?;
	let parsers = ParserRegistry::global().get_parsers_for_extension(extension);
	if parsers.is_empty() {
		// TRANSLATORS: Error shown when no parser supports a file's extension; {} is the extension (without the leading dot)
		return Err(anyhow::anyhow!(t("No parser found for extension: .{}").replace("{}", extension)));
	}
	tracing::debug!(path = %context.file_path, extension, candidates = parsers.len(), "parsing document");
	let mut last_error = None;
	for parser in parsers {
		match parser.parse(context) {
			Ok(mut doc) => {
				doc.compute_stats();
				tracing::debug!(path = %context.file_path, parser = parser.name(), "parsed document");
				return Ok(doc);
			}
			Err(e) => {
				if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					return Err(e);
				}
				tracing::warn!(path = %context.file_path, parser = parser.name(), error = %e, "parser failed, trying next");
				last_error = Some(e);
			}
		}
	}
	tracing::warn!(path = %context.file_path, extension, "all parsers failed");
	Err(last_error.unwrap_or_else(|| {
		// TRANSLATORS: Error shown when every parser for a file's extension failed; {} is the extension (without the leading dot)
		anyhow::anyhow!(t("All parsers failed for extension: .{}").replace("{}", extension))
	}))
}

#[must_use]
pub fn get_parser_flags_for_context(context: &ParserContext) -> ParserFlags {
	let path = Path::new(&context.file_path);
	let extension =
		context.forced_extension.as_ref().map_or_else(|| resolve_extension(path).unwrap_or(""), |ext| ext.as_str());
	ParserRegistry::global()
		.get_parsers_for_extension(extension)
		.iter()
		.fold(ParserFlags::NONE, |acc, p| acc | p.supported_flags())
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
	!ParserRegistry::global().get_parsers_for_extension(&normalized).is_empty()
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
	for parser in parsers {
		for ext in parser.extensions() {
			if !ext.is_empty() {
				all_extensions.insert(*ext);
			}
		}
	}
	let mut parts = String::new();
	let all_ext_part = join_extensions(all_extensions.iter().copied());
	if !all_ext_part.is_empty() {
		parts.push_str("All Supported Files (");
		parts.push_str(&all_ext_part);
		parts.push_str(")|");
		parts.push_str(&all_ext_part);
		parts.push('|');
	}
	for parser in parsers {
		if parser.extensions().is_empty() {
			continue;
		}
		let ext_part = join_extensions(parser.extensions().iter().copied());
		if ext_part.is_empty() {
			continue;
		}
		parts.push_str(parser.name());
		parts.push_str(" (");
		parts.push_str(&ext_part);
		parts.push_str(")|");
		parts.push_str(&ext_part);
		parts.push('|');
	}
	// On macOS, NSOpenPanel treats *.*  as "allow everything", which disables
	// filtering for all groups.  Only add the catch-all on other platforms.
	#[cfg(not(target_os = "macos"))]
	parts.push_str("All Files (*.*)|*.*");
	#[cfg(target_os = "macos")]
	{
		// Remove the trailing '|' left by the last per-parser entry.
		if parts.ends_with('|') {
			parts.pop();
		}
	}
	parts
}

pub trait ConverterOutput {
	fn get_headings(&self) -> &[HeadingInfo];
	fn get_links(&self) -> &[LinkInfo];
	fn get_images(&self) -> &[ImageInfo];
	fn get_figures(&self) -> &[ImageInfo];
	fn get_tables(&self) -> &[TableInfo];
	fn get_separators(&self) -> &[SeparatorInfo];
	fn get_lists(&self) -> &[ListInfo];
	fn get_list_items(&self) -> &[ListItemInfo];
	fn get_bolds(&self) -> &[FormatInfo];
	fn get_italics(&self) -> &[FormatInfo];
	fn get_underlines(&self) -> &[FormatInfo];
}

fn add_headings(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for heading in converter.get_headings() {
		let marker_type = util::toc::heading_level_to_marker_type(heading.level);
		buffer.add_marker(
			Marker::new(marker_type, offset + heading.offset).with_text(heading.text.clone()).with_level(heading.level),
		);
	}
}

fn add_links(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for link in converter.get_links() {
		buffer.add_marker(
			Marker::new(MarkerType::Link, offset + link.offset)
				.with_text(link.text.clone())
				.with_reference(link.reference.clone()),
		);
	}
}

fn add_images(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for image in converter.get_images() {
		buffer.add_marker(Marker::new(MarkerType::Image, offset + image.offset).with_text(image.alt_text.clone()));
	}
}

fn add_figures(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for figure in converter.get_figures() {
		buffer.add_marker(Marker::new(MarkerType::Figure, offset + figure.offset).with_text(figure.alt_text.clone()));
	}
}

fn add_tables_separators_lists(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for table in converter.get_tables() {
		buffer.add_marker(
			Marker::new(MarkerType::Table, offset + table.offset)
				.with_text(table.text.clone())
				.with_reference(table.html_content.clone())
				.with_length(table.length),
		);
	}
	for separator in converter.get_separators() {
		buffer.add_marker(
			Marker::new(MarkerType::Separator, offset + separator.offset)
				.with_text("Separator".to_string())
				.with_length(separator.length),
		);
	}
	for list in converter.get_lists() {
		buffer.add_marker(
			Marker::new(MarkerType::List, offset + list.offset).with_level(list.item_count).with_length(list.length),
		);
	}
	for list_item in converter.get_list_items() {
		buffer.add_marker(
			Marker::new(MarkerType::ListItem, offset + list_item.offset)
				.with_text(list_item.text.clone())
				.with_level(list_item.level),
		);
	}
}

fn add_formatting(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for bold in converter.get_bolds() {
		buffer.add_marker(Marker::new(MarkerType::Bold, offset + bold.offset).with_length(bold.length));
	}
	for italic in converter.get_italics() {
		buffer.add_marker(Marker::new(MarkerType::Italic, offset + italic.offset).with_length(italic.length));
	}
	for underline in converter.get_underlines() {
		buffer.add_marker(Marker::new(MarkerType::Underline, offset + underline.offset).with_length(underline.length));
	}
}

/// Transfer all converter markers to a `DocumentBuffer`.
/// `offset` is added to each marker position (for multi-section parsers like CHM/EPUB).
pub fn add_converter_markers(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	add_headings(buffer, converter, offset);
	add_links(buffer, converter, offset);
	add_images(buffer, converter, offset);
	add_figures(buffer, converter, offset);
	add_tables_separators_lists(buffer, converter, offset);
	add_formatting(buffer, converter, offset);
}

/// Like `add_converter_markers` but excludes links, for parsers that resolve link hrefs specially.
pub fn add_converter_markers_excluding_links(
	buffer: &mut DocumentBuffer,
	converter: &dyn ConverterOutput,
	offset: usize,
) {
	add_headings(buffer, converter, offset);
	add_images(buffer, converter, offset);
	add_figures(buffer, converter, offset);
	add_tables_separators_lists(buffer, converter, offset);
	add_formatting(buffer, converter, offset);
}

#[must_use]
pub fn is_external_url(url: &str) -> bool {
	let lower = url.to_ascii_lowercase();
	lower.starts_with("http:") || lower.starts_with("https:") || lower.starts_with("mailto:")
}

#[cfg(test)]
mod tests;
