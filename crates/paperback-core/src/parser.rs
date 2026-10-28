use std::{
	collections::{BTreeSet, HashMap},
	path::Path,
	string::String,
	sync::OnceLock,
};

use anyhow::Result;
use paperback_formats::FormatMeta;

use crate::{
	document::{
		DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem, is_heading_marker,
	},
	t,
	types::{
		FormatInfo, FormulaInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo,
	},
};

pub mod cbr;
pub mod cbz;
pub mod chm;
pub mod convert;
pub mod daisy;
pub mod epub;
pub mod fb2;
pub mod hlp;
pub mod html;
pub mod m4b;
pub mod man;
pub mod markdown;
pub mod mobi;
pub mod mp3;
mod odf_crypto;
pub mod odp;
pub mod odt;
pub mod pdf;
pub mod powerpoint;
pub mod rst;
pub mod rtf;
pub mod text;
pub mod util;
pub mod word;
pub mod wri;

pub const PASSWORD_REQUIRED_ERROR_PREFIX: &str = "[password_required]";

pub trait Parser: Send + Sync {
	/// # Errors
	///
	/// Returns an error if the file cannot be read or parsed.
	fn parse(&self, context: &ParserContext) -> Result<Document>;
}

/// Format metadata is attached here at registration so parsers hold none that could drift from the `paperback-formats` table.
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

	/// # Errors
	///
	/// Returns an error if the file cannot be read or parsed.
	pub fn parse(&self, context: &ParserContext) -> Result<Document> {
		self.parser.parse(context)
	}
}

/// Entry order decides which parser is tried first for an extension several formats claim, so it should match the format table's declaration order.
macro_rules! parser_registry {
	($($format:ident => $parser:expr),+ $(,)?) => {{
		let mut registry = ParserRegistry::new();
		$(registry.register(&paperback_formats::$format, $parser);)+
		registry
	}};
}

pub struct ParserRegistry {
	parsers: Vec<RegisteredParser>,
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
				CBR => cbr::CbrParser,
				CBZ => cbz::CbzParser,
				CHM => chm::ChmParser,
				HLP => hlp::HlpParser,
				DAISY => daisy::DaisyParser,
				WORD => word::WordParser,
				EPUB => epub::EpubParser,
				FB2 => fb2::Fb2Parser,
				HTML => html::HtmlParser,
				PDF => pdf::PdfParser,
				MAN => man::ManParser,
				MARKDOWN => markdown::MarkdownParser,
				M4B => m4b::M4bParser,
				MP3 => mp3::Mp3Parser,
				MOBI => mobi::MobiParser,
				FODP => odp::FodpParser,
				ODP => odp::OdpParser,
				FODT => odt::FodtParser,
				ODT => odt::OdtParser,
				POWERPOINT => powerpoint::PowerpointParser,
				RST => rst::RstParser,
				RTF => rtf::RtfParser,
				TEXT => text::TextParser,
				WRI => wri::WriParser,
			}
		})
	}
}

fn resolve_extension(path: &Path) -> Option<&str> {
	// A loose DAISY 2.02 book's master file; DAISY can't claim `.html` without capturing every ordinary HTML file.
	if path.file_name().is_some_and(|n| n.eq_ignore_ascii_case("ncc.html")) {
		return Some("opf");
	}
	// Manual pages are named for their section (`ls.1`, `Tcl_Init.3tcl`, `printf.3.gz`), which names the format as surely as an extension.
	if man::is_manual_page_name(path) {
		return Some("man");
	}
	path.extension().and_then(|e| e.to_str())
}

/// # Errors
///
/// Returns an error if no parser for the file's extension can parse it.
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

/// Prefer this to [`parser_supports_extension`] when a path is at hand: a manual page is named for its section, which only the whole name shows.
#[must_use]
pub fn parser_supports_path(path: &Path) -> bool {
	resolve_extension(path).is_some_and(parser_supports_extension)
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
	// NSOpenPanel treats *.* as "allow everything", which disables filtering for every group.
	#[cfg(not(target_os = "macos"))]
	parts.push_str("All Files (*.*)|*.*");
	#[cfg(target_os = "macos")]
	{
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
	fn get_formulas(&self) -> &[FormulaInfo];
	fn get_separators(&self) -> &[SeparatorInfo];
	fn get_lists(&self) -> &[ListInfo];
	fn get_list_items(&self) -> &[ListItemInfo];
	fn get_bolds(&self) -> &[FormatInfo];
	fn get_italics(&self) -> &[FormatInfo];
	fn get_underlines(&self) -> &[FormatInfo];
}

fn add_headings(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	for heading in converter.get_headings() {
		let marker_type = util::toc::heading_level_to_marker_type(heading.level);
		buffer.add_marker(
			Marker::new(marker_type, offset + heading.offset.from_start())
				.with_text(heading.text.clone())
				.with_level(heading.level),
		);
	}
}

fn add_links(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	for link in converter.get_links() {
		buffer.add_marker(
			Marker::new(MarkerType::Link, offset + link.offset.from_start())
				.with_text(link.text.clone())
				.with_reference(link.reference.clone()),
		);
	}
}

/// The picture's location goes in the marker's reference so it can be saved; `resolve` is for formats whose sections each have their own base.
fn add_images(
	buffer: &mut DocumentBuffer,
	converter: &dyn ConverterOutput,
	offset: DocPos,
	resolve: &dyn Fn(&str) -> String,
) {
	let reference = |image: &ImageInfo| {
		if image.source.is_empty() || is_data_uri(&image.source) {
			image.source.clone()
		} else {
			resolve(&image.source)
		}
	};
	for image in converter.get_images() {
		buffer.add_marker(
			Marker::new(MarkerType::Image, offset + image.offset.from_start())
				.with_text(image.alt_text.clone())
				.with_reference(reference(image))
				.with_length(image.length),
		);
	}
	for figure in converter.get_figures() {
		buffer.add_marker(
			Marker::new(MarkerType::Figure, offset + figure.offset.from_start())
				.with_text(figure.alt_text.clone())
				.with_reference(reference(figure))
				.with_length(figure.length),
		);
	}
}

fn add_tables_separators_lists(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	for table in converter.get_tables() {
		buffer.add_marker(
			Marker::new(MarkerType::Table, offset + table.offset.from_start())
				.with_text(table.text.clone())
				.with_reference(table.html_content.clone())
				.with_length(table.length),
		);
	}
	for separator in converter.get_separators() {
		buffer.add_marker(
			Marker::new(MarkerType::Separator, offset + separator.offset.from_start())
				.with_text("Separator".to_string())
				.with_length(separator.length),
		);
	}
	for list in converter.get_lists() {
		buffer.add_marker(
			Marker::new(MarkerType::List, offset + list.offset.from_start())
				.with_level(list.item_count)
				.with_length(list.length),
		);
	}
	for list_item in converter.get_list_items() {
		buffer.add_marker(
			Marker::new(MarkerType::ListItem, offset + list_item.offset.from_start())
				.with_text(list_item.text.clone())
				.with_level(list_item.level),
		);
	}
}

fn add_formulas(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	for formula in converter.get_formulas() {
		buffer.add_marker(
			Marker::new(MarkerType::Formula, offset + formula.offset.from_start())
				.with_text(formula.text.clone())
				.with_reference(formula.mathml.clone())
				.with_length(formula.length),
		);
	}
}

fn add_formatting(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	for bold in converter.get_bolds() {
		buffer.add_marker(Marker::new(MarkerType::Bold, offset + bold.offset.from_start()).with_length(bold.length));
	}
	for italic in converter.get_italics() {
		buffer.add_marker(
			Marker::new(MarkerType::Italic, offset + italic.offset.from_start()).with_length(italic.length),
		);
	}
	for underline in converter.get_underlines() {
		buffer.add_marker(
			Marker::new(MarkerType::Underline, offset + underline.offset.from_start()).with_length(underline.length),
		);
	}
}

/// For documents that name their sections only in a table of contents, such as a PDF outline over same-size text or an old CHM whose `.hhc` topics are styled paragraphs rather than `<h1>`.
pub fn add_heading_markers(buffer: &mut DocumentBuffer, items: &[TocItem], level: i32) {
	add_heading_markers_where(buffer, items, level, &|_| true);
}

/// An entry `wanted` turns down still has its children offered, since a section that wrote its own heading may sit above subsections that did not.
pub fn add_heading_markers_where(
	buffer: &mut DocumentBuffer,
	items: &[TocItem],
	level: i32,
	wanted: &dyn Fn(DocPos) -> bool,
) {
	for item in items {
		if wanted(item.offset) {
			let marker_type = match level {
				1 => MarkerType::Heading1,
				2 => MarkerType::Heading2,
				3 => MarkerType::Heading3,
				4 => MarkerType::Heading4,
				5 => MarkerType::Heading5,
				_ => MarkerType::Heading6,
			};
			buffer.add_marker(Marker::new(marker_type, item.offset).with_text(item.name.clone()).with_level(level));
		}
		add_heading_markers_where(buffer, &item.children, level + 1, wanted);
	}
}

/// Skips entries whose span already holds a real heading, so a well-marked-up book is left alone and nothing is announced twice; classic Gutenberg EPUBs and old CHMs and MOBIs need this.
pub fn add_toc_heading_markers(buffer: &mut DocumentBuffer, toc_items: &[TocItem]) {
	let existing = heading_positions(buffer);
	let spans = toc_entry_offsets(toc_items);
	add_heading_markers_where(buffer, toc_items, 1, &|offset| !span_has_heading(&spans, &existing, offset));
}

fn heading_positions(buffer: &DocumentBuffer) -> Vec<DocPos> {
	let mut positions: Vec<DocPos> =
		buffer.markers.iter().filter(|marker| is_heading_marker(marker.mtype)).map(|marker| marker.position).collect();
	positions.sort_unstable();
	positions
}

fn toc_entry_offsets(items: &[TocItem]) -> Vec<DocPos> {
	let mut offsets = Vec::new();
	collect_toc_offsets(items, &mut offsets);
	offsets.sort_unstable();
	offsets
}

fn collect_toc_offsets(items: &[TocItem], out: &mut Vec<DocPos>) {
	for item in items {
		out.push(item.offset);
		collect_toc_offsets(&item.children, out);
	}
}

fn span_has_heading(spans: &[DocPos], existing: &[DocPos], offset: DocPos) -> bool {
	let end = spans.iter().copied().find(|start| *start > offset);
	existing.iter().any(|position| *position >= offset && end.is_none_or(|end| *position < end))
}

pub fn add_converter_markers(buffer: &mut DocumentBuffer, converter: &dyn ConverterOutput, offset: DocPos) {
	add_headings(buffer, converter, offset);
	add_links(buffer, converter, offset);
	add_images(buffer, converter, offset, &str::to_string);
	add_tables_separators_lists(buffer, converter, offset);
	add_formulas(buffer, converter, offset);
	add_formatting(buffer, converter, offset);
}

/// For parsers that resolve link hrefs and image sources against their own sections.
pub fn add_converter_markers_excluding_links(
	buffer: &mut DocumentBuffer,
	converter: &dyn ConverterOutput,
	offset: DocPos,
	resolve_image: &dyn Fn(&str) -> String,
) {
	add_headings(buffer, converter, offset);
	add_images(buffer, converter, offset, resolve_image);
	add_tables_separators_lists(buffer, converter, offset);
	add_formulas(buffer, converter, offset);
	add_formatting(buffer, converter, offset);
}

#[must_use]
pub fn is_external_url(url: &str) -> bool {
	let lower = url.to_ascii_lowercase();
	lower.starts_with("http:") || lower.starts_with("https:") || lower.starts_with("mailto:")
}

#[must_use]
pub fn is_data_uri(source: &str) -> bool {
	source.get(..5).is_some_and(|scheme| scheme.eq_ignore_ascii_case("data:"))
}

#[must_use]
pub fn is_remote_url(text: &str) -> bool {
	["http://", "https://"].iter().any(|scheme| {
		text.len() > scheme.len() && text.get(..scheme.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
	})
}

#[must_use]
pub fn url_file_name(url: &str) -> Option<String> {
	let after_scheme = url.split_once("://").map_or(url, |(_, rest)| rest);
	let before_query = after_scheme.split(['?', '#']).next().unwrap_or_default();
	let (_, path) = before_query.split_once('/')?;
	let segment = path.rsplit('/').next().unwrap_or_default();
	let decoded = percent_encoding::percent_decode_str(segment).decode_utf8_lossy();
	let name = decoded.rsplit(['/', '\\']).next().unwrap_or_default().trim();
	(!name.is_empty()).then(|| name.to_string())
}

#[must_use]
pub fn url_extension(url: &str) -> Option<String> {
	let name = url_file_name(url)?;
	Path::new(&name).extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase)
}

#[cfg(test)]
mod tests;
