//! Tables go through HTML because every other format's tables reach the reader through [`crate::parser::convert::table_text`], and the table view opened with Enter is given the same HTML.

use std::{collections::HashMap, fmt::Write as _};

use super::marked_content::collect_text;
use crate::{
	document::{DocPos, DocumentBuffer, Marker, MarkerType},
	parser::convert::table_text::{display_lines_and_length, html_table_to_display},
	pdf_engine::Tag,
	util::text::{collapse_whitespace, trim_string},
};

/// Anything between the table and its rows (a `THead`, a `Div` some producer put in) is walked through rather than written, so a table keeps its shape whatever wraps it.
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

pub(super) fn append_pdf_table_to_buffer(
	buffer: &mut DocumentBuffer,
	html: String,
	pos: DocPos,
	current_lines_info: &mut Vec<(DocPos, String)>,
	page_display_text: &mut String,
	render_tables_inline: bool,
) {
	let display_text = html_table_to_display(&html, render_tables_inline);
	// Not a raw `split('\n')`, which would emit a spurious blank line for an empty inline table.
	let (lines, _) = display_lines_and_length(&display_text);
	for line in lines {
		let line_pos = buffer.end();
		current_lines_info.push((line_pos, line.clone()));
		buffer.append(&line);
		buffer.append("\n");
		page_display_text.push_str(&line);
		page_display_text.push('\n');
	}
	let table_len = buffer.end() - pos;
	buffer.add_marker(Marker::new(MarkerType::Table, pos).with_reference(html).with_length(table_len));
}
