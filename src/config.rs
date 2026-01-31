use std::{
	cmp::Ordering,
	env,
	fmt::Write,
	fs,
	path::{Path, PathBuf},
};

use base64::{
	Engine,
	engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use sha1::{Digest, Sha1};
use wxdragon::config::{Config, ConfigStyle};

const CONFIG_VERSION_LEGACY: i64 = 0;
const CONFIG_VERSION_1: i64 = 1;
const CONFIG_VERSION_2: i64 = 2;
const CONFIG_VERSION_CURRENT: i64 = CONFIG_VERSION_2;
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

pub struct ConfigManager {
	config: Option<Config>,
	initialized: bool,
}

impl Default for ConfigManager {
	fn default() -> Self {
		Self::new()
	}
}

impl ConfigManager {
	#[must_use]
	pub const fn new() -> Self {
		Self { config: None, initialized: false }
	}

	#[must_use]
	pub fn initialize(&mut self) -> bool {
		let config_path = get_config_path();
		let config = Config::new(
			"Paperback",
			Some("Paperback"),
			Some(&config_path),
			None,
			ConfigStyle::USE_LOCAL_FILE | ConfigStyle::USE_NO_ESCAPE_CHARACTERS,
		);
		self.config = Some(config);
		self.initialized = true;
		self.load_defaults();
		true
	}

	pub fn flush(&self) {
		if let Some(ref config) = self.config {
			config.flush(false);
		}
	}

	const fn is_ready(&self) -> bool {
		self.initialized
	}

	const fn config(&self) -> Option<&Config> {
		if self.initialized { self.config.as_ref() } else { None }
	}

	fn with_path<T>(&self, path: &str, f: impl FnOnce(&Config) -> T) -> Option<T> {
		let config = self.config()?;
		config.set_path(path);
		let result = f(config);
		config.set_path("/");
		Some(result)
	}

	pub fn get_app_string(&self, key: &str, default_value: &str) -> String {
		self.with_path("/app", |config| config.read_string(key, default_value))
			.unwrap_or_else(|| default_value.to_string())
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		self.with_path("/app", |config| config.read_bool(key, default_value)).unwrap_or(default_value)
	}

	pub fn get_app_int(&self, key: &str, default_value: i32) -> i32 {
		self.with_path("/app", |config| {
			let value: i64 = config.read_long(key, i64::from(default_value));
			value.try_into().unwrap_or(default_value)
		})
		.unwrap_or(default_value)
	}

	fn get_app_long(&self, key: &str, default_value: i64) -> i64 {
		self.with_path("/app", |config| config.read_long(key, default_value)).unwrap_or(default_value)
	}

	pub fn set_app_string(&self, key: &str, value: &str) {
		let _ = self.with_path("/app", |config| config.write_string(key, value));
	}

	pub fn set_app_bool(&self, key: &str, value: bool) {
		let _ = self.with_path("/app", |config| config.write_bool(key, value));
	}

	pub fn set_app_int(&self, key: &str, value: i32) {
		let _ = self.with_path("/app", |config| config.write_long(key, i64::from(value)));
	}

	fn set_app_long(&self, key: &str, value: i64) {
		let _ = self.with_path("/app", |config| config.write_long(key, value));
	}

	pub fn get_document_string(&self, path: &str, key: &str, default_value: &str) -> String {
		let Some(config) = self.config() else { return default_value.to_string() };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		let result = config.read_string(key, default_value);
		config.set_path("/");
		result
	}

	pub fn get_document_bool(&self, path: &str, key: &str, default_value: bool) -> bool {
		let Some(config) = self.config() else { return default_value };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		let result = config.read_bool(key, default_value);
		config.set_path("/");
		result
	}

	pub fn get_document_int(&self, path: &str, key: &str, default_value: i64) -> i64 {
		let Some(config) = self.config() else { return default_value };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		let result = config.read_long(key, default_value);
		config.set_path("/");
		result
	}

	pub fn set_document_string(&self, path: &str, key: &str, value: &str) {
		if let Some(config) = self.config() {
			let section = get_document_section(path);
			config.set_path(&format!("/{section}"));
			config.write_string("path", path);
			config.write_string(key, value);
			config.set_path("/");
		}
	}

	pub fn set_document_bool(&self, path: &str, key: &str, value: bool) {
		if let Some(config) = self.config() {
			let section = get_document_section(path);
			config.set_path(&format!("/{section}"));
			config.write_string("path", path);
			config.write_bool(key, value);
			config.set_path("/");
		}
	}

	pub fn set_document_int(&self, path: &str, key: &str, value: i64) {
		if let Some(config) = self.config() {
			let section = get_document_section(path);
			config.set_path(&format!("/{section}"));
			config.write_string("path", path);
			config.write_long(key, value);
			config.set_path("/");
		}
	}

	pub fn add_recent_document(&self, path: &str) {
		if !self.is_ready() {
			return;
		}
		self.ensure_document_path(path);
		let mut recent = self.get_recent_documents();
		if let Some(idx) = recent.iter().position(|p| p == path) {
			recent.remove(idx);
		}
		recent.insert(0, path.to_string());
		while recent.len() > MAX_RECENT_DOCUMENTS_TO_SHOW {
			recent.pop();
		}
		self.write_recent_documents(&recent);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		if !self.is_ready() {
			return Vec::new();
		}
		let Some(config) = self.config() else { return Vec::new() };
		config.set_path("/recent_documents");
		let mut result = Vec::new();
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
				result.push(path);
			}
			config.set_path("/recent_documents");
		}
		config.set_path("/");
		result
	}

	pub fn rebuild_recent_documents(&self) {
		let Some(config) = self.config() else { return };
		config.set_path("/");
		if config.has_group("recent_documents") {
			return;
		}
		let mut combined = self.get_recent_documents();
		for doc in self.get_all_documents() {
			if !combined.iter().any(|existing| existing == &doc) {
				combined.push(doc);
			}
		}
		self.write_recent_documents(&combined);
	}

	pub fn add_opened_document(&self, path: &str) {
		let mut opened = self.get_opened_documents();
		if opened.iter().any(|p| p == path) {
			return;
		}
		opened.push(path.to_string());
		self.write_opened_documents(&opened);
	}

	pub fn remove_opened_document(&self, path: &str) {
		let mut opened = self.get_opened_documents();
		if let Some(idx) = opened.iter().position(|p| p == path) {
			opened.remove(idx);
			self.write_opened_documents(&opened);
		}
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		if !self.is_ready() {
			return Vec::new();
		}
		let Some(config) = self.config() else { return Vec::new() };
		config.set_path("/opened_documents");
		let entries = config.get_entries();
		config.set_path("/");
		if !entries.is_empty() {
			let mut sorted_entries: Vec<_> = entries.into_iter().collect();
			sorted_entries.sort();
			config.set_path("/opened_documents");
			let result: Vec<String> =
				sorted_entries.iter().map(|key| config.read_string(key, "")).filter(|v| !v.is_empty()).collect();
			config.set_path("/");
			return result;
		}
		// Fallback: check old-style opened flag on documents
		let mut opened = Vec::new();
		for path in self.get_recent_documents() {
			if self.get_document_opened(&path) {
				opened.push(path);
			}
		}
		for path in self.get_all_documents() {
			if self.get_document_opened(&path) && !opened.contains(&path) {
				opened.push(path);
			}
		}
		opened
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
		if !self.is_ready() {
			return Vec::new();
		}
		let Some(config) = self.config() else { return Vec::new() };
		config.set_path("/find_history");
		let mut result = Vec::new();
		for idx in 0.. {
			let key = format!("item{idx}");
			if !config.has_entry(&key) {
				break;
			}
			let entry = config.read_string(&key, "");
			if entry.is_empty() {
				break;
			}
			result.push(entry);
		}
		config.set_path("/");
		result
	}

	pub fn add_find_history(&self, text: &str, max_len: usize) {
		if !self.is_ready() {
			return;
		}
		let trimmed = text.trim();
		if trimmed.is_empty() {
			return;
		}
		let mut history = self.get_find_history();
		if let Some(idx) = history.iter().position(|entry| entry == trimmed) {
			history.remove(idx);
		}
		history.insert(0, trimmed.to_string());
		while history.len() > max_len {
			history.pop();
		}
		if let Some(config) = self.config() {
			config.delete_group("find_history");
			config.set_path("/find_history");
			for (idx, entry) in history.iter().enumerate() {
				let key = format!("item{idx}");
				config.write_string(&key, entry);
			}
			config.set_path("/");
		}
	}

	pub fn set_document_position(&self, path: &str, position: i64) {
		self.set_document_int(path, "last_position", position);
	}

	#[must_use]
	pub fn get_document_position(&self, path: &str) -> i64 {
		self.get_document_int(path, "last_position", 0)
	}

	#[must_use]
	pub fn get_validated_document_position(&self, path: &str, max_position: i64) -> i64 {
		let saved = self.get_document_position(path);
		if saved > 0 && saved <= max_position { saved } else { -1 }
	}

	pub fn set_navigation_history(&self, path: &str, history: &[i64], history_index: usize) {
		let Some(config) = self.config() else { return };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		if history.is_empty() {
			config.delete_entry("navigation_history", false);
			config.delete_entry("navigation_history_index", false);
		} else {
			let history_string = history.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
			config.write_string("path", path);
			config.write_string("navigation_history", &history_string);
			let index: i64 = history_index
				.try_into()
				.expect("navigation_history_index does not fit into i64");
			config.write_long("navigation_history_index", index);
		}
		config.set_path("/");
	}

	pub fn get_navigation_history(&self, path: &str) -> NavigationHistory {
		let mut nav = NavigationHistory::default();
		let Some(config) = self.config() else { return nav };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		let history = config.read_string("navigation_history", "");
		if !history.is_empty() {
			for token in history.split(',') {
				let trimmed = token.trim();
				if trimmed.is_empty() {
					continue;
				}
				if let Ok(pos) = trimmed.parse::<i64>() {
					nav.positions.push(pos);
				}
			}
		}
		let value = config.read_long("navigation_history_index", 0);
		nav.index = usize::try_from(value).unwrap_or(0);
		config.set_path("/");
		nav
	}

	pub fn set_document_opened(&self, path: &str, opened: bool) {
		self.set_document_bool(path, "opened", opened);
	}

	#[must_use]
	pub fn get_document_opened(&self, path: &str) -> bool {
		self.get_document_bool(path, "opened", false)
	}

	pub fn remove_document_history(&self, path: &str) {
		let mut recent = self.get_recent_documents();
		if let Some(idx) = recent.iter().position(|p| p == path) {
			recent.remove(idx);
		}
		self.write_recent_documents(&recent);
		let section = get_document_section(path);
		if let Some(config) = self.config() {
			config.delete_group(&section);
		}
	}

	pub fn get_all_documents(&self) -> Vec<String> {
		let Some(config) = self.config() else { return Vec::new() };
		config.set_path("/");
		let groups = config.get_groups();
		let mut docs = Vec::new();
		for group in groups {
			if group.starts_with("doc_") {
				config.set_path(&format!("/{group}"));
				let path = config.read_string("path", "");
				if !path.is_empty() {
					docs.push(path);
				}
				config.set_path("/");
			}
		}
		docs
	}

	pub fn add_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		let mut bookmarks = self.get_bookmarks(path);
		if bookmarks.iter().any(|bm| bm.start == start && bm.end == end) {
			return;
		}
		bookmarks.push(Bookmark { start, end, note: note.to_string() });
		bookmarks.sort_by(|a, b| a.start.cmp(&b.start));
		self.write_bookmarks(path, &bookmarks);
	}

	pub fn remove_bookmark(&self, path: &str, start: i64, end: i64) {
		let mut bookmarks = self.get_bookmarks(path);
		if let Some(idx) = bookmarks.iter().position(|bm| bm.start == start && bm.end == end) {
			bookmarks.remove(idx);
			self.write_bookmarks(path, &bookmarks);
		}
	}

	pub fn toggle_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.start == start && bm.end == end) {
			self.remove_bookmark(path, start, end);
		} else {
			self.add_bookmark(path, start, end, note);
		}
	}

	pub fn update_bookmark_note(&self, path: &str, start: i64, end: i64, note: &str) {
		let mut bookmarks = self.get_bookmarks(path);
		if let Some(item) = bookmarks.iter_mut().find(|bm| bm.start == start && bm.end == end) {
			item.note = note.to_string();
			self.write_bookmarks(path, &bookmarks);
		}
	}

	pub fn get_bookmarks(&self, path: &str) -> Vec<Bookmark> {
		let Some(config) = self.config() else { return Vec::new() };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		let bookmark_string = config.read_string("bookmarks", "");
		config.set_path("/");

		if bookmark_string.is_empty() {
			return Vec::new();
		}
		let mut results = Vec::new();
		for token in bookmark_string.split(',') {
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
					let decoded_note = decode_note(note_str);
					results.push(Bookmark { start, end, note: decoded_note });
				}
			} else if let Ok(position) = trimmed.parse::<i64>() {
				results.push(Bookmark { start: position, end: position, note: String::new() });
			}
		}
		results.sort_by(|a, b| a.start.cmp(&b.start));
		results
	}

	pub fn set_document_format(&self, path: &str, format: &str) {
		self.set_document_string(path, "format", format);
	}

	pub fn get_document_format(&self, path: &str) -> String {
		self.get_document_string(path, "format", "")
	}

	pub fn set_document_password(&self, path: &str, password: &str) {
		if password.is_empty() {
			let Some(config) = self.config() else { return };
			let section = get_document_section(path);
			config.set_path(&format!("/{section}"));
			config.delete_entry("password", false);
			config.set_path("/");
		} else {
			self.set_document_string(path, "password", password);
		}
	}

	pub fn get_document_password(&self, path: &str) -> String {
		self.get_document_string(path, "password", "")
	}

	/// Import document settings from a .paperback sidecar file if it exists.
	/// This is a simplified version that only imports bookmarks and position.
	pub fn import_document_settings(&self, path: &str) {
		let import_path = format!("{path}.paperback");
		if Path::new(&import_path).exists() {
			self.import_settings_from_file(path, &import_path);
		}
	}

	/// Import document settings from a specified file.
	/// Parses simple key=value format from the import file.
	pub fn import_settings_from_file(&self, doc_path: &str, import_path: &str) {
		if !self.is_ready() || !Path::new(import_path).exists() {
			return;
		}
		let Ok(content) = std::fs::read_to_string(import_path) else { return };
		for line in content.lines() {
			let line = line.trim();
			if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
				continue;
			}
			if let Some((key, value)) = line.split_once('=') {
				let key = key.trim();
				let value = value.trim();
				match key {
					"last_position" => {
						if let Ok(pos) = value.parse::<i64>() {
							self.set_document_position(doc_path, pos);
						}
					}
					"bookmarks" => {
						self.set_document_string(doc_path, "bookmarks", value);
					}
					"format" => {
						self.set_document_format(doc_path, value);
					}
					_ => {}
				}
			}
		}
	}

	/// Export document settings to a .paperback sidecar file.
	pub fn export_document_settings(&self, doc_path: &str, export_path: &str) {
		if !self.is_ready() {
			return;
		}
		let mut content = String::new();
		content.push_str("# Paperback document settings\n");
		let position = self.get_document_position(doc_path);
		if position > 0 {
			let _ = write!(content, "last_position={}\n", position);
		}
		let format = self.get_document_format(doc_path);
		if !format.is_empty() {
			content.push_str(&format!("format={format}\n"));
		}
		let bookmarks = self.get_bookmarks(doc_path);
		if !bookmarks.is_empty() {
			let encoded = bookmarks
				.iter()
				.map(|bm| format!("{}:{}:{}", bm.start, bm.end, encode_note(&bm.note)))
				.collect::<Vec<_>>()
				.join(",");
			content.push_str(&format!("bookmarks={encoded}\n"));
		}
		let _ = std::fs::write(export_path, content);
	}

	pub fn needs_migration(&self) -> bool {
		if !self.is_ready() {
			return false;
		}
		let version = self.get_app_long("version", CONFIG_VERSION_LEGACY);
		if version == CONFIG_VERSION_CURRENT {
			return false;
		}
		let Some(config) = self.config() else { return false };
		config.set_path("/");
		let has_old_positions = config.has_group("positions");
		let has_old_globals = config.has_entry("restore_previous_documents") || config.has_entry("word_wrap");
		let has_old_opened = config.has_group("opened_documents");
		let needs_v1_to_v2 = version == CONFIG_VERSION_1;
		has_old_positions || has_old_globals || has_old_opened || needs_v1_to_v2
	}

	pub fn migrate_config(&self) -> bool {
		if !self.is_ready() {
			return false;
		}
		let Some(config) = self.config() else { return false };
		let version = self.get_app_long("version", CONFIG_VERSION_LEGACY);
		if version == CONFIG_VERSION_LEGACY {
			// Migrate old root-level settings to /app
			config.set_path("/");
			let restore_docs = config.read_bool("restore_previous_documents", true);
			let word_wrap = config.read_bool("word_wrap", false);
			if !self.has_app_entry("restore_previous_documents") {
				self.set_app_bool("restore_previous_documents", restore_docs);
			}
			if !self.has_app_entry("word_wrap") {
				self.set_app_bool("word_wrap", word_wrap);
			}
			// Migrate old positions section
			config.set_path("/positions");
			let entries = config.get_entries();
			for path in entries {
				let position_str = config.read_string(&path, "");
				if let Ok(pos) = position_str.parse::<i64>() {
					if pos > 0 {
						config.set_path("/");
						self.set_document_position(&path, pos);
						config.set_path("/positions");
					}
				}
			}
			config.set_path("/");
			config.delete_group("positions");
			// Clean up old root entries
			config.delete_entry("restore_previous_documents", false);
			config.delete_entry("word_wrap", false);
		} else if version == CONFIG_VERSION_1 {
			// V1 to V2: migrate bookmark format (add end position)
			let groups = config.get_groups();
			for group in groups {
				if !group.starts_with("doc_") {
					continue;
				}
				config.set_path(&format!("/{group}"));
				let old_bookmarks = config.read_string("bookmarks", "");
				if old_bookmarks.is_empty() {
					config.set_path("/");
					continue;
				}
				let mut new_bookmarks = String::new();
				let mut first = true;
				for token in old_bookmarks.split(',') {
					let trimmed = token.trim();
					if trimmed.is_empty() {
						continue;
					}
					if !first {
						new_bookmarks.push(',');
					}
					let colon_count = trimmed.matches(':').count();
					if colon_count == 0 {
						if let Ok(pos) = trimmed.parse::<i64>() {
							let _ = write!(&mut new_bookmarks, "{pos}:{pos}:");
						}
					} else if colon_count == 1 {
						new_bookmarks.push_str(trimmed);
						new_bookmarks.push(':');
					} else {
						new_bookmarks.push_str(trimmed);
					}
					first = false;
				}
				if !new_bookmarks.is_empty() {
					config.write_string("bookmarks", &new_bookmarks);
				}
				config.set_path("/");
			}
		}
		self.set_app_long("version", CONFIG_VERSION_CURRENT);
		true
	}

	fn has_app_entry(&self, key: &str) -> bool {
		self.with_path("/app", |config| config.has_entry(key)).unwrap_or(false)
	}

	fn load_defaults(&self) {
		if self.needs_migration() {
			self.migrate_config();
		}
		let defaults: &[(&str, bool, Option<i64>, Option<&str>)] = &[
			("restore_previous_documents", true, None, None),
			("word_wrap", false, None, None),
			("minimize_to_tray", false, None, None),
			("start_maximized", false, None, None),
			("compact_go_menu", true, None, None),
			("navigation_wrap", false, None, None),
			("check_for_updates_on_startup", true, None, None),
			("recent_documents_to_show", false, Some(DEFAULT_RECENT_DOCUMENTS_TO_SHOW), None),
			("sleep_timer_duration", false, Some(30), None),
			("language", false, None, Some("")),
			("active_document", false, None, Some("")),
		];

		for (key, is_bool, int_val, str_val) in defaults {
			if !self.has_app_entry(key) {
				if *is_bool {
					// Default bool values are the second element when is_bool is true
					// Defaults: restore_previous_documents=true, word_wrap=false, etc.
					let default_val = matches!(
						*key,
						"restore_previous_documents" | "compact_go_menu" | "check_for_updates_on_startup"
					);
					self.set_app_bool(key, default_val);
				} else if let Some(val) = int_val {
					self.set_app_long(key, *val);
				} else if let Some(val) = str_val {
					self.set_app_string(key, val);
				}
			}
		}

		if self.get_app_long("version", CONFIG_VERSION_LEGACY) != CONFIG_VERSION_CURRENT {
			self.set_app_long("version", CONFIG_VERSION_CURRENT);
		}
		self.rebuild_recent_documents();
	}

	fn write_recent_documents(&self, documents: &[String]) {
		let Some(config) = self.config() else { return };
		config.delete_group("recent_documents");
		for doc in documents {
			self.ensure_document_path(doc);
		}
		config.set_path("/recent_documents");
		for (idx, doc) in documents.iter().enumerate() {
			let doc_id = escape_document_path(doc);
			let key = format!("doc{idx}");
			config.write_string(&key, &doc_id);
		}
		config.set_path("/");
	}

	fn write_opened_documents(&self, documents: &[String]) {
		let Some(config) = self.config() else { return };
		config.delete_group("opened_documents");
		config.set_path("/opened_documents");
		for (idx, doc) in documents.iter().enumerate() {
			let key = format!("File{idx}");
			config.write_string(&key, doc);
		}
		config.set_path("/");
	}

	fn write_bookmarks(&self, path: &str, bookmarks: &[Bookmark]) {
		let Some(config) = self.config() else { return };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		if bookmarks.is_empty() {
			config.delete_entry("bookmarks", false);
		} else {
			let encoded = bookmarks
				.iter()
				.map(|bm| format!("{}:{}:{}", bm.start, bm.end, encode_note(&bm.note)))
				.collect::<Vec<_>>()
				.join(",");
			config.write_string("path", path);
			config.write_string("bookmarks", &encoded);
		}
		config.set_path("/");
	}

	fn ensure_document_path(&self, path: &str) {
		let Some(config) = self.config() else { return };
		let section = get_document_section(path);
		config.set_path(&format!("/{section}"));
		if !config.has_entry("path") {
			config.write_string("path", path);
		}
		config.set_path("/");
	}
}

impl Drop for ConfigManager {
	fn drop(&mut self) {
		if !self.initialized {
			return;
		}
		self.flush();
		self.config = None;
		self.initialized = false;
	}
}

pub fn get_sorted_document_list(
	config: &ConfigManager,
	open_paths: &[String],
	filter: &str,
) -> Vec<crate::ui_types::DocumentListItem> {
	use crate::ui_types::{DocumentListItem, DocumentListStatus};

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

fn get_config_path() -> String {
	let exe_dir = get_exe_directory();
	let is_installed = (0..10).any(|i| exe_dir.join(format!("unins{i:03}.exe")).exists());
	if is_installed {
		if let Some(appdata) = env::var_os("APPDATA") {
			let config_dir = PathBuf::from(appdata).join("Paperback");
			let _ = fs::create_dir_all(&config_dir);
			return config_dir.join("Paperback.ini").to_string_lossy().to_string();
		}
	}
	exe_dir.join("Paperback.ini").to_string_lossy().to_string()
}

fn get_exe_directory() -> PathBuf {
	env::current_exe()
		.ok()
		.and_then(|p| p.parent().map(std::path::Path::to_path_buf))
		.unwrap_or_else(|| PathBuf::from("."))
}

fn get_document_section(path: &str) -> String {
	escape_document_path(path)
}

fn escape_document_path(path: &str) -> String {
	let mut hasher = Sha1::new();
	hasher.update(path.as_bytes());
	let digest = hasher.finalize();
	let encoded = URL_SAFE_NO_PAD.encode(digest);
	format!("doc_{encoded}")
}

fn encode_note(note: &str) -> String {
	if note.is_empty() {
		return String::new();
	}
	STANDARD.encode(note.as_bytes())
}

fn decode_note(encoded: &str) -> String {
	if encoded.is_empty() {
		return String::new();
	}
	STANDARD.decode(encoded).map(|bytes| String::from_utf8_lossy(&bytes).to_string()).unwrap_or_default()
}
