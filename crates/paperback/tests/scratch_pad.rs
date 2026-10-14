#![cfg(target_os = "windows")]

mod common;

use common::{Notifications, fixture};
use windows::Win32::UI::Accessibility::NotificationProcessing_ImportantMostRecent as IMPORTANT;

/// The reader binds bare letters and digits to navigation: `H` to the next heading, `T` to the next
/// table, `P` to the next page, `1` to the next level-1 heading. A scratch pad is a text editor, so
/// in one those are what they look like. The status bar counts the characters the control holds, so
/// a key that navigated instead of typing would leave the count exactly where it was - which is how
/// this catches the menubar's accelerators eating the key before the control ever sees it.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn a_scratch_pad_types_the_keys_the_reader_uses_for_navigation() {
	let app = common::launch("scratch-typing");
	common::wait_for_reader_focus(app.pid);

	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch1 - Paperback");
	common::wait_for_status_prefix(app.pid, "Line 1, Character 1, Reading 0%");

	for key in ['H', 'T', 'P', '1'] {
		common::press(key);
	}
	common::wait_for_status_prefix(app.pid, "Line 1, Character 5, Reading 100%");
}

/// Return has to break the line rather than activate whatever the caret is on, which in the reader
/// is a link or a table.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn return_breaks_the_line_in_a_scratch_pad() {
	let app = common::launch("scratch-return");
	common::wait_for_reader_focus(app.pid);

	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch1 - Paperback");
	for _ in 0..2 {
		common::enter();
	}
	common::wait_for_status_prefix(app.pid, "Line 3, ");
}

/// A second scratch pad counts up rather than reusing a number, and the pair of them behave the
/// same way, so the tab a keystroke reaches is the tab the reader is looking at.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn a_second_scratch_pad_counts_up_and_types_too() {
	let app = common::launch("scratch-numbering");
	common::wait_for_reader_focus(app.pid);

	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch1 - Paperback");
	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch2 - Paperback");
	common::wait_for_status_prefix(app.pid, "Line 1, Character 1, Reading 0%");

	common::press('H');
	common::wait_for_status_prefix(app.pid, "Line 1, Character 2, Reading 100%");
}

/// Select All and Copy say what they did, as they do in a document. Nothing else tells a reader who
/// cannot see the screen that anything happened, and left to the control's own handling the two keys
/// do their work in silence.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn copy_and_select_all_announce_in_a_scratch_pad() {
	let app = common::launch("scratch-copy");
	common::wait_for_reader_focus(app.pid);
	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch1 - Paperback");
	common::type_text("hello");
	common::wait_for_status_prefix(app.pid, "Line 1, Character 6,");

	let notifications = Notifications::listen(app.pid);
	common::press_ctrl('A');
	notifications.wait_for("Selected 5 characters.", IMPORTANT);
	common::press_ctrl('C');
	notifications.wait_for("Copied 5 characters.", IMPORTANT);
}

/// The reader's shortcuts belong to the frame rather than to whichever control has focus, so they
/// reach the document from the tab strip as well. Landing on a heading moves the caret into the
/// document, which shows as focus leaving the tab strip - so this both asks whether the key worked
/// from there and confirms that the tab in front, not the focused control, is what decides it.
///
/// It is also why greying out and shortcut-stripping are driven by the tab in front rather than by
/// where focus is: there is no control here for a key to be typed into, and the same key still has
/// to do the right thing for the document that is showing.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn the_readers_shortcuts_work_from_the_tab_strip() {
	let app = common::launch("scratch-tab-strip");
	common::wait_for_reader_focus(app.pid);

	common::focus_tab_strip(app.pid);
	common::press('H');
	common::wait_for_reader_focus(app.pid);
}

/// The other half of taking the reader's shortcuts off the menu for a scratch pad: they have to come
/// back with the document. `H` twice walks to the second heading, which is where the fixture's
/// second heading is, and on the way back from a scratch pad it only does that if the label the
/// shortcut was taken off has had it put back.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn the_readers_shortcuts_come_back_on_a_document() {
	let app = common::launch("scratch-shortcut-restore");
	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));

	common::press_ctrl('N');
	common::wait_for_title(app.pid, "scratch1 - Paperback");
	common::press_ctrl('1');

	// `H` twice walks to the second heading. In the document that lands on line 4; in the scratch
	// pad the same key would type instead, and the status bar would say so.
	common::press('H');
	common::press('H');
	common::wait_for_status_prefix(app.pid, "Line 4, ");
}
