use std::collections::HashMap;

use bitflags::bitflags;
use scraper::Html;

use super::{format_spans::FormatSpans, line_builder::LineBuilder, list_style::ListStyle};
use crate::{
	parser::ConverterOutput,
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
};

bitflags! {
	#[derive(Default, Clone, Copy)]
	struct ProcessingFlags: u8 {
		const IN_BODY = 1;
		const IN_CODE = 4;
		const IN_LINK = 8;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlSourceMode {
	NativeHtml,
	Markdown,
}

#[derive(Default)]
pub struct HtmlToText {
	text: LineBuilder,
	id_positions: HashMap<String, usize>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	title: String,
	flags: ProcessingFlags,
	current_link_href: String,
	current_link_text: String,
	list_style_stack: Vec<ListStyle>,
	list_level: i32,
	/// Indices into `lists` for currently open `<ul>`/`<ol>` elements, in nesting order.
	/// `None` marks an open list that was not recorded (no direct `<li>`), keeping the stack
	/// balanced with the start/close handlers so list lengths are set on the right entries.
	open_lists: Vec<Option<usize>>,
	link_start_pos: usize,
	format_spans: FormatSpans,
	source_mode_markdown: bool,
	/// When `true`, tables are emitted as their full tab-separated rendering; otherwise as a
	/// `"[Table]: <first row>"` placeholder. A config flag, not parse state: it survives `clear()`.
	render_tables_inline: bool,
}

impl HtmlToText {
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

	pub fn convert(&mut self, html_content: &str, mode: HtmlSourceMode) -> bool {
		self.clear();
		self.source_mode_markdown = mode == HtmlSourceMode::Markdown;
		let document = Html::parse_document(html_content);
		let root = document.tree.root();
		self.process_node(root, &document);
		self.text.finalize_current_line();
		true
	}

	#[must_use]
	pub fn get_text(&self) -> String {
		self.text.get_text()
	}

	#[must_use]
	pub fn get_title(&self) -> &str {
		&self.title
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
	pub const fn get_id_positions(&self) -> &HashMap<String, usize> {
		&self.id_positions
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
		self.lists.clear();
		self.list_items.clear();
		self.title.clear();
		self.flags = ProcessingFlags::empty();
		self.current_link_href.clear();
		self.current_link_text.clear();
		self.list_style_stack.clear();
		self.list_level = 0;
		self.open_lists.clear();
		self.link_start_pos = 0;
		self.format_spans.clear();
	}
}

mod process;

impl ConverterOutput for HtmlToText {
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
