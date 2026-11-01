use crate::{
	config::{Bookmark, ConfigManager as RustConfigManager},
	document::DocPos,
	types as ffi,
};

pub fn bookmark_navigate(
	manager: &RustConfigManager,
	path: &str,
	position: DocPos,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> ffi::BookmarkNavResult {
	let mut bookmarks: Vec<Bookmark> = manager.get_bookmarks(path);
	if notes_only {
		bookmarks.retain(|b| !b.note.is_empty());
	}
	if bookmarks.is_empty() {
		return ffi::BookmarkNavResult {
			found: false,
			start: DocPos::ZERO,
			note: String::new(),
			index: -1,
			wrapped: false,
		};
	}
	bookmarks.sort_by_key(|b| b.start);
	let mut wrapped = false;
	let mut hit = if next {
		bookmarks.iter().position(|b| b.start > position)
	} else {
		bookmarks.iter().rposition(|b| b.start < position)
	};
	if hit.is_none() && wrap {
		wrapped = true;
		hit = Some(if next { 0 } else { bookmarks.len() - 1 });
	}
	let hit = hit.map(|idx| (idx, bookmarks[idx].clone()));
	if let Some((idx, bm)) = hit {
		let index = i32::try_from(idx).unwrap_or(-1);
		return ffi::BookmarkNavResult { found: true, start: bm.start, note: bm.note, index, wrapped };
	}
	ffi::BookmarkNavResult { found: false, start: DocPos::ZERO, note: String::new(), index: -1, wrapped }
}

/// Playback has moved a moment past a bookmark just jumped to, so without this "previous" would find it again, as a media player's previous-track button allows for.
const PREVIOUS_SLACK_MS: u64 = 1500;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AudioBookmarkHit {
	pub audio_ms: u64,
	pub start: DocPos,
	pub note: String,
	pub index: usize,
	pub wrapped: bool,
}

/// Separate from [`bookmark_navigate`] because every bookmark in an audiobook chapter shares the chapter's position, so positions cannot order them.
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

pub fn bookmark_note_at_position(manager: &RustConfigManager, path: &str, position: DocPos) -> String {
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
	current_pos: DocPos,
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
	let closest_index = bookmarks
		.iter()
		.enumerate()
		.min_by_key(|(_, b)| b.start.since(current_pos).max(current_pos.since(b.start)))
		.map_or(-1, |(idx, _)| i32::try_from(idx).unwrap_or(-1));
	ffi::FilteredBookmarks { items, closest_index }
}

#[cfg(test)]
mod audio_tests {
	use super::*;

	fn config_with(times: &[(u64, &str)]) -> RustConfigManager {
		let config = RustConfigManager::in_memory();
		for (ms, note) in times {
			config.add_audio_bookmark("book.mp3", 0.into(), *ms, note);
		}
		config
	}

	#[test]
	fn next_goes_to_the_first_bookmark_after_the_current_time() {
		let config = config_with(&[(10_000, ""), (60_000, "")]);
		let hit = audio_bookmark_navigate(&config, "book.mp3", 20_000, false, true, false).unwrap();
		assert_eq!((60_000, 1, false), (hit.audio_ms, hit.index, hit.wrapped));
	}

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
		config.add_bookmark("book.mp3", 0.into(), 0.into(), "");
		assert!(audio_bookmark_navigate(&config, "book.mp3", 0, true, true, false).is_none());
	}
}
