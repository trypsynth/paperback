//! Bookmark navigation and filtering, backed by [`ConfigManager`]'s per-document bookmark list.

use crate::{
	config::{Bookmark, ConfigManager as RustConfigManager},
	types as ffi,
};

pub fn bookmark_navigate(
	manager: &RustConfigManager,
	path: &str,
	position: i64,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> ffi::BookmarkNavResult {
	let mut bookmarks: Vec<Bookmark> = manager.get_bookmarks(path);
	if notes_only {
		bookmarks.retain(|b| !b.note.is_empty());
	}
	if bookmarks.is_empty() {
		return ffi::BookmarkNavResult { found: false, start: -1, note: String::new(), index: -1, wrapped: false };
	}
	bookmarks.sort_by_key(|b| b.start);
	let find_from = |from: i64, forward: bool, list: &[Bookmark]| -> Option<(usize, Bookmark)> {
		if forward {
			list.iter().enumerate().find(|(_, b)| b.start > from).map(|(i, b)| (i, b.clone()))
		} else {
			list.iter().enumerate().rev().find(|(_, b)| b.start < from).map(|(i, b)| (i, b.clone()))
		}
	};
	let mut wrapped = false;
	let mut hit = if next { find_from(position, true, &bookmarks) } else { find_from(position, false, &bookmarks) };
	if hit.is_none() && wrap {
		wrapped = true;
		hit = if next { find_from(-1, true, &bookmarks) } else { find_from(i64::MAX / 2, false, &bookmarks) };
	}
	if let Some((idx, bm)) = hit {
		let index = i32::try_from(idx).unwrap_or(-1);
		return ffi::BookmarkNavResult { found: true, start: bm.start, note: bm.note, index, wrapped };
	}
	ffi::BookmarkNavResult { found: false, start: -1, note: String::new(), index: -1, wrapped }
}

/// How far behind the current playback time a bookmark has to be for "previous" to go to it.
///
/// Jumping to a bookmark and then asking for the previous one would otherwise find the bookmark
/// just jumped to, since playback has moved a moment past it and it now counts as behind. This is
/// the same allowance a media player gives its "previous track" button.
const PREVIOUS_SLACK_MS: u64 = 1500;

/// Where an audio bookmark search landed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AudioBookmarkHit {
	/// The playback time to seek to.
	pub audio_ms: u64,
	/// The chapter's position in the text, for the caret.
	pub start: i64,
	pub note: String,
	/// The bookmark's place among the ones searched, counting from zero.
	pub index: usize,
	pub wrapped: bool,
}

/// The next or previous audio bookmark from `current_ms`, for a document that is only audio.
///
/// The playback-time counterpart to [`bookmark_navigate`]. It has to be separate because an
/// audiobook's text positions cannot order its bookmarks: every bookmark in a chapter shares the
/// chapter's position. Text bookmarks are ignored.
pub fn audio_bookmark_navigate(
	manager: &RustConfigManager,
	path: &str,
	current_ms: u64,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> Option<AudioBookmarkHit> {
	let mut bookmarks: Vec<(u64, Bookmark)> = manager
		.get_bookmarks(path)
		.into_iter()
		.filter(|bm| !notes_only || !bm.note.is_empty())
		.filter_map(|bm| bm.audio_ms.map(|ms| (ms, bm)))
		.collect();
	bookmarks.sort_by_key(|(ms, _)| *ms);
	let found = |list: &[(u64, Bookmark)]| {
		if next {
			list.iter().position(|(ms, _)| *ms > current_ms)
		} else {
			list.iter().rposition(|(ms, _)| *ms + PREVIOUS_SLACK_MS < current_ms)
		}
	};
	let (index, wrapped) = match found(&bookmarks) {
		Some(index) => (index, false),
		None if wrap && !bookmarks.is_empty() => (if next { 0 } else { bookmarks.len() - 1 }, true),
		None => return None,
	};
	let (audio_ms, bookmark) = bookmarks.swap_remove(index);
	Some(AudioBookmarkHit { audio_ms, start: bookmark.start, note: bookmark.note, index, wrapped })
}

pub fn bookmark_note_at_position(manager: &RustConfigManager, path: &str, position: i64) -> String {
	manager
		.get_bookmarks(path)
		.into_iter()
		.find(|bm| bm.start == position && !bm.note.is_empty())
		.map(|bm| bm.note)
		.unwrap_or_default()
}

pub fn get_filtered_bookmarks(
	manager: &RustConfigManager,
	path: &str,
	current_pos: i64,
	filter: ffi::BookmarkFilterType,
) -> ffi::FilteredBookmarks {
	let mut bookmarks: Vec<Bookmark> = manager.get_bookmarks(path);
	match filter {
		ffi::BookmarkFilterType::BookmarksOnly => {
			bookmarks.retain(|b| b.note.is_empty());
		}
		ffi::BookmarkFilterType::NotesOnly => {
			bookmarks.retain(|b| !b.note.is_empty());
		}
		ffi::BookmarkFilterType::All => {}
	}
	bookmarks.sort_by_key(|b| (b.start, b.audio_ms));
	let items: Vec<ffi::BookmarkDisplayItem> = bookmarks
		.iter()
		.map(|b| ffi::BookmarkDisplayItem {
			start: b.start,
			end: b.end,
			note: b.note.clone(),
			is_whole_line: b.start == b.end,
			audio_ms: b.audio_ms,
		})
		.collect();
	let closest_index = if bookmarks.is_empty() {
		-1
	} else {
		let mut closest_idx = 0;
		let mut min_distance = i64::MAX;
		for (idx, b) in bookmarks.iter().enumerate() {
			let distance = (b.start - current_pos).abs();
			if distance < min_distance {
				min_distance = distance;
				closest_idx = idx;
			}
		}
		i32::try_from(closest_idx).unwrap_or(-1)
	};
	ffi::FilteredBookmarks { items, closest_index }
}

#[cfg(test)]
mod audio_tests {
	use super::*;

	fn config_with(times: &[(u64, &str)]) -> RustConfigManager {
		let config = RustConfigManager::in_memory();
		for (ms, note) in times {
			config.add_audio_bookmark("book.mp3", 0, *ms, note);
		}
		config
	}

	#[test]
	fn next_goes_to_the_first_bookmark_after_the_current_time() {
		let config = config_with(&[(10_000, ""), (60_000, "")]);
		let hit = audio_bookmark_navigate(&config, "book.mp3", 20_000, false, true, false).unwrap();
		assert_eq!((60_000, 1, false), (hit.audio_ms, hit.index, hit.wrapped));
	}

	/// Just after jumping to a bookmark, playback is a moment past it. "Previous" has to skip it
	/// and go to the one before, or it would keep landing on the same bookmark.
	#[test]
	fn previous_skips_the_bookmark_playback_just_left() {
		let config = config_with(&[(10_000, ""), (60_000, "")]);
		let hit = audio_bookmark_navigate(&config, "book.mp3", 60_400, false, false, false).unwrap();
		assert_eq!(10_000, hit.audio_ms);
	}

	#[test]
	fn wrapping_goes_round_to_the_other_end() {
		let config = config_with(&[(10_000, ""), (60_000, "")]);
		let hit = audio_bookmark_navigate(&config, "book.mp3", 90_000, true, true, false).unwrap();
		assert_eq!((10_000, true), (hit.audio_ms, hit.wrapped));
		assert!(audio_bookmark_navigate(&config, "book.mp3", 90_000, false, true, false).is_none());
	}

	#[test]
	fn notes_only_skips_bookmarks_without_a_note() {
		let config = config_with(&[(10_000, ""), (60_000, "remember this")]);
		let hit = audio_bookmark_navigate(&config, "book.mp3", 0, false, true, true).unwrap();
		assert_eq!((60_000, "remember this"), (hit.audio_ms, hit.note.as_str()));
	}

	#[test]
	fn text_bookmarks_are_not_audio_bookmarks() {
		let config = RustConfigManager::in_memory();
		config.add_bookmark("book.mp3", 0, 0, "");
		assert!(audio_bookmark_navigate(&config, "book.mp3", 0, true, true, false).is_none());
	}
}
