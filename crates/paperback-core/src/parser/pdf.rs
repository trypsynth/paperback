use std::{
	collections::{HashMap, HashSet},
	io::Cursor,
};

use anyhow::Result;
use pdfium::PdfiumDocument;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	ocr::image_only_placeholder,
	parser::{Parser, util::path::extract_title_from_path},
	util::text::display_len,
};

mod images;
mod links;
mod metadata;
mod paragraphs;
mod repair;
mod running;
mod structure;
mod text;
mod toc;

use images::{append_images, images_before_each_paragraph, page_image_tops};
use links::{PendingLink, collect_annotation_links, collect_web_links, place_links};
use metadata::{map_load_error, metadata_value};
pub use paragraphs::join_wrapped_lines;
use paragraphs::{join_paragraphs, split_lines};
use running::{EDGE_LINES, PageEdges, RunningText};
use structure::extract_tagged_page_text;
use text::{Line, extract_text_lines, median_line_font_size};
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
	lines: Vec<Line>,
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
	// Every page is scanned, rather than the document stopping at the first image it finds,
	// because each page places its own images and because an image-only page still needs its OCR
	// placeholder when earlier pages contributed text. Taken before the text so that a tagged
	// page whose tree names no figure can place them among its blocks as it writes them.
	let mut content = PageContent { image_tops: page_image_tops(&page), ..Default::default() };
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

/// Splice a tagged page's own buffer into the document, moving everything it holds from
/// page-relative positions to where the page landed.
fn append_tagged_page(buffer: &mut DocumentBuffer, page: DocumentBuffer, page_start_offset: usize) {
	buffer.append(&page.content);
	for mut marker in page.markers {
		marker.position += page_start_offset;
		buffer.add_marker(marker);
	}
}

/// Which of a tagged page's lines are running headers or footers, by their index among the
/// page's lines.
///
/// Only the lines at the page's edges are looked at, and only a line the tags say nothing about
/// at all. A running head is plain text by the time it reaches here; a line that carries a
/// marker is a heading, a list item, a table or a figure, and a document that opens a dozen of
/// its pages with a figure would otherwise lose every one of them. A page whose every line
/// matches keeps them all, so a page is never emptied.
fn tagged_running_lines(page: &TaggedPage, running_text: &RunningText) -> HashSet<usize> {
	let count = page.lines_info.len();
	let marked: HashSet<usize> = page.buffer.markers.iter().map(|marker| marker.position).collect();
	let at_an_edge = |index: usize| index < EDGE_LINES || index + EDGE_LINES >= count;
	let doomed: HashSet<usize> = (0..count)
		.filter(|index| at_an_edge(*index))
		.filter(|index| {
			let (position, text) = &page.lines_info[*index];
			// The untagged path keeps a heading safe by its size; here the tags say outright
			// which lines are headings, so no size is asked for.
			!marked.contains(position) && running_text.contains(text, 0.0)
		})
		.collect();
	if doomed.len() == count { HashSet::new() } else { doomed }
}

/// Rebuild a tagged page without the lines named in `doomed`, moving everything that sat after
/// one of them up by as much as it took. A marker inside a dropped line goes with it.
fn without_lines(page: TaggedPage, doomed: &HashSet<usize>) -> TaggedPage {
	if doomed.is_empty() {
		return page;
	}
	let mut rebuilt = TaggedPage {
		buffer: DocumentBuffer::new(),
		display_text: String::new(),
		lines_info: Vec::new(),
		toc_items: Vec::new(),
	};
	// Where each of the old page's lines starts now, and how far it runs. A line that is not
	// kept maps to nothing, and whatever pointed into it is dropped with it.
	let mut moved: Vec<(usize, usize, Option<usize>)> = Vec::with_capacity(page.lines_info.len());
	for (index, (position, text)) in page.lines_info.iter().enumerate() {
		let length = display_len(text) + 1;
		if doomed.contains(&index) {
			moved.push((*position, length, None));
			continue;
		}
		let start = rebuilt.buffer.current_position();
		moved.push((*position, length, Some(start)));
		rebuilt.lines_info.push((start, text.clone()));
		rebuilt.buffer.append(text);
		rebuilt.buffer.append("\n");
		rebuilt.display_text.push_str(text);
		rebuilt.display_text.push('\n');
	}
	// A position past the last line, which a marker placed at the end of a page holds, keeps
	// standing at the end of it.
	let end = rebuilt.buffer.current_position();
	let move_position = |position: usize| {
		moved
			.iter()
			.find(|(start, length, _)| position >= *start && position < start + length)
			.map_or(Some(end), |(start, _, moved_to)| moved_to.map(|moved_to| moved_to + (position - start)))
	};
	for mut marker in page.buffer.markers {
		if let Some(position) = move_position(marker.position) {
			marker.position = position;
			rebuilt.buffer.add_marker(marker);
		}
	}
	rebuilt.toc_items = page
		.toc_items
		.into_iter()
		.filter_map(|(level, mut item)| {
			item.offset = move_position(item.offset)?;
			Some((level, item))
		})
		.collect();
	rebuilt
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
		let untagged = || pages.iter().filter(|page| page.tagged.is_none());
		let edges: Vec<PageEdges> = untagged().map(|page| PageEdges::of(&page.lines)).collect();
		// A tagged page gets a survey of its own. Most tagged PDFs mark their running headers as
		// artifacts and never hand them over as text at all, but a maker that tags them as
		// ordinary paragraphs would otherwise repeat the book's title and the page number between
		// two paragraphs on every page of it.
		let tagged_edges: Vec<PageEdges> = pages
			.iter()
			.filter_map(|page| page.tagged.as_ref())
			.map(|tagged| {
				let lines: Vec<String> = tagged.lines_info.iter().map(|(_, text)| text.clone()).collect();
				PageEdges::of_texts(&lines)
			})
			.collect();
		let tagged_running = running::detect_tagged(&tagged_edges);
		// The body size is taken over the whole document rather than one page: a chapter opening
		// is mostly heading, and the survey has to know a heading from a running head.
		let body_font_size = median_line_font_size(&untagged().flat_map(|page| page.lines.clone()).collect::<Vec<_>>());
		let running_text = running::detect(&edges, body_font_size);
		tracing::debug!(
			running_text_count = running_text.len(),
			body_font_size,
			"surveyed pages for running headers and footers"
		);
		tracing::debug!(
			tagged_running_text_count = tagged_running.len(),
			tagged_page_count = tagged_edges.len(),
			"surveyed tagged pages for running headers and footers"
		);
		for page in &mut pages {
			if let Some(tagged) = page.tagged.take() {
				let doomed = tagged_running_lines(&tagged, &tagged_running);
				page.tagged = Some(without_lines(tagged, &doomed));
			}
		}
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
				let line_tops: Vec<f64> = line_infos.iter().map(|line| line.top).collect();
				let paragraphs = if context.join_pdf_paragraphs {
					join_paragraphs(&line_infos, body_size)
				} else {
					split_lines(&line_infos, body_size)
				};
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

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use super::{TaggedPage, running, tagged_running_lines, without_lines};
	use crate::document::{DocumentBuffer, Marker, MarkerType, TocItem};

	/// A tagged page built from finished lines, with a marker on whichever of them is named.
	fn tagged_page(lines: &[&str], marked: &[usize]) -> TaggedPage {
		let mut page = TaggedPage {
			buffer: DocumentBuffer::new(),
			display_text: String::new(),
			lines_info: Vec::new(),
			toc_items: Vec::new(),
		};
		for (index, text) in lines.iter().enumerate() {
			let position = page.buffer.current_position();
			if marked.contains(&index) {
				page.buffer.add_marker(Marker::new(MarkerType::Heading1, position).with_text((*text).to_string()));
				page.toc_items.push((1, TocItem::new((*text).to_string(), String::new(), position)));
			}
			page.lines_info.push((position, (*text).to_string()));
			page.buffer.append(text);
			page.buffer.append("\n");
			page.display_text.push_str(text);
			page.display_text.push('\n');
		}
		page
	}

	/// Five pages that all open with the book's title and close with a page number, which is what
	/// furniture on a tagged page looks like.
	fn running_text() -> running::RunningText {
		let bodies = ["Salt.", "Pepper.", "Thyme.", "Parsley.", "Sage."];
		let edges: Vec<running::PageEdges> = (1..=5)
			.map(|page| {
				running::PageEdges::of_texts(&[
					"The Book Of Tests".to_string(),
					bodies[page - 1].to_string(),
					format!("{page}"),
				])
			})
			.collect();
		running::detect_tagged(&edges)
	}

	#[test]
	fn a_tagged_page_loses_its_running_head_and_page_number() {
		let page = tagged_page(&["The Book Of Tests", "Body of page 6.", "6"], &[]);
		let doomed = tagged_running_lines(&page, &running_text());
		assert_eq!(doomed, HashSet::from([0, 2]));
		let rebuilt = without_lines(page, &doomed);
		assert_eq!(rebuilt.buffer.content, "Body of page 6.\n");
		assert_eq!(rebuilt.lines_info, vec![(0, "Body of page 6.".to_string())]);
		assert_eq!(rebuilt.display_text, "Body of page 6.\n");
	}

	#[test]
	fn a_line_the_tags_speak_for_is_never_furniture() {
		// The same repeated title, this time tagged as the page's heading.
		let page = tagged_page(&["The Book Of Tests", "Rosemary.", "6"], &[0]);
		assert_eq!(tagged_running_lines(&page, &running_text()), HashSet::from([2]));
	}

	#[test]
	fn a_page_of_nothing_but_furniture_keeps_it() {
		let page = tagged_page(&["The Book Of Tests", "6"], &[]);
		assert!(tagged_running_lines(&page, &running_text()).is_empty(), "a page is never emptied");
	}

	#[test]
	fn a_line_repeated_on_a_few_pages_only_is_not_furniture() {
		// Four pages carry it, which is enough for an untagged page, out of twenty tagged ones.
		// The signature blanks out digits, so each page is given a word of its own rather than a
		// number, or every page would read as the same line.
		let word = |page: usize| format!("{}", char::from(b'a' + u8::try_from(page).expect("a small page number")));
		let edges: Vec<running::PageEdges> = (0..20)
			.map(|page| {
				let opening = if page < 4 {
					"The following applies.".to_string()
				} else {
					format!("Opening {} here.", word(page))
				};
				running::PageEdges::of_texts(&[opening, format!("Closing {} here.", word(page))])
			})
			.collect();
		let detected = running::detect_tagged(&edges);
		let page = tagged_page(&["The following applies.", "Closing a here."], &[]);
		assert!(tagged_running_lines(&page, &detected).is_empty());
	}

	#[test]
	fn markers_and_toc_items_move_up_with_the_lines_they_sit_on() {
		let page = tagged_page(&["Furniture", "Chapter One", "Body."], &[1]);
		let rebuilt = without_lines(page, &HashSet::from([0]));
		assert_eq!(rebuilt.buffer.content, "Chapter One\nBody.\n");
		let heading = rebuilt.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Heading1);
		assert_eq!(heading.expect("the heading survives").position, 0, "it moved up by the dropped line");
		assert_eq!(rebuilt.toc_items[0].1.offset, 0);
	}

	#[test]
	fn a_marker_inside_a_dropped_line_goes_with_it() {
		let page = tagged_page(&["Furniture", "Body."], &[0]);
		let rebuilt = without_lines(page, &HashSet::from([0]));
		assert!(rebuilt.buffer.markers.is_empty(), "the dropped line's heading is dropped too");
		assert!(rebuilt.toc_items.is_empty());
	}
}
