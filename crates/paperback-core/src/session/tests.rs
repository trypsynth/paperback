use super::*;
use crate::document::{Document, DocumentBuffer, Marker};

mod accessors;
mod audio;
mod find_all;
mod formula;
mod images;
mod links;
mod navigation;
mod ocr;
mod reading_step;
mod webview;

fn sample_session(parser_flags: ParserFlags) -> DocumentSession {
	let mut buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
	buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0.into()).with_reference("chapter1.xhtml".to_string()));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 0.into()));
	buffer.add_marker(Marker::new(MarkerType::Heading1, 0.into()).with_level(1).with_text("H1".to_string()));
	buffer.add_marker(
		Marker::new(MarkerType::Link, 6.into())
			.with_text("line2".to_string())
			.with_reference("https://example.com".to_string()),
	);
	buffer.add_marker(Marker::new(MarkerType::List, 6.into()).with_level(1));
	buffer.add_marker(Marker::new(MarkerType::ListItem, 6.into()).with_level(1).with_text("item".to_string()));
	buffer.add_marker(Marker::new(MarkerType::PageBreak, 8.into()));
	buffer.add_marker(
		Marker::new(MarkerType::Table, 12.into())
			.with_length(5.into())
			.with_text("line3".to_string())
			.with_reference("<table/>".to_string()),
	);
	buffer.add_marker(Marker::new(MarkerType::Separator, 5.into()).with_length(1.into()));
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags,
		parse_settings: ParseSettings::default(),
		password: String::new(),
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
		parse_settings: ParseSettings::default(),
		password: String::new(),
	}
}

fn session_from_buffer(buffer: DocumentBuffer) -> DocumentSession {
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::empty(),
		parse_settings: ParseSettings::default(),
		password: String::new(),
	}
}
