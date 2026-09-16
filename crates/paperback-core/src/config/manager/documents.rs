//! The recent-documents and opened-documents lists, plus the small per-document housekeeping
//! operations (rename, forget) that touch both those lists and the per-document config entries
//! together. Per-document *content* (position, bookmarks, ...) lives in
//! [`super::document_state`]/[`super::bookmarks`] instead - this file is about which paths the
//! app knows about and in what lists, not what's stored against any one of them.

use super::ConfigManager;

const MAX_RECENT_DOCUMENTS: usize = 100;

impl ConfigManager {
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
		}
		self.dirty.set(true);
		self.trim_recent_documents();
	}

	/// Drops stored recent documents beyond the `recent_documents_to_show` setting.
	pub fn trim_recent_documents(&self) {
		if !self.initialized {
			return;
		}
		let mut data = self.data.borrow_mut();
		let limit = usize::try_from(data.app.recent_documents_to_show.max(0)).unwrap_or(0).min(MAX_RECENT_DOCUMENTS);
		if data.recent_documents.len() > limit {
			data.recent_documents.truncate(limit);
			self.dirty.set(true);
		}
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().recent_documents.clone()
	}

	/// Empties the recent list; per-document entries (positions, bookmarks) are kept.
	pub fn clear_recent_documents(&self) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().recent_documents.clear();
		self.dirty.set(true);
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

	pub fn clear_opened_documents(&self) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().opened_documents.clear();
		self.dirty.set(true);
	}

	/// Sets the per-document opened flag. Prefer `add_opened_document`/`remove_opened_document`
	/// for maintaining the opened-documents list.
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
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.documents.remove(&key);
		}
		self.dirty.set(true);
	}

	pub fn rename_document_path(&self, old_path: &str, new_path: &str) {
		if !self.initialized {
			return;
		}
		let mut data = self.data.borrow_mut();
		for p in &mut data.recent_documents {
			if p == old_path {
				*p = new_path.to_string();
			}
		}
		for p in &mut data.opened_documents {
			if p == old_path {
				*p = new_path.to_string();
			}
		}
		if let Some(doc_key) = data.path_hashes.remove(old_path) {
			data.path_hashes.insert(new_path.to_string(), doc_key.clone());
			if let Some(doc) = data.documents.get_mut(&doc_key) {
				doc.path = new_path.to_string();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_all_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().documents.values().map(|d| d.path.clone()).filter(|p| !p.is_empty()).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn config_with_limit(limit: i32) -> ConfigManager {
		let mut config = ConfigManager::new();
		config.initialized = true;
		config.set_app_int("recent_documents_to_show", limit);
		config
	}

	#[test]
	fn add_recent_document_trims_to_setting() {
		let config = config_with_limit(2);
		for path in ["a", "b", "c"] {
			config.add_recent_document(path);
		}
		assert_eq!(config.get_recent_documents(), vec!["c", "b"]);
	}

	#[test]
	fn trim_recent_documents_caps_at_hard_maximum() {
		let config = config_with_limit(500);
		for i in 0..101 {
			config.add_recent_document(&format!("doc{i}"));
		}
		assert_eq!(config.get_recent_documents().len(), 100);
	}

	#[test]
	fn trim_recent_documents_shrinks_after_setting_lowered() {
		let config = config_with_limit(25);
		for path in ["a", "b", "c", "d", "e"] {
			config.add_recent_document(path);
		}
		config.set_app_int("recent_documents_to_show", 3);
		config.trim_recent_documents();
		assert_eq!(config.get_recent_documents(), vec!["e", "d", "c"]);
	}

	#[test]
	fn trim_recent_documents_is_a_no_op_when_within_limit() {
		let config = config_with_limit(25);
		config.add_recent_document("a");
		config.add_recent_document("b");
		config.dirty.set(false);
		config.trim_recent_documents();
		assert_eq!(config.get_recent_documents(), vec!["b", "a"]);
		assert!(!config.dirty.get());
	}

	#[test]
	fn clear_recent_documents_keeps_document_entries() {
		let config = config_with_limit(25);
		config.add_recent_document("a");
		config.clear_recent_documents();
		assert!(config.get_recent_documents().is_empty());
		assert_eq!(config.get_all_documents(), vec!["a"]);
	}

	#[test]
	fn remove_opened_document_matches_stored_string() {
		let config = config_with_limit(25);
		config.add_opened_document("a");
		config.remove_opened_document("a");
		assert!(config.get_opened_documents().is_empty());
	}

	#[test]
	fn remove_opened_document_leaves_other_entries() {
		let config = config_with_limit(25);
		config.add_opened_document("a");
		config.add_opened_document("b");
		config.remove_opened_document("a");
		assert_eq!(config.get_opened_documents(), vec!["b"]);
	}

	#[test]
	fn clear_opened_documents_empties_list() {
		let config = config_with_limit(25);
		config.add_opened_document("a");
		config.add_opened_document("b");
		config.clear_opened_documents();
		assert!(config.get_opened_documents().is_empty());
	}
}
