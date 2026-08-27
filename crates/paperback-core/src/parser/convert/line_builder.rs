//! Shared line/position-tracking core for [`super::xml_to_text::XmlToText`] and
//! [`super::html_to_text::HtmlToText`]: both converters build their output by accumulating text
//! into `current_line`, flushing finished lines into `lines`, and tracking
//! `get_current_text_position()` (the display-unit offset markers/headings/links anchor to) via
//! a running `cached_char_length` rather than recomputing it from `lines` on every call.

use std::mem;

use super::table_text::push_finalized_line;
use crate::util::text::{collapse_whitespace, display_len};

#[derive(Default)]
pub(super) struct LineBuilder {
	pub(super) lines: Vec<String>,
	pub(super) current_line: String,
	cached_char_length: usize,
	preserve_whitespace_depth: usize,
}

impl LineBuilder {
	pub(super) fn clear(&mut self) {
		self.lines.clear();
		self.current_line.clear();
		self.cached_char_length = 0;
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

	/// Adds a finished line, collapsing/trimming whitespace and dropping it if that leaves it
	/// empty - unless whitespace is being preserved (inside `<pre>`/`<code>`), in which case the
	/// line is kept verbatim (just trailing `\n`/`\r` stripped).
	pub(super) fn add_line(&mut self, mut line: String) {
		if self.is_preserving_whitespace() {
			while line.ends_with(['\n', '\r']) {
				line.pop();
			}
			self.cached_char_length += display_len(&line) + 1;
			self.lines.push(line);
		} else {
			let collapsed = collapse_whitespace(&line);
			let collapsed = collapsed.trim().to_string();
			if collapsed.is_empty() {
				return;
			}
			self.cached_char_length += display_len(&collapsed) + 1;
			self.lines.push(collapsed);
		}
	}

	/// Push a line to the output verbatim (no whitespace collapsing/trimming), updating the
	/// cached length so position tracking stays correct. Used for table rows whose tab
	/// separators and empty cells must not be mangled by [`Self::add_line`].
	pub(super) fn push_finalized_line(&mut self, line: String) {
		push_finalized_line(&mut self.lines, &mut self.cached_char_length, line);
	}

	pub(super) fn finalize_current_line(&mut self) {
		let line = mem::take(&mut self.current_line);
		self.add_line(line);
	}

	fn current_display_len(&self) -> usize {
		if self.is_preserving_whitespace() {
			return display_len(&self.current_line);
		}
		let collapsed = collapse_whitespace(&self.current_line);
		// Use trim_start() not trim(): trailing whitespace before an inline element (e.g. a
		// space before a link) IS preserved in the output line, so including it in the position
		// count keeps link/anchor offsets correctly aligned with the final text.
		let trimmed = collapsed.trim_start();
		display_len(trimmed)
	}

	pub(super) fn get_current_text_position(&self) -> usize {
		self.cached_char_length + self.current_display_len()
	}

	pub(super) const fn separator_line() -> &'static str {
		"----------------------------------------"
	}
}
