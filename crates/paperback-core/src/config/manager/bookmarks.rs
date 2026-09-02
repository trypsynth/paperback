//! Per-document bookmarks: add/remove/toggle by `[start, end)` range, editing a bookmark's note,
//! and reading them all back.

use super::ConfigManager;
use crate::config::settings::{Bookmark, StoredBookmark};

impl ConfigManager {
	pub fn add_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if doc.bookmarks.iter().any(|bm| bm.start == start && bm.end == end) {
				return;
			}
			doc.bookmarks.push(StoredBookmark { start, end, note: note.to_string() });
			doc.bookmarks.sort_by_key(|a| a.start);
		}
		self.dirty.set(true);
	}

	pub fn remove_bookmark(&self, path: &str, start: i64, end: i64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(idx) = doc.bookmarks.iter().position(|bm| bm.start == start && bm.end == end) {
				doc.bookmarks.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn toggle_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.start == start && bm.end == end) {
			self.remove_bookmark(path, start, end);
		} else {
			self.add_bookmark(path, start, end, note);
		}
	}

	pub fn update_bookmark_note(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(bm) = doc.bookmarks.iter_mut().find(|bm| bm.start == start && bm.end == end) {
				bm.note = note.to_string();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_bookmarks(&self, path: &str) -> Vec<Bookmark> {
		if !self.initialized {
			return Vec::new();
		}
		self.data
			.borrow()
			.documents
			.get(&self.get_doc_key(path))
			.map(|d| {
				d.bookmarks.iter().map(|bm| Bookmark { start: bm.start, end: bm.end, note: bm.note.clone() }).collect()
			})
			.unwrap_or_default()
	}
}
