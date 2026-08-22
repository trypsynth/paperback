//! The serializable config data model: per-session view types ([`Bookmark`],
//! [`NavigationHistory`], [`FindSettings`], [`ReadabilityFont`]), their on-disk TOML
//! shapes ([`StoredBookmark`], [`AppSettings`], [`DocumentConfig`]), and [`ConfigData`],
//! the root structure [`super::manager::ConfigManager`] reads and writes as a whole.
//! Behavior (persistence, per-document lookups) lives in `manager`; this module only
//! defines what the data looks like and its defaults.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shortcuts::{HotkeyConfig, ShortcutsConfig};

const CONFIG_VERSION: u32 = 4;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i64 = 25;

#[derive(Clone, Debug, Default)]
pub struct Bookmark {
	pub start: i64,
	pub end: i64,
	pub note: String,
}

#[derive(Clone, Debug, Default)]
pub struct NavigationHistory {
	pub positions: Vec<i64>,
	pub index: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FindSettings {
	pub match_case: bool,
	pub whole_word: bool,
	pub use_regex: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadabilityFont {
	pub face_name: String,
	pub point_size: i32,
	pub style: i32,
	pub weight: i32,
	pub underlined: bool,
	pub strikethrough: bool,
	/// RGB color packed as 0xRRGGBB, or -1 for default (no override)
	pub color: i32,
	/// Platform-specific font encoding identifier; 0 means default. Ignored on
	/// platforms that manage encoding themselves (e.g. Android/iOS Unicode rendering).
	pub encoding: i32,
}

impl Default for ReadabilityFont {
	fn default() -> Self {
		Self {
			face_name: String::new(),
			point_size: 0,
			style: 0,
			weight: 0,
			underlined: false,
			strikethrough: false,
			color: -1,
			encoding: 0,
		}
	}
}

impl ReadabilityFont {
	#[must_use]
	pub const fn is_default(&self) -> bool {
		self.face_name.is_empty() && self.point_size == 0
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StoredBookmark {
	pub start: i64,
	pub end: i64,
	#[serde(default)]
	pub note: String,
}

const fn default_true() -> bool {
	true
}
const fn default_recent_documents_to_show() -> i64 {
	DEFAULT_RECENT_DOCUMENTS_TO_SHOW
}
const fn default_sleep_timer() -> i64 {
	30
}
const fn default_reading_speed_wpm() -> i64 {
	150
}
const fn default_font_color() -> i64 {
	-1
}
const fn default_bg_color() -> i64 {
	-1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
	#[serde(default = "default_true")]
	pub restore_previous_documents: bool,
	#[serde(default)]
	pub word_wrap: bool,
	#[serde(default = "default_true")]
	pub render_tables_inline: bool,
	#[serde(default)]
	pub navigation_wrap: bool,
	#[serde(default)]
	pub find_match_case: bool,
	#[serde(default)]
	pub find_whole_word: bool,
	#[serde(default)]
	pub find_use_regex: bool,
	#[serde(default = "default_recent_documents_to_show")]
	pub recent_documents_to_show: i64,
	#[serde(default = "default_sleep_timer")]
	pub sleep_timer_duration: i64,
	#[serde(default = "default_reading_speed_wpm")]
	pub reading_speed_wpm: i64,
	#[serde(default)]
	pub font_face_name: String,
	#[serde(default)]
	pub font_point_size: i64,
	#[serde(default)]
	pub font_style: i64,
	#[serde(default)]
	pub font_weight: i64,
	#[serde(default)]
	pub font_underlined: bool,
	#[serde(default)]
	pub font_strikethrough: bool,
	#[serde(default = "default_font_color")]
	pub font_color: i64,
	#[serde(default = "default_bg_color")]
	pub bg_color: i64,
	#[serde(default)]
	pub text_alignment: i64,
	#[serde(default)]
	pub letter_spacing: i64,
	#[serde(default)]
	pub paragraph_spacing: i64,
	#[serde(default)]
	pub line_spacing: i64,
	#[serde(default)]
	pub hotkey: HotkeyConfig,
	#[serde(default)]
	pub shortcuts: ShortcutsConfig,
	/// Pass-through storage for host-specific settings (e.g. desktop UI preferences).
	/// Keys written here are preserved on read/write so host consumers can store their
	/// own fields alongside the generic ones without conflict.
	#[serde(flatten, default)]
	pub extra: HashMap<String, toml::Value>,
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			restore_previous_documents: true,
			word_wrap: false,
			render_tables_inline: true,
			navigation_wrap: false,
			find_match_case: false,
			find_whole_word: false,
			find_use_regex: false,
			recent_documents_to_show: DEFAULT_RECENT_DOCUMENTS_TO_SHOW,
			sleep_timer_duration: 30,
			reading_speed_wpm: 150,
			font_face_name: String::new(),
			font_point_size: 0,
			font_style: 0,
			font_weight: 0,
			font_underlined: false,
			font_strikethrough: false,
			font_color: -1,
			bg_color: -1,
			text_alignment: 0,
			letter_spacing: 0,
			paragraph_spacing: 0,
			line_spacing: 0,
			hotkey: HotkeyConfig::default(),
			shortcuts: ShortcutsConfig::default(),
			extra: HashMap::new(),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentConfig {
	pub path: String,
	#[serde(default)]
	pub last_position: i64,
	/// Elapsed milliseconds into a recorded narration, for documents that have one. Kept
	/// separately from `last_position` because the caret and the audio are independent axes:
	/// resuming an audiobook should return to what was last *heard*, which a text position can
	/// only approximate to the nearest clip. `None` for documents with no audio.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub audio_time_ms: Option<u64>,
	#[serde(default)]
	pub navigation_history: Vec<i64>,
	#[serde(default)]
	pub navigation_history_index: usize,
	#[serde(default)]
	pub bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub temporary_bookmark: Option<i64>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub format: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub password: String,
	#[serde(default)]
	pub opened: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigData {
	pub version: u32,
	#[serde(default)]
	pub app: AppSettings,
	#[serde(default)]
	pub recent_documents: Vec<String>,
	#[serde(default)]
	pub opened_documents: Vec<String>,
	#[serde(default)]
	pub find_history: Vec<String>,
	#[serde(default)]
	pub documents: HashMap<String, DocumentConfig>,
	#[serde(default)]
	pub path_hashes: HashMap<String, String>,
}

impl Default for ConfigData {
	fn default() -> Self {
		Self {
			version: CONFIG_VERSION,
			app: AppSettings::default(),
			recent_documents: Vec::new(),
			opened_documents: Vec::new(),
			find_history: Vec::new(),
			documents: HashMap::new(),
			path_hashes: HashMap::new(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn temporary_bookmark_serializes_and_loads() {
		let doc =
			DocumentConfig { path: "book.epub".into(), temporary_bookmark: Some(12_345), ..DocumentConfig::default() };
		let serialized = toml::to_string(&doc).unwrap();
		let parsed: DocumentConfig = toml::from_str(&serialized).unwrap();
		assert_eq!(parsed.temporary_bookmark, Some(12_345));
	}

	#[test]
	fn temporary_bookmark_defaults_to_none_when_missing() {
		// Old config files without the field must load as None.
		let parsed: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(parsed.temporary_bookmark, None);
	}

	#[test]
	fn audio_time_round_trips_and_defaults_to_none_when_missing() {
		let doc = DocumentConfig { path: "book.zip".into(), audio_time_ms: Some(999), ..DocumentConfig::default() };
		let parsed: DocumentConfig = toml::from_str(&toml::to_string(&doc).unwrap()).unwrap();
		assert_eq!(parsed.audio_time_ms, Some(999));
		// Text-only documents and older config files carry no field at all.
		let bare: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(bare.audio_time_ms, None);
		assert!(!toml::to_string(&bare).unwrap().contains("audio_time_ms"));
	}
}
