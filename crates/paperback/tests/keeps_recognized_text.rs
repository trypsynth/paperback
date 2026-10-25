#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

use common::fixture;

/// A page read by OCR comes back with the document the next time it is opened, rather than as its placeholder again (#1160).
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop and an OCR language"]
fn a_page_read_by_ocr_is_there_when_the_document_is_opened_again() {
	let has_words = |text: Option<String>| text.is_some_and(|text| text.to_lowercase().contains(fixture::COMIC_WORDS));
	let mut app = common::launch_on("keeps-recognized-text", fixture::write_comic, |config| {
		config.app.restore_previous_documents = true;
	});
	common::wait_for_reader_focus(app.pid);
	assert!(!has_words(common::reader_text(app.pid)), "the page should start as its placeholder");
	common::enter();
	let pid = app.pid;
	common::wait_until(
		Duration::from_secs(30),
		|| has_words(common::reader_text(pid)),
		|| format!("OCR text never arrived: {:?}", common::reader_text(pid)),
	);
	common::alt_f4();
	let status = app.wait_exit(Duration::from_secs(10));
	assert!(status.success(), "app exited with {status:?}");
	let saved = std::fs::read_dir(app.config_dir().join("ocr")).map_or(0, Iterator::count);
	assert_eq!(saved, 1, "the page should be kept for the next time");
	let app = app.relaunch();
	common::wait_for_reader_focus(app.pid);
	assert!(
		has_words(common::reader_text(app.pid)),
		"the page should come back read: {:?}",
		common::reader_text(app.pid)
	);
	assert!(app.logs().contains("put back the pages ocr read before"), "the text should have come from what was kept");
}
