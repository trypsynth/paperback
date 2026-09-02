//! The recent-documents and opened-documents lists, plus the small per-document housekeeping
//! operations (rename, forget) that touch both those lists and the per-document config entries
//! together. Per-document *content* (position, bookmarks, ...) lives in
//! [`super::document_state`]/[`super::bookmarks`] instead - this file is about which paths the
//! app knows about and in what lists, not what's stored against any one of them.

use std::path::Path;

use super::ConfigManager;

const MAX_RECENT_DOCUMENTS_TO_SHOW: usize = 100;

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
			while data.recent_documents.len() > MAX_RECENT_DOCUMENTS_TO_SHOW {
				data.recent_documents.pop();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().recent_documents.clone()
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

	pub fn get_opened_documents_existing(&self) -> Vec<String> {
		self.get_opened_documents().into_iter().filter(|path| Path::new(path).exists()).collect()
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
