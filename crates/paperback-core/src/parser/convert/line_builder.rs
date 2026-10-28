//! A running `cached_char_length` saves recounting `lines` every time a marker asks for the current position.

use std::mem;

use super::table_text::push_finalized_line;
use crate::{
	document::{DocLen, DocPos},
	util::text::collapse_whitespace,
};

#[derive(Default)]
pub(super) struct LineBuilder {
	pub(super) lines: Vec<String>,
	pub(super) current_line: String,
	cached_char_length: DocLen,
	preserve_whitespace_depth: usize,
}

impl LineBuilder {
	pub(super) fn clear(&mut self) {
		self.lines.clear();
		self.current_line.clear();
		self.cached_char_length = DocLen::ZERO;
		self.preserve_whitespace_depth = 0;
	}

	pub(super) fn get_text(&self) -> String {
		self.lines.join("\n")
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

	pub(super) fn add_line(&mut self, mut line: String) {
		if self.is_preserving_whitespace() {
			while line.ends_with(['\n', '\r']) {
				line.pop();
			}
			self.cached_char_length += DocLen::of(&line) + DocLen::ONE;
			self.lines.push(line);
		} else {
			let collapsed = collapse_whitespace(&line);
			let collapsed = collapsed.trim().to_string();
			if collapsed.is_empty() {
				return;
			}
			self.cached_char_length += DocLen::of(&collapsed) + DocLen::ONE;
			self.lines.push(collapsed);
		}
	}

	pub(super) fn push_finalized_line(&mut self, line: String) {
		push_finalized_line(&mut self.lines, &mut self.cached_char_length, line);
	}

	pub(super) fn finalize_current_line(&mut self) {
		let line = mem::take(&mut self.current_line);
		self.add_line(line);
	}

	fn current_len(&self) -> DocLen {
		if self.is_preserving_whitespace() {
			return DocLen::of(&self.current_line);
		}
		let collapsed = collapse_whitespace(&self.current_line);
		// Not trim(): trailing whitespace before an inline element such as a link survives into the output line, so counting it keeps anchor offsets aligned.
		let trimmed = collapsed.trim_start();
		DocLen::of(trimmed)
	}

	pub(super) fn get_current_text_position(&self) -> DocPos {
		(self.cached_char_length + self.current_len()).as_pos()
	}

	pub(super) const fn separator_line() -> &'static str {
		"----------------------------------------"
	}
}
