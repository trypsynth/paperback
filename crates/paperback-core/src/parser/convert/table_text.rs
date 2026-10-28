use scraper::{ElementRef, Html, Node};

use crate::{
	document::DocLen,
	parser::convert::formula::{formula_text, is_dom_formula},
	util::text::{collapse_whitespace, trim_string},
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

/// Cells are inserted without HTML-escaping; the `html_table_to_*` functions collapse their whitespace downstream.
#[must_use]
pub fn build_html_table_from_grid(rows: &[Vec<String>]) -> String {
	let mut html = String::from("<table border=\"1\">");
	for row in rows {
		html.push_str("<tr>");
		for cell in row {
			html.push_str("<td>");
			html.push_str(cell);
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

/// Shared by `HtmlToText` and `XmlToText` so their `+1` newline accounting can't diverge; table rows' tabs and empty cells must not be collapsed.
pub fn push_finalized_line(lines: &mut Vec<String>, cached_len: &mut DocLen, line: String) {
	*cached_len += DocLen::of(&line) + DocLen::ONE;
	lines.push(line);
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
