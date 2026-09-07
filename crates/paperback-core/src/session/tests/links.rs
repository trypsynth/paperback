use super::*;

#[test]
fn activate_link_returns_not_found_outside_link_text() {
	let session = sample_session(ParserFlags::NONE);
	let result = session.activate_link(2);
	assert!(!result.found);
	assert_eq!(result.action, LinkAction::NotFound);
}

#[test]
fn activate_link_resolves_external_links() {
	let session = sample_session(ParserFlags::NONE);
	let result = session.activate_link(7);
	assert!(result.found);
	assert_eq!(result.action, LinkAction::External);
	assert_eq!(result.url, "https://example.com");
}

#[test]
fn link_list_reports_closest_index_and_text() {
	let session = sample_session(ParserFlags::NONE);
	let list = session.link_list(7);
	assert_eq!(list.items.len(), 1);
	assert_eq!(list.items[0].offset, 6);
	assert_eq!(list.items[0].text, "line2");
	assert_eq!(list.closest_index, 0);
}

#[test]
fn element_list_returns_table_markers_with_their_caption_text() {
	let session = sample_session(ParserFlags::NONE);
	let list = session.element_list(MarkerType::Table, 0);
	assert_eq!(list.items.len(), 1);
	assert_eq!(list.items[0].offset, 12);
	assert_eq!(list.items[0].text, "line3");
	assert_eq!(list.closest_index, -1);
	assert_eq!(session.element_list(MarkerType::Table, 12).closest_index, 0);
}

#[test]
fn element_list_lists_markers_fall_back_to_the_line_they_sit_on() {
	// sample_session's List marker at 6 carries no text, and the ListItem marker at the same
	// offset must not leak into the List rows.
	let session = sample_session(ParserFlags::NONE);
	let list = session.element_list(MarkerType::List, 7);
	assert_eq!(list.items.len(), 1);
	assert_eq!(list.items[0].offset, 6);
	assert_eq!(list.items[0].text, "line2");
	assert_eq!(list.closest_index, 0);
}

#[test]
fn element_list_prefers_a_marker_caption_over_the_line_content() {
	let mut buffer = DocumentBuffer::with_content("Price list    Quantity\n".to_string());
	buffer.add_marker(Marker::new(MarkerType::Table, 0).with_text("Annual Report".to_string()).with_length(24));
	let session = session_from_buffer(buffer);
	let list = session.element_list(MarkerType::Table, 0);
	assert_eq!(list.items[0].text, "Annual Report");
}

#[test]
fn element_list_preserves_the_indented_first_list_line() {
	let mut buffer = DocumentBuffer::with_content("  - Alpha\n".to_string());
	buffer.add_marker(Marker::new(MarkerType::List, 0).with_level(1));
	let session = session_from_buffer(buffer);
	let list = session.element_list(MarkerType::List, 0);
	assert_eq!(list.items[0].text, "  - Alpha");
}

#[test]
fn element_list_closest_index_tracks_the_last_marker_at_or_before_position() {
	let mut buffer = DocumentBuffer::with_content("AAA\nBBB\nCCC\n".to_string());
	// Added out of order to prove rows come back sorted by position.
	buffer.add_marker(Marker::new(MarkerType::List, 4).with_level(1));
	buffer.add_marker(Marker::new(MarkerType::List, 0).with_level(1));
	let session = session_from_buffer(buffer);
	let rows = session.element_list(MarkerType::List, 4);
	assert_eq!(rows.items.len(), 2);
	assert_eq!((rows.items[0].offset, rows.items[0].text.as_str()), (0, "AAA"));
	assert_eq!((rows.items[1].offset, rows.items[1].text.as_str()), (4, "BBB"));
	assert_eq!(session.element_list(MarkerType::List, 0).closest_index, 0);
	assert_eq!(session.element_list(MarkerType::List, 3).closest_index, 0);
	assert_eq!(session.element_list(MarkerType::List, 4).closest_index, 1);
}

#[test]
fn element_list_is_empty_when_the_type_is_absent() {
	let session = sample_session(ParserFlags::NONE);
	let list = session.element_list(MarkerType::Image, 0);
	assert!(list.items.is_empty());
	assert_eq!(list.closest_index, -1);
}

#[test]
fn activate_link_returns_not_found_when_reference_missing() {
	let mut buffer = DocumentBuffer::with_content("line1\nline2".to_string());
	buffer.add_marker(Marker::new(MarkerType::Link, 6).with_text("line2".to_string()));
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		last_stable_position: None,
	};
	let result = session.activate_link(7);
	assert!(!result.found);
	assert_eq!(result.action, LinkAction::NotFound);
}
