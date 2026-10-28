use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shortcuts::{HotkeyConfig, ShortcutsConfig};
use crate::document::DocPos;

const CONFIG_VERSION: u32 = 4;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i64 = 25;

#[derive(Clone, Debug, Default)]
pub struct Bookmark {
	pub start: DocPos,
	pub end: DocPos,
	pub note: String,
	pub audio_ms: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct NavigationHistory {
	pub positions: Vec<DocPos>,
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
	/// 0xRRGGBB, or -1 for no override.
	pub color: i32,
	/// 0 means default.
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
	pub start: DocPos,
	pub end: DocPos,
	#[serde(default)]
	pub note: String,
	/// An audiobook's text is a blank line per chapter, so every bookmark in a chapterless MP3 has the same position and only the playback time tells them apart.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub audio_ms: Option<u64>,
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
	#[serde(default = "default_true")]
	pub join_pdf_paragraphs: bool,
	#[serde(default = "default_true")]
	pub strip_running_text: bool,
	#[serde(default = "default_true")]
	pub markdown_dollar_math: bool,
	#[serde(default)]
	pub navigation_wrap: bool,
	#[serde(default)]
	pub find_match_case: bool,
	#[serde(default)]
	pub find_whole_word: bool,
	#[serde(default)]
	pub find_use_regex: bool,
	/// Off suits a screen reader user, who hears the line either way and would otherwise hear the match spoken as a selection.
	#[serde(default = "default_true")]
	pub highlight_found_text: bool,
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
	/// Host-specific settings, kept through a read and write so each front end can store its own fields.
	#[serde(flatten, default)]
	pub extra: HashMap<String, toml::Value>,
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			restore_previous_documents: true,
			word_wrap: false,
			render_tables_inline: true,
			join_pdf_paragraphs: true,
			strip_running_text: true,
			markdown_dollar_math: true,
			navigation_wrap: false,
			find_match_case: false,
			find_whole_word: false,
			find_use_regex: false,
			highlight_found_text: true,
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

/// Speech settings a reader chose for one document; each left out follows the app's own.
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentSpeech {
	/// The platform's own voice identifier, which another platform won't find and so ignores.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub voice: Option<String>,
	/// Percentages of the platform's own range, as its rate and pitch controls show them, since each engine scales differently.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub rate_percent: Option<i32>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub pitch_percent: Option<i32>,
}

impl DocumentSpeech {
	#[must_use]
	pub fn is_empty(&self) -> bool {
		*self == Self::default()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentConfig {
	pub path: String,
	#[serde(default)]
	pub last_position: DocPos,
	/// Apart from `last_position` because resuming should return to what was last heard, which a text position only approximates to the nearest clip.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub audio_time_ms: Option<u64>,
	#[serde(default)]
	pub navigation_history: Vec<DocPos>,
	#[serde(default)]
	pub navigation_history_index: usize,
	#[serde(default)]
	pub bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub temporary_bookmark: Option<DocPos>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub format: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub password: String,
	#[serde(default)]
	pub opened: bool,
	/// Decides whether a document opened from a link is downloaded again at startup.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub download_size: Option<u64>,
	/// Names a document opened from a link in lists better than its link can.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub download_name: Option<String>,
	#[serde(default, skip_serializing_if = "DocumentSpeech::is_empty")]
	pub speech: DocumentSpeech,
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
		let doc = DocumentConfig {
			path: "book.epub".into(),
			temporary_bookmark: Some(12_345.into()),
			..DocumentConfig::default()
		};
		let serialized = toml::to_string(&doc).unwrap();
		let parsed: DocumentConfig = toml::from_str(&serialized).unwrap();
		assert_eq!(parsed.temporary_bookmark, Some(12_345.into()));
	}

	/// Config files from before this setting must keep the highlighting they always had.
	#[test]
	fn highlight_found_text_defaults_to_true_when_missing() {
		let parsed: AppSettings = toml::from_str("word_wrap = true\n").unwrap();
		assert!(parsed.highlight_found_text);
	}

	#[test]
	fn positions_round_trip_as_plain_integers() {
		let saved = "path = \"book.epub\"\nlast_position = 4321\nnavigation_history = [10, 200, 4321]\nnavigation_history_index = 2\ntemporary_bookmark = 77\nopened = false\n\n[[bookmarks]]\nstart = 5\nend = 9\nnote = \"a note\"\n";
		let doc: DocumentConfig = toml::from_str(saved).unwrap();
		assert_eq!(doc.last_position, 4321);
		assert_eq!(doc.navigation_history, vec![10, 200, 4321]);
		assert_eq!(doc.temporary_bookmark, Some(77.into()));
		assert_eq!((doc.bookmarks[0].start, doc.bookmarks[0].end), (5.into(), 9.into()));
		assert_eq!(toml::to_string(&doc).unwrap(), saved);
	}

	/// An edited file's negative position must not lose the whole config.
	#[test]
	fn a_negative_position_loads_as_the_start() {
		let doc: DocumentConfig =
			toml::from_str("path = \"book.epub\"\nlast_position = -1\nnavigation_history = [-5, 30]\n").unwrap();
		assert_eq!(doc.last_position, DocPos::ZERO);
		assert_eq!(doc.navigation_history, vec![0, 30]);
	}

	#[test]
	fn temporary_bookmark_defaults_to_none_when_missing() {
		let parsed: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(parsed.temporary_bookmark, None);
	}

	#[test]
	fn audio_time_round_trips_and_defaults_to_none_when_missing() {
		let doc = DocumentConfig { path: "book.zip".into(), audio_time_ms: Some(999), ..DocumentConfig::default() };
		let parsed: DocumentConfig = toml::from_str(&toml::to_string(&doc).unwrap()).unwrap();
		assert_eq!(parsed.audio_time_ms, Some(999));
		let bare: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(bare.audio_time_ms, None);
		assert!(!toml::to_string(&bare).unwrap().contains("audio_time_ms"));
	}

	#[test]
	fn a_documents_own_speech_round_trips_and_is_left_out_when_it_has_none() {
		let speech =
			DocumentSpeech { voice: Some("com.apple.voice.Alex".into()), rate_percent: Some(60), pitch_percent: None };
		let doc = DocumentConfig { path: "book.epub".into(), speech: speech.clone(), ..DocumentConfig::default() };
		let saved = toml::to_string(&doc).unwrap();
		assert!(!saved.contains("pitch_percent"));
		assert_eq!(toml::from_str::<DocumentConfig>(&saved).unwrap().speech, speech);
		let bare = DocumentConfig { path: "book.epub".into(), ..DocumentConfig::default() };
		assert!(!toml::to_string(&bare).unwrap().contains("speech"));
	}

	#[test]
	fn download_size_round_trips_and_defaults_to_none_when_missing() {
		let doc = DocumentConfig {
			path: "https://example.org/book.epub".into(),
			download_size: Some(1_500_000),
			..DocumentConfig::default()
		};
		let parsed: DocumentConfig = toml::from_str(&toml::to_string(&doc).unwrap()).unwrap();
		assert_eq!(parsed.download_size, Some(1_500_000));
		let bare: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(bare.download_size, None);
		assert!(!toml::to_string(&bare).unwrap().contains("download_size"));
	}

	#[test]
	fn download_name_round_trips_and_defaults_to_none_when_missing() {
		let doc = DocumentConfig {
			path: "https://example.org/download?id=7".into(),
			download_name: Some("Moby Dick.epub".into()),
			..DocumentConfig::default()
		};
		let parsed: DocumentConfig = toml::from_str(&toml::to_string(&doc).unwrap()).unwrap();
		assert_eq!(parsed.download_name.as_deref(), Some("Moby Dick.epub"));
		let bare: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(bare.download_name, None);
		assert!(!toml::to_string(&bare).unwrap().contains("download_name"));
	}
}
