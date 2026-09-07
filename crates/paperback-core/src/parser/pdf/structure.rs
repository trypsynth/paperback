//! Extracting text from a PDF page via its tagged structure tree, when one is present and
//! trustworthy: walking the tree's `P`/`H1`-`H6`/`L`/`Table`/etc. elements and resolving each
//! leaf's marked-content id to the text pdfium associated with it. This is the preferred
//! extraction path: it recovers real paragraph/heading/list/table structure that the
//! plain-text fallback ([`super::text`]) can only guess at from font size and line shape.
//! Some PDFs advertise a structure tree while leaving most of their text untagged, though, so
//! [`extract_tagged_page_text`] first checks how much of the page's text is actually covered
//! by a marked-content id and bails out to the caller's plain-text fallback below
//! [`MIN_MCID_COVERAGE`]. It bails out the same way when the tree it is handed leads to no text
//! at all, which is what a PDF exported from Apple Pages gives pdfium.

use std::{collections::HashMap, fmt::Write as _};

use pdfium::{PdfiumPage, PdfiumStructElement, PdfiumTextPage};

use super::text::{is_invisible_space, reorder_run};
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
				// A space the page never renders (see `is_invisible_space`) would land in the
				// middle of a word here just as it does in plain extraction.
				if ch == ' ' && is_invisible_space(text_page, i, char_count) {
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
	// pdfium hands back a null child for a top-level element it cannot load, so a page whose only
	// element is one of those walks to nothing at all. A PDF exported from Apple Pages does just
	// that: every glyph carries a marked-content id, the tree reports one child, and that child
	// cannot be reached. Reporting the tagged path as used would hand the caller an empty page
	// and stop plain extraction from ever running over text that is there.
	if mcid_to_text.is_empty() || (0..child_count).all(|i| struct_tree.child(i).is_err()) {
		tracing::warn!(page_index, "page structure tree leads to no text, falling back to plain extraction");
		return false;
	}
	let mut current_block = String::new();
	let mut pending_label = String::new();
	for i in 0..child_count {
		if let Ok(child) = struct_tree.child(i) {
			process_struct_element(
				&child,
				&mcid_to_text,
				buffer,
				page_display_text,
				&mut current_block,
				&mut pending_label,
				current_lines_info,
				flat_toc_items,
				render_tables_inline,
			);
		}
	}
	flush_block(&mut pending_label, &mut current_block, buffer, page_display_text, current_lines_info);
	true
}

/// Replaces a list label made only of private-use characters with a plain bullet.
///
/// The Symbol and Wingdings bullets reach the text layer as private-use code points (U+F0B7 for
/// the Symbol font's, to name the common one). They mean nothing outside the font that drew them,
/// and a screen reader says nothing at all when it meets one. A label that is only those stands
/// in for a bullet, so it is written as one.
pub(super) fn normalize_list_label(label: &str) -> String {
	let is_private_use = |c: char| ('\u{E000}'..='\u{F8FF}').contains(&c);
	if !label.is_empty() && label.chars().all(is_private_use) { "\u{2022}".to_string() } else { label.to_string() }
}

fn flush_block(
	pending_label: &mut String,
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	let trimmed = trim_string(&collapse_whitespace(current_block));
	current_block.clear();
	if trimmed.is_empty() {
		// A label waits here for the block that carries its item's text. An item that never gets
		// any text writes the label out itself, at the end of the item.
		return;
	}
	let line = if pending_label.is_empty() { trimmed } else { format!("{pending_label} {trimmed}") };
	pending_label.clear();
	let offset = buffer.current_position();
	current_lines_info.push((offset, line.clone()));
	buffer.append(&line);
	buffer.append("\n");
	page_display_text.push_str(&line);
	page_display_text.push('\n');
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
	pending_label: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
) {
	let elem_type = elem.element_type().unwrap_or_default();
	if elem_type == "Lbl" {
		// A list item's bullet or number. It is held back rather than added to the block being
		// built, because the paragraph inside the LBody beside it starts by flushing that block:
		// the label would go out as a line of its own, which arrowing through the item meets as a
		// stop with nothing to say.
		let mut label = String::new();
		collect_text(elem, mcid_to_text, &mut label);
		let label = trim_string(&collapse_whitespace(&label));
		if !label.is_empty() {
			*pending_label = normalize_list_label(&label);
		}
		return;
	}
	if elem_type == "Table" {
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info);
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
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info);
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
				pending_label,
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
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info);
		}
		// An item with a label and no text of its own: the label is all there is to show, so it
		// goes out on its own here rather than onto the front of whatever comes next. Writing it
		// at the end of whichever block encloses it also stops a stray label, in a document that
		// puts one somewhere no list item follows, from travelling any further than that.
		if !pending_label.is_empty() {
			current_block.push_str(pending_label);
			pending_label.clear();
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info);
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
mod tests;
