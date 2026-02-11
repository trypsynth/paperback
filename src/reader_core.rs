use bitflags::bitflags;
use regex::RegexBuilder;

use crate::{
	config::{Bookmark, ConfigManager as RustConfigManager},
	document::{DocumentHandle, MarkerType},
	parser::is_external_url,
	types::{self as ffi, HeadingInfo},
};

fn heading_info(doc: &DocumentHandle, idx: i32) -> Option<HeadingInfo> {
	doc.heading_info(idx)
}

fn select_marker_index(
	doc: &DocumentHandle,
	position: i64,
	wrap: bool,
	direction: ffi::NavDirection,
	kind: MarkerType,
) -> (Option<usize>, bool) {
	let initial = match direction {
		ffi::NavDirection::Next => doc.next_marker_index(position, kind),
		ffi::NavDirection::Previous => doc.previous_marker_index(position, kind),
	};
	if initial.is_some() {
		return (initial, false);
	}
	if !wrap {
		return (None, false);
	}
	let alt_pos = match direction {
		ffi::NavDirection::Previous => i64::try_from(doc.document().buffer.content.len()).unwrap_or(0) + 1,
		ffi::NavDirection::Next => -1,
	};
	(
		match direction {
			ffi::NavDirection::Next => doc.next_marker_index(alt_pos, kind),
			ffi::NavDirection::Previous => doc.previous_marker_index(alt_pos, kind),
		},
		true,
	)
}

const fn build_nav_result(found: bool, wrapped: bool, offset: usize, level: i32, text: String) -> ffi::NavResult {
	ffi::NavResult { found, wrapped, offset, marker_level: level, marker_text: text }
}

pub fn reader_navigate(doc: &DocumentHandle, req: &ffi::NavRequest) -> ffi::NavResult {
	use ffi::NavTarget;
	match req.target {
		NavTarget::Section | NavTarget::Page => {
			let kind = if req.target == NavTarget::Section { MarkerType::SectionBreak } else { MarkerType::PageBreak };
			let (idx_opt, wrapped) = select_marker_index(doc, req.position, req.wrap, req.direction, kind);
			if let Some(idx) = idx_opt {
				let Ok(idx_i32) = i32::try_from(idx) else {
					return build_nav_result(false, wrapped, 0, 0, String::new());
				};
				let offset = doc.marker_position(idx_i32).unwrap_or(0);
				return build_nav_result(true, wrapped, offset, 0, String::new());
			}
			build_nav_result(false, wrapped, 0, 0, String::new())
		}
		NavTarget::Heading => {
			let level_filter = if req.level_filter > 0 { Some(req.level_filter) } else { None };
			let (idx_opt, wrapped) = match req.direction {
				ffi::NavDirection::Next => doc.next_heading_index(req.position, level_filter),
				ffi::NavDirection::Previous => doc.previous_heading_index(req.position, level_filter),
			}
			.map_or((None, false), |idx| (usize::try_from(idx).ok(), false));
			let (idx_final, wrapped_final) = if idx_opt.is_none() && req.wrap {
				let alt_pos = match req.direction {
					ffi::NavDirection::Previous => i64::try_from(doc.document().buffer.content.len()).unwrap_or(0) + 1,
					ffi::NavDirection::Next => -1,
				};
				let retry = match req.direction {
					ffi::NavDirection::Next => doc.next_heading_index(alt_pos, level_filter),
					ffi::NavDirection::Previous => doc.previous_heading_index(alt_pos, level_filter),
				};
				let retry_idx = retry.and_then(|i| usize::try_from(i).ok());
				(retry_idx, retry.is_some())
			} else {
				(idx_opt, wrapped)
			};
			if let Some(idx) = idx_final {
				let Ok(idx_i32) = i32::try_from(idx) else {
					return build_nav_result(false, wrapped_final, 0, 0, String::new());
				};
				let offset = doc.marker_position(idx_i32).unwrap_or(0);
				let (level, text) = doc.document().buffer.markers.get(idx).map_or_else(
					|| heading_info(doc, idx_i32).map_or((0, String::new()), |h| (h.level, h.text)),
					|marker| (marker.level, marker.text.clone()),
				);
				return build_nav_result(true, wrapped_final, offset, level, text);
			}
			build_nav_result(false, wrapped_final, 0, 0, String::new())
		}
		NavTarget::List | NavTarget::ListItem | NavTarget::Link | NavTarget::Table | NavTarget::Separator => {
			let kind = match req.target {
				NavTarget::List => MarkerType::List,
				NavTarget::ListItem => MarkerType::ListItem,
				NavTarget::Link => MarkerType::Link,
				NavTarget::Table => MarkerType::Table,
				NavTarget::Separator => MarkerType::Separator,
				_ => unreachable!("NavTarget should only be List, ListItem, Link, Table, or Separator in this branch"),
			};
			let (idx_opt, wrapped) = select_marker_index(doc, req.position, req.wrap, req.direction, kind);
			if let Some(idx) = idx_opt {
				let marker = doc.document().buffer.markers.get(idx);
				let offset = marker.map_or(0, |m| m.position);
				let level = marker.map_or(0, |m| m.level);
				let text = marker.map(|m| m.text.clone()).unwrap_or_default();
				return build_nav_result(true, wrapped, offset, level, text);
			}
			build_nav_result(false, wrapped, 0, 0, String::new())
		}
	}
}

pub fn reader_search(haystack: &str, needle: &str, start: i64, options: SearchOptions) -> i64 {
	if needle.is_empty() {
		return -1;
	}
	let start_utf16 = usize::try_from(start.clamp(0, i64::MAX)).unwrap_or(0);

	let utf16_to_byte_index = |s: &str, utf16_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (byte_idx, ch) in s.char_indices() {
			let len16 = ch.len_utf16();
			if utf16_count >= utf16_idx {
				return byte_idx;
			}
			utf16_count += len16;
		}
		s.len()
	};
	let byte_to_utf16_index = |s: &str, byte_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (idx, ch) in s.char_indices() {
			if idx >= byte_idx {
				break;
			}
			utf16_count += ch.len_utf16();
		}
		utf16_count
	};
	let start_byte = utf16_to_byte_index(haystack, start_utf16);

	// Build regex for search - this avoids copying/lowercasing the entire haystack
	let escaped_needle =
		if options.contains(SearchOptions::REGEX) { needle.to_string() } else { regex::escape(needle) };
	let pattern =
		if options.contains(SearchOptions::WHOLE_WORD) { format!(r"\b{escaped_needle}\b") } else { escaped_needle };
	let mut builder = RegexBuilder::new(&pattern);
	if !options.contains(SearchOptions::MATCH_CASE) {
		builder.case_insensitive(true);
	}
	let Ok(re) = builder.build() else {
		return -1;
	};

	if options.contains(SearchOptions::FORWARD) {
		if let Some(m) = re.find(&haystack[start_byte..]) {
			let byte_pos = start_byte + m.start();
			let utf16_pos = byte_to_utf16_index(haystack, byte_pos);
			return i64::try_from(utf16_pos).unwrap_or(-1);
		}
	} else {
		let mut last: Option<usize> = None;
		let end_byte = start_byte.min(haystack.len());
		for m in re.find_iter(&haystack[..end_byte]) {
			last = Some(m.start());
		}
		if let Some(pos) = last {
			let utf16_pos = byte_to_utf16_index(haystack, pos);
			return i64::try_from(utf16_pos).unwrap_or(-1);
		}
	}
	-1
}

pub fn reader_search_with_wrap(haystack: &str, needle: &str, start: i64, options: SearchOptions) -> ffi::SearchResult {
	let position = reader_search(haystack, needle, start, options);
	if position >= 0 {
		return ffi::SearchResult { found: true, wrapped: false, position };
	}
	let wrap_pos = if options.contains(SearchOptions::FORWARD) {
		0
	} else {
		i64::try_from(haystack.encode_utf16().count()).unwrap_or(0)
	};
	let wrapped_position = reader_search(haystack, needle, wrap_pos, options);
	if wrapped_position >= 0 {
		return ffi::SearchResult { found: true, wrapped: true, position: wrapped_position };
	}
	ffi::SearchResult { found: false, wrapped: false, position: -1 }
}

bitflags! {
	#[derive(Copy, Clone)]
	pub struct SearchOptions: u8 {
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const WHOLE_WORD = 1 << 2;
		const REGEX = 1 << 3;
	}
}

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

fn normalize_index(positions: &[i64], index: usize) -> usize {
	if positions.is_empty() {
		return 0;
	}
	index.min(positions.len().saturating_sub(1))
}

fn trim_history(positions: &mut Vec<i64>, index: &mut usize, max_len: usize) {
	if max_len == 0 {
		return;
	}
	while positions.len() > max_len {
		positions.remove(0);
		if *index > 0 {
			*index -= 1;
		}
	}
}

pub fn record_history_position(positions: &mut Vec<i64>, index: &mut usize, current_pos: i64, max_len: usize) {
	if positions.is_empty() {
		positions.push(current_pos);
		*index = 0;
		trim_history(positions, index, max_len);
		return;
	}
	*index = normalize_index(positions, *index);
	if positions[*index] != current_pos {
		if *index + 1 < positions.len() {
			if positions[*index + 1] != current_pos {
				positions.truncate(*index + 1);
				positions.push(current_pos);
			}
		} else {
			positions.push(current_pos);
		}
		*index += 1;
	}
	trim_history(positions, index, max_len);
}

#[derive(Debug, Clone)]
pub struct HistoryNavResult {
	pub found: bool,
	pub target: i64,
	pub positions: Vec<i64>,
	pub index: usize,
}

pub fn history_go_previous(
	history: &[i64],
	history_index: usize,
	current_pos: i64,
	max_len: usize,
) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { found: false, target: -1, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index > 0 {
		index -= 1;
		let target = positions.get(index).copied().unwrap_or(-1);
		return HistoryNavResult { found: target >= 0, target, positions, index };
	}
	HistoryNavResult { found: false, target: -1, positions, index }
}

pub fn history_go_next(history: &[i64], history_index: usize, current_pos: i64, max_len: usize) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { found: false, target: -1, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index + 1 < positions.len() {
		index += 1;
		let target = positions.get(index).copied().unwrap_or(-1);
		return HistoryNavResult { found: target >= 0, target, positions, index };
	}
	HistoryNavResult { found: false, target: -1, positions, index }
}

#[derive(Debug, Clone)]
pub struct LinkNavigation {
	pub found: bool,
	pub is_external: bool,
	pub offset: usize,
	pub url: String,
}

fn current_section_path(doc: &DocumentHandle, position: usize) -> Option<String> {
	let idx = doc.section_index(position)?;
	let idx = usize::try_from(idx).ok()?;
	let manifest_id = doc.document().spine_items.get(idx)?;
	doc.document().manifest_items.get(manifest_id).cloned()
}

fn find_fragment_offset(doc: &DocumentHandle, fragment: &str, scoped_path: Option<&str>) -> Option<usize> {
	let fragment = fragment.trim_start_matches('#');
	if fragment.is_empty() {
		return None;
	}
	if let Some(path) = scoped_path {
		let key = format!("{path}#{fragment}");
		if let Some(offset) = doc.document().id_positions.get(&key) {
			return Some(*offset);
		}
	}
	doc.document().id_positions.get(fragment).copied()
}

fn find_manifest_id_for_path(doc: &DocumentHandle, path: &str) -> Option<String> {
	doc.document().manifest_items.iter().find_map(|(id, p)| if p == path { Some(id.clone()) } else { None })
}

fn spine_section_bounds(doc: &DocumentHandle, spine_index: usize) -> (usize, usize) {
	let start = i32::try_from(spine_index)
		.ok()
		.and_then(|idx| doc.get_marker_position_by_index(MarkerType::SectionBreak, idx))
		.unwrap_or(0);
	let end = if spine_index + 1 < doc.document().spine_items.len() {
		i32::try_from(spine_index + 1)
			.ok()
			.and_then(|idx| doc.get_marker_position_by_index(MarkerType::SectionBreak, idx))
			.unwrap_or_else(|| doc.document().buffer.content.len())
	} else {
		doc.document().buffer.content.len()
	};
	(start, end)
}

pub fn resolve_link(doc: &DocumentHandle, href: &str, current_position: i64) -> LinkNavigation {
	let href_trimmed = href.trim();
	if href_trimmed.is_empty() {
		return LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() };
	}
	if is_external_url(href_trimmed) {
		return LinkNavigation { found: true, is_external: true, offset: 0, url: href_trimmed.to_string() };
	}
	let current_section = current_section_path(doc, usize::try_from(current_position.max(0)).unwrap_or(0));
	if let Some(fragment) = href_trimmed.strip_prefix('#') {
		if let Some(offset) = find_fragment_offset(doc, fragment, current_section.as_deref()) {
			return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
		}
		return LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() };
	}
	let mut parts = href_trimmed.splitn(2, '#');
	let file_path = parts.next().unwrap_or_default();
	let fragment = parts.next().unwrap_or_default();
	if let Some(manifest_id) = find_manifest_id_for_path(doc, file_path) {
		if let Some(spine_index) = doc.document().spine_items.iter().position(|id| id == &manifest_id) {
			let (section_start, section_end) = spine_section_bounds(doc, spine_index);
			let mut offset = section_start;
			if !fragment.is_empty() {
				if let Some(found) = find_fragment_offset(doc, fragment, Some(file_path)) {
					if found >= section_start && found < section_end {
						offset = found;
					}
				}
			}
			return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
		}
	}
	if !fragment.is_empty() {
		if let Some(offset) = find_fragment_offset(doc, fragment, current_section.as_deref()) {
			return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
		}
	}
	LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() }
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use super::*;
	use crate::document::{Document, DocumentBuffer, DocumentHandle, Marker, MarkerType};

	#[test]
	fn reader_search_handles_basic_and_whole_word() {
		let haystack = "Hello world";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "hello", 0, options), 0);
		let haystack = "hello_world";
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(haystack, "hello", 0, options), -1);
	}

	#[test]
	fn reader_search_handles_utf16_offsets() {
		let haystack = "a😀b";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "b", 0, options), 3);
	}

	#[test]
	fn reader_search_handles_match_case() {
		let haystack = "Hello hello";
		let options = SearchOptions::FORWARD | SearchOptions::MATCH_CASE;
		assert_eq!(reader_search(haystack, "hello", 0, options), 6);
		assert_eq!(reader_search(haystack, "Hello", 0, options), 0);
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "HELLO", 0, options), 0);
	}

	#[test]
	fn reader_search_with_wrap_wraps_forward() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD;
		let result = reader_search_with_wrap(haystack, "a", 1, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 0);
	}

	#[test]
	fn record_history_position_appends_and_trims() {
		let mut positions = vec![1, 2, 3];
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 4, 3);
		assert_eq!(positions, vec![2, 3, 4]);
		assert_eq!(index, 2);
	}

	#[test]
	fn record_history_position_truncates_forward_history() {
		let mut positions = vec![10, 20, 30];
		let mut index = 1;
		record_history_position(&mut positions, &mut index, 25, 10);
		assert_eq!(positions, vec![10, 20, 25]);
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_and_next() {
		let history = vec![10, 20, 30];
		let prev = history_go_previous(&history, 2, 30, 10);
		assert!(prev.found);
		assert_eq!(prev.target, 20);
		assert_eq!(prev.index, 1);
		let next = history_go_next(&history, 0, 10, 10);
		assert!(next.found);
		assert_eq!(next.target, 20);
		assert_eq!(next.index, 1);
	}

	#[test]
	fn reader_search_backward_finds_previous_match() {
		let haystack = "one two one";
		let options = SearchOptions::empty();
		assert_eq!(reader_search(haystack, "one", 11, options), 8);
	}

	#[test]
	fn reader_search_with_regex_invalid_pattern_returns_not_found() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD | SearchOptions::REGEX;
		assert_eq!(reader_search(haystack, "(", 0, options), -1);
	}

	#[test]
	fn reader_search_whole_word_positive_case() {
		let haystack = "alpha beta gamma";
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(haystack, "beta", 0, options), 6);
	}

	#[test]
	fn reader_search_clamps_negative_start_to_zero() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "a", -500, options), 0);
	}

	#[test]
	fn reader_search_with_wrap_wraps_backward() {
		let haystack = "abca";
		let options = SearchOptions::empty();
		let result = reader_search_with_wrap(haystack, "a", 0, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 3);
	}

	#[test]
	fn record_history_position_does_not_duplicate_current_position() {
		let mut positions = vec![10, 20, 30];
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 30, 10);
		assert_eq!(positions, vec![10, 20, 30]);
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_returns_not_found_for_empty_history() {
		let result = history_go_previous(&[], 0, 0, 10);
		assert!(!result.found);
		assert_eq!(result.target, -1);
		assert_eq!(result.positions, Vec::<i64>::new());
	}

	#[test]
	fn history_go_next_returns_not_found_at_end() {
		let history = vec![10, 20];
		let result = history_go_next(&history, 1, 20, 10);
		assert!(!result.found);
		assert_eq!(result.target, -1);
		assert_eq!(result.index, 1);
	}

	fn sample_link_doc_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content("x".repeat(220));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 100));
		let mut manifest_items = HashMap::new();
		manifest_items.insert("id1".to_string(), "chapter1.xhtml".to_string());
		manifest_items.insert("id2".to_string(), "chapter2.xhtml".to_string());
		let mut id_positions = HashMap::new();
		id_positions.insert("chapter1.xhtml#intro".to_string(), 10);
		id_positions.insert("chapter2.xhtml#target".to_string(), 120);
		id_positions.insert("global".to_string(), 180);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.spine_items = vec!["id1".to_string(), "id2".to_string()];
		doc.manifest_items = manifest_items;
		doc.id_positions = id_positions;
		DocumentHandle::new(doc)
	}

	#[test]
	fn resolve_link_handles_empty_and_external_hrefs() {
		let doc = sample_link_doc_handle();
		let empty = resolve_link(&doc, "  ", 0);
		assert!(!empty.found);
		let ext = resolve_link(&doc, "https://example.com", 0);
		assert!(ext.found);
		assert!(ext.is_external);
		assert_eq!(ext.url, "https://example.com");
	}

	#[test]
	fn resolve_link_fragment_prefers_current_section_scoped_id() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "#target", 150);
		assert!(result.found);
		assert!(!result.is_external);
		assert_eq!(result.offset, 120);
	}

	#[test]
	fn resolve_link_fragment_falls_back_to_global_id() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "#global", 150);
		assert!(result.found);
		assert_eq!(result.offset, 180);
	}

	#[test]
	fn resolve_link_file_path_uses_section_start_when_no_fragment() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml", 0);
		assert!(result.found);
		assert_eq!(result.offset, 100);
	}

	#[test]
	fn resolve_link_file_path_uses_fragment_within_section_bounds() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml#target", 0);
		assert!(result.found);
		assert_eq!(result.offset, 120);
	}

	#[test]
	fn resolve_link_file_path_ignores_fragment_outside_section_bounds() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml#intro", 0);
		assert!(result.found);
		assert_eq!(result.offset, 100);
	}

	#[test]
	fn resolve_link_returns_not_found_for_unknown_targets() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "missing.xhtml#none", 0);
		assert!(!result.found);
	}
}
