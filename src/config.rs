use std::{
	cell::{Cell, RefCell},
	cmp::Ordering,
	collections::HashMap,
	env,
	fmt::{self, Display, Formatter},
	fs,
	path::{Path, PathBuf},
	str::FromStr,
};

use base64::{
	Engine,
	engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::types::DocumentListItem;

const CONFIG_VERSION: u32 = 4;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i64 = 25;
const MAX_RECENT_DOCUMENTS_TO_SHOW: usize = 100;

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
	/// wxFontEncoding value, 0 for default
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
struct StoredBookmark {
	start: i64,
	end: i64,
	#[serde(default)]
	note: String,
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
fn default_update_channel() -> String {
	"stable".to_string()
}
fn default_font_color() -> i64 {
	-1
}
fn default_bg_color() -> i64 {
	-1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AppSettings {
	#[serde(default = "default_true")]
	restore_previous_documents: bool,
	#[serde(default)]
	word_wrap: bool,
	#[serde(default)]
	minimize_to_tray: bool,
	#[serde(default)]
	start_maximized: bool,
	#[serde(default = "default_true")]
	compact_go_menu: bool,
	#[serde(default)]
	navigation_wrap: bool,
	#[serde(default = "default_true")]
	check_for_updates_on_startup: bool,
	#[serde(default)]
	find_match_case: bool,
	#[serde(default)]
	find_whole_word: bool,
	#[serde(default)]
	find_use_regex: bool,
	#[serde(default = "default_recent_documents_to_show")]
	recent_documents_to_show: i64,
	#[serde(default = "default_sleep_timer")]
	sleep_timer_duration: i64,
	#[serde(default)]
	language: String,
	#[serde(default)]
	active_document: String,
	#[serde(default = "default_update_channel")]
	update_channel: String,
	#[serde(default)]
	font_face_name: String,
	#[serde(default)]
	font_point_size: i64,
	#[serde(default)]
	font_style: i64,
	#[serde(default)]
	font_weight: i64,
	#[serde(default)]
	font_underlined: bool,
	#[serde(default)]
	font_strikethrough: bool,
	#[serde(default = "default_font_color")]
	font_color: i64,
	#[serde(default)]
	font_encoding: i64,
	#[serde(default = "default_bg_color")]
	bg_color: i64,
	#[serde(default)]
	text_alignment: i64,
	#[serde(default)]
	letter_spacing: i64,
	#[serde(default)]
	paragraph_spacing: i64,
	#[serde(default = "default_reading_speed_wpm")]
	reading_speed_wpm: i64,
	#[serde(default)]
	line_spacing: i64,
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			restore_previous_documents: true,
			word_wrap: false,
			minimize_to_tray: false,
			start_maximized: false,
			compact_go_menu: true,
			navigation_wrap: false,
			check_for_updates_on_startup: true,
			find_match_case: false,
			find_whole_word: false,
			find_use_regex: false,
			recent_documents_to_show: DEFAULT_RECENT_DOCUMENTS_TO_SHOW,
			sleep_timer_duration: 30,
			language: String::new(),
			active_document: String::new(),
			update_channel: "stable".to_string(),
			font_face_name: String::new(),
			font_point_size: 0,
			font_style: 0,
			font_weight: 0,
			font_underlined: false,
			font_strikethrough: false,
			font_color: -1,
			font_encoding: 0,
			bg_color: -1,
			text_alignment: 0,
			letter_spacing: 0,
			paragraph_spacing: 0,
			reading_speed_wpm: 150,
			line_spacing: 0,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct DocumentConfig {
	path: String,
	#[serde(default)]
	last_position: i64,
	#[serde(default)]
	navigation_history: Vec<i64>,
	#[serde(default)]
	navigation_history_index: usize,
	#[serde(default)]
	bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	format: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	password: String,
	#[serde(default)]
	opened: bool,
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
struct ConfigData {
	version: u32,
	#[serde(default)]
	app: AppSettings,
	#[serde(default)]
	recent_documents: Vec<String>,
	#[serde(default)]
	opened_documents: Vec<String>,
	#[serde(default)]
	find_history: Vec<String>,
	#[serde(default)]
	documents: HashMap<String, DocumentConfig>,
	#[serde(default)]
	path_hashes: HashMap<String, String>,
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

	pub fn initialize(&mut self) -> bool {
		let toml_path = get_config_path();
		let ini_path = toml_path.with_extension("ini");

		let (data, needs_save) = if toml_path.exists() {
			match fs::read_to_string(&toml_path).ok().and_then(|s| toml::from_str::<ConfigData>(&s).ok()) {
				Some(d) => (d, false),
				None => (ConfigData::default(), true),
			}
		} else if ini_path.exists() {
			(migrate_from_ini(&ini_path), true)
		} else {
			(ConfigData::default(), true)
		};

		self.config_path = toml_path;
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
		match key {
			"language" => data.app.language.clone(),
			"active_document" => data.app.active_document.clone(),
			"update_channel" => data.app.update_channel.clone(),
			_ => default_value.to_string(),
		}
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		match key {
			"restore_previous_documents" => data.app.restore_previous_documents,
			"word_wrap" => data.app.word_wrap,
			"minimize_to_tray" => data.app.minimize_to_tray,
			"start_maximized" => data.app.start_maximized,
			"compact_go_menu" => data.app.compact_go_menu,
			"navigation_wrap" => data.app.navigation_wrap,
			"check_for_updates_on_startup" => data.app.check_for_updates_on_startup,
			"find_match_case" => data.app.find_match_case,
			"find_whole_word" => data.app.find_whole_word,
			"find_use_regex" => data.app.find_use_regex,
			_ => default_value,
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
			_ => return default_value,
		};
		v.try_into().unwrap_or(default_value)
	}

	pub fn set_app_string(&self, key: &str, value: &str) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"language" => data.app.language = value.to_string(),
				"active_document" => data.app.active_document = value.to_string(),
				"update_channel" => data.app.update_channel = value.to_string(),
				_ => return,
			}
		}
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
				"minimize_to_tray" => data.app.minimize_to_tray = value,
				"start_maximized" => data.app.start_maximized = value,
				"compact_go_menu" => data.app.compact_go_menu = value,
				"navigation_wrap" => data.app.navigation_wrap = value,
				"check_for_updates_on_startup" => data.app.check_for_updates_on_startup = value,
				"find_match_case" => data.app.find_match_case = value,
				"find_whole_word" => data.app.find_whole_word = value,
				"find_use_regex" => data.app.find_use_regex = value,
				_ => return,
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
				_ => return,
			}
		}
		self.dirty.set(true);
	}

	pub fn get_update_channel(&self) -> UpdateChannel {
		let s = self.get_app_string("update_channel", "stable");
		s.parse().unwrap_or_default()
	}

	pub fn set_update_channel(&self, channel: UpdateChannel) {
		self.set_app_string("update_channel", &channel.to_string());
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
			encoding: data.app.font_encoding.try_into().unwrap_or(0),
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
			data.app.font_encoding = i64::from(font.encoding);
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
		self.data.borrow().documents.get(&self.get_doc_key(path)).map_or(0, |d| d.last_position)
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
		if let Some(doc) = self.data.borrow().documents.get(&self.get_doc_key(path)) {
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
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.documents.remove(&self.get_doc_key(path));
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
		self.data.borrow().documents.get(&self.get_doc_key(path)).map(|d| d.format.clone()).unwrap_or_default()
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
		self.data.borrow().documents.get(&self.get_doc_key(path)).map(|d| d.password.clone()).unwrap_or_default()
	}

	/// Import document settings from a `.paperback` sidecar file if it exists.
	pub fn import_document_settings(&self, path: &str) {
		let import_path = format!("{path}.paperback");
		if Path::new(&import_path).exists() {
			self.import_settings_from_file(path, &import_path);
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

fn get_config_path() -> PathBuf {
	let exe_dir = get_exe_directory();
	let is_installed = (0..10).any(|i| exe_dir.join(format!("unins{i:03}.exe")).exists());
	if is_installed {
		if let Some(appdata) = env::var_os("APPDATA") {
			let config_dir = PathBuf::from(appdata).join("Paperback");
			let _ = fs::create_dir_all(&config_dir);
			return config_dir.join("Paperback.toml");
		}
	}
	exe_dir.join("Paperback.toml")
}

fn get_exe_directory() -> PathBuf {
	env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
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

fn migrate_from_ini(ini_path: &Path) -> ConfigData {
	use wxdragon::config::{Config, ConfigStyle};

	let config = Config::new(
		"Paperback",
		Some("Paperback"),
		Some(&ini_path.to_string_lossy()),
		None,
		ConfigStyle::USE_LOCAL_FILE | ConfigStyle::USE_NO_ESCAPE_CHARACTERS,
	);
	let mut data = ConfigData::default();
	config.set_path("/app");
	data.app.restore_previous_documents = config.read_bool("restore_previous_documents", true);
	data.app.word_wrap = config.read_bool("word_wrap", false);
	data.app.minimize_to_tray = config.read_bool("minimize_to_tray", false);
	data.app.start_maximized = config.read_bool("start_maximized", false);
	data.app.compact_go_menu = config.read_bool("compact_go_menu", true);
	data.app.navigation_wrap = config.read_bool("navigation_wrap", false);
	data.app.check_for_updates_on_startup = config.read_bool("check_for_updates_on_startup", true);
	data.app.find_match_case = config.read_bool("find_match_case", false);
	data.app.find_whole_word = config.read_bool("find_whole_word", false);
	data.app.find_use_regex = config.read_bool("find_use_regex", false);
	data.app.recent_documents_to_show = config.read_long("recent_documents_to_show", DEFAULT_RECENT_DOCUMENTS_TO_SHOW);
	data.app.sleep_timer_duration = config.read_long("sleep_timer_duration", 30);
	data.app.language = config.read_string("language", "");
	data.app.active_document = config.read_string("active_document", "");
	data.app.update_channel = config.read_string("update_channel", "stable");
	config.set_path("/");
	config.set_path("/recent_documents");
	for idx in 0.. {
		let key = format!("doc{idx}");
		if !config.has_entry(&key) {
			break;
		}
		let doc_id = config.read_string(&key, "");
		if doc_id.is_empty() {
			break;
		}
		config.set_path("/");
		config.set_path(&format!("/{doc_id}"));
		let path = config.read_string("path", "");
		if !path.is_empty() {
			data.recent_documents.push(path);
		}
		config.set_path("/recent_documents");
	}
	config.set_path("/");
	config.set_path("/opened_documents");
	let entries = config.get_entries();
	if !entries.is_empty() {
		let mut sorted_entries: Vec<_> = entries.into_iter().collect();
		sorted_entries.sort();
		for key in &sorted_entries {
			let path = config.read_string(key, "");
			if !path.is_empty() {
				data.opened_documents.push(path);
			}
		}
	}
	config.set_path("/");
	config.set_path("/find_history");
	for idx in 0.. {
		let key = format!("item{idx}");
		if !config.has_entry(&key) {
			break;
		}
		let entry = config.read_string(&key, "");
		if entry.is_empty() {
			break;
		}
		data.find_history.push(entry);
	}
	config.set_path("/");
	config.set_path("/");
	let groups = config.get_groups();
	for group in groups {
		if !group.starts_with("doc_") {
			continue;
		}
		config.set_path(&format!("/{group}"));
		let path = config.read_string("path", "");
		if path.is_empty() {
			config.set_path("/");
			continue;
		}
		let mut doc = DocumentConfig::default();
		doc.path = path.clone();
		doc.last_position = config.read_long("last_position", 0);
		doc.opened = config.read_bool("opened", false);
		doc.format = config.read_string("format", "");
		doc.password = config.read_string("password", "");
		let history_str = config.read_string("navigation_history", "");
		if !history_str.is_empty() {
			doc.navigation_history = history_str.split(',').filter_map(|t| t.trim().parse::<i64>().ok()).collect();
		}
		let history_index = config.read_long("navigation_history_index", 0);
		doc.navigation_history_index = usize::try_from(history_index).unwrap_or(0);
		// Parse old CSV bookmark format: `start:end:base64note,...`
		let bookmark_str = config.read_string("bookmarks", "");
		if !bookmark_str.is_empty() {
			for token in bookmark_str.split(',') {
				let trimmed = token.trim();
				if trimmed.is_empty() {
					continue;
				}
				if trimmed.contains(':') {
					let mut parts = trimmed.splitn(3, ':');
					let start_str = parts.next().unwrap_or_default();
					let end_str = parts.next().unwrap_or_default();
					let note_str = parts.next().unwrap_or_default();
					if let (Ok(start), Ok(end)) = (start_str.parse::<i64>(), end_str.parse::<i64>()) {
						doc.bookmarks.push(StoredBookmark { start, end, note: decode_note(note_str) });
					}
				} else if let Ok(pos) = trimmed.parse::<i64>() {
					doc.bookmarks.push(StoredBookmark { start: pos, end: pos, note: String::new() });
				}
			}
			doc.bookmarks.sort_by(|a, b| a.start.cmp(&b.start));
		}
		data.documents.insert(group, doc);
		config.set_path("/");
	}
	data
}

fn decode_note(encoded: &str) -> String {
	if encoded.is_empty() {
		return String::new();
	}
	STANDARD.decode(encoded).map(|bytes| String::from_utf8_lossy(&bytes).to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

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

	#[test]
	fn decode_note_round_trip_with_unicode() {
		use base64::{Engine, engine::general_purpose::STANDARD};
		let original = "note with unicode: cafe\u{0301} and \u{1F600}";
		let encoded = STANDARD.encode(original.as_bytes());
		assert!(!encoded.is_empty());
		assert_eq!(decode_note(&encoded), original);
	}

	#[test]
	fn decode_note_handles_empty_and_invalid_input() {
		assert_eq!(decode_note(""), "");
		assert_eq!(decode_note("%%%not-base64%%%"), "");
	}

	#[test]
	fn update_channel_default_is_stable() {
		assert_eq!(UpdateChannel::default(), UpdateChannel::Stable);
	}

	#[rstest]
	#[case(UpdateChannel::Stable, "stable")]
	#[case(UpdateChannel::Dev, "dev")]
	fn update_channel_display(#[case] channel: UpdateChannel, #[case] expected: &str) {
		assert_eq!(channel.to_string(), expected);
	}

	#[rstest]
	#[case("stable", UpdateChannel::Stable)]
	#[case("dev", UpdateChannel::Dev)]
	#[case("STABLE", UpdateChannel::Stable)]
	#[case("DEV", UpdateChannel::Dev)]
	#[case("Stable", UpdateChannel::Stable)]
	fn update_channel_from_str_valid(#[case] input: &str, #[case] expected: UpdateChannel) {
		assert_eq!(input.parse::<UpdateChannel>(), Ok(expected));
	}

	#[rstest]
	#[case("")]
	#[case("unknown")]
	#[case("stab le")]
	#[case("stable ")]
	fn update_channel_from_str_invalid(#[case] input: &str) {
		assert!(input.parse::<UpdateChannel>().is_err());
	}
}
