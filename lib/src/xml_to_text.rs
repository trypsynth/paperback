use std::collections::HashMap;

use roxmltree::{Document, Node, NodeType, ParsingOptions};

use crate::{
	html_to_text::{HeadingInfo, LinkInfo, ListInfo, ListItemInfo},
	parser::utils::collect_element_text,
	utils::text::{collapse_whitespace, display_len, remove_soft_hyphens, trim_string},
};

#[derive(Clone, Copy, Default)]
struct ListStyle {
	ordered: bool,
	item_number: i32,
}

#[derive(Default)]
pub struct XmlToText {
	lines: Vec<String>,
	current_line: String,
	id_positions: HashMap<String, usize>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	section_offsets: Vec<usize>,
	in_body: bool,
	preserve_whitespace: bool,
	list_level: i32,
	list_style_stack: Vec<ListStyle>,
	cached_char_length: usize,
}

impl XmlToText {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
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
		self.lists.clear();
		self.list_items.clear();
		self.section_offsets.clear();
		self.in_body = false;
		self.preserve_whitespace = false;
		self.list_level = 0;
		self.cached_char_length = 0;
		self.list_style_stack.clear();
	}

	fn process_node(&mut self, node: Node<'_, '_>) {
		let (tag_name, skip_children) = match node.node_type() {
			NodeType::Element => {
				let tag_name = node.tag_name().name();
				if Self::is_ignored_element(tag_name) {
					return;
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
		} else if Self::tag_is(tag_name, "body") {
			self.in_body = true;
		} else if Self::tag_is(tag_name, "pre") {
			self.finalize_current_line();
			self.preserve_whitespace = true;
		} else if Self::tag_is(tag_name, "br") {
			self.finalize_current_line();
		} else if Self::tag_is(tag_name, "li") {
			self.handle_list_item_xml(node);
		} else if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") {
			self.handle_list_start_xml(tag_name, node);
		}
		if self.in_body {
			if let Some(id) = node.attribute("id") {
				self.id_positions.insert(id.to_string(), self.get_current_text_position());
			}
		}
		skip_children
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
				let bullet = format!("{}. ", style.item_number);
				style.item_number += 1;
				bullet
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
		if tag_name == "ol" {
			style.ordered = true;
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
			self.lists.push(ListInfo { offset: self.get_current_text_position(), item_count });
		}
	}

	fn handle_heading_xml(&mut self, tag_name: &str, node: Node<'_, '_>) {
		if self.in_body {
			let mut chars = tag_name.chars();
			if let (Some(h), Some(level_char)) = (chars.next(), chars.next()) {
				if h.eq_ignore_ascii_case(&'h') && level_char.is_ascii_digit() {
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
	}

	fn handle_element_closing_xml(&mut self, tag_name: &str) {
		if Self::is_block_element(tag_name) {
			self.finalize_current_line();
		}
		if Self::tag_is(tag_name, "pre") {
			self.preserve_whitespace = false;
		}
		if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") {
			self.list_level = (self.list_level - 1).max(0);
			self.list_style_stack.pop();
		}
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
			if self.preserve_whitespace {
				self.current_line.push_str(&processed_text);
			} else {
				self.current_line.push_str(&collapse_whitespace(&processed_text));
			}
		}
	}

	fn add_line(&mut self, mut line: String) {
		if self.preserve_whitespace {
			while line.ends_with(['\n', '\r']) {
				line.pop();
			}
			self.cached_char_length += display_len(&line) + 1;
			self.lines.push(line);
		} else {
			let collapsed = collapse_whitespace(&line);
			if collapsed.trim().is_empty() {
				return;
			}
			self.cached_char_length += display_len(&collapsed) + 1;
			self.lines.push(collapsed);
		}
	}

	fn finalize_current_line(&mut self) {
		let line = std::mem::take(&mut self.current_line);
		self.add_line(line);
	}

	fn get_current_text_position(&self) -> usize {
		let trimmed = self.current_line.trim_end_matches(' ');
		self.cached_char_length + display_len(trimmed)
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
			"li",
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
		]
		.iter()
		.any(|t| Self::tag_is(tag_name, t))
	}

	fn is_ignored_element(tag_name: &str) -> bool {
		["script", "style", "noscript", "iframe", "object", "embed"].iter().any(|t| Self::tag_is(tag_name, t))
	}

	fn tag_is(tag_name: &str, expected: &str) -> bool {
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
