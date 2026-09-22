//! Per-document bookmarks: add/remove/toggle by `[start, end)` range, editing a bookmark's note,
//! and reading them all back.
//!
//! A bookmark in a document that is only audio is identified by its playback time instead (see
//! [`StoredBookmark::audio_ms`]), since its text position is shared by everything in its
//! chapter. The range functions only ever touch text bookmarks and the `audio_` ones only audio
//! bookmarks, so a text bookmark and an audio one at the same position never get confused.

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
			if doc.bookmarks.iter().any(|bm| is_text_bookmark_at(bm, start, end)) {
				return;
			}
			doc.bookmarks.push(StoredBookmark { start, end, note: note.to_string(), audio_ms: None });
			sort_bookmarks(&mut doc.bookmarks);
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
			if let Some(idx) = doc.bookmarks.iter().position(|bm| is_text_bookmark_at(bm, start, end)) {
				doc.bookmarks.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn toggle_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.audio_ms.is_none() && bm.start == start && bm.end == end) {
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
			if let Some(bm) = doc.bookmarks.iter_mut().find(|bm| is_text_bookmark_at(bm, start, end)) {
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
				d.bookmarks
					.iter()
					.map(|bm| Bookmark { start: bm.start, end: bm.end, note: bm.note.clone(), audio_ms: bm.audio_ms })
					.collect()
			})
			.unwrap_or_default()
	}

	/// Adds a bookmark at `audio_ms` in the recording, filed under the chapter at `position`.
	pub fn add_audio_bookmark(&self, path: &str, position: i64, audio_ms: u64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if doc.bookmarks.iter().any(|bm| bm.audio_ms == Some(audio_ms)) {
				return;
			}
			doc.bookmarks.push(StoredBookmark {
				start: position,
				end: position,
				note: note.to_string(),
				audio_ms: Some(audio_ms),
			});
			sort_bookmarks(&mut doc.bookmarks);
		}
		self.dirty.set(true);
	}

	pub fn remove_audio_bookmark(&self, path: &str, audio_ms: u64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			doc.bookmarks.retain(|bm| bm.audio_ms != Some(audio_ms));
		}
		self.dirty.set(true);
	}

	pub fn update_audio_bookmark_note(&self, path: &str, audio_ms: u64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(bm) = doc.bookmarks.iter_mut().find(|bm| bm.audio_ms == Some(audio_ms)) {
				bm.note = note.to_string();
			}
		}
		self.dirty.set(true);
	}

	/// The audio bookmark nearest `audio_ms` and no further than `within_ms` from it.
	///
	/// Playback moves on between one key press and the next, so pressing "toggle bookmark" twice
	/// never lands on the same millisecond. This is what lets the second press find the bookmark
	/// the first one set.
	pub fn audio_bookmark_near(&self, path: &str, audio_ms: u64, within_ms: u64) -> Option<Bookmark> {
		self.get_bookmarks(path)
			.into_iter()
			.filter_map(|bm| bm.audio_ms.map(|ms| (ms.abs_diff(audio_ms), bm)))
			.filter(|(distance, _)| *distance <= within_ms)
			.min_by_key(|(distance, _)| *distance)
			.map(|(_, bm)| bm)
	}
}

fn is_text_bookmark_at(bookmark: &StoredBookmark, start: i64, end: i64) -> bool {
	bookmark.audio_ms.is_none() && bookmark.start == start && bookmark.end == end
}

/// Document order: by position, and within one position (the chapter, for an audiobook) by time.
fn sort_bookmarks(bookmarks: &mut [StoredBookmark]) {
	bookmarks.sort_by_key(|bm| (bm.start, bm.audio_ms));
}

#[cfg(test)]
mod tests {
	use super::*;

	fn config() -> ConfigManager {
		let mut config = ConfigManager::new();
		config.initialized = true;
		config
	}

	/// The reported case: a chapterless MP3's text is one character, so every bookmark in it is
	/// at position 0. Stored by position alone, the second bookmark found the first and removed it.
	#[test]
	fn two_audio_bookmarks_in_one_chapter_are_both_kept() {
		let config = config();
		config.add_audio_bookmark("book.mp3", 0, 10_000, "");
		config.add_audio_bookmark("book.mp3", 0, 60_000, "");
		let times: Vec<_> = config.get_bookmarks("book.mp3").iter().map(|bm| bm.audio_ms).collect();
		assert_eq!(vec![Some(10_000), Some(60_000)], times);
	}

	/// A second press a moment later has to find the bookmark the first press set.
	#[test]
	fn a_bookmark_a_moment_away_is_found_but_one_further_off_is_not() {
		let config = config();
		config.add_audio_bookmark("book.mp3", 0, 10_000, "");
		assert_eq!(Some(10_000), config.audio_bookmark_near("book.mp3", 10_400, 1000).and_then(|bm| bm.audio_ms));
		assert!(config.audio_bookmark_near("book.mp3", 12_000, 1000).is_none());
	}

	/// A text bookmark and an audio bookmark at the same position are different bookmarks, so
	/// removing or editing one must not touch the other.
	#[test]
	fn text_and_audio_bookmarks_at_one_position_stay_separate() {
		let config = config();
		config.add_bookmark("book.mp3", 0, 0, "text");
		config.add_audio_bookmark("book.mp3", 0, 5000, "audio");
		config.remove_bookmark("book.mp3", 0, 0);
		let left = config.get_bookmarks("book.mp3");
		assert_eq!(1, left.len());
		assert_eq!(Some(5000), left[0].audio_ms);
		config.update_audio_bookmark_note("book.mp3", 5000, "edited");
		assert_eq!("edited", config.get_bookmarks("book.mp3")[0].note);
		config.remove_audio_bookmark("book.mp3", 5000);
		assert!(config.get_bookmarks("book.mp3").is_empty());
	}

	/// A config written before audio bookmarks existed has no `audio_ms`, and an ordinary
	/// bookmark written now leaves it out, so neither older nor newer builds see a change.
	#[test]
	fn a_text_bookmark_serializes_exactly_as_it_did_before() {
		let stored = StoredBookmark { start: 3, end: 7, note: String::new(), audio_ms: None };
		assert!(!toml::to_string(&stored).unwrap().contains("audio_ms"));
		let read: StoredBookmark = toml::from_str("start = 3\nend = 7\n").unwrap();
		assert_eq!(None, read.audio_ms);
	}
}
