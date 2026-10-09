use super::*;
use crate::util::text::display_len;

/// The session API takes positions as `i64`, while `display_len` measures in `usize`.
fn display_len_i64(s: &str) -> i64 {
	i64::try_from(display_len(s)).unwrap()
}

#[test]
fn status_and_percent_helpers_handle_bounds() {
	let session = sample_session(ParserFlags::NONE);
	let start = session.get_status_info(-5);
	assert_eq!(start.line_number, 1);
	assert_eq!(start.character_number, 1);
	assert_eq!(start.percentage, 0);
	let end = session.get_status_info(999);
	assert_eq!(end.percentage, 100);
	assert_eq!(session.position_from_percent(-10), 0);
	assert_eq!(session.position_from_percent(101), 17);
	assert_eq!(session.position_from_percent(1), 1);
}

#[test]
fn line_and_position_helpers_are_consistent() {
	let session = sample_session(ParserFlags::NONE);
	assert_eq!(session.line_count(), 3);
	assert_eq!(session.position_from_line(1), 0);
	assert_eq!(session.position_from_line(2), 6);
	assert_eq!(session.position_from_line(999), 17);
}

#[test]
fn page_helpers_report_counts_and_offsets() {
	let session = sample_session(ParserFlags::NONE);
	assert_eq!(session.page_count(), 2);
	assert!(session.current_page(0) > 0);
	assert!(session.current_page(8) >= session.current_page(0));
	assert_eq!(session.page_offset(1), 0);
	assert_eq!(session.page_offset(2), 8);
	assert_eq!(session.page_offset(0), -1);
	assert_eq!(session.page_offset(-1), -1);
}

#[test]
fn page_offsets_returns_every_page_break_in_document_order() {
	let session = sample_session(ParserFlags::NONE);
	let offsets = session.page_offsets();
	assert_eq!(offsets, vec![0, 8]);
	assert_eq!(offsets.len(), session.page_count());
	for (index, &offset) in offsets.iter().enumerate() {
		let page = i32::try_from(index).unwrap_or(0) + 1;
		assert_eq!(offset, session.page_offset(page), "page {page} offset must match page_offset");
	}
}

#[test]
fn page_offsets_is_empty_without_page_breaks() {
	let session = session_with_content("no pages here\n");
	assert!(session.page_offsets().is_empty());
	assert_eq!(session.page_count(), 0);
}

#[test]
fn page_offsets_are_in_document_order_even_for_unsorted_insertion() {
	let mut buffer = DocumentBuffer::with_content("aaaaaaaaa\nbbbbbbbbb\n".to_string());
	// Inserted out of order: the buffer sorts markers at DocumentHandle::new.
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 9));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
	let session = session_from_buffer(buffer);
	assert_eq!(session.page_offsets(), vec![0, 9]);
}

#[test]
fn text_range_and_line_text_extract_expected_content() {
	let session = sample_session(ParserFlags::NONE);
	assert_eq!(session.get_text_range(0, 5), "line1");
	assert_eq!(session.get_text_range(5, 5), "");
	assert_eq!(session.get_line_text(0), "line1");
	assert_eq!(session.get_line_text(7), "line2");
	assert_eq!(session.get_line_text(999), "line3");
}

#[test]
fn has_headings_checks_specific_and_any_levels() {
	let session = sample_session(ParserFlags::NONE);
	assert!(session.has_headings(None));
	assert!(session.has_headings(Some(1)));
	assert!(!session.has_headings(Some(2)));
	assert!(!session.has_headings(Some(99)));
}

#[test]
fn get_formatting_markers_returns_only_bold_italic_underline_markers() {
	let mut buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
	buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
	buffer.add_marker(Marker::new(MarkerType::Bold, 0).with_length(5));
	buffer.add_marker(Marker::new(MarkerType::Italic, 6).with_length(5));
	buffer.add_marker(Marker::new(MarkerType::Underline, 12).with_length(5));
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		parse_settings: ParseSettings::default(),
	};
	let markers = session.get_formatting_markers();
	assert_eq!(markers.len(), 3);
	assert_eq!(markers[0].mtype, MarkerType::Bold);
	assert_eq!(markers[0].position, 0);
	assert_eq!(markers[0].length, 5);
	assert_eq!(markers[1].mtype, MarkerType::Italic);
	assert_eq!(markers[1].position, 6);
	assert_eq!(markers[1].length, 5);
	assert_eq!(markers[2].mtype, MarkerType::Underline);
	assert_eq!(markers[2].position, 12);
	assert_eq!(markers[2].length, 5);
}

#[test]
fn table_and_section_accessors_require_in_range_and_reference() {
	let session = sample_session(ParserFlags::NONE);
	assert_eq!(session.get_table_at_position(13).as_deref(), Some("<table/>"));
	assert!(session.get_table_at_position(2).is_none());
	assert_eq!(session.get_current_section_path(0).as_deref(), Some("chapter1.xhtml"));
}

#[test]
fn get_text_segment_current_direction_finds_enclosing_paragraph_start() {
	// "line2" spans bytes 6..11; position 8 lands mid-paragraph, e.g. where a link marker
	// embedded in the middle of a sentence would sit. The Current direction must still
	// return the full enclosing paragraph, not a suffix truncated from the given position.
	let session = sample_session(ParserFlags::NONE);
	let seg = session.get_text_segment(8, SegmentTypeFfi::Paragraph, SegmentDirectionFfi::Current);
	assert_eq!(seg.text, "line2");
	assert_eq!(seg.start_pos, 6);
	assert_eq!(seg.end_pos, 11);
}

#[test]
fn heading_tree_builds_parent_links_and_closest_index() {
	let mut buffer = DocumentBuffer::with_content("a\nb\nc".to_string());
	buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
	buffer.add_marker(Marker::new(MarkerType::Heading2, 2).with_level(2).with_text("H2".to_string()));
	buffer.add_marker(Marker::new(MarkerType::Heading1, 4).with_level(1).with_text("H1b".to_string()));
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		parse_settings: ParseSettings::default(),
	};
	let tree = session.heading_tree(3);
	assert_eq!(tree.items.len(), 3);
	assert_eq!(tree.items[0].parent_index, -1);
	assert_eq!(tree.items[1].parent_index, 0);
	assert_eq!(tree.items[2].parent_index, -1);
	assert_eq!(tree.closest_index, 1);
}

#[test]
fn get_current_section_path_returns_none_when_reference_empty() {
	let mut buffer = DocumentBuffer::with_content("line".to_string());
	buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0));
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		parse_settings: ParseSettings::default(),
	};
	assert!(session.get_current_section_path(0).is_none());
}

/// Builds a session whose buffer contains a table marker spanning a display range, used to
/// exercise `get_table_at_position`'s half-open `[position, position + length)` check.
fn table_session() -> DocumentSession {
	// Layout (display units): "before\n" (0..7), table span "tbl\n" (7..11), "after\n" (11..17).
	let html = "<table><tr><td>a</td><td>b</td></tr></table>";
	let mut buffer = DocumentBuffer::with_content("before\ntbl\nafter\n".to_string());
	// Table marker length is the DISPLAY extent of the emitted span ("tbl\n" -> 4).
	buffer.add_marker(
		Marker::new(MarkerType::Table, 7).with_length(4).with_text("tbl".to_string()).with_reference(html.to_string()),
	);
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	doc.compute_stats();
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		parse_settings: ParseSettings::default(),
	}
}

#[test]
fn get_table_at_position_uses_display_length() {
	let session = table_session();
	// Table marker at display position 7 with length 4 -> half-open range [7, 11).
	assert_eq!(session.get_table_at_position(7).as_deref(), Some("<table><tr><td>a</td><td>b</td></tr></table>"));
	assert_eq!(session.get_table_at_position(10).as_deref(), Some("<table><tr><td>a</td><td>b</td></tr></table>"));
	assert!(session.get_table_at_position(11).is_none());
	assert!(session.get_table_at_position(2).is_none());
}

#[test]
fn get_table_at_position_handles_multibyte_extent() {
	// A table marker whose display extent exceeds its char-count would have been mis-measured
	// by the old `text.chars().count()` logic. Here the displayed text is shorter (in chars)
	// than the display extent, so the caret near the end is only inside the table when using
	// `marker.length`.
	let mut buffer = DocumentBuffer::with_content("\u{1F600}\u{1F600}\u{1F600}".to_string());
	// Three non-BMP emoji: 3 chars but 6 display (UTF-16) units. Marker spans the whole range.
	buffer.add_marker(
		Marker::new(MarkerType::Table, 0)
			.with_length(6)
			.with_text("x".to_string())
			.with_reference("<table/>".to_string()),
	);
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	doc.compute_stats();
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		parse_settings: ParseSettings::default(),
	};
	// Position 5 is within [0, 6) by display length but would be outside [0, 1) by char count.
	assert_eq!(session.get_table_at_position(5).as_deref(), Some("<table/>"));
	assert!(session.get_table_at_position(6).is_none());
}

#[test]
fn line_and_status_helpers_use_display_positions_after_emoji() {
	let content = "😀😀😀😀\nab\ncdefgh\nij\n";
	let ab = display_len("😀😀😀😀\n");
	let next = display_len("😀😀😀😀\nab\n");
	let ab_pos = display_len_i64("😀😀😀😀\n");
	let next_pos = display_len_i64("😀😀😀😀\nab\n");
	let end = display_len_i64(content);
	let mut buffer = DocumentBuffer::with_content(content.to_string());
	buffer.add_marker(Marker::new(MarkerType::Bold, ab).with_length(display_len("ab")));
	buffer.add_marker(Marker::new(MarkerType::Italic, next).with_length(display_len("cdefgh")));
	let session = session_from_buffer(buffer);
	assert_eq!(session.get_line_text(ab_pos), "ab");
	assert_eq!(session.first_content_line_after(ab_pos), "ab");
	assert_eq!(session.line_from_position(ab_pos), 2);
	assert_eq!(session.position_from_line(2), ab_pos);
	assert_eq!(session.position_from_line(3), next_pos);
	let status = session.get_status_info(ab_pos);
	assert_eq!(status.line_number, 2);
	assert_eq!(status.character_number, 6);
	assert_eq!(status.percentage, 27);
	assert_eq!(session.position_from_percent(27), ab_pos);
	assert_eq!(session.position_from_percent(1), display_len_i64("😀"));
	let markers = session.get_line_markers(2);
	assert_eq!(markers.len(), 1);
	assert_eq!(markers[0].mtype, MarkerType::Bold);
	assert_eq!(markers[0].position, ab_pos);
	assert_eq!(markers[0].length, display_len_i64("ab"));
	assert_eq!(session.get_line_markers(3)[0].mtype, MarkerType::Italic);
	assert_eq!(session.line_from_position(ab_pos - 1), 1);
	assert_eq!(session.get_line_text(ab_pos - 1), "😀😀😀😀");
	assert_eq!(session.get_line_text(-1), "😀😀😀😀");
	assert_eq!(session.line_from_position(-1), 1);
	assert_eq!(session.position_from_line(0), 0);
	assert_eq!(session.position_from_line(999), end);
	assert_eq!(session.position_from_percent(-1), 0);
	assert_eq!(session.position_from_percent(101), end);
	assert_eq!(session.get_line_text(i64::MAX), "");
	assert_eq!(session.first_content_line_after(i64::MAX), "");
	assert_eq!(session.line_from_position(i64::MAX), 5);
	let status = session.get_status_info(i64::MAX);
	assert_eq!(status.line_number, 5);
	assert_eq!(status.character_number, 19);
	assert_eq!(status.percentage, 100);
}

#[test]
fn line_helpers_handle_empty_and_unterminated_documents() {
	let empty = session_with_content("");
	assert_eq!(empty.get_line_text(100), "");
	assert_eq!(empty.first_content_line_after(100), "");
	assert_eq!(empty.line_from_position(100), 1);
	assert_eq!(empty.position_from_line(100), 0);
	assert_eq!(empty.position_from_percent(100), 0);
	assert_eq!(empty.get_status_info(100).percentage, 0);
	let session = session_with_content("😀\nab");
	assert_eq!(session.get_line_text(i64::MAX), "ab");
	assert_eq!(session.first_content_line_after(i64::MAX), "ab");
	assert_eq!(session.position_from_line(2), display_len_i64("😀\n"));
	assert_eq!(session.position_from_line(3), display_len_i64("😀\nab"));
}

#[test]
fn first_content_line_after_skips_headers_after_emoji() {
	let session = session_with_content("😀\nPage 27\n\nab\n");
	assert_eq!(session.first_content_line_after(display_len_i64("😀\n")), "ab");
}

#[test]
fn line_segments_keep_char_indices_when_line_helpers_use_display_units() {
	let session = session_with_content("😀😀😀😀\nab\ncdefgh\nij\n");
	let segment = session.get_text_segment(5, SegmentTypeFfi::Line, SegmentDirectionFfi::Current);
	assert_eq!(segment.text, "ab");
	assert_eq!(segment.start_pos, 5);
	assert_eq!(segment.end_pos, 8);
}
