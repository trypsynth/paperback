use std::collections::HashMap;

use roxmltree::{Document, Node, NodeType, ParsingOptions};

use crate::{
	html_to_text::{HeadingInfo, LinkInfo, ListInfo, ListItemInfo},
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
		if self.lines.is_empty() {
			return String::new();
		}
		let mut result = String::new();
		for (idx, line) in self.lines.iter().enumerate() {
			result.push_str(line);
			if idx + 1 < self.lines.len() {
				result.push('\n');
			}
		}
		result
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
		let mut tag_name = String::new();
		let mut skip_children = false;
		match node.node_type() {
			NodeType::Element => {
				tag_name = node.tag_name().name().to_ascii_lowercase();
				if matches!(tag_name.as_str(), "script" | "style" | "noscript" | "iframe" | "object" | "embed") {
					return;
				}
				if tag_name == "section" {
					self.section_offsets.push(self.get_current_text_position());
				}
				if tag_name == "a" {
					let link_text = Self::get_element_text(node);
					if !link_text.is_empty() {
						let href = node.attribute("href").unwrap_or("").to_string();
						let processed_link_text = collapse_whitespace(&link_text);
						let link_offset = self.get_current_text_position();
						self.current_line.push_str(&processed_link_text);
						self.links.push(LinkInfo { offset: link_offset, text: processed_link_text, reference: href });
						skip_children = true;
					}
				} else if tag_name == "body" {
					self.in_body = true;
				} else if tag_name == "pre" {
					self.finalize_current_line();
					self.preserve_whitespace = true;
				} else if tag_name == "br" {
					self.finalize_current_line();
				} else if tag_name == "li" {
					self.finalize_current_line();
					let li_text = Self::get_element_text(node);
					self.list_items.push(ListItemInfo {
						offset: self.get_current_text_position(),
						level: self.list_level,
						text: li_text,
					});
					self.current_line.push_str(&" ".repeat(usize::try_from(self.list_level * 2).unwrap_or(0)));
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
				} else if tag_name == "ul" || tag_name == "ol" {
					self.list_level += 1;
					let mut style = ListStyle::default();
					if tag_name == "ol" {
						style.ordered = true;
					}
					self.list_style_stack.push(style);
					let mut item_count = 0;
					for child in node.children() {
						if child.node_type() == NodeType::Element && child.tag_name().name().eq_ignore_ascii_case("li")
						{
							item_count += 1;
						}
					}
					if item_count > 0 {
						self.finalize_current_line();
						self.lists.push(ListInfo { offset: self.get_current_text_position(), item_count });
					}
				}
				if self.in_body {
					if let Some(id) = node.attribute("id") {
						self.id_positions.insert(id.to_string(), self.get_current_text_position());
					}
				}
				if self.in_body
					&& tag_name.len() == 2
					&& tag_name.starts_with('h')
					&& tag_name.as_bytes()[1].is_ascii_digit()
				{
					let level = tag_name.as_bytes()[1] - b'0';
					if (1..=6).contains(&level) {
						self.finalize_current_line();
						let heading_offset = self.get_current_text_position();
						let text = Self::get_element_text(node);
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
			NodeType::Text => {
				self.process_text_node(node);
			}
			_ => {}
		}
		if !skip_children {
			for child in node.children() {
				self.process_node(child);
			}
		}
		if node.node_type() == NodeType::Element {
			if Self::is_block_element(&tag_name) {
				self.finalize_current_line();
			}
			if tag_name == "pre" {
				self.preserve_whitespace = false;
			}
			if tag_name == "ul" || tag_name == "ol" {
				self.list_level = (self.list_level - 1).max(0);
				self.list_style_stack.pop();
			}
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

	fn add_line(&mut self, line: &str) {
		if self.preserve_whitespace {
			let mut processed_line = line.to_string();
			while processed_line.ends_with(['\n', '\r']) {
				processed_line.pop();
			}
			self.cached_char_length += display_len(&processed_line) + 1;
			self.lines.push(processed_line);
		} else {
			let trimmed = trim_string(&collapse_whitespace(line));
			if !trimmed.is_empty() {
				self.cached_char_length += display_len(&trimmed) + 1;
				self.lines.push(trimmed);
			}
		}
	}

	fn finalize_current_line(&mut self) {
		self.add_line(&self.current_line.clone());
		self.current_line.clear();
	}

	fn get_current_text_position(&self) -> usize {
		let trimmed = self.current_line.trim_end_matches(' ');
		self.cached_char_length + display_len(trimmed)
	}

	fn get_element_text(node: Node<'_, '_>) -> String {
		Self::collect_text(node)
	}

	fn collect_text(node: Node<'_, '_>) -> String {
		match node.node_type() {
			NodeType::Text => node.text().unwrap_or("").to_string(),
			NodeType::Comment => String::new(),
			NodeType::Element => node.children().map(Self::collect_text).collect(),
			_ => String::new(),
		}
	}

	fn is_block_element(tag_name: &str) -> bool {
		matches!(
			tag_name,
			"div"
				| "p" | "pre"
				| "h1" | "h2"
				| "h3" | "h4"
				| "h5" | "h6"
				| "blockquote"
				| "ul" | "ol"
				| "li" | "section"
				| "article" | "header"
				| "footer" | "nav"
				| "aside" | "main"
				| "figure" | "figcaption"
				| "address" | "hr"
				| "table" | "thead"
				| "tbody" | "tfoot"
				| "tr" | "td"
				| "th"
		)
	}

	const fn get_bullet_for_level(level: i32) -> &'static str {
		match level {
			1 => "\u{2022}",
			2 => "\u{25E6}",
			_ => "-",
		}
	}
}
