use std::{
	fmt::{self, Display, Formatter},
	fs,
	path::PathBuf,
	str::FromStr,
	sync::OnceLock,
};

use homeport::Homeport;
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

	/// Dependencies stay at info from Debug up, since below that they bury Paperback's own lines.
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

/// Read without a [`ConfigManager`], since initializing one writes a fresh config when the file is missing or unreadable, too early at startup.
pub fn log_level_from_disk() -> LogLevel {
	let text = fs::read_to_string(config_toml_path()).unwrap_or_default();
	let table = text.parse::<toml::Table>().unwrap_or_default();
	let level = table.get("app").and_then(|app| app.get("log_level")).and_then(toml::Value::as_str);
	level.and_then(|level| level.parse().ok()).unwrap_or_default()
}

/// How Paperback was installed, which decides where its settings live: beside a portable copy, or in the user's config folder for an install, an app bundle or an `AppImage`.
pub fn home() -> &'static Homeport {
	static HOME: OnceLock<Homeport> = OnceLock::new();
	HOME.get_or_init(|| Homeport::new("Paperback").with_config_override("PAPERBACK_CONFIG_DIR"))
}

pub fn config_dir() -> PathBuf {
	home().create_config_dir().unwrap_or_else(|_| home().config_dir())
}

pub fn config_toml_path() -> PathBuf {
	config_dir().join("Paperback.toml")
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
