use std::collections::HashMap;

use bitflags::bitflags;
use ego_tree::NodeRef;
use scraper::{Html, Node};

use crate::utils::text::{collapse_whitespace, display_len, remove_soft_hyphens, trim_string};

bitflags! {
	#[derive(Default, Clone, Copy)]
	struct ProcessingFlags: u8 {
		const IN_BODY = 1;
		const PRESERVE_WHITESPACE = 2;
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
pub struct HeadingInfo {
	pub offset: usize,
	pub level: i32,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct LinkInfo {
	pub offset: usize,
	pub text: String,
	pub reference: String,
}

#[derive(Debug, Clone)]
pub struct ListInfo {
	pub offset: usize,
	pub item_count: i32,
}

#[derive(Debug, Clone)]
pub struct ListItemInfo {
	pub offset: usize,
	pub level: i32,
	pub text: String,
}

#[derive(Debug, Clone)]
struct ListStyle {
	ordered: bool,
	item_number: i32,
}

impl Default for ListStyle {
	fn default() -> Self {
		Self { ordered: false, item_number: 1 }
	}
}

pub struct HtmlToText {
	lines: Vec<String>,
	current_line: String,
	id_positions: HashMap<String, usize>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	title: String,
	flags: ProcessingFlags,
	current_link_href: String,
	current_link_text: String,
	list_style_stack: Vec<ListStyle>,
	list_level: i32,
	link_start_pos: usize,
	source_mode: HtmlSourceMode,
	cached_char_length: usize,
}

impl HtmlToText {
	pub fn new() -> Self {
		Self {
			lines: Vec::new(),
			current_line: String::new(),
			id_positions: HashMap::new(),
			headings: Vec::new(),
			links: Vec::new(),
			lists: Vec::new(),
			list_items: Vec::new(),
			title: String::new(),
			flags: ProcessingFlags::empty(),
			current_link_href: String::new(),
			current_link_text: String::new(),
			list_style_stack: Vec::new(),
			list_level: 0,
			link_start_pos: 0,
			source_mode: HtmlSourceMode::NativeHtml,
			cached_char_length: 0,
		}
	}

	pub fn convert(&mut self, html_content: &str, mode: HtmlSourceMode) -> bool {
		self.clear();
		self.source_mode = mode;
		let document = Html::parse_document(html_content);
		let root = document.tree.root();
		self.process_node(root, &document);
		self.finalize_current_line();
		true
	}

	pub fn get_text(&self) -> String {
		self.lines.join("\n")
	}

	pub fn get_title(&self) -> &str {
		&self.title
	}

	pub fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}

	pub fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}

	pub fn get_lists(&self) -> &[ListInfo] {
		&self.lists
	}

	pub fn get_list_items(&self) -> &[ListItemInfo] {
		&self.list_items
	}

	pub const fn get_id_positions(&self) -> &HashMap<String, usize> {
		&self.id_positions
	}

	fn clear(&mut self) {
		self.lines.clear();
		self.current_line.clear();
		self.id_positions.clear();
		self.headings.clear();
		self.links.clear();
		self.lists.clear();
		self.list_items.clear();
		self.title.clear();
		self.flags = ProcessingFlags::empty();
		self.current_link_href.clear();
		self.current_link_text.clear();
		self.list_style_stack.clear();
		self.list_level = 0;
		self.link_start_pos = 0;
		self.cached_char_length = 0;
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

	fn handle_element_opening(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if let Node::Element(element) = node.value() {
			if tag_name == "a" && !self.flags.contains(ProcessingFlags::IN_LINK) {
				self.flags.insert(ProcessingFlags::IN_LINK);
				if let Some(href) = element.attr("href") {
					self.current_link_href = href.to_string();
				}
				self.link_start_pos = self.get_current_text_position();
			}
		}
		if tag_name == "title" && self.title.is_empty() {
			self.title = Self::get_element_text(node, document);
			self.title = trim_string(&collapse_whitespace(&self.title));
		} else if tag_name == "body" {
			self.flags.insert(ProcessingFlags::IN_BODY);
		} else if tag_name == "pre" {
			self.finalize_current_line();
			self.flags.insert(ProcessingFlags::PRESERVE_WHITESPACE);
		} else if tag_name == "code" {
			self.flags.insert(ProcessingFlags::IN_CODE);
		} else if tag_name == "br" {
			self.finalize_current_line();
		}
	}

	fn handle_list_item(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if tag_name == "li" {
			self.finalize_current_line();
			let li_text = Self::get_element_text(node, document);
			self.list_items.push(ListItemInfo {
				offset: self.get_current_text_position(),
				level: self.list_level,
				text: li_text,
			});
			for _ in 0..self.list_level {
				self.current_line.push_str("  ");
			}
			if let Some(style) = self.list_style_stack.last_mut() {
				if style.ordered {
					use std::fmt::Write;
					let _ = write!(&mut self.current_line, "{}. ", style.item_number);
					style.item_number += 1;
				} else {
					self.current_line.push_str(Self::get_bullet_for_level(self.list_level));
					self.current_line.push(' ');
				}
			} else {
				self.current_line.push_str(Self::get_bullet_for_level(self.list_level));
				self.current_line.push(' ');
			}
		}
	}

	fn handle_list_start(&mut self, tag_name: &str, node: NodeRef<'_, Node>) {
		if tag_name == "ul" || tag_name == "ol" {
			self.list_level += 1;
			let mut style = ListStyle::default();
			if tag_name == "ol" {
				style.ordered = true;
			}
			self.list_style_stack.push(style);
			let mut item_count = 0;
			for child in node.children() {
				if let Node::Element(child_elem) = child.value() {
					if child_elem.name() == "li" {
						item_count += 1;
					}
				}
			}
			if item_count > 0 {
				self.finalize_current_line();
				self.lists.push(ListInfo { offset: self.get_current_text_position(), item_count });
			}
		}
	}

	fn handle_heading(&mut self, tag_name: &str, node: NodeRef<'_, Node>, document: &Html) {
		if self.flags.contains(ProcessingFlags::IN_BODY) {
			if let Node::Element(element) = node.value() {
				if let Some(id) = element.attr("id") {
					self.id_positions.insert(id.to_string(), self.cached_char_length);
				}
			}
			if tag_name.len() == 2
				&& tag_name.starts_with('h')
				&& tag_name.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
			{
				if let Some(level_char) = tag_name.chars().nth(1) {
					if let Some(level) = level_char.to_digit(10) {
						if (1..=6).contains(&level) {
							self.finalize_current_line();
							let heading_offset = self.get_current_text_position();
							let heading_text = Self::get_element_text(node, document);
							if !heading_text.is_empty() {
								#[allow(clippy::cast_possible_wrap)]
								self.headings.push(HeadingInfo {
									offset: heading_offset,
									level: level as i32,
									text: heading_text,
								});
							}
						}
					}
				}
			}
		}
	}

	fn process_element_children(&mut self, node: NodeRef<'_, Node>, document: &Html, tag_name: &str) {
		let is_markdown_code = self.source_mode == HtmlSourceMode::Markdown
			&& self.flags.contains(ProcessingFlags::IN_CODE)
			&& self.flags.contains(ProcessingFlags::PRESERVE_WHITESPACE)
			&& tag_name == "code";
		if is_markdown_code {
			for child in node.children() {
				if let Node::Element(_) = child.value() {
					let html_str = Self::serialize_node(child, document);
					self.current_line.push_str(&html_str);
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
				self.current_line.push_str(&collapsed_text);
			}
			self.current_link_href.clear();
			self.current_link_text.clear();
		}
		if tag_name == "pre" {
			self.flags.remove(ProcessingFlags::PRESERVE_WHITESPACE);
		}
		if tag_name == "code" {
			self.flags.remove(ProcessingFlags::IN_CODE);
		}
		if tag_name == "ul" || tag_name == "ol" {
			self.list_level -= 1;
			self.list_style_stack.pop();
		}
		if Self::is_block_element(tag_name) {
			self.finalize_current_line();
		}
	}

	fn handle_text_node(&mut self, text: &scraper::node::Text) {
		if !self.flags.contains(ProcessingFlags::IN_BODY) {
			return;
		}
		let text_content = text.text.to_string();
		if text_content.is_empty() {
			return;
		}
		let processed_text = remove_soft_hyphens(&text_content);
		if self.flags.contains(ProcessingFlags::PRESERVE_WHITESPACE) {
			let lines: Vec<&str> = processed_text.split('\n').collect();
			for (i, line) in lines.iter().enumerate() {
				self.current_line.push_str(line);
				if i < lines.len() - 1 {
					self.finalize_current_line();
				}
			}
		} else if self.flags.contains(ProcessingFlags::IN_LINK) {
			self.current_link_text.push_str(&collapse_whitespace(&processed_text));
		} else {
			self.current_line.push_str(&collapse_whitespace(&processed_text));
		}
	}

	fn get_element_text(node: NodeRef<'_, Node>, _document: &Html) -> String {
		Self::collect_text(node)
	}

	fn collect_text(node: NodeRef<'_, Node>) -> String {
		match node.value() {
			Node::Text(text) => text.text.to_string(),
			Node::Element(_) => node.children().map(Self::collect_text).collect(),
			_ => String::new(),
		}
	}

	fn serialize_node(node: NodeRef<'_, Node>, _document: &Html) -> String {
		match node.value() {
			Node::Element(_) => {
				scraper::ElementRef::wrap(node).map_or_else(String::new, |element_ref| element_ref.html())
			}
			Node::Text(text) => text.text.to_string(),
			_ => String::new(),
		}
	}

	fn add_line(&mut self, line: String) {
		if self.flags.contains(ProcessingFlags::PRESERVE_WHITESPACE) {
			self.cached_char_length += display_len(&line) + 1; // +1 for newline
			self.lines.push(line);
		} else {
			let processed_line = collapse_whitespace(&line);
			if processed_line.trim().is_empty() {
				return;
			}
			self.cached_char_length += display_len(&processed_line) + 1; // +1 for newline
			self.lines.push(processed_line);
		}
	}

	fn finalize_current_line(&mut self) {
		let line = std::mem::take(&mut self.current_line);
		self.add_line(line);
	}

	fn get_current_text_position(&self) -> usize {
		self.cached_char_length + display_len(&self.current_line)
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
				| "li" | "dl"
				| "dt" | "dd"
				| "section" | "article"
				| "header" | "footer"
				| "nav" | "aside"
				| "main" | "figure"
				| "figcaption"
				| "address" | "hr"
				| "table" | "thead"
				| "tbody" | "tfoot"
				| "tr" | "td"
				| "th"
		)
	}
}

impl Default for HtmlToText {
	fn default() -> Self {
		Self::new()
	}
}
