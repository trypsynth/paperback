//! Extracting text from a PDF page via its tagged structure tree, when one is present and
//! trustworthy: walking the tree's `P`/`H1`-`H6`/`L`/`Table`/etc. elements and resolving each
//! leaf's marked-content id to the text pdfium associated with it. This is the preferred
//! extraction path — it recovers real paragraph/heading/list/table structure that the
//! plain-text fallback ([`super::text`]) can only guess at from font size and line shape —
//! but some PDFs advertise a structure tree while leaving most of their text untagged, so
//! [`extract_tagged_page_text`] first checks how much of the page's text is actually covered
//! by a marked-content id and bails out to the caller's plain-text fallback below
//! [`MIN_MCID_COVERAGE`].

use std::{collections::HashMap, fmt::Write as _};

use pdfium::{PdfiumPage, PdfiumStructElement, PdfiumTextPage};

use super::text::reorder_run;
use crate::{
	document::{DocumentBuffer, Marker, MarkerType, TocItem},
	parser::convert::table_text::{display_lines_and_length, html_table_to_display},
	util::text::{collapse_whitespace, display_len, trim_string},
};

/// Minimum fraction of visible text glyphs that must be associated with a
/// marked-content ID for the tagged-extraction path to be trusted. Some PDFs
/// advertise a structure tree while leaving their text essentially untagged
/// (no MCIDs, or wrapped only in `/Artifact` marks); below this threshold the
/// structure tree is treated as unreliable and plain extraction is used instead.
const MIN_MCID_COVERAGE: f64 = 0.5;

/// Attempts tagged extraction for one page: builds the marked-content-id → text map, checks
/// its coverage against [`MIN_MCID_COVERAGE`], and if trusted walks the structure tree into
/// `buffer`/`page_display_text`/`current_lines_info`/`flat_toc_items`. Returns whether tagged
/// extraction was actually used; the caller falls back to plain-text extraction when it isn't.
#[allow(clippy::too_many_arguments)]
pub(super) fn extract_tagged_page_text(
	page: &PdfiumPage,
	text_page: &PdfiumTextPage,
	page_index: i32,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	flat_toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
) -> bool {
	let Some(struct_tree) = page.struct_tree() else { return false };
	let child_count = struct_tree.count_children();
	if child_count == 0 {
		return false;
	}
	let mut mcid_to_text: HashMap<i32, String> = HashMap::new();
	let mut real_char_count: usize = 0;
	let mut mcid_char_count: usize = 0;
	if let Ok(char_count) = text_page.char_count() {
		let mut current_mcid = -1;
		// Chars of the current marked-content run with their pdfium index, so RTL
		// runs can be reordered visual→logical per run.
		let mut current_chars: Vec<(char, i32)> = Vec::new();
		for i in 0..char_count {
			let unicode = text_page.get_unicode(i);
			if let Some(ch) = char::from_u32(unicode) {
				if (ch.is_control() && !matches!(ch, '\n' | '\r' | '\t')) || ch == '\u{00AD}' {
					continue;
				}
				let is_generated = text_page.is_generated(i).unwrap_or(false);
				let mut char_mcid = -1;
				if !is_generated && let Ok(obj) = text_page.get_text_object(i) {
					char_mcid = obj.get_marked_content_id();
				}
				if !is_generated && !ch.is_whitespace() {
					real_char_count += 1;
					if char_mcid >= 0 {
						mcid_char_count += 1;
					}
				}
				if char_mcid >= 0 && char_mcid != current_mcid {
					if current_mcid >= 0 && !current_chars.is_empty() {
						mcid_to_text.entry(current_mcid).or_default().push_str(&reorder_run(text_page, &current_chars));
					}
					current_chars.clear();
					current_mcid = char_mcid;
				}
				current_chars.push((ch, i));
			}
		}
		if current_mcid >= 0 && !current_chars.is_empty() {
			mcid_to_text.entry(current_mcid).or_default().push_str(&reorder_run(text_page, &current_chars));
		}
	}
	let coverage = if real_char_count > 0 { mcid_char_count as f64 / real_char_count as f64 } else { 1.0 };
	let tagged_trusted = coverage >= MIN_MCID_COVERAGE;
	tracing::debug!(page_index, coverage, tagged_trusted, "computed mcid coverage for page structure tree");
	if !tagged_trusted {
		tracing::warn!(
			page_index,
			coverage,
			"page advertises a structure tree but mcid coverage is too low, falling back to plain extraction"
		);
		return false;
	}
	let mut current_block = String::new();
	for i in 0..child_count {
		if let Ok(child) = struct_tree.child(i) {
			process_struct_element(
				&child,
				&mcid_to_text,
				buffer,
				page_display_text,
				&mut current_block,
				current_lines_info,
				flat_toc_items,
				render_tables_inline,
			);
		}
	}
	flush_block(&mut current_block, buffer, page_display_text, current_lines_info);
	true
}

fn flush_block(
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	let trimmed = trim_string(&collapse_whitespace(current_block));
	if !trimmed.is_empty() {
		let offset = buffer.current_position();
		current_lines_info.push((offset, trimmed.clone()));
		buffer.append(&trimmed);
		buffer.append("\n");
		page_display_text.push_str(&trimmed);
		page_display_text.push('\n');
	}
	current_block.clear();
}

/// Like `flush_block`, but preserves line breaks within the content instead of
/// collapsing them. Used for preformatted elements like `Code`.
fn flush_block_lines(
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	let text = current_block.clone();
	current_block.clear();
	for line in text.split('\n') {
		let trimmed = trim_string(&collapse_whitespace(line));
		if !trimmed.is_empty() {
			let offset = buffer.current_position();
			current_lines_info.push((offset, trimmed.clone()));
			buffer.append(&trimmed);
			buffer.append("\n");
			page_display_text.push_str(&trimmed);
			page_display_text.push('\n');
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn process_struct_element(
	elem: &PdfiumStructElement,
	mcid_to_text: &HashMap<i32, String>,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_block: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
) {
	let elem_type = elem.element_type().unwrap_or_default();
	if elem_type == "Table" {
		flush_block(current_block, buffer, page_display_text, current_lines_info);
		let html = build_html_table(elem, mcid_to_text);
		let pos = buffer.current_position();
		append_pdf_table_to_buffer(buffer, html, pos, current_lines_info, page_display_text, render_tables_inline);
		return;
	}
	let is_block = matches!(
		elem_type.as_str(),
		"P" | "H"
			| "H1" | "H2"
			| "H3" | "H4"
			| "H5" | "H6"
			| "L" | "LI"
			| "Div" | "Sect"
			| "Part" | "Art"
			| "TOC" | "TOCI"
			| "Code"
	);
	let preserve_lines = elem_type == "Code";
	if is_block {
		flush_block(current_block, buffer, page_display_text, current_lines_info);
	}
	let block_start_pos = buffer.current_position() + display_len(current_block);
	let count = elem.count_children();
	for i in 0..count {
		if let Ok(child) = elem.child(i) {
			process_struct_element(
				&child,
				mcid_to_text,
				buffer,
				page_display_text,
				current_block,
				current_lines_info,
				toc_items,
				render_tables_inline,
			);
		} else if let Some(mcid) = elem.child_marked_content_id(i)
			&& let Some(text) = mcid_to_text.get(&mcid)
		{
			current_block.push_str(text);
		}
	}
	if is_block {
		if preserve_lines {
			flush_block_lines(current_block, buffer, page_display_text, current_lines_info);
		} else {
			flush_block(current_block, buffer, page_display_text, current_lines_info);
		}
		let heading_level = match elem_type.as_str() {
			"H1" | "H" => Some(1), // "H" is a fallback generic heading, treated as H1
			"H2" => Some(2),
			"H3" => Some(3),
			"H4" => Some(4),
			"H5" => Some(5),
			"H6" => Some(6),
			_ => None,
		};
		if let Some(level) = heading_level {
			let mut title = String::new();
			collect_text(elem, mcid_to_text, &mut title);
			let title = trim_string(&collapse_whitespace(&title));
			if !title.is_empty() {
				let marker_type = match level {
					1 => MarkerType::Heading1,
					2 => MarkerType::Heading2,
					3 => MarkerType::Heading3,
					4 => MarkerType::Heading4,
					5 => MarkerType::Heading5,
					_ => MarkerType::Heading6,
				};
				buffer.add_marker(Marker::new(marker_type, block_start_pos).with_text(title.clone()).with_level(level));
				toc_items.push((level as u32, TocItem::new(title, String::new(), block_start_pos)));
			}
		}
		if elem_type == "L" || elem_type == "TOC" {
			let child_count = elem.count_children();
			buffer.add_marker(Marker::new(MarkerType::List, block_start_pos).with_level(child_count));
		}
		if elem_type == "LI" || elem_type == "TOCI" {
			let mut li_text = String::new();
			collect_text(elem, mcid_to_text, &mut li_text);
			let li_text = trim_string(&collapse_whitespace(&li_text));
			buffer.add_marker(Marker::new(MarkerType::ListItem, block_start_pos).with_text(li_text));
		}
	}
}

fn build_html_table(elem: &PdfiumStructElement, mcid_to_text: &HashMap<i32, String>) -> String {
	let elem_type = elem.element_type().unwrap_or_default();
	if elem_type == "Table" {
		let mut html = String::from("<table border=\"1\">\n");
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html.push_str("</table>\n");
		html
	} else if elem_type == "TR" {
		let mut html = String::from("<tr>\n");
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html.push_str("</tr>\n");
		html
	} else if elem_type == "TH" || elem_type == "TD" {
		let mut html = format!("<{}>", elem_type.to_lowercase());
		let mut cell_text = String::new();
		collect_text(elem, mcid_to_text, &mut cell_text);
		html.push_str(&html_escape(&trim_string(&collapse_whitespace(&cell_text))));
		let _ = writeln!(html, "</{}>", elem_type.to_lowercase());
		html
	} else {
		let mut html = String::new();
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html
	}
}

fn collect_text(elem: &PdfiumStructElement, mcid_to_text: &HashMap<i32, String>, out: &mut String) {
	let count = elem.count_children();
	for i in 0..count {
		if let Ok(child) = elem.child(i) {
			collect_text(&child, mcid_to_text, out);
		} else if let Some(mcid) = elem.child_marked_content_id(i)
			&& let Some(text) = mcid_to_text.get(&mcid)
		{
			out.push_str(text);
		}
	}
}

fn html_escape(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Append a PDF table's on-screen text to the buffer and add the Table marker. The text is produced
/// by [`crate::parser::convert::table_text::html_table_to_display`]: the full tab-separated rendering when
/// `render_tables_inline` is set, otherwise a `"[Table]: <first row>"` placeholder. The helper
/// output may span multiple lines (one per table row); each line is recorded as its own
/// `current_lines_info` / `page_display_text` line, mirroring the rest of the PDF line tracking.
/// Extracted from `process_struct_element` so the logic is unit-testable without live pdfium objects.
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

#[cfg(test)]
mod tests {
	use super::append_pdf_table_to_buffer;
	use crate::document::{DocumentBuffer, MarkerType};

	/// OFF mode: the PDF table helper emits a single `"[Table]: <first row>"` placeholder line and
	/// the Table marker's length equals the emitted display extent. The HTML has a non-BMP char
	/// (U+1D11E, G Clef) in a cell to lock display-unit math (it takes 2 UTF-16 units).
	#[test]
	fn pdf_table_helper_emits_placeholder_when_off() {
		use crate::util::text::display_len;
		let html = "<table border=\"1\">\n<tr>\n<td>Kop</td>\n<td>\u{1D11E}</td>\n</tr>\n</table>\n".to_string();
		let mut buffer = DocumentBuffer::new();
		let pos = buffer.current_position();
		let mut lines_info = Vec::new();
		let mut page_text = String::new();
		append_pdf_table_to_buffer(&mut buffer, html.clone(), pos, &mut lines_info, &mut page_text, false);
		// Placeholder: first row with tabs->spaces.
		assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
		assert_eq!(lines_info.len(), 1, "placeholder is a single line");
		assert!(lines_info[0].1.starts_with("[Table]: "));
		// Table marker length equals the emitted display extent.
		let placeholder_len = display_len("[Table]: Kop \u{1D11E}") + 1; // +1 for trailing newline
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker present");
		assert_eq!(table_marker.position, 0);
		assert_eq!(table_marker.length, placeholder_len, "marker length in display units");
		assert_eq!(table_marker.reference, html, "marker keeps the table HTML");
	}

	/// ON mode: the helper emits the full TSV; multi-row tables produce one line per row, and the
	/// marker length spans all emitted lines.
	#[test]
	fn pdf_table_helper_emits_tsv_when_inline() {
		use crate::util::text::display_len;
		let html =
			"<table border=\"1\">\n<tr>\n<td>Kop</td>\n<td>\u{1D11E}</td>\n</tr>\n<tr>\n<td>a</td>\n<td>b</td>\n</tr>\n</table>\n"
				.to_string();
		let mut buffer = DocumentBuffer::new();
		let pos = buffer.current_position();
		let mut lines_info = Vec::new();
		let mut page_text = String::new();
		append_pdf_table_to_buffer(&mut buffer, html, pos, &mut lines_info, &mut page_text, true);
		// Two rows -> "Kop\t𝄞\na\tb\n".
		assert_eq!(buffer.content, "Kop\t\u{1D11E}\na\tb\n");
		assert_eq!(lines_info.len(), 2, "one line per table row");
		assert_eq!(lines_info[0].1, "Kop\t\u{1D11E}");
		assert_eq!(lines_info[1].1, "a\tb");
		let expected_len = display_len("Kop\t\u{1D11E}\na\tb\n");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker present");
		assert_eq!(table_marker.length, expected_len, "marker length spans all emitted rows");
	}

	/// An empty inline table must emit no line at all (a raw `split('\n')` would emit one spurious
	/// blank line). The buffer stays empty and the Table marker has zero length.
	#[test]
	fn pdf_table_helper_empty_inline_emits_no_line() {
		let html = "<table border=\"1\">\n</table>\n".to_string();
		let mut buffer = DocumentBuffer::new();
		let pos = buffer.current_position();
		let mut lines_info = Vec::new();
		let mut page_text = String::new();
		append_pdf_table_to_buffer(&mut buffer, html, pos, &mut lines_info, &mut page_text, true);
		assert_eq!(buffer.content, "", "empty inline table appends nothing");
		assert!(lines_info.is_empty(), "no lines recorded");
		assert!(page_text.is_empty(), "no page display text");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker present");
		assert_eq!(table_marker.length, 0, "zero-length marker for empty inline table");
	}
}
