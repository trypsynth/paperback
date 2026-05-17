use std::sync::Mutex;

use crate::config::ConfigManager;

/// Thread-safe wrapper around ConfigManager for UniFFI exposure.
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

	pub fn set_document_position(&self, path: String, position: i64) {
		self.inner.lock().unwrap().set_document_position(&path, position);
	}

	pub fn get_document_position(&self, path: String) -> i64 {
		self.inner.lock().unwrap().get_document_position(&path)
	}

	pub fn add_recent_document(&self, path: String) {
		self.inner.lock().unwrap().add_recent_document(&path);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_recent_documents()
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
