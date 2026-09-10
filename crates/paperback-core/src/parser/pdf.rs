use std::collections::HashMap;

use anyhow::Result;
use pdfium::PdfiumDocument;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	ocr::image_only_placeholder,
	parser::{Parser, util::path::extract_title_from_path},
};

mod images;
mod links;
mod metadata;
mod running;
mod structure;
mod text;
mod toc;

use images::{append_images, images_before_each_paragraph, page_image_tops};
use links::{PendingLink, collect_annotation_links, collect_web_links, place_links};
use metadata::{map_load_error, metadata_value};
use running::{EDGE_LINES, PageEdges, RunningText};
use structure::extract_tagged_page_text;
use text::{extract_text_lines, join_paragraphs, median_line_font_size};
use toc::{add_heading_markers, build_toc_tree, extract_toc};

/// Everything one page contributes, as read from pdfium and before any of it is placed in the
/// document. The whole document is read before the first page is assembled, because which lines
/// are running headers is only known once every page's edges have been seen - and pdfium is
/// asked for each page exactly once, since loading a page and its text is the bulk of what
/// parsing a PDF costs.
#[derive(Default)]
struct PageContent {
	/// A page with a trustworthy structure tree arrives already laid out, in a buffer of its
	/// own whose positions are relative to the page.
	tagged: Option<TaggedPage>,
	/// A page without one arrives as visual lines, still to have its running headers taken out
	/// and its wrapped lines joined into paragraphs. Each line carries its font size and the top
	/// edge it was set at.
	lines: Vec<(String, f64, f64)>,
	/// Kept apart because each kind is placed by its own walk through the page's text: a link
	/// annotation may sit before a bare URL that pdfium's scanner reported first.
	web_links: Vec<PendingLink>,
	annotation_links: Vec<PendingLink>,
	/// The top edge of each image the page draws, ordered down the page.
	image_tops: Vec<f64>,
}

/// A page laid out by [`extract_tagged_page_text`], in its own buffer.
struct TaggedPage {
	buffer: DocumentBuffer,
	display_text: String,
	lines_info: Vec<(usize, String)>,
	toc_items: Vec<(u32, TocItem)>,
}

/// Read one page from pdfium: its text, its links, and the images it draws.
fn read_page(document: &PdfiumDocument, page_index: i32, render_tables_inline: bool) -> PageContent {
	let Ok(page) = document.page(page_index) else {
		tracing::warn!(page_index, "failed to load pdf page, skipping its text");
		return PageContent::default();
	};
	let Ok(text_page) = page.text() else {
		tracing::warn!(page_index, "failed to load text for pdf page, skipping its text");
		return PageContent::default();
	};
	let mut content = PageContent::default();
	let mut tagged = TaggedPage {
		buffer: DocumentBuffer::new(),
		display_text: String::new(),
		lines_info: Vec::new(),
		toc_items: Vec::new(),
	};
	if extract_tagged_page_text(
		&page,
		&text_page,
		page_index,
		&mut tagged.buffer,
		&mut tagged.display_text,
		&mut tagged.lines_info,
		&mut tagged.toc_items,
		render_tables_inline,
	) {
		content.tagged = Some(tagged);
	} else {
		content.lines = extract_text_lines(&text_page, page_index);
	}
	content.web_links = collect_web_links(&text_page);
	content.annotation_links = collect_annotation_links(&page, &text_page, document);
	// Every page is scanned, rather than the document stopping at the first image it finds,
	// because each page places its own images and because an image-only page still needs its OCR
	// placeholder when earlier pages contributed text.
	content.image_tops = page_image_tops(&page);
	content
}

/// Splice a tagged page's own buffer into the document, moving everything it holds from
/// page-relative positions to where the page landed.
fn append_tagged_page(buffer: &mut DocumentBuffer, page: DocumentBuffer, page_start_offset: usize) {
	buffer.append(&page.content);
	for mut marker in page.markers {
		marker.position += page_start_offset;
		buffer.add_marker(marker);
	}
}

/// Drop the running headers and footers from one page's lines. Only lines within
/// [`EDGE_LINES`] of either edge are considered, so a body line that happens to read like a
/// running head is never dropped, and a page whose every line matches (a part title repeating
/// the book's name, say) keeps them: a page is never emptied by this.
fn strip_running_text(lines: &mut Vec<(String, f64, f64)>, running_text: &RunningText) {
	let is_running = |index: usize, lines: &Vec<(String, f64, f64)>| {
		let at_an_edge = index < EDGE_LINES || index + EDGE_LINES >= lines.len();
		at_an_edge && running_text.contains(&lines[index].0, lines[index].1)
	};
	let doomed: Vec<usize> = (0..lines.len()).filter(|index| is_running(*index, lines)).collect();
	if doomed.len() == lines.iter().filter(|(line, ..)| !line.trim().is_empty()).count() {
		return;
	}
	for index in doomed.into_iter().rev() {
		lines.remove(index);
	}
}

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
		let pages: Vec<PageContent> =
			(0..page_count).map(|page_index| read_page(&document, page_index, render_tables_inline)).collect();
		// Only an untagged page needs this: a tagged one's structure tree marks its running
		// headers as artifacts and never hands them over as text in the first place.
		let untagged = || pages.iter().filter(|page| page.tagged.is_none());
		let edges: Vec<PageEdges> = untagged().map(|page| PageEdges::of(&page.lines)).collect();
		// The body size is taken over the whole document rather than one page: a chapter opening
		// is mostly heading, and the survey has to know a heading from a running head.
		let body_font_size = median_line_font_size(&untagged().flat_map(|page| page.lines.clone()).collect::<Vec<_>>());
		let running_text = running::detect(&edges, body_font_size);
		tracing::debug!(
			running_text_count = running_text.len(),
			body_font_size,
			"surveyed pages for running headers and footers"
		);
		for (page_index, page) in pages.into_iter().enumerate() {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{page_index}"), marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let page_start_offset = buffer.current_position();
			let mut page_display_text = String::new();
			let mut current_lines_info = Vec::new();
			if let Some(tagged) = page.tagged {
				any_tags_processed = true;
				has_any_text = true;
				append_tagged_page(&mut buffer, tagged.buffer, page_start_offset);
				page_display_text = tagged.display_text;
				current_lines_info =
					tagged.lines_info.into_iter().map(|(pos, text)| (pos + page_start_offset, text)).collect();
				flat_toc_items.extend(tagged.toc_items.into_iter().map(|(level, mut item)| {
					item.offset += page_start_offset;
					(level, item)
				}));
			} else {
				let mut line_infos = page.lines;
				// Measured before the running headers come out. They are set at or below the
				// body's size, so dropping them pulls the median up, and a heading only just
				// above it would stop counting as one.
				let body_size = median_line_font_size(&line_infos);
				strip_running_text(&mut line_infos, &running_text);
				let line_tops: Vec<f64> = line_infos.iter().map(|(.., top)| *top).collect();
				let lines: Vec<(String, f64)> = line_infos.into_iter().map(|(text, size, _)| (text, size)).collect();
				let paragraphs = join_paragraphs(&lines, body_size);
				if !paragraphs.is_empty() {
					has_any_text = true;
				}
				// An untagged page says nothing about where its images belong in its text, so each
				// one is placed by the height it was drawn at. A page with no text at all is left
				// alone: the image-only placeholder below is more use on a scanned page than a
				// line for each of the pieces it was scanned into.
				let paragraph_tops: Vec<f64> = paragraphs
					.iter()
					.map(|(.., line_index)| line_tops.get(*line_index).copied().unwrap_or(f64::NEG_INFINITY))
					.collect();
				let image_counts = images_before_each_paragraph(&page.image_tops, &paragraph_tops);
				for (index, (text, is_heading, _)) in paragraphs.iter().enumerate() {
					append_images(image_counts[index], &mut buffer, &mut page_display_text, &mut current_lines_info);
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
				if !paragraphs.is_empty() {
					let trailing = image_counts[paragraphs.len()];
					append_images(trailing, &mut buffer, &mut page_display_text, &mut current_lines_info);
				}
			}
			let page_has_image = !page.image_tops.is_empty();
			if page_has_image {
				has_any_images = true;
			}
			// A page with an image but no extractable text gets a placeholder the user can OCR
			// from (Enter on it replaces it with the recognized text). The marker, not the text, is
			// what the OCR flow matches on, so changing the UI language cannot strand a placeholder.
			let page_has_text = buffer.current_position() > page_start_offset;
			if !page_has_text && page_has_image {
				let placeholder_position = buffer.current_position();
				buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, placeholder_position));
				buffer.append(&image_only_placeholder());
				buffer.append("\n");
			}
			place_links(&page.web_links, page_start_offset, &page_display_text, &mut buffer);
			place_links(&page.annotation_links, page_start_offset, &page_display_text, &mut buffer);
			page_lines_info.push(current_lines_info);
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
