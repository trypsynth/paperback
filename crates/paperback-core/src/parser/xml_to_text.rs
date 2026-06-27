use std::{collections::HashMap, mem};

use roxmltree::{Document, Node, NodeType, ParsingOptions};

use crate::{
	parser::util::xml::collect_element_text,
	t,
	types::{HeadingInfo, LinkInfo, ListInfo, ListItemInfo, PageBreakInfo, SeparatorInfo, TableInfo},
	util::text::{collapse_whitespace, display_len, format_list_item, remove_soft_hyphens, trim_string},
};

#[derive(Clone)]
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
pub struct XmlToText {
	lines: Vec<String>,
	current_line: String,
	id_positions: HashMap<String, usize>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<crate::types::ImageInfo>,
	figures: Vec<crate::types::ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	page_breaks: Vec<PageBreakInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	section_offsets: Vec<usize>,
	position_watch: Option<usize>,
	watched_byte_offset: Option<usize>,
	in_body: bool,
	preserve_whitespace_depth: usize,
	list_level: i32,
	list_style_stack: Vec<ListStyle>,
	/// Indices into `lists` for currently open `<ul>`/`<ol>` elements, in nesting order.
	/// `None` marks an open list that was not recorded (no direct `<li>`), keeping the stack
	/// balanced with the start/close handlers so list lengths are set on the right entries.
	open_lists: Vec<Option<usize>>,
	cached_char_length: usize,
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
		let Ok(doc) = Document::parse_with_options(xml_content, options) else {
			return false;
		};
		for child in doc.root().children() {
			self.process_node(child);
		}
		self.finalize_current_line();
		true
	}

	#[must_use]
	pub fn get_text(&self) -> String {
		self.lines.join("\n")
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
	pub fn get_images(&self) -> &[crate::types::ImageInfo] {
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

	pub fn clear(&mut self) {
		self.lines.clear();
		self.current_line.clear();
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
		self.preserve_whitespace_depth = 0;
		self.list_level = 0;
		self.cached_char_length = 0;
		self.list_style_stack.clear();
		self.open_lists.clear();
	}

	fn process_node(&mut self, node: Node<'_, '_>) {
		let (tag_name, skip_children) = match node.node_type() {
			NodeType::Element => {
				let tag_name = node.tag_name().name();
				if Self::is_ignored_element(tag_name) {
					return;
				}
				if let Some(target) = self.position_watch
					&& self.in_body && self.get_current_text_position() <= target
				{
					self.watched_byte_offset = Some(node.range().start);
				}
				let skip_children = self.handle_element_opening_xml(tag_name, node);
				self.handle_heading_xml(tag_name, node);
				(Some(tag_name), skip_children)
			}
			NodeType::Text => {
				self.process_text_node(node);
				(None, false)
			}
			_ => (None, false),
		};
		if !skip_children {
			for child in node.children() {
				self.process_node(child);
			}
		}
		if let Some(tag_name) = tag_name {
			self.handle_element_closing_xml(tag_name);
		}
	}

	fn handle_element_opening_xml(&mut self, tag_name: &str, node: Node<'_, '_>) -> bool {
		let mut skip_children = false;
		if Self::tag_is(tag_name, "table") {
			self.handle_table_xml(node);
			return true;
		}
		if Self::tag_is(tag_name, "hr") && self.in_body {
			self.finalize_current_line();
			let offset = self.get_current_text_position();
			let line = Self::separator_line();
			self.current_line.push_str(line);
			self.finalize_current_line();
			self.separators.push(SeparatorInfo { offset, length: display_len(line) });
			return true;
		}
		if Self::tag_is(tag_name, "pagenum") {
			let text = collapse_whitespace(&collect_element_text(node)).trim().to_string();
			self.page_breaks.push(PageBreakInfo { offset: self.get_current_text_position(), text });
			return true;
		}
		if Self::tag_is(tag_name, "section") {
			self.section_offsets.push(self.get_current_text_position());
		}
		if Self::tag_is(tag_name, "a") {
			let link_text = collect_element_text(node);
			if !link_text.is_empty() {
				let href = node.attribute("href").unwrap_or("").to_string();
				let processed_link_text = collapse_whitespace(&link_text);
				let link_offset = self.get_current_text_position();
				self.current_line.push_str(&processed_link_text);
				self.links.push(LinkInfo { offset: link_offset, text: processed_link_text, reference: href });
				skip_children = true;
			}
		} else if Self::tag_is(tag_name, "body")
			|| Self::tag_is(tag_name, "book")
			|| Self::tag_is(tag_name, "frontmatter")
			|| Self::tag_is(tag_name, "bodymatter")
			|| Self::tag_is(tag_name, "rearmatter")
		{
			self.in_body = true;
		} else if Self::tag_is(tag_name, "pre") {
			self.finalize_current_line();
			self.start_preserve_whitespace();
		} else if Self::tag_is(tag_name, "code") {
			self.start_preserve_whitespace();
		} else if Self::tag_is(tag_name, "br") {
			self.finalize_current_line();
		} else if Self::tag_is(tag_name, "li") {
			self.handle_list_item_xml(node);
		} else if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") || Self::tag_is(tag_name, "list") {
			self.handle_list_start_xml(tag_name, node);
		}
		if self.in_body {
			if let Some(id) = node.attribute("id").or_else(|| node.attribute("name")) {
				self.id_positions.insert(id.to_string(), self.get_current_text_position());
			}
			if Self::tag_is(tag_name, "img") || Self::tag_is(tag_name, "image") || Self::tag_is(tag_name, "figure") {
				let mut description = node
					.attribute("alt")
					.or_else(|| node.attribute("aria-label"))
					.or_else(|| node.attribute("aria-description"))
					.or_else(|| node.attribute("title"))
					.map(collapse_whitespace)
					.unwrap_or_default();

				if description.is_empty() && Self::tag_is(tag_name, "figure") {
					for child in node.children() {
						if child.is_element() && Self::tag_is(child.tag_name().name(), "figcaption") {
							description = collapse_whitespace(&collect_element_text(child));
							break;
						}
					}
				}

				if !description.is_empty() {
					let is_figure = Self::tag_is(tag_name, "figure");
					let label = if is_figure { t("Figure") } else { t("Image") };
					let image_text = format!("[{label}: {description}]");
					let offset = self.get_current_text_position();
					self.current_line.push_str(&image_text);
					let info = crate::types::ImageInfo { offset, alt_text: description };
					if is_figure {
						self.figures.push(info);
					} else {
						self.images.push(info);
					}
				}
			}
		}
		skip_children
	}

	fn handle_table_xml(&mut self, node: Node<'_, '_>) {
		self.finalize_current_line();
		let table_xml = node.document().input_text()[node.range()].to_string();
		let start_offset = self.get_current_text_position();
		// Emit the table's on-screen text via the shared helper instead of recursing children to
		// emit one cell per line. The helper output may contain tabs and span multiple lines; push
		// each line verbatim so tab separators and empty cells survive whitespace collapsing.
		let render = crate::parser::table_text::table_render_bundle(&table_xml, self.render_tables_inline);
		for line in render.lines {
			self.push_finalized_line(line);
		}
		let table_caption = render.caption;
		let display_length = render.display_length;
		self.tables.push(TableInfo {
			offset: start_offset,
			text: table_caption,
			html_content: table_xml,
			length: display_length,
		});
	}

	/// Push a line to the output verbatim (no whitespace collapsing/trimming), updating the cached
	/// length so position tracking stays correct. Used for table rows whose tab separators and empty
	/// cells must not be mangled by `add_line`.
	fn push_finalized_line(&mut self, line: String) {
		crate::parser::table_text::push_finalized_line(&mut self.lines, &mut self.cached_char_length, line);
	}

	fn handle_list_item_xml(&mut self, node: Node<'_, '_>) {
		self.finalize_current_line();
		let li_text = collect_element_text(node);
		self.list_items.push(ListItemInfo {
			offset: self.get_current_text_position(),
			level: self.list_level,
			text: li_text,
		});
		let indent = usize::try_from(self.list_level).unwrap_or(0) * 2;
		self.current_line.push_str(&" ".repeat(indent));
		let bullet = if let Some(style) = self.list_style_stack.last_mut() {
			if style.ordered {
				let item_text = format_list_item(style.item_number, &style.list_type);
				style.item_number += 1;
				format!("{item_text}. ")
			} else {
				format!("{} ", Self::get_bullet_for_level(self.list_level))
			}
		} else {
			format!("{} ", Self::get_bullet_for_level(self.list_level))
		};
		self.current_line.push_str(&bullet);
	}

	fn handle_list_start_xml(&mut self, tag_name: &str, node: Node<'_, '_>) {
		self.list_level += 1;
		let mut style = ListStyle::default();
		if Self::tag_is(tag_name, "ol") {
			style.ordered = true;
			if let Some(start_val) = node.attribute("start")
				&& let Ok(start_num) = start_val.parse::<i32>()
			{
				style.item_number = start_num;
			}
			if let Some(type_val) = node.attribute("type") {
				style.list_type = type_val.to_lowercase();
			}
		}
		self.list_style_stack.push(style);
		let mut item_count = 0;
		for child in node.children() {
			if child.node_type() == NodeType::Element && child.tag_name().name().eq_ignore_ascii_case("li") {
				item_count += 1;
			}
		}
		if item_count > 0 {
			self.finalize_current_line();
			self.open_lists.push(Some(self.lists.len()));
			self.lists.push(ListInfo { offset: self.get_current_text_position(), item_count, length: 0 });
		} else {
			self.open_lists.push(None);
		}
	}

	fn handle_heading_xml(&mut self, tag_name: &str, node: Node<'_, '_>) {
		if self.in_body {
			let mut chars = tag_name.chars();
			if let (Some(h), Some(level_char)) = (chars.next(), chars.next())
				&& h.eq_ignore_ascii_case(&'h')
				&& level_char.is_ascii_digit()
			{
				let level = level_char as u8 - b'0';
				if (1..=6).contains(&level) {
					self.finalize_current_line();
					let heading_offset = self.get_current_text_position();
					let text = collect_element_text(node);
					if !text.is_empty() {
						let normalized = trim_string(&collapse_whitespace(&text));
						if !normalized.is_empty() {
							self.headings.push(HeadingInfo {
								offset: heading_offset,
								level: i32::from(level),
								text: normalized,
							});
						}
					}
				}
			}
		}
	}

	fn handle_element_closing_xml(&mut self, tag_name: &str) {
		let is_pre = Self::tag_is(tag_name, "pre");
		if is_pre {
			self.finalize_current_line();
			self.stop_preserve_whitespace();
		} else {
			if Self::is_block_element(tag_name) {
				self.finalize_current_line();
			}
			if Self::tag_is(tag_name, "code") {
				self.stop_preserve_whitespace();
			}
		}
		if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") {
			self.list_level = (self.list_level - 1).max(0);
			self.list_style_stack.pop();
			if let Some(open) = self.open_lists.pop().flatten() {
				self.finalize_current_line();
				let offset = self.lists[open].offset;
				self.lists[open].length = self.get_current_text_position().saturating_sub(offset);
			}
		}
	}

	const fn start_preserve_whitespace(&mut self) {
		self.preserve_whitespace_depth += 1;
	}

	const fn stop_preserve_whitespace(&mut self) {
		if self.preserve_whitespace_depth > 0 {
			self.preserve_whitespace_depth -= 1;
		}
	}

	const fn is_preserving_whitespace(&self) -> bool {
		self.preserve_whitespace_depth > 0
	}

	fn process_text_node(&mut self, node: Node<'_, '_>) {
		if !self.in_body {
			return;
		}
		if let Some(text) = node.text() {
			if text.is_empty() {
				return;
			}
			let processed_text = remove_soft_hyphens(text);
			if self.is_preserving_whitespace() {
				self.current_line.push_str(&processed_text);
			} else {
				let mut collapsed = collapse_whitespace(&processed_text);
				if self.current_line.is_empty() {
					collapsed = collapsed.trim_start().to_string();
				} else if self.current_line.ends_with(' ') && collapsed.starts_with(' ') {
					collapsed.remove(0);
				}
				if !collapsed.is_empty() {
					self.current_line.push_str(&collapsed);
				}
			}
		}
	}

	fn add_line(&mut self, mut line: String) {
		if self.is_preserving_whitespace() {
			while line.ends_with(['\n', '\r']) {
				line.pop();
			}
			self.cached_char_length += display_len(&line) + 1;
			self.lines.push(line);
		} else {
			let collapsed = collapse_whitespace(&line);
			let collapsed = collapsed.trim().to_string();
			if collapsed.is_empty() {
				return;
			}
			self.cached_char_length += display_len(&collapsed) + 1;
			self.lines.push(collapsed);
		}
	}

	const fn separator_line() -> &'static str {
		"----------------------------------------"
	}

	fn finalize_current_line(&mut self) {
		let line = mem::take(&mut self.current_line);
		self.add_line(line);
	}

	fn current_display_len(&self) -> usize {
		if self.is_preserving_whitespace() {
			return display_len(&self.current_line);
		}
		let collapsed = collapse_whitespace(&self.current_line);
		// Use trim_start() not trim(): trailing whitespace before an inline element IS
		// preserved in the output line, so including it in the position count keeps
		// link/anchor offsets correctly aligned with the final text.
		let trimmed = collapsed.trim_start();
		display_len(trimmed)
	}

	fn get_current_text_position(&self) -> usize {
		self.cached_char_length + self.current_display_len()
	}

	fn is_block_element(tag_name: &str) -> bool {
		[
			"div",
			"p",
			"pre",
			"h1",
			"h2",
			"h3",
			"h4",
			"h5",
			"h6",
			"blockquote",
			"ul",
			"ol",
			"list",
			"li",
			"dl",
			"dt",
			"dd",
			"section",
			"article",
			"header",
			"footer",
			"nav",
			"aside",
			"main",
			"figure",
			"figcaption",
			"address",
			"hr",
			"table",
			"thead",
			"tbody",
			"tfoot",
			"tr",
			"td",
			"th",
			"level1",
			"level2",
			"level3",
			"level4",
			"level5",
			"level6",
			"frontmatter",
			"bodymatter",
			"rearmatter",
			"doctitle",
			"docauthor",
		]
		.iter()
		.any(|t| Self::tag_is(tag_name, t))
	}

	fn is_ignored_element(tag_name: &str) -> bool {
		["script", "style", "noscript", "iframe", "object", "embed"].iter().any(|t| Self::tag_is(tag_name, t))
	}

	const fn tag_is(tag_name: &str, expected: &str) -> bool {
		tag_name.eq_ignore_ascii_case(expected)
	}

	const fn get_bullet_for_level(level: i32) -> &'static str {
		match level {
			1 => "\u{2022}",
			2 => "\u{25E6}",
			_ => "-",
		}
	}
}

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

impl crate::parser::ConverterOutput for XmlToText {
	fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}
	fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}
	fn get_images(&self) -> &[crate::types::ImageInfo] {
		&self.images
	}
	fn get_figures(&self) -> &[crate::types::ImageInfo] {
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
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[test]
	fn test_link_collection() {
		let xml = "<root><body><a href=\"https://example.com\">Hello   world</a></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let links = converter.get_links();
		assert_eq!(links.len(), 1);
		assert_eq!(links[0].text, "Hello world");
		assert_eq!(links[0].reference, "https://example.com");
		assert_eq!(converter.get_text(), "Hello world");
	}

	#[test]
	fn test_heading_normalization() {
		let xml = "<root><body><h2>  Hello \n world </h2></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let headings = converter.get_headings();
		assert_eq!(headings.len(), 1);
		assert_eq!(headings[0].level, 2);
		assert_eq!(headings[0].text, "Hello world");
	}

	#[test]
	fn test_ordered_list_metadata() {
		let xml = "<root><body><ol start=\"2\"><li>One</li><li>Two</li></ol></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let lists = converter.get_lists();
		let items = converter.get_list_items();
		assert_eq!(lists.len(), 1);
		assert_eq!(lists[0].item_count, 2);
		assert_eq!(items.len(), 2);
		assert_eq!(items[0].level, 1);
		assert_eq!(items[0].text, "One");
		assert_eq!(items[1].text, "Two");
	}

	#[rstest]
	#[case("h1", 1)]
	#[case("h2", 2)]
	#[case("h3", 3)]
	#[case("h4", 4)]
	#[case("h5", 5)]
	#[case("h6", 6)]
	fn heading_levels_h1_to_h6(#[case] tag: &str, #[case] expected_level: i32) {
		let xml = format!("<root><body><{tag}>Title</{tag}></body></root>");
		let mut converter = XmlToText::new();
		assert!(converter.convert(&xml));
		let headings = converter.get_headings();
		assert_eq!(headings.len(), 1);
		assert_eq!(headings[0].level, expected_level);
		assert_eq!(headings[0].text, "Title");
	}

	#[test]
	fn hr_produces_separator() {
		let xml = "<root><body><p>Before</p><hr/><p>After</p></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		assert_eq!(converter.get_separators().len(), 1);
	}

	#[test]
	fn unordered_list_items_have_level_one() {
		let xml = "<root><body><ul><li>First</li><li>Second</li></ul></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let items = converter.get_list_items();
		assert_eq!(items.len(), 2);
		assert_eq!(items[0].level, 1);
		assert_eq!(items[1].level, 1);
		assert_eq!(items[0].text, "First");
	}

	#[test]
	fn nested_list_increments_level() {
		let xml = "<root><body><ul><li>A</li><ul><li>B</li></ul></ul></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let items = converter.get_list_items();
		assert!(items.len() >= 2, "expected at least two list items");
		let level_a = items.iter().find(|i| i.text == "A").map(|i| i.level).unwrap_or(0);
		let level_b = items.iter().find(|i| i.text == "B").map(|i| i.level).unwrap_or(0);
		assert!(level_b > level_a, "nested item should have a higher level");
	}

	#[test]
	fn table_is_detected() {
		let xml = "<root><body><table><tr><td>Cell</td></tr></table></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		assert_eq!(converter.get_tables().len(), 1);
	}

	#[test]
	fn find_anchor_byte_offset_locates_block_containing_position() {
		let xml = "<root><body><p>First paragraph.</p><p>Second paragraph.</p></body></root>";
		// Text output: "First paragraph.\nSecond paragraph." — second paragraph starts at 17.
		let mut converter = XmlToText::new();
		let offset = converter.find_anchor_byte_offset(xml, 20).expect("offset for position in second paragraph");
		assert!(xml[offset..].starts_with("<p>Second"), "got offset {offset}: {}", &xml[offset..]);
		let offset = converter.find_anchor_byte_offset(xml, 5).expect("offset for position in first paragraph");
		assert!(xml[offset..].starts_with("<p>First"), "got offset {offset}: {}", &xml[offset..]);
	}

	#[test]
	fn find_anchor_byte_offset_at_position_zero_uses_first_body_element() {
		let xml = "<root><head><title>T</title></head><body><p>First.</p></body></root>";
		let mut converter = XmlToText::new();
		let offset = converter.find_anchor_byte_offset(xml, 0).expect("offset at start");
		assert!(xml[offset..].starts_with("<p>First."), "got offset {offset}: {}", &xml[offset..]);
	}

	#[test]
	fn find_anchor_byte_offset_picks_nearest_inline_element() {
		let xml = "<root><body><p>Start <em>middle</em> end of line</p></body></root>";
		// Position inside " end of line" — nearest preceding element start is <em>.
		let mut converter = XmlToText::new();
		let offset = converter.find_anchor_byte_offset(xml, 16).expect("offset for position after em");
		assert!(xml[offset..].starts_with("<em>"), "got offset {offset}: {}", &xml[offset..]);
	}

	#[test]
	fn find_anchor_byte_offset_returns_none_for_invalid_xml() {
		let mut converter = XmlToText::new();
		assert_eq!(converter.find_anchor_byte_offset("<p>broken", 0), None);
	}

	#[test]
	fn inject_anchor_at_position_inserts_span_before_block() {
		let xml = "<root><body><p>First paragraph.</p><p>Second paragraph.</p></body></root>";
		let result = inject_anchor_at_position(xml, 20, "reading-pos").expect("injection succeeds");
		assert!(result.contains(r#"</p><span id="reading-pos"></span><p>Second paragraph.</p>"#), "got: {result}");
	}

	#[test]
	fn inject_anchor_at_position_returns_none_for_invalid_xml() {
		assert_eq!(inject_anchor_at_position("<p>broken", 0, "reading-pos"), None);
	}

	#[test]
	fn dl_dt_dd_produce_separate_lines() {
		let xml = "<root><body><dl><dt>Term</dt><dd>Definition</dd></dl></body></root>";
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
		let text = converter.get_text();
		let lines: Vec<&str> = text.lines().collect();
		assert!(lines.iter().any(|l| *l == "Term"), "dt content should be on its own line");
		assert!(lines.iter().any(|l| *l == "Definition"), "dd content should be on its own line");
	}
	/// `TableInfo.length` must equal the emitted display extent (display units), NOT the
	/// emitted text's byte length. Prefix text ensures start_offset > 0. With inline rendering the
	/// emitted row is the TSV "A\t𝄞"; a non-BMP char (U+1D11E, G Clef, width 2) locks the math.
	#[test]
	fn xml_table_display_length_is_display_extent_not_byte_length() {
		// "Intro\n" → 6 display units. Inline table row: "A\t𝄞" = 4 display units + newline = 5.
		let xml = concat!(
			"<root><body><p>Intro</p>",
			"<table><tr><td>A</td><td>\u{1D11E}</td></tr></table>",
			"</body></root>"
		);
		let mut converter = XmlToText::with_render_tables_inline(true);
		assert!(converter.convert(xml));
		let tables = converter.get_tables();
		assert_eq!(tables.len(), 1, "expected exactly one table");
		let table = &tables[0];

		assert_eq!(table.offset, 6, "table starts after 'Intro\\n'");
		// display_length = 5 (display extent); emitted byte length = 6 — they differ.
		assert_eq!(table.length, 5, "length must be the display extent (5), not byte length (6)");
	}

	/// OFF mode emits the `"[Table]: <first row>"` placeholder; ON mode emits the full TSV.
	#[test]
	fn xml_table_emits_placeholder_or_tsv_by_flag() {
		let xml = "<root><body><table><tr><td>A</td><td>B</td></tr><tr><td>c</td><td>d</td></tr></table></body></root>";

		let mut off = XmlToText::new();
		assert!(off.convert(xml));
		assert_eq!(off.get_text(), "[Table]: A B");

		let mut on = XmlToText::with_render_tables_inline(true);
		assert!(on.convert(xml));
		assert_eq!(on.get_text(), "A\tB\nc\td");
	}

	/// Two XML tables: second table's offset equals first offset + first display_length.
	#[test]
	fn xml_two_tables_offsets_are_cumulative() {
		let xml = concat!(
			"<root><body>",
			"<table><tr><td>X</td></tr></table>",
			"<table><tr><td>Y</td></tr></table>",
			"</body></root>"
		);
		let mut converter = XmlToText::new();
		assert!(converter.convert(xml));
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
