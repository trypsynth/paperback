#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

use common::fixture;

/// F5 in a document opened from a link downloads it again, rather than reading its working copy,
/// and the reader stays where they were.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn f5_downloads_a_link_document_again_where_reading_stopped() {
	let (app, server) = common::launch_with_link("link-f5", |_| {});
	common::wait_for_reader_focus(app.pid);
	common::press('H');
	common::press('H');
	let position = common::wait_for_status_prefix(app.pid, "Line 4, ");
	assert_eq!(server.requests(), 1);

	common::f5();
	common::wait_until(
		Duration::from_secs(10),
		|| server.requests() == 2,
		|| format!("{} requests", server.requests()),
	);
	common::wait_for_reader_focus(app.pid);
	common::wait_for_status_prefix(app.pid, &position);
}

/// A document opened from a link that was open when Paperback closed is downloaded again when it
/// starts, since it is small.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn relaunching_downloads_a_small_link_document_again() {
	let (mut app, server) =
		common::launch_with_link("link-restore", |config| config.app.restore_previous_documents = true);
	common::wait_for_reader_focus(app.pid);

	common::alt_f4();
	let status = app.wait_exit(Duration::from_secs(10));
	assert!(status.success(), "app exited with {status:?}");

	let app = app.relaunch();
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
	assert_eq!(server.requests(), 2);
}
