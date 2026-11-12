use std::{env, fs, fs::File, io::Write};

use zip::{ZipWriter, write::FileOptions};

use super::*;
use crate::audio::AudioLocation;

fn session_with_audio(timeline: AudioTimeline) -> DocumentSession {
	let buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
	let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
	doc.set_buffer(buffer);
	doc.set_audio(timeline);
	DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "book.zip".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::empty(),
		parse_settings: ParseSettings::default(),
		password: String::new(),
	}
}

/// The case a character percentage gets wrong: 50% of the text at the boundary, a quarter of the recording.
fn lopsided_timeline(second_duration_known: bool) -> AudioTimeline {
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let short = builder.add_source(AudioLocation::File("one.mp3".to_string()), Some(60_000));
	let long = builder.add_source(
		AudioLocation::File("two.mp3".to_string()),
		if second_duration_known { Some(180_000) } else { None },
	);
	builder.add_clip(short, 0, 60_000, 0.into(), 1.into());
	builder.add_clip(long, 0, 180_000, 1.into(), 2.into());
	builder.build()
}

#[test]
fn audio_progress_follows_the_recording_not_the_text() {
	let session = session_with_audio(lopsided_timeline(true));
	assert_eq!(session.audio_progress_percent(0), Some(0));
	assert_eq!(session.audio_progress_percent(60_000), Some(25));
	assert_eq!(session.audio_progress_percent(120_000), Some(50));
	assert_eq!(session.audio_progress_percent(240_000), Some(100));
}

#[test]
fn audio_progress_reaches_a_hundred_only_at_the_end() {
	let session = session_with_audio(lopsided_timeline(true));
	assert_eq!(session.audio_progress_percent(239_000), Some(99));
}

#[test]
fn audio_elapsed_for_percent_is_the_inverse() {
	let session = session_with_audio(lopsided_timeline(true));
	assert_eq!(session.audio_elapsed_for_percent(0), Some(0));
	assert_eq!(session.audio_elapsed_for_percent(25), Some(60_000));
	assert_eq!(session.audio_elapsed_for_percent(100), Some(240_000));
}

#[test]
fn audio_percent_clamps_out_of_range_input() {
	let session = session_with_audio(lopsided_timeline(true));
	assert_eq!(session.audio_elapsed_for_percent(-10), Some(0));
	assert_eq!(session.audio_elapsed_for_percent(500), Some(240_000));
}

#[test]
fn audio_percent_declines_when_a_duration_is_unknown() {
	let session = session_with_audio(lopsided_timeline(false));
	assert_eq!(session.audio_progress_percent(60_000), None);
	assert_eq!(session.audio_elapsed_for_percent(50), None);
}

#[test]
fn audio_percent_is_none_without_audio() {
	let session = session_with_content(
		"line1
line2",
	);
	assert_eq!(session.audio_progress_percent(0), None);
	assert_eq!(session.audio_elapsed_for_percent(50), None);
}

fn two_source_timeline() -> AudioTimeline {
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let source0 = builder.add_source(AudioLocation::File("chapter1.mp3".to_string()), Some(9000));
	let source1 = builder.add_source(AudioLocation::File("chapter2.mp3".to_string()), Some(4000));
	builder.add_clip(source0, 0, 9000, 0.into(), 10.into());
	builder.add_clip(source1, 0, 4000, 10.into(), 17.into());
	builder.build()
}

#[test]
fn has_audio_ffi_is_false_without_a_timeline() {
	let session = sample_session(ParserFlags::empty());
	assert!(!session.has_audio_ffi());
}

#[test]
fn has_audio_ffi_is_true_with_a_non_empty_timeline() {
	let session = session_with_audio(two_source_timeline());
	assert!(session.has_audio_ffi());
	assert_eq!(session.audio_source_count_ffi(), 2);
	assert_eq!(session.audio_clip_count_ffi(), 2);
}

#[test]
fn audio_clip_ffi_reports_found_and_its_fields() {
	let session = session_with_audio(two_source_timeline());
	let clip = session.audio_clip_ffi(1);
	assert!(clip.found);
	assert_eq!(clip.source, 1);
	assert_eq!(clip.clip_begin_ms, 0);
	assert_eq!(clip.clip_end_ms, 4000);
	assert_eq!(clip.start, 10);
	assert_eq!(clip.end, 17);
}

#[test]
fn audio_clip_ffi_is_not_found_out_of_range() {
	let session = session_with_audio(two_source_timeline());
	assert!(!session.audio_clip_ffi(99).found);
	assert!(!sample_session(ParserFlags::empty()).audio_clip_ffi(0).found);
}

#[test]
fn audio_cursor_at_elapsed_ffi_resolves_the_containing_clip() {
	let session = session_with_audio(two_source_timeline());
	let cursor = session.audio_cursor_at_elapsed_ffi(9500);
	assert!(cursor.found);
	assert_eq!(cursor.clip_index, 1);
	assert_eq!(cursor.seek_ms, 500);
}

#[test]
fn audio_point_for_position_ffi_anchors_to_the_clip_start() {
	let session = session_with_audio(two_source_timeline());
	let point = session.audio_point_for_position_ffi(15.into());
	assert!(point.found);
	assert_eq!(point.position, 10);
	assert_eq!(point.time_ms, 9000);
}

#[test]
fn audio_point_for_position_ffi_is_not_found_in_a_gap() {
	let session = session_with_audio(two_source_timeline());
	assert!(!session.audio_point_for_position_ffi(30.into()).found);
}

#[test]
fn audio_elapsed_for_source_position_ffi_round_trips_and_has_a_sentinel() {
	let session = session_with_audio(two_source_timeline());
	assert_eq!(session.audio_elapsed_for_source_position_ffi(1, 500), 9500);
	assert_eq!(session.audio_elapsed_for_source_position_ffi(9, 0), -1);
}

#[test]
fn audio_next_source_after_ffi_advances_and_has_a_sentinel_at_the_end() {
	let session = session_with_audio(two_source_timeline());
	assert_eq!(session.audio_next_source_after_ffi(0), 1);
	assert_eq!(session.audio_next_source_after_ffi(1), -1);
}

#[test]
fn audio_source_direct_path_ffi_returns_the_path_for_a_file_source_only() {
	let session = session_with_audio(two_source_timeline());
	assert_eq!(session.audio_source_direct_path_ffi(0), "chapter1.mp3");
	assert_eq!(session.audio_source_direct_path_ffi(99), "");
}

#[test]
fn audio_extract_source_ffi_copies_a_file_source() {
	let dir = env::temp_dir().join("paperback-session-audio-extract-test");
	fs::create_dir_all(&dir).unwrap();
	let source_path = dir.join("chapter1.mp3");
	fs::write(&source_path, b"chapter-one-bytes").unwrap();
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let source = builder.add_source(AudioLocation::File(source_path.to_string_lossy().to_string()), None);
	builder.add_clip(source, 0, 1000, 0.into(), 10.into());
	let session = session_with_audio(builder.build());
	let output_path = dir.join("out.mp3");
	assert!(session.audio_extract_source_ffi(0, output_path.to_string_lossy().to_string()));
	assert_eq!(fs::read(&output_path).unwrap(), b"chapter-one-bytes");
}

#[test]
fn audio_extract_source_ffi_extracts_a_zip_entry_source() {
	let dir = env::temp_dir().join("paperback-session-audio-extract-zip-test");
	fs::create_dir_all(&dir).unwrap();
	let zip_path = dir.join("book.zip");
	{
		let file = File::create(&zip_path).unwrap();
		let mut writer = ZipWriter::new(file);
		writer.start_file("chapter1.mp3", FileOptions::<()>::default()).unwrap();
		writer.write_all(b"zipped-chapter-bytes").unwrap();
		writer.finish().unwrap();
	}
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let source = builder.add_source(
		AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "chapter1.mp3".to_string(),
			password: None,
		},
		None,
	);
	builder.add_clip(source, 0, 1000, 0.into(), 10.into());
	let session = session_with_audio(builder.build());
	let output_path = dir.join("out.mp3");
	assert!(session.audio_extract_source_ffi(0, output_path.to_string_lossy().to_string()));
	assert_eq!(fs::read(&output_path).unwrap(), b"zipped-chapter-bytes");
	assert_eq!(session.audio_source_direct_path_ffi(0), "");
}

#[test]
fn audio_extract_source_ffi_extracts_a_password_protected_zip_entry_source() {
	let dir = env::temp_dir().join("paperback-session-audio-extract-encrypted-zip-test");
	fs::create_dir_all(&dir).unwrap();
	let zip_path = dir.join("book.zip");
	{
		let file = File::create(&zip_path).unwrap();
		let mut writer = ZipWriter::new(file);
		let options = FileOptions::<()>::default().with_aes_encryption(zip::AesMode::Aes256, "hunter2");
		writer.start_file("chapter1.mp3", options).unwrap();
		writer.write_all(b"zipped-chapter-bytes").unwrap();
		writer.finish().unwrap();
	}
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let source = builder.add_source(
		AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "chapter1.mp3".to_string(),
			password: Some("hunter2".to_string()),
		},
		None,
	);
	builder.add_clip(source, 0, 1000, 0.into(), 10.into());
	let session = session_with_audio(builder.build());
	let output_path = dir.join("out.mp3");
	assert!(session.audio_extract_source_ffi(0, output_path.to_string_lossy().to_string()));
	assert_eq!(fs::read(&output_path).unwrap(), b"zipped-chapter-bytes");
}

/// Adjacent tracks of a plain audio bundle sit one placeholder character apart however long they run.
fn adjacent_tracks_timeline() -> AudioTimeline {
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let one = builder.add_source(AudioLocation::File("one.mp3".to_string()), Some(60_000));
	let two = builder.add_source(AudioLocation::File("two.mp3".to_string()), Some(60_000));
	builder.add_clip(one, 0, 60_000, 0.into(), 1.into());
	builder.add_clip(two, 0, 60_000, 1.into(), 2.into());
	builder.build()
}

// #890: a jump between tracks minutes apart moves the caret one character, so only audio distance shows it.
#[test]
fn record_jump_uses_audio_distance_when_text_distance_is_too_sparse_to_tell() {
	let mut session = session_with_audio(adjacent_tracks_timeline());
	session.record_jump(0.into(), 1.into());
	let (history, index) = session.get_history();
	assert_eq!(history, &[0]);
	assert_eq!(index, 0);
}

#[test]
fn record_jump_still_ignores_a_short_audio_seek() {
	let mut session = session_with_audio(adjacent_tracks_timeline());
	session.record_jump(0.into(), 0.into());
	let (history, _) = session.get_history();
	assert!(history.is_empty());
}

/// Two chapters, of one and three minutes, each a heading in the text and an entry in the table of contents.
fn session_with_chapters(toc: bool) -> DocumentSession {
	let mut builder = crate::audio::AudioTimelineBuilder::new();
	let one = builder.add_source(AudioLocation::File("one.mp3".to_string()), Some(60_000));
	let two = builder.add_source(AudioLocation::File("two.mp3".to_string()), Some(180_000));
	builder.add_clip(one, 0, 60_000, 0.into(), 6.into());
	builder.add_clip(two, 0, 180_000, 6.into(), 12.into());
	let mut session = session_with_audio(builder.build());
	if toc {
		let mut document = session.handle.document().clone();
		document.toc_items = vec![
			crate::document::TocItem::new("Chapter 1".to_string(), String::new(), 0.into()),
			crate::document::TocItem::new("Chapter 2".to_string(), String::new(), 6.into()),
		];
		session.handle = DocumentHandle::new(document);
	}
	session
}

#[test]
fn audio_time_is_of_the_whole_recording() {
	let session = session_with_chapters(true);
	assert_eq!(session.audio_time_announcement(0).as_deref(), Some("0:00 of 4:00"));
	assert_eq!(session.audio_time_announcement(90_500).as_deref(), Some("1:30 of 4:00"));
	assert_eq!(session.audio_time_announcement(999_000).as_deref(), Some("4:00 of 4:00"));
}

#[test]
fn audio_time_waits_for_every_length() {
	let session = session_with_audio(lopsided_timeline(false));
	assert_eq!(session.audio_time_announcement(0), None);
	assert_eq!(session.audio_chapter_announcement(0), None);
	assert_eq!(session.audio_time_announcement_ffi(0), "");
}

#[test]
fn chapter_time_is_of_the_chapter_playing() {
	let session = session_with_chapters(true);
	assert_eq!(session.audio_chapter_announcement(30_000).as_deref(), Some("Chapter 1, 0:30 of 1:00, 0:30 left"));
	assert_eq!(session.audio_chapter_announcement(60_000).as_deref(), Some("Chapter 2, 0:00 of 3:00, 3:00 left"));
	assert_eq!(session.audio_chapter_announcement(150_000).as_deref(), Some("Chapter 2, 1:30 of 3:00, 1:30 left"));
}

#[test]
fn a_book_without_chapters_is_one() {
	let session = session_with_chapters(false);
	assert_eq!(session.audio_chapter_announcement(150_000).as_deref(), Some("2:30 of 4:00, 1:30 left"));
}

#[test]
fn go_to_time_entries_resolve_through_the_session() {
	let session = session_with_chapters(true);
	assert!(session.audio_time_known_ffi());
	assert_eq!(session.audio_time_entry_ffi("1:30".to_string(), 0), 90_000);
	assert_eq!(session.audio_time_entry_ffi("+30".to_string(), 60_000), 90_000);
	assert_eq!(session.audio_time_entry_ffi("soon".to_string(), 0), -1);
	assert_eq!(session.audio_time_entry_ffi("5:00".to_string(), 0), -2);
	assert_eq!(session.format_audio_time_ffi(3_723_000), "1:02:03");
	assert_eq!(session_with_audio(lopsided_timeline(false)).audio_time_entry_ffi("1:00".to_string(), 0), -3);
}
