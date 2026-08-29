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
/// Every fact about a format — its name, extensions and supported navigation features — lives
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
mod tests {
	use std::{iter, ptr};

	use rstest::rstest;

	use super::*;
	use crate::types::{FormatInfo, HeadingInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo};

	struct MockConverter {
		headings: Vec<HeadingInfo>,
		links: Vec<LinkInfo>,
		images: Vec<ImageInfo>,
		figures: Vec<ImageInfo>,
		tables: Vec<TableInfo>,
		separators: Vec<SeparatorInfo>,
		lists: Vec<ListInfo>,
		list_items: Vec<ListItemInfo>,
		bolds: Vec<FormatInfo>,
		italics: Vec<FormatInfo>,
		underlines: Vec<FormatInfo>,
	}

	impl ConverterOutput for MockConverter {
		fn get_headings(&self) -> &[HeadingInfo] {
			&self.headings
		}

		fn get_links(&self) -> &[LinkInfo] {
			&self.links
		}

		fn get_images(&self) -> &[ImageInfo] {
			&self.images
		}

		fn get_figures(&self) -> &[ImageInfo] {
			&self.figures
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

		fn get_bolds(&self) -> &[FormatInfo] {
			&self.bolds
		}

		fn get_italics(&self) -> &[FormatInfo] {
			&self.italics
		}

		fn get_underlines(&self) -> &[FormatInfo] {
			&self.underlines
		}
	}
	fn sample_converter() -> MockConverter {
		MockConverter {
			headings: vec![HeadingInfo { offset: 1, level: 2, text: "Heading".to_string() }],
			links: vec![LinkInfo { offset: 2, text: "Link".to_string(), reference: "#a".to_string() }],
			images: vec![],
			figures: vec![],
			tables: vec![TableInfo {
				offset: 3,
				text: "T".to_string(),
				html_content: "<table/>".to_string(),
				length: 11,
			}],
			separators: vec![SeparatorInfo { offset: 4, length: 7 }],
			lists: vec![ListInfo { offset: 5, item_count: 3, length: 4 }],
			list_items: vec![ListItemInfo { offset: 6, level: 1, text: "Item".to_string() }],
			bolds: vec![],
			italics: vec![],
			underlines: vec![],
		}
	}

	#[test]
	fn join_extensions_formats_and_skips_empty_entries() {
		let joined = join_extensions(["txt", "", "md"]);
		assert_eq!(joined, "*.txt;*.md");
	}

	#[test]
	fn join_extensions_returns_empty_for_empty_input() {
		let joined = join_extensions(iter::empty::<&str>());
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
			images: vec![],
			figures: vec![],
			tables: vec![],
			separators: vec![],
			lists: vec![],
			list_items: vec![],
			bolds: vec![],
			italics: vec![],
			underlines: vec![],
		};
		let mut buffer = DocumentBuffer::new();
		add_converter_markers(&mut buffer, &converter, 0);
		assert!(buffer.markers.is_empty());
	}

	#[test]
	fn resolve_extension_routes_loose_ncc_html_to_opf() {
		let book_directory = Path::new("books").join("daisy2");
		assert_eq!(resolve_extension(&book_directory.join("ncc.html")), Some("opf"));
		assert_eq!(resolve_extension(&book_directory.join("NCC.HTML")), Some("opf"));
	}

	#[test]
	fn resolve_extension_leaves_other_html_files_alone() {
		let chapter = Path::new("books").join("chapter1.html");
		assert_eq!(resolve_extension(&chapter), Some("html"));
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

	/// Guards against a format being declared in `paperback-formats` with no parser wired up
	/// in `parser_registry!` (it would show up in the installer and open dialog but fail to
	/// open), and against a parser being registered twice.
	#[test]
	fn every_declared_format_has_exactly_one_parser() {
		let registered: Vec<&'static FormatMeta> =
			ParserRegistry::global().all_parsers().iter().map(RegisteredParser::format).collect();
		for format in paperback_formats::ALL {
			let count = registered.iter().filter(|candidate| ptr::eq(**candidate, *format)).count();
			assert_eq!(count, 1, "{} must have exactly one registered parser, found {count}", format.name);
		}
		assert_eq!(registered.len(), paperback_formats::ALL.len(), "a parser was registered for an unlisted format");
	}

	/// Registration order decides which parser wins a shared extension, so it must track the
	/// order of the format table rather than drifting from it.
	#[test]
	fn registration_order_matches_the_format_table() {
		let registered: Vec<&str> = ParserRegistry::global().all_parsers().iter().map(RegisteredParser::name).collect();
		let declared: Vec<&str> = paperback_formats::ALL.iter().map(|format| format.name).collect();
		assert_eq!(registered, declared);
	}

	/// `.zip` is claimed by both DAISY and Word, and `parse_document` tries claimants in
	/// registration order, so DAISY must stay ahead of Word.
	#[test]
	fn zip_is_offered_to_daisy_before_word() {
		let names: Vec<_> =
			ParserRegistry::global().get_parsers_for_extension("zip").iter().map(|p| p.name()).collect();
		assert_eq!(names, vec![paperback_formats::DAISY.name, paperback_formats::WORD.name]);
	}

	/// The registry stores parsers in a `Vec`, so the file dialog's filter groups appear in a
	/// stable order rather than whatever order a hash map happened to yield.
	#[test]
	fn file_filter_lists_groups_in_registration_order() {
		let filter = build_file_filter_string();
		// Groups are "Label (*.ext;…)|*.ext;…" pairs, so every other field is a label.
		let labels: Vec<&str> =
			filter.split('|').step_by(2).filter_map(|group| group.split_once(" (").map(|(label, _)| label)).collect();
		let expected: Vec<&str> = ParserRegistry::global().all_parsers().iter().map(RegisteredParser::name).collect();
		assert_eq!(labels.first().copied(), Some("All Supported Files"));
		assert_eq!(&labels[1..=expected.len()], expected.as_slice());
	}

	#[test]
	fn file_filter_string_contains_text_files_group_name() {
		let filter = build_file_filter_string();
		assert!(filter.contains("Text Files ("));
	}
	/// `add_tables_separators_lists` sets the Table marker's `length` to `table.length`
	/// (display units) and offsets it by the base offset.
	#[test]
	fn add_tables_separators_lists_sets_table_marker_length() {
		let converter = MockConverter {
			headings: vec![],
			links: vec![],
			images: vec![],
			figures: vec![],
			tables: vec![TableInfo {
				offset: 10,
				text: "T".to_string(),
				html_content: "<table/>".to_string(),
				length: 7, // display-unit field — must appear as marker length
			}],
			separators: vec![],
			lists: vec![],
			list_items: vec![],
			bolds: vec![],
			italics: vec![],
			underlines: vec![],
		};
		let mut buffer = DocumentBuffer::new();
		let base_offset = 100usize;
		add_converter_markers(&mut buffer, &converter, base_offset);
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.position, base_offset + 10, "position = offset + table.offset");
		assert_eq!(table_marker.length, 7, "marker length must equal table length, not byte length");
		assert_eq!(table_marker.reference, "<table/>", "marker reference must be the table HTML");
	}
}
