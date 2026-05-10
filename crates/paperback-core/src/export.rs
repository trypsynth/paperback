pub mod html;
pub mod markdown;

use crate::document::DocumentHandle;

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
