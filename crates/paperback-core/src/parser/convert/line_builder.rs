//! A running `cached_char_length` saves recounting the text every time a marker asks for the current position.

use std::{fmt, mem};

use crate::{
	document::{DocLen, DocPos},
	util::text::{collapse_whitespace, is_space_like},
};

#[derive(Default)]
pub(super) struct LineBuilder {
	/// Each finished line followed by its newline, so the text is built once instead of as lines joined into a second copy.
	text: String,
	current_line: String,
	current_len: CurrentLength,
	cached_char_length: DocLen,
	preserve_whitespace_depth: usize,
}

/// Counted as the line grows, since collapsing the whole line again for every marker made a paragraph full of links quadratic.
#[derive(Default)]
struct CurrentLength {
	raw: DocLen,
	/// What `collapse_whitespace(line).trim_start()` would count: each character that isn't a space, and one for each run of spaces after the first of them.
	collapsed: DocLen,
	seen_text: bool,
	in_spaces: bool,
}

impl CurrentLength {
	fn push(&mut self, text: &str) {
		for ch in text.chars() {
			self.raw += DocLen::ONE;
			if !is_space_like(ch) {
				self.collapsed += DocLen::ONE;
				self.seen_text = true;
				self.in_spaces = false;
			} else if self.seen_text && !self.in_spaces {
				self.collapsed += DocLen::ONE;
				self.in_spaces = true;
			}
		}
	}
}

impl fmt::Write for LineBuilder {
	fn write_str(&mut self, text: &str) -> fmt::Result {
		self.push_str(text);
		Ok(())
	}
}

impl LineBuilder {
	pub(super) fn clear(&mut self) {
		self.text.clear();
		self.clear_current_line();
		self.cached_char_length = DocLen::ZERO;
		self.preserve_whitespace_depth = 0;
	}

	pub(super) fn get_text(&self) -> &str {
		self.text.strip_suffix('\n').unwrap_or(&self.text)
	}

	pub(super) fn take_text(&mut self) -> String {
		let mut text = mem::take(&mut self.text);
		text.pop();
		text
	}

	pub(super) fn current_line(&self) -> &str {
		&self.current_line
	}

	pub(super) fn push_str(&mut self, text: &str) {
		self.current_len.push(text);
		self.current_line.push_str(text);
	}

	pub(super) fn push(&mut self, ch: char) {
		self.push_str(ch.encode_utf8(&mut [0; 4]));
	}

	pub(super) fn clear_current_line(&mut self) {
		self.current_line.clear();
		self.current_len = CurrentLength::default();
	}

	pub(super) const fn is_preserving_whitespace(&self) -> bool {
		self.preserve_whitespace_depth > 0
	}

	pub(super) fn start_preserve_whitespace(&mut self) {
		self.preserve_whitespace_depth += 1;
	}

	pub(super) fn stop_preserve_whitespace(&mut self) {
		if self.preserve_whitespace_depth > 0 {
			self.preserve_whitespace_depth -= 1;
		}
	}

	fn add_line(&mut self, line: &str) {
		if self.is_preserving_whitespace() {
			let line = line.trim_end_matches(['\n', '\r']);
			self.push_finalized_line(line);
		} else {
			let collapsed = collapse_whitespace(line);
			let collapsed = collapsed.trim();
			if !collapsed.is_empty() {
				self.push_finalized_line(collapsed);
			}
		}
	}

	/// Taken as it is, since a table row's tabs and empty cells must not be collapsed.
	pub(super) fn push_finalized_line(&mut self, line: &str) {
		self.cached_char_length += DocLen::of(line) + DocLen::ONE;
		self.text.push_str(line);
		self.text.push('\n');
	}

	pub(super) fn finalize_current_line(&mut self) {
		let line = mem::take(&mut self.current_line);
		self.current_len = CurrentLength::default();
		self.add_line(&line);
		// Handed back so the next line reuses its allocation.
		self.current_line = line;
		self.current_line.clear();
	}

	pub(super) fn get_current_text_position(&self) -> DocPos {
		// Not trimmed at the end: trailing whitespace before an inline element such as a link survives into the output line, so counting it keeps anchor offsets aligned.
		let current = if self.is_preserving_whitespace() { self.current_len.raw } else { self.current_len.collapsed };
		(self.cached_char_length + current).as_pos()
	}

	pub(super) const fn separator_line() -> &'static str {
		"----------------------------------------"
	}
}
