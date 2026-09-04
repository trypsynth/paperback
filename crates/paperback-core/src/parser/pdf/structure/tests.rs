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
