//! Filled text and `.nf` text are kept apart, since a synopsis read as one long line is unreadable and prose read one source sentence to a line is not much better.

use super::escape::{Font, Piece};
use crate::{
	document::{DocPos, DocumentBuffer, Marker, MarkerType},
	parser::convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FontState {
	current: Font,
	previous: Font,
}

impl FontState {
	const fn new() -> Self {
		Self { current: Font::Roman, previous: Font::Roman }
	}

	fn set(&mut self, font: Font) {
		let next = if font == Font::Previous { self.previous } else { font };
		if next != self.current {
			self.previous = self.current;
			self.current = next;
		}
	}
}

pub(super) struct Renderer {
	pub(super) buffer: DocumentBuffer,
	font: FontState,
	run_start: Option<DocPos>,
	pending_link: Option<String>,
	link_start: Option<DocPos>,
	line_has_text: bool,
	paragraph_has_text: bool,
	filling: bool,
	list_start: Option<DocPos>,
	list_items: usize,
}

impl Renderer {
	pub(super) fn new() -> Self {
		Self {
			buffer: DocumentBuffer::new(),
			font: FontState::new(),
			run_start: None,
			pending_link: None,
			link_start: None,
			line_has_text: false,
			paragraph_has_text: false,
			filling: true,
			list_start: None,
			list_items: 0,
		}
	}

	pub(super) const fn set_filling(&mut self, filling: bool) {
		self.filling = filling;
	}

	/// The font carries across lines, as it does in roff.
	pub(super) fn write_line(&mut self, pieces: &[Piece]) {
		if !pieces.iter().any(|piece| matches!(piece, Piece::Text(text) if !text.trim().is_empty())) {
			// A line with no text of its own still ends a paragraph while filling: roff reads a blank line as a break.
			if self.filling {
				self.end_paragraph();
			} else {
				self.break_line();
			}
			return;
		}
		if self.line_has_text {
			if self.filling {
				self.write_text(" ");
			} else {
				self.break_line();
			}
		}
		for piece in pieces {
			match piece {
				Piece::Text(text) => self.write_text(text),
				Piece::Font(font) => self.set_font(*font),
			}
		}
	}

	fn write_text(&mut self, text: &str) {
		if text.is_empty() {
			return;
		}
		// A link starts at its first word rather than where the macro stood, since the space joining it to the sentence before it is written between the two.
		if !text.trim().is_empty() && self.pending_link.is_some() {
			self.link_start = Some(self.buffer.end());
		}
		if self.run_start.is_none() && matches!(self.font.current, Font::Bold | Font::Italic) {
			self.run_start = Some(self.buffer.end());
		}
		self.buffer.append(text);
		if !text.trim().is_empty() {
			self.line_has_text = true;
			self.paragraph_has_text = true;
		}
	}

	fn set_font(&mut self, font: Font) {
		let before = self.font.current;
		self.font.set(font);
		if self.font.current == before {
			return;
		}
		self.close_run(before);
		if matches!(self.font.current, Font::Bold | Font::Italic) {
			self.run_start = Some(self.buffer.end());
		}
	}

	fn close_run(&mut self, font: Font) {
		let Some(start) = self.run_start.take() else { return };
		let length = self.buffer.end().since(start);
		if length.is_zero() {
			return;
		}
		let mtype = match font {
			Font::Bold => MarkerType::Bold,
			Font::Italic => MarkerType::Italic,
			_ => return,
		};
		self.buffer.add_marker(Marker::new(mtype, start).with_length(length));
	}

	pub(super) fn break_line(&mut self) {
		if !self.line_has_text {
			return;
		}
		self.close_run(self.font.current);
		self.buffer.append("\n");
		self.line_has_text = false;
		if matches!(self.font.current, Font::Bold | Font::Italic) {
			self.run_start = Some(self.buffer.end());
		}
	}

	pub(super) fn end_paragraph(&mut self) {
		if !self.paragraph_has_text {
			return;
		}
		self.break_line();
		self.buffer.append("\n");
		self.paragraph_has_text = false;
	}

	pub(super) fn heading(&mut self, text: &str, level: i32) -> Option<DocPos> {
		self.end_list();
		self.end_paragraph();
		if text.trim().is_empty() {
			return None;
		}
		let position = self.buffer.end();
		let mtype = if level <= 1 { MarkerType::Heading1 } else { MarkerType::Heading2 };
		self.buffer.add_marker(Marker::new(mtype, position).with_level(level).with_text(text.to_string()));
		self.buffer.append(text);
		self.buffer.append("\n\n");
		self.line_has_text = false;
		self.paragraph_has_text = false;
		Some(position)
	}

	/// The tag is a list item, so a page's options can be walked one by one.
	pub(super) fn start_item(&mut self, tag: &[Piece]) {
		self.end_paragraph();
		if self.list_start.is_none() {
			self.list_start = Some(self.buffer.end());
			self.list_items = 0;
		}
		self.list_items += 1;
		let position = self.buffer.end();
		let text = super::escape::plain_pieces(tag);
		self.buffer.add_marker(Marker::new(MarkerType::ListItem, position).with_text(text));
		self.write_line(tag);
		self.break_line();
	}

	/// Goes through the shared HTML table renderer so the text, caption and inline-or-placeholder choice are decided in one place for every format.
	pub(super) fn table(&mut self, rows: &[Vec<String>], inline: bool) {
		self.end_list();
		self.end_paragraph();
		if rows.is_empty() {
			return;
		}
		let html = build_html_table_from_grid(rows);
		let caption = table_caption_from_html(&html).unwrap_or_else(|| "table".to_string());
		let start = self.buffer.end();
		self.buffer.append(&html_table_to_display(&html, inline));
		self.buffer.append(
			"

",
		);
		let length = self.buffer.end().since(start);
		self.buffer.add_marker(
			Marker::new(MarkerType::Table, start).with_text(caption).with_reference(html).with_length(length),
		);
		self.line_has_text = false;
		self.paragraph_has_text = false;
	}

	pub(super) fn end_list(&mut self) {
		let Some(start) = self.list_start.take() else { return };
		// A lone tagged paragraph is not a list worth navigating into.
		if self.list_items < 2 {
			return;
		}
		let length = self.buffer.end().since(start);
		if !length.is_zero() {
			self.buffer.add_marker(Marker::new(MarkerType::List, start).with_length(length));
		}
	}

	pub(super) fn begin_link(&mut self, url: String) {
		self.pending_link = Some(url);
		self.link_start = None;
	}

	pub(super) fn end_link(&mut self) {
		let (Some(url), Some(start)) = (self.pending_link.take(), self.link_start.take()) else {
			return;
		};
		let length = self.buffer.end().since(start);
		if length.is_zero() {
			return;
		}
		let text = self.buffer.text(start..start + length).to_string();
		self.buffer
			.add_marker(Marker::new(MarkerType::Link, start).with_length(length).with_reference(url).with_text(text));
	}

	pub(super) fn finish(mut self) -> DocumentBuffer {
		self.end_list();
		self.end_paragraph();
		self.buffer
	}
}
