use super::*;
use crate::document::TocItem;

fn segment(segment_type: SegmentTypeFfi) -> StepUnitFfi {
	StepUnitFfi::Segment { segment_type }
}

fn session_with_toc(buffer: DocumentBuffer, toc_items: Vec<TocItem>, parser_flags: ParserFlags) -> DocumentSession {
	let mut doc = Document::new();
	doc.set_buffer(buffer);
	doc.toc_items = toc_items;
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags,
		parse_settings: ParseSettings::default(),
		password: String::new(),
	}
}

fn find(query: &str) -> StepUnitFfi {
	StepUnitFfi::Find { query: query.to_string(), options: SearchOptionsFfi::default() }
}

#[test]
fn next_and_previous_step_by_paragraph() {
	let session = session_with_content("First one.\nSecond one.\nThird one.");
	let next = session.reading_step(0.into(), segment(SegmentTypeFfi::Paragraph), SegmentDirectionFfi::Next);
	assert_eq!(next, ReadingStepFfi { found: true, position: 11.into(), text: "Second one.".to_string() });
	let back = session.reading_step(next.position, segment(SegmentTypeFfi::Paragraph), SegmentDirectionFfi::Previous);
	assert_eq!(back, ReadingStepFfi { found: true, position: 0.into(), text: "First one.".to_string() });
}

#[test]
fn previous_on_the_first_paragraph_is_no_move() {
	let session = session_with_content("First one.\nSecond one.");
	for unit in [SegmentTypeFfi::Paragraph, SegmentTypeFfi::Line] {
		let step = session.reading_step(0.into(), segment(unit), SegmentDirectionFfi::Previous);
		assert!(!step.found, "{unit:?} at the top of the book moved to {step:?}");
		assert_eq!(step.position, DocPos::ZERO);
	}
}

#[test]
fn next_past_the_last_paragraph_is_not_found() {
	let session = session_with_content("First one.\nLast one.");
	let step = session.reading_step(11.into(), segment(SegmentTypeFfi::Paragraph), SegmentDirectionFfi::Next);
	assert!(!step.found);
	assert_eq!(step.position, DocPos::from_raw(11));
}

#[test]
fn a_marker_with_no_text_reads_as_its_section_title() {
	let mut buffer = DocumentBuffer::with_content("Intro.\n \nAfter.".to_string());
	buffer.add_marker(Marker::new(MarkerType::SectionBreak, 7.into()));
	let toc = vec![TocItem::new("Chapter Two".to_string(), String::new(), 7.into())];
	let session = session_with_toc(buffer, toc, ParserFlags::SUPPORTS_SECTIONS);
	let step = session.reading_step(0.into(), segment(SegmentTypeFfi::Section), SegmentDirectionFfi::Next);
	assert_eq!(step, ReadingStepFfi { found: true, position: 7.into(), text: "Chapter Two".to_string() });
}

#[test]
fn find_lands_on_each_match_and_wraps() {
	let session = session_with_content("cat one\ndog\ncat two");
	let first = session.reading_step(0.into(), find("cat"), SegmentDirectionFfi::Current);
	assert_eq!(first, ReadingStepFfi { found: true, position: 0.into(), text: "cat one".to_string() });
	let second = session.reading_step(first.position, find("cat"), SegmentDirectionFfi::Next);
	assert_eq!(second, ReadingStepFfi { found: true, position: 12.into(), text: "cat two".to_string() });
	let wrapped = session.reading_step(second.position, find("cat"), SegmentDirectionFfi::Next);
	assert_eq!(wrapped.position, DocPos::from_raw(0));
	let back = session.reading_step(second.position, find("cat"), SegmentDirectionFfi::Previous);
	assert_eq!(back.position, DocPos::from_raw(0));
}

#[test]
fn find_with_a_single_match_has_nowhere_else_to_go() {
	let session = session_with_content("one cat here");
	let found = session.reading_step(0.into(), find("cat"), SegmentDirectionFfi::Current);
	assert_eq!(found.position, DocPos::from_raw(4));
	assert!(!session.reading_step(found.position, find("cat"), SegmentDirectionFfi::Next).found);
	assert!(!session.reading_step(found.position, find("cat"), SegmentDirectionFfi::Previous).found);
	assert!(!session.reading_step(0.into(), find("dog"), SegmentDirectionFfi::Current).found);
}

#[test]
fn reading_ahead_prepares_the_paragraphs_around_a_line_step() {
	let session = session_with_content("One.\nTwo.\nThree.\nFour.");
	let ahead = session.reading_ahead(5.into());
	assert_eq!(
		ahead,
		ReadingAheadFfi { next: "Three.".to_string(), after_next: "Four.".to_string(), previous: "One.".to_string() }
	);
	assert_eq!(session.reading_ahead(0.into()).previous, "");
	assert_eq!(
		session.reading_ahead(17.into()),
		ReadingAheadFfi { previous: "Three.".to_string(), ..ReadingAheadFfi::default() }
	);
}

#[test]
fn the_section_title_is_the_latest_and_deepest_entry_at_or_before_the_position() {
	let mut part = TocItem::new("Part One".to_string(), String::new(), 10.into());
	part.children = vec![
		TocItem::new("Chapter 1".to_string(), String::new(), 10.into()),
		TocItem::new("Chapter 2".to_string(), String::new(), 30.into()),
	];
	let toc = vec![TocItem::new("Preface".to_string(), String::new(), 0.into()), part];
	let session = session_with_toc(DocumentBuffer::with_content("x".repeat(40)), toc, ParserFlags::empty());
	assert_eq!(session.section_title_at(5.into()), "Preface");
	assert_eq!(session.section_title_at(10.into()), "Chapter 1");
	assert_eq!(session.section_title_at(29.into()), "Chapter 1");
	assert_eq!(session.section_title_at(35.into()), "Chapter 2");
	assert_eq!(session_with_content("x").section_title_at(0.into()), "");
	// Naming the first section here would announce a chapter the reader has not reached.
	let late = vec![TocItem::new("Chapter 1".to_string(), String::new(), 10.into())];
	assert_eq!(
		session_with_toc(DocumentBuffer::with_content("x".repeat(20)), late, ParserFlags::empty())
			.section_title_at(5.into()),
		""
	);
}
