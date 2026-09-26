//! Selecting part of a document by page, for `pb --pages`.
//!
//! A page is not a thing `paperback-core` knows about. It is a [`MarkerType::PageBreak`] sitting
//! at a position in the flat text buffer, and which parsers emit one varies: a PDF's pages are
//! real, a presentation's are slides, an RTF's are the `\page` breaks in the source, an EPUB's
//! come from the publisher's page-list and are absent from most books, and a Markdown file has
//! none at all. This module works off those markers, which is the same definition `paperback.exe`
//! uses -- `DocumentSession::navigate_page` gates on `has_marker(PageBreak)` and `page_count`
//! counts them, neither consulting `ParserFlags::SUPPORTS_PAGES`. Keying off that flag instead
//! would have made `pb` disagree with the app it ships beside, since it is deliberately unset for
//! presentations.
//!
//! Working off the markers rather than off the file extension also means no format has to be
//! special-cased. An EPUB that carries a page-list has real printed page numbers and is selected
//! from them; one that does not has no markers at all, and says so rather than handing back the
//! whole book as though the request had been honoured.
//!
//! Because a document's length is not known until it has been read, a specification is parsed
//! into ranges first and resolved against the real page count afterwards. That is what lets
//! `--pages 80-end` mean "through to the last page" without the reader counting first.

use anyhow::{Result, bail};
use paperback_core::document::{Document, DocumentBuffer, Marker, MarkerType, TocItem};

/// What one element of a `--pages` specification asks for, before the page count is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageRange {
	/// An inclusive pair of 1-based page numbers, as in `5-10`.
	Fixed(usize, usize),
	/// From a 1-based page through to the last one, as in `80-end`.
	ToEnd(usize),
}

impl PageRange {
	/// The first page asked for, which decides whether a range can be satisfied at all.
	const fn start(&self) -> usize {
		match *self {
			Self::Fixed(start, _) | Self::ToEnd(start) => start,
		}
	}

	/// The last page asked for, or `None` when the range runs to the end of the document.
	const fn end(&self) -> Option<usize> {
		match *self {
			Self::Fixed(_, end) => Some(end),
			Self::ToEnd(_) => None,
		}
	}

	/// The last page covered in a document of `page_count` pages.
	fn last(&self, page_count: usize) -> usize {
		self.end().unwrap_or(page_count)
	}

	/// The first page of this range that a document of `page_count` pages does not have, which is
	/// what an error should name.
	///
	/// For `8-700` against a 417-page document that is 700 rather than 8, because 8 is there and
	/// saying otherwise points the reader at the wrong end of their own command.
	fn past_page(&self, page_count: usize) -> usize {
		match self.end() {
			Some(end) if end > page_count => end,
			_ => self.start(),
		}
	}

	/// The 1-based pages this range covers, or `None` when any part of it falls outside a document
	/// of `page_count` pages.
	///
	/// A range that starts inside the document and ends past its last page is refused too, not
	/// quietly shortened: `--pages 5-10` on a five-page file is a mistyped end, and returning five
	/// pages would leave the reader believing they had the other five.
	fn resolve(&self, page_count: usize) -> Option<std::ops::RangeInclusive<usize>> {
		if self.start() > page_count || self.last(page_count) > page_count {
			return None;
		}
		Some(self.start()..=self.last(page_count))
	}
}

/// Every page a `--pages` specification asks for, in document order.
///
/// Held as merged ranges rather than a list of page numbers, because `end` cannot be turned into
/// a number until the document has been parsed.
#[derive(Debug, Clone)]
pub struct PageSelection(Vec<PageRange>);

impl PageSelection {
	/// Reads a `--pages` value such as `5-10,55,80-end`.
	///
	/// Parts are separated by a comma or a semicolon, and the two may be mixed, so a shell that
	/// makes commas awkward to type can use `5;18-22;38` and get the same reading of it.
	///
	/// Pages are 1-based, matching both the page numbers printed in a PDF and the labels the
	/// parsers write onto their own page-break markers, so page 0 does not exist.
	///
	/// # Errors
	///
	/// Returns an error naming the offending part of the specification if an element is empty,
	/// reversed, zero, or not a page range at all.
	pub fn parse(spec: &str) -> Result<Self> {
		let mut ranges = Vec::new();
		for element in spec.split([',', ';']) {
			let element = element.trim();
			if element.is_empty() {
				// Reachable via a stray or doubled comma. The all-empty specification is caught
				// by the emptiness check in `resolve`, which has the document to talk about.
				bail!("no page given for the empty part of --pages \"{spec}\"");
			}
			ranges.push(parse_range(element, spec)?);
		}
		Ok(Self(merge(ranges)))
	}

	/// The pages asked for, as 0-based indexes, in a document of `page_count` pages.
	///
	/// A range reaching past the last page is an error rather than a shorter range: `--pages 5-10`
	/// on a five-page file is a reader who mistyped the end, and quietly handing back five pages
	/// would leave them believing they had the other five.
	///
	/// # Errors
	///
	/// Returns an error if any range starts past the last page. `end` resolves to the last page, so
	/// this is only meaningful once the count is known.
	fn pages(&self, page_count: usize) -> Result<Vec<usize>> {
		let mut selected: Vec<usize> = self
			.0
			.iter()
			.map(|range| {
				range
					.resolve(page_count)
					// The page named is the one the reader wrote that does not exist. For `8-700` on
					// a 417-page file that is 700: page 8 is there, and being told otherwise sends
					// them looking for a problem in the wrong end of their command.
					.ok_or_else(|| {
						anyhow::anyhow!("page {} is past the last page of {page_count}", range.past_page(page_count))
					})
			})
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.flatten()
			.collect();
		selected.sort_unstable();
		selected.dedup();
		Ok(selected.into_iter().map(|page| page - 1).collect())
	}

	/// The selected pages as 0-based indexes into a document's page-break markers, ascending.
	///
	/// # Errors
	///
	/// Returns an error if the document has no pages at all, or if a range starts past its last
	/// page -- naming the real count, since a reader who asked for `80-end` on a 12-page file is
	/// owed that `12` more than they are owed an empty document.
	pub fn resolve(&self, doc: &Document) -> Result<Vec<usize>> {
		let breaks = page_breaks(doc);
		if breaks.is_empty() {
			bail!("{}", no_pages_message(doc));
		}
		// Checked here rather than left to `pages`, so that the document is named in the message a
		// reader actually sees. The page named is the one they wrote that does not exist.
		for range in &self.0 {
			if range.resolve(breaks.len()).is_none() {
				bail!(
					"page {} is past the last page; {} has {} {}",
					range.past_page(breaks.len()),
					describe(doc),
					breaks.len(),
					page_word(breaks.len())
				);
			}
		}
		self.pages(breaks.len())
	}
}

/// Reads one element of a `--pages` specification: `7`, `5-10`, or `80-end`.
fn parse_range(element: &str, spec: &str) -> Result<PageRange> {
	let unreadable =
		|| anyhow::anyhow!("cannot read \"{element}\" in --pages \"{spec}\" as a page number or a range of them");
	let zero = || anyhow::anyhow!("page 0 in --pages \"{spec}\" does not exist; pages are numbered from 1");
	let Some((start, end)) = element.split_once('-') else {
		// A bare number, as in `55`. `+5` and `05` parse but are not what anyone meant to type,
		// so the round-trip check rejects them rather than silently accepting a second spelling.
		let page = element.parse::<usize>().map_err(|_| unreadable())?;
		if element != page.to_string() {
			return Err(unreadable());
		}
		if page == 0 {
			return Err(zero());
		}
		return Ok(PageRange::Fixed(page, page));
	};
	let (start, end) = (start.trim(), end.trim());
	let start = parse_page(start).ok_or_else(unreadable)?;
	if start == 0 {
		return Err(zero());
	}
	// Matched case-insensitively because a capitalised format name elsewhere in the command line
	// makes `End` a likely typing.
	if end.eq_ignore_ascii_case("end") {
		return Ok(PageRange::ToEnd(start));
	}
	let end = parse_page(end).ok_or_else(unreadable)?;
	if end == 0 {
		return Err(zero());
	}
	if end < start {
		bail!("--pages \"{spec}\" starts at page {start} but ends at page {end}, which is before it");
	}
	Ok(PageRange::Fixed(start, end))
}

/// One page number, rejecting anything that is not written the way a page number is.
fn parse_page(text: &str) -> Option<usize> {
	let page = text.parse::<usize>().ok()?;
	(text == page.to_string()).then_some(page)
}

/// Sorts the ranges and merges those that overlap or touch, so that no page is selected twice and
/// the selection comes out in document order however it was typed.
fn merge(ranges: Vec<PageRange>) -> Vec<PageRange> {
	let mut sorted = ranges;
	// A `ToEnd` range sorts after a `Fixed` one starting at the same page, because it reaches
	// further, and so has to be the one that survives the merge.
	sorted.sort_unstable_by_key(|range| (range.start(), range.end().is_some()));
	let mut merged: Vec<PageRange> = Vec::new();
	for range in sorted {
		// A range already running to the last page swallows everything after it, and nothing
		// sorts beyond it.
		let merged_into_previous =
			merged.last().is_some_and(|previous| previous.end().is_none_or(|end| range.start() <= end + 1));
		if !merged_into_previous {
			merged.push(range);
			continue;
		}
		let previous = merged.pop().expect("checked as present just above");
		// Whichever of the two reached further wins; a `ToEnd` on either side keeps `ToEnd`.
		merged.push(match (previous, range) {
			(PageRange::ToEnd(_), _) | (_, PageRange::ToEnd(_)) => PageRange::ToEnd(previous.start()),
			(PageRange::Fixed(start, previous_end), PageRange::Fixed(_, end)) => {
				PageRange::Fixed(start, previous_end.max(end))
			}
		});
	}
	merged
}

/// The display-unit positions of every page-break marker, in document order.
///
/// Sorted rather than taken in buffer order, because a parser may add its markers in any order
/// and page 5 has to be the fifth page of the book, not the fifth marker added.
fn page_breaks(doc: &Document) -> Vec<usize> {
	let mut positions: Vec<usize> = doc
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::PageBreak)
		.map(|marker| marker.position)
		.collect();
	positions.sort_unstable();
	positions
}

const fn page_word(page_count: usize) -> &'static str {
	if page_count == 1 { "page" } else { "pages" }
}

/// How to refer to the document in an error, given that this module never sees the file path.
fn describe(doc: &Document) -> String {
	if doc.title.is_empty() { "this document".to_string() } else { format!("\"{}\"", doc.title) }
}

/// Why a document has nothing to select.
///
/// The common case is an EPUB, whose pages come from a page-list the publisher may simply not have
/// supplied -- and which the reader cannot see from the outside. Saying what is missing, rather
/// than only that something is, is what tells them the file was read fine and the request was the
/// problem.
fn no_pages_message(doc: &Document) -> String {
	let mut message = format!("{} has no pages to select from", describe(doc));
	// A document with chapters but no pages is the EPUB case, and the chapters are what the reader
	// would reach for instead.
	if !doc.toc_items.is_empty() {
		message.push_str("; it has a table of contents, but no page numbers");
	}
	message
}

/// A kept stretch of the original document, and where it ended up.
///
/// `start`/`end` are display-unit positions in the original; `offset` is how far the start of this
/// span sits from the start of the new content. Together they translate any position in the
/// original into a position in the extract, which is what every marker and toc offset needs.
/// `page` is the 0-based page it came from, kept so that two stretches can be told apart from two
/// halves of one continuous run.
struct Span {
	start: usize,
	end: usize,
	offset: usize,
	page: usize,
}

/// Cuts `doc` down to the pages in `selection`.
///
/// The result is a whole, self-consistent [`Document`]: its buffer is rebuilt from the sliced
/// text, and every position carried by a marker, a table-of-contents entry or an id is rebased to
/// match, or dropped when it falls outside. Handing the renderers positions that no longer point
/// where they used to does not crash them -- both bound their cursors by the content and skip what
/// they cannot reach -- so the failure would be output that looks plausible and is wrong, which is
/// the failure mode this function exists to prevent.
///
/// # Errors
///
/// Returns an error if the selection cannot be satisfied, as described on
/// [`PageSelection::resolve`], or if the selected pages hold no text at all.
pub fn apply(selection: &PageSelection, doc: &Document) -> Result<Document> {
	let selected = selection.resolve(doc)?;
	let breaks = page_breaks(doc);
	let total = doc.buffer.total_display_len();

	// Each page runs from its own page-break marker to the next one, and the last page runs to the
	// end of the content. Two adjacent pages selected together still produce two spans, so that
	// the separator between them is added the same way whatever was asked for.
	let mut content = String::new();
	let mut spans: Vec<Span> = Vec::new();
	for &page in &selected {
		let start = breaks[page];
		let end = breaks.get(page + 1).copied().unwrap_or(total);
		if start >= end {
			continue;
		}
		let byte_start = doc.buffer.byte_index_for_display(start);
		let byte_end = doc.buffer.byte_index_for_display(end);
		let offset = display_len(&content);
		// Only a page the reader left out needs a break before the next one. Pages asked for
		// together were adjacent in the document and already carry whatever separator the parser
		// gave them, so adding one here would open a blank line between every page of a
		// continuous run. Compared against the last page actually emitted, which is the only
		// neighbour whose adjacency matters.
		if spans.last().is_some_and(|previous| previous.page + 1 != page) {
			content.push('\n');
		}
		content.push_str(&doc.buffer.content[byte_start..byte_end]);
		spans.push(Span { start, end, offset, page });
	}
	if spans.is_empty() {
		bail!("no text on the pages asked for");
	}

	// The new position of something that used to sit at `position`, or `None` when it fell in a
	// part of the document that is not in the extract. The end of a span belongs to the span
	// before it, so a marker sitting exactly on a page boundary stays with the page it heads.
	let rebase = |position: usize| -> Option<usize> {
		spans
			.iter()
			.find(|span| position >= span.start && position < span.end)
			.or_else(|| spans.last().filter(|span| position == span.end))
			.map(|span| span.offset + (position - span.start))
	};

	let mut buffer = DocumentBuffer::with_content(content);
	for marker in &doc.buffer.markers {
		let Some(position) = rebase(marker.position) else { continue };
		let Some(span) = spans.iter().find(|span| marker.position >= span.start && marker.position < span.end) else {
			continue;
		};
		// A marker whose span runs past the end of the kept text would have the renderer emit a
		// closing tag with no opening one, or blank the tail of the extract entirely. Both are
		// silent, so the length is trimmed to what survived and the marker dropped if nothing did.
		let length = marker.length.min(span.end - marker.position);
		if marker.length != 0 && length == 0 {
			continue;
		}
		buffer.add_marker(Marker {
			mtype: marker.mtype,
			position,
			text: marker.text.clone(),
			reference: marker.reference.clone(),
			level: marker.level,
			length,
		});
	}

	let mut out = Document::new().with_title(doc.title.clone()).with_author(doc.author.clone());
	out.toc_items = doc.toc_items.iter().filter_map(|item| rebase_toc(item, &rebase)).collect();
	out.id_positions =
		doc.id_positions.iter().filter_map(|(id, position)| rebase(*position).map(|at| (id.clone(), at))).collect();
	out.spine_items.clone_from(&doc.spine_items);
	out.manifest_items.clone_from(&doc.manifest_items);
	out.audio_only = doc.audio_only;
	out.set_buffer(buffer);
	// The word and line counts `pb --metadata` prints are derived from the content, so they have
	// to be re-derived from the extract rather than left describing the whole book.
	out.compute_stats();
	Ok(out)
}

/// Keeps a table-of-contents entry and its subtree if it lands in the extract, rebasing each
/// offset. An entry pointing outside is dropped along with its children, since a parent that is
/// not in the extract makes its children's positions meaningless to follow.
fn rebase_toc(item: &TocItem, rebase: &impl Fn(usize) -> Option<usize>) -> Option<TocItem> {
	let mut copy = item.clone();
	copy.offset = rebase(item.offset)?;
	copy.children = item.children.iter().filter_map(|child| rebase_toc(child, rebase)).collect();
	Some(copy)
}

/// The display width of `text`, which is what a position in the extract is counted in.
///
/// Uses the same helper `DocumentBuffer` uses to index content, so a length measured here and one
/// measured there are the same number rather than two that agree only on ASCII.
fn display_len(text: &str) -> usize {
	paperback_core::util::text::display_len(text)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Enough pages that no range in these tests runs off the end by accident.
	const PAGES: usize = 200;

	fn pages(spec: &str) -> Vec<usize> {
		PageSelection::parse(spec).expect("a valid --pages value").pages(PAGES).expect("the range fits the document")
	}

	fn message(spec: &str) -> String {
		PageSelection::parse(spec).expect_err("the specification cannot be read").to_string()
	}

	#[test]
	fn a_single_page_selects_that_page() {
		assert_eq!(pages("5"), vec![4]);
		assert_eq!(pages("1"), vec![0]);
	}

	#[test]
	fn a_range_selects_inclusive_of_both_ends() {
		assert_eq!(pages("5-10"), (4..10).collect::<Vec<_>>());
	}

	#[test]
	fn several_ranges_are_all_selected() {
		assert_eq!(pages("5-7,55,92-94"), vec![4, 5, 6, 54, 91, 92, 93]);
	}

	/// The commas and spaces a reader types around the parts should not become the error.
	#[test]
	fn spaces_around_the_parts_are_forgiven() {
		assert_eq!(pages(" 5 - 10 , 55 "), vec![4, 5, 6, 7, 8, 9, 54]);
	}

	/// A semicolon separates parts as well as a comma, for a shell that makes commas awkward, and
	/// the two may be mixed because a reader who has started with one is not thereby forbidden the
	/// other.
	#[test]
	fn a_semicolon_separates_parts_as_well_as_a_comma() {
		assert_eq!(pages("5;18-22;38"), vec![4, 17, 18, 19, 20, 21, 37]);
		assert_eq!(pages("5-10;55"), vec![4, 5, 6, 7, 8, 9, 54]);
		assert_eq!(pages("5,10;55"), vec![4, 9, 54]);
	}

	#[test]
	fn a_semicolon_is_not_a_way_to_sneak_past_a_rejection() {
		// A reversed range is a reversed range whichever separator introduced it, and an element
		// left empty by a trailing semicolon is as empty as one left by a trailing comma.
		assert!(message("5-10;22-18").contains("starts at page 22"), "{}", message("5-10;22-18"));
		assert!(message("5-10;").contains("5-10;"), "{}", message("5-10;"));
		assert!(message("5;;10").contains("5;;10"), "{}", message("5;;10"));
	}

	#[test]
	fn ranges_are_ordered_by_page_however_they_were_typed() {
		assert_eq!(pages("92-94,5-7,55"), vec![4, 5, 6, 54, 91, 92, 93]);
	}

	/// `--pages 5-10,8-12` asks for two overlapping stretches. Selecting 8-10 twice would print
	/// those pages twice in the output, which reads as a fault in the document rather than in the
	/// command.
	#[test]
	fn overlapping_ranges_are_merged_rather_than_repeated() {
		assert_eq!(pages("5-10,8-12"), (4..12).collect::<Vec<_>>());
	}

	#[test]
	fn touching_ranges_are_merged_too() {
		assert_eq!(pages("5-10,11-12"), (4..12).collect::<Vec<_>>());
	}

	/// A page is a page whether it was named once or three times.
	#[test]
	fn a_repeated_page_is_selected_once() {
		assert_eq!(pages("7,7,7"), vec![6]);
	}

	#[test]
	fn a_range_to_end_runs_through_to_the_last_page() {
		assert_eq!(pages("80-end"), (79..PAGES).collect::<Vec<_>>());
	}

	/// A fixed range and one running to the end are the same request once they meet.
	#[test]
	fn a_fixed_range_and_one_to_end_merge_into_the_longer() {
		assert_eq!(pages("9-10,8-end"), (7..PAGES).collect::<Vec<_>>());
		assert_eq!(pages("8-end,9-10"), (7..PAGES).collect::<Vec<_>>());
	}

	#[test]
	fn end_is_read_in_any_case() {
		assert!(PageSelection::parse("80-END").is_ok());
		assert!(PageSelection::parse("80-End").is_ok());
	}

	/// A reversed range is the mistake a reader is most likely to make, and quietly treating it
	/// as an empty selection would hide it.
	#[test]
	fn a_reversed_range_says_which_way_round_it_is() {
		let message = message("10-5");
		assert!(message.contains("starts at page 10"), "{message}");
		assert!(message.contains("ends at page 5"), "{message}");
	}

	/// Page 0 is the one number guaranteed never to exist, and resolving it to nothing would hand
	/// back an empty document instead of an explanation.
	#[test]
	fn page_zero_is_refused_rather_than_resolved_to_nothing() {
		assert!(message("0").contains("numbered from 1"), "{}", message("0"));
		assert!(message("0-5").contains("numbered from 1"), "{}", message("0-5"));
		assert!(message("5-0").contains("numbered from 1"), "{}", message("5-0"));
	}

	#[test]
	fn an_empty_element_names_the_specification_it_came_from() {
		assert!(message("5-10,,55").contains("5-10,,55"), "{}", message("5-10,,55"));
	}

	#[test]
	fn text_that_is_not_a_range_is_refused() {
		for spec in ["five", "5-x", "5-", "-10", "5..10", "5 10", "+5", "007"] {
			assert!(PageSelection::parse(spec).is_err(), "{spec} was accepted");
		}
	}

	/// The reader who typed the command needs to see which part of it was wrong, and a batch of
	/// conversions gives them no other way to tell which file is being complained about.
	#[test]
	fn a_rejection_quotes_the_part_that_was_wrong() {
		assert!(message("5-10,banana").contains("banana"), "{}", message("5-10,banana"));
	}

	// --- the document half ---

	/// A document of `page_count` pages, each holding one line of its own text, with a page-break
	/// marker at the head of every page. This is the shape every paginated parser produces: the
	/// PDF parser labels its markers "Page N" and the presentation parsers "Slide N", and both
	/// place one at the start of each page.
	fn document_of_pages(page_count: usize, label: &str) -> Document {
		let mut buffer = DocumentBuffer::new();
		for page in 0..page_count {
			let position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("{label} {}", page + 1)));
			buffer.append(&format!("page{} text\n", page + 1));
		}
		let mut doc = Document::new().with_title("Test".to_string());
		doc.set_buffer(buffer);
		doc
	}

	fn extract_document(spec: &str, doc: &Document) -> Document {
		let selection = PageSelection::parse(spec).expect("a valid --pages value");
		apply(&selection, doc).expect("the selection can be satisfied")
	}

	fn extract(spec: &str, doc: &Document) -> String {
		extract_document(spec, doc).buffer.content
	}

	#[test]
	fn a_single_page_comes_back_on_its_own() {
		let doc = document_of_pages(5, "Page");
		assert_eq!(extract("3", &doc), "page3 text\n");
	}

	#[test]
	fn a_range_of_pages_comes_back_in_order() {
		let doc = document_of_pages(5, "Page");
		assert_eq!(extract("2-4", &doc), "page2 text\npage3 text\npage4 text\n");
	}

	/// Two stretches asked for separately are two extracts. Running the last line of one into the
	/// first of the next would read as a single broken paragraph.
	#[test]
	fn pages_that_are_not_adjacent_are_separated() {
		let doc = document_of_pages(6, "Page");
		assert_eq!(extract("1,4", &doc), "page1 text\n\npage4 text\n");
	}

	#[test]
	fn a_range_reaching_the_last_page_includes_it() {
		let doc = document_of_pages(4, "Page");
		assert_eq!(extract("3-end", &doc), "page3 text\npage4 text\n");
	}

	#[test]
	fn the_final_page_is_reachable_by_number() {
		let doc = document_of_pages(4, "Page");
		assert_eq!(extract("4", &doc), "page4 text\n");
	}

	/// Presentations page by slide in the GUI, and their parsers label the markers "Slide N". pb
	/// has to agree with the app it ships beside.
	#[test]
	fn slides_are_pages_too() {
		let doc = document_of_pages(3, "Slide");
		assert_eq!(extract("2", &doc), "page2 text\n");
	}

	/// A range past the end has to say how long the document actually is, or the reader is left
	/// with an empty file and no idea why.
	#[test]
	fn a_range_past_the_end_says_how_many_pages_there_are() {
		let doc = document_of_pages(4, "Page");
		let selection = PageSelection::parse("9-end").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 9 does not exist").to_string();
		assert!(message.contains('9'), "{message}");
		assert!(message.contains("4 pages"), "{message}");
	}

	/// `--pages 5-10` on a five-page file starts inside the document and runs off the end of it.
	/// Treating that as "the pages that are there" would hand back five pages and leave the reader
	/// believing they had the other five -- and indexing page 10 of a five-page document panicked.
	#[test]
	fn a_range_running_off_the_end_is_refused_rather_than_truncated() {
		let doc = document_of_pages(5, "Page");
		for spec in ["5-10", "4-10", "3-99"] {
			let selection = PageSelection::parse(spec).expect("valid");
			let message = apply(&selection, &doc).expect_err("the range runs off the end").to_string();
			assert!(message.contains("5 pages"), "{spec}: {message}");
		}
	}

	/// The page an error names is the one the reader wrote that does not exist. `--pages 8-700` on
	/// a 417-page file used to be refused as "page 8 is past the last page", which is false: page 8
	/// is there, and 700 is what is not.
	#[test]
	fn the_error_names_the_page_that_is_missing_rather_than_the_one_that_is_not() {
		let doc = document_of_pages(417, "Page");
		let selection = PageSelection::parse("8-700").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 700 does not exist").to_string();
		assert!(message.contains("700"), "{message}");
		assert!(!message.contains("page 8 is past"), "{message}");
		assert!(message.contains("417 pages"), "{message}");

		// A range that starts past the end has only the one page to name.
		let selection = PageSelection::parse("900-end").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 900 does not exist").to_string();
		assert!(message.contains("900"), "{message}");
	}

	/// The route a PDF takes, which counts its pages before reading any of them, has to name the
	/// same page and the same document as the route that slices a parsed one.
	#[test]
	fn both_routes_refuse_a_missing_page_the_same_way() {
		let selection = PageSelection::parse("8-700").expect("valid");
		let fast = wanted_pages(&selection, 417, "py.pdf").expect_err("page 700 does not exist").to_string();
		assert!(fast.contains("700"), "{fast}");
		assert!(fast.contains("py.pdf"), "{fast}");
		assert!(fast.contains("417 pages"), "{fast}");

		let doc = document_of_pages(417, "Page");
		let slow = apply(&selection, &doc).expect_err("page 700 does not exist").to_string();
		// The slow route names the document by its title, the fast one by its path, so only the
		// page and the count are expected to match.
		assert!(slow.contains("700"), "{slow}");
		assert!(slow.contains("417 pages"), "{slow}");
	}

	/// The last page by number is in range, however the range was written to reach it.
	#[test]
	fn a_range_ending_exactly_on_the_last_page_is_fine() {
		let doc = document_of_pages(5, "Page");
		assert_eq!(extract("3-5", &doc), "page3 text\npage4 text\npage5 text\n");
	}

	/// The case that decides whether a format is selectable at all: a document with chapters but
	/// no page numbers has nothing to select, and saying so beats handing back the whole book as
	/// though the request had been honoured. An EPUB whose publisher supplied no page-list lands
	/// here, since its pages come from metadata the reader cannot see.
	#[test]
	fn a_document_with_no_pages_says_so() {
		let mut doc = Document::new().with_title("A Book".to_string());
		doc.set_buffer(DocumentBuffer::with_content("chapter one\nchapter two\n".to_string()));
		doc.toc_items = vec![TocItem::new("One".to_string(), String::new(), 0)];
		let selection = PageSelection::parse("1-2").expect("valid");
		let message = apply(&selection, &doc).expect_err("there are no pages").to_string();
		assert!(message.contains("no pages"), "{message}");
		assert!(message.contains("table of contents"), "{message}");
	}

	/// The counts `pb --metadata` prints come from the content, so they have to describe the
	/// extract rather than the whole book.
	#[test]
	fn the_extract_reports_the_counts_of_the_extract() {
		let doc = document_of_pages(6, "Page");
		let out = extract_document("2-3", &doc);
		assert_eq!(out.buffer.content, "page2 text\npage3 text\n");
		assert_eq!(out.stats.line_count, 2, "the extract counted lines from the whole document");
	}

	/// Markers carry positions in display units, which on Windows and macOS are UTF-16 code units.
	/// An emoji is one char, two UTF-16 units and three UTF-8 bytes, so treating a position as a
	/// byte offset gives the right answer for every ASCII document and the wrong one here.
	#[test]
	fn an_emoji_on_a_page_does_not_shift_the_pages_after_it() {
		let mut buffer = DocumentBuffer::new();
		for page in 1..=3 {
			let position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {page}")));
			buffer.append(&format!("\u{1F600} page{page} text\n"));
		}
		let mut doc = Document::new().with_title("Test".to_string());
		doc.set_buffer(buffer);
		let out = extract_document("2-3", &doc);
		assert_eq!(out.buffer.content, "\u{1F600} page2 text\n\u{1F600} page3 text\n");
		// The page-break markers still sit at the head of their own pages, which is only true if
		// every position was measured in the units the buffer indexes by.
		let heads: Vec<usize> = out
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		assert_eq!(heads, vec![0, display_len("\u{1F600} page2 text\n")]);
	}

	/// A marker that starts on a discarded page would otherwise have the renderer close a span it
	/// never opened.
	#[test]
	fn a_marker_starting_on_a_discarded_page_is_dropped() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 0).with_text("Page 1".to_string()));
		buffer.append("bold across\n");
		let cut = buffer.current_position();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, cut).with_text("Page 2".to_string()));
		buffer.append("the rest\n");
		// A bold run begun on page 1 and running into page 2, which is what a heading split across
		// a page boundary looks like.
		buffer.add_marker(Marker {
			mtype: MarkerType::Bold,
			position: 0,
			length: cut + 4,
			..Marker::new(MarkerType::Bold, 0)
		});
		let mut doc = Document::new().with_title("Test".to_string());
		doc.set_buffer(buffer);
		let out = extract_document("2", &doc);
		assert_eq!(out.buffer.content, "the rest\n");
		assert!(
			out.buffer.markers.iter().all(|marker| marker.mtype != MarkerType::Bold),
			"a bold marker from the discarded page survived"
		);
	}

	/// A table marker left pointing past the end of the extract makes the HTML renderer skip every
	/// remaining character, quietly blanking the tail. Dropping it is the fix.
	#[test]
	fn a_table_from_a_discarded_page_does_not_blank_the_extract() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 0).with_text("Page 1".to_string()));
		buffer.append("a\tb\nc\td\n");
		let table_length = display_len("a\tb\nc\td\n");
		buffer.add_marker(
			Marker::new(MarkerType::Table, 0).with_reference("a\tb\nc\td".to_string()).with_length(table_length),
		);
		let cut = buffer.current_position();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, cut).with_text("Page 2".to_string()));
		buffer.append("kept text\n");
		let mut doc = Document::new().with_title("Test".to_string());
		doc.set_buffer(buffer);
		let out = extract_document("2", &doc);
		assert_eq!(out.buffer.content, "kept text\n");
		let html = paperback_core::export::render(
			&paperback_core::document::DocumentHandle::new(out),
			paperback_core::export::ExportFormat::Html,
		);
		assert!(html.contains("kept text"), "the extract lost its text: {html}");
	}

	/// Every position a sliced document hands on has to be inside that document, or the renderers
	/// and the table of contents are being handed coordinates from a different book.
	#[test]
	fn no_position_in_the_extract_points_outside_it() {
		let doc = document_of_pages(5, "Page");
		for spec in ["1", "2-3", "1,3,5", "4-end", "1-end"] {
			let out = extract_document(spec, &doc);
			let total = out.buffer.total_display_len();
			for marker in &out.buffer.markers {
				assert!(marker.position <= total, "{spec}: {marker:?} starts past the end of {total}");
				assert!(marker.position + marker.length <= total, "{spec}: {marker:?} runs past the end of {total}");
			}
			for item in &out.toc_items {
				assert!(item.offset <= total, "{spec}: {item:?} points past the end of {total}");
			}
		}
	}

	#[test]
	fn every_page_is_the_whole_document() {
		let doc = document_of_pages(3, "Page");
		assert_eq!(extract("1-end", &doc), doc.buffer.content);
	}

	#[test]
	fn a_document_with_one_page_can_still_be_taken_whole() {
		let doc = document_of_pages(1, "Page");
		assert_eq!(extract("1", &doc), "page1 text\n");
		assert_eq!(extract("1-end", &doc), "page1 text\n");
	}
}

/// The 0-based pages a `--pages` specification asks for, for handing to a parser that can read
/// only those.
///
/// This is the same selection [`apply`] would make, worked out before the document is read. A
/// parser that honours it returns just those pages, already carrying their own page numbers, so
/// the extract is the same one `apply` would have produced -- arrived at without reading the rest.
///
/// # Errors
///
/// Returns an error if the document is shorter than the pages asked for, or has none at all. The
/// message is worded as [`PageSelection::resolve`] words it, so that a reader gets the same
/// sentence whichever route their document took.
pub fn wanted_pages(selection: &PageSelection, page_count: usize, name: &str) -> Result<Vec<usize>> {
	if page_count == 0 {
		bail!("{name} has no pages to select from");
	}
	// Checked here for the same reason, so that the page named is the one the reader wrote and
	// that does not exist, and the document is named alongside it.
	for range in selection.0.iter() {
		if range.resolve(page_count).is_none() {
			bail!(
				"page {} is past the last page; {name} has {page_count} {}",
				range.past_page(page_count),
				page_word(page_count)
			);
		}
	}
	selection.pages(page_count)
}
