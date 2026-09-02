//! Per-document reading state: last position, audio playback time, the temporary bookmark,
//! navigation history, and the detected format/password used to reopen it.

use super::ConfigManager;
use crate::config::settings::NavigationHistory;

impl ConfigManager {
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

	/// Records how far into its narration an audio document had played. `None` leaves any
	/// stored value alone, so a document that hasn't started playing can't wipe it.
	pub fn set_document_audio_time(&self, path: &str, time_ms: Option<u64>) {
		if !self.initialized {
			return;
		}
		let Some(time_ms) = time_ms else { return };
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).audio_time_ms = Some(time_ms);
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_document_audio_time(&self, path: &str) -> Option<u64> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.audio_time_ms)
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

	/// Sets the single per-document temporary bookmark position (`None` clears it).
	pub fn set_temporary_bookmark(&self, path: &str, position: Option<i64>) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).temporary_bookmark = position;
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_temporary_bookmark(&self, path: &str) -> Option<i64> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.temporary_bookmark)
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn temporary_bookmark_set_get_overwrite_clear() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let path = "book.epub";
		assert_eq!(config.get_temporary_bookmark(path), None);
		config.set_temporary_bookmark(path, Some(42_000));
		assert_eq!(config.get_temporary_bookmark(path), Some(42_000));
		config.set_temporary_bookmark(path, Some(43_000));
		assert_eq!(config.get_temporary_bookmark(path), Some(43_000));
		config.set_temporary_bookmark(path, None);
		assert_eq!(config.get_temporary_bookmark(path), None);
	}

	#[test]
	fn audio_time_set_get_and_overwrite() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let path = "book.zip";
		assert_eq!(config.get_document_audio_time(path), None);
		config.set_document_audio_time(path, Some(24_739_688));
		assert_eq!(config.get_document_audio_time(path), Some(24_739_688));
		config.set_document_audio_time(path, Some(24_800_000));
		assert_eq!(config.get_document_audio_time(path), Some(24_800_000));
	}

	/// A player that hasn't established a position yet reports `None`, and that must not wipe
	/// the stored time, since otherwise merely opening a book would discard where it was up to.
	#[test]
	fn audio_time_none_does_not_clear_a_stored_value() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let path = "book.zip";
		config.set_document_audio_time(path, Some(5_000));
		config.set_document_audio_time(path, None);
		assert_eq!(config.get_document_audio_time(path), Some(5_000));
	}
}
