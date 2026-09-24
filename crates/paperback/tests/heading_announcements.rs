//! What screen readers are told when navigating by heading.

#![cfg(target_os = "windows")]

mod common;

use common::{Notifications, fixture};
use windows::Win32::UI::Accessibility::NotificationProcessing_ImportantMostRecent as IMPORTANT;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn heading_keys_announce_each_heading_and_the_end_of_the_list() {
	let app = common::launch("heading-announcements");
	common::wait_for_reader_focus(app.pid);
	let notifications = Notifications::listen(app.pid);
	let h1 = format!("{} Heading level 1", fixture::H1);
	let first_h2 = format!("{} Heading level 2", fixture::H2_FIRST);

	common::press('H');
	notifications.wait_for(&h1, IMPORTANT);
	common::press('H');
	notifications.wait_for(&first_h2, IMPORTANT);
	common::press_shifted('H');
	notifications.wait_for(&h1, IMPORTANT);

	common::press('H');
	notifications.wait_for(&first_h2, IMPORTANT);
	common::press('H');
	notifications.wait_for(&format!("{} Heading level 2", fixture::H2_LAST), IMPORTANT);
	common::press('H');
	notifications.wait_for("No next heading.", IMPORTANT);
}
