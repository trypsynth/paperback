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
