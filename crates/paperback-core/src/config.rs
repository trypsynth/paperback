use std::{
	cell::{Cell, RefCell},
	cmp::Ordering,
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::types::DocumentListItem;

const CONFIG_VERSION: u32 = 4;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i64 = 25;
const MAX_RECENT_DOCUMENTS_TO_SHOW: usize = 100;

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

#[derive(Clone, Debug, PartialEq)]
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
	pub fn is_default(&self) -> bool {
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

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeyConfig {
	pub ctrl: bool,
	pub alt: bool,
	pub shift: bool,
	pub win: bool,
	pub key: char,
}

impl Default for HotkeyConfig {
	fn default() -> Self {
		Self { ctrl: true, alt: true, shift: false, win: false, key: 'P' }
	}
}

fn default_true() -> bool {
	true
}
fn default_recent_documents_to_show() -> i64 {
	DEFAULT_RECENT_DOCUMENTS_TO_SHOW
}
fn default_sleep_timer() -> i64 {
	30
}
fn default_reading_speed_wpm() -> i64 {
	150
}
fn default_font_color() -> i64 {
	-1
}
fn default_bg_color() -> i64 {
	-1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
	#[serde(default = "default_true")]
	pub restore_previous_documents: bool,
	#[serde(default)]
	pub word_wrap: bool,
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
			extra: HashMap::new(),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentConfig {
	pub path: String,
	#[serde(default)]
	pub last_position: i64,
	#[serde(default)]
	pub navigation_history: Vec<i64>,
	#[serde(default)]
	pub navigation_history_index: usize,
	#[serde(default)]
	pub bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub format: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub password: String,
	#[serde(default)]
	pub opened: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct SidecarData {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	last_position: Option<i64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	format: Option<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	bookmarks: Vec<StoredBookmark>,
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

pub struct ConfigManager {
	data: RefCell<ConfigData>,
	config_path: PathBuf,
	dirty: Cell<bool>,
	initialized: bool,
}

impl Default for ConfigManager {
	fn default() -> Self {
		Self::new()
	}
}

impl ConfigManager {
	#[must_use]
	pub fn new() -> Self {
		Self {
			data: RefCell::new(ConfigData::default()),
			config_path: PathBuf::new(),
			dirty: Cell::new(false),
			initialized: false,
		}
	}

	pub fn initialize(&mut self, config_path: PathBuf) -> bool {
		let (data, needs_save) = if config_path.exists() {
			match fs::read_to_string(&config_path).ok().and_then(|s| toml::from_str::<ConfigData>(&s).ok()) {
				Some(d) => (d, false),
				None => (ConfigData::default(), true),
			}
		} else {
			(ConfigData::default(), true)
		};

		self.config_path = config_path;
		self.initialized = true;
		*self.data.borrow_mut() = data;

		if needs_save {
			self.dirty.set(true);
			self.flush();
		}

		true
	}

	pub fn refresh_document_hash(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let data = self.data.borrow();
			if data.path_hashes.contains_key(path) {
				return;
			}
		}
		let digest = compute_document_hash(path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		if let Some(old_key) = data.path_hashes.get(path).cloned() {
			if old_key != new_key {
				if let Some(mut doc) = data.documents.remove(&old_key) {
					doc.path = path.to_string();
					data.documents.insert(new_key.clone(), doc);
				}
				data.path_hashes.insert(path.to_string(), new_key);
				self.dirty.set(true);
			}
		} else {
			if !data.documents.contains_key(&new_key) {
				let mut old_hasher = Sha1::new();
				old_hasher.update(path.as_bytes());
				let old_encoded = URL_SAFE_NO_PAD.encode(old_hasher.finalize());
				let old_key = format!("doc_{old_encoded}");

				if let Some(mut doc) = data.documents.remove(&old_key) {
					doc.path = path.to_string();
					data.documents.insert(new_key.clone(), doc);
				}
			}
			data.path_hashes.insert(path.to_string(), new_key);
			self.dirty.set(true);
		}
	}

	pub fn associate_uri_with_local_file(&self, uri: &str, local_path: &str) {
		let digest = compute_document_hash(local_path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		data.path_hashes.insert(uri.to_string(), new_key.clone());
		self.dirty.set(true);
	}

	pub fn get_doc_key(&self, path: &str) -> String {
		{
			let data = self.data.borrow();
			if let Some(hash) = data.path_hashes.get(path) {
				return hash.clone();
			}
		}

		let digest = compute_document_hash(path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		if !data.documents.contains_key(&new_key) {
			let mut old_hasher = Sha1::new();
			old_hasher.update(path.as_bytes());
			let old_encoded = URL_SAFE_NO_PAD.encode(old_hasher.finalize());
			let old_key = format!("doc_{old_encoded}");

			if let Some(doc) = data.documents.remove(&old_key) {
				data.documents.insert(new_key.clone(), doc);
			}
		}

		data.path_hashes.insert(path.to_string(), new_key.clone());
		self.dirty.set(true);
		new_key
	}

	pub fn flush(&self) {
		if !self.initialized || !self.dirty.get() {
			return;
		}
		let data = self.data.borrow();
		if let Ok(s) = toml::to_string_pretty(&*data) {
			let _ = fs::write(&self.config_path, s);
			self.dirty.set(false);
		}
	}

	pub fn get_app_string(&self, key: &str, default_value: &str) -> String {
		if !self.initialized {
			return default_value.to_string();
		}
		let data = self.data.borrow();
		data.app.extra.get(key).and_then(|v| v.as_str()).map_or_else(|| default_value.to_string(), str::to_string)
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		match key {
			"restore_previous_documents" => data.app.restore_previous_documents,
			"word_wrap" => data.app.word_wrap,
			"navigation_wrap" => data.app.navigation_wrap,
			"find_match_case" => data.app.find_match_case,
			"find_whole_word" => data.app.find_whole_word,
			"find_use_regex" => data.app.find_use_regex,
			_ => data.app.extra.get(key).and_then(|v| v.as_bool()).unwrap_or(default_value),
		}
	}

	pub fn get_app_int(&self, key: &str, default_value: i32) -> i32 {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		let v: i64 = match key {
			"recent_documents_to_show" => data.app.recent_documents_to_show,
			"sleep_timer_duration" => data.app.sleep_timer_duration,
			"reading_speed_wpm" => data.app.reading_speed_wpm,
			_ => {
				return data
					.app
					.extra
					.get(key)
					.and_then(|v| v.as_integer())
					.and_then(|i| i32::try_from(i).ok())
					.unwrap_or(default_value);
			}
		};
		v.try_into().unwrap_or(default_value)
	}

	pub fn set_app_string(&self, key: &str, value: &str) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.extra.insert(key.to_string(), toml::Value::String(value.to_string()));
		self.dirty.set(true);
	}

	pub fn set_app_bool(&self, key: &str, value: bool) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"restore_previous_documents" => data.app.restore_previous_documents = value,
				"word_wrap" => data.app.word_wrap = value,
				"navigation_wrap" => data.app.navigation_wrap = value,
				"find_match_case" => data.app.find_match_case = value,
				"find_whole_word" => data.app.find_whole_word = value,
				"find_use_regex" => data.app.find_use_regex = value,
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Boolean(value));
				}
			}
		}
		self.dirty.set(true);
	}

	pub fn set_app_int(&self, key: &str, value: i32) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"recent_documents_to_show" => data.app.recent_documents_to_show = i64::from(value),
				"sleep_timer_duration" => data.app.sleep_timer_duration = i64::from(value),
				"reading_speed_wpm" => data.app.reading_speed_wpm = i64::from(value),
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Integer(i64::from(value)));
				}
			}
		}
		self.dirty.set(true);
	}

	pub fn get_readability_font(&self) -> ReadabilityFont {
		if !self.initialized {
			return ReadabilityFont::default();
		}
		let data = self.data.borrow();
		ReadabilityFont {
			face_name: data.app.font_face_name.clone(),
			point_size: data.app.font_point_size.try_into().unwrap_or(0),
			style: data.app.font_style.try_into().unwrap_or(0),
			weight: data.app.font_weight.try_into().unwrap_or(0),
			underlined: data.app.font_underlined,
			strikethrough: data.app.font_strikethrough,
			color: data.app.font_color.try_into().unwrap_or(-1),
			encoding: data
				.app
				.extra
				.get("font_encoding")
				.and_then(|v| v.as_integer())
				.and_then(|i| i32::try_from(i).ok())
				.unwrap_or(0),
		}
	}

	pub fn set_readability_font(&self, font: &ReadabilityFont) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			data.app.font_face_name = font.face_name.clone();
			data.app.font_point_size = i64::from(font.point_size);
			data.app.font_style = i64::from(font.style);
			data.app.font_weight = i64::from(font.weight);
			data.app.font_underlined = font.underlined;
			data.app.font_strikethrough = font.strikethrough;
			data.app.font_color = i64::from(font.color);
			data.app.extra.insert("font_encoding".to_string(), toml::Value::Integer(i64::from(font.encoding)));
		}
		self.dirty.set(true);
	}

	pub fn get_line_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.line_spacing.try_into().unwrap_or(0)
	}

	pub fn set_line_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.line_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_bg_color(&self) -> i32 {
		if !self.initialized {
			return -1;
		}
		self.data.borrow().app.bg_color.try_into().unwrap_or(-1)
	}

	pub fn set_bg_color(&self, color: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.bg_color = i64::from(color);
		self.dirty.set(true);
	}

	pub fn get_text_alignment(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.text_alignment.try_into().unwrap_or(0)
	}

	pub fn set_text_alignment(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.text_alignment = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_letter_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.letter_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_letter_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.letter_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_paragraph_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.paragraph_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_paragraph_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.paragraph_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_hotkey(&self) -> HotkeyConfig {
		if !self.initialized {
			return HotkeyConfig::default();
		}
		self.data.borrow().app.hotkey.clone()
	}

	pub fn set_hotkey(&self, hotkey: &HotkeyConfig) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.hotkey = hotkey.clone();
		self.dirty.set(true);
	}

	pub fn add_recent_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path);
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.recent_documents.insert(0, path.to_string());
			while data.recent_documents.len() > MAX_RECENT_DOCUMENTS_TO_SHOW {
				data.recent_documents.pop();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().recent_documents.clone()
	}

	pub fn add_opened_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if !data.opened_documents.iter().any(|p| p == path) {
				data.opened_documents.push(path.to_string());
			}
		}
		self.dirty.set(true);
	}

	pub fn remove_opened_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.opened_documents.iter().position(|p| p == path) {
				data.opened_documents.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().opened_documents.clone()
	}

	pub fn get_opened_documents_existing(&self) -> Vec<String> {
		self.get_opened_documents().into_iter().filter(|path| Path::new(path).exists()).collect()
	}

	pub fn get_find_settings(&self) -> FindSettings {
		FindSettings {
			match_case: self.get_app_bool("find_match_case", false),
			whole_word: self.get_app_bool("find_whole_word", false),
			use_regex: self.get_app_bool("find_use_regex", false),
		}
	}

	pub fn set_find_settings(&self, settings: FindSettings) {
		self.set_app_bool("find_match_case", settings.match_case);
		self.set_app_bool("find_whole_word", settings.whole_word);
		self.set_app_bool("find_use_regex", settings.use_regex);
	}

	pub fn get_find_history(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().find_history.clone()
	}

	pub fn add_find_history(&self, text: &str, max_len: usize) {
		if !self.initialized {
			return;
		}
		let trimmed = text.trim().to_string();
		if trimmed.is_empty() {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.find_history.iter().position(|e| e == &trimmed) {
				data.find_history.remove(idx);
			}
			data.find_history.insert(0, trimmed);
			while data.find_history.len() > max_len {
				data.find_history.pop();
			}
		}
		self.dirty.set(true);
	}

	pub fn set_document_position(&self, path: &str, position: i64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).last_position = position;
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_document_position(&self, path: &str) -> i64 {
		if !self.initialized {
			return 0;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map_or(0, |d| d.last_position)
	}

	#[must_use]
	pub fn get_validated_document_position(&self, path: &str, max_position: i64) -> i64 {
		let saved = self.get_document_position(path);
		if saved > 0 && saved <= max_position { saved } else { -1 }
	}

	pub fn set_navigation_history(&self, path: &str, history: &[i64], history_index: usize) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			doc.navigation_history = history.to_vec();
			doc.navigation_history_index = history_index;
		}
		self.dirty.set(true);
	}

	pub fn get_navigation_history(&self, path: &str) -> NavigationHistory {
		let mut nav = NavigationHistory::default();
		if !self.initialized {
			return nav;
		}
		let key = self.get_doc_key(path);
		if let Some(doc) = self.data.borrow().documents.get(&key) {
			nav.positions = doc.navigation_history.clone();
			nav.index = doc.navigation_history_index;
		}
		nav
	}

	/// Sets the per-document opened flag. Prefer `add_opened_document` /`remove_opened_document` for maintaining the opened-documents list.
	pub fn set_document_opened(&self, path: &str, opened: bool) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).opened = opened;
		}
		self.dirty.set(true);
	}

	pub fn remove_document_history(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.documents.remove(&key);
		}
		self.dirty.set(true);
	}

	pub fn get_all_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().documents.values().map(|d| d.path.clone()).filter(|p| !p.is_empty()).collect()
	}

	pub fn add_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if doc.bookmarks.iter().any(|bm| bm.start == start && bm.end == end) {
				return;
			}
			doc.bookmarks.push(StoredBookmark { start, end, note: note.to_string() });
			doc.bookmarks.sort_by(|a, b| a.start.cmp(&b.start));
		}
		self.dirty.set(true);
	}

	pub fn remove_bookmark(&self, path: &str, start: i64, end: i64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(idx) = doc.bookmarks.iter().position(|bm| bm.start == start && bm.end == end) {
				doc.bookmarks.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn toggle_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.start == start && bm.end == end) {
			self.remove_bookmark(path, start, end);
		} else {
			self.add_bookmark(path, start, end, note);
		}
	}

	pub fn update_bookmark_note(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(bm) = doc.bookmarks.iter_mut().find(|bm| bm.start == start && bm.end == end) {
				bm.note = note.to_string();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_bookmarks(&self, path: &str) -> Vec<Bookmark> {
		if !self.initialized {
			return Vec::new();
		}
		self.data
			.borrow()
			.documents
			.get(&self.get_doc_key(path))
			.map(|d| {
				d.bookmarks.iter().map(|bm| Bookmark { start: bm.start, end: bm.end, note: bm.note.clone() }).collect()
			})
			.unwrap_or_default()
	}

	pub fn set_document_format(&self, path: &str, format: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).format = format.to_string();
		}
		self.dirty.set(true);
	}

	pub fn get_document_format(&self, path: &str) -> String {
		if !self.initialized {
			return String::new();
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map(|d| d.format.clone()).unwrap_or_default()
	}

	pub fn set_document_password(&self, path: &str, password: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).password = password.to_string();
		}
		self.dirty.set(true);
	}

	pub fn get_document_password(&self, path: &str) -> String {
		if !self.initialized {
			return String::new();
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map(|d| d.password.clone()).unwrap_or_default()
	}

	/// Import document settings from a `.paperback` sidecar file if it exists.
	pub fn import_document_settings(&self, path: &str) {
		let import_path = std::path::Path::new(path).with_extension("paperback");
		if let Some(import_path_str) = import_path.to_str() {
			if import_path.exists() {
				self.import_settings_from_file(path, import_path_str);
			}
		}
	}

	/// Import document settings from a specified TOML sidecar file.
	pub fn import_settings_from_file(&self, doc_path: &str, import_path: &str) {
		if !self.initialized || !Path::new(import_path).exists() {
			return;
		}
		let Ok(content) = fs::read_to_string(import_path) else { return };
		let Ok(sidecar) = toml::from_str::<SidecarData>(&content) else { return };
		if let Some(pos) = sidecar.last_position {
			self.set_document_position(doc_path, pos);
		}
		if let Some(format) = sidecar.format {
			self.set_document_format(doc_path, &format);
		}
		if !sidecar.bookmarks.is_empty() {
			let key = self.get_doc_key(doc_path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, doc_path).bookmarks = sidecar.bookmarks;
			self.dirty.set(true);
		}
	}

	/// Export document settings to a `.paperback` sidecar TOML file.
	pub fn export_document_settings(&self, doc_path: &str, export_path: &str) {
		if !self.initialized {
			return;
		}
		let key = self.get_doc_key(doc_path);
		let data = self.data.borrow();
		let doc = data.documents.get(&key);
		let sidecar = SidecarData {
			last_position: doc.map(|d| d.last_position).filter(|&p| p > 0),
			format: doc.and_then(|d| if d.format.is_empty() { None } else { Some(d.format.clone()) }),
			bookmarks: doc.map(|d| d.bookmarks.clone()).unwrap_or_default(),
		};
		if let Ok(s) = toml::to_string_pretty(&sidecar) {
			let _ = fs::write(export_path, s);
		}
	}

	fn doc_entry_mut<'a>(data: &'a mut ConfigData, key: String, path: &str) -> &'a mut DocumentConfig {
		let entry = data.documents.entry(key).or_insert_with(DocumentConfig::default);
		if entry.path.is_empty() {
			entry.path = path.to_string();
		}
		entry
	}
}

impl Drop for ConfigManager {
	fn drop(&mut self) {
		if !self.initialized {
			return;
		}
		self.flush();
	}
}

pub fn get_sorted_document_list(config: &ConfigManager, open_paths: &[String], filter: &str) -> Vec<DocumentListItem> {
	use crate::types::{DocumentListItem, DocumentListStatus};

	let recent_docs = config.get_recent_documents();
	let all_docs = config.get_all_documents();
	let mut doc_paths: Vec<String> = Vec::new();
	for path in &recent_docs {
		if !doc_paths.contains(path) {
			doc_paths.push(path.clone());
		}
	}
	let mut rest: Vec<String> = all_docs.iter().filter(|path| !doc_paths.contains(path)).cloned().collect();
	rest.sort_by(|a, b| {
		let a_path = Path::new(a);
		let b_path = Path::new(b);
		let a_name = a_path.file_name().and_then(|n| n.to_str()).unwrap_or(a);
		let b_name = b_path.file_name().and_then(|n| n.to_str()).unwrap_or(b);
		let name_cmp = a_name.to_lowercase().cmp(&b_name.to_lowercase());
		if name_cmp != Ordering::Equal {
			return name_cmp;
		}
		a.to_lowercase().cmp(&b.to_lowercase())
	});
	doc_paths.extend(rest);
	let filter_lower = filter.to_lowercase();
	doc_paths
		.into_iter()
		.filter_map(|path| {
			let path_obj = Path::new(&path);
			let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
			if !filter.is_empty() && !filename.to_lowercase().contains(&filter_lower) {
				return None;
			}
			let status = if !path_obj.exists() {
				DocumentListStatus::Missing
			} else if open_paths.contains(&path) {
				DocumentListStatus::Open
			} else {
				DocumentListStatus::Closed
			};
			Some(DocumentListItem { path, filename, status })
		})
		.collect()
}

pub fn compute_document_hash(path: &str) -> [u8; 20] {
	let mut hasher = Sha1::new();
	if let Ok(mut file) = fs::File::open(path) {
		use std::io::{Read, Seek, SeekFrom};
		let mut buffer = [0; 65536];
		let mut total_read = 0;
		let max_read = 1024 * 1024;
		while total_read < max_read {
			let to_read = std::cmp::min(buffer.len(), max_read - total_read);
			if let Ok(n) = file.read(&mut buffer[..to_read]) {
				if n == 0 {
					break;
				}
				hasher.update(&buffer[..n]);
				total_read += n;
			} else {
				break;
			}
		}
		if let Ok(metadata) = file.metadata() {
			let file_size = metadata.len();
			hasher.update(&file_size.to_le_bytes());
			if file_size > max_read as u64 {
				let seek_pos = std::cmp::max(file_size.saturating_sub(max_read as u64), max_read as u64);
				if file.seek(SeekFrom::Start(seek_pos)).is_ok() {
					let mut end_read = 0;
					let end_max = (file_size - seek_pos) as usize;
					while end_read < end_max {
						let to_read = std::cmp::min(buffer.len(), end_max - end_read);
						if let Ok(n) = file.read(&mut buffer[..to_read]) {
							if n == 0 {
								break;
							}
							hasher.update(&buffer[..n]);
							end_read += n;
						} else {
							break;
						}
					}
				}
			}
		}
	} else {
		hasher.update(path.as_bytes());
	}
	hasher.finalize().into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn doc_key_is_stable_and_prefixed() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let a = config.get_doc_key("C:\\books\\a.epub");
		let b = config.get_doc_key("C:\\books\\a.epub");
		assert_eq!(a, b);
		assert!(a.starts_with("doc_"));
		assert!(!a.contains('/'));
	}

	#[test]
	fn doc_key_differs_for_different_inputs() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let a = config.get_doc_key("book-a.epub");
		let b = config.get_doc_key("book-b.epub");
		assert_ne!(a, b);
	}
}
