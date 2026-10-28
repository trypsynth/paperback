//! Recognized text replaces what it stands for rather than going in front of it, since an insertion at a page's start pushes the page break and image markers past the words (see [`Edit`]).

use std::{collections::HashMap, sync::atomic::AtomicBool};

use anyhow::{Context, Result};
use native_ocr::{Error as OcrError, Line, max_image_dimension, recognize_lines};
use paperback_core::{
	document::{ContentBytePos, DocPos, Document, Edit, Marker, MarkerType},
	ocr::{
		PageRenderer, RecognizedLine, RecognizedWord, arrange, default_workers, image_only_placeholder, recognize_pages,
	},
};

/// Matches the app's own gate, so pb and the app agree about which documents Batch OCR applies to.
pub fn is_ocr_able(file_path: &str) -> bool {
	["pdf", "cbz", "cbr"].iter().any(|extension| file_path.to_ascii_lowercase().ends_with(&format!(".{extension}")))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Selection {
	pub image_pages: bool,
	pub text_pages: bool,
}

impl Selection {
	pub const fn any(self) -> bool {
		self.image_pages || self.text_pages
	}
}

/// A page that fails is reported and skipped, so one bad page doesn't throw away the rest of a thousand-page scan.
pub fn pages(doc: &mut Document, file_path: &str, password: Option<&str>, selection: Selection) -> Result<()> {
	if !selection.any() {
		return Ok(());
	}
	if !is_ocr_able(file_path) {
		eprintln!("pb: {file_path} has no pages to read, since it is not a PDF or a comic archive");
		return Ok(());
	}
	let mut edits = page_edits(doc, selection);
	if edits.is_empty() {
		return Ok(());
	}
	let max_dimension = max_image_dimension();
	// A page number too big for an `i32` could never be rendered, so its edit keeps what it had.
	let page_numbers: Vec<i32> = edits.iter().filter_map(|edit| i32::try_from(edit.page).ok()).collect();
	let mut results: HashMap<i32, Result<String, OcrError>> = HashMap::with_capacity(page_numbers.len());
	// The command line cannot stop a run part-way, so nothing raises this.
	let cancel = AtomicBool::new(false);
	recognize_pages(
		file_path,
		password,
		&page_numbers,
		default_workers(),
		&cancel,
		|renderer, page| recognize_page(renderer, page, max_dimension),
		|page, result| {
			results.insert(page, result);
		},
	)
	.with_context(|| format!("failed to open {file_path} to read its pages"))?;
	let mut failures = Vec::new();
	for edit in &mut edits {
		let Ok(page) = i32::try_from(edit.page) else { continue };
		match results.remove(&page) {
			Some(Ok(text)) => {
				let text = text.trim().to_string();
				// Blanking a page the engine read nothing from would claim the scan is blank.
				if !text.is_empty() {
					edit.text = if edit.whole_page { format!("{text}\n") } else { text };
				}
			}
			Some(Err(error)) => failures.push((edit.page, error.to_string())),
			None => {}
		}
	}
	let read = edits.iter().filter(|edit| !edit.text.is_empty()).count();
	let pages_wanted = edits.len();
	let edits: Vec<Edit> =
		edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
	doc.buffer.replace_ranges(edits);
	// After the text is in place, so a run that recognised most of a book still produces it.
	for (page, error) in &failures {
		eprintln!("pb: page {page} could not be read: {error}");
	}
	if read > 0 || !failures.is_empty() {
		eprintln!(
			"pb: read {read} of {pages_wanted} {}; {} could not be read",
			if pages_wanted == 1 { "page" } else { "pages" },
			failures.len()
		);
	}
	doc.compute_stats();
	Ok(())
}

fn recognize_page(renderer: &mut PageRenderer, page: i32, max_dimension: u32) -> Result<String, OcrError> {
	// `render` takes a 0-based index; the document numbers pages from one.
	let rendered = renderer.render(page - 1, max_dimension).map_err(|error| OcrError::Failed(error.to_string()))?;
	let lines: Vec<RecognizedLine> =
		recognize_lines(&rendered.rgba, rendered.width, rendered.height)?.into_iter().map(recognized).collect();
	Ok(arrange(&lines, true))
}

fn recognized(line: Line) -> RecognizedLine {
	let words = line
		.words
		.into_iter()
		.map(|word| RecognizedWord {
			text: word.text,
			x: word.bounds.x,
			y: word.bounds.y,
			width: word.bounds.width,
			height: word.bounds.height,
		})
		.collect();
	RecognizedLine { text: line.text, words }
}

/// Every edit is filled in before any is applied, so it carries the page number rather than relying on offsets that go stale once the first page lands.
#[derive(Debug)]
struct PageEdit {
	page: usize,
	start: DocPos,
	end: DocPos,
	whole_page: bool,
	text: String,
}

/// `--pages` keeps the markers naming the pages an extract was cut from, so rendering by position would read the wrong sheet and pass as a misreading.
/// The PDF and comic parsers name each page `"Page {n}"` for its number in the file, as [`PageRenderer`] numbers them.
fn source_page(marker: &Marker, position: usize) -> usize {
	marker
		.text
		.strip_prefix("Page ")
		.and_then(|page| page.parse::<usize>().ok())
		.filter(|page| *page > 0)
		.unwrap_or(position)
}

/// A scanned page gives up only its instruction line, keeping the newline so the next page doesn't run into it.
/// A text page gives up its whole span and the markers in it, since a page re-read from its picture has no headings or links to point at.
fn page_edits(doc: &Document, selection: Selection) -> Vec<PageEdit> {
	let buffer = &doc.buffer;
	let mut breaks: Vec<&Marker> =
		buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::PageBreak).collect();
	breaks.sort_unstable_by_key(|marker| marker.position);
	let scanned: Vec<DocPos> = buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::ImageOnlyPage)
		.map(|marker| marker.position)
		.collect();
	let total = buffer.end();
	let mut edits = Vec::new();
	// Every page, since a page that isn't wanted still counts toward the position `source_page` falls back to.
	for (index, marker) in breaks.iter().enumerate() {
		let start = marker.position;
		let page = source_page(marker, index + 1);
		let end = breaks.get(index + 1).map_or(total, |next| next.position);
		if scanned.contains(&start) {
			if !selection.image_pages {
				continue;
			}
			// A page at the very top has no newline in front of it to find a start after.
			let byte_start = buffer.byte_at(start);
			let head = buffer
				.slice(..byte_start)
				.rfind('\n')
				.map_or(ContentBytePos::ZERO, |offset| ContentBytePos::from_raw(offset + 1));
			let rest = buffer.slice(byte_start..);
			let length = rest.find('\n').unwrap_or(rest.len());
			let line_start = buffer.pos_at_byte(head);
			let line_end = buffer.pos_at_byte(byte_start + length);
			if line_end > line_start {
				edits.push(PageEdit {
					page,
					start: line_start,
					end: line_end,
					whole_page: false,
					text: image_only_placeholder(),
				});
			}
			continue;
		}
		if selection.text_pages && end > start {
			edits.push(PageEdit { page, start, end, whole_page: true, text: String::new() });
		}
	}
	edits
}

#[cfg(test)]
mod tests {
	use paperback_core::document::{DocLen, DocumentBuffer, Marker};

	use super::*;

	fn document() -> Document {
		let mut buffer = DocumentBuffer::new();
		for page in 0..3 {
			let position = buffer.end();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", page + 1)));
			buffer.append(&format!("text of page {}\n", page + 1));
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc
	}

	fn extract_of_pages(first: usize, count: usize) -> Document {
		let mut buffer = DocumentBuffer::new();
		for index in 0..count {
			let position = buffer.end();
			buffer
				.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", first + index)));
			buffer.append(&format!("text of page {}\n", first + index));
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc
	}

	fn scanned_extract_of_pages(first: usize, count: usize) -> Document {
		let mut buffer = DocumentBuffer::new();
		for index in 0..count {
			let position = buffer.end();
			buffer
				.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", first + index)));
			buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
			buffer.append(&format!("{}\n", image_only_placeholder()));
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc
	}

	fn images_only() -> Selection {
		Selection { image_pages: true, text_pages: false }
	}

	fn text_only() -> Selection {
		Selection { image_pages: false, text_pages: true }
	}

	fn both() -> Selection {
		Selection { image_pages: true, text_pages: true }
	}

	fn read(pages: &mut Document, selection: Selection) -> String {
		// These documents have nothing to render, so only the edits are applied.
		let edits = page_edits(pages, selection);
		let edits: Vec<Edit> =
			edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
		pages.buffer.replace_ranges(edits);
		pages.buffer.content.clone()
	}

	#[test]
	fn a_document_with_no_scanned_pages_has_nothing_to_read() {
		assert!(page_edits(&document(), images_only()).is_empty());
	}

	#[test]
	fn a_scanned_page_is_reported_by_the_number_the_document_gives_it() {
		let mut doc = document();
		let start =
			doc.buffer.pos_at_byte(ContentBytePos::from_raw(doc.buffer.content.find("text of page 2").unwrap()));
		doc.buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, start));
		let edits = page_edits(&doc, images_only());
		assert_eq!(edits.len(), 1);
		assert_eq!(edits[0].page, 2);
	}

	/// `--pages 43000-43002 --ocr-image-pages` once rendered page 1 three times, silently, since the words still landed on the right page of the extract.
	#[test]
	fn a_scanned_page_of_an_extract_is_rendered_from_its_own_page_of_the_file() {
		let doc = scanned_extract_of_pages(43000, 3);
		let pages: Vec<usize> = page_edits(&doc, images_only()).iter().map(|edit| edit.page).collect();
		assert_eq!(pages, vec![43000, 43001, 43002]);
	}

	#[test]
	fn a_text_page_of_an_extract_is_rendered_from_its_own_page_of_the_file() {
		let doc = extract_of_pages(43000, 3);
		let pages: Vec<usize> = page_edits(&doc, text_only()).iter().map(|edit| edit.page).collect();
		assert_eq!(pages, vec![43000, 43001, 43002]);
	}

	#[test]
	fn a_marker_that_names_no_page_is_numbered_by_where_it_sits() {
		let mut doc = document();
		for marker in &mut doc.buffer.markers {
			if marker.mtype == MarkerType::PageBreak {
				marker.text.clear();
			}
		}
		let pages: Vec<usize> = page_edits(&doc, text_only()).iter().map(|edit| edit.page).collect();
		assert_eq!(pages, vec![1, 2, 3]);
	}

	#[test]
	fn the_two_kinds_of_page_are_asked_for_separately() {
		let mut doc = document();
		let start =
			doc.buffer.pos_at_byte(ContentBytePos::from_raw(doc.buffer.content.find("text of page 2").unwrap()));
		doc.buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, start));

		let scanned: Vec<usize> =
			page_edits(&doc, images_only()).iter().filter(|edit| !edit.whole_page).map(|edit| edit.page).collect();
		assert_eq!(scanned, vec![2], "the scanned page is the only one asked for");

		let with_text: Vec<usize> =
			page_edits(&doc, text_only()).iter().filter(|edit| edit.whole_page).map(|edit| edit.page).collect();
		assert_eq!(with_text, vec![1, 3], "the scanned page is left alone, since it has no text layer");

		let all: Vec<usize> = page_edits(&doc, both()).iter().map(|edit| edit.page).collect();
		assert_eq!(all, vec![1, 2, 3], "both together read every page");
	}

	#[test]
	fn a_format_with_no_bitmap_pages_is_not_swept() {
		assert!(is_ocr_able("book.pdf"));
		assert!(is_ocr_able("book.CBZ"));
		assert!(is_ocr_able("comic.cbr"));
		assert!(!is_ocr_able("book.epub"));
		assert!(!is_ocr_able("notes.txt"));
	}

	/// No newline precedes the first page's instruction line, so a search for one misses it.
	#[test]
	fn the_instruction_on_the_first_page_is_found_too() {
		let mut buffer = DocumentBuffer::new();
		for page in 1..=2 {
			let position = buffer.end();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {page}")));
			buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
			buffer.append(&image_only_placeholder());
			buffer.append("\n");
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		let mut edits = page_edits(&doc, images_only());
		assert_eq!(edits.len(), 2, "both lines should be found: {edits:?}");
		edits[0].text = "read from the first page".to_string();
		let edits: Vec<Edit> =
			edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
		doc.buffer.replace_ranges(edits);
		assert_eq!(doc.buffer.content, format!("read from the first page\n{}\n", image_only_placeholder()));
	}

	/// Inserting in front of the line pushed the page break past the words, so HTML put a scanned page's rule after its text.
	#[test]
	fn the_words_go_where_the_line_was_so_the_page_break_stays_at_the_head_of_the_page() {
		let mut buffer = DocumentBuffer::new();
		let placeholder = image_only_placeholder();
		for page in 1..=2 {
			let position = buffer.end();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {page}")));
			// As the comic parser builds a page: break, artwork and instruction at one position.
			buffer.add_marker(Marker::new(MarkerType::Image, position).with_reference(format!("{page}.png")));
			buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
			buffer.append(&placeholder);
			buffer.append("\n");
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);

		let mut edits = page_edits(&doc, images_only());
		edits[0].text = "balloon one".to_string();
		let edits: Vec<Edit> =
			edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
		doc.buffer.replace_ranges(edits);
		assert_eq!(
			doc.buffer.content,
			format!("balloon one\n{placeholder}\n"),
			"the page it did not read keeps its line"
		);
		let heads: Vec<DocPos> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		assert_eq!(
			heads,
			vec![DocPos::ZERO, DocLen::of("balloon one\n").as_pos()],
			"a page's break has to stay at the head of its own page"
		);
		let html = paperback_core::export::render(
			&paperback_core::document::DocumentHandle::new(doc),
			paperback_core::export::ExportFormat::Html,
		);
		let text_at = html.find("balloon one").unwrap_or(0);
		let rule_at = html.find("<hr").unwrap_or(0);
		assert!(rule_at < text_at, "the page's rule came after its text in the HTML: {html}");
	}

	#[test]
	fn a_page_re_read_gives_up_its_whole_span_and_keeps_the_pages_apart() {
		let mut doc = document();
		let mut edits = page_edits(&doc, text_only());
		for (index, edit) in edits.iter_mut().enumerate() {
			edit.text = format!("words for page {}", index + 1);
		}
		let edits: Vec<Edit> = edits
			.into_iter()
			.map(|edit| Edit { start: edit.start, end: edit.end, text: format!("{}\n", edit.text) })
			.collect();
		doc.buffer.replace_ranges(edits);
		assert_eq!(
			doc.buffer.content, "words for page 1\nwords for page 2\nwords for page 3\n",
			"each page on its own line"
		);
	}

	#[test]
	fn a_page_re_read_does_not_keep_markers_pointing_into_the_words_it_gave_up() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, DocPos::ZERO).with_text("Page 1".to_string()));
		buffer.append("stale text from an older pass\n");
		// Inside the page, since the marker at its head is the page break, which has to survive.
		buffer.add_marker(
			Marker::new(MarkerType::Heading1, DocPos::from_raw(6)).with_level(1).with_text("Stale".to_string()),
		);
		let cut = buffer.end();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, cut).with_text("Page 2".to_string()));
		buffer.append("kept text\n");
		let mut doc = Document::new();
		doc.set_buffer(buffer);

		let mut edits = page_edits(&doc, text_only());
		edits[0].text = "words read from the picture".to_string();
		let edits: Vec<Edit> = edits
			.drain(..1)
			.map(|edit| Edit { start: edit.start, end: edit.end, text: format!("{}\n", edit.text) })
			.collect();
		doc.buffer.replace_ranges(edits);
		assert!(
			doc.buffer.markers.iter().all(|marker| marker.mtype != MarkerType::Heading1),
			"a heading from the text the page gave up survived"
		);
		assert_eq!(doc.buffer.content, "words read from the picture\nkept text\n");
	}

	#[test]
	fn a_document_with_neither_kind_asked_for_is_left_alone() {
		assert!(!Selection::default().any());
		assert!(page_edits(&document(), Selection::default()).is_empty());
		assert_eq!(
			page_edits(&document(), text_only()).len(),
			3,
			"text pages alone take every page of a document that has them"
		);
	}

	#[test]
	fn a_page_the_engine_read_nothing_from_keeps_what_it_had() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, DocPos::ZERO).with_text("Page 1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, DocPos::ZERO));
		buffer.append(&image_only_placeholder());
		buffer.append("\n");
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		assert_eq!(read(&mut doc, images_only()), format!("{}\n", image_only_placeholder()));
	}
}
