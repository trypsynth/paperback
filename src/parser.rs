use std::{
	collections::{BTreeSet, HashMap},
	sync::OnceLock,
};

use anyhow::Result;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	types::{HeadingInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
};

pub mod chm;
pub mod docx;
pub mod epub;
pub mod fb2;
pub mod html;
pub mod markdown;
pub mod odp;
pub mod odt;
pub mod ooxml;
pub mod path;
pub mod pdf;
pub mod pptx;
pub mod rtf;
pub mod text;
pub mod toc;
pub mod xml;
pub mod xml_doc;

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
			registry.register(xml_doc::XmlParser);
			registry.register(pdf::PdfParser);
			registry.register(markdown::MarkdownParser);
			registry.register(odp::OdpParser);
			registry.register(odt::OdtParser);
			registry.register(pptx::PptxParser);
			registry.register(rtf::RtfParser);
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
	fn get_tables(&self) -> &[TableInfo];
	fn get_separators(&self) -> &[SeparatorInfo];
	fn get_lists(&self) -> &[ListInfo];
	fn get_list_items(&self) -> &[ListItemInfo];
}

fn add_headings(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	for heading in converter.get_headings() {
		let marker_type = toc::heading_level_to_marker_type(heading.level);
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
		buffer.add_marker(Marker::new(MarkerType::List, offset + list.offset).with_level(list.item_count));
	}
	for list_item in converter.get_list_items() {
		buffer.add_marker(
			Marker::new(MarkerType::ListItem, offset + list_item.offset)
				.with_text(list_item.text.clone())
				.with_level(list_item.level),
		);
	}
}

/// Transfer all converter markers to a `DocumentBuffer`.
/// `offset` is added to each marker position (for multi-section parsers like CHM/EPUB).
pub fn add_converter_markers(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: usize) {
	add_headings(buffer, converter, offset);
	add_links(buffer, converter, offset);
	add_tables_separators_lists(buffer, converter, offset);
}

/// Like `add_converter_markers` but excludes links, for parsers that resolve link hrefs specially.
pub fn add_converter_markers_excluding_links(
	buffer: &mut DocumentBuffer,
	converter: &dyn ConverterOutput,
	offset: usize,
) {
	add_headings(buffer, converter, offset);
	add_tables_separators_lists(buffer, converter, offset);
}

#[must_use]
pub fn is_external_url(url: &str) -> bool {
	let lower = url.to_ascii_lowercase();
	lower.starts_with("http:") || lower.starts_with("https:") || lower.starts_with("mailto:")
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::types::{HeadingInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo};

	struct MockConverter {
		headings: Vec<HeadingInfo>,
		links: Vec<LinkInfo>,
		tables: Vec<TableInfo>,
		separators: Vec<SeparatorInfo>,
		lists: Vec<ListInfo>,
		list_items: Vec<ListItemInfo>,
	}

	impl ConverterOutput for MockConverter {
		fn get_headings(&self) -> &[HeadingInfo] {
			&self.headings
		}

		fn get_links(&self) -> &[LinkInfo] {
			&self.links
		}

		fn get_tables(&self) -> &[TableInfo] {
			&self.tables
		}

		fn get_separators(&self) -> &[SeparatorInfo] {
			&self.separators
		}

		fn get_lists(&self) -> &[ListInfo] {
			&self.lists
		}

		fn get_list_items(&self) -> &[ListItemInfo] {
			&self.list_items
		}
	}

	fn sample_converter() -> MockConverter {
		MockConverter {
			headings: vec![HeadingInfo { offset: 1, level: 2, text: "Heading".to_string() }],
			links: vec![LinkInfo { offset: 2, text: "Link".to_string(), reference: "#a".to_string() }],
			tables: vec![TableInfo {
				offset: 3,
				text: "T".to_string(),
				html_content: "<table/>".to_string(),
				length: 11,
			}],
			separators: vec![SeparatorInfo { offset: 4, length: 7 }],
			lists: vec![ListInfo { offset: 5, item_count: 3 }],
			list_items: vec![ListItemInfo { offset: 6, level: 1, text: "Item".to_string() }],
		}
	}

	#[test]
	fn join_extensions_formats_and_skips_empty_entries() {
		let joined = join_extensions(["txt", "", "md"]);
		assert_eq!(joined, "*.txt;*.md");
	}

	#[test]
	fn join_extensions_returns_empty_for_empty_input() {
		let joined = join_extensions(std::iter::empty::<&str>());
		assert_eq!(joined, "");
	}

	#[rstest]
	#[case("http://example.com", true)]
	#[case("HTTPS://example.com", true)]
	#[case("MailTo:test@example.com", true)]
	#[case("ftp://example.com", false)]
	#[case("#local", false)]
	#[case("https//example.com", false)]
	#[case("mailtox:test@example.com", false)]
	#[case("httpx://example.com", false)]
	fn is_external_url_classifies_schemes(#[case] url: &str, #[case] expected: bool) {
		assert_eq!(is_external_url(url), expected);
	}

	#[rstest]
	#[case("txt", true)]
	#[case(".TXT", true)]
	#[case("log", true)]
	#[case("", false)]
	#[case(".", false)]
	#[case("notarealextension", false)]
	#[case(" txt", false)]
	#[case("txt ", false)]
	#[case("..txt", true)]
	#[case("...log", true)]
	fn parser_supports_extension_classifies_inputs(#[case] extension: &str, #[case] expected: bool) {
		assert_eq!(parser_supports_extension(extension), expected);
	}

	#[test]
	fn file_filter_string_contains_supported_and_fallback_groups() {
		let filter = build_file_filter_string();
		assert!(filter.contains("All Supported Files ("));
		assert!(filter.contains("*.txt"));
		assert!(filter.contains("*.epub"));
		#[cfg(not(target_os = "macos"))]
		assert!(filter.ends_with("All Files (*.*)|*.*"));
	}

	#[test]
	fn add_converter_markers_transfers_all_marker_types_with_offset() {
		let converter = sample_converter();
		let mut buffer = DocumentBuffer::new();
		add_converter_markers(&mut buffer, &converter, 100);
		assert_eq!(buffer.markers.len(), 6);
		assert_eq!(buffer.markers[0].mtype, MarkerType::Heading2);
		assert_eq!(buffer.markers[0].position, 101);
		assert_eq!(buffer.markers[0].text, "Heading");
		assert_eq!(buffer.markers[1].mtype, MarkerType::Link);
		assert_eq!(buffer.markers[1].position, 102);
		assert_eq!(buffer.markers[1].reference, "#a");
		assert_eq!(buffer.markers[2].mtype, MarkerType::Table);
		assert_eq!(buffer.markers[2].length, 11);
		assert_eq!(buffer.markers[3].mtype, MarkerType::Separator);
		assert_eq!(buffer.markers[3].length, 7);
		assert_eq!(buffer.markers[4].mtype, MarkerType::List);
		assert_eq!(buffer.markers[4].level, 3);
		assert_eq!(buffer.markers[5].mtype, MarkerType::ListItem);
		assert_eq!(buffer.markers[5].level, 1);
	}

	#[test]
	fn add_converter_markers_excluding_links_skips_link_markers() {
		let converter = sample_converter();
		let mut buffer = DocumentBuffer::new();
		add_converter_markers_excluding_links(&mut buffer, &converter, 10);
		assert_eq!(buffer.markers.len(), 5);
		assert!(buffer.markers.iter().all(|marker| marker.mtype != MarkerType::Link));
	}

	#[test]
	fn add_converter_markers_handles_empty_converter_output() {
		let converter = MockConverter {
			headings: vec![],
			links: vec![],
			tables: vec![],
			separators: vec![],
			lists: vec![],
			list_items: vec![],
		};
		let mut buffer = DocumentBuffer::new();
		add_converter_markers(&mut buffer, &converter, 0);
		assert!(buffer.markers.is_empty());
	}

	#[test]
	fn parse_document_errors_when_missing_extension() {
		let context = ParserContext::new("no_extension".to_string());
		let err = parse_document(&context).expect_err("expected missing extension error");
		assert!(err.to_string().contains("No file extension found"));
	}

	#[test]
	fn parse_document_errors_for_unknown_forced_extension() {
		let context = ParserContext::new("anything".to_string()).with_forced_extension("unknown_ext".to_string());
		let err = parse_document(&context).expect_err("expected unknown parser error");
		assert!(err.to_string().contains("No parser found for extension"));
	}

	#[test]
	fn get_parser_flags_for_context_returns_none_for_unknown_extension() {
		let context = ParserContext::new("doc.unknown_ext".to_string());
		assert_eq!(get_parser_flags_for_context(&context), ParserFlags::NONE);
	}

	#[test]
	fn file_filter_string_contains_text_files_group_name() {
		let filter = build_file_filter_string();
		assert!(filter.contains("Text Files ("));
	}
}
