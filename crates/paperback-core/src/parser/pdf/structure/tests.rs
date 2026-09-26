use std::collections::HashMap;

use super::{
	ImagePlacement, UnclaimedImages, append_pdf_table_to_buffer, flush_block,
	marked_content::{PageText, TreeFacts, fold_repeated_references},
	normalize_list_label,
};
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

/// The Symbol font's bullet reaches the text layer as U+F0B7, a private-use code point a screen
/// reader reads as nothing. A label made only of those is a bullet, and is written as one.
#[test]
fn a_private_use_list_label_becomes_a_bullet() {
	assert_eq!(normalize_list_label("\u{F0B7}"), "\u{2022}");
	assert_eq!(normalize_list_label("\u{F0A7}"), "\u{2022}");
}

/// Labels that carry real characters are left exactly as the document wrote them.
#[test]
fn an_ordinary_list_label_is_left_alone() {
	assert_eq!(normalize_list_label("1."), "1.");
	assert_eq!(normalize_list_label("\u{2022}"), "\u{2022}");
	assert_eq!(normalize_list_label("a)"), "a)");
	assert_eq!(normalize_list_label("\u{F0B7} 1."), "\u{F0B7} 1.");
}

/// The case behind issue #797: a list item's label is held back until the paragraph inside its
/// `LBody` arrives, then goes out on the front of that one line. The label on a line of its own is
/// what made arrowing through a tagged list stop at silent rows.
#[test]
fn a_list_label_joins_the_line_that_follows_it() {
	let mut buffer = DocumentBuffer::new();
	let mut lines_info = Vec::new();
	let mut page_text = String::new();
	let mut label = "\u{2022}".to_string();
	let mut block = String::new();
	let tops = HashMap::new();
	let mut no_images = ImagePlacement { mcid_tops: &tops, unclaimed: UnclaimedImages::new(&[]), block_top: None };
	// The paragraph inside the LBody starts by flushing, with nothing yet collected.
	flush_block(&mut label, &mut block, &mut buffer, &mut page_text, &mut lines_info, &mut no_images);
	assert_eq!(buffer.content, "", "an empty block emits nothing and keeps the label waiting");
	assert_eq!(label, "\u{2022}");
	block.push_str("ANTIPASTI PER DUE");
	flush_block(&mut label, &mut block, &mut buffer, &mut page_text, &mut lines_info, &mut no_images);
	assert_eq!(buffer.content, "\u{2022} ANTIPASTI PER DUE\n");
	assert_eq!(lines_info.len(), 1, "one line, not a label line and a text line");
	assert!(label.is_empty(), "the label is spent once it is written");
}

/// The tree facts gathered from a page whose tree names these ids, in this order.
fn facts_naming(mcids: &[i32]) -> TreeFacts {
	let mut facts = TreeFacts::default();
	for mcid in mcids {
		facts.note_reference(*mcid);
	}
	facts
}

/// A paragraph running onto a new page names that page's first id once for itself and once more
/// for each piece of text it lost there.
#[test]
fn a_reference_made_twice_counts_as_a_repeat() {
	assert_eq!(facts_naming(&[0, 0, 3]).repeats, HashMap::from([(0, 1)]));
	assert_eq!(facts_naming(&[0, 0, 0]).repeats, HashMap::from([(0, 2)]));
}

/// Each id's text on a page, `"t0 "` for id 0 and so on.
fn text_of(mcids: &[i32]) -> HashMap<i32, String> {
	mcids.iter().map(|mcid| (*mcid, format!("t{mcid} "))).collect()
}

/// The page's text after folding, for content running through the ids in `order` with those in
/// `artifacts` inside `Artifact` marks, under a tree naming `named`.
fn folded(order: &[i32], artifacts: &[i32], named: &[i32]) -> HashMap<i32, String> {
	let mut text = text_of(order);
	fold_repeated_references(order, &artifacts.iter().copied().collect(), &facts_naming(named), &mut text);
	text
}

/// A paragraph names id 0 twice, a line number (artifact 1) follows it, and the words after the
/// line number (id 2) are named by nothing.
#[test]
fn a_repeated_reference_takes_the_span_after_the_line_number() {
	let text = folded(&[0, 1, 2, 3], &[1], &[0, 0, 3]);
	assert_eq!(text[&0], "t0 t2 ");
	assert_eq!(text[&3], "t3 ", "the next paragraph keeps its own text");
}

/// Id 0 named three times, with two line numbers and two pieces of text after them.
#[test]
fn a_reference_made_three_times_takes_two_spans() {
	assert_eq!(folded(&[0, 1, 2, 3, 4, 5], &[1, 3], &[0, 0, 0, 5])[&0], "t0 t2 t4 ");
}

#[test]
fn recovery_stops_at_the_next_referenced_mcid() {
	assert_eq!(folded(&[0, 3, 5], &[], &[0, 0, 3])[&0], "t0 ", "id 5 lies past id 3, which the tree names");
}

#[test]
fn recovery_takes_no_more_parts_than_extra_references() {
	assert_eq!(folded(&[0, 2, 4, 3], &[], &[0, 0, 3])[&0], "t0 t2 ");
}

/// Guard: text the tree never names stays out of a page whose tree names every id once.
#[test]
fn a_page_without_repeats_is_left_alone() {
	assert_eq!(folded(&[0, 1, 2, 3], &[1], &[0, 3]), text_of(&[0, 1, 2, 3]));
}

#[test]
fn an_mcid_named_twice_is_read_once() {
	let text = text_of(&[0, 3]);
	let mut page = PageText::new(&text);
	assert_eq!(page.take(0), Some("t0 "));
	assert_eq!(page.take(0), None, "the second reference reads nothing");
	assert_eq!(page.take(3), Some("t3 "));
}
