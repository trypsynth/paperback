//! `tbl` tables, the preprocessor language a manual page writes a table in.
//!
//! A table opens with `.TS` and closes with `.TE`. Between them come an options line, one or
//! more lines describing the columns, and then the rows, whose cells are separated by tabs. A
//! cell too long for one line is written between `T{` and `T}` and holds ordinary roff, macros
//! and all.
//!
//! What this reads is the grid: the options that change how a row is read (the separator), the
//! rows, and the cells. The rest of what `tbl` can say is about how a table looks on paper -
//! column widths, alignment, rules, boxes - and is dropped.

use super::escape;

/// The cell separator, and whether a row of rules was seen. Everything else the options line
/// says is about the look of the table.
struct Options {
	separator: char,
}

impl Default for Options {
	fn default() -> Self {
		Self { separator: '\t' }
	}
}

/// Reads the options line, whose only part that changes what the rows mean is `tab(x)`.
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

/// Whether a line describes the columns rather than holding a row of them.
///
/// The format section ends at the line whose last character is a full stop, which is also the
/// last line of it, so a caller reads lines until this returns `true`.
fn ends_format(line: &str) -> bool {
	line.trim_end().ends_with('.')
}

/// A row that draws a rule rather than holding anything.
fn is_rule(line: &str) -> bool {
	matches!(line.trim(), "_" | "=")
}

/// Splits a row into its cells, leaving the separators inside a `T{` block alone: the block
/// holds ordinary roff, and a tab in it belongs to the text.
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

/// The text of one cell, which may run to several lines and hold roff macros of its own.
fn cell_text(cell: &str) -> String {
	let mut parts: Vec<String> = Vec::new();
	for line in cell.lines() {
		if super::is_comment(line) {
			continue;
		}
		let text = match super::control_line(line) {
			// A cell is usually set with the font macros, and `.BR printf ()` in one is still a
			// cross-reference rather than three words.
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

/// One row of the grid, or the end of the table.
pub(super) enum Row {
	Cells(Vec<String>),
	/// A rule, or a line describing the columns: nothing to show.
	Nothing,
	End,
}

/// Reads a `tbl` table into a grid, one row at a time.
///
/// `next_line` hands over the next source line, or `None` at the end of the file, so the reader
/// can drive this over whatever it is reading from.
pub(super) struct Table {
	options: Options,
	past_format: bool,
}

impl Table {
	pub(super) const fn new() -> Self {
		Self { options: Options { separator: '\t' }, past_format: false }
	}

	/// Reads one line of the table.
	///
	/// `continuation` is called when a cell block is left open at the end of a line, and has to
	/// return the next line of the table.
	pub(super) fn read_line(&mut self, line: &str, mut continuation: impl FnMut() -> Option<String>) -> Row {
		if line.trim() == ".TE" {
			return Row::End;
		}
		if super::is_comment(line) {
			return Row::Nothing;
		}
		if !self.past_format {
			// The options line comes first and ends in a semicolon; the column formats follow
			// and end at a line ending in a full stop.
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

/// How many `T{` blocks a row leaves open.
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

	/// A separator inside a cell block belongs to the text of the cell.
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
