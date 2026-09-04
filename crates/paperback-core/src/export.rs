pub mod epub_direct;
pub mod html;
pub mod markdown;

use crate::{
	document::{DocumentHandle, Marker},
	util::text::display_len,
};

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

/// Recovers the end position of a Link marker's implied span. Link markers store no
/// explicit length; both the HTML and Markdown renderers infer it from the display width
/// of `marker.text` after collapsing embedded whitespace runs to single spaces, matching
/// how the text was originally written into the content when the link was parsed. Returns
/// `None` when the recovered span is empty, in which case the marker has nothing to link
/// and the caller should skip it entirely.
fn link_span_end(marker: &Marker) -> Option<usize> {
	let text: String = marker.text.split_whitespace().collect::<Vec<_>>().join(" ");
	let implied_len = if marker.length > 0 { marker.length } else { display_len(&text) };
	if implied_len == 0 { None } else { Some(marker.position + implied_len) }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker, MarkerType};

	fn sample_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content("Chapter One\nBody text.".to_string());
		buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("Chapter One".to_string()));
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

	/// Each variant has to reach a different renderer; if two ever aliased, an export menu entry
	/// would silently produce the wrong format.
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
