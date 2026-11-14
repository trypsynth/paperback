use super::*;
use crate::{
	audio::{AudioLocation, AudioTimeline, AudioTimelineBuilder},
	document::TocItem,
};

/// "One two three four" is the first chapter, 0..19, and "five six seven eight nine ten" the second, 19..48.
const TEXT: &str = "One two three four\nfive six seven eight nine ten";

fn session_with(toc_items: Vec<TocItem>, audio: Option<AudioTimeline>, audio_only: bool) -> DocumentSession {
	let mut doc = Document::new();
	doc.set_buffer(DocumentBuffer::with_content(TEXT.to_string()));
	doc.toc_items = toc_items;
	doc.audio_only = audio_only;
	if let Some(audio) = audio {
		doc.set_audio(audio);
	}
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::empty(),
		parse_settings: ParseSettings::default(),
		password: String::new(),
	}
}

fn chapters() -> Vec<TocItem> {
	vec![
		TocItem::new("Chapter 1".to_string(), String::new(), 0.into()),
		TocItem::new("Chapter 2".to_string(), String::new(), 19.into()),
	]
}

/// One clip per chapter, of one and three minutes; the second's length is known only when asked.
fn timeline(lengths_known: bool) -> AudioTimeline {
	let mut builder = AudioTimelineBuilder::new();
	let one = builder.add_source(AudioLocation::File("one.mp3".to_string()), Some(60_000));
	let two = builder.add_source(AudioLocation::File("two.mp3".to_string()), lengths_known.then_some(180_000));
	builder.add_clip(one, 0, 60_000, 0.into(), 19.into());
	builder.add_clip(two, 0, 180_000, 19.into(), 48.into());
	builder.build()
}

#[test]
fn text_chapter_is_the_entry_the_caret_is_in() {
	let session = session_with(chapters(), None, false);
	assert_eq!(session.text_chapter_announcement(0.into()), "Chapter 1, 0%, 4 words left");
	assert_eq!(session.text_chapter_announcement(8.into()), "Chapter 1, 42%, 2 words left");
	assert_eq!(session.text_chapter_announcement(19.into()), "Chapter 2, 0%, 6 words left");
	assert_eq!(session.text_chapter_announcement(48.into()), "Chapter 2, 100%, 0 words left");
}

#[test]
fn one_word_left_is_singular() {
	let session = session_with(chapters(), None, false);
	assert_eq!(session.text_chapter_announcement(14.into()), "Chapter 1, 73%, 1 word left");
}

#[test]
fn text_before_the_first_entry_or_without_any_is_untitled() {
	let late = vec![TocItem::new("Chapter 2".to_string(), String::new(), 19.into())];
	assert_eq!(session_with(late, None, false).text_chapter_announcement(4.into()), "21%, 3 words left");
	assert_eq!(session_with(Vec::new(), None, false).text_chapter_announcement(19.into()), "39%, 6 words left");
}

#[test]
fn a_nested_entry_names_the_chapter_and_ends_its_parent() {
	let mut part = TocItem::new("Part 1".to_string(), String::new(), 0.into());
	part.children = vec![
		TocItem::new("Chapter 1".to_string(), String::new(), 0.into()),
		TocItem::new("Chapter 2".to_string(), String::new(), 19.into()),
	];
	let session = session_with(vec![part], None, false);
	assert_eq!(session.text_chapter_announcement(8.into()), "Chapter 1, 42%, 2 words left");
}

#[test]
fn chapter_progress_is_by_time_where_the_recording_can_be_placed() {
	let session = session_with(chapters(), Some(timeline(true)), false);
	assert_eq!(
		session.chapter_announcement(8.into(), Some(30_000)).as_deref(),
		Some("Chapter 1, 0:30 of 1:00, 0:30 left")
	);
	assert_eq!(session.chapter_announcement(8.into(), None).as_deref(), Some("Chapter 1, 42%, 2 words left"));
}

#[test]
fn chapter_progress_falls_back_to_text_while_a_length_is_unknown() {
	let session = session_with(chapters(), Some(timeline(false)), false);
	assert_eq!(session.chapter_announcement(8.into(), Some(30_000)).as_deref(), Some("Chapter 1, 42%, 2 words left"));
}

#[test]
fn an_audio_only_book_of_unknown_length_has_no_answer() {
	let session = session_with(chapters(), Some(timeline(false)), true);
	assert_eq!(session.chapter_announcement(8.into(), Some(30_000)), None);
	assert_eq!(session.chapter_announcement_ffi(8.into(), 30_000), "");
	assert_eq!(
		session_with(chapters(), None, false).chapter_announcement_ffi(8.into(), -1),
		"Chapter 1, 42%, 2 words left"
	);
}

#[test]
fn chapter_progress_comes_in_parts_by_words_or_by_time() {
	let session = session_with(chapters(), Some(timeline(true)), false);
	assert_eq!(
		session.chapter_progress_ffi(8.into(), -1),
		ChapterProgressFfi {
			found: true,
			title: "Chapter 1".to_string(),
			percent: 42,
			words_left: 2,
			elapsed_ms: -1,
			total_ms: -1,
		}
	);
	assert_eq!(
		session.chapter_progress_ffi(8.into(), 105_000),
		ChapterProgressFfi {
			found: true,
			title: "Chapter 2".to_string(),
			percent: 25,
			words_left: -1,
			elapsed_ms: 45_000,
			total_ms: 180_000,
		}
	);
}

#[test]
fn chapter_progress_in_parts_falls_back_as_the_announcement_does() {
	let by_words = session_with(chapters(), Some(timeline(false)), false).chapter_progress_ffi(8.into(), 30_000);
	assert_eq!((by_words.found, by_words.percent, by_words.words_left), (true, 42, 2));
	let unknown = session_with(chapters(), Some(timeline(false)), true).chapter_progress_ffi(8.into(), 30_000);
	assert!(!unknown.found);
	let untitled = session_with(Vec::new(), None, false).chapter_progress_ffi(19.into(), -1);
	assert_eq!((untitled.title.as_str(), untitled.percent, untitled.words_left), ("", 39, 6));
}
