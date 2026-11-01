//! Only the grid is read; the rest of what `tbl` says is about how a table looks on paper (widths, alignment, rules, boxes) and is dropped.

use super::escape;

struct Options {
	separator: char,
}

impl Default for Options {
	fn default() -> Self {
		Self { separator: '\t' }
	}
}

/// `tab(x)` is the only option that changes what the rows mean.
fn read_options(line: &str) -> Options {
	let mut options = Options::default();
	if let Some(start) = line.find("tab(") {
		let rest = &line[start + "tab(".len()..];
		if let Some(separator) = rest.chars().next()
			&& separator != ')'
		{
			options.separator = separator;
		}
	}
	options
}

fn ends_format(line: &str) -> bool {
	line.trim_end().ends_with('.')
}

fn is_rule(line: &str) -> bool {
	matches!(line.trim(), "_" | "=")
}

/// A `T{` block holds ordinary roff, so a separator inside it belongs to the text.
fn split_cells(row: &str, separator: char) -> Vec<String> {
	let mut cells = Vec::new();
	let mut current = String::new();
	let mut depth = 0usize;
	let mut chars = row.chars().peekable();
	while let Some(c) = chars.next() {
		if c == 'T' && matches!(chars.peek(), Some('{' | '}')) {
			let brace = chars.next().unwrap_or('{');
			if brace == '{' {
				depth += 1;
			} else {
				depth = depth.saturating_sub(1);
			}
			current.push('\n');
			continue;
		}
		if c == separator && depth == 0 {
			cells.push(std::mem::take(&mut current));
			continue;
		}
		current.push(c);
	}
	cells.push(current);
	cells
}

fn cell_text(cell: &str) -> String {
	let mut parts: Vec<String> = Vec::new();
	for line in cell.lines() {
		if super::is_comment(line) {
			continue;
		}
		let text = match super::control_line(line) {
			// A cell is usually set with the font macros, and `.BR printf ()` in one is still a cross-reference rather than three words.
			Some((name, rest)) => super::font_macro_pieces(name, rest).map(|pieces| escape::plain_pieces(&pieces)),
			None => Some(escape::plain(line)),
		};
		match text {
			Some(text) if !text.trim().is_empty() => parts.push(text.trim().to_string()),
			_ => {}
		}
	}
	parts.join(" ")
}

pub(super) enum Row {
	Cells(Vec<String>),
	Nothing,
	End,
}

pub(super) struct Table {
	options: Options,
	past_format: bool,
}

impl Table {
	pub(super) const fn new() -> Self {
		Self { options: Options { separator: '\t' }, past_format: false }
	}

	/// `continuation` supplies the next line while a `T{` block is left open.
	pub(super) fn read_line(&mut self, line: &str, mut continuation: impl FnMut() -> Option<String>) -> Row {
		if line.trim() == ".TE" {
			return Row::End;
		}
		if super::is_comment(line) {
			return Row::Nothing;
		}
		if !self.past_format {
			// The options line comes first and ends in a semicolon; the column formats follow and end at a line ending in a full stop.
			if line.trim_end().ends_with(';') {
				self.options = read_options(line);
				return Row::Nothing;
			}
			if ends_format(line) {
				self.past_format = true;
			}
			return Row::Nothing;
		}
		// `.T&` starts a new set of column formats partway down a table.
		if line.trim() == ".T&" {
			self.past_format = false;
			return Row::Nothing;
		}
		if is_rule(line) {
			return Row::Nothing;
		}
		let mut row = line.to_string();
		while unclosed_blocks(&row) > 0 {
			let Some(next) = continuation() else { break };
			if next.trim() == ".TE" {
				break;
			}
			row.push('\n');
			row.push_str(&next);
		}
		let cells: Vec<String> = split_cells(&row, self.options.separator).iter().map(|cell| cell_text(cell)).collect();
		if cells.iter().all(|cell| cell.trim().is_empty()) {
			return Row::Nothing;
		}
		Row::Cells(cells)
	}
}

fn unclosed_blocks(row: &str) -> usize {
	let opens = row.matches("T{").count();
	let closes = row.matches("T}").count();
	opens.saturating_sub(closes)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_separator_can_be_changed() {
		assert_eq!(read_options("center box tab(:);").separator, ':');
		assert_eq!(read_options("allbox;").separator, '\t');
	}

	#[test]
	fn cells_split_on_the_separator() {
		assert_eq!(split_cells("a\tb\tc", '\t'), ["a", "b", "c"]);
		assert_eq!(split_cells("a:b", ':'), ["a", "b"]);
	}

	#[test]
	fn a_separator_inside_a_block_does_not_split() {
		let cells = split_cells("T{\none\ttwo\nT}\tthree", '\t');
		assert_eq!(cells.len(), 2);
		assert!(cells[0].contains("one\ttwo"), "{cells:?}");
	}

	#[test]
	fn a_cell_is_rendered_through_its_macros() {
		assert_eq!(cell_text("\n.BR printf ()\n"), "printf()");
		assert_eq!(cell_text("plain text"), "plain text");
		assert_eq!(cell_text("\n.\\\" a comment\nreal text\n"), "real text");
	}
}
