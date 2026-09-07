//! The actual tree-walk: [`XmlToText::process_node`] and everything it dispatches to per tag.

use roxmltree::{Node, NodeType};

use super::XmlToText;
use crate::{
	parser::{
		convert::{
			block_elements::is_block_element, format_spans::FormatKind, line_builder::LineBuilder,
			list_style::ListStyle, table_text::table_render_bundle,
		},
		util::xml::collect_element_text,
	},
	t,
	types::{HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, PageBreakInfo, SeparatorInfo, TableInfo},
	util::text::{collapse_whitespace, display_len, format_list_item, remove_soft_hyphens, trim_string},
};

impl XmlToText {
	pub(super) fn process_node(&mut self, node: Node<'_, '_>) {
		let (tag_name, skip_children) = match node.node_type() {
			NodeType::Element => {
				let tag_name = node.tag_name().name();
				if Self::is_ignored_element(tag_name) {
					return;
				}
				if let Some(target) = self.position_watch
					&& self.in_body && self.text.get_current_text_position() <= target
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

	/// Re-anchors `node`'s `id_positions` entry (if any) to `position`. Headings, list items,
	/// tables and `<hr>` call `finalize_current_line` while opening, which moves the text
	/// position past where `handle_element_opening_xml` first captured the id.
	fn resync_id_position(&mut self, node: Node<'_, '_>, position: usize) {
		if self.in_body
			&& let Some(id) = node.attribute("id").or_else(|| node.attribute("name"))
		{
			self.id_positions.insert(id.to_string(), position);
		}
	}

	/// Records every id *inside* `node` at `position`. `<a>` and `<table>` emit their text
	/// through a helper and skip recursing into their children, so without this the ids on
	/// those children are never seen at all, and a DAISY SMIL `<par>` anchored to one (a
	/// narrated `<em>` inside a link, say) loses its audio clip entirely. `position` is the
	/// containing element's own start, since the skipped subtree's text is emitted as one
	/// run with no per-descendant offsets to attribute; for the common case of an element
	/// wrapping the whole link that is exactly right.
	fn record_descendant_ids(&mut self, node: Node<'_, '_>, position: usize) {
		if !self.in_body {
			return;
		}
		for descendant in node.descendants().skip(1) {
			if descendant.is_element()
				&& let Some(id) = descendant.attribute("id").or_else(|| descendant.attribute("name"))
			{
				self.id_positions.entry(id.to_string()).or_insert(position);
			}
		}
	}

	fn handle_element_opening_xml(&mut self, tag_name: &str, node: Node<'_, '_>) -> bool {
		let mut skip_children = false;
		// Recorded before the tag-specific handling below, which (notably for `<a>`) can push
		// this element's own text into `current_line` and shift the position past its start.
		if self.in_body
			&& let Some(id) = node.attribute("id").or_else(|| node.attribute("name"))
		{
			self.id_positions.insert(id.to_string(), self.text.get_current_text_position());
		}
		if Self::tag_is(tag_name, "table") {
			self.handle_table_xml(node);
			return true;
		}
		if Self::tag_is(tag_name, "hr") && self.in_body {
			self.text.finalize_current_line();
			let offset = self.text.get_current_text_position();
			self.resync_id_position(node, offset);
			let line = LineBuilder::separator_line();
			self.text.current_line.push_str(line);
			self.text.finalize_current_line();
			self.separators.push(SeparatorInfo { offset, length: display_len(line) });
			return true;
		}
		if Self::tag_is(tag_name, "pagenum") {
			let text = collapse_whitespace(&collect_element_text(node)).trim().to_string();
			self.page_breaks.push(PageBreakInfo { offset: self.text.get_current_text_position(), text });
			return true;
		}
		if Self::tag_is(tag_name, "section") {
			self.section_offsets.push(self.text.get_current_text_position());
		}
		if Self::tag_is(tag_name, "a") {
			let link_text = collect_element_text(node);
			if !link_text.is_empty() {
				let href = node.attribute("href").unwrap_or("").to_string();
				let processed_link_text = collapse_whitespace(&link_text);
				let link_offset = self.text.get_current_text_position();
				self.text.current_line.push_str(&processed_link_text);
				self.links.push(LinkInfo { offset: link_offset, text: processed_link_text, reference: href });
				self.record_descendant_ids(node, link_offset);
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
			self.text.finalize_current_line();
			self.text.start_preserve_whitespace();
		} else if Self::tag_is(tag_name, "code") {
			self.text.start_preserve_whitespace();
		} else if Self::tag_is(tag_name, "br") {
			self.text.finalize_current_line();
		} else if Self::tag_is(tag_name, "li") {
			self.handle_list_item_xml(node);
		} else if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") || Self::tag_is(tag_name, "list") {
			self.handle_list_start_xml(tag_name, node);
		} else if Self::tag_is(tag_name, "b") || Self::tag_is(tag_name, "strong") {
			self.format_spans.open(&FormatKind::Bold, self.text.get_current_text_position());
		} else if Self::tag_is(tag_name, "i") || Self::tag_is(tag_name, "em") {
			self.format_spans.open(&FormatKind::Italic, self.text.get_current_text_position());
		} else if Self::tag_is(tag_name, "u") {
			self.format_spans.open(&FormatKind::Underline, self.text.get_current_text_position());
		}
		if self.in_body
			&& (Self::tag_is(tag_name, "img") || Self::tag_is(tag_name, "image") || Self::tag_is(tag_name, "figure"))
		{
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
				// TRANSLATORS: Label inserted before a figure or image's description, e.g. "[Figure: a cat sleeping]"
				let label = if is_figure { t("Figure") } else { t("Image") };
				let image_text = format!("[{label}: {description}]");
				let offset = self.text.get_current_text_position();
				self.text.current_line.push_str(&image_text);
				let info = ImageInfo { offset, alt_text: description, length: display_len(&image_text) };
				if is_figure {
					self.figures.push(info);
				} else {
					self.images.push(info);
				}
			}
		}
		skip_children
	}

	fn handle_table_xml(&mut self, node: Node<'_, '_>) {
		self.text.finalize_current_line();
		let table_xml = node.document().input_text()[node.range()].to_string();
		let start_offset = self.text.get_current_text_position();
		self.resync_id_position(node, start_offset);
		self.record_descendant_ids(node, start_offset);
		// Emit the table's on-screen text via the shared helper instead of recursing children to
		// emit one cell per line. The helper output may contain tabs and span multiple lines; push
		// each line verbatim so tab separators and empty cells survive whitespace collapsing.
		let render = table_render_bundle(&table_xml, self.render_tables_inline);
		for line in render.lines {
			self.text.push_finalized_line(line);
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

	fn handle_list_item_xml(&mut self, node: Node<'_, '_>) {
		self.text.finalize_current_line();
		self.resync_id_position(node, self.text.get_current_text_position());
		let li_text = collect_element_text(node);
		self.list_items.push(ListItemInfo {
			offset: self.text.get_current_text_position(),
			level: self.list_level,
			text: li_text,
		});
		let indent = usize::try_from(self.list_level).unwrap_or(0) * 2;
		self.text.current_line.push_str(&" ".repeat(indent));
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
		self.text.current_line.push_str(&bullet);
	}

	fn handle_list_start_xml(&mut self, tag_name: &str, node: Node<'_, '_>) {
		self.list_level += 1;
		let style = ListStyle::new(Self::tag_is(tag_name, "ol"), node.attribute("start"), node.attribute("type"));
		self.list_style_stack.push(style);
		let mut item_count = 0;
		for child in node.children() {
			if child.node_type() == NodeType::Element && child.tag_name().name().eq_ignore_ascii_case("li") {
				item_count += 1;
			}
		}
		if item_count > 0 {
			self.text.finalize_current_line();
			let offset = self.text.get_current_text_position();
			self.resync_id_position(node, offset);
			self.open_lists.push(Some(self.lists.len()));
			self.lists.push(ListInfo { offset, item_count, length: 0 });
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
					self.text.finalize_current_line();
					let heading_offset = self.text.get_current_text_position();
					self.resync_id_position(node, heading_offset);
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
			self.text.finalize_current_line();
			self.text.stop_preserve_whitespace();
		} else {
			if is_block_element(tag_name) {
				self.text.finalize_current_line();
			}
			if Self::tag_is(tag_name, "code") {
				self.text.stop_preserve_whitespace();
			} else if Self::tag_is(tag_name, "b") || Self::tag_is(tag_name, "strong") {
				self.format_spans.close(&FormatKind::Bold, self.text.get_current_text_position());
			} else if Self::tag_is(tag_name, "i") || Self::tag_is(tag_name, "em") {
				self.format_spans.close(&FormatKind::Italic, self.text.get_current_text_position());
			} else if Self::tag_is(tag_name, "u") {
				self.format_spans.close(&FormatKind::Underline, self.text.get_current_text_position());
			}
		}
		if Self::tag_is(tag_name, "ul") || Self::tag_is(tag_name, "ol") {
			self.list_level = (self.list_level - 1).max(0);
			self.list_style_stack.pop();
			if let Some(open) = self.open_lists.pop().flatten() {
				self.text.finalize_current_line();
				let offset = self.lists[open].offset;
				self.lists[open].length = self.text.get_current_text_position().saturating_sub(offset);
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
			if self.text.is_preserving_whitespace() {
				self.text.current_line.push_str(&processed_text);
			} else {
				let mut collapsed = collapse_whitespace(&processed_text);
				if self.text.current_line.is_empty() {
					collapsed = collapsed.trim_start().to_string();
				} else if self.text.current_line.ends_with(' ') && collapsed.starts_with(' ') {
					collapsed.remove(0);
				}
				if !collapsed.is_empty() {
					self.text.current_line.push_str(&collapsed);
				}
			}
		}
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
