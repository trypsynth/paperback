//! Laying roff out as text: paragraphs, headings, list items and font runs.
//!
//! Roff fills text by default, which means the line breaks in the source are not line breaks in
//! the output: a paragraph is written one sentence to a line and set as one run. `.nf` turns
//! filling off, and there the breaks are the content. The renderer keeps that distinction, since
//! a synopsis or an example read as one long line is unreadable, and prose read one sentence to
//! a line is not much better.

use super::escape::{Font, Piece};
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	parser::convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
};

/// How the current run of text is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FontState {
	current: Font,
	previous: Font,
}

impl FontState {
	const fn new() -> Self {
		Self { current: Font::Roman, previous: Font::Roman }
	}

	/// Applies a font change, resolving `\fP` against the font before the current one.
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
	/// Where the current bold or italic run started, if one is open.
	run_start: Option<usize>,
	/// The target of a link waiting for its first word, and then where that word began.
	pending_link: Option<String>,
	link_start: Option<usize>,
	/// Whether anything has been written to the line being built.
	line_has_text: bool,
	/// Whether the paragraph being built has any lines in it yet.
	paragraph_has_text: bool,
	/// Whether text lines join into a paragraph (`.fi`) or stand as written (`.nf`).
	filling: bool,
	/// Where the run of list items being built started, and how many items it holds.
	list_start: Option<usize>,
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

	/// Lays down one source line's worth of pieces.
	///
	/// While filling, a line joins the one before it with a space; otherwise it starts a line of
	/// its own. Either way the font carries across lines, which is what roff does.
	pub(super) fn write_line(&mut self, pieces: &[Piece]) {
		if !pieces.iter().any(|piece| matches!(piece, Piece::Text(text) if !text.trim().is_empty())) {
			// A line with no text of its own still ends a paragraph while filling: roff reads a
			// blank line as a break.
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

	/// Writes text in whatever font is current, opening and closing marker runs as it goes.
	fn write_text(&mut self, text: &str) {
		if text.is_empty() {
			return;
		}
		// A link starts at its first word rather than where the macro stood, since the space
		// joining it to the sentence before it is written between the two.
		if !text.trim().is_empty() && self.pending_link.is_some() {
			self.link_start = Some(self.buffer.current_position());
		}
		if self.run_start.is_none() && matches!(self.font.current, Font::Bold | Font::Italic) {
			self.run_start = Some(self.buffer.current_position());
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
			self.run_start = Some(self.buffer.current_position());
		}
	}

	/// Marks the span a bold or italic run covered, if it covered anything.
	fn close_run(&mut self, font: Font) {
		let Some(start) = self.run_start.take() else { return };
		let length = self.buffer.current_position().saturating_sub(start);
		if length == 0 {
			return;
		}
		let mtype = match font {
			Font::Bold => MarkerType::Bold,
			Font::Italic => MarkerType::Italic,
			_ => return,
		};
		self.buffer.add_marker(Marker::new(mtype, start).with_length(length));
	}

	/// Ends the line being built without ending the paragraph, as `.br` does.
	pub(super) fn break_line(&mut self) {
		if !self.line_has_text {
			return;
		}
		self.close_run(self.font.current);
		self.buffer.append("\n");
		self.line_has_text = false;
		if matches!(self.font.current, Font::Bold | Font::Italic) {
			self.run_start = Some(self.buffer.current_position());
		}
	}

	/// Ends the paragraph being built, leaving a blank line after it.
	pub(super) fn end_paragraph(&mut self) {
		if !self.paragraph_has_text {
			return;
		}
		self.break_line();
		self.buffer.append("\n");
		self.paragraph_has_text = false;
	}

	/// Starts a section, as `.SH` and `.SS` do. The heading is its own line. Returns where it
	/// landed, for the table of contents.
	pub(super) fn heading(&mut self, text: &str, level: i32) -> Option<usize> {
		self.end_list();
		self.end_paragraph();
		if text.trim().is_empty() {
			return None;
		}
		let position = self.buffer.current_position();
		let mtype = if level <= 1 { MarkerType::Heading1 } else { MarkerType::Heading2 };
		self.buffer.add_marker(Marker::new(mtype, position).with_level(level).with_text(text.to_string()));
		self.buffer.append(text);
		self.buffer.append("\n\n");
		self.line_has_text = false;
		self.paragraph_has_text = false;
		Some(position)
	}

	/// Starts a tagged paragraph, as `.TP` and a tagged `.IP` do: the tag on its own line, then
	/// the body under it. The tag is a list item, so a page's options can be walked one by one.
	pub(super) fn start_item(&mut self, tag: &[Piece]) {
		self.end_paragraph();
		if self.list_start.is_none() {
			self.list_start = Some(self.buffer.current_position());
			self.list_items = 0;
		}
		self.list_items += 1;
		let position = self.buffer.current_position();
		let text = super::escape::plain_pieces(tag);
		self.buffer.add_marker(Marker::new(MarkerType::ListItem, position).with_text(text));
		self.write_line(tag);
		self.break_line();
	}

	/// Lays down a table, as every other format does: the grid becomes the same HTML the
	/// shared renderer takes, so the text of it, its caption, and whether it is written out in
	/// full or stands as a placeholder are all decided in one place.
	pub(super) fn table(&mut self, rows: &[Vec<String>], inline: bool) {
		self.end_list();
		self.end_paragraph();
		if rows.is_empty() {
			return;
		}
		let escaped: Vec<Vec<String>> =
			rows.iter().map(|row| row.iter().map(|cell| escape_html(cell)).collect()).collect();
		let html = build_html_table_from_grid(&escaped);
		let caption = table_caption_from_html(&html).unwrap_or_else(|| "table".to_string());
		let start = self.buffer.current_position();
		self.buffer.append(&html_table_to_display(&html, inline));
		self.buffer.append(
			"

",
		);
		let length = self.buffer.current_position().saturating_sub(start);
		self.buffer.add_marker(
			Marker::new(MarkerType::Table, start).with_text(caption).with_reference(html).with_length(length),
		);
		self.line_has_text = false;
		self.paragraph_has_text = false;
	}

	/// Closes the run of list items being built, marking the whole run as one list so the reader
	/// can step over it.
	pub(super) fn end_list(&mut self) {
		let Some(start) = self.list_start.take() else { return };
		// A lone tagged paragraph is not a list worth navigating into.
		if self.list_items < 2 {
			return;
		}
		let length = self.buffer.current_position().saturating_sub(start);
		if length > 0 {
			self.buffer.add_marker(Marker::new(MarkerType::List, start).with_length(length));
		}
	}

	/// Opens a link, as `.UR` does. The text of it is whatever is written before it closes.
	pub(super) fn begin_link(&mut self, url: String) {
		self.pending_link = Some(url);
		self.link_start = None;
	}

	/// Closes a link, as `.UE` does, marking the text written since it opened.
	pub(super) fn end_link(&mut self) {
		let (Some(url), Some(start)) = (self.pending_link.take(), self.link_start.take()) else {
			return;
		};
		let length = self.buffer.current_position().saturating_sub(start);
		if length == 0 {
			return;
		}
		let text: String = self.buffer.content.chars().skip(start).take(length).collect();
		self.buffer
			.add_marker(Marker::new(MarkerType::Link, start).with_length(length).with_reference(url).with_text(text));
	}

	/// Everything written, with the last paragraph closed off.
	pub(super) fn finish(mut self) -> DocumentBuffer {
		self.end_list();
		self.end_paragraph();
		self.buffer
	}
}

/// Escapes a cell for the HTML the shared table renderer reads back. A manual page is full
/// of angle brackets and ampersands, which would otherwise be read as markup.
fn escape_html(cell: &str) -> String {
	let mut out = String::with_capacity(cell.len());
	for c in cell.chars() {
		match c {
			'&' => out.push_str("&amp;"),
			'<' => out.push_str("&lt;"),
			'>' => out.push_str("&gt;"),
			_ => out.push(c),
		}
	}
	out
}
