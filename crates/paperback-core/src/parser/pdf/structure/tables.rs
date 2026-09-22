//! A tagged PDF's tables: turning the `Table`/`TR`/`TH`/`TD` elements into HTML, and writing that
//! table into the page.
//!
//! The HTML is the intermediate form the rest of Paperback already speaks. Every other format's
//! tables reach the reader through
//! [`crate::parser::convert::table_text`], so a PDF's do too rather than growing a renderer of
//! their own, and the table view the reader opens with Enter is given the same HTML.

use std::{collections::HashMap, fmt::Write as _};

use super::marked_content::collect_text;
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	parser::convert::table_text::{display_lines_and_length, html_table_to_display},
	pdfium::Tag,
	util::text::{collapse_whitespace, trim_string},
};

/// Walks a `Table` element into HTML. Anything between the table and its rows that is not a row
/// itself (a `THead`, a `Div` some producer put in) is walked through rather than written, so a
/// table keeps its shape whatever it is wrapped in.
pub(super) fn build_html_table(elem: &Tag, mcid_to_text: &HashMap<i32, String>) -> String {
	let elem_type = elem.kind().unwrap_or_default();
	match elem_type.as_str() {
		"Table" => wrap("table border=\"1\"", "table", elem, mcid_to_text),
		"TR" => wrap("tr", "tr", elem, mcid_to_text),
		"TH" | "TD" => {
			let tag = elem_type.to_lowercase();
			let mut cell_text = String::new();
			collect_text(elem, mcid_to_text, &mut cell_text);
			let mut html = format!("<{tag}>");
			html.push_str(&html_escape(&trim_string(&collapse_whitespace(&cell_text))));
			let _ = writeln!(html, "</{tag}>");
			html
		}
		_ => children(elem, mcid_to_text),
	}
}

/// One element's HTML: an opening tag, whatever its children come to, and a closing tag.
fn wrap(open: &str, close: &str, elem: &Tag, mcid_to_text: &HashMap<i32, String>) -> String {
	let mut html = format!("<{open}>\n");
	html.push_str(&children(elem, mcid_to_text));
	let _ = writeln!(html, "</{close}>");
	html
}

fn children(elem: &Tag, mcid_to_text: &HashMap<i32, String>) -> String {
	let mut html = String::new();
	for i in 0..elem.child_count() {
		if let Some(child) = elem.child(i) {
			html.push_str(&build_html_table(&child, mcid_to_text));
		}
	}
	html
}

fn html_escape(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Append a PDF table's on-screen text to the buffer and add the Table marker. The text is produced
/// by [`crate::parser::convert::table_text::html_table_to_display`]: the full tab-separated rendering when
/// `render_tables_inline` is set, otherwise a `"[Table]: <first row>"` placeholder. The helper
/// output may span multiple lines (one per table row); each line is recorded as its own
/// `current_lines_info` / `page_display_text` line, mirroring the rest of the PDF line tracking.
/// Kept apart from the walk so the logic is unit-testable without live pdfium objects.
pub(super) fn append_pdf_table_to_buffer(
	buffer: &mut DocumentBuffer,
	html: String,
	pos: usize,
	current_lines_info: &mut Vec<(usize, String)>,
	page_display_text: &mut String,
	render_tables_inline: bool,
) {
	let display_text = html_table_to_display(&html, render_tables_inline);
	// `display_lines_and_length` guards the empty case (an empty inline table) by returning no
	// lines, where a raw `split('\n')` would yield one `""` and emit a spurious blank line.
	let (lines, _) = display_lines_and_length(&display_text);
	for line in lines {
		let line_pos = buffer.current_position();
		current_lines_info.push((line_pos, line.clone()));
		buffer.append(&line);
		buffer.append("\n");
		page_display_text.push_str(&line);
		page_display_text.push('\n');
	}
	let display_len = buffer.current_position() - pos;
	buffer.add_marker(Marker::new(MarkerType::Table, pos).with_reference(html).with_length(display_len));
}
