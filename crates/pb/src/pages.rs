//! A page is a `PageBreak` marker, the same definition the app uses; `ParserFlags::SUPPORTS_PAGES` is deliberately unset for presentations, so keying off it would make pb disagree with the app.
//! A specification is parsed before the document is read and resolved against its page count after, which is what lets `80-end` work.

use anyhow::{Result, bail};
use paperback_core::document::{DocLen, DocPos, Document, DocumentBuffer, Marker, MarkerType, TocItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageRange {
	Fixed(usize, usize),
	ToEnd(usize),
}

impl PageRange {
	const fn start(&self) -> usize {
		match *self {
			Self::Fixed(start, _) | Self::ToEnd(start) => start,
		}
	}

	const fn end(&self) -> Option<usize> {
		match *self {
			Self::Fixed(_, end) => Some(end),
			Self::ToEnd(_) => None,
		}
	}

	fn last(&self, page_count: usize) -> usize {
		self.end().unwrap_or(page_count)
	}

	/// `book.p7.md` reads as the page asked for, where `book.p7-7.md` reads as a range typed the long way.
	fn label(&self) -> String {
		match *self {
			Self::Fixed(start, end) if start == end => start.to_string(),
			Self::Fixed(start, end) => format!("{start}-{end}"),
			Self::ToEnd(start) => format!("{start}-end"),
		}
	}

	/// For `8-700` against 417 pages this is 700, not 8, which is there.
	const fn past_page(&self, page_count: usize) -> usize {
		match self.end() {
			Some(end) if end > page_count => end,
			_ => self.start(),
		}
	}

	/// A range running past the last page is refused, not shortened: `5-10` on five pages is a mistyped end, and five pages back would hide that.
	fn resolve(&self, page_count: usize) -> Option<std::ops::RangeInclusive<usize>> {
		if self.start() > page_count || self.last(page_count) > page_count {
			return None;
		}
		Some(self.start()..=self.last(page_count))
	}
}

/// Held as ranges rather than page numbers, because `end` can't become a number until the document is parsed.
#[derive(Debug, Clone)]
pub struct PageSelection(Vec<PageRange>);

impl PageSelection {
	/// Semicolons work as commas, for a shell that makes commas awkward to type.
	pub fn parse(spec: &str) -> Result<Self> {
		let mut ranges = Vec::new();
		for element in spec.split([',', ';']) {
			let element = element.trim();
			if element.is_empty() {
				// The all-empty specification is caught in `resolve`, which has the document to talk about.
				bail!("no page given for the empty part of --pages \"{spec}\"");
			}
			ranges.push(parse_range(element, spec)?);
		}
		Ok(Self(merge(ranges)))
	}

	fn pages(&self, page_count: usize) -> Result<Vec<usize>> {
		let mut selected: Vec<usize> = self
			.0
			.iter()
			.map(|range| {
				range.resolve(page_count).ok_or_else(|| {
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

	pub fn resolve(&self, doc: &Document) -> Result<Vec<usize>> {
		let breaks = page_breaks(doc);
		if breaks.is_empty() {
			bail!("{}", no_pages_message(doc));
		}
		// Checked here rather than left to `pages`, so the message names the document and its real page count.
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

	/// Joined with `_` so `1-5;20` isn't `p1-5-20`, which reads as 1 to 520; built from the merged ranges so `5-10,7-12` and `5-12` name the same file.
	#[must_use]
	pub fn label(&self) -> String {
		self.0.iter().map(PageRange::label).collect::<Vec<_>>().join("_")
	}
}

fn parse_range(element: &str, spec: &str) -> Result<PageRange> {
	let unreadable =
		|| anyhow::anyhow!("cannot read \"{element}\" in --pages \"{spec}\" as a page number or a range of them");
	let zero = || anyhow::anyhow!("page 0 in --pages \"{spec}\" does not exist; pages are numbered from 1");
	let Some((start, end)) = element.split_once('-') else {
		// `+5` and `05` parse but aren't what anyone meant, so the round-trip check rejects them.
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
	// A capitalised format name elsewhere on the command line makes `End` a likely typing.
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

fn parse_page(text: &str) -> Option<usize> {
	let page = text.parse::<usize>().ok()?;
	(text == page.to_string()).then_some(page)
}

fn merge(ranges: Vec<PageRange>) -> Vec<PageRange> {
	let mut sorted = ranges;
	// Which of a `ToEnd` and a `Fixed` at the same start sorts first doesn't matter: the merge keeps the `ToEnd` either way.
	sorted.sort_unstable_by_key(|range| (range.start(), range.end().is_some()));
	let mut merged: Vec<PageRange> = Vec::new();
	for range in sorted {
		// Saturating, since a reader may write the largest page number there is.
		let merged_into_previous = merged
			.last()
			.is_some_and(|previous| previous.end().is_none_or(|end| range.start() <= end.saturating_add(1)));
		if !merged_into_previous {
			merged.push(range);
			continue;
		}
		let previous = merged.pop().expect("checked as present just above");
		merged.push(match (previous, range) {
			(PageRange::ToEnd(_), _) | (_, PageRange::ToEnd(_)) => PageRange::ToEnd(previous.start()),
			(PageRange::Fixed(start, previous_end), PageRange::Fixed(_, end)) => {
				PageRange::Fixed(start, previous_end.max(end))
			}
		});
	}
	merged
}

/// Sorted, since a parser may add its markers in any order and page 5 must be the fifth page, not the fifth marker added.
fn page_breaks(doc: &Document) -> Vec<DocPos> {
	let mut positions: Vec<DocPos> = doc
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

/// This module never sees the file path.
fn describe(doc: &Document) -> String {
	if doc.title.is_empty() { "this document".to_string() } else { format!("\"{}\"", doc.title) }
}

/// Usually an EPUB without a publisher page-list, which the reader can't see from outside, so it says the file was read fine.
fn no_pages_message(doc: &Document) -> String {
	let mut message = format!("{} has no pages to select from", describe(doc));
	if !doc.toc_items.is_empty() {
		message.push_str("; it has a table of contents, but no page numbers");
	}
	message
}

struct Span {
	start: DocPos,
	end: DocPos,
	/// Where `start` landed in the new content.
	offset: DocPos,
	/// Tells two stretches apart from two halves of one continuous run.
	page: usize,
}

/// Every marker, toc entry and id is rebased or dropped: renderers don't crash on stale positions, they produce plausible output that is wrong.
pub fn apply(selection: &PageSelection, doc: &Document) -> Result<Document> {
	let selected = selection.resolve(doc)?;
	let breaks = page_breaks(doc);
	let total = doc.buffer.end();
	let mut content = String::new();
	let mut spans: Vec<Span> = Vec::new();
	for &page in &selected {
		let start = breaks[page];
		let end = breaks.get(page + 1).copied().unwrap_or(total);
		if start >= end {
			continue;
		}
		let page_text = doc.buffer.text(start..end);
		let offset = DocLen::of(&content).as_pos();
		// Adjacent pages already carry the parser's separator, so only a gap gets a break.
		if spans.last().is_some_and(|previous| previous.page + 1 != page) {
			content.push('\n');
		}
		content.push_str(page_text);
		spans.push(Span { start, end, offset, page });
	}
	if spans.is_empty() {
		bail!("no text on the pages asked for");
	}
	// A marker exactly on a page boundary stays with the page it heads.
	let rebase = |position: DocPos| -> Option<DocPos> {
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
		// A marker running past the kept text makes the renderer emit an unopened closing tag or silently blank the tail.
		let length = marker.length.min(span.end - marker.position);
		if !marker.length.is_zero() && length.is_zero() {
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
	// Or `pb --metadata` would print the whole book's counts.
	out.compute_stats();
	Ok(out)
}

/// A dropped entry's children go with it, since their positions mean nothing without their parent.
fn rebase_toc(item: &TocItem, rebase: &impl Fn(DocPos) -> Option<DocPos>) -> Option<TocItem> {
	let mut copy = item.clone();
	copy.offset = rebase(item.offset)?;
	copy.children = item.children.iter().filter_map(|child| rebase_toc(child, rebase)).collect();
	Some(copy)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Enough that no range in these tests runs off the end by accident.
	const PAGES: usize = 200;

	const LAST_PAGE: usize = usize::MAX;

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

	#[test]
	fn spaces_around_the_parts_are_forgiven() {
		assert_eq!(pages(" 5 - 10 , 55 "), vec![4, 5, 6, 7, 8, 9, 54]);
	}

	#[test]
	fn a_semicolon_separates_parts_as_well_as_a_comma() {
		assert_eq!(pages("5;18-22;38"), vec![4, 17, 18, 19, 20, 21, 37]);
		assert_eq!(pages("5-10;55"), vec![4, 5, 6, 7, 8, 9, 54]);
		assert_eq!(pages("5,10;55"), vec![4, 9, 54]);
	}

	#[test]
	fn a_semicolon_is_not_a_way_to_sneak_past_a_rejection() {
		assert!(message("5-10;22-18").contains("starts at page 22"), "{}", message("5-10;22-18"));
		assert!(message("5-10;").contains("5-10;"), "{}", message("5-10;"));
		assert!(message("5;;10").contains("5;;10"), "{}", message("5;;10"));
	}

	#[test]
	fn ranges_are_ordered_by_page_however_they_were_typed() {
		assert_eq!(pages("92-94,5-7,55"), vec![4, 5, 6, 54, 91, 92, 93]);
	}

	#[test]
	fn the_largest_page_number_does_not_wrap_around_when_ranges_are_merged() {
		let selection = PageSelection::parse(&format!("1-{LAST_PAGE},2")).expect("a valid --pages value");
		assert_eq!(selection.0, vec![PageRange::Fixed(1, LAST_PAGE)]);
	}

	#[test]
	fn overlapping_ranges_are_merged_rather_than_repeated() {
		assert_eq!(pages("5-10,8-12"), (4..12).collect::<Vec<_>>());
	}

	#[test]
	fn touching_ranges_are_merged_too() {
		assert_eq!(pages("5-10,11-12"), (4..12).collect::<Vec<_>>());
	}

	#[test]
	fn a_repeated_page_is_selected_once() {
		assert_eq!(pages("7,7,7"), vec![6]);
	}

	#[test]
	fn a_range_to_end_runs_through_to_the_last_page() {
		assert_eq!(pages("80-end"), (79..PAGES).collect::<Vec<_>>());
	}

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

	#[test]
	fn a_reversed_range_says_which_way_round_it_is() {
		let message = message("10-5");
		assert!(message.contains("starts at page 10"), "{message}");
		assert!(message.contains("ends at page 5"), "{message}");
	}

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

	#[test]
	fn a_rejection_quotes_the_part_that_was_wrong() {
		assert!(message("5-10,banana").contains("banana"), "{}", message("5-10,banana"));
	}

	/// The shape every paginated parser produces: a labelled break at the head of each page.
	fn document_of_pages(page_count: usize, label: &str) -> Document {
		let mut buffer = DocumentBuffer::new();
		for page in 0..page_count {
			let position = buffer.end();
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

	/// The app pages presentations by slide, and pb has to agree with it.
	#[test]
	fn slides_are_pages_too() {
		let doc = document_of_pages(3, "Slide");
		assert_eq!(extract("2", &doc), "page2 text\n");
	}

	#[test]
	fn a_range_past_the_end_says_how_many_pages_there_are() {
		let doc = document_of_pages(4, "Page");
		let selection = PageSelection::parse("9-end").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 9 does not exist").to_string();
		assert!(message.contains('9'), "{message}");
		assert!(message.contains("4 pages"), "{message}");
	}

	/// Indexing page 10 of a five-page document once panicked.
	#[test]
	fn a_range_running_off_the_end_is_refused_rather_than_truncated() {
		let doc = document_of_pages(5, "Page");
		for spec in ["5-10", "4-10", "3-99"] {
			let selection = PageSelection::parse(spec).expect("valid");
			let message = apply(&selection, &doc).expect_err("the range runs off the end").to_string();
			assert!(message.contains("5 pages"), "{spec}: {message}");
		}
	}

	/// `8-700` on 417 pages was once refused as "page 8 is past the last page".
	#[test]
	fn the_error_names_the_page_that_is_missing_rather_than_the_one_that_is_not() {
		let doc = document_of_pages(417, "Page");
		let selection = PageSelection::parse("8-700").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 700 does not exist").to_string();
		assert!(message.contains("700"), "{message}");
		assert!(!message.contains("page 8 is past"), "{message}");
		assert!(message.contains("417 pages"), "{message}");

		let selection = PageSelection::parse("900-end").expect("valid");
		let message = apply(&selection, &doc).expect_err("page 900 does not exist").to_string();
		assert!(message.contains("900"), "{message}");
	}

	#[test]
	fn a_range_ending_exactly_on_the_last_page_is_fine() {
		let doc = document_of_pages(5, "Page");
		assert_eq!(extract("3-5", &doc), "page3 text\npage4 text\npage5 text\n");
	}

	#[test]
	fn a_document_with_no_pages_says_so() {
		let mut doc = Document::new().with_title("A Book".to_string());
		doc.set_buffer(DocumentBuffer::with_content("chapter one\nchapter two\n".to_string()));
		doc.toc_items = vec![TocItem::new("One".to_string(), String::new(), DocPos::ZERO)];
		let selection = PageSelection::parse("1-2").expect("valid");
		let message = apply(&selection, &doc).expect_err("there are no pages").to_string();
		assert!(message.contains("no pages"), "{message}");
		assert!(message.contains("table of contents"), "{message}");
	}

	#[test]
	fn the_extract_reports_the_counts_of_the_extract() {
		let doc = document_of_pages(6, "Page");
		let out = extract_document("2-3", &doc);
		assert_eq!(out.buffer.content, "page2 text\npage3 text\n");
		assert_eq!(out.stats.line_count, 2, "the extract counted lines from the whole document");
	}

	/// An emoji is one char but four bytes, so treating positions as bytes only works for ASCII.
	#[test]
	fn an_emoji_on_a_page_does_not_shift_the_pages_after_it() {
		let mut buffer = DocumentBuffer::new();
		for page in 1..=3 {
			let position = buffer.end();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {page}")));
			buffer.append(&format!("\u{1F600} page{page} text\n"));
		}
		let mut doc = Document::new().with_title("Test".to_string());
		doc.set_buffer(buffer);
		let out = extract_document("2-3", &doc);
		assert_eq!(out.buffer.content, "\u{1F600} page2 text\n\u{1F600} page3 text\n");
		let heads: Vec<DocPos> = out
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		assert_eq!(heads, vec![DocPos::ZERO, DocLen::of("\u{1F600} page2 text\n").as_pos()]);
	}

	#[test]
	fn a_marker_starting_on_a_discarded_page_is_dropped() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, DocPos::ZERO).with_text("Page 1".to_string()));
		buffer.append("bold across\n");
		let cut = buffer.end();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, cut).with_text("Page 2".to_string()));
		buffer.append("the rest\n");
		buffer.add_marker(Marker {
			mtype: MarkerType::Bold,
			position: DocPos::ZERO,
			length: cut.from_start() + DocLen::from_raw(4),
			..Marker::new(MarkerType::Bold, DocPos::ZERO)
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

	/// A table marker past the end of the extract made the HTML renderer skip every remaining character.
	#[test]
	fn a_table_from_a_discarded_page_does_not_blank_the_extract() {
		let mut buffer = DocumentBuffer::new();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, DocPos::ZERO).with_text("Page 1".to_string()));
		buffer.append("a\tb\nc\td\n");
		let table_length = DocLen::of("a\tb\nc\td\n");
		buffer.add_marker(
			Marker::new(MarkerType::Table, DocPos::ZERO)
				.with_reference("a\tb\nc\td".to_string())
				.with_length(table_length),
		);
		let cut = buffer.end();
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

	#[test]
	fn no_position_in_the_extract_points_outside_it() {
		let doc = document_of_pages(5, "Page");
		for spec in ["1", "2-3", "1,3,5", "4-end", "1-end"] {
			let out = extract_document(spec, &doc);
			let total = out.buffer.end();
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
