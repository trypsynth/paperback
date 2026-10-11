#![cfg(target_os = "windows")]

mod common;

use common::fixture;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn a_link_on_the_command_line_opens_the_document_it_names() {
	let (app, _server) = common::launch_with_link("link-command-line");
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
}

/// The reading position is kept under the link, not under the downloaded copy, which is gone once
/// the tab closes: reopening downloads the document again and finds the position.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn a_link_reopened_after_closing_comes_back_where_reading_stopped() {
	let (app, _server) = common::launch_with_link("link-reopen");
	common::wait_for_reader_focus(app.pid);
	common::press('H');
	common::press('H');
	let position = common::wait_for_status_prefix(app.pid, "Line 4, ");

	common::ctrl_f4();
	common::wait_for_title(app.pid, "Paperback");
	common::press_ctrl_shift('T');
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
	common::wait_for_status_prefix(app.pid, &position);
}

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn open_from_url_opens_the_link_typed_into_it() {
	let (app, server) = common::launch_with_link("link-open-from-url");
	common::wait_for_reader_focus(app.pid);
	common::ctrl_f4();
	common::wait_for_title(app.pid, "Paperback");

	common::press_ctrl('L');
	common::wait_for_focus(app.pid, std::time::Duration::from_secs(10), |_, name| name == "Link:");
	common::press_ctrl('A');
	common::type_text(&server.link());
	common::enter();
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
}
