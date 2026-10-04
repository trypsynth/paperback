//! Reading the text off pages with a picture on them, by rendering each one and handing the pixels to the platform OCR engine.
//!
//! Two kinds of page are worth this. A scanned page is nothing but a picture, so it has no text to extract at all; the parsers write a line saying "[Image only. Press enter to OCR.]" in its place and mark it, which is what `--ocr-image-pages` looks for. A page whose text layer came from an older or worse OCR pass, or that carries no text for material living only in its images, has text that is no better than no text; `--ocr-text` re-reads those, and covers the scanned pages too, which is the wider sweep the app's "Include text pages too" does.
//!
//! What is read is a rendering of the page, not the text layer it replaces, which is what makes re-reading a page that already has text worth doing at all.
//!
//! The recognized text is put in place of what it replaces rather than in front of it, for the reason given on [`Edit`]: an insertion at a page's start moves every marker sitting there past the words, the page break and the image marker naming the artwork among them, so in HTML the rule for a scanned page comes out after its text.

use anyhow::{Context, Result};
use native_ocr::{Error as OcrError, max_image_dimension, recognize_rgba};
use paperback_core::{
	document::{Document, Edit, MarkerType},
	ocr::{PageRenderer, image_only_placeholder},
	parser::pdf::join_wrapped_lines,
};

/// Whether a file has pages worth rendering for OCR at all.
///
/// Only the three formats whose pages are a picture or a page image can be rendered: there is nothing to rasterize a page of an EPUB with, and reporting that a whole document was read when the reader got an empty text file would be worse than saying there was nothing to read. Matches the app's own gate, so pb and the app agree about which documents Batch OCR applies to.
pub fn is_ocr_able(file_path: &str) -> bool {
	["pdf", "cbz", "cbr"].iter().any(|extension| file_path.to_ascii_lowercase().ends_with(&format!(".{extension}")))
}

/// Which kinds of page to read, from the flags given.
///
/// The two are separate rather than one widening the other, because the kinds are disjoint: a page
/// is either a picture with nothing on it or it carries a text layer, and a reader who wants only
/// one of those has a reason for it. Both flags together are every page, which is the sweep the app
/// offers as one checkbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Selection {
	/// Pages that are a picture and nothing else, found through the instruction the parsers wrote.
	pub image_pages: bool,
	/// Pages carrying a text layer of their own, for one that came from an older or worse OCR pass
	/// or that misses text living only in a page's images.
	pub text_pages: bool,
}

impl Selection {
	/// Whether this asks for any kind of page at all.
	pub const fn any(self) -> bool {
		self.image_pages || self.text_pages
	}
}

/// Reads the text off the pages of `doc` in `selection`, replacing what each page had.
///
/// A document whose renderer cannot be opened reads nothing, as in the app: a page of an EPUB is not a bitmap, so there is nothing to read off one. Reported rather than passed over in silence, because a reader who asked for a whole document to be read and got the original text back has been told nothing.
///
/// Pages are numbered as the document numbers them, so a report of which pages were read and which were not is a report the reader can act on. A page that fails contributes nothing, and says so on stderr, because a page of a scan that silently came out blank is a page the reader would otherwise trust.
///
/// # Errors
///
/// Returns an error if the document cannot be opened for rendering. A page that fails to render or recognize is not an error: it is reported and skipped, so one unreadable page in a thousand-page scan does not throw the other nine hundred and ninety-nine away.
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
	let mut renderer = PageRenderer::open(file_path, password)
		.with_context(|| format!("failed to open {file_path} to read its pages"))?;
	let max_dimension = max_image_dimension();
	let mut failures = Vec::new();
	for edit in &mut edits {
		let page = edit.page;
		match recognize_page(&mut renderer, page, max_dimension) {
			Ok(text) => {
				let text = join_wrapped_lines(text.trim());
				// A page the engine read nothing from keeps what it had. Blanking it would turn an
				// instruction the reader cannot follow into a page that reads as blank, which is a
				// claim about the scan that was never made.
				if !text.is_empty() {
					edit.text = if edit.whole_page { format!("{text}\n") } else { text };
				}
			}
			Err(error) => failures.push((page, error.to_string())),
		}
	}
	let read = edits.iter().filter(|edit| !edit.text.is_empty()).count();
	let pages_wanted = edits.len();
	let edits: Vec<Edit> =
		edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
	doc.buffer.replace_ranges(edits);
	// Reported after the text is in place, so that a run which recognised most of a book still produces the book and says what it missed.
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

/// Renders one 1-based page and recognizes the text on it.
fn recognize_page(renderer: &mut PageRenderer, page: usize, max_dimension: u32) -> Result<String, OcrError> {
	// `render` takes a 0-based page index; the document numbers its pages from one. A page that
	// fell out of the subtraction is a page the document does not have, and pdfium's own bounds
	// check is what refuses it.
	let rendered = renderer
		.render(i32::try_from(page - 1).unwrap_or(-1), max_dimension)
		.map_err(|error| OcrError::Failed(error.to_string()))?;
	recognize_rgba(&rendered.rgba, rendered.width, rendered.height)
}

/// One page's replacement, and which page it belongs to.
///
/// Carries the page number because every edit is filled in before any of them is applied, so the offsets captured here go stale as soon as the first page's text lands. The app re-resolves its spans for the same reason.
#[derive(Debug)]
struct PageEdit {
	page: usize,
	start: usize,
	end: usize,
	/// True for a page whose whole span goes, false for one where only the instruction line does.
	whole_page: bool,
	/// What the page had until the engine says otherwise. Empty once a page has been read, which is
	/// also what a page read as blank looks like, so the two are told apart by the caller rather
	/// than by this field.
	text: String,
}

/// The edits that put each page's words where that page's text was.
///
/// A scanned page gives up only its instruction line, and the newline closing it is left where it
/// was so that the next page does not run into this one. A page that already had text gives up its
/// whole span, newline included, and carries a newline of its own back -- which is also what drops
/// the markers inside it, since a page re-read from its picture has no headings, links or tables to
/// point at. That is the same plain text a scanned page ends up with, so both kinds of page come
/// out the same way.
///
/// The kinds are asked for separately, so a scanned page is left alone by a request for text pages
/// alone and a page with a text layer is left alone by a request for scanned pages alone. Only both
/// together read every page.
fn page_edits(doc: &Document, selection: Selection) -> Vec<PageEdit> {
	let buffer = &doc.buffer;
	let mut breaks: Vec<usize> = buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::PageBreak)
		.map(|marker| marker.position)
		.collect();
	breaks.sort_unstable();
	let scanned: Vec<usize> = buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::ImageOnlyPage)
		.map(|marker| marker.position)
		.collect();
	let total = buffer.total_display_len();
	let mut edits = Vec::new();
	// Every page rather than the scanned ones, since a page number is a page's position among the
	// breaks, and the breaks are what the document numbers its pages by.
	for (index, start) in breaks.iter().copied().enumerate() {
		let page = index + 1;
		let end = breaks.get(index + 1).copied().unwrap_or(total);
		if scanned.contains(&start) {
			if !selection.image_pages {
				continue;
			}
			// The marker sits at the head of its line, which for a page at the very top of the
			// document is the very start of the content and has no newline in front of it to find a
			// start after.
			let byte_start = buffer.byte_index_for_display(start);
			let head = buffer.content[..byte_start].rfind('\n').map_or(0, |offset| offset + 1);
			// The newline is left where it was rather than carried in the replacement.
			let rest = &buffer.content[byte_start..];
			let length = rest.find('\n').unwrap_or(rest.len());
			let line_start = buffer.display_index_for_byte(head);
			let line_end = buffer.display_index_for_byte(byte_start + length);
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
	use paperback_core::document::{DocumentBuffer, Marker};

	use super::*;

	/// Three pages, the second of them a picture with nothing on it.
	fn document() -> Document {
		let mut buffer = DocumentBuffer::new();
		for page in 0..3 {
			let position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", page + 1)));
			buffer.append(&format!("text of page {}\n", page + 1));
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
		// The renderer is never reached, because these documents have nothing to render, so the
		// edits are built and left alone. That is the shape the caller acts on, which is what the
		// page-break ordering and the span choice are about.
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

	/// The page a failure is reported against has to be the page the document numbers it as, or the report points the reader at a page they cannot find.
	#[test]
	fn a_scanned_page_is_reported_by_the_number_the_document_gives_it() {
		let mut doc = document();
		// Put the marker where page 2's text starts.
		let start = doc.buffer.display_index_for_byte(doc.buffer.content.find("text of page 2").unwrap_or(0));
		doc.buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, start));
		let edits = page_edits(&doc, images_only());
		assert_eq!(edits.len(), 1);
		assert_eq!(edits[0].page, 2);
	}

	/// The two flags ask for disjoint kinds of page, so neither is a widening of the other and
	/// either may be given alone. A reader who wants one sort of page has a reason for it, and
	/// re-reading a book that already has a good text layer over a thousand scanned pages is not
	/// what they asked for.
	#[test]
	fn the_two_kinds_of_page_are_asked_for_separately() {
		let mut doc = document();
		let start = doc.buffer.display_index_for_byte(doc.buffer.content.find("text of page 2").unwrap_or(0));
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

	/// A format with no bitmap pages to render is not swept, which is what is_ocr_able decides, so
	/// an EPUB is not reported as a whole document read when it was not.
	#[test]
	fn a_format_with_no_bitmap_pages_is_not_swept() {
		assert!(is_ocr_able("book.pdf"));
		assert!(is_ocr_able("book.CBZ"));
		assert!(is_ocr_able("comic.cbr"));
		assert!(!is_ocr_able("book.epub"));
		assert!(!is_ocr_able("notes.txt"));
	}

	/// A document with the instruction on its first page is the one that breaks: there is no newline in front of that line for its start to be found after, so a reader looking for one concludes the line has no start and leaves it where it is.
	#[test]
	fn the_instruction_on_the_first_page_is_found_too() {
		let mut buffer = DocumentBuffer::new();
		for page in 1..=2 {
			let position = buffer.current_position();
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

	/// The bug this shape was chosen to avoid: a page's own markers have to stay at the head of it. Inserting the words in front of the line instead of in place of it pushes every marker sitting there past the words, and the page's break with them, so an HTML export puts the rule for a scanned page after its text.
	#[test]
	fn the_words_go_where_the_line_was_so_the_page_break_stays_at_the_head_of_the_page() {
		let mut buffer = DocumentBuffer::new();
		let placeholder = image_only_placeholder();
		for page in 1..=2 {
			let position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {page}")));
			// Shaped the way the comic parser builds a page: the break, the artwork and the
			// instruction all at the same position.
			buffer.add_marker(Marker::new(MarkerType::Image, position).with_reference(format!("{page}.png")));
			buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
			buffer.append(&placeholder);
			buffer.append("\n");
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);

		// What the OCR pass does with the first page it read.
		let mut edits = page_edits(&doc, images_only());
		edits[0].text = "balloon one".to_string();
		let edits: Vec<Edit> =
			edits.into_iter().map(|edit| Edit { start: edit.start, end: edit.end, text: edit.text }).collect();
		doc.buffer.replace_ranges(edits);
		assert_eq!(
			doc.buffer.content, "balloon one\n[Image only. Press enter to OCR.]\n",
			"the page it did not read keeps its line"
		);
		let heads: Vec<usize> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		assert_eq!(
			heads,
			vec![0, display_len("balloon one\n")],
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

	/// A page read again gives up its whole span, and the replacement carries a newline, or the
	/// next page's text runs into this one's and the page break lands mid-line.
	#[test]
	fn a_page_re_read_gives_up_its_whole_span_and_keeps_the_pages_apart() {
		let mut doc = document();
		let mut edits = page_edits(&doc, text_only());
		// Every page is read, so every edit carries words: an edit left empty would blank the page
		// it was built for, which is the behaviour being checked somewhere else.
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

	/// A page re-read from its picture has no headings, links or tables to point at, so the markers
	/// inside the span it gave up go with it. Leaving them would hand the renderer coordinates into
	/// text that is no longer there.
	#[test]
	fn a_page_re_read_does_not_keep_markers_pointing_into_the_words_it_gave_up() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 0).with_text("Page 1".to_string()));
		buffer.append("stale text from an older pass\n");
		// Inside the page rather than at its head: a marker at the head is the page break itself,
		// which has to survive for the page to still be a page.
		buffer.add_marker(Marker::new(MarkerType::Heading1, 6).with_level(1).with_text("Stale".to_string()));
		let cut = buffer.current_position();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, cut).with_text("Page 2".to_string()));
		buffer.append("kept text\n");
		let mut doc = Document::new();
		doc.set_buffer(buffer);

		// Only page 1 is read, so page 2's edit is left out rather than blanked.
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

	/// With neither flag there is nothing to do, and asking for nothing must not read the whole book.
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
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 0).with_text("Page 1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, 0));
		buffer.append(&image_only_placeholder());
		buffer.append("\n");
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		// An edit left empty by the caller keeps the line it was built to hold, which is what an
		// unrecognized page looks like: the text was never filled in, so the replacement is the
		// placeholder rather than a blank.
		assert_eq!(read(&mut doc, images_only()), format!("{}\n", image_only_placeholder()));
	}

	fn display_len(text: &str) -> usize {
		paperback_core::util::text::display_len(text)
	}
}
