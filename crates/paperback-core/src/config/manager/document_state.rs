use super::ConfigManager;
use crate::{
	config::settings::{DocumentSpeech, NavigationHistory},
	document::{DocPos, ReplaceOutcome},
};

impl ConfigManager {
	/// Inserting OCR text on page 40 moves every stored position after it, so they would all drift without this.
	pub fn shift_document_positions(&self, path: &str, outcome: &ReplaceOutcome) {
		if !self.initialized || outcome.is_empty() {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			doc.last_position = outcome.shift(doc.last_position);
			for position in &mut doc.navigation_history {
				*position = outcome.shift(*position);
			}
			if let Some(position) = doc.temporary_bookmark.as_mut() {
				*position = outcome.shift(*position);
			}
			for bookmark in &mut doc.bookmarks {
				bookmark.start = outcome.shift(bookmark.start);
				bookmark.end = outcome.shift(bookmark.end);
			}
			doc.bookmarks.sort_by_key(|bookmark| bookmark.start);
		}
		self.dirty.set(true);
	}

	pub fn set_document_position(&self, path: &str, position: DocPos) {
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

	/// `None` leaves the stored value alone, so a document that hasn't started playing can't wipe it.
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
	pub fn get_document_position(&self, path: &str) -> DocPos {
		if !self.initialized {
			return DocPos::ZERO;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map_or(DocPos::ZERO, |d| d.last_position)
	}

	#[must_use]
	pub fn get_validated_document_position(&self, path: &str, max_position: DocPos) -> Option<DocPos> {
		let saved = self.get_document_position(path);
		(saved > DocPos::ZERO && saved <= max_position).then_some(saved)
	}

	pub fn set_temporary_bookmark(&self, path: &str, position: Option<DocPos>) {
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
	pub fn get_temporary_bookmark(&self, path: &str) -> Option<DocPos> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.temporary_bookmark)
	}

	pub fn set_navigation_history(&self, path: &str, history: &[DocPos], history_index: usize) {
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
			nav.positions.clone_from(&doc.navigation_history);
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

	pub fn set_document_speech(&self, path: &str, speech: &DocumentSpeech) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).speech = speech.clone();
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_document_speech(&self, path: &str) -> DocumentSpeech {
		if !self.initialized {
			return DocumentSpeech::default();
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map(|d| d.speech.clone()).unwrap_or_default()
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

	pub fn set_document_download(&self, path: &str, name: &str, size: u64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let entry = Self::doc_entry_mut(&mut data, key, path);
			entry.download_size = Some(size);
			entry.download_name = Some(name.to_string());
		}
		self.dirty.set(true);
	}

	pub fn get_document_download_name(&self, path: &str) -> Option<String> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.download_name.clone())
	}

	pub fn get_document_download_size(&self, path: &str) -> Option<u64> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.download_size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_download_size_is_kept_for_each_link() {
		let config = ConfigManager::in_memory();
		assert_eq!(config.get_document_download_size("https://example.org/a.epub"), None);
		config.set_document_download("https://example.org/a.epub", "a.epub", 1_500_000);
		config.set_document_download("https://example.org/a.epub", "a.epub", 1_600_000);
		assert_eq!(config.get_document_download_size("https://example.org/a.epub"), Some(1_600_000));
		assert_eq!(config.get_document_download_size("https://example.org/b.epub"), None);
	}

	#[test]
	fn the_download_name_is_kept_for_each_link() {
		let config = ConfigManager::in_memory();
		assert_eq!(config.get_document_download_name("https://example.org/get?id=1"), None);
		config.set_document_download("https://example.org/get?id=1", "Old.epub", 10);
		config.set_document_download("https://example.org/get?id=1", "Moby Dick.epub", 10);
		assert_eq!(
			config.get_document_download_name("https://example.org/get?id=1").as_deref(),
			Some("Moby Dick.epub")
		);
		assert_eq!(config.get_document_download_name("https://example.org/get?id=2"), None);
	}

	#[test]
	fn temporary_bookmark_set_get_overwrite_clear() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let path = "book.epub";
		assert_eq!(config.get_temporary_bookmark(path), None);
		config.set_temporary_bookmark(path, Some(42_000.into()));
		assert_eq!(config.get_temporary_bookmark(path), Some(42_000.into()));
		config.set_temporary_bookmark(path, Some(43_000.into()));
		assert_eq!(config.get_temporary_bookmark(path), Some(43_000.into()));
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
