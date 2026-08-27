use std::collections::HashMap;

use roxmltree::{Document, ParsingOptions};

use super::{format_spans::FormatSpans, line_builder::LineBuilder, list_style::ListStyle};
use crate::{
	parser::ConverterOutput,
	types::{
		FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, PageBreakInfo, SeparatorInfo, TableInfo,
	},
};

#[derive(Default)]
pub struct XmlToText {
	text: LineBuilder,
	id_positions: HashMap<String, usize>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	page_breaks: Vec<PageBreakInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	section_offsets: Vec<usize>,
	position_watch: Option<usize>,
	watched_byte_offset: Option<usize>,
	in_body: bool,
	list_level: i32,
	list_style_stack: Vec<ListStyle>,
	/// Indices into `lists` for currently open `<ul>`/`<ol>` elements, in nesting order.
	/// `None` marks an open list that was not recorded (no direct `<li>`), keeping the stack
	/// balanced with the start/close handlers so list lengths are set on the right entries.
	open_lists: Vec<Option<usize>>,
	format_spans: FormatSpans,
	/// When `true`, tables are emitted as their full tab-separated rendering; otherwise as a
	/// `"[Table]: <first row>"` placeholder. A config flag, not parse state: it survives `clear()`.
	render_tables_inline: bool,
}

impl XmlToText {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Like [`new`](Self::new) but sets whether tables are rendered inline (full TSV) or as a
	/// placeholder. Threaded from the owning parser's `ParserContext`; preserved across
	/// `convert`/`clear`.
	#[must_use]
	pub fn with_render_tables_inline(render_tables_inline: bool) -> Self {
		Self { render_tables_inline, ..Self::default() }
	}

	pub fn convert(&mut self, xml_content: &str) -> bool {
		self.clear();
		let options = ParsingOptions { allow_dtd: true, ..ParsingOptions::default() };
		let doc = match Document::parse_with_options(xml_content, options) {
			Ok(doc) => doc,
			Err(e) => {
				tracing::warn!(error = %e, "failed to parse xml document");
				return false;
			}
		};
		for child in doc.root().children() {
			self.process_node(child);
		}
		self.text.finalize_current_line();
		tracing::debug!(bytes = xml_content.len(), lines = self.text.lines.len(), "converted xml document to text");
		true
	}

	#[must_use]
	pub fn get_text(&self) -> String {
		self.text.get_text()
	}

	/// Returns the source byte offset of the start tag of the element nearest
	/// at-or-before `target_position` (a character position in the converted text),
	/// suitable as an insertion point for a navigation anchor.
	pub fn find_anchor_byte_offset(&mut self, xml_content: &str, target_position: usize) -> Option<usize> {
		self.position_watch = Some(target_position);
		self.watched_byte_offset = None;
		let converted = self.convert(xml_content);
		self.position_watch = None;
		let result = self.watched_byte_offset.take();
		if converted { result } else { None }
	}

	#[must_use]
	pub const fn get_id_positions(&self) -> &HashMap<String, usize> {
		&self.id_positions
	}

	#[must_use]
	pub fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}

	#[must_use]
	pub fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}

	#[must_use]
	pub fn get_images(&self) -> &[ImageInfo] {
		&self.images
	}

	#[must_use]
	pub fn get_page_breaks(&self) -> &[PageBreakInfo] {
		&self.page_breaks
	}

	#[must_use]
	pub fn get_tables(&self) -> &[TableInfo] {
		&self.tables
	}

	#[must_use]
	pub fn get_separators(&self) -> &[SeparatorInfo] {
		&self.separators
	}

	#[must_use]
	pub fn get_lists(&self) -> &[ListInfo] {
		&self.lists
	}

	#[must_use]
	pub fn get_list_items(&self) -> &[ListItemInfo] {
		&self.list_items
	}

	#[must_use]
	pub fn get_section_offsets(&self) -> &[usize] {
		&self.section_offsets
	}

	#[must_use]
	pub fn get_bolds(&self) -> &[FormatInfo] {
		self.format_spans.bolds()
	}

	#[must_use]
	pub fn get_italics(&self) -> &[FormatInfo] {
		self.format_spans.italics()
	}

	#[must_use]
	pub fn get_underlines(&self) -> &[FormatInfo] {
		self.format_spans.underlines()
	}

	pub fn clear(&mut self) {
		self.text.clear();
		self.id_positions.clear();
		self.headings.clear();
		self.links.clear();
		self.images.clear();
		self.figures.clear();
		self.tables.clear();
		self.separators.clear();
		self.page_breaks.clear();
		self.lists.clear();
		self.list_items.clear();
		self.section_offsets.clear();
		self.in_body = false;
		self.list_level = 0;
		self.list_style_stack.clear();
		self.open_lists.clear();
		self.format_spans.clear();
	}
}

mod process;

/// Inserts an empty `<span id="{anchor_id}"></span>` into `xml_content`.
///
/// The span is placed before the element nearest at-or-before `target_position`
/// (a character position in the converted text). Returns `None` when the
/// content is not valid XML.
#[must_use]
pub fn inject_anchor_at_position(xml_content: &str, target_position: usize, anchor_id: &str) -> Option<String> {
	let byte_offset = XmlToText::new().find_anchor_byte_offset(xml_content, target_position)?;
	let mut result = xml_content.to_string();
	result.insert_str(byte_offset, &format!("<span id=\"{anchor_id}\"></span>"));
	Some(result)
}

impl ConverterOutput for XmlToText {
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
		self.format_spans.bolds()
	}
	fn get_italics(&self) -> &[FormatInfo] {
		self.format_spans.italics()
	}
	fn get_underlines(&self) -> &[FormatInfo] {
		self.format_spans.underlines()
	}
}

#[cfg(test)]
mod tests;
