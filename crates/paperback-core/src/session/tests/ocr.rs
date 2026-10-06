//! Locating and replacing the image-only page placeholders the PDF parser leaves behind.

use super::*;
use crate::ocr::image_only_placeholder;

/// Two pages of text with an image-only page between them, shaped the way the PDF parser builds
/// one: a page break at the start of each page, and an `ImageOnlyPage` marker sharing the
/// placeholder line's start.
fn session_with_image_only_page() -> DocumentSession {
	let content = format!("page one\n{}\npage three\n", image_only_placeholder());
	let mut buffer = DocumentBuffer::with_content(content);
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 9));
	buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, 9));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 43));
	session_from_buffer(buffer)
}

#[test]
fn line_bounds_at_and_line_text_at_return_the_containing_line() {
	let session = session_with_content("aaa\nbbb\nccc");
	// (start, end) are display units; end excludes the line's trailing newline.
	assert_eq!(session.line_bounds_at(0), Some((0, 3)));
	assert_eq!(session.line_text_at(0), "aaa");
	assert_eq!(session.line_bounds_at(5), Some((4, 7)));
	assert_eq!(session.line_text_at(5), "bbb");
	assert_eq!(session.line_bounds_at(11), Some((8, 11)));
	assert_eq!(session.line_text_at(11), "ccc");
}

#[test]
fn image_only_pages_reports_the_page_number_and_offset() {
	let session = session_with_image_only_page();
	assert_eq!(session.image_only_pages(), vec![(2, 9)]);
}

#[test]
fn image_only_page_at_matches_anywhere_on_the_placeholder_line() {
	let session = session_with_image_only_page();
	assert_eq!(session.image_only_page_at(9), Some(9));
	// Mid-line and at the line's end, since the caret can sit anywhere on it.
	assert_eq!(session.image_only_page_at(20), Some(9));
	assert_eq!(session.image_only_page_at(42), Some(9));
	// The lines either side are ordinary text.
	assert_eq!(session.image_only_page_at(0), None);
	assert_eq!(session.image_only_page_at(45), None);
}

#[test]
fn replace_image_only_pages_swaps_the_line_and_clears_the_marker() {
	let mut session = session_with_image_only_page();
	let outcome = session.replace_image_only_pages(&[(9, "recognized text".to_string())]);
	assert_eq!(session.line_text_at(9), "recognized text");
	// The marker is gone, so the page is not offered for OCR a second time.
	assert!(session.image_only_pages().is_empty());
	assert_eq!(session.image_only_page_at(9), None);
	// The page after it moved by the length difference, and page navigation follows.
	assert_eq!(outcome.total_delta, 15 - 33);
	assert_eq!(session.page_offset(3), 43 + outcome.total_delta);
	assert_eq!(session.line_text_at(session.page_offset(3)), "page three");
}

#[test]
fn replace_image_only_pages_ignores_offsets_that_are_not_placeholders() {
	let mut session = session_with_image_only_page();
	let outcome = session.replace_image_only_pages(&[(0, "not a placeholder".to_string())]);
	assert_eq!(outcome.total_delta, 0);
	assert_eq!(session.line_text_at(0), "page one");
	assert_eq!(session.image_only_pages(), vec![(2, 9)]);
}

/// A session whose lines are each their own page, with a page break at every line start: the
/// shape every paged parser produces. The breaks are placed from the buffer's own running display
/// position rather than written down, so a fixture containing an astral character still lines up
/// with however that platform measures display length.
fn session_with_paged_lines(lines: &[&str]) -> DocumentSession {
	let mut buffer = DocumentBuffer::new();
	for line in lines {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, buffer.current_position()));
		buffer.append(line);
		buffer.append("\n");
	}
	session_from_buffer(buffer)
}

/// A page break with no text and no image between it and the next, which is what the PDF parser
/// leaves for a blank page: two breaks at the same offset, so the page's span is empty.
fn session_with_blank_middle_page() -> DocumentSession {
	let mut buffer = DocumentBuffer::new();
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
	buffer.append("page one\n");
	buffer.add_marker(Marker::new(MarkerType::PageBreak, buffer.current_position()));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, buffer.current_position()));
	buffer.append("page three\n");
	session_from_buffer(buffer)
}

fn marker_count(session: &DocumentSession, mtype: MarkerType) -> usize {
	session.handle.document().buffer.markers.iter().filter(|marker| marker.mtype == mtype).count()
}

#[test]
fn page_text_bounds_covers_each_page_and_ends_the_last_one_at_the_document_end() {
	let session = session_with_paged_lines(&["page one", "page two", "page three"]);
	assert_eq!(session.page_text_bounds(1), Some((0, 9)));
	assert_eq!(session.page_text_bounds(2), Some((9, 18)));
	// The last page has no following page break, so it runs to the end of the document.
	assert_eq!(session.page_text_bounds(3), Some((18, 29)));
	assert_eq!(session.page_text_bounds(4), None);
	assert_eq!(session.page_text_bounds(0), None);
	assert_eq!(session.page_text_bounds(-1), None);
}

#[test]
fn page_text_bounds_of_a_blank_page_is_empty_rather_than_negative() {
	let session = session_with_blank_middle_page();
	// Two page breaks at the same offset, so the page owns no text at all.
	assert_eq!(session.page_text_bounds(2), Some((9, 9)));
	// Replacing one is a pure insertion, and must not underflow.
	let mut session = session;
	session.replace_ocr_pages(&[(2, "found something".to_string())]);
	assert_eq!(session.line_text_at(9), "found something");
	assert_eq!(session.page_count(), 3);
}

/// The change in document length a page swap should report, given the text that went in and out.
/// Both are ASCII, so their byte and display lengths agree on every platform.
fn expected_delta(new_text: &str, old_text: &str) -> i64 {
	i64::try_from(new_text.len()).unwrap_or(0) - i64::try_from(old_text.len()).unwrap_or(0)
}

#[test]
fn replace_ocr_pages_swaps_a_text_page_and_leaves_the_pagination_alone() {
	let mut session = session_with_paged_lines(&["page one", "page two", "page three"]);
	let before = session.page_offset(3);
	let outcome = session.replace_ocr_pages(&[(2, "NEW".to_string())]);
	assert_eq!(outcome.total_delta, expected_delta("NEW\n", "page two\n"));
	// The page break that starts the replaced page survives, so the document still has three
	// pages and the caret's page arithmetic still means something.
	assert_eq!(session.page_count(), 3);
	assert_eq!(session.page_offset(3), before + outcome.total_delta);
	assert_eq!(session.current_page(session.page_offset(3)), 3);
}

#[test]
fn replace_ocr_pages_leaves_the_next_page_starting_on_its_own_line() {
	let mut session = session_with_paged_lines(&["page one", "page two", "page three"]);
	session.replace_ocr_pages(&[(2, "NEW".to_string())]);
	// The page span includes the trailing newline, so the replacement has to put one back. If it
	// did not, this line would read "NEWpage three" and the page break would land mid-line.
	assert_eq!(session.line_text_at(session.page_offset(3)), "page three");
	assert_eq!(session.line_bounds_at(session.page_offset(3)), Some((13, 23)));
}

#[test]
fn replace_ocr_pages_discards_the_markers_inside_a_replaced_text_page() {
	let mut buffer = DocumentBuffer::with_content("page one\npage two\npage three\n".to_string());
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 9));
	buffer.add_marker(Marker::new(MarkerType::Heading1, 10).with_level(1).with_text("Two".to_string()));
	buffer.add_marker(
		Marker::new(MarkerType::Link, 12)
			.with_text("two".to_string())
			.with_reference("https://example.com".to_string()),
	);
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 18));
	let mut session = session_from_buffer(buffer);
	session.replace_ocr_pages(&[(2, "NEW".to_string())]);
	// A page re-read because its text was untrusted should not keep markers pointing into text
	// that is no longer there. An image-only page ends up with no markers either, so this is
	// the same plain result rather than a new inconsistency.
	assert_eq!(marker_count(&session, MarkerType::Heading1), 0);
	assert_eq!(marker_count(&session, MarkerType::Link), 0);
	assert_eq!(marker_count(&session, MarkerType::PageBreak), 3);
}

#[test]
fn replace_ocr_pages_still_takes_the_placeholder_path_for_an_image_only_page() {
	let mut session = session_with_image_only_page();
	let outcome = session.replace_ocr_pages(&[(2, "recognized".to_string())]);
	assert_eq!(session.line_text_at(9), "recognized");
	assert_eq!(outcome.total_delta, 10 - 33);
	// The placeholder line was replaced, not the whole page span, so the newline that ends it is
	// still the one the placeholder line already had. Had the replacement wrongly carried a
	// newline of its own, a blank line would now sit between the two.
	assert_eq!(session.line_text_at(session.page_offset(3) - 1), "recognized");
	assert_eq!(session.line_text_at(session.page_offset(3)), "page three");
	assert_eq!(session.image_only_pages(), vec![]);
}

#[test]
fn replace_ocr_pages_ignores_pages_the_document_does_not_have() {
	let mut session = session_with_paged_lines(&["page one", "page two"]);
	let outcome = session.replace_ocr_pages(&[(9, "out of range".to_string())]);
	assert_eq!(outcome.total_delta, 0);
	assert_eq!(session.line_text_at(0), "page one");
	assert_eq!(session.page_count(), 2);
}

#[test]
fn replace_ocr_pages_measures_spans_in_display_units() {
	// An emoji is two UTF-16 code units on Windows and macOS but one Unicode scalar on GTK, so
	// nothing here hardcodes an offset. What is being checked is that the page span, the page
	// break it starts on and the caret are all measured in the same units whatever those are.
	let mut session = session_with_paged_lines(&["\u{1F600} one", "page two", "page three"]);
	let third = session.page_offset(3);
	assert_eq!(session.line_text_at(third), "page three");
	let outcome = session.replace_ocr_pages(&[(2, "NEW".to_string())]);
	assert_eq!(outcome.total_delta, expected_delta("NEW\n", "page two\n"));
	assert_eq!(session.page_offset(3), third + outcome.total_delta);
	assert_eq!(session.line_text_at(session.page_offset(3)), "page three");
}

#[test]
fn a_comic_page_stays_page_aligned_after_being_recognized() {
	// Shaped the way the comic parser builds one: a page break, an image marker and an
	// `ImageOnlyPage` marker all sharing the placeholder line's start.
	let content = format!("{}\n{}\n", image_only_placeholder(), image_only_placeholder());
	let mut buffer = DocumentBuffer::with_content(content);
	for position in [0, image_only_placeholder().len() + 1] {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, position));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
	}
	let mut session = session_from_buffer(buffer);
	// Recognizing a page consumes its `ImageOnlyPage` marker but leaves the page break, so the
	// page still has a span and can be recognized again -- which is the whole point of being
	// able to re-OCR a page that already has text.
	session.replace_ocr_pages(&[(1, "balloon one".to_string())]);
	// Page 2 moved by however much shorter "balloon one" is than the placeholder line.
	assert_eq!(session.image_only_pages(), vec![(2, 12)]);
	assert_eq!(session.page_count(), 2);
	assert_eq!(session.line_text_at(0), "balloon one");
	session.replace_ocr_pages(&[(1, "better balloon".to_string())]);
	assert_eq!(session.page_count(), 2);
	assert_eq!(session.line_text_at(0), "better balloon");
	assert_eq!(session.line_text_at(session.page_offset(2)), image_only_placeholder());
}
