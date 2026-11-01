pub mod annotations;
pub mod epub_direct;
pub mod html;
pub mod markdown;

use crate::document::{DocLen, DocPos, DocumentHandle, Marker};

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
	Text,
	Html,
	Markdown,
}

#[must_use]
pub fn render(doc: &DocumentHandle, format: ExportFormat) -> String {
	match format {
		ExportFormat::Text => doc.document().buffer.content.clone(),
		ExportFormat::Html => html::render(doc),
		ExportFormat::Markdown => markdown::render(doc.document()),
	}
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
	fn text_export_is_the_buffer_verbatim() {
		let handle = sample_handle();
		assert_eq!(render(&handle, ExportFormat::Text), "Chapter One\nBody text.");
	}

	#[test]
	fn html_export_emits_markup() {
		let rendered = render(&sample_handle(), ExportFormat::Html);
		assert!(rendered.contains('<'), "expected html markup: {rendered:?}");
		assert!(rendered.contains("Chapter One"));
	}

	#[test]
	fn markdown_export_marks_up_the_heading() {
		let rendered = render(&sample_handle(), ExportFormat::Markdown);
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
