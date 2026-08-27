use std::{collections::HashMap, fmt::Write};

use bitflags::bitflags;
use ego_tree::NodeRef;
use scraper::{ElementRef, Html, Node, node};

use super::{
	block_elements::is_block_element,
	format_spans::{FormatKind, FormatSpans},
	line_builder::LineBuilder,
	table_text::{collect_dom_text, table_render_bundle},
};
use crate::{
	parser::ConverterOutput,
	t,
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
	util::text::{collapse_whitespace, display_len, format_list_item, remove_soft_hyphens, trim_string},
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

#[derive(Debug, Clone)]
struct ListStyle {
	ordered: bool,
	item_number: i32,
	list_type: String,
}

impl Default for ListStyle {
	fn default() -> Self {
		Self { ordered: false, item_number: 1, list_type: "1".to_string() }
	}
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

	const fn get_bullet_for_level(level: i32) -> &'static str {
		match level {
			2 => "◦",
			3 => "*",
			4 => "-",
			_ => "•",
		}
	}

	fn process_node(&mut self, node: NodeRef<'_, Node>, document: &Html) {
		match node.value() {
			Node::Element(element) => {
				let tag_name = element.name();
				if tag_name == "table" {
					if self.flags.contains(ProcessingFlags::IN_BODY)
						&& let Some(id) = element.attr("id").or_else(|| element.attr("name"))
					{
						self.id_positions.insert(id.to_string(), self.text.get_current_text_position());
					}
					self.handle_table(node, document);
					return;
				}
				self.handle_element_opening(tag_name, node, document);
				self.handle_list_item(tag_name, node, document);
				self.handle_list_start(tag_name, node);
				self.handle_heading(tag_name, node, document);
				if matches!(tag_name, "script" | "style" | "noscript" | "iframe" | "object" | "embed") {
					return;
				}
				self.process_element_children(node, document, tag_name);
				self.handle_element_closing(tag_name);
			}
			Node::Text(text) => {
				self.handle_text_node(text);
			}
			Node::Comment(_) => {}
			_ => {
				for child in node.children() {
					self.process_node(child, document);
				}
			}
		}
	}

	fn handle_table(&mut self, node: NodeRef<'_, Node>, document: &Html) {
		self.text.finalize_current_line();
		let table_html = Self::serialize_node(node, document);
		let start_offset = self.text.get_current_text_position();
		// Emit the table's on-screen text via the shared helper instead of recursing children to
		// emit one cell per line. The helper output may contain tabs and span multiple lines; push
		// each line verbatim so tab separators and empty cells survive whitespace collapsing.
		let render = table_render_bundle(&table_html, self.render_tables_inline);
		for line in render.lines {
			self.text.push_finalized_line(line);
		}
		let table_caption = render.caption;
		let display_length = render.display_length;
		self.tables.push(TableInfo {
			offset: start_offset,
			text: table_caption,
			html_content: table_html,
			length: display_length,
		});
	}

	fn handle_element_opening(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if let Node::Element(element) = node.value() {
			if self.flags.contains(ProcessingFlags::IN_BODY) {
				if let Some(id) = element.attr("id").or_else(|| element.attr("name")) {
					self.id_positions.insert(id.to_string(), self.text.get_current_text_position());
				}
				if tag_name == "img" || tag_name == "image" || tag_name == "figure" {
					let mut description = element
						.attr("alt")
						.or_else(|| element.attr("aria-label"))
						.or_else(|| element.attr("aria-description"))
						.or_else(|| element.attr("title"))
						.map(collapse_whitespace)
						.unwrap_or_default();

					if description.is_empty() && tag_name == "figure" {
						for child in node.children() {
							if let Node::Element(child_elem) = child.value()
								&& child_elem.name() == "figcaption"
							{
								description = collapse_whitespace(&Self::collect_text(child));
								break;
							}
						}
					}

					if !description.is_empty() {
						let is_figure = tag_name == "figure";
						// TRANSLATORS: Label inserted before a figure or image's description, e.g. "[Figure: a cat sleeping]"
						let label = if is_figure { t("Figure") } else { t("Image") };
						let image_text = format!("[{label}: {description}]");
						let offset = self.text.get_current_text_position();
						self.text.current_line.push_str(&image_text);
						let info = ImageInfo { offset, alt_text: description };
						if is_figure {
							self.figures.push(info);
						} else {
							self.images.push(info);
						}
					}
				}
			}
			if tag_name == "a" && !self.flags.contains(ProcessingFlags::IN_LINK) {
				self.flags.insert(ProcessingFlags::IN_LINK);
				if let Some(href) = element.attr("href") {
					self.current_link_href = href.to_string();
				}
				self.link_start_pos = self.text.get_current_text_position();
			}
			if tag_name == "b" || tag_name == "strong" {
				self.format_spans.open(&FormatKind::Bold, self.text.get_current_text_position());
			} else if tag_name == "i" || tag_name == "em" {
				self.format_spans.open(&FormatKind::Italic, self.text.get_current_text_position());
			} else if tag_name == "u" {
				self.format_spans.open(&FormatKind::Underline, self.text.get_current_text_position());
			}
		}
		if tag_name == "title" && self.title.is_empty() {
			self.title = Self::get_element_text(node, document);
			self.title = trim_string(&collapse_whitespace(&self.title));
		} else if tag_name == "body" {
			self.flags.insert(ProcessingFlags::IN_BODY);
		} else if tag_name == "pre" {
			self.text.finalize_current_line();
			self.text.start_preserve_whitespace();
		} else if tag_name == "hr" && self.flags.contains(ProcessingFlags::IN_BODY) {
			self.text.finalize_current_line();
			let offset = self.text.get_current_text_position();
			let line = LineBuilder::separator_line();
			self.text.current_line.push_str(line);
			self.text.finalize_current_line();
			self.separators.push(SeparatorInfo { offset, length: display_len(line) });
		} else if tag_name == "code" {
			self.flags.insert(ProcessingFlags::IN_CODE);
			self.text.start_preserve_whitespace();
		} else if tag_name == "br" {
			self.text.finalize_current_line();
		}
	}

	fn handle_list_item(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if tag_name == "li" {
			self.text.finalize_current_line();
			let li_text = Self::get_element_text(node, document);
			self.list_items.push(ListItemInfo {
				offset: self.text.get_current_text_position(),
				level: self.list_level,
				text: li_text,
			});
			for _ in 0..self.list_level {
				self.text.current_line.push_str("  ");
			}
			if let Some(style) = self.list_style_stack.last_mut() {
				if style.ordered {
					let item_text = format_list_item(style.item_number, &style.list_type);
					let _ = write!(&mut self.text.current_line, "{item_text}. ");
					style.item_number += 1;
				} else {
					self.text.current_line.push_str(Self::get_bullet_for_level(self.list_level));
					self.text.current_line.push(' ');
				}
			} else {
				self.text.current_line.push_str(Self::get_bullet_for_level(self.list_level));
				self.text.current_line.push(' ');
			}
		}
	}

	fn handle_list_start(&mut self, tag_name: &str, node: NodeRef<'_, Node>) {
		if tag_name == "ul" || tag_name == "ol" {
			self.list_level += 1;
			let mut style = ListStyle::default();
			if tag_name == "ol" {
				style.ordered = true;
				if let Some(element) = ElementRef::wrap(node) {
					if let Some(start_val) = element.attr("start")
						&& let Ok(start_num) = start_val.parse::<i32>()
					{
						style.item_number = start_num;
					}
					if let Some(type_val) = element.attr("type") {
						style.list_type = type_val.to_lowercase();
					}
				}
			}
			self.list_style_stack.push(style);
			let mut item_count = 0;
			for child in node.children() {
				if let Node::Element(child_elem) = child.value()
					&& child_elem.name() == "li"
				{
					item_count += 1;
				}
			}
			if item_count > 0 {
				self.text.finalize_current_line();
				self.open_lists.push(Some(self.lists.len()));
				self.lists.push(ListInfo { offset: self.text.get_current_text_position(), item_count, length: 0 });
			} else {
				self.open_lists.push(None);
			}
		}
	}

	fn handle_heading(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if self.flags.contains(ProcessingFlags::IN_BODY)
			&& tag_name.len() == 2
			&& tag_name.starts_with('h')
			&& tag_name.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
			&& let Some(level_char) = tag_name.chars().nth(1)
			&& let Some(level) = level_char.to_digit(10)
			&& (1..=6).contains(&level)
		{
			self.text.finalize_current_line();
			let heading_offset = self.text.get_current_text_position();
			let heading_text = Self::get_element_text(node, document);
			if !heading_text.is_empty() {
				#[allow(clippy::cast_possible_wrap)]
				self.headings.push(HeadingInfo { offset: heading_offset, level: level as i32, text: heading_text });
			}
		}
	}

	fn process_element_children(&mut self, node: NodeRef<'_, Node>, document: &Html, tag_name: &str) {
		let is_markdown_code = self.source_mode_markdown
			&& self.flags.contains(ProcessingFlags::IN_CODE)
			&& self.text.is_preserving_whitespace()
			&& tag_name == "code";
		if is_markdown_code {
			for child in node.children() {
				if let Node::Element(_) = child.value() {
					let html_str = Self::serialize_node(child, document);
					self.text.current_line.push_str(&html_str);
				} else {
					self.process_node(child, document);
				}
			}
		} else {
			for child in node.children() {
				self.process_node(child, document);
			}
		}
	}

	fn handle_element_closing(&mut self, tag_name: &str) {
		if tag_name == "a" && self.flags.contains(ProcessingFlags::IN_LINK) {
			self.flags.remove(ProcessingFlags::IN_LINK);
			if !self.current_link_text.is_empty() {
				let collapsed_text = collapse_whitespace(&self.current_link_text);
				self.links.push(LinkInfo {
					offset: self.link_start_pos,
					text: collapsed_text.clone(),
					reference: self.current_link_href.clone(),
				});
				self.text.current_line.push_str(&collapsed_text);
			}
			self.current_link_href.clear();
			self.current_link_text.clear();
		}
		if tag_name == "code" {
			self.flags.remove(ProcessingFlags::IN_CODE);
			self.text.stop_preserve_whitespace();
		}
		if tag_name == "ul" || tag_name == "ol" {
			self.list_level -= 1;
			self.list_style_stack.pop();
			if let Some(open) = self.open_lists.pop().flatten() {
				self.text.finalize_current_line();
				let offset = self.lists[open].offset;
				self.lists[open].length = self.text.get_current_text_position().saturating_sub(offset);
			}
		}
		if tag_name == "pre" {
			let has_preserved_trailing_whitespace =
				self.text.is_preserving_whitespace() && self.text.current_line.trim().is_empty();
			if has_preserved_trailing_whitespace {
				self.text.current_line.clear();
			} else {
				self.text.finalize_current_line();
			}
			self.text.stop_preserve_whitespace();
		} else if is_block_element(tag_name) {
			self.text.finalize_current_line();
		}
		if tag_name == "b" || tag_name == "strong" {
			self.format_spans.close(&FormatKind::Bold, self.text.get_current_text_position());
		} else if tag_name == "i" || tag_name == "em" {
			self.format_spans.close(&FormatKind::Italic, self.text.get_current_text_position());
		} else if tag_name == "u" {
			self.format_spans.close(&FormatKind::Underline, self.text.get_current_text_position());
		}
	}

	fn handle_text_node(&mut self, text: &node::Text) {
		if !self.flags.contains(ProcessingFlags::IN_BODY) {
			return;
		}
		let text_content = text.text.to_string();
		if text_content.is_empty() {
			return;
		}
		let processed_text = remove_soft_hyphens(&text_content);
		if self.text.is_preserving_whitespace() {
			let lines: Vec<&str> = processed_text.split('\n').collect();
			for (i, line) in lines.iter().enumerate() {
				self.text.current_line.push_str(line);
				if i < lines.len() - 1 {
					self.text.finalize_current_line();
				}
			}
		} else if self.flags.contains(ProcessingFlags::IN_LINK) {
			self.current_link_text.push_str(&collapse_whitespace(&processed_text));
		} else {
			self.text.current_line.push_str(&collapse_whitespace(&processed_text));
		}
	}

	fn get_element_text(node: NodeRef<'_, Node>, _document: &Html) -> String {
		Self::collect_text(node)
	}

	fn collect_text(node: NodeRef<'_, Node>) -> String {
		let mut buffer = String::new();
		collect_dom_text(node, &mut buffer, false);
		buffer
	}

	fn serialize_node(node: NodeRef<'_, Node>, _document: &Html) -> String {
		match node.value() {
			Node::Element(_) => ElementRef::wrap(node).map_or_else(String::new, |element_ref| element_ref.html()),
			Node::Text(text) => text.text.to_string(),
			_ => String::new(),
		}
	}
}

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
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{
		document::{DocumentBuffer, MarkerType},
		parser::add_converter_markers,
	};

	/// End-to-end: the `HtmlToText` converter emits each table's on-screen text at parse time, and a
	/// heading that follows the table is offset by the emitted display extent. Verified in both
	/// modes: OFF (placeholder) and ON (full TSV). The fixture has an "Intro" paragraph before the
	/// table (so the table offset is non-zero) and an `<h2>` after it.
	#[rstest]
	#[case(false)]
	#[case(true)]
	fn html_converter_emits_table_inline_or_placeholder(#[case] inline: bool) {
		let html = concat!(
			"<html><body>",
			"<p>Intro</p>",
			"<table><tr><td>A</td><td>B</td></tr></table>",
			"<h2>After heading</h2>",
			"</body></html>"
		);

		let mut converter = HtmlToText::with_render_tables_inline(inline);
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));

		let tables = converter.get_tables();
		assert_eq!(tables.len(), 1);
		assert_eq!(tables[0].offset, 6, "table follows 'Intro\n' (6 display units)");

		let table_line = if inline { "A\tB" } else { "[Table]: A B" };
		let expected_text = format!("Intro\n{table_line}\nAfter heading");
		assert_eq!(converter.get_text(), expected_text, "table emitted as {table_line:?}");

		// display_length equals the emitted display extent (the table line plus its newline).
		let expected_display_length = display_len(table_line) + 1;
		assert_eq!(tables[0].length, expected_display_length);

		// The heading marker that follows the table sits right after the emitted table span.
		let headings = converter.get_headings();
		assert_eq!(headings.len(), 1);
		assert_eq!(
			headings[0].offset,
			tables[0].offset + expected_display_length,
			"h2 immediately follows the emitted table span"
		);

		// Through the real marker path, the Table marker's length matches the emitted extent.
		let mut buffer = DocumentBuffer::with_content(converter.get_text());
		add_converter_markers(&mut buffer, &converter, 0);
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.length, expected_display_length);
	}

	#[test]
	fn test_title_and_text() {
		let html = "<html><head><title>  Hello   World </title></head><body><p>Hi</p></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert_eq!(converter.get_title(), "Hello World");
		assert_eq!(converter.get_text(), "Hi");
	}

	#[test]
	fn test_link_collection() {
		let html = "<html><body><a href=\"https://example.com\">Hello   world</a></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let links = converter.get_links();
		assert_eq!(links.len(), 1);
		assert_eq!(links[0].text, "Hello world");
		assert_eq!(links[0].reference, "https://example.com");
		assert_eq!(converter.get_text(), "Hello world");
	}

	#[rstest]
	#[case("b", MarkerType::Bold)]
	#[case("strong", MarkerType::Bold)]
	#[case("i", MarkerType::Italic)]
	#[case("em", MarkerType::Italic)]
	#[case("u", MarkerType::Underline)]
	fn test_format_span_collection(#[case] tag: &str, #[case] kind: MarkerType) {
		let html = format!("<html><body><p>Some <{tag}>bold</{tag}> text</p></body></html>");
		let mut converter = HtmlToText::new();
		assert!(converter.convert(&html, HtmlSourceMode::NativeHtml));
		let spans = match kind {
			MarkerType::Bold => converter.get_bolds(),
			MarkerType::Italic => converter.get_italics(),
			MarkerType::Underline => converter.get_underlines(),
			_ => unreachable!(),
		};
		assert_eq!(spans.len(), 1);
		let text = converter.get_text();
		assert_eq!(&text[spans[0].offset..spans[0].offset + spans[0].length], "bold");
	}

	#[test]
	fn test_format_span_nested_bold_and_italic() {
		let html = "<html><body><p><b>outer <i>inner</i></b></p></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let text = converter.get_text();

		let italics = converter.get_italics();
		assert_eq!(italics.len(), 1);
		assert_eq!(&text[italics[0].offset..italics[0].offset + italics[0].length], "inner");

		let bolds = converter.get_bolds();
		assert_eq!(bolds.len(), 1);
		assert_eq!(&text[bolds[0].offset..bolds[0].offset + bolds[0].length], "outer inner");
	}

	#[test]
	fn test_format_span_fully_inside_link_is_dropped_known_limitation() {
		// Known limitation: text inside an `<a>` is buffered in `current_link_text` and only
		// pushed into `current_line` on `</a>` (see `handle_text_node`/`handle_element_closing`),
		// so `get_current_text_position()` does not advance while inside a link. A `<b>`/`<i>`/`<u>`
		// that opens and closes fully inside an `<a>` therefore sees `start == end` and is recorded
		// as a zero-length span rather than spanning the link's text. This degrades gracefully (no
		// panic, no bad offset; the link itself is still recorded correctly) rather than corrupting
		// data, so it is left as-is (no changes to the `<a>` deferred-buffering behavior).
		let html = "<html><body><a href=\"https://example.com\"><b>bold</b></a></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let bolds = converter.get_bolds();
		assert_eq!(bolds.len(), 1);
		assert_eq!(bolds[0].length, 0);
		let links = converter.get_links();
		assert_eq!(links.len(), 1);
		assert_eq!(links[0].text, "bold");
	}

	#[test]
	fn test_no_format_spans_without_formatting_tags() {
		let html = "<html><body><p>Plain paragraph with no formatting.</p></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert!(converter.get_bolds().is_empty());
		assert!(converter.get_italics().is_empty());
		assert!(converter.get_underlines().is_empty());
	}

	#[test]
	fn test_ordered_list_metadata() {
		let html = "<html><body><ol start=\"3\" type=\"a\"><li>First</li><li>Second</li></ol></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let lists = converter.get_lists();
		let items = converter.get_list_items();
		assert_eq!(lists.len(), 1);
		assert_eq!(lists[0].item_count, 2);
		// The recorded length must span the whole list, reaching at least the last item's start.
		assert!(lists[0].length > 0);
		assert!(lists[0].offset + lists[0].length >= items[1].offset);
		// End lands at most one line break past the content (trailing newline at document end).
		assert!(lists[0].offset + lists[0].length <= display_len(&converter.get_text()) + 1);
		assert_eq!(items.len(), 2);
		assert_eq!(items[0].level, 1);
		assert_eq!(items[0].text, "First");
		assert_eq!(items[1].text, "Second");
	}

	#[test]
	fn test_table_caption_fallback() {
		let html = "<html><body><table><tr><td>Header</td></tr></table></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let tables = converter.get_tables();
		assert_eq!(tables.len(), 1);
		assert_eq!(tables[0].text, "Header");
	}

	#[rstest]
	#[case("h1", 1)]
	#[case("h2", 2)]
	#[case("h3", 3)]
	#[case("h4", 4)]
	#[case("h5", 5)]
	#[case("h6", 6)]
	fn heading_levels_h1_to_h6(#[case] tag: &str, #[case] expected_level: i32) {
		let html = format!("<html><body><{tag}>Title</{tag}></body></html>");
		let mut converter = HtmlToText::new();
		assert!(converter.convert(&html, HtmlSourceMode::NativeHtml));
		let headings = converter.get_headings();
		assert_eq!(headings.len(), 1);
		assert_eq!(headings[0].level, expected_level);
		assert_eq!(headings[0].text, "Title");
	}

	#[test]
	fn hr_produces_separator() {
		let html = "<html><body><p>Before</p><hr/><p>After</p></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert_eq!(converter.get_separators().len(), 1);
	}

	#[test]
	fn nested_ul_increments_list_level() {
		let html = "<html><body><ul><li>Outer<ul><li>Inner</li></ul></li></ul></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let items = converter.get_list_items();
		assert!(items.len() >= 2, "expected at least two list items");
		let outer_level = items.iter().find(|i| i.text == "Outer").map_or(0, |i| i.level);
		let inner_level = items.iter().find(|i| i.text == "Inner").map_or(0, |i| i.level);
		assert!(inner_level > outer_level, "nested item should have a higher level");
	}

	#[test]
	fn element_id_is_indexed() {
		let html = "<html><body><p id=\"anchor\">Content</p></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert!(converter.get_id_positions().contains_key("anchor"));
	}

	#[test]
	fn pre_block_preserves_whitespace_characters() {
		let html = "<html><body><pre>  spaced  </pre></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert!(converter.get_text().contains("  spaced  "));
	}

	/// A `<pre>` block whose source text uses CRLF line endings must not leak a stray `\r` onto
	/// the end of each preserved line - the shared `LineBuilder::add_line` strips trailing
	/// `\r`/`\n` from a preserved line before storing it, same as `XmlToText` already relied on.
	#[test]
	fn pre_block_with_crlf_content_does_not_leak_carriage_returns() {
		let html = "<html><body><pre>line one\r\nline two</pre></body></html>";
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		assert!(!converter.get_text().contains('\r'), "got: {:?}", converter.get_text());
	}

	#[test]
	fn clear_resets_converter_state() {
		let html1 = "<html><head><title>First</title></head><body><h1>One</h1></body></html>";
		let html2 = "<html><head><title>Second</title></head><body><p>Two</p></body></html>";
		let mut converter = HtmlToText::new();
		converter.convert(html1, HtmlSourceMode::NativeHtml);
		converter.clear();
		assert!(converter.convert(html2, HtmlSourceMode::NativeHtml));
		assert_eq!(converter.get_title(), "Second");
		assert_eq!(converter.get_text(), "Two");
		assert!(converter.get_headings().is_empty());
	}

	#[test]
	fn html_table_display_length_is_display_extent_not_byte_length() {
		let html = concat!(
			"<html><body><p>Intro</p>",
			"<table><tr><td>A</td><td>\u{1D11E}</td></tr></table>",
			"</body></html>"
		);
		let mut converter = HtmlToText::with_render_tables_inline(true);
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let tables = converter.get_tables();
		assert_eq!(tables.len(), 1, "expected exactly one table");
		let table = &tables[0];
		assert_eq!(table.offset, 6, "table starts after 'Intro\n'");
		assert_eq!(table.length, 5, "length must be the display extent (5 display units), not byte length (6)");
	}

	#[test]
	fn html_two_tables_offsets_are_cumulative() {
		let html = concat!(
			"<html><body>",
			"<table><tr><td>X</td></tr></table>",
			"<table><tr><td>Y</td></tr></table>",
			"</body></html>"
		);
		let mut converter = HtmlToText::new();
		assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
		let tables = converter.get_tables();
		assert_eq!(tables.len(), 2, "expected two tables");

		let t1_offset = tables[0].offset;
		let t1_display_length = tables[0].length;
		let t2_offset = tables[1].offset;

		assert_eq!(t1_offset, 0, "first table starts at 0");
		assert!(t1_display_length > 0, "first table has non-zero display_length");
		assert_eq!(
			t2_offset,
			t1_offset + t1_display_length,
			"second table offset must equal first offset + first display_length"
		);
	}
}
