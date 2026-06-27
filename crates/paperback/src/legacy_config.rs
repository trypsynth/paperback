/// One-time migration from the legacy wxWidgets INI config (Paperback.ini) to TOML.
///
/// Called at startup before `ConfigManager::initialize()`. If an INI file exists
/// and no TOML file exists yet, reads the INI via wxdragon's Config and writes a
/// TOML file that the core's `ConfigManager` can load normally. This module is the
/// only place in the binary that directly constructs `ConfigData`.
use std::fs;

use base64::{Engine, engine::general_purpose::STANDARD};
use paperback_core::config::{ConfigData, DocumentConfig, StoredBookmark};
use toml::Value as TomlValue;
use wxdragon::config::{Config, ConfigStyle};

use crate::config_ext::config_toml_path;

pub fn migrate_if_needed() {
	let toml_path = config_toml_path();
	let ini_path = toml_path.with_extension("ini");
	if toml_path.exists() || !ini_path.exists() {
		return;
	}
	tracing::info!(ini = %ini_path.display(), toml = %toml_path.display(), "migrating config from INI to TOML");
	let data = read_ini(&ini_path);
	match toml::to_string(&data) {
		Ok(serialized) => {
			if let Some(parent) = toml_path.parent() {
				let _ = fs::create_dir_all(parent);
			}
			if let Err(e) = fs::write(&toml_path, &serialized) {
				tracing::error!(path = %toml_path.display(), error = %e, "failed to write migrated config");
			} else {
				tracing::info!("config migration complete");
			}
		}
		Err(e) => tracing::error!(error = %e, "failed to serialize migrated config"),
	}
}

fn read_ini(ini_path: &std::path::Path) -> ConfigData {
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
	data.app.navigation_wrap = config.read_bool("navigation_wrap", false);
	data.app.find_match_case = config.read_bool("find_match_case", false);
	data.app.find_whole_word = config.read_bool("find_whole_word", false);
	data.app.find_use_regex = config.read_bool("find_use_regex", false);
	data.app.recent_documents_to_show = config.read_long("recent_documents_to_show", 25);
	data.app.sleep_timer_duration = config.read_long("sleep_timer_duration", 30);
	// Desktop-only settings go into the extra passthrough map so they are preserved
	// in the TOML file even though core's AppSettings doesn't know about them.
	data.app
		.extra
		.insert("minimize_to_tray".to_string(), TomlValue::Boolean(config.read_bool("minimize_to_tray", false)));
	data.app
		.extra
		.insert("start_maximized".to_string(), TomlValue::Boolean(config.read_bool("start_maximized", false)));
	data.app.extra.insert("compact_go_menu".to_string(), TomlValue::Boolean(config.read_bool("compact_go_menu", true)));
	data.app.extra.insert(
		"check_for_updates_on_startup".to_string(),
		TomlValue::Boolean(config.read_bool("check_for_updates_on_startup", true)),
	);
	data.app.extra.insert("language".to_string(), TomlValue::String(config.read_string("language", "")));
	data.app.extra.insert("active_document".to_string(), TomlValue::String(config.read_string("active_document", "")));
	data.app
		.extra
		.insert("update_channel".to_string(), TomlValue::String(config.read_string("update_channel", "stable")));
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
			doc.bookmarks.sort_by_key(|a| a.start);
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
	STANDARD.decode(encoded).map(|bytes: Vec<u8>| String::from_utf8_lossy(&bytes).to_string()).unwrap_or_default()
}
