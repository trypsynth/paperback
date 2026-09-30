/// Desktop-specific configuration helpers that do not belong in `paperback-core`.
///
/// This module owns:
/// - `UpdateChannel`, the desktop auto-update channel selector.
/// - `config_toml_path()`, Windows/installer-aware path resolution for the TOML config file.
/// - `get_update_channel` / `set_update_channel`, typed helpers wrapping the generic string API.
use std::{
	env,
	fmt::{self, Display, Formatter},
	fs,
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

/// How much Paperback writes to its log, from nothing at all to everything it can say.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LogLevel {
	Off,
	Error,
	Warn,
	Info,
	#[default]
	Debug,
	Trace,
}

impl LogLevel {
	/// In the order the Options dialog lists them.
	pub const ALL: [Self; 6] = [Self::Off, Self::Error, Self::Warn, Self::Info, Self::Debug, Self::Trace];

	/// The filter for this level. Dependencies are held at info from Debug up, since below that they bury Paperback's own lines; Debug is the level every log was written at before it could be chosen.
	pub const fn filter(self) -> &'static str {
		match self {
			Self::Off => "off",
			Self::Error => "error",
			Self::Warn => "warn",
			Self::Info => "info",
			Self::Debug => "info,paperback=debug,paperback_core=debug",
			Self::Trace => "info,paperback=trace,paperback_core=trace",
		}
	}
}

impl Display for LogLevel {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Off => "off",
			Self::Error => "error",
			Self::Warn => "warn",
			Self::Info => "info",
			Self::Debug => "debug",
			Self::Trace => "trace",
		})
	}
}

impl FromStr for LogLevel {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::ALL.into_iter().find(|level| level.to_string().eq_ignore_ascii_case(s)).ok_or(())
	}
}

pub fn get_log_level(config: &ConfigManager) -> LogLevel {
	config.get_app_string("log_level", "debug").parse().unwrap_or_default()
}

pub fn set_log_level(config: &ConfigManager, level: LogLevel) {
	config.set_app_string("log_level", &level.to_string());
}

/// The log level straight from `Paperback.toml`, for logging to start with before a [`ConfigManager`] exists. Read without one because initializing a manager writes a fresh config when the file is missing or unreadable, which is not something to do before the app has even started.
pub fn log_level_from_disk() -> LogLevel {
	let text = fs::read_to_string(config_toml_path()).unwrap_or_default();
	let table = text.parse::<toml::Table>().unwrap_or_default();
	let level = table.get("app").and_then(|app| app.get("log_level")).and_then(toml::Value::as_str);
	level.and_then(|level| level.parse().ok()).unwrap_or_default()
}

/// Returns the directory where Paperback stores its config and log files.
///
/// `$PAPERBACK_CONFIG_DIR`, when set, overrides every rule below.
/// On macOS app bundles: `~/Library/Application Support/Paperback/`.
/// On Windows installer builds: `%APPDATA%\Paperback\`.
/// On Linux, only when running from an AppImage (`$APPIMAGE` set, the same check
/// `linux_integration.rs` uses): `$XDG_CONFIG_HOME/Paperback` (default `~/.config/Paperback`).
/// An AppImage mounts itself read-only at a fresh temp path on every launch, so the
/// exe-directory convention below is never writable and never persists between runs there;
/// a portable Linux build (the plain extracted tar.gz binary, no `$APPIMAGE`) has no such
/// problem and uses it like every other portable build.
/// Otherwise: the directory containing the executable (portable convention).
pub fn config_dir() -> PathBuf {
	if let Some(dir) = env::var_os("PAPERBACK_CONFIG_DIR") {
		let dir = PathBuf::from(dir);
		let _ = fs::create_dir_all(&dir);
		return dir;
	}
	#[cfg(target_os = "linux")]
	if env::var_os("APPIMAGE").is_some() {
		let dir = xdg_config_home().join("Paperback");
		let _ = fs::create_dir_all(&dir);
		return dir;
	}
	let exe_dir = get_exe_directory();
	#[cfg(target_os = "macos")]
	if is_app_bundle(&exe_dir)
		&& let Some(home) = env::var_os("HOME")
	{
		let dir = PathBuf::from(home).join("Library/Application Support/Paperback");
		let _ = fs::create_dir_all(&dir);
		return dir;
	}
	#[cfg(target_os = "windows")]
	{
		let is_installed = (0..10).any(|i| exe_dir.join(format!("unins{i:03}.exe")).exists());
		if is_installed && let Some(appdata) = env::var_os("APPDATA") {
			let dir = PathBuf::from(appdata).join("Paperback");
			let _ = fs::create_dir_all(&dir);
			return dir;
		}
	}
	exe_dir
}

/// `$XDG_CONFIG_HOME`, falling back to `~/.config` per the XDG Base Directory spec.
#[cfg(target_os = "linux")]
fn xdg_config_home() -> PathBuf {
	if let Some(dir) = env::var_os("XDG_CONFIG_HOME") {
		return PathBuf::from(dir);
	}
	let home = env::var_os("HOME").unwrap_or_else(|| ".".into());
	PathBuf::from(home).join(".config")
}

/// Returns the path to `Paperback.toml`.
pub fn config_toml_path() -> PathBuf {
	config_dir().join("Paperback.toml")
}

#[cfg(target_os = "macos")]
fn is_app_bundle(exe_dir: &Path) -> bool {
	exe_dir.components().any(|c| c.as_os_str().to_string_lossy().ends_with(".app"))
}

fn get_exe_directory() -> PathBuf {
	env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
	use super::LogLevel;

	#[test]
	fn every_log_level_reads_back_as_itself() {
		for level in LogLevel::ALL {
			assert_eq!(level.to_string().parse::<LogLevel>(), Ok(level));
		}
	}

	/// Debug is what every log was written at before the level could be chosen, so an existing config with no level keeps it.
	#[test]
	fn a_missing_or_unknown_log_level_is_debug() {
		assert_eq!(LogLevel::default(), LogLevel::Debug);
		assert_eq!("loud".parse::<LogLevel>().unwrap_or_default(), LogLevel::Debug);
	}
}
