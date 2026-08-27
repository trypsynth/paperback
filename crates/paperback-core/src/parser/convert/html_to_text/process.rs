//! The actual tree-walk: [`HtmlToText::process_node`] and everything it dispatches to per tag.

use std::fmt::Write;

use ego_tree::NodeRef;
use scraper::{ElementRef, Html, Node, node};

use super::{HtmlToText, ProcessingFlags};
use crate::{
	parser::convert::{
		block_elements::is_block_element,
		format_spans::FormatKind,
		line_builder::LineBuilder,
		list_style::ListStyle,
		table_text::{collect_dom_text, table_render_bundle},
	},
	t,
	types::{HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
	util::text::{collapse_whitespace, display_len, format_list_item, remove_soft_hyphens, trim_string},
};

impl HtmlToText {
	const fn get_bullet_for_level(level: i32) -> &'static str {
		match level {
			2 => "◦",
			3 => "*",
			4 => "-",
			_ => "•",
		}
	}

	pub(super) fn process_node(&mut self, node: NodeRef<'_, Node>, document: &Html) {
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
			let element = ElementRef::wrap(node);
			let start_attr = element.and_then(|e| e.attr("start"));
			let type_attr = element.and_then(|e| e.attr("type"));
			let style = ListStyle::new(tag_name == "ol", start_attr, type_attr);
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
