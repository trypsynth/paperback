#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

/// Reads the scratch pad edit box's text via the Value pattern, so a test can tell what actually
/// made it into the control.
fn scratch_value(pid: u32) -> Result<String, uiautomation::Error> {
	let automation = uiautomation::UIAutomation::new().expect("automation");
	let root = automation.get_root_element().expect("root");
	let window = automation
		.create_matcher()
		.from(root)
		.depth(3)
		.filter_fn(Box::new(move |e: &uiautomation::core::UIElement| {
			Ok(e.get_process_id()? == pid && e.get_name().ok().map(|n| n == "Scratch Pad").unwrap_or(false))
		}))
		.control_type(uiautomation::controls::ControlType::Window)
		.timeout(10_000)
		.find_first()
		.expect("scratch pad window");
	let edit = automation
		.create_matcher()
		.from(window)
		.depth(2)
		.control_type(uiautomation::controls::ControlType::Edit)
		.timeout(10_000)
		.find_first()
		.expect("scratch pad edit box");
	let pattern: uiautomation::patterns::UIValuePattern = edit.get_pattern().expect("value pattern");
	pattern.get_value()
}

/// Waits until the scratch pad window exists (UIA can find the window with our pid and title).
fn wait_for_scratch_window(pid: u32) {
	let deadline = std::time::Instant::now() + Duration::from_secs(10);
	loop {
		if scratch_value(pid).is_ok() {
			return;
		}
		assert!(std::time::Instant::now() < deadline, "scratch pad window never appeared");
		std::thread::sleep(Duration::from_millis(250));
	}
}

/// Waits until the scratch pad window has been hidden (UIA can no longer find it).
fn wait_for_scratch_hidden(pid: u32) {
	let deadline = std::time::Instant::now() + Duration::from_secs(10);
	loop {
		if scratch_value(pid).is_err() {
			return;
		}
		assert!(std::time::Instant::now() < deadline, "scratch pad window never hid");
		std::thread::sleep(Duration::from_millis(250));
	}
}

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn tab_types_a_tab_and_keeps_doing_so_after_the_window_is_closed_and_reopened() {
	let app = common::launch("scratch-tab");
	common::wait_for_reader_focus(app.pid);

	// First open: a tab should insert a tab.
	common::press_ctrl('N');
	wait_for_scratch_window(app.pid);
	common::type_text("a");
	common::press_tab();
	common::type_text("b");
	assert_eq!(scratch_value(app.pid).expect("value"), "a\tb", "first open: Tab should type a tab");

	// Close the scratch pad (Alt+F4 hides it) and reopen it with Ctrl+N, the exact sequence from
	// the bug report.
	common::alt_f4();
	common::press_ctrl('N');
	wait_for_scratch_window(app.pid);
	common::type_text("c");
	common::press_tab();
	common::type_text("d");
	assert_eq!(scratch_value(app.pid).expect("value"), "a\tb\tc\td", "after reopen: Tab should still type a tab");

	// Escape closes it, keeping the text for the next open, exactly like Alt+F4 and Close.
	common::press_escape();
	wait_for_scratch_hidden(app.pid);
	common::press_ctrl('N');
	wait_for_scratch_window(app.pid);
	assert_eq!(scratch_value(app.pid).expect("value"), "a\tb\tc\td", "after Escape+reopen: text should still be there");
}
