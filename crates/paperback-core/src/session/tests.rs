use super::*;
use crate::document::{Document, DocumentBuffer, Marker};

mod accessors;
mod audio;
mod links;
mod navigation;
mod webview;

fn sample_session(parser_flags: ParserFlags) -> DocumentSession {
	let mut buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
	buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0).with_reference("chapter1.xhtml".to_string()));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
	buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
	buffer.add_marker(
		Marker::new(MarkerType::Link, 6)
			.with_text("line2".to_string())
			.with_reference("https://example.com".to_string()),
	);
	buffer.add_marker(Marker::new(MarkerType::List, 6).with_level(1));
	buffer.add_marker(Marker::new(MarkerType::ListItem, 6).with_level(1).with_text("item".to_string()));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 8));
	buffer.add_marker(
		Marker::new(MarkerType::Table, 12)
			.with_length(5)
			.with_text("line3".to_string())
			.with_reference("<table/>".to_string()),
	);
	buffer.add_marker(Marker::new(MarkerType::Separator, 5).with_length(1));
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags,
		last_stable_position: None,
	}
}

fn session_with_content(content: &str) -> DocumentSession {
	let buffer = DocumentBuffer::with_content(content.to_string());
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
