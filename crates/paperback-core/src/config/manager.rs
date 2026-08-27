//! [`ConfigManager`], the stateful TOML-backed store built on [`super::settings::ConfigData`]:
//! this file holds its core lifecycle (init/flush) and the content-hash-keyed document lookup
//! every other accessor goes through, with the accessors themselves grouped into submodules by
//! concern - generic app settings, readability, keybindings, the recent/opened document lists,
//! Find, per-document reading state, bookmarks, and `.paperback` sidecar import/export - plus
//! [`get_sorted_document_list`] and [`compute_document_hash`], the two free functions built on
//! top of it.

use std::{
	cell::{Cell, RefCell},
	fs,
	path::PathBuf,
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha1::{Digest, Sha1};

use super::settings::{ConfigData, DocumentConfig};

mod app_settings;
mod bookmarks;
mod document_list;
mod document_state;
mod documents;
mod find;
mod keybindings;
mod readability;
mod sidecar;

pub use document_list::{compute_document_hash, get_sorted_document_list};

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
			fs::read_to_string(&config_path)
				.ok()
				.and_then(|s| toml::from_str::<ConfigData>(&s).ok())
				.map_or_else(|| (ConfigData::default(), true), |d| (d, false))
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
		if !self.initialized || !self.dirty.get() {
			return;
		}
		let data = self.data.borrow();
		if let Ok(s) = toml::to_string_pretty(&*data) {
			let _ = fs::write(&self.config_path, s);
			self.dirty.set(false);
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
