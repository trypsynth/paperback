use std::{
	env,
	fmt::Write,
	fs::{self, OpenOptions},
	path::{Path, PathBuf},
	string::ToString,
};

use base64::{
	Engine,
	engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use configparser::ini::Ini;
use sha1::{Digest, Sha1};

const CONFIG_VERSION_LEGACY: i32 = 0;
const CONFIG_VERSION_1: i32 = 1;
const CONFIG_VERSION_2: i32 = 2;
const CONFIG_VERSION_CURRENT: i32 = CONFIG_VERSION_2;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i32 = 25;
const MAX_RECENT_DOCUMENTS_TO_SHOW: usize = 100;
const CONFIG_DIRECTORY: &str = "paperback";
const CONFIG_FILENAME: &str = "paperback.ini";

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

pub struct ConfigManager {
	data: Ini,
	config_path: PathBuf,
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
		Self { data: Ini::new(), config_path: PathBuf::new(), initialized: false }
	}

	#[must_use]
	pub fn initialize(&mut self) -> bool {
		self.config_path = get_config_path();
		if let Some(parent) = self.config_path.parent() {
			let _ = fs::create_dir_all(parent);
		}
		let loaded = self.data.load(&self.config_path);
		if loaded.is_err() {
			self.data = Ini::new();
		}
		self.initialized = true;
		self.load_defaults();
		true
	}

	pub fn flush(&self) {
		if self.initialized {
			if let Some(parent) = self.config_path.parent() {
				let _ = fs::create_dir_all(parent);
			}
			let _ = self.data.write(&self.config_path);
		}
	}

	pub fn shutdown(&mut self) {
		if !self.initialized {
			return;
		}
		self.flush();
		self.data = Ini::new();
		self.initialized = false;
	}

	const fn is_ready(&self) -> bool {
		self.initialized
	}

	pub fn get_string(&self, key: &str, default_value: &str) -> String {
		self.get_value(None, key).unwrap_or_else(|| default_value.to_string())
	}

	pub fn get_bool(&self, key: &str, default_value: bool) -> bool {
		self.get_value(None, key).map_or(default_value, |v| parse_bool(&v, default_value))
	}

	pub fn get_int(&self, key: &str, default_value: i32) -> i32 {
		self.get_value(None, key).and_then(|v| v.parse::<i32>().ok()).unwrap_or(default_value)
	}

	pub fn set_string(&mut self, key: &str, value: &str) {
		self.set_value(None, key, value);
	}

	pub fn set_bool(&mut self, key: &str, value: bool) {
		self.set_value(None, key, &format_bool(value));
	}

	pub fn set_int(&mut self, key: &str, value: i32) {
		self.set_value(None, key, &value.to_string());
	}

	pub fn get_app_string(&self, key: &str, default_value: &str) -> String {
		self.get_value(Some("app"), key).unwrap_or_else(|| default_value.to_string())
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		self.get_value(Some("app"), key).map_or(default_value, |v| parse_bool(&v, default_value))
	}

	pub fn get_app_int(&self, key: &str, default_value: i32) -> i32 {
		self.get_value(Some("app"), key).and_then(|v| v.parse::<i32>().ok()).unwrap_or(default_value)
	}

	pub fn set_app_string(&mut self, key: &str, value: &str) {
		self.set_value(Some("app"), key, value);
	}

	pub fn set_app_bool(&mut self, key: &str, value: bool) {
		self.set_value(Some("app"), key, &format_bool(value));
	}

	pub fn set_app_int(&mut self, key: &str, value: i32) {
		self.set_value(Some("app"), key, &value.to_string());
	}

	pub fn get_document_string(&self, path: &str, key: &str, default_value: &str) -> String {
		let section = get_document_section(path);
		self.get_value(Some(&section), key).unwrap_or_else(|| default_value.to_string())
	}

	pub fn get_document_bool(&self, path: &str, key: &str, default_value: bool) -> bool {
		let section = get_document_section(path);
		self.get_value(Some(&section), key).map_or(default_value, |v| parse_bool(&v, default_value))
	}

	pub fn get_document_int(&self, path: &str, key: &str, default_value: i64) -> i64 {
		let section = get_document_section(path);
		self.get_value(Some(&section), key).and_then(|v| v.parse::<i64>().ok()).unwrap_or(default_value)
	}

	pub fn set_document_string(&mut self, path: &str, key: &str, value: &str) {
		let section = get_document_section(path);
		self.set_value(Some(&section), "path", path);
		self.set_value(Some(&section), key, value);
	}

	pub fn set_document_bool(&mut self, path: &str, key: &str, value: bool) {
		let section = get_document_section(path);
		self.set_value(Some(&section), "path", path);
		self.set_value(Some(&section), key, &format_bool(value));
	}

	pub fn set_document_int(&mut self, path: &str, key: &str, value: i64) {
		let section = get_document_section(path);
		self.set_value(Some(&section), "path", path);
		self.set_value(Some(&section), key, &value.to_string());
	}

	pub fn add_recent_document(&mut self, path: &str) {
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
		let mut result = Vec::new();
		for idx in 0.. {
			let key = format!("doc{idx}");
			let doc_id = match self.get_value(Some("recent_documents"), &key) {
				Some(v) if !v.is_empty() => v,
				_ => break,
			};
			if let Some(path) = self.get_value(Some(&doc_id), "path") {
				if !path.is_empty() {
					result.push(path);
				}
			}
		}
		result
	}

	pub fn clear_recent_documents(&mut self) {
		self.remove_section(Some("recent_documents"));
	}

	pub fn rebuild_recent_documents(&mut self) {
		if self.data.get_map_ref().contains_key("recent_documents") {
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

	pub fn add_opened_document(&mut self, path: &str) {
		let mut opened = self.get_opened_documents();
		if opened.iter().any(|p| p == path) {
			return;
		}
		opened.push(path.to_string());
		self.write_opened_documents(&opened);
	}

	pub fn remove_opened_document(&mut self, path: &str) {
		let mut opened = self.get_opened_documents();
		if let Some(idx) = opened.iter().position(|p| p == path) {
			opened.remove(idx);
			self.write_opened_documents(&opened);
		}
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		let mut entries = self.iter_section(Some("opened_documents"));
		entries.sort_by(|a, b| a.0.cmp(&b.0));
		entries.into_iter().map(|(_, v)| v).collect()
	}

	pub fn clear_opened_documents(&mut self) {
		self.remove_section(Some("opened_documents"));
	}

	pub fn set_document_position(&mut self, path: &str, position: i64) {
		self.set_document_int(path, "last_position", position);
	}

	#[must_use]
	pub fn get_document_position(&self, path: &str) -> i64 {
		self.get_document_int(path, "last_position", 0)
	}

	pub fn set_navigation_history(&mut self, path: &str, history: &[i64], history_index: usize) {
		let section = get_document_section(path);
		if history.is_empty() {
			self.remove_entry(Some(&section), "navigation_history");
			self.remove_entry(Some(&section), "navigation_history_index");
			return;
		}
		let history_string = history.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
		self.set_value(Some(&section), "path", path);
		self.set_value(Some(&section), "navigation_history", &history_string);
		self.set_value(Some(&section), "navigation_history_index", &history_index.to_string());
	}

	pub fn get_navigation_history(&self, path: &str) -> NavigationHistory {
		let mut nav = NavigationHistory::default();
		let section = get_document_section(path);
		if let Some(history) = self.get_value(Some(&section), "navigation_history") {
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
		if let Some(index) = self.get_value(Some(&section), "navigation_history_index") {
			if let Ok(idx) = index.parse::<usize>() {
				nav.index = idx;
			}
		}
		nav
	}

	pub fn set_document_opened(&mut self, path: &str, opened: bool) {
		self.set_document_bool(path, "opened", opened);
	}

	#[must_use]
	pub fn get_document_opened(&self, path: &str) -> bool {
		self.get_document_bool(path, "opened", false)
	}

	pub fn get_all_opened_documents(&self) -> Vec<String> {
		let mut result = Vec::new();
		for section in self.section_names() {
			if !section.starts_with("doc_") {
				continue;
			}
			if let Some(opened) = self.get_value(Some(&section), "opened") {
				if parse_bool(&opened, false) {
					if let Some(path) = self.get_value(Some(&section), "path") {
						if !path.is_empty() {
							result.push(path);
						}
					}
				}
			}
		}
		result
	}

	pub fn remove_document_history(&mut self, path: &str) {
		let mut recent = self.get_recent_documents();
		if let Some(idx) = recent.iter().position(|p| p == path) {
			recent.remove(idx);
		}
		self.write_recent_documents(&recent);
		let section = get_document_section(path);
		self.remove_section(Some(&section));
	}

	pub fn remove_navigation_history(&mut self, path: &str) {
		let section = get_document_section(path);
		self.remove_entry(Some(&section), "navigation_history");
		self.remove_entry(Some(&section), "navigation_history_index");
	}

	pub fn get_all_documents(&self) -> Vec<String> {
		let mut docs = Vec::new();
		for section in self.section_names() {
			if section.starts_with("doc_") {
				if let Some(path) = self.get_value(Some(&section), "path") {
					if !path.is_empty() {
						docs.push(path);
					}
				}
			}
		}
		docs
	}

	pub fn add_bookmark(&mut self, path: &str, start: i64, end: i64, note: &str) {
		let mut bookmarks = self.get_bookmarks(path);
		if bookmarks.iter().any(|bm| bm.start == start && bm.end == end) {
			return;
		}
		bookmarks.push(Bookmark { start, end, note: note.to_string() });
		bookmarks.sort_by(|a, b| a.start.cmp(&b.start));
		self.write_bookmarks(path, &bookmarks);
	}

	pub fn remove_bookmark(&mut self, path: &str, start: i64, end: i64) {
		let mut bookmarks = self.get_bookmarks(path);
		if let Some(idx) = bookmarks.iter().position(|bm| bm.start == start && bm.end == end) {
			bookmarks.remove(idx);
			self.write_bookmarks(path, &bookmarks);
		}
	}

	pub fn toggle_bookmark(&mut self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.start == start && bm.end == end) {
			self.remove_bookmark(path, start, end);
		} else {
			self.add_bookmark(path, start, end, note);
		}
	}

	pub fn update_bookmark_note(&mut self, path: &str, start: i64, end: i64, note: &str) {
		let mut bookmarks = self.get_bookmarks(path);
		if let Some(item) = bookmarks.iter_mut().find(|bm| bm.start == start && bm.end == end) {
			item.note = note.to_string();
			self.write_bookmarks(path, &bookmarks);
		}
	}

	pub fn get_bookmarks(&self, path: &str) -> Vec<Bookmark> {
		let section = get_document_section(path);
		let bookmark_string = self.get_value(Some(&section), "bookmarks").unwrap_or_default();
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

	pub fn clear_bookmarks(&mut self, path: &str) {
		let section = get_document_section(path);
		self.remove_entry(Some(&section), "bookmarks");
	}

	pub fn get_next_bookmark(&self, path: &str, current_position: i64) -> Bookmark {
		for bm in self.get_bookmarks(path) {
			if bm.start > current_position {
				return bm;
			}
		}
		Bookmark { start: -1, end: -1, note: String::new() }
	}

	pub fn get_previous_bookmark(&self, path: &str, current_position: i64) -> Bookmark {
		let mut bookmarks = self.get_bookmarks(path);
		bookmarks.reverse();
		for bm in bookmarks {
			if bm.start < current_position {
				return bm;
			}
		}
		Bookmark { start: -1, end: -1, note: String::new() }
	}

	pub fn set_document_format(&mut self, path: &str, format: &str) {
		self.set_document_string(path, "format", format);
	}

	pub fn get_document_format(&self, path: &str) -> String {
		self.get_document_string(path, "format", "")
	}

	pub fn set_document_password(&mut self, path: &str, password: &str) {
		if password.is_empty() {
			let section = get_document_section(path);
			self.remove_entry(Some(&section), "password");
		} else {
			self.set_document_string(path, "password", password);
		}
	}

	pub fn get_document_password(&self, path: &str) -> String {
		self.get_document_string(path, "password", "")
	}

	pub fn needs_migration(&self) -> bool {
		if !self.is_ready() {
			return false;
		}
		let version = self.get_app_int("version", CONFIG_VERSION_LEGACY);
		if version == CONFIG_VERSION_CURRENT {
			return false;
		}
		let has_old_positions = !self.iter_section(Some("positions")).is_empty();
		let has_old_globals =
			self.get_value(None, "restore_previous_documents").is_some() || self.get_value(None, "word_wrap").is_some();
		let has_old_opened = self.section_names().iter().any(|s| s == "opened_documents");
		let needs_v1_to_v2 = version == CONFIG_VERSION_1;
		has_old_positions || has_old_globals || has_old_opened || needs_v1_to_v2
	}

	pub fn migrate_config(&mut self) -> bool {
		if !self.is_ready() {
			return false;
		}
		let version = self.get_app_int("version", CONFIG_VERSION_LEGACY);
		if version == CONFIG_VERSION_LEGACY {
			let restore_docs = self.get_bool("restore_previous_documents", true);
			let word_wrap = self.get_bool("word_wrap", false);
			if self.get_value(Some("app"), "restore_previous_documents").is_none() {
				self.set_app_bool("restore_previous_documents", restore_docs);
			}
			if self.get_value(Some("app"), "word_wrap").is_none() {
				self.set_app_bool("word_wrap", word_wrap);
			}
			let old_recent: Vec<String> = self
				.iter_section(Some("recent_documents"))
				.into_iter()
				.filter_map(|(_, v)| if v.is_empty() { None } else { Some(v) })
				.collect();
			for (path, position) in self.iter_section(Some("positions")) {
				if let Ok(pos) = position.parse::<i64>() {
					if pos > 0 {
						self.set_document_position(&path, pos);
					}
				}
			}
			self.remove_section(Some("recent_documents"));
			for path in old_recent {
				self.add_recent_document(&path);
			}
			let opened_paths: Vec<String> = self
				.iter_section(Some("opened_documents"))
				.into_iter()
				.filter_map(|(_, v)| if v.is_empty() { None } else { Some(v) })
				.collect();
			for path in opened_paths {
				self.set_document_opened(&path, true);
			}
			self.remove_section(Some("positions"));
			self.remove_entry(None, "restore_previous_documents");
			self.remove_entry(None, "word_wrap");
			self.remove_section(Some("opened_documents"));
		} else if version == CONFIG_VERSION_1 {
			let sections = self.section_names();
			for section in sections {
				if !section.starts_with("doc_") {
					continue;
				}
				let old_bookmarks = self.get_value(Some(&section), "bookmarks").unwrap_or_default();
				if old_bookmarks.is_empty() {
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
					self.set_value(Some(&section), "bookmarks", &new_bookmarks);
				}
			}
		}
		self.set_app_int("version", CONFIG_VERSION_CURRENT);
		true
	}

	pub fn export_document_settings(&self, doc_path: &str, export_path: &str) {
		if !self.is_ready() {
			return;
		}
		let mut export_data = Ini::new();
		let section = get_document_section(doc_path);
		for (key, value) in self.iter_section(Some(&section)) {
			if key == "path" {
				continue;
			}
			export_data.set("DEFAULT", &key, Some(value));
		}
		let _ = export_data.write(export_path);
	}

	pub fn import_document_settings(&mut self, path: &str) {
		let import_path = format!("{path}.paperback");
		if Path::new(&import_path).exists() {
			self.import_settings_from_file(path, &import_path);
		}
	}

	pub fn import_settings_from_file(&mut self, doc_path: &str, import_path: &str) {
		if !self.is_ready() || !Path::new(import_path).exists() {
			return;
		}
		let mut import_data = Ini::new();
		if import_data.load(import_path).is_err() {
			return;
		}
		let section = get_document_section(doc_path);
		for props in import_data.get_map_ref().values() {
			for (key, value) in props {
				if let Some(value) = value {
					self.set_value(Some(&section), key, value);
				}
			}
		}
		self.set_value(Some(&section), "path", doc_path);
		let _ = self.data.write(&self.config_path);
	}

	fn load_defaults(&mut self) {
		if self.needs_migration() {
			self.migrate_config();
		}
		let defaults = [
			("restore_previous_documents", format_bool(true)),
			("word_wrap", format_bool(false)),
			("minimize_to_tray", format_bool(false)),
			("start_maximized", format_bool(false)),
			("compact_go_menu", format_bool(true)),
			("navigation_wrap", format_bool(false)),
			("check_for_updates_on_startup", format_bool(true)),
			("recent_documents_to_show", DEFAULT_RECENT_DOCUMENTS_TO_SHOW.to_string()),
			("sleep_timer_duration", "30".to_string()),
			("language", String::new()),
			("active_document", String::new()),
		];
		for (key, value) in &defaults {
			if self.get_value(Some("app"), key).is_none() {
				self.set_value(Some("app"), key, value);
			}
		}
		if self.get_app_int("version", CONFIG_VERSION_LEGACY) != CONFIG_VERSION_CURRENT {
			self.set_app_int("version", CONFIG_VERSION_CURRENT);
		}
		self.rebuild_recent_documents();
	}

	fn write_recent_documents(&mut self, documents: &[String]) {
		self.remove_section(Some("recent_documents"));
		for doc in documents {
			self.ensure_document_path(doc);
		}
		for (idx, doc) in documents.iter().enumerate() {
			let doc_id = escape_document_path(doc);
			let key = format!("doc{idx}");
			self.set_value(Some("recent_documents"), &key, &doc_id);
		}
	}

	fn write_opened_documents(&mut self, documents: &[String]) {
		self.remove_section(Some("opened_documents"));
		for (idx, doc) in documents.iter().enumerate() {
			let key = format!("File{idx}");
			self.set_value(Some("opened_documents"), &key, doc);
		}
	}

	fn write_bookmarks(&mut self, path: &str, bookmarks: &[Bookmark]) {
		let section = get_document_section(path);
		if bookmarks.is_empty() {
			self.remove_entry(Some(&section), "bookmarks");
			return;
		}
		let encoded = bookmarks
			.iter()
			.map(|bm| format!("{}:{}:{}", bm.start, bm.end, encode_note(&bm.note)))
			.collect::<Vec<_>>()
			.join(",");
		self.set_value(Some(&section), "path", path);
		self.set_value(Some(&section), "bookmarks", &encoded);
	}

	fn ensure_document_path(&mut self, path: &str) {
		let section = get_document_section(path);
		if self.get_value(Some(&section), "path").is_none() {
			self.set_value(Some(&section), "path", path);
		}
	}

	fn get_value(&self, section: Option<&str>, key: &str) -> Option<String> {
		if !self.is_ready() {
			return None;
		}
		let sec = section.unwrap_or("DEFAULT");
		self.data.get(sec, key)
	}

	fn set_value(&mut self, section: Option<&str>, key: &str, value: &str) {
		if !self.is_ready() {
			return;
		}
		let sec = section.unwrap_or("DEFAULT");
		self.data.set(sec, key, Some(value.to_string()));
	}

	fn remove_entry(&mut self, section: Option<&str>, key: &str) {
		let sec_name = section.unwrap_or("DEFAULT");
		let _ = self.data.remove_key(sec_name, key);
	}

	fn remove_section(&mut self, section: Option<&str>) {
		if let Some(sec) = section {
			let _ = self.data.remove_section(sec);
		}
	}

	fn iter_section(&self, section: Option<&str>) -> Vec<(String, String)> {
		let sec_name = section.unwrap_or("DEFAULT").to_string();
		self.data
			.get_map_ref()
			.get(&sec_name)
			.map(|props| props.iter().filter_map(|(k, v)| v.as_ref().map(|v| (k.clone(), v.clone()))).collect())
			.unwrap_or_default()
	}

	fn section_names(&self) -> Vec<String> {
		self.data.sections()
	}
}

pub fn get_sorted_document_list(
	config: &ConfigManager,
	open_paths: &[String],
	filter: &str,
) -> Vec<crate::bridge::ffi::FfiDocumentListItem> {
	use crate::bridge::ffi::{DocumentListStatus, FfiDocumentListItem};

	let recent_docs = config.get_recent_documents();
	let all_docs = config.get_all_documents();

	// Start with recent documents
	let mut doc_paths: Vec<String> = Vec::new();
	for path in &recent_docs {
		if !doc_paths.contains(path) {
			doc_paths.push(path.clone());
		}
	}

	// Add remaining documents, sorted alphabetically
	let mut rest: Vec<String> = all_docs.iter().filter(|path| !doc_paths.contains(path)).cloned().collect();

	rest.sort_by(|a, b| {
		let a_path = Path::new(a);
		let b_path = Path::new(b);
		let a_name = a_path.file_name().and_then(|n| n.to_str()).unwrap_or(a);
		let b_name = b_path.file_name().and_then(|n| n.to_str()).unwrap_or(b);

		// Compare filenames case-insensitively
		let name_cmp = a_name.to_lowercase().cmp(&b_name.to_lowercase());
		if name_cmp != std::cmp::Ordering::Equal {
			return name_cmp;
		}

		// If filenames are equal, compare full paths
		a.to_lowercase().cmp(&b.to_lowercase())
	});

	doc_paths.extend(rest);

	// Convert to FfiDocumentListItem, applying filter
	let filter_lower = filter.to_lowercase();
	doc_paths
		.into_iter()
		.filter_map(|path| {
			let path_obj = Path::new(&path);
			let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();

			// Apply filter if specified
			if !filter.is_empty() && !filename.to_lowercase().contains(&filter_lower) {
				return None;
			}

			// Determine status
			let status = if !path_obj.exists() {
				DocumentListStatus::Missing
			} else if open_paths.contains(&path) {
				DocumentListStatus::Open
			} else {
				DocumentListStatus::Closed
			};

			Some(FfiDocumentListItem { path, filename, status })
		})
		.collect()
}

fn format_bool(value: bool) -> String {
	if value { "1".to_string() } else { "0".to_string() }
}

fn parse_bool(value: &str, default_value: bool) -> bool {
	match value.trim().to_ascii_lowercase().as_str() {
		"1" | "true" | "yes" | "on" => true,
		"0" | "false" | "no" | "off" => false,
		_ => default_value,
	}
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

fn get_config_path() -> PathBuf {
	let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
	let exe_dir = exe_path.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
	let mut force_appdata = false;
	#[cfg(windows)]
	{
		if let Ok(program_files) = env::var("ProgramFiles") {
			if exe_path.starts_with(&program_files) {
				force_appdata = true;
			}
		}
		if let Ok(program_files_x86) = env::var("ProgramFiles(x86)") {
			if exe_path.starts_with(&program_files_x86) {
				force_appdata = true;
			}
		}
		if !force_appdata && is_directory_writable(&exe_dir) {
			return exe_dir.join(CONFIG_FILENAME);
		}
	}
	#[cfg(not(windows))]
	if is_directory_writable(&exe_dir) {
		return exe_dir.join(CONFIG_FILENAME);
	}
	let base_dir = config_root_dir().unwrap_or(exe_dir);
	let appdata_dir = base_dir.join(CONFIG_DIRECTORY);
	if !appdata_dir.exists() {
		let _ = fs::create_dir_all(&appdata_dir);
	}
	appdata_dir.join(CONFIG_FILENAME)
}

fn is_directory_writable(path: &Path) -> bool {
	if !path.is_dir() {
		return false;
	}
	let file = path.join(".write_test_tmp");
	OpenOptions::new().write(true).create_new(true).open(&file).and_then(|_| fs::remove_file(&file)).is_ok()
}

fn config_root_dir() -> Option<PathBuf> {
	#[cfg(windows)]
	{
		env::var("APPDATA").or_else(|_| env::var("LOCALAPPDATA")).ok().map(PathBuf::from)
	}
	#[cfg(not(windows))]
	{
		env::var("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")))
			.ok()
	}
}
