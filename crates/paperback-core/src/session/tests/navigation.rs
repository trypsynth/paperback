use std::{env, fs::File, io::Write};

use zip::{ZipWriter, write::FileOptions};

use super::*;
use crate::{
	config::ConfigManager,
	types::{NavDirection, NavTarget},
};

#[test]
fn first_content_line_after_skips_headers_and_blank_lines() {
	let session = session_with_content("27\n\nChapter One\nThe old house stood at the end of the lane.\n");
	assert_eq!(session.first_content_line_after(0), "Chapter One");
}

#[test]
fn first_content_line_after_skips_bare_page_number_header() {
	let session = session_with_content("27\n\nThe old house stood at the end of the lane.\n");
	assert_eq!(session.first_content_line_after(0), "The old house stood at the end of the lane.");
}

#[test]
fn first_content_line_after_returns_empty_without_content() {
	let session = session_with_content("27\n\n");
	assert_eq!(session.first_content_line_after(0), "");
}

#[test]
fn navigation_result_constructors_have_expected_flags() {
	let not_found = NavigationResult::not_found();
	assert!(!not_found.found);
	assert!(!not_found.not_supported);
	let not_supported = NavigationResult::not_supported();
	assert!(!not_supported.found);
	assert!(not_supported.not_supported);
}

#[test]
fn link_activation_result_not_found_defaults() {
	let result = LinkActivationResult::not_found();
	assert!(!result.found);
	assert_eq!(result.action, LinkAction::NotFound);
	assert_eq!(result.offset, 0);
	assert_eq!(result.url, "");
}

#[test]
fn set_history_clamps_out_of_range_index() {
	let mut session = sample_session(ParserFlags::NONE);
	session.set_history(&[10, 20], 99);
	let (history, index) = session.get_history();
	assert_eq!(history, &[10, 20]);
	assert_eq!(index, 1);
}

#[test]
fn set_history_empty_resets_index_to_zero() {
	let mut session = sample_session(ParserFlags::NONE);
	session.set_history(&[], 99);
	let (history, index) = session.get_history();
	assert!(history.is_empty());
	assert_eq!(index, 0);
}

#[test]
fn check_and_record_history_records_only_after_threshold() {
	let mut session = sample_session(ParserFlags::NONE);
	session.check_and_record_history(100);
	session.check_and_record_history(200);
	session.check_and_record_history(450);
	session.check_and_record_history(900);
	let (history, index) = session.get_history();
	assert_eq!(history, &[100, 450]);
	assert_eq!(index, 1);
}

#[test]
fn nav_helpers_build_expected_request() {
	assert_eq!(DocumentSession::nav_direction(true), NavDirection::Next);
	assert_eq!(DocumentSession::nav_direction(false), NavDirection::Previous);
	let req = DocumentSession::nav_request(7, true, false, NavTarget::Heading, 2);
	assert_eq!(req.position, 7);
	assert!(req.wrap);
	assert_eq!(req.direction, NavDirection::Previous);
	assert_eq!(req.target, NavTarget::Heading);
	assert_eq!(req.level_filter, 2);
}

#[test]
fn navigate_section_returns_not_supported_without_flag() {
	let session = sample_session(ParserFlags::NONE);
	let result = session.navigate_section(0, false, true);
	assert!(!result.found);
	assert!(result.not_supported);
}

#[test]
fn navigate_list_and_list_item_require_support_flag() {
	let session = sample_session(ParserFlags::NONE);
	assert!(session.navigate_list(0, false, true).not_supported);
	assert!(session.navigate_list_item(0, false, true).not_supported);
	let session = sample_session(ParserFlags::SUPPORTS_LISTS);
	assert!(!session.navigate_list(0, false, true).not_supported);
	assert!(!session.navigate_list_item(0, false, true).not_supported);
}

#[test]
fn history_navigation_returns_not_found_when_empty() {
	let mut session = sample_session(ParserFlags::NONE);
	assert!(!session.history_go_back(0).found);
	assert!(!session.history_go_forward(0).found);
}

#[test]
fn history_navigation_updates_index_and_returns_targets() {
	let mut session = sample_session(ParserFlags::NONE);
	session.set_history(&[10, 20, 30], 2);
	let back = session.history_go_back(30);
	assert!(back.found);
	assert_eq!(back.offset, 20);
	let forward = session.history_go_forward(20);
	assert!(forward.found);
	assert_eq!(forward.offset, 30);
}

#[test]
fn navigate_page_returns_found_and_page_marker_index() {
	let session = sample_session(ParserFlags::NONE);
	let result = session.navigate_page(0, false, true);
	assert!(result.found);
	assert!(!result.not_supported);
	assert_eq!(result.offset, 8);
	assert!(result.marker_index >= 0);
}

#[test]
fn navigate_link_returns_found_when_link_exists() {
	let session = sample_session(ParserFlags::NONE);
	let result = session.navigate_link(0, false, true);
	assert!(result.found);
	assert!(!result.not_supported);
	assert_eq!(result.offset, 6);
}

#[test]
fn navigate_table_and_separator_return_found() {
	let session = sample_session(ParserFlags::NONE);
	let table = session.navigate_table(0, false, true);
	assert!(table.found);
	assert_eq!(table.offset, 12);
	let separator = session.navigate_separator(0, false, true);
	assert!(separator.found);
	assert_eq!(separator.offset, 5);
}

#[test]
fn navigate_heading_respects_level_support() {
	let session = sample_session(ParserFlags::NONE);
	let any_level = session.navigate_heading(-1, false, true, 0);
	assert!(!any_level.not_supported);
	assert!(any_level.found);
	let missing_level = session.navigate_heading(-1, false, true, 2);
	assert!(missing_level.not_supported);
	assert!(!missing_level.found);
}

#[test]
fn navigate_section_returns_found_when_flag_enabled() {
	let session = sample_session(ParserFlags::SUPPORTS_SECTIONS);
	let result = session.navigate_section(-1, false, true);
	assert!(result.found);
	assert!(!result.not_supported);
}

/// A plain audio bundle's buffer is one placeholder space per file with no newline anywhere, so
/// the line enclosing any marker is the whole book. Section navigation must therefore announce
/// the marker's own text (the file name), not the line it falls on, or every section reads out
/// the same run of blanks.
#[test]
fn navigate_section_in_an_audio_only_book_announces_the_file_name() {
	let mut buffer = DocumentBuffer::new();
	for name in ["Track 1", "Track 2", "Track 3"] {
		let position = buffer.current_position();
		buffer.append(" ");
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, position).with_text(name.to_string()));
	}
	let mut doc = Document::new().with_title("Some Audiobook".to_string());
	doc.set_buffer(buffer);
	doc.audio_only = true;
	let session = DocumentSession {
		handle: DocumentHandle::new(doc),
		file_path: "Some Audiobook.zip".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::SUPPORTS_SECTIONS,
		last_stable_position: None,
	};
	let first = session.navigate_section(-1, false, true);
	assert!(first.found);
	assert_eq!(first.marker_text, "Track 1");
	let second = session.navigate_section(first.offset, false, true);
	assert!(second.found);
	assert_eq!(second.marker_text, "Track 2");
	let back = session.navigate_section(second.offset, false, false);
	assert!(back.found);
	assert_eq!(back.marker_text, "Track 1");
}

#[test]
fn navigate_bookmark_and_note_return_not_found_with_empty_config() {
	let session = sample_session(ParserFlags::NONE);
	let config = ConfigManager::new();
	assert!(!session.navigate_bookmark(&config, 0, false, true).found);
	assert!(!session.navigate_note(&config, 0, false, true).found);
}

#[test]
fn bookmark_display_at_position_returns_not_found_without_data() {
	let session = sample_session(ParserFlags::NONE);
	let config = ConfigManager::new();
	let display = session.bookmark_display_at_position(&config, 0);
	assert!(!display.found);
	assert_eq!(display.note, "");
	assert_eq!(display.snippet, "");
}

/// The whole path a real audiobook takes: a zip that is nothing but audio files, opened through
/// `DocumentSession::new`, stepped through with the same `navigate_section` the `[` and `]` keys
/// call. Guards the seam the two narrower tests leave open, where the parser stores the file name
/// on the marker but `reader_navigate` decides whether a section's marker text reaches the UI.
#[test]
fn navigate_section_names_each_file_of_a_real_plain_audio_zip() {
	let dir = env::temp_dir().join("paperback-session-plain-audio-section-nav-test");
	std::fs::create_dir_all(&dir).unwrap();
	let zip_path = dir.join("Some Audiobook.zip");
	{
		let file = File::create(&zip_path).unwrap();
		let mut writer = ZipWriter::new(file);
		for entry in ["Track 2.mp3", "Track 10.mp3", "Track 1.mp3"] {
			writer.start_file(entry, FileOptions::<()>::default()).unwrap();
			writer.write_all(b"fake-audio").unwrap();
		}
		writer.finish().unwrap();
	}
	let session = DocumentSession::new(&zip_path.to_string_lossy(), "", "", false).expect("plain audio zip opens");
	let mut announced = Vec::new();
	let mut position = -1;
	for _ in 0..3 {
		let result = session.navigate_section(position, false, true);
		assert!(result.found);
		announced.push(result.marker_text.clone());
		position = result.offset;
	}
	// Natural order, and a distinct name per section rather than the same blank line three times.
	assert_eq!(announced, vec!["Track 1", "Track 2", "Track 10"]);
	assert!(!session.navigate_section(position, false, true).found, "no fourth file to step onto");
}
