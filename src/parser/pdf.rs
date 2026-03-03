use anyhow::{Result, anyhow};
use pdfium::{PdfiumDocument, PdfiumError};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, Parser, path::extract_title_from_path},
	text::{collapse_whitespace, trim_string},
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
		let document =
			PdfiumDocument::new_from_path(&context.file_path, context.password.as_deref()).map_err(map_load_error)?;
		let mut buffer = DocumentBuffer::new();
		let mut page_offsets = Vec::new();
		let page_count = document.page_count();
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let Ok(page) = document.page(page_index) else {
				continue;
			};
			let Ok(text_page) = page.text() else {
				continue;
			};
			let raw_text = text_page.full();
			let lines = process_text_lines(&raw_text);
			for line in lines {
				buffer.append(&line);
				buffer.append("\n");
			}
		}

		let title = metadata_value(&document, "Title").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = metadata_value(&document, "Author").unwrap_or_default();
		let toc_items = extract_toc(&document, &page_offsets);

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

fn map_load_error(err: PdfiumError) -> anyhow::Error {
	match err {
		PdfiumError::PasswordError => anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect"),
		other => anyhow!("Failed to open PDF document: {other}"),
	}
}

fn metadata_value(document: &PdfiumDocument, key: &str) -> Option<String> {
	document.metadata_value(key).ok().map(|value| trim_string(&value)).filter(|value| !value.is_empty())
}

fn extract_toc(document: &PdfiumDocument, page_offsets: &[usize]) -> Vec<TocItem> {
	let Ok(bookmarks) = document.toc(16) else {
		return Vec::new();
	};
	if bookmarks.is_empty() {
		return Vec::new();
	}

	let mut items = Vec::<(u32, TocItem)>::new();
	for bookmark in &bookmarks {
		let Some(level) = bookmark.level() else {
			continue;
		};
		let Ok(raw_title) = bookmark.title() else {
			continue;
		};
		let title = trim_string(&collapse_whitespace(&raw_title));
		if title.is_empty() {
			continue;
		}
		let Ok(dest) = bookmark.dest(document) else {
			continue;
		};
		let Some(page_index) = dest.index(document) else {
			continue;
		};
		let Ok(page_index) = usize::try_from(page_index) else {
			continue;
		};
		let Some(&offset) = page_offsets.get(page_index) else {
			continue;
		};
		items.push((level, TocItem::new(title, String::new(), offset)));
	}

	build_toc_tree(items)
}

fn build_toc_tree(flat_items: Vec<(u32, TocItem)>) -> Vec<TocItem> {
	let mut root = Vec::<TocItem>::new();
	let mut path = Vec::<usize>::new();
	let mut level_stack = Vec::<u32>::new();

	for (level, item) in flat_items {
		while let Some(&last_level) = level_stack.last() {
			if last_level < level {
				break;
			}
			level_stack.pop();
			path.pop();
		}

		let siblings = children_at_path_mut(&mut root, &path);
		siblings.push(item);
		path.push(siblings.len() - 1);
		level_stack.push(level);
	}

	root
}

fn children_at_path_mut<'a>(nodes: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = nodes;
	for &index in path {
		current = &mut current[index].children;
	}
	current
}
