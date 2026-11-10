use std::{
	cell::{Cell, RefCell},
	ffi::OsString,
	fs, io,
	path::{Path, PathBuf},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha1::{Digest, Sha1};

use super::{
	ActionId, KeyChord,
	settings::{ConfigData, DocumentConfig},
};

mod app_settings;
mod bookmarks;
mod document_list;
mod document_state;
mod documents;
mod find;
mod keybindings;
mod readability;
mod sidecar;

pub use document_list::{
	compute_document_hash, document_missing, document_name, filter_document_list, get_sorted_document_list,
};

pub struct ConfigManager {
	data: RefCell<ConfigData>,
	config_path: PathBuf,
	dirty: Cell<bool>,
	initialized: bool,
	// Parsed once per change, as every keystroke looks a chord up here.
	shortcut_chords: RefCell<Vec<(ActionId, KeyChord)>>,
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
			shortcut_chords: RefCell::new(super::ShortcutsConfig::default().chords()),
		}
	}

	#[cfg(test)]
	pub(crate) fn in_memory() -> Self {
		let mut manager = Self::new();
		manager.initialized = true;
		manager
	}

	pub fn initialize(&mut self, config_path: PathBuf) -> bool {
		let parsed = match fs::read_to_string(&config_path) {
			Ok(s) => Some(toml::from_str::<ConfigData>(&s).map_err(|e| e.to_string())),
			Err(e) if e.kind() == io::ErrorKind::NotFound => None,
			Err(e) => Some(Err(e.to_string())),
		};
		let (data, needs_save) = match parsed {
			None => (ConfigData::default(), true),
			Some(Ok(d)) => (d, false),
			Some(Err(e)) => {
				// Bookmarks and positions for every book live in this file, so it is kept for recovery rather than written over.
				let aside = sibling(&config_path, ".bad");
				tracing::warn!(error = %e, aside = %aside.display(), "config could not be read; starting from defaults");
				let moved = fs::rename(&config_path, &aside).is_ok();
				(ConfigData::default(), moved)
			}
		};
		self.config_path = config_path;
		self.initialized = true;
		*self.shortcut_chords.borrow_mut() = data.app.shortcuts.chords();
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
		data.path_hashes.insert(uri.to_string(), new_key);
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
		if !self.initialized || !self.dirty.get() || self.config_path.as_os_str().is_empty() {
			return;
		}
		let data = self.data.borrow();
		let Ok(s) = toml::to_string_pretty(&*data) else {
			return;
		};
		// A crash mid-write would otherwise leave a truncated file, which the next launch can't parse.
		let tmp = sibling(&self.config_path, ".tmp");
		match fs::write(&tmp, s).and_then(|()| fs::rename(&tmp, &self.config_path)) {
			Ok(()) => self.dirty.set(false),
			Err(e) => tracing::warn!(error = %e, "could not save config"),
		}
	}

	fn doc_entry_mut<'a>(data: &'a mut ConfigData, key: String, path: &str) -> &'a mut DocumentConfig {
		let entry = data.documents.entry(key).or_default();
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

fn sibling(path: &Path, suffix: &str) -> PathBuf {
	let mut name = OsString::from(path.as_os_str());
	name.push(suffix);
	PathBuf::from(name)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn an_unparseable_config_is_kept_aside_not_overwritten() {
		let dir = std::env::temp_dir().join(format!("paperback-config-aside-{}", std::process::id()));
		let _ = fs::remove_dir_all(&dir);
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join("paperback.toml");
		fs::write(&path, "not = [valid").unwrap();
		let mut config = ConfigManager::new();
		config.initialize(path.clone());
		assert_eq!(fs::read_to_string(sibling(&path, ".bad")).unwrap(), "not = [valid");
		assert!(toml::from_str::<ConfigData>(&fs::read_to_string(&path).unwrap()).is_ok());
		assert!(!sibling(&path, ".tmp").exists());
		drop(config);
		let _ = fs::remove_dir_all(&dir);
	}

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
