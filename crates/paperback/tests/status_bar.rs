//! The status bar's position text.

#![cfg(target_os = "windows")]

mod common;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn status_bar_shows_the_caret_line_from_the_moment_a_document_opens() {
	let app = common::launch("status-bar");
	common::wait_for_reader_focus(app.pid);
	common::wait_for_status_prefix(app.pid, "Line 1, Character 1, Reading 0%");

	common::arrow_down();
	common::wait_for_status_prefix(app.pid, "Line 2, ");
}
