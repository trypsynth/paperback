use std::collections::HashMap;

use anyhow::Result;
use rayon::prelude::*;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	ocr::image_only_placeholder,
	parser::{Parser, add_heading_markers, util::path::extract_title_from_path},
	pdf_engine::{PAGE_IMAGE_COVERAGE, PdfDocument, PdfError, PdfPage},
};

mod images;
mod links;
mod metadata;
mod paragraphs;
mod running;
mod structure;
mod tagged;
mod text;
mod toc;

use images::{append_image, images_before_each_paragraph, page_largest_image_coverage};
use links::{PendingLink, collect_annotation_links, collect_web_links, place_links};
use metadata::{map_load_error, metadata_title, metadata_value};
use paragraphs::{join_paragraphs, split_lines};
pub use paragraphs::{join_wrapped_lines, join_wrapped_runs};
use running::{EDGE_LINES, PageEdges, RunningText};
use structure::extract_tagged_page_text;
use tagged::{TaggedPage, append_tagged_page, tagged_running_lines, without_lines};
use text::{Line, PageEvidence, WordPages, confirms, extract_text_lines, median_line_font_size};
use toc::{build_toc_tree, extract_toc};

/// Every page is read before any is assembled, because running headers are only known once every page's edges have been seen.
#[derive(Default)]
struct PageContent {
	/// Its buffer's positions are relative to the page.
	tagged: Option<TaggedPage>,
	lines: Vec<Line>,
	/// Kept apart because a link annotation may sit before a bare URL that pdfium's scanner reported first, so each kind is placed by its own walk.
	web_links: Vec<PendingLink>,
	annotation_links: Vec<PendingLink>,
	image_tops: Vec<f64>,
	image_paths: Vec<String>,
	largest_image_coverage: f64,
}

fn read_page(
	page: Result<PdfPage<'_>, PdfError>,
	page_index: i32,
	render_tables_inline: bool,
	evidence: &mut PageEvidence,
) -> PageContent {
	let Ok(page) = page else {
		tracing::warn!(page_index, "failed to load pdf page, skipping its text");
		return PageContent::default();
	};
	let Some(text_page) = page.text() else {
		tracing::warn!(page_index, "failed to load text for pdf page, skipping its text");
		return PageContent::default();
	};
	// Taken before the text so a tagged page whose tree names no figure can place its images among its blocks as it writes them.
	let announced = page.announced_images();
	let mut content = PageContent {
		image_tops: announced.iter().map(|image| image.top).collect(),
		image_paths: announced.into_iter().map(|image| image.path).collect(),
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
		text_page,
		page_index,
		&mut tagged.buffer,
		&mut tagged.display_text,
		&mut tagged.lines_info,
		&mut tagged.toc_items,
		render_tables_inline,
		&content.image_tops,
		&content.image_paths,
		evidence,
	) {
		content.tagged = Some(tagged);
	} else {
		content.lines = extract_text_lines(text_page, page_index, evidence);
	}
	content.web_links = collect_web_links(text_page);
	content.annotation_links = collect_annotation_links(&page, text_page);
	content
}

/// A scanned sheet is one image over the whole page; half is well clear of a figure sitting among paragraphs.
const SCAN_IMAGE_COVERAGE: f64 = 0.5;

/// Room for a page number dressed up as "- 42 -", far short of a page of words.
const MAX_FURNITURE_CHARS: usize = 12;

/// Needs both halves: a page of text can carry a page number, and a full-page figure can carry a caption, which unlike a scan's watermark or running title never repeats.
fn is_scanned_with_only_furniture(page: &PageContent, running: &RunningText) -> bool {
	if page.largest_image_coverage < SCAN_IMAGE_COVERAGE {
		return false;
	}
	let text = if let Some(tagged) = &page.tagged {
		tagged.display_text.clone()
	} else {
		let body = text_without_running_lines(&page.lines, running);
		if body.trim().is_empty() && !page.lines.is_empty() {
			return true;
		}
		body
	};
	looks_like_page_number(&text)
}

/// Only edge lines are checked, the only place [`RunningText::contains`] is meant to be asked about.
fn text_without_running_lines(lines: &[Line], running: &RunningText) -> String {
	let bottom_edge = lines.len().saturating_sub(running::EDGE_LINES);
	lines
		.iter()
		.enumerate()
		.filter(|(index, line)| {
			let at_edge = *index < running::EDGE_LINES || *index >= bottom_edge;
			!(at_edge && running.contains(&line.text, line.size))
		})
		.map(|(_, line)| line.text.as_str())
		.collect::<Vec<_>>()
		.join(" ")
}

/// Letters rule it out, which keeps a figure-with-caption page from being mistaken for a scan.
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

enum UntaggedLine {
	Text(String, bool),
	/// Only the content-stream path: an untagged page has no description to give for an image.
	Image(String),
}

fn untagged_page_lines(page: &PageContent, running_text: &RunningText, join: bool) -> Vec<UntaggedLine> {
	let mut line_infos = page.lines.clone();
	// Measured before the running headers come out: they sit at or below body size, so dropping them would pull the median up past a heading only just above it.
	let body_size = median_line_font_size(&line_infos);
	strip_running_text(&mut line_infos, running_text);
	let line_tops: Vec<f64> = line_infos.iter().map(|line| line.top).collect();
	let paragraphs = if join { join_paragraphs(&line_infos, body_size) } else { split_lines(&line_infos, body_size) };
	// The caller's image-only placeholder is more use on a scanned page than a line for each piece it was scanned into.
	if paragraphs.is_empty() {
		return Vec::new();
	}
	// An untagged page says nothing about where its images belong, so each is placed by the height it was drawn at.
	let paragraph_tops: Vec<f64> = paragraphs
		.iter()
		.map(|(.., line_index)| line_tops.get(*line_index).copied().unwrap_or(f64::NEG_INFINITY))
		.collect();
	let image_counts = images_before_each_paragraph(&page.image_tops, &paragraph_tops);
	let mut lines = Vec::with_capacity(paragraphs.len());
	let mut next_image = 0usize;
	let take_image = |next_image: &mut usize| {
		let path = page.image_paths.get(*next_image).cloned().unwrap_or_default();
		*next_image += 1;
		UntaggedLine::Image(path)
	};
	for (index, (text, is_heading, _)) in paragraphs.iter().enumerate() {
		for _ in 0..image_counts[index] {
			lines.push(take_image(&mut next_image));
		}
		lines.push(UntaggedLine::Text(text.clone(), *is_heading));
	}
	for _ in 0..image_counts[paragraphs.len()] {
		lines.push(take_image(&mut next_image));
	}
	lines
}

/// Tagged lines are only taken out here, so `strip` has to be honoured here rather than by ignoring the returned survey.
fn strip_tagged_running_text(pages: &mut [PageContent], strip: bool) -> RunningText {
	let untagged = || pages.iter().filter(|page| page.tagged.is_none());
	let edges: Vec<PageEdges> = untagged().map(|page| PageEdges::of(&page.lines)).collect();
	// Taken over the whole document because a chapter opening is mostly heading.
	let body_font_size = median_line_font_size(&untagged().flat_map(|page| page.lines.clone()).collect::<Vec<_>>());
	let running_text = running::detect(&edges, body_font_size);
	tracing::debug!(
		running_text_count = running_text.len(),
		body_font_size,
		"surveyed pages for running headers and footers"
	);
	// Most tagged PDFs mark running headers as artifacts, but some tag them as ordinary paragraphs.
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
	if strip {
		for page in pages.iter_mut() {
			if let Some(tagged) = page.tagged.take() {
				let doomed = tagged_running_lines(&tagged, &tagged_running);
				page.tagged = Some(without_lines(tagged, &doomed));
			}
		}
	}
	running_text
}

/// Only edge lines are considered, so a body line reading like a running head stays, and a page whose every line matches (a part title repeating the book's name) is never emptied.
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

fn load_document(context: &ParserContext) -> Result<PdfDocument> {
	PdfDocument::open(&context.file_path, context.password.as_deref()).map_err(|err| {
		let mapped = map_load_error(err);
		tracing::warn!(path = %context.file_path, error = %mapped, "failed to load pdf document");
		mapped
	})
}

/// Bookmarks win over structure-tree and font-size headings where they exist, because a person wrote them.
fn resolve_headings(
	buffer: &mut DocumentBuffer,
	toc_items: &mut Vec<TocItem>,
	flat_toc_items: Vec<(u32, TocItem)>,
	detected: Vec<(DocPos, String)>,
	any_tags_processed: bool,
) {
	if any_tags_processed {
		if toc_items.is_empty() {
			*toc_items = build_toc_tree(flat_toc_items);
		} else if flat_toc_items.is_empty() {
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

/// Runs of neighbouring pages, so the pages a thread judges hyphens by lie close to the ones it reads.
const RUN_LENGTH: i32 = 16;

/// Each thread keeps one page cache so a book's fonts are read once per thread, and reads runs in order so [`PageEvidence`] judges almost every hyphen from its own pages; the few it misjudges are read again against every page.
fn read_pages(document: &PdfDocument, page_count: i32, render_tables_inline: bool) -> Vec<PageContent> {
	let runs = usize::try_from((page_count + RUN_LENGTH - 1) / RUN_LENGTH).unwrap_or(0);
	let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).clamp(1, runs.max(1));
	let (read, words): (Vec<_>, Vec<_>) = std::thread::scope(|scope| {
		#[allow(clippy::needless_collect, reason = "every thread has to be started before the first one is waited on")]
		let handles: Vec<_> = (0..threads)
			.map(|thread| {
				scope.spawn(move || {
					let cache = document.page_cache();
					let mut words = WordPages::default();
					let mut pages = Vec::new();
					for run in (thread..runs).step_by(threads) {
						let start = i32::try_from(run).unwrap_or(i32::MAX).saturating_mul(RUN_LENGTH);
						for index in start..(start + RUN_LENGTH).min(page_count) {
							let mut evidence = PageEvidence::new(index, std::slice::from_ref(&words));
							let content = read_page(
								document.page_with(index, &cache),
								index,
								render_tables_inline,
								&mut evidence,
							);
							let (own, judgements) = evidence.finish();
							words.add(index, own);
							pages.push((index, content, judgements));
						}
					}
					(pages, words)
				})
			})
			.collect();
		handles.into_iter().map(|handle| handle.join().unwrap_or_else(|panic| std::panic::resume_unwind(panic))).unzip()
	});
	let mut read: Vec<_> = read.into_iter().flatten().collect();
	read.sort_unstable_by_key(|(index, ..)| *index);
	let misjudged: Vec<i32> = read
		.iter()
		.filter(|(index, _, judgements)| !confirms(judgements, *index, &words))
		.map(|(index, ..)| *index)
		.collect();
	tracing::debug!(misjudged = misjudged.len(), "pdf pages read again for their hyphens");
	let reread: Vec<PageContent> = misjudged
		.par_iter()
		.map_init(
			|| document.page_cache(),
			|cache, &index| {
				read_page(
					document.page_with(index, cache),
					index,
					render_tables_inline,
					&mut PageEvidence::new(index, &words),
				)
			},
		)
		.collect();
	let mut pages: Vec<PageContent> = read.into_iter().map(|(_, content, _)| content).collect();
	for (index, content) in misjudged.into_iter().zip(reread) {
		pages[usize::try_from(index).unwrap_or(usize::MAX)] = content;
	}
	pages
}

/// Judged by the word's shape alone, with no evidence from the rest of the document.
pub(crate) fn keeps_line_end_hyphen(left: &str, right: &str) -> bool {
	text::keeps_hyphen(left, right, None)
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
		let mut page_lines_info: Vec<Vec<(DocPos, String)>> = Vec::new();
		let page_count = document.page_count();
		let mut any_tags_processed = false;
		let mut flat_toc_items = Vec::new();
		let mut has_any_text = false;
		let mut has_any_images = false;
		let mut detected_heading_positions: Vec<(DocPos, String)> = Vec::new();
		let mut pages = read_pages(&document, page_count, render_tables_inline);
		// The survey always runs because it also measures body text size; the flag only decides whether its answer is used.
		let survey = strip_tagged_running_text(&mut pages, context.strip_running_text);
		let running_text = if context.strip_running_text { survey } else { survey.recognizing_nothing() };
		// Laying out an untagged page is what putting the document together spends its time on, and it needs only the page and the survey, so every page is laid out at once.
		let untagged: Vec<Vec<UntaggedLine>> = pages
			.par_iter()
			.map(|page| {
				if page.tagged.is_none() {
					untagged_page_lines(page, &running_text, context.join_pdf_paragraphs)
				} else {
					Vec::new()
				}
			})
			.collect();
		for ((page_index, page), untagged) in pages.into_iter().enumerate().zip(untagged) {
			let marker_position = buffer.end();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{page_index}"), marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let page_start_offset = buffer.end();
			let mut page_display_text = String::new();
			let mut current_lines_info = Vec::new();
			// Without this a scanned book with burned-in page numbers reads as "[Image]" and a bare number on every page, with no way to OCR the words.
			if is_scanned_with_only_furniture(&page, &running_text) {
				has_any_images = true;
				let placeholder_position = buffer.end();
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
				current_lines_info = tagged
					.lines_info
					.into_iter()
					.map(|(pos, text)| (pos + page_start_offset.from_start(), text))
					.collect();
				flat_toc_items.extend(tagged.toc_items.into_iter().map(|(level, mut item)| {
					item.offset += page_start_offset.from_start();
					(level, item)
				}));
			} else {
				for line in untagged {
					match line {
						UntaggedLine::Image(path) => {
							let reference = format!("pdf:{page_index}:{path}");
							append_image(
								&mut buffer,
								MarkerType::Image,
								"",
								&mut page_display_text,
								&mut current_lines_info,
								&reference,
							);
						}
						UntaggedLine::Text(text, is_heading) => {
							has_any_text = true;
							let current_offset = buffer.end();
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
			// A page-sized scan is left out of `image_tops` so it isn't announced, but a page that is nothing else still needs its OCR placeholder.
			let page_has_image = !page.image_tops.is_empty() || page.largest_image_coverage >= PAGE_IMAGE_COVERAGE;
			if page_has_image {
				has_any_images = true;
			}
			// The OCR flow matches on the marker, not the text, so changing the UI language cannot strand a placeholder.
			let page_has_text = buffer.end() > page_start_offset;
			if !page_has_text && page_has_image {
				let placeholder_position = buffer.end();
				buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, placeholder_position));
				buffer.append(&image_only_placeholder());
				buffer.append("\n");
			}
			place_links(&page.web_links, page_start_offset, &page_display_text, &mut buffer);
			place_links(&page.annotation_links, page_start_offset, &page_display_text, &mut buffer);
			page_lines_info.push(current_lines_info);
		}
		let title = metadata_title(&document).unwrap_or_else(|| extract_title_from_path(&context.file_path));
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
	use super::{
		Line, PageContent, PageEdges, TaggedPage, is_scanned_with_only_furniture, looks_like_page_number,
		running::detect,
	};
	use crate::document::DocumentBuffer;

	const BODY: f64 = 10.0;

	fn line(text: &str) -> Line {
		Line { text: text.to_string(), size: BODY, top: 0.0, bottom: 0.0, monospaced: false, bold: false }
	}

	fn page(coverage: f64, lines: &[&str]) -> PageContent {
		PageContent {
			largest_image_coverage: coverage,
			lines: lines.iter().map(|text| line(text)).collect(),
			..Default::default()
		}
	}

	fn running(pages: &[PageContent]) -> super::RunningText {
		let edges: Vec<PageEdges> = pages.iter().map(|page| PageEdges::of(&page.lines)).collect();
		detect(&edges, BODY)
	}

	/// The reported case: a scanned magazine with the same advert stamped on every page.
	#[test]
	fn a_scan_whose_only_text_repeats_on_every_page_is_a_scan() {
		let pages: Vec<_> = (0..6).map(|_| page(0.95, &["Crowdfunded 190+ magazines, WeChat jiage8117"])).collect();
		assert!(is_scanned_with_only_furniture(&pages[0], &running(&pages)));
	}

	/// Fills `lines_info` because the tagged survey reads it and `without_lines` rebuilds from it; without it the page would never be stripped and the test would pass for the wrong reason.
	fn tagged_page(blocks: &[&str]) -> PageContent {
		let mut tagged = TaggedPage {
			buffer: DocumentBuffer::new(),
			display_text: String::new(),
			lines_info: Vec::new(),
			toc_items: Vec::new(),
		};
		for block in blocks {
			let position = tagged.buffer.end();
			tagged.lines_info.push((position, (*block).to_string()));
			tagged.buffer.append(block);
			tagged.buffer.append("\n");
		}
		tagged.display_text = tagged.buffer.content.clone();
		PageContent { tagged: Some(tagged), ..Default::default() }
	}

	fn tagged_texts(pages: &[PageContent]) -> Vec<String> {
		pages.iter().map(|page| page.tagged.as_ref().map_or(String::new(), |t| t.buffer.content.clone())).collect()
	}

	/// Tagged lines are removed inside the survey, so a flag applied only to its answer would still lose them.
	#[test]
	fn turning_the_strip_off_keeps_the_repeated_lines_of_a_tagged_page() {
		let make = || {
			(0..6)
				.map(|page| {
					// The survey won't empty a page whose every line repeats, and it blanks digit runs, so each body differs in letters.
					let body = format!("words unique to the {} page", char::from(b'a' + u8::try_from(page).unwrap()));
					tagged_page(&["BOOK TITLE", body.as_str(), "a closing line", "and one after that"])
				})
				.collect::<Vec<_>>()
		};
		let mut stripped = make();
		super::strip_tagged_running_text(&mut stripped, true);
		let kept = tagged_texts(&stripped);
		assert!(
			kept.iter().all(|text| !text.contains("BOOK TITLE")),
			"the survey should have taken the repeated line out: {kept:?}"
		);

		let mut untouched = make();
		super::strip_tagged_running_text(&mut untouched, false);
		assert!(
			tagged_texts(&untouched).iter().all(|text| text.contains("BOOK TITLE")),
			"a document converted as it is should keep every line"
		);
	}

	#[test]
	fn a_full_page_figure_with_its_own_caption_is_not_a_scan() {
		let mut pages: Vec<_> = (0..6).map(|_| page(0.95, &["Crowdfunded 190+ magazines, WeChat jiage8117"])).collect();
		pages[0] = page(0.95, &["Figure 3. The river delta at dawn.", "Crowdfunded 190+ magazines, WeChat jiage8117"]);
		assert!(!is_scanned_with_only_furniture(&pages[0], &running(&pages)));
	}

	#[test]
	fn repeated_text_on_a_page_without_a_large_image_is_not_a_scan() {
		let pages: Vec<_> = (0..6).map(|_| page(0.1, &["Crowdfunded 190+ magazines, WeChat jiage8117"])).collect();
		assert!(!is_scanned_with_only_furniture(&pages[0], &running(&pages)));
	}

	#[test]
	fn a_page_number_is_recognised_as_furniture() {
		assert!(looks_like_page_number("42"));
		assert!(looks_like_page_number("  42  "));
		assert!(looks_like_page_number("- 42 -"));
		assert!(looks_like_page_number("[12]"));
	}

	#[test]
	fn text_with_letters_is_not_furniture() {
		assert!(!looks_like_page_number("Page 3"));
		assert!(!looks_like_page_number("Chapter One"));
		assert!(!looks_like_page_number("iv"));
		assert!(!looks_like_page_number(""));
		assert!(!looks_like_page_number("- -"));
		assert!(!looks_like_page_number("1234567890123"));
	}
}
