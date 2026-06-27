use scraper::{Html, Node};

use crate::util::text::{collapse_whitespace, display_len, trim_string};

/// `<table>…</table>` HTML -> tab-separated text: rows by '\n', cells (`<td>`/`<th>`) by '\t'.
/// Cell text whitespace-collapsed + trimmed; internal '\t'/'\n' -> space; nested tables flattened.
///
/// Only the outermost table's grid structure is honored: rows and cells that belong to a nested
/// `<table>` do not start new rows/cells, they are flattened into the enclosing cell's text.
#[must_use]
pub fn html_table_to_tsv(html: &str) -> String {
	let fragment = Html::parse_fragment(html);
	let Some(table) = find_first_table(fragment.tree.root()) else {
		return String::new();
	};
	let mut rows: Vec<String> = Vec::new();
	collect_rows(table, &mut rows);
	rows.join("\n")
}

/// Produce the on-screen text for a table in the requested display mode (see `tsv_to_display`).
#[must_use]
pub fn html_table_to_display(html: &str, inline: bool) -> String {
	tsv_to_display(&html_table_to_tsv(html), inline)
}

/// Render a tab-separated table body in the requested display mode.
///
/// - `inline == true`  -> the TSV unchanged.
/// - `inline == false` -> a uniform placeholder `"[Table]: <first row>"`, where `<first row>` is
///   the first row with tab separators replaced by single spaces. Empty TSV -> just `"[Table]"`.
fn tsv_to_display(tsv: &str, inline: bool) -> String {
	if inline {
		return tsv.to_string();
	}
	let first_line = tsv.split('\n').next().unwrap_or("");
	if first_line.is_empty() { "[Table]".to_string() } else { format!("[Table]: {}", first_line.replace('\t', " ")) }
}

#[must_use]
pub fn table_caption_from_html(html: &str) -> Option<String> {
	let fragment = Html::parse_fragment(html);
	let table = find_first_table(fragment.tree.root())?;
	if let Some(caption) = caption_element_text(table) {
		return Some(caption);
	}
	// Reuse the already-parsed tree instead of calling html_table_to_tsv(html) again.
	let mut rows: Vec<String> = Vec::new();
	collect_rows(table, &mut rows);
	let first_row = first_tsv_row_text(&rows.join("\n"));
	if first_row.is_empty() { None } else { Some(first_row) }
}

/// Text of the table's explicit `<caption>` element, if present and non-empty.
fn caption_element_text(table: ego_tree::NodeRef<'_, Node>) -> Option<String> {
	for child in table.children() {
		if let Node::Element(element) = child.value()
			&& element.name() == "caption"
		{
			let caption = cell_text(child);
			if !caption.is_empty() {
				return Some(caption);
			}
		}
	}
	None
}

#[must_use]
pub fn table_caption_from_tsv(tsv: &str) -> String {
	let caption = first_tsv_row_text(tsv);
	if caption.is_empty() { "table".to_string() } else { caption }
}

fn first_tsv_row_text(tsv: &str) -> String {
	let first_line = tsv.split('\n').next().unwrap_or("").replace('\t', " ");
	trim_string(&collapse_whitespace(&first_line))
}

pub struct TableRenderBundle {
	pub caption: String,
	pub lines: Vec<String>,
	pub display_length: usize,
}

/// Split `display_text` into lines and compute the total display-unit length (each line's
/// display width + 1 for its trailing newline).  Returns `(lines, display_length)`.
#[must_use]
pub fn display_lines_and_length(display_text: &str) -> (Vec<String>, usize) {
	if display_text.is_empty() {
		return (Vec::new(), 0);
	}
	let lines: Vec<String> = display_text.split('\n').map(str::to_string).collect();
	let display_length = lines.iter().map(|line| display_len(line) + 1).sum();
	(lines, display_length)
}

/// Push a line to a parser's output verbatim (no whitespace collapsing/trimming), updating its
/// cached display length so position tracking stays correct. Shared by `HtmlToText` and `XmlToText`
/// so the `+1` newline accounting can never diverge between the two. Used for table rows whose tab
/// separators and empty cells must not be mangled.
pub fn push_finalized_line(lines: &mut Vec<String>, cached_len: &mut usize, line: String) {
	*cached_len += display_len(&line) + 1; // +1 for the line's newline
	lines.push(line);
}

#[must_use]
pub fn table_render_bundle(html: &str, inline: bool) -> TableRenderBundle {
	// Parse the HTML once; derive TSV, caption, and display text from the same tree.
	let fragment = Html::parse_fragment(html);
	let (tsv, caption) = match find_first_table(fragment.tree.root()) {
		Some(table) => {
			let mut rows: Vec<String> = Vec::new();
			collect_rows(table, &mut rows);
			let tsv = rows.join("\n");
			// Prefer an explicit <caption> element; fall back to the first TSV row.
			let caption = caption_element_text(table).unwrap_or_else(|| table_caption_from_tsv(&tsv));
			(tsv, caption)
		}
		None => (String::new(), table_caption_from_tsv("")),
	};
	let (lines, display_length) = display_lines_and_length(&tsv_to_display(&tsv, inline));
	TableRenderBundle { caption, lines, display_length }
}

/// Find the first (outermost) `<table>` element in document order, descending through wrappers.
fn find_first_table(node: ego_tree::NodeRef<'_, Node>) -> Option<ego_tree::NodeRef<'_, Node>> {
	for child in node.children() {
		if let Node::Element(element) = child.value() {
			if element.name() == "table" {
				return Some(child);
			}
			if let Some(found) = find_first_table(child) {
				return Some(found);
			}
		}
	}
	None
}

/// Gather the rows of `table`, descending through grouping wrappers (`thead`/`tbody`/`tfoot`)
/// to reach `<tr>` elements, but never descending into a nested table.
fn collect_rows(node: ego_tree::NodeRef<'_, Node>, rows: &mut Vec<String>) {
	for child in node.children() {
		if let Node::Element(element) = child.value() {
			match element.name() {
				"tr" => rows.push(collect_row(child)),
				// A nested table's rows belong to a cell, not this grid: skip them here.
				"table" => {}
				_ => collect_rows(child, rows),
			}
		}
	}
}

/// Collect the cells of a single row, joined by tabs. Recurses through wrapper elements to find
/// `<td>`/`<th>` but stops at nested tables (their cells are flattened into the parent cell text).
fn collect_row(row: ego_tree::NodeRef<'_, Node>) -> String {
	let mut cells: Vec<String> = Vec::new();
	collect_cells(row, &mut cells);
	cells.join("\t")
}

fn collect_cells(node: ego_tree::NodeRef<'_, Node>, cells: &mut Vec<String>) {
	for child in node.children() {
		if let Node::Element(element) = child.value() {
			match element.name() {
				"td" | "th" => cells.push(cell_text(child)),
				// A nested table inside a row (but outside a cell) is not part of this grid.
				"table" => {}
				_ => collect_cells(child, cells),
			}
		}
	}
}

/// Collect the flattened text of a single cell: descendant text nodes concatenated, with
/// `<br>` rendered as a space, then whitespace-collapsed, trimmed, and any residual `\t`/`\n`
/// replaced by a single space. Nested tables contribute only their text (no grid structure).
fn cell_text(cell: ego_tree::NodeRef<'_, Node>) -> String {
	let mut raw = String::new();
	collect_cell_text(cell, &mut raw);
	let collapsed = collapse_whitespace(&raw);
	let trimmed = trim_string(&collapsed);
	trimmed.replace(['\t', '\n'], " ")
}

fn collect_cell_text(node: ego_tree::NodeRef<'_, Node>, buffer: &mut String) {
	match node.value() {
		Node::Text(text) => buffer.push_str(&text.text),
		Node::Element(element) => {
			if element.name() == "br" {
				buffer.push(' ');
			}
			for child in node.children() {
				collect_cell_text(child, buffer);
			}
		}
		_ => {}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn two_by_two_table_is_tab_and_newline_separated() {
		let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "a\tb\nc\td");
	}

	#[test]
	fn header_row_cells_are_included() {
		let html = "<table><tr><th>H1</th><th>H2</th></tr><tr><td>v1</td><td>v2</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "H1\tH2\nv1\tv2");
	}

	#[test]
	fn empty_cells_produce_empty_strings_between_tabs() {
		let html = "<table><tr><td></td><td>b</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "\tb");
	}

	#[test]
	fn nested_table_is_flattened_to_text() {
		let html = "<table><tr><td>outer<table><tr><td>inner</td></tr></table></td><td>x</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "outerinner\tx");
	}

	#[test]
	fn embedded_tab_and_newline_collapse_to_single_space() {
		let html = "<table><tr><td>a\t\nb</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "a b");
	}

	#[test]
	fn entities_decoded_and_br_becomes_space() {
		let html = "<table><tr><td>1 &lt; 2 &amp; 3<br/>line2</td></tr></table>";
		assert_eq!(html_table_to_tsv(html), "1 < 2 & 3 line2");
	}

	#[test]
	fn empty_table_yields_empty_string() {
		assert_eq!(html_table_to_tsv("<table></table>"), "");
	}

	#[test]
	fn html_table_to_display_inline_2x2_table() {
		let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
		assert_eq!(html_table_to_display(html, true), "a\tb\nc\td");
	}

	#[test]
	fn html_table_to_display_placeholder_2x2_table() {
		let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
		assert_eq!(html_table_to_display(html, false), "[Table]: a b");
	}

	#[test]
	fn html_table_to_display_placeholder_single_cell() {
		let html = "<table><tr><td>x</td></tr></table>";
		assert_eq!(html_table_to_display(html, false), "[Table]: x");
	}

	#[test]
	fn html_table_to_display_placeholder_empty_table() {
		let html = "<table></table>";
		assert_eq!(html_table_to_display(html, false), "[Table]");
	}

	#[test]
	fn html_table_to_display_inline_empty_table() {
		let html = "<table></table>";
		assert_eq!(html_table_to_display(html, true), "");
	}

	#[test]
	fn html_table_to_display_placeholder_header_body_table() {
		let html = "<table><tr><th>H1</th><th>H2</th></tr><tr><td>v1</td><td>v2</td></tr></table>";
		assert_eq!(html_table_to_display(html, false), "[Table]: H1 H2");
	}

	#[test]
	fn table_caption_prefers_caption_element() {
		let html = "<table><caption>Cap</caption><tr><td>row</td></tr></table>";
		assert_eq!(table_caption_from_html(html), Some("Cap".to_string()));
	}

	#[test]
	fn table_caption_falls_back_to_first_row() {
		let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
		assert_eq!(table_caption_from_html(html), Some("a b".to_string()));
	}

	#[test]
	fn table_caption_returns_none_for_empty_table() {
		assert_eq!(table_caption_from_html("<table></table>"), None);
	}

	#[test]
	fn table_lines_preserve_empty_rows_and_report_display_length() {
		let (lines, display_length) = display_lines_and_length("a\tb\n\tc");
		assert_eq!(lines, vec!["a\tb".to_string(), "\tc".to_string()]);
		assert_eq!(display_length, 7);
	}

	#[test]
	fn table_lines_handle_empty_display_text() {
		let (lines, display_length) = display_lines_and_length("");
		assert!(lines.is_empty());
		assert_eq!(display_length, 0);
	}

	#[test]
	fn table_render_bundle_includes_caption_lines_and_length() {
		let bundle = table_render_bundle("<table><caption>Cap</caption><tr><td>a</td><td>b</td></tr></table>", true);
		assert_eq!(bundle.caption, "Cap");
		assert_eq!(bundle.lines, vec!["a\tb".to_string()]);
		assert_eq!(bundle.display_length, 4);
	}

	#[test]
	fn table_render_bundle_falls_back_to_default_caption() {
		let bundle = table_render_bundle("<table></table>", true);
		assert_eq!(bundle.caption, "table");
		assert!(bundle.lines.is_empty());
		assert_eq!(bundle.display_length, 0);
	}
}
