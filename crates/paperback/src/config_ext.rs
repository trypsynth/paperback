/// Desktop-specific configuration helpers that do not belong in `paperback-core`.
///
/// This module owns:
/// - `UpdateChannel` — the desktop auto-update channel selector.
/// - `config_toml_path()` — Windows/installer-aware path resolution for the TOML config file.
/// - `get_update_channel` / `set_update_channel` — typed helpers wrapping the generic string API.
use std::{
	env,
	fmt::{self, Display, Formatter},
	path::{Path, PathBuf},
	str::FromStr,
};

use paperback_core::config::ConfigManager;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum UpdateChannel {
	#[default]
	Stable,
	Dev,
}

impl Display for UpdateChannel {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Stable => write!(f, "stable"),
			Self::Dev => write!(f, "dev"),
		}
	}
}

impl FromStr for UpdateChannel {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"stable" => Ok(Self::Stable),
			"dev" => Ok(Self::Dev),
			_ => Err(()),
		}
	}
}

pub fn get_update_channel(config: &ConfigManager) -> UpdateChannel {
	config.get_app_string("update_channel", "stable").parse().unwrap_or_default()
}

pub fn set_update_channel(config: &ConfigManager, channel: UpdateChannel) {
	config.set_app_string("update_channel", &channel.to_string());
}

/// Returns the path to `Paperback.toml`.
///
/// On macOS, if running inside an app bundle (`*.app/Contents/MacOS/`), the config
/// lives in `~/Library/Application Support/Paperback/`. On Windows installer
/// distributions (detected by the presence of `unins000.exe` next to the executable),
/// it lives in `%APPDATA%\Paperback\`. Otherwise it lives next to the executable,
/// which is the portable and non-Windows/non-bundled convention.
pub fn config_toml_path() -> PathBuf {
	let exe_dir = get_exe_directory();
	#[cfg(target_os = "macos")]
	if is_app_bundle(&exe_dir) {
		if let Some(home) = env::var_os("HOME") {
			let config_dir = PathBuf::from(home).join("Library/Application Support/Paperback");
			let _ = std::fs::create_dir_all(&config_dir);
			return config_dir.join("Paperback.toml");
		}
	}
	let is_installed = (0..10).any(|i| exe_dir.join(format!("unins{i:03}.exe")).exists());
	if is_installed && let Some(appdata) = env::var_os("APPDATA") {
		let config_dir = PathBuf::from(appdata).join("Paperback");
		let _ = std::fs::create_dir_all(&config_dir);
		return config_dir.join("Paperback.toml");
	}
	exe_dir.join("Paperback.toml")
}

#[cfg(target_os = "macos")]
fn is_app_bundle(exe_dir: &Path) -> bool {
	exe_dir.components().any(|c| c.as_os_str().to_string_lossy().ends_with(".app"))
}

fn get_exe_directory() -> PathBuf {
	env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
}
