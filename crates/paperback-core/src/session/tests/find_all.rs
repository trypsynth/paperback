use super::*;
use crate::reader_core::SearchOptions;

fn session_with_marks(content: &str, markers: Vec<Marker>) -> DocumentSession {
	let mut buffer = DocumentBuffer::with_content(content.to_string());
	for marker in markers {
		buffer.add_marker(marker);
	}
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::empty(),
		last_stable_position: None,
	}
}

#[test]
fn find_all_groups_by_line_and_orders_rows() {
	let session = session_with_content("alpha beta\ngamma beta\n\ndelta beta\n");
	let rows = session.find_all_lines("beta", SearchOptions::empty());
	assert_eq!(rows.len(), 3, "the blank middle line holds no match and is skipped");
	assert_eq!(rows[0].text, "alpha beta");
	assert_eq!(rows[1].text, "gamma beta");
	assert_eq!(rows[2].text, "delta beta");
	assert_eq!(rows[0].page, 0);
	assert!(rows.iter().all(|row| row.matches.len() == 1));
}

#[test]
fn find_all_dedupes_by_line_identity_not_text() {
	let session = session_with_content("same word same\nother\nsame word same\n");
	let rows = session.find_all_lines("same", SearchOptions::empty());
	assert_eq!(rows.len(), 2, "two identical sentences on two lines are two rows");
	assert_eq!(rows[0].text, "same word same");
	assert_eq!(rows[1].text, "same word same");
	assert_eq!(rows[0].matches.len(), 2, "both occurrences on the line are kept on its one row");
	assert_eq!(rows[1].matches.len(), 2);
	assert_eq!((rows[0].matches[0].start, rows[0].matches[0].end), (0, 4));
	assert_eq!((rows[0].matches[1].start, rows[0].matches[1].end), (10, 14));
}

#[test]
fn find_all_multiple_matches_on_a_line_preserve_order() {
	let session = session_with_content("a a a\n");
	let rows = session.find_all_lines("a", SearchOptions::empty());
	assert_eq!(rows.len(), 1);
	let spans = &rows[0].matches;
	assert_eq!(spans.len(), 3);
	assert_eq!((spans[0].start, spans[0].end), (0, 1));
	assert_eq!((spans[1].start, spans[1].end), (2, 3));
	assert_eq!((spans[2].start, spans[2].end), (4, 5));
}

#[test]
fn find_all_respects_case_and_whole_word() {
	let session = session_with_content("The cat. the the theatre\n");
	let options = SearchOptions::WHOLE_WORD | SearchOptions::MATCH_CASE;
	let rows = session.find_all_lines("the", options);
	assert_eq!(rows.len(), 1);
	assert_eq!(rows[0].matches.len(), 2, "\"The\" and \"theatre\" are excluded");
}

#[test]
fn find_all_uses_regex_and_real_spans() {
	let session = session_with_content("a12 b345 c6\n");
	let rows = session.find_all_lines(r"\d+", SearchOptions::REGEX);
	assert_eq!(rows.len(), 1);
	let spans = &rows[0].matches;
	assert_eq!(spans.len(), 3);
	assert_eq!((spans[0].start, spans[0].end), (1, 3));
	assert_eq!((spans[1].start, spans[1].end), (5, 8));
	assert_eq!((spans[2].start, spans[2].end), (10, 11));
}

#[test]
fn find_all_without_page_markers_reports_page_zero() {
	let session = session_with_content("just beta here\n");
	let rows = session.find_all_lines("beta", SearchOptions::empty());
	assert_eq!(rows.len(), 1);
	assert_eq!(rows[0].page, 0);
}

#[test]
fn find_all_assigns_pages_from_page_break_markers() {
	// Three lines of identical length; a PageBreak marker sits at each line's start.
	let session = session_with_marks(
		"aaa beta\nbbb beta\nccc beta\n",
		vec![
			Marker::new(MarkerType::PageBreak, 0),
			Marker::new(MarkerType::PageBreak, 9),
			Marker::new(MarkerType::PageBreak, 18),
		],
	);
	let rows = session.find_all_lines("beta", SearchOptions::empty());
	assert_eq!(rows.len(), 3);
	let pages: Vec<i32> = rows.iter().map(|row| row.page).collect();
	assert_eq!(pages, vec![1, 2, 3]);
}

#[test]
fn find_all_keeps_the_whole_line_across_an_astral_char() {
	// The emoji is inside the line, between the two matches; the row must still be the whole
	// line, whichever side of the astral character the match sits on.
	let session = session_with_content("beta 😀 beta\n");
	let rows = session.find_all_lines("beta", SearchOptions::empty());
	assert_eq!(rows.len(), 1);
	assert_eq!(rows[0].text, "beta 😀 beta");
	assert_eq!(rows[0].matches.len(), 2);
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[test]
fn find_all_reports_utf16_spans_past_an_astral_char() {
	// On Windows/macOS the display unit is a UTF-16 code unit, so the emoji is two units and the
	// second match starts at 8.
	let session = session_with_content("beta 😀 beta\n");
	let rows = session.find_all_lines("beta", SearchOptions::empty());
	assert_eq!((rows[0].matches[1].start, rows[0].matches[1].end), (8, 12));
}

#[test]
fn find_all_is_empty_for_no_matches_or_empty_query() {
	let session = session_with_content("nothing to see\n");
	assert!(session.find_all_lines("zzz", SearchOptions::empty()).is_empty());
	assert!(session.find_all_lines("", SearchOptions::empty()).is_empty());
}
