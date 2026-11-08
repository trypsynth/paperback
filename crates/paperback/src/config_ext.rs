use std::{
	fmt::{self, Display, Formatter},
	fs,
	path::PathBuf,
	str::FromStr,
	sync::OnceLock,
};

use homeport::Homeport;
use logkeeper::Level;
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

/// Debug is what every log was written at before the level could be chosen, so an existing config with no level keeps it.
pub const DEFAULT_LOG_LEVEL: Level = Level::Debug;

fn parse_log_level(name: &str) -> Level {
	name.parse().unwrap_or(DEFAULT_LOG_LEVEL)
}

pub fn get_log_level(config: &ConfigManager) -> Level {
	parse_log_level(&config.get_app_string("log_level", DEFAULT_LOG_LEVEL.as_str()))
}

pub fn set_log_level(config: &ConfigManager, level: Level) {
	config.set_app_string("log_level", &level.to_string());
}

/// Read without a [`ConfigManager`], since initializing one writes a fresh config when the file is missing or unreadable, too early at startup.
pub fn log_level_from_disk() -> Level {
	let text = fs::read_to_string(config_toml_path()).unwrap_or_default();
	let table = text.parse::<toml::Table>().unwrap_or_default();
	let level = table.get("app").and_then(|app| app.get("log_level")).and_then(toml::Value::as_str);
	level.map_or(DEFAULT_LOG_LEVEL, parse_log_level)
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
	use logkeeper::Level;

	use super::parse_log_level;

	#[test]
	fn a_missing_or_unknown_log_level_is_debug() {
		assert_eq!(parse_log_level("loud"), Level::Debug);
		assert_eq!(parse_log_level("Warn"), Level::Warn);
	}
}
