use anyhow::Result;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{Parser, utils::extract_title_from_path},
	pdfium::{PdfDocument, PdfiumLibrary},
	utils::text::{collapse_whitespace, trim_string},
};

pub struct PdfParser;

impl Parser for PdfParser {
	fn name(&self) -> &'static str {
		"PDF Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["pdf"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_PAGES | ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let _library = PdfiumLibrary::new();
		let document = PdfDocument::load(&context.file_path, context.password.as_deref())?;
		let mut buffer = DocumentBuffer::new();
		let mut page_offsets = Vec::new();
		let page_count = document.page_count()?;
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			page_offsets.push(marker_position);
			let Some(page) = document.load_page(page_index) else { continue };
			if let Some(text_page) = page.load_text_page() {
				let raw_text = text_page.extract_text();
				let lines = process_text_lines(&raw_text);
				for line in lines {
					buffer.append(&line);
					buffer.append("\n");
				}
			}
		}
		let title =
			document.extract_metadata(b"Title\0").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = document.extract_metadata(b"Author\0").unwrap_or_default();
		let toc_items = document.extract_toc(&page_offsets);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.title = title;
		doc.author = author;
		doc.toc_items = toc_items;
		Ok(doc)
	}
}

fn process_text_lines(raw_text: &str) -> Vec<String> {
	raw_text
		.lines()
		.filter_map(|line| {
			let collapsed = collapse_whitespace(line);
			let trimmed = trim_string(&collapsed);
			if trimmed.is_empty() { None } else { Some(trimmed) }
		})
		.collect()
}
