pub mod html;
pub mod markdown;

use crate::document::Document;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
	Text,
	Html,
	Markdown,
}

#[must_use]
pub fn render(doc: &Document, format: ExportFormat) -> String {
	match format {
		ExportFormat::Text => doc.buffer.content.clone(),
		ExportFormat::Html => html::render(doc),
		ExportFormat::Markdown => markdown::render(doc),
	}
}
