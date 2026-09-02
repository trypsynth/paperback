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
	bookmarks.sort_by_key(|b| b.start);
	let items: Vec<ffi::BookmarkDisplayItem> = bookmarks
		.iter()
		.map(|b| ffi::BookmarkDisplayItem {
			start: b.start,
			end: b.end,
			note: b.note.clone(),
			is_whole_line: b.start == b.end,
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
