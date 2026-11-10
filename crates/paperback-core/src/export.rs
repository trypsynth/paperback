pub mod annotations;
pub mod html;
pub mod markdown;

use std::borrow::Cow;

use crate::document::{DocLen, DocPos, DocumentHandle, Marker};

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
	Text,
	Html,
	Markdown,
}

#[must_use]
pub fn render(doc: &DocumentHandle, format: ExportFormat) -> Cow<'_, str> {
	match format {
		ExportFormat::Text => Cow::Borrowed(&doc.document().buffer.content),
		ExportFormat::Html => Cow::Owned(html::render(doc)),
		ExportFormat::Markdown => Cow::Owned(markdown::render(doc.document())),
	}
}

// `Chars` skips in chunks, so reaching the next tag this way costs far less than visiting every char on the way.
fn split_after_chars(text: &str, count: usize) -> (&str, &str) {
	let mut chars = text.chars();
	if count > 0 {
		chars.nth(count - 1);
	}
	text.split_at(text.len() - chars.as_str().len())
}

// Link markers store no length, so it is inferred from the text with whitespace runs collapsed, as the parser wrote it into the content.
fn link_span_end(marker: &Marker) -> Option<DocPos> {
	let text: String = marker.text.split_whitespace().collect::<Vec<_>>().join(" ");
	let implied_len = if marker.length.is_zero() { DocLen::of(&text) } else { marker.length };
	if implied_len.is_zero() { None } else { Some(marker.position + implied_len) }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker, MarkerType};

	fn sample_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content("Chapter One\nBody text.".to_string());
		buffer
			.add_marker(Marker::new(MarkerType::Heading1, 0.into()).with_level(1).with_text("Chapter One".to_string()));
		let mut doc = Document::new().with_title("Sample".to_string());
		doc.set_buffer(buffer);
		DocumentHandle::new(doc)
	}

	#[test]
	fn split_after_chars_counts_chars_and_stops_at_the_end() {
		assert_eq!(split_after_chars("éa€b", 3), ("éa€", "b"));
		assert_eq!(split_after_chars("éa", 2), ("éa", ""));
		assert_eq!(split_after_chars("éa", 9), ("éa", ""));
		assert_eq!(split_after_chars("éa", 0), ("", "éa"));
	}

	#[test]
	fn text_export_is_the_buffer_verbatim() {
		let handle = sample_handle();
		assert_eq!(render(&handle, ExportFormat::Text), "Chapter One\nBody text.");
	}

	#[test]
	fn html_export_emits_markup() {
		let handle = sample_handle();
		let rendered = render(&handle, ExportFormat::Html);
		assert!(rendered.contains('<'), "expected html markup: {rendered:?}");
		assert!(rendered.contains("Chapter One"));
	}

	#[test]
	fn markdown_export_marks_up_the_heading() {
		let handle = sample_handle();
		let rendered = render(&handle, ExportFormat::Markdown);
		assert!(rendered.contains("# Chapter One"), "expected a markdown heading: {rendered:?}");
	}

	#[test]
	fn the_three_formats_render_differently() {
		let handle = sample_handle();
		let text = render(&handle, ExportFormat::Text);
		let html = render(&handle, ExportFormat::Html);
		let markdown = render(&handle, ExportFormat::Markdown);
		assert_ne!(text, html);
		assert_ne!(text, markdown);
		assert_ne!(html, markdown);
	}
}
