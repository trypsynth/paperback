//! Reopening the last session's document on the next launch.

#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

use common::fixture;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn relaunching_restores_the_document_focused_where_reading_stopped() {
	let mut app = common::launch_with("restores-last-document", |config| config.app.restore_previous_documents = true);
	common::wait_for_reader_focus(app.pid);
	common::press('H');
	common::press('H');
	let position = common::wait_for_status_prefix(app.pid, "Line 4, ");

	common::alt_f4();
	let status = app.wait_exit(Duration::from_secs(10));
	assert!(status.success(), "app exited with {status:?}");

	let app = app.relaunch();
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
	common::wait_for_status_prefix(app.pid, &position);
}
