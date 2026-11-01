#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn ctrl_comma_opens_settings() {
	let app = common::launch("ctrl-comma-settings");
	common::wait_for_reader_focus(app.pid);
	common::press_ctrl_punctuation(',');
	assert!(common::window_appears(app.pid, "Settings", Duration::from_secs(10)), "Ctrl+, did not open Settings");
}
