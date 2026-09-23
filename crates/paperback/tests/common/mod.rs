//! Shared harness for UI Automation tests: launches the real binary against a generated
//! fixture, with its config and logs redirected to a temp dir.

#![allow(dead_code)]

pub mod fixture;

use std::{
	path::{Path, PathBuf},
	process::{Child, Command},
	time::{Duration, Instant},
};

use paperback_core::config::ConfigData;
use uiautomation::{UIAutomation, controls::ControlType, core::UIElement};
use windows::{
	Win32::{
		Foundation::CloseHandle,
		System::Threading::{OpenMutexW, SYNCHRONIZATION_SYNCHRONIZE},
	},
	core::w,
};

pub struct App {
	pub child: Child,
	pub pid: u32,
	base: PathBuf,
}

impl App {
	/// Where the app keeps `Paperback.toml` and its logs.
	pub fn config_dir(&self) -> PathBuf {
		self.base.join("config")
	}

	/// The fixture document `launch` wrote.
	pub fn fixture(&self) -> PathBuf {
		self.base.join("fixture").join(fixture::HTML_NAME)
	}

	/// The current run's log.
	pub fn logs(&self) -> String {
		std::fs::read_to_string(self.config_dir().join("paperback.log")).unwrap_or_default()
	}

	/// Polls until the app exits; returns its exit status.
	pub fn wait_exit(&mut self, timeout: Duration) -> std::process::ExitStatus {
		let deadline = Instant::now() + timeout;
		loop {
			if let Some(status) = self.child.try_wait().expect("try_wait") {
				return status;
			}
			assert!(Instant::now() < deadline, "app did not exit within {timeout:?}");
			std::thread::sleep(Duration::from_millis(200));
		}
	}
}

impl Drop for App {
	fn drop(&mut self) {
		let _ = self.child.kill();
		let _ = self.child.wait();
		let _ = std::fs::remove_dir_all(&self.base);
	}
}

/// Launches the app on the generated HTML fixture with the default seeded config.
pub fn launch(test_name: &str) -> App {
	launch_with(test_name, |_| {})
}

/// Launches the app on the generated HTML fixture; `configure` adjusts the seeded config
/// before it is written. `test_name` keys the temp dir.
pub fn launch_with(test_name: &str, configure: impl FnOnce(&mut ConfigData)) -> App {
	let base = std::env::temp_dir().join(format!("paperback-ui-{test_name}-{}", std::process::id()));
	let _ = std::fs::remove_dir_all(&base);
	let fixture = fixture::write_html(&base.join("fixture"));
	let config_dir = base.join("config");
	std::fs::create_dir_all(&config_dir).expect("create config dir");
	let mut config = seeded_config();
	configure(&mut config);
	std::fs::write(config_dir.join("Paperback.toml"), toml::to_string(&config).expect("serialize config"))
		.expect("write config");
	let child = spawn(&config_dir, Some(&fixture));
	let pid = child.id();
	App { child, pid, base }
}

/// The config every launch starts from: no update check, no restored session, English UI.
fn seeded_config() -> ConfigData {
	let mut config = ConfigData::default();
	config.app.restore_previous_documents = false;
	config.app.extra.insert("check_for_updates_on_startup".to_string(), toml::Value::Boolean(false));
	config.app.extra.insert("language".to_string(), toml::Value::String("en".to_string()));
	config
}

fn spawn(config_dir: &Path, document: Option<&Path>) -> Child {
	assert_no_running_instance();
	let mut command = Command::new(env!("CARGO_BIN_EXE_paperback"));
	if let Some(document) = document {
		command.arg(document);
	}
	command
		.env("PAPERBACK_CONFIG_DIR", config_dir)
		.env("RUST_LOG", "paperback=debug")
		.spawn()
		.expect("launch paperback")
}

/// Panics while another Paperback holds the single-instance mutex.
fn assert_no_running_instance() {
	let Ok(mutex) = (unsafe { OpenMutexW(SYNCHRONIZATION_SYNCHRONIZE, false, w!("paperback_running")) }) else {
		return;
	};
	unsafe {
		let _ = CloseHandle(mutex);
	}
	panic!("close Paperback before running the UI tests");
}

fn automation() -> UIAutomation {
	UIAutomation::new().expect("UI Automation unavailable")
}

/// The app's focused element type and name, or None while another process has focus or the
/// window is not up yet.
pub fn focused(pid: u32) -> Option<(ControlType, String)> {
	let element = automation().get_focused_element().ok()?;
	if element.get_process_id().ok()? != pid {
		return None;
	}
	Some((element.get_control_type().ok()?, element.get_name().ok()?))
}

/// The reading text control, the multi-line read-only text control of each tab.
pub fn is_reader(control_type: ControlType, _name: &str) -> bool {
	control_type == ControlType::Document
}

/// Polls the focused element until `expected` matches it; panics on timeout.
pub fn wait_for_focus(
	pid: u32,
	timeout: Duration,
	expected: impl Fn(ControlType, &str) -> bool,
) -> (ControlType, String) {
	let deadline = Instant::now() + timeout;
	let mut last = focused(pid);
	loop {
		if let Some((control_type, name)) = last.as_ref()
			&& expected(*control_type, name)
		{
			return (*control_type, name.clone());
		}
		assert!(Instant::now() < deadline, "expected focus not reached within {timeout:?}; focused element: {last:?}");
		std::thread::sleep(Duration::from_millis(250));
		last = focused(pid);
	}
}

/// Waits until the reading control has focus and still has it a second later.
pub fn wait_for_reader_focus(pid: u32) {
	wait_for_focus(pid, Duration::from_secs(30), is_reader);
	std::thread::sleep(Duration::from_secs(1));
	wait_for_focus(pid, Duration::from_secs(10), is_reader);
}

/// The app's top-level frame.
pub fn main_window(pid: u32) -> UIElement {
	let automation = automation();
	let root = automation.get_root_element().expect("desktop root");
	automation
		.create_matcher()
		.from(root)
		.depth(2)
		.filter_fn(Box::new(move |e: &UIElement| Ok(e.get_process_id()? == pid)))
		.control_type(ControlType::Window)
		.timeout(10_000)
		.find_first()
		.expect("app window")
}

/// Polls the main window's title until it reads `expected`; panics on timeout.
pub fn wait_for_title(pid: u32, expected: &str) {
	let window = main_window(pid);
	let deadline = Instant::now() + Duration::from_secs(10);
	loop {
		let title = window.get_name().unwrap_or_default();
		if title == expected {
			return;
		}
		assert!(Instant::now() < deadline, "title stayed at {title:?}, expected {expected:?}");
		std::thread::sleep(Duration::from_millis(250));
	}
}

/// Polls `condition` until it holds; panics with `describe()` on timeout.
pub fn wait_until(timeout: Duration, mut condition: impl FnMut() -> bool, describe: impl Fn() -> String) {
	let deadline = Instant::now() + timeout;
	loop {
		if condition() {
			return;
		}
		assert!(Instant::now() < deadline, "not reached within {timeout:?}: {}", describe());
		std::thread::sleep(Duration::from_millis(250));
	}
}
