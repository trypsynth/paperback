use scraper::{ElementRef, Html, Node};

use crate::{
	document::DocLen,
	parser::convert::formula::{formula_text, is_dom_formula},
	util::{
		html::push_escaped,
		text::{collapse_whitespace, trim_string},
	},
};

/// Only the outermost table's grid is honoured; a nested `<table>` is flattened into its enclosing cell's text.
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

#[must_use]
pub fn html_table_to_display(html: &str, inline: bool) -> String {
	tsv_to_display(&html_table_to_tsv(html), inline)
}

/// Cells are plain text; the `html_table_to_*` functions collapse their whitespace downstream.
#[must_use]
pub fn build_html_table_from_grid(rows: &[Vec<String>]) -> String {
	let mut html = String::from("<table border=\"1\">");
	for row in rows {
		html.push_str("<tr>");
		for cell in row {
			html.push_str("<td>");
			for ch in cell.chars() {
				push_escaped(ch, &mut html);
			}
			html.push_str("</td>");
		}
		html.push_str("</tr>");
	}
	html.push_str("</table>");
	html
}

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
	pub length: DocLen,
}

#[must_use]
pub fn display_lines_and_length(display_text: &str) -> (Vec<String>, DocLen) {
	if display_text.is_empty() {
		return (Vec::new(), DocLen::ZERO);
	}
	let lines: Vec<String> = display_text.split('\n').map(str::to_string).collect();
	let length = lines.iter().map(|line| DocLen::of(line) + DocLen::ONE).sum();
	(lines, length)
}

#[must_use]
pub fn table_render_bundle(html: &str, inline: bool) -> TableRenderBundle {
	let fragment = Html::parse_fragment(html);
	let (tsv, caption) = find_first_table(fragment.tree.root()).map_or_else(
		|| (String::new(), table_caption_from_tsv("")),
		|table| {
			let mut rows: Vec<String> = Vec::new();
			collect_rows(table, &mut rows);
			let tsv = rows.join("\n");
			let caption = caption_element_text(table).unwrap_or_else(|| table_caption_from_tsv(&tsv));
			(tsv, caption)
		},
	);
	let (lines, length) = display_lines_and_length(&tsv_to_display(&tsv, inline));
	TableRenderBundle { caption, lines, length }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell {
	pub text: String,
	pub is_header: bool,
	pub row: usize,
	pub column: usize,
	pub row_span: usize,
	pub column_span: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableGrid {
	/// Only a `<caption>`; unlike the table's display caption, never its first row.
	pub caption: String,
	/// Each cell once, at the row and column it starts in, in reading order.
	pub cells: Vec<TableCell>,
	pub row_count: usize,
	pub column_count: usize,
	/// The leading rows that name the columns below them.
	pub header_row_count: usize,
}

impl TableGrid {
	#[must_use]
	pub fn row_text(&self, row: usize) -> String {
		let cells: Vec<&str> = self
			.cells
			.iter()
			.filter(|cell| cell.row == row && !cell.text.is_empty())
			.map(|cell| cell.text.as_str())
			.collect();
		cells.join(" ")
	}
}

/// The outermost table's cells placed on its grid, spans and all, or `None` when it has no rows.
#[must_use]
pub fn html_table_grid(html: &str) -> Option<TableGrid> {
	let fragment = Html::parse_fragment(html);
	let table = find_first_table(fragment.tree.root())?;
	let mut rows = Vec::new();
	collect_row_nodes(table, false, &mut rows);
	if rows.is_empty() {
		return None;
	}
	let mut occupied: Vec<Vec<bool>> = vec![Vec::new(); rows.len()];
	let mut cells = Vec::new();
	let mut header_rows = Vec::with_capacity(rows.len());
	for (row, &(node, in_head)) in rows.iter().enumerate() {
		let mut cell_nodes = Vec::new();
		collect_cell_nodes(node, &mut cell_nodes);
		header_rows.push(in_head || (!cell_nodes.is_empty() && cell_nodes.iter().all(|cell| is_element(*cell, "th"))));
		let mut column = 0;
		for cell in cell_nodes {
			while occupied[row].get(column).copied().unwrap_or(false) {
				column += 1;
			}
			let row_span = span(cell, "rowspan").min(rows.len() - row);
			let column_span = span(cell, "colspan");
			for taken in &mut occupied[row..row + row_span] {
				if taken.len() < column + column_span {
					taken.resize(column + column_span, false);
				}
				taken[column..column + column_span].fill(true);
			}
			cells.push(TableCell {
				text: cell_text(cell),
				is_header: in_head || is_element(cell, "th"),
				row,
				column,
				row_span,
				column_span,
			});
			column += column_span;
		}
	}
	let column_count = occupied.iter().map(Vec::len).max().unwrap_or(0).max(1);
	let header_row_count = header_rows.iter().take_while(|&&header| header).count();
	Some(TableGrid {
		caption: caption_element_text(table).unwrap_or_default(),
		cells,
		row_count: rows.len(),
		column_count,
		header_row_count,
	})
}

// A span past this is a broken attribute, not a table that wide.
const MAX_SPAN: usize = 1000;

fn span(cell: ego_tree::NodeRef<'_, Node>, attribute: &str) -> usize {
	let Node::Element(element) = cell.value() else { return 1 };
	element.attr(attribute).and_then(|value| value.trim().parse::<usize>().ok()).unwrap_or(1).clamp(1, MAX_SPAN)
}

fn is_element(node: ego_tree::NodeRef<'_, Node>, name: &str) -> bool {
	matches!(node.value(), Node::Element(element) if element.name() == name)
}

fn collect_row_nodes<'a>(
	node: ego_tree::NodeRef<'a, Node>,
	in_head: bool,
	rows: &mut Vec<(ego_tree::NodeRef<'a, Node>, bool)>,
) {
	for child in node.children() {
		if let Node::Element(element) = child.value() {
			match element.name() {
				"tr" => rows.push((child, in_head)),
				"table" => {}
				"thead" => collect_row_nodes(child, true, rows),
				_ => collect_row_nodes(child, in_head, rows),
			}
		}
	}
}

fn collect_cell_nodes<'a>(node: ego_tree::NodeRef<'a, Node>, cells: &mut Vec<ego_tree::NodeRef<'a, Node>>) {
	for child in node.children() {
		if let Node::Element(element) = child.value() {
			match element.name() {
				"td" | "th" => cells.push(child),
				"table" => {}
				_ => collect_cell_nodes(child, cells),
			}
		}
	}
}

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

fn cell_text(cell: ego_tree::NodeRef<'_, Node>) -> String {
	let mut raw = String::new();
	collect_dom_text(cell, &mut raw, true);
	let collapsed = collapse_whitespace(&raw);
	let trimmed = trim_string(&collapsed);
	trimmed.replace(['\t', '\n'], " ")
}

/// `br_as_space` keeps text either side of a `<br>` from gluing together in table cells.
/// A formula contributes the text the reading buffer shows for it, since its descendant text would concatenate every token and repeat any TeX annotation.
pub(crate) fn collect_dom_text(node: ego_tree::NodeRef<'_, Node>, buffer: &mut String, br_as_space: bool) {
	match node.value() {
		Node::Text(text) => buffer.push_str(&text.text),
		Node::Element(element) if is_dom_formula(element) => {
			let formula = ElementRef::wrap(node).expect("an element node wraps as an element");
			if let Some(text) = formula_text(&formula.html(), element.attr("alttext"), || formula.text().collect()) {
				buffer.push_str(&text);
			}
		}
		Node::Element(element) => {
			if br_as_space && element.name() == "br" {
				buffer.push(' ');
			}
			for child in node.children() {
				collect_dom_text(child, buffer, br_as_space);
			}
		}
		_ => {}
	}
}

#[cfg(test)]
mod tests;
