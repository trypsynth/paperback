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
