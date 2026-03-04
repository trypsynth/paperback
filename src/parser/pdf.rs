use std::collections::HashMap;

use anyhow::{Result, anyhow};
use pdfium::{PdfiumDocument, PdfiumError, lib};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, Parser, path::extract_title_from_path},
	text::{collapse_whitespace, display_len, trim_string},
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
		let mut id_positions = HashMap::new();
		let page_count = document.page_count();
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{}", page_index), marker_position);
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

			let page_start_offset = buffer.current_position();
			let mut page_display_text = String::new();

			for line in lines {
				buffer.append(&line);
				buffer.append("\n");
				page_display_text.push_str(&line);
				page_display_text.push('\n');
			}

			// Load implicit web links
			if let Ok(links) = text_page.load_web_links() {
				let count = lib().FPDFLink_CountWebLinks(&links);
				let mut last_search_pos = 0;
				for i in 0..count {
					let mut start = 0;
					let mut char_count = 0;
					if lib().FPDFLink_GetTextRange(&links, i, &mut start, &mut char_count).is_ok() {
						let link_text = text_page.extract(start, char_count);
						let trimmed_link = trim_string(&collapse_whitespace(&link_text));
						if trimmed_link.is_empty() {
							continue;
						}

						let mut url_buffer = vec![0u16; 2048];
						let len = lib().FPDFLink_GetURL(&links, i, &mut url_buffer[0], 2048);
						if len > 0 {
							let url = String::from_utf16_lossy(&url_buffer[..(len as usize - 1)]);
							if let Some(pos) = page_display_text[last_search_pos..].find(&trimmed_link) {
								let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
								let marker_pos = page_start_offset
									+ display_len(&page_display_text[..last_search_pos])
									+ display_len(text_before);
								let link_len = display_len(&trimmed_link);
								buffer.add_marker(
									Marker::new(MarkerType::Link, marker_pos)
										.with_text(trimmed_link.clone())
										.with_reference(url)
										.with_length(link_len),
								);
								last_search_pos += pos + trimmed_link.len();
							}
						}
					}
				}
			}

			// Load explicit annotations (internal and external links)
			let annot_count = lib().FPDFPage_GetAnnotCount(&page);
			let mut last_search_pos = 0;
			for i in 0..annot_count {
				if let Ok(annot) = lib().FPDFPage_GetAnnot(&page, i) {
					if lib().FPDFAnnot_GetSubtype(&annot) == pdfium::pdfium_constants::FPDF_ANNOT_LINK {
						let mut rect = pdfium::pdfium_types::FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
						if lib().FPDFAnnot_GetRect(&annot, &mut rect).is_ok() {
							let mut text_buffer = vec![0u16; 2048];
							let len = lib().FPDFText_GetBoundedText(
								&text_page,
								rect.left as f64,
								rect.top as f64,
								rect.right as f64,
								rect.bottom as f64,
								&mut text_buffer[0],
								2048,
							);
							if len > 0 {
								let text = String::from_utf16_lossy(&text_buffer[..(len as usize - 1)]);
								let trimmed_link = trim_string(&collapse_whitespace(&text));
								if trimmed_link.is_empty() {
									continue;
								}

								let mut url = String::new();
								if let Ok(link) = lib().FPDFAnnot_GetLink(&annot) {
									if let Ok(action) = lib().FPDFLink_GetAction(&link) {
										let action_type = lib().FPDFAction_GetType(&action);
										// PDFACTION_URI is 3
										if action_type == 3 {
											let mut uri_buffer = vec![0u8; 2048];
											let uri_len = lib().FPDFAction_GetURIPath(
												&document,
												&action,
												Some(&mut uri_buffer),
												2048,
											);
											if uri_len > 0 {
												url = String::from_utf8_lossy(&uri_buffer[..(uri_len as usize - 1)])
													.to_string();
											}
										}
									}
									if url.is_empty() {
										let dest_result = lib().FPDFLink_GetDest(&document, &link);
										let dest = dest_result.ok().or_else(|| {
											lib()
												.FPDFLink_GetAction(&link)
												.ok()
												.and_then(|action| lib().FPDFAction_GetDest(&document, &action).ok())
										});
										if let Some(dest) = dest {
											let dest_page = lib().FPDFDest_GetDestPageIndex(&document, &dest);
											if dest_page >= 0 {
												url = format!("#page_{}", dest_page);
											}
										}
									}
								}

								if !url.is_empty() {
									if let Some(pos) = page_display_text[last_search_pos..].find(&trimmed_link) {
										let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
										let marker_pos = page_start_offset
											+ display_len(&page_display_text[..last_search_pos])
											+ display_len(text_before);
										let link_len = display_len(&trimmed_link);
										buffer.add_marker(
											Marker::new(MarkerType::Link, marker_pos)
												.with_text(trimmed_link.clone())
												.with_reference(url)
												.with_length(link_len),
										);
										last_search_pos += pos + trimmed_link.len();
									}
								}
							}
						}
					}
				}
			}
		}

		let title = metadata_value(&document, "Title").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = metadata_value(&document, "Author").unwrap_or_default();
		let toc_items = extract_toc(&document, &page_offsets);

		add_heading_markers(&mut buffer, &toc_items, 1);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.title = title;
		doc.author = author;
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}

fn add_heading_markers(buffer: &mut DocumentBuffer, items: &[TocItem], level: i32) {
	for item in items {
		let marker_type = match level {
			1 => MarkerType::Heading1,
			2 => MarkerType::Heading2,
			3 => MarkerType::Heading3,
			4 => MarkerType::Heading4,
			5 => MarkerType::Heading5,
			_ => MarkerType::Heading6,
		};
		buffer.add_marker(Marker::new(marker_type, item.offset).with_text(item.name.clone()).with_level(level));
		add_heading_markers(buffer, &item.children, level + 1);
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
