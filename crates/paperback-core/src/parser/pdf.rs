use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use pdfium::PdfiumDocument;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	ocr::image_only_placeholder,
	parser::{Parser, add_heading_markers, util::path::extract_title_from_path},
};

mod images;
mod links;
mod metadata;
mod paragraphs;
mod repair;
mod running;
mod structure;
mod tagged;
mod text;
mod toc;

use images::{append_images, images_before_each_paragraph, page_image_tops, page_largest_image_coverage};
use links::{PendingLink, collect_annotation_links, collect_web_links, place_links};
use metadata::{map_load_error, metadata_value};
pub use paragraphs::join_wrapped_lines;
use paragraphs::{join_paragraphs, split_lines};
use running::{EDGE_LINES, PageEdges, RunningText};
use structure::extract_tagged_page_text;
use tagged::{TaggedPage, append_tagged_page, tagged_running_lines, without_lines};
use text::{Line, extract_text_lines, median_line_font_size};
use toc::{build_toc_tree, extract_toc};

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
	lines: Vec<Line>,
	/// Kept apart because each kind is placed by its own walk through the page's text: a link
	/// annotation may sit before a bare URL that pdfium's scanner reported first.
	web_links: Vec<PendingLink>,
	annotation_links: Vec<PendingLink>,
	/// The top edge of each image the page draws, ordered down the page.
	image_tops: Vec<f64>,
	/// The fraction of the page the largest single image covers. Near 1.0 marks a scanned page.
	largest_image_coverage: f64,
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
	// Every page is scanned, rather than the document stopping at the first image it finds,
	// because each page places its own images and because an image-only page still needs its OCR
	// placeholder when earlier pages contributed text. Taken before the text so that a tagged
	// page whose tree names no figure can place them among its blocks as it writes them.
	let mut content = PageContent {
		image_tops: page_image_tops(&page),
		largest_image_coverage: page_largest_image_coverage(&page),
		..Default::default()
	};
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
		&content.image_tops,
	) {
		content.tagged = Some(tagged);
	} else {
		content.lines = extract_text_lines(&text_page, page_index);
	}
	content.web_links = collect_web_links(&text_page);
	content.annotation_links = collect_annotation_links(&page, &text_page, document);
	content
}

/// How much of a page one image must cover before the page is a candidate to be a scan rather
/// than a page of text. A scanned sheet is a single image laid over the whole page; half is well
/// clear of a figure sitting among paragraphs.
const SCAN_IMAGE_COVERAGE: f64 = 0.5;

/// The most text a scanned page's page number can run to. A real page of words runs far past
/// this; a page number, even dressed up as "- 42 -", does not.
const MAX_FURNITURE_CHARS: usize = 12;

/// Whether a page is a scanned image carrying nothing but a page number, which is a scan to be
/// offered for OCR rather than a page of text.
///
/// The two halves are both needed. A page can carry a page number and still be a page of text, so
/// a large image is required; a page can be one big image and still be a figure with a caption, so
/// the text is required to be nothing more than a page number. See [`looks_like_page_number`].
fn is_scanned_with_only_furniture(page: &PageContent) -> bool {
	if page.largest_image_coverage < SCAN_IMAGE_COVERAGE {
		return false;
	}
	let text = match &page.tagged {
		Some(tagged) => tagged.display_text.clone(),
		None => page.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" "),
	};
	looks_like_page_number(&text)
}

/// Whether a piece of text is nothing but a page number: short, and made only of digits, spaces
/// and punctuation. Letters rule it out, so a caption or a heading is never taken for one, which
/// is what keeps a figure-with-caption page from being mistaken for a scan.
fn looks_like_page_number(text: &str) -> bool {
	let trimmed = text.split_whitespace().collect::<Vec<_>>().join(" ");
	if trimmed.is_empty() || trimmed.chars().count() > MAX_FURNITURE_CHARS {
		return false;
	}
	let mut has_digit = false;
	for ch in trimmed.chars() {
		if ch.is_ascii_digit() {
			has_digit = true;
		} else if !ch.is_whitespace() && !ch.is_ascii_punctuation() {
			return false;
		}
	}
	has_digit
}

/// One line an untagged page contributes, in the order the reader meets it.
enum UntaggedLine {
	/// A paragraph, and whether the size it was set in marked it out as a heading.
	Text(String, bool),
	/// An image the page drew here. An untagged page has no description to give for one.
	Image,
}

/// Work out what an untagged page contributes: its running headers dropped, its wrapped lines
/// joined into paragraphs, and the images it draws placed among those by the height each was
/// drawn at.
///
/// Kept apart from writing the lines out so that both kinds of page reach the document the same
/// way, as finished lines: a tagged one from [`structure`], an untagged one from here.
fn untagged_page_lines(page: &PageContent, running_text: &RunningText, join: bool) -> Vec<UntaggedLine> {
	let mut line_infos = page.lines.clone();
	// Measured before the running headers come out. They are set at or below the body's size, so
	// dropping them pulls the median up, and a heading only just above it would stop counting as
	// one.
	let body_size = median_line_font_size(&line_infos);
	strip_running_text(&mut line_infos, running_text);
	let line_tops: Vec<f64> = line_infos.iter().map(|line| line.top).collect();
	let paragraphs = if join { join_paragraphs(&line_infos, body_size) } else { split_lines(&line_infos, body_size) };
	// A page with no text at all is left alone: the image-only placeholder the caller writes is
	// more use on a scanned page than a line for each of the pieces it was scanned into.
	if paragraphs.is_empty() {
		return Vec::new();
	}
	// An untagged page says nothing about where its images belong in its text, so each one is
	// placed by the height it was drawn at.
	let paragraph_tops: Vec<f64> = paragraphs
		.iter()
		.map(|(.., line_index)| line_tops.get(*line_index).copied().unwrap_or(f64::NEG_INFINITY))
		.collect();
	let image_counts = images_before_each_paragraph(&page.image_tops, &paragraph_tops);
	let mut lines = Vec::with_capacity(paragraphs.len());
	for (index, (text, is_heading, _)) in paragraphs.iter().enumerate() {
		lines.extend((0..image_counts[index]).map(|_| UntaggedLine::Image));
		lines.push(UntaggedLine::Text(text.clone(), *is_heading));
	}
	lines.extend((0..image_counts[paragraphs.len()]).map(|_| UntaggedLine::Image));
	lines
}

/// Survey every page for the lines it repeats as furniture, take those lines out of the pages
/// that arrived finished, and hand back what the rest of the document is measured against.
///
/// The two kinds of page are surveyed apart because they are counted differently: an untagged
/// page offers visual lines and is judged with the body's font size to hand, a tagged page offers
/// whole blocks and is judged by how much of the document repeats them. See [`running`].
fn strip_tagged_running_text(pages: &mut [PageContent]) -> RunningText {
	let untagged = || pages.iter().filter(|page| page.tagged.is_none());
	let edges: Vec<PageEdges> = untagged().map(|page| PageEdges::of(&page.lines)).collect();
	// The body size is taken over the whole document rather than one page: a chapter opening is
	// mostly heading, and the survey has to know a heading from a running head.
	let body_font_size = median_line_font_size(&untagged().flat_map(|page| page.lines.clone()).collect::<Vec<_>>());
	let running_text = running::detect(&edges, body_font_size);
	tracing::debug!(
		running_text_count = running_text.len(),
		body_font_size,
		"surveyed pages for running headers and footers"
	);
	// A tagged page gets a survey of its own. Most tagged PDFs mark their running headers as
	// artifacts and never hand them over as text at all, but a maker that tags them as ordinary
	// paragraphs would otherwise repeat the book's title and the page number between two
	// paragraphs on every page of it.
	let tagged_edges: Vec<PageEdges> = pages
		.iter()
		.filter_map(|page| page.tagged.as_ref())
		.map(|tagged| {
			let lines: Vec<String> = tagged.lines_info.iter().map(|(_, text)| text.clone()).collect();
			PageEdges::of_texts(&lines)
		})
		.collect();
	let tagged_running = running::detect_tagged(&tagged_edges);
	tracing::debug!(
		tagged_running_text_count = tagged_running.len(),
		tagged_page_count = tagged_edges.len(),
		"surveyed tagged pages for running headers and footers"
	);
	for page in pages.iter_mut() {
		if let Some(tagged) = page.tagged.take() {
			let doomed = tagged_running_lines(&tagged, &tagged_running);
			page.tagged = Some(without_lines(tagged, &doomed));
		}
	}
	running_text
}

/// Drop the running headers and footers from one page's lines. Only lines within
/// [`EDGE_LINES`] of either edge are considered, so a body line that happens to read like a
/// running head is never dropped, and a page whose every line matches (a part title repeating
/// the book's name, say) keeps them: a page is never emptied by this.
fn strip_running_text(lines: &mut Vec<Line>, running_text: &RunningText) {
	let is_running = |index: usize, lines: &Vec<Line>| {
		let at_an_edge = index < EDGE_LINES || index + EDGE_LINES >= lines.len();
		at_an_edge && running_text.contains(&lines[index].text, lines[index].size)
	};
	let doomed: Vec<usize> = (0..lines.len()).filter(|index| is_running(*index, lines)).collect();
	if doomed.len() == lines.iter().filter(|line| !line.text.trim().is_empty()).count() {
		return;
	}
	for index in doomed.into_iter().rev() {
		lines.remove(index);
	}
}

/// Opens the document, and opens it a second time over a repaired copy when the first one hands
/// back a structure tree nothing can be loaded from. See [`repair`] for what is repaired and why.
fn load_document(context: &ParserContext) -> Result<PdfiumDocument> {
	let document = PdfiumDocument::new_from_path(&context.file_path, context.password.as_deref()).map_err(|err| {
		let mapped = map_load_error(err);
		tracing::warn!(path = %context.file_path, error = %mapped, "failed to load pdf document");
		mapped
	})?;
	if !structure_tree_unreachable(&document) {
		return Ok(document);
	}
	let Some(bytes) = repair::repaired_bytes(&context.file_path) else { return Ok(document) };
	match PdfiumDocument::new_from_reader(Cursor::new(bytes), context.password.as_deref()) {
		Ok(repaired) if !structure_tree_unreachable(&repaired) => {
			tracing::debug!(path = %context.file_path, "gave the pdf a parent tree to reach its structure through");
			Ok(repaired)
		}
		Ok(_) => Ok(document),
		Err(err) => {
			tracing::warn!(path = %context.file_path, error = %err, "failed to load the repaired pdf document");
			Ok(document)
		}
	}
}

/// Whether the document advertises a structure tree whose top-level elements pdfium cannot load,
/// which is what a missing parent tree looks like from here. Only the first few pages are asked,
/// enough to meet a page that has a tree at all without walking a long document to find one.
fn structure_tree_unreachable(document: &PdfiumDocument) -> bool {
	const PROBED_PAGES: i32 = 10;
	for page_index in 0..document.page_count().min(PROBED_PAGES) {
		let Ok(page) = document.page(page_index) else { continue };
		let Some(tree) = page.struct_tree() else { continue };
		let count = tree.count_children();
		if count == 0 {
			continue;
		}
		return (0..count).all(|index| tree.child(index).is_err());
	}
	false
}

/// Settle which of the three sources of headings the document ends up with, and make sure the
/// buffer carries a marker for each one so heading navigation can stop on it.
///
/// A PDF can say what its headings are in three ways, and they do not agree. Its bookmarks are
/// already in `toc_items` if it had any. Its structure tree's `H1`-`H6` elements arrive as
/// `flat_toc_items`, already placed. Failing both, an untagged page's larger lines were guessed at
/// and arrive as `detected`. Bookmarks win where they exist, because a person wrote them.
fn resolve_headings(
	buffer: &mut DocumentBuffer,
	toc_items: &mut Vec<TocItem>,
	flat_toc_items: Vec<(u32, TocItem)>,
	detected: Vec<(usize, String)>,
	any_tags_processed: bool,
) {
	if any_tags_processed {
		if toc_items.is_empty() {
			*toc_items = build_toc_tree(flat_toc_items);
		} else if flat_toc_items.is_empty() {
			// Bookmarks, from a tagged document whose tree named no heading of its own.
			add_heading_markers(buffer, toc_items, 1);
		}
	} else if toc_items.is_empty() && !detected.is_empty() {
		for (pos, text) in &detected {
			buffer.add_marker(Marker::new(MarkerType::Heading1, *pos).with_text(text.clone()).with_level(1));
		}
		*toc_items = detected.into_iter().map(|(pos, text)| TocItem::new(text, String::new(), pos)).collect();
	} else {
		add_heading_markers(buffer, toc_items, 1);
	}
}

pub struct PdfParser;

impl Parser for PdfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing pdf document");
		let render_tables_inline = context.render_tables_inline;
		let document = load_document(context)?;
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
		let mut pages: Vec<PageContent> =
			(0..page_count).map(|page_index| read_page(&document, page_index, render_tables_inline)).collect();
		let running_text = strip_tagged_running_text(&mut pages);
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
			// A page that is one big scanned image wearing nothing but a page number is a scan, not
			// a page of text, and the page number is furniture over the top of it. Its text is
			// dropped and the page is offered for OCR like any other image-only page. This is what
			// makes a scanned book with burned-in page numbers readable at all: without it the
			// reader meets "[Image]" and a bare number on every page and no way to reach the words.
			if is_scanned_with_only_furniture(&page) {
				let placeholder_position = buffer.current_position();
				has_any_images = true;
				buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, placeholder_position));
				buffer.append(&image_only_placeholder());
				buffer.append("\n");
				page_lines_info.push(current_lines_info);
				continue;
			}
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
				for line in untagged_page_lines(&page, &running_text, context.join_pdf_paragraphs) {
					match line {
						UntaggedLine::Image => {
							append_images(1, &mut buffer, &mut page_display_text, &mut current_lines_info);
						}
						UntaggedLine::Text(text, is_heading) => {
							has_any_text = true;
							let current_offset = buffer.current_position();
							if is_heading {
								detected_heading_positions.push((current_offset, text.clone()));
							}
							current_lines_info.push((current_offset, text.clone()));
							buffer.append(&text);
							buffer.append("\n");
							page_display_text.push_str(&text);
							page_display_text.push('\n');
						}
					}
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
		resolve_headings(&mut buffer, &mut toc_items, flat_toc_items, detected_heading_positions, any_tags_processed);
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

#[cfg(test)]
mod tests {
	use super::looks_like_page_number;

	/// A scanned page's page number is furniture, however it is dressed, so the page can be offered
	/// for OCR rather than read as a bare number.
	#[test]
	fn a_page_number_is_recognised_as_furniture() {
		assert!(looks_like_page_number("42"));
		assert!(looks_like_page_number("  42  "));
		assert!(looks_like_page_number("- 42 -"));
		assert!(looks_like_page_number("[12]"));
	}

	/// Anything with a letter in it is real text, which is what keeps a caption or a heading from
	/// being mistaken for a page number.
	#[test]
	fn text_with_letters_is_not_furniture() {
		assert!(!looks_like_page_number("Page 3"));
		assert!(!looks_like_page_number("Chapter One"));
		assert!(!looks_like_page_number("iv"));
		assert!(!looks_like_page_number(""));
		// Punctuation with no digit is not a page number either.
		assert!(!looks_like_page_number("- -"));
		// Too long to be a page number, even all in digits.
		assert!(!looks_like_page_number("1234567890123"));
	}
}
