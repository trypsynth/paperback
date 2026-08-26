use std::collections::HashMap;

use anyhow::Result;
use pdfium::{PdfiumDocument, lib};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

mod links;
mod metadata;
mod structure;
mod text;
mod toc;

use links::{extract_annotation_links, extract_web_links};
use metadata::{map_load_error, metadata_value};
use structure::extract_tagged_page_text;
use text::{extract_text_lines, join_paragraphs, median_line_font_size};
use toc::{add_heading_markers, build_toc_tree, extract_toc};

pub struct PdfParser;

impl Parser for PdfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing pdf document");
		let render_tables_inline = context.render_tables_inline;
		let document =
			PdfiumDocument::new_from_path(&context.file_path, context.password.as_deref()).map_err(|err| {
				let mapped = map_load_error(err);
				tracing::warn!(path = %context.file_path, error = %mapped, "failed to load pdf document");
				mapped
			})?;
		let mut buffer = DocumentBuffer::new();
		let mut page_offsets = Vec::new();
		let mut id_positions = HashMap::new();
		let mut page_lines_info: Vec<Vec<(usize, String)>> = Vec::new();
		let page_count = document.page_count();
		let mut any_tags_processed = false;
		let mut flat_toc_items = Vec::new();
		let mut has_any_text = false;
		let mut has_any_images = false;
		let mut detected_heading_positions: Vec<(usize, String)> = Vec::new();
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{page_index}"), marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let Ok(page) = document.page(page_index) else {
				tracing::warn!(page_index, "failed to load pdf page, skipping its text");
				page_lines_info.push(Vec::new());
				continue;
			};
			let Ok(text_page) = page.text() else {
				tracing::warn!(page_index, "failed to load text for pdf page, skipping its text");
				page_lines_info.push(Vec::new());
				continue;
			};
			let page_start_offset = buffer.current_position();
			let mut page_display_text = String::new();
			let mut current_lines_info = Vec::new();
			let tags_processed = extract_tagged_page_text(
				&page,
				&text_page,
				page_index,
				&mut buffer,
				&mut page_display_text,
				&mut current_lines_info,
				&mut flat_toc_items,
				render_tables_inline,
			);
			if tags_processed {
				any_tags_processed = true;
				has_any_text = true;
			} else {
				let line_infos = extract_text_lines(&text_page, page_index);
				let body_size = median_line_font_size(&line_infos);
				let paragraphs = join_paragraphs(&line_infos, body_size);
				if !paragraphs.is_empty() {
					has_any_text = true;
				}
				for (text, is_heading) in &paragraphs {
					let current_offset = buffer.current_position();
					if *is_heading {
						detected_heading_positions.push((current_offset, text.clone()));
					}
					current_lines_info.push((current_offset, text.clone()));
					buffer.append(text);
					buffer.append("\n");
					page_display_text.push_str(text);
					page_display_text.push('\n');
				}
			}
			// Check for image objects on this page
			if !has_any_images {
				let obj_count = lib().FPDFPage_CountObjects(&page);
				for i in 0..obj_count {
					if let Ok(obj) = lib().FPDFPage_GetObject(&page, i)
						&& lib().FPDFPageObj_GetType(&obj) == pdfium::pdfium_constants::FPDF_PAGEOBJ_IMAGE
					{
						has_any_images = true;
						break;
					}
				}
			}
			extract_web_links(&text_page, page_start_offset, &page_display_text, &mut buffer);
			extract_annotation_links(&page, &text_page, &document, page_start_offset, &page_display_text, &mut buffer);
			page_lines_info.push(current_lines_info);
		}
		if !has_any_text && has_any_images {
			tracing::warn!(path = %context.file_path, "pdf has images but no extractable text, likely needs ocr");
			let marker_position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, marker_position).with_text(String::new()));
			// TRANSLATORS: Notice inserted into the extracted text when a PDF has images but no text layer at all
			buffer.append(&t("This PDF contains images only, with no extractable text. You may need to run it through OCR software to read its contents."));
			buffer.append("\n");
		}
		let title = metadata_value(&document, "Title").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = metadata_value(&document, "Author").unwrap_or_default();
		let mut toc_items = extract_toc(&document, &page_offsets, &page_lines_info);
		let toc_source = if !toc_items.is_empty() {
			"bookmarks"
		} else if any_tags_processed {
			if flat_toc_items.is_empty() { "none" } else { "structure tree" }
		} else if !detected_heading_positions.is_empty() {
			"font-size detected headings"
		} else {
			"none"
		};
		if any_tags_processed {
			if toc_items.is_empty() {
				toc_items = build_toc_tree(flat_toc_items);
			} else if flat_toc_items.is_empty() {
				add_heading_markers(&mut buffer, &toc_items, 1);
			}
		} else if toc_items.is_empty() && !detected_heading_positions.is_empty() {
			for (pos, text) in &detected_heading_positions {
				buffer.add_marker(Marker::new(MarkerType::Heading1, *pos).with_text(text.clone()).with_level(1));
			}
			toc_items = detected_heading_positions
				.into_iter()
				.map(|(pos, text)| TocItem::new(text, String::new(), pos))
				.collect();
		} else {
			add_heading_markers(&mut buffer, &toc_items, 1);
		}
		tracing::debug!(toc_source, toc_item_count = toc_items.len(), "resolved pdf toc source");
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.title = title;
		doc.author = author;
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		tracing::debug!(
			path = %context.file_path,
			page_count,
			tagged_extraction_used = any_tags_processed,
			images_only = (!has_any_text && has_any_images),
			toc_source,
			"finished parsing pdf document"
		);
		Ok(doc)
	}
}
