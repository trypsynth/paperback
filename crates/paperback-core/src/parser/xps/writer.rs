use super::{
	page::Page,
	structure::{Block, Fragment, FragmentKind},
};
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	parser::convert::table_text::{
		build_html_table_from_grid, display_lines_and_length, html_table_to_display, table_caption_from_html,
	},
	util::text::{collapse_whitespace, trim_string},
};

/// Writes a page that declares its structure.
pub(super) struct PageWriter<'a> {
	buffer: &'a mut DocumentBuffer,
	page: &'a Page,
	render_tables_inline: bool,
	/// A fragment need not name everything on its page, and text nothing names must still reach the reader.
	read: Vec<bool>,
}

impl<'a> PageWriter<'a> {
	pub fn new(buffer: &'a mut DocumentBuffer, page: &'a Page, render_tables_inline: bool) -> Self {
		Self { buffer, page, render_tables_inline, read: vec![false; page.runs.len()] }
	}

	fn text_of(&mut self, names: &[String]) -> String {
		let mut text = String::new();
		for name in names {
			for index in self.page.groups.get(name).cloned().unwrap_or_default() {
				self.read[index] = true;
				text.push_str(&self.page.runs[index].text);
			}
		}
		trim_string(&collapse_whitespace(&text))
	}

	fn paragraphs(&mut self, blocks: &[Block]) -> Vec<String> {
		let names =
			blocks.iter().filter_map(|block| if let Block::Paragraph(names) = block { Some(names) } else { None });
		names.map(|names| self.text_of(names)).filter(|text| !text.is_empty()).collect()
	}

	fn write_line(&mut self, text: &str) {
		if !text.is_empty() {
			self.buffer.append(text);
			self.buffer.append("\n");
		}
	}

	/// Word draws a bullet as a paragraph of its own, which means nothing read aloud; a number has a digit and stays.
	fn item_text(&mut self, blocks: &[Block]) -> String {
		let mut parts = self.paragraphs(blocks);
		if parts.len() > 1 && !parts[0].chars().any(char::is_alphanumeric) {
			parts.remove(0);
		}
		parts.join(" ")
	}

	fn write_list(&mut self, items: &[Vec<Block>], level: i32) {
		let start = self.buffer.end();
		for item in items {
			let item_start = self.buffer.end();
			let text = self.item_text(item);
			if !text.is_empty() {
				self.write_line(&text);
				self.buffer.add_marker(Marker::new(MarkerType::ListItem, item_start).with_text(text).with_level(level));
			}
			self.write_nested(item, level);
		}
		let count = i32::try_from(items.len()).unwrap_or(i32::MAX);
		let length = self.buffer.end() - start;
		self.buffer.add_marker(Marker::new(MarkerType::List, start).with_level(count).with_length(length));
	}

	fn write_nested(&mut self, blocks: &[Block], level: i32) {
		for block in blocks {
			match block {
				Block::Paragraph(_) => {}
				Block::List(items) => self.write_list(items, level + 1),
				Block::Table(rows) => self.write_table(rows),
			}
		}
	}

	fn write_table(&mut self, rows: &[Vec<Vec<Block>>]) {
		let grid: Vec<Vec<String>> =
			rows.iter().map(|row| row.iter().map(|cell| self.paragraphs(cell).join(" ")).collect()).collect();
		let html = build_html_table_from_grid(&grid);
		let caption = table_caption_from_html(&html).unwrap_or_else(|| "table".to_string());
		let display = html_table_to_display(&html, self.render_tables_inline);
		let (_, length) = display_lines_and_length(&display);
		let offset = self.buffer.end();
		self.write_line(&display);
		let marker = Marker::new(MarkerType::Table, offset).with_text(caption).with_reference(html).with_length(length);
		self.buffer.add_marker(marker);
	}

	fn write_blocks(&mut self, blocks: &[Block]) {
		for block in blocks {
			match block {
				Block::Paragraph(names) => {
					let text = self.text_of(names);
					self.write_line(&text);
				}
				Block::List(items) => self.write_list(items, 1),
				Block::Table(rows) => self.write_table(rows),
			}
		}
	}

	fn write_unnamed_runs(&mut self) {
		for index in 0..self.read.len() {
			if !self.read[index] {
				self.write_line(&trim_string(&collapse_whitespace(&self.page.runs[index].text)));
			}
		}
	}

	pub fn write(mut self, fragments: &[Fragment]) {
		for kind in [FragmentKind::Header, FragmentKind::Content, FragmentKind::Footer] {
			for fragment in fragments.iter().filter(|fragment| fragment.kind == kind) {
				self.write_blocks(&fragment.blocks);
			}
		}
		self.write_unnamed_runs();
	}
}
