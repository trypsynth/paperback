use std::sync::Mutex;

use crate::config::ConfigManager;

/// Thread-safe wrapper around `ConfigManager` for `UniFFI` exposure.
pub struct ConfigManagerFfi {
	inner: Mutex<ConfigManager>,
}

impl ConfigManagerFfi {
	#[must_use]
	pub fn new() -> Self {
		Self { inner: Mutex::new(ConfigManager::new()) }
	}

	pub fn initialize(&self, config_path: String) -> bool {
		self.inner.lock().unwrap().initialize(config_path.into())
	}

	pub fn get_doc_key(&self, path: String) -> String {
		self.inner.lock().unwrap().get_doc_key(&path)
	}

	pub fn get_app_string(&self, key: String, default_value: String) -> String {
		self.inner.lock().unwrap().get_app_string(&key, &default_value)
	}

	pub fn set_app_string(&self, key: String, value: String) {
		self.inner.lock().unwrap().set_app_string(&key, &value);
	}

	pub fn get_app_bool(&self, key: String, default_value: bool) -> bool {
		self.inner.lock().unwrap().get_app_bool(&key, default_value)
	}

	pub fn set_app_bool(&self, key: String, value: bool) {
		self.inner.lock().unwrap().set_app_bool(&key, value);
	}

	pub fn associate_uri_with_local_file(&self, uri: String, local_path: String) {
		self.inner.lock().unwrap().associate_uri_with_local_file(&uri, &local_path);
	}

	pub fn set_document_position(&self, path: String, position: i64) {
		self.inner.lock().unwrap().set_document_position(&path, position);
	}

	pub fn get_document_position(&self, path: String) -> i64 {
		self.inner.lock().unwrap().get_document_position(&path)
	}

	pub fn set_document_password(&self, path: String, password: String) {
		self.inner.lock().unwrap().set_document_password(&path, &password);
	}

	pub fn get_document_password(&self, path: String) -> String {
		self.inner.lock().unwrap().get_document_password(&path)
	}

	pub fn add_recent_document(&self, path: String) {
		self.inner.lock().unwrap().add_recent_document(&path);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_recent_documents()
	}

	pub fn add_opened_document(&self, path: String) {
		self.inner.lock().unwrap().add_opened_document(&path);
	}

	pub fn remove_opened_document(&self, path: String) {
		self.inner.lock().unwrap().remove_opened_document(&path);
	}

	pub fn set_document_opened(&self, path: String, opened: bool) {
		self.inner.lock().unwrap().set_document_opened(&path, opened);
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_opened_documents()
	}

	pub fn remove_document_history(&self, path: String) {
		self.inner.lock().unwrap().remove_document_history(&path);
	}

	pub fn get_supported_extensions(&self) -> Vec<String> {
		let mut exts = std::collections::HashSet::new();
		for parser in crate::parser::ParserRegistry::global().all_parsers() {
			for ext in parser.extensions {
				exts.insert(ext);
			}
		}
		exts.into_iter().collect()
	}

	pub fn get_find_history(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_find_history()
	}

	pub fn add_find_history(&self, text: String, max_len: i32) {
		self.inner.lock().unwrap().add_find_history(&text, max_len as usize);
	}

	pub fn import_document_settings(&self, path: String) {
		self.inner.lock().unwrap().import_document_settings(&path);
	}

	pub fn import_settings_from_file(&self, doc_path: String, import_path: String) {
		self.inner.lock().unwrap().import_settings_from_file(&doc_path, &import_path);
	}

	pub fn export_document_settings(&self, doc_path: String, export_path: String) {
		self.inner.lock().unwrap().export_document_settings(&doc_path, &export_path);
	}

	pub fn flush(&self) {
		self.inner.lock().unwrap().flush();
	}
}

impl Default for ConfigManagerFfi {
	fn default() -> Self {
		Self::new()
	}
}
