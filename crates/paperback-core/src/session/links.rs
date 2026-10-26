//! Reference lists and jump targets built from document markers: the link list and
//! heading tree feeding the "Elements"/"Table of Contents" dialogs, link activation for
//! the reader's Enter-key handling, and bookmark note lookup for the status bar.

use super::{
	DocumentSession, HeadingTreeFfi, HeadingTreeItemFfi, LinkAction, LinkActivationResult, LinkListFfi, LinkListItemFfi,
};
use crate::{
	config::ConfigManager,
	document::{self, DocLen, DocPos, MarkerType},
	export::annotations::AnnotationEntry,
	reader_core::{get_filtered_bookmarks, resolve_link},
	types::{self as ffi},
};

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	#[must_use]
	pub fn get_heading_tree_ffi(&self, position: DocPos) -> HeadingTreeFfi {
		let tree = self.heading_tree(position);
		HeadingTreeFfi {
			items: tree
				.items
				.into_iter()
				.map(|i| HeadingTreeItemFfi { offset: i.offset, text: i.text, parent_index: i.parent_index })
				.collect(),
			closest_index: tree.closest_index,
		}
	}

	#[must_use]
	pub fn get_link_list_ffi(&self, position: DocPos) -> LinkListFfi {
		let list = self.link_list(position);
		LinkListFfi {
			items: list.items.into_iter().map(|i| LinkListItemFfi { offset: i.offset, text: i.text }).collect(),
			closest_index: list.closest_index,
		}
	}

	/// Whether `position` is on a link that activating would follow.
	#[must_use]
	pub fn is_on_link(&self, position: DocPos) -> bool {
		self.link_href_at(position).is_some()
	}

	/// The href of the link `position` is on, if any.
	fn link_href_at(&self, position: DocPos) -> Option<String> {
		let pos = position.min(self.handle.document().buffer.end());
		let link_index = self.handle.current_marker_index(pos, MarkerType::Link)?;
		let marker = self.handle.document().buffer.markers.get(link_index)?;
		let link_end = marker.position + DocLen::of(&marker.text);
		if pos < marker.position || pos > link_end || marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn activate_link(&self, position: DocPos) -> LinkActivationResult {
		let Some(href) = self.link_href_at(position) else {
			return LinkActivationResult::not_found();
		};
		let resolution = resolve_link(&self.handle, &href, position);
		if !resolution.found {
			LinkActivationResult::not_found()
		} else if resolution.is_external {
			LinkActivationResult {
				found: true,
				action: LinkAction::External,
				offset: DocPos::ZERO,
				url: resolution.url,
			}
		} else {
			LinkActivationResult {
				found: true,
				action: LinkAction::Internal,
				offset: resolution.offset,
				url: String::new(),
			}
		}
	}
}

impl DocumentSession {
	/// The bookmarks kept under `path`, each with the text it marks, for an export that has to be readable without the book.
	#[must_use]
	pub fn annotation_entries(
		&self,
		config: &ConfigManager,
		path: &str,
		filter: ffi::BookmarkFilterType,
	) -> Vec<AnnotationEntry> {
		get_filtered_bookmarks(config, path, 0, filter)
			.items
			.into_iter()
			.map(|b| AnnotationEntry {
				passage: if b.is_whole_line {
					self.get_line_text(self.nearest_pos(b.start))
				} else {
					self.get_text_range_display(self.nearest_pos(b.start), self.nearest_pos(b.end))
				},
				note: b.note,
				audio_ms: b.audio_ms,
			})
			.collect()
	}

	/// The bookmark at `position` among those kept under `path`, which is where the document's
	/// settings are: the link for a document opened from one.
	#[must_use]
	pub fn bookmark_display_at_position(
		&self,
		config: &ConfigManager,
		path: &str,
		position: DocPos,
	) -> ffi::BookmarkDisplayAtPosition {
		let bookmark =
			config.get_bookmarks(path).into_iter().find(|bm| i64::try_from(position).is_ok_and(|p| bm.start == p));
		let Some(bookmark) = bookmark else {
			return ffi::BookmarkDisplayAtPosition { found: false, note: String::new(), snippet: String::new() };
		};
		let snippet = if bookmark.start == bookmark.end {
			self.line_text_at(self.nearest_pos(bookmark.start))
		} else {
			self.get_text_range_display(self.nearest_pos(bookmark.start), self.nearest_pos(bookmark.end))
		};
		ffi::BookmarkDisplayAtPosition { found: true, note: bookmark.note, snippet }
	}

	/// One row per marker of `mtype`, in document order, for the Elements dialog's flat views.
	/// Each row shows the marker's own text (a table's caption) or, when the marker carries none,
	/// the whole line the marker sits on (a list's first rendered item) — the same text reading
	/// navigation announces for it. `closest_index` is the index of the last row whose marker is
	/// at or before `position`.
	#[must_use]
	pub fn element_list(&self, mtype: MarkerType, position: DocPos) -> ffi::ElementList {
		let pos = position.min(self.handle.document().buffer.end());
		let mut closest_index = -1;
		let mut items = Vec::new();
		for marker in self.handle.document().buffer.markers.iter().filter(|marker| marker.mtype == mtype) {
			let text = if marker.text.is_empty() { self.get_line_text(marker.position) } else { marker.text.clone() };
			if marker.position <= pos {
				closest_index = i32::try_from(items.len()).unwrap_or(-1);
			}
			items.push(ffi::ElementListItem { offset: marker.position, text });
		}
		ffi::ElementList { items, closest_index }
	}

	#[must_use]
	pub fn link_list(&self, position: DocPos) -> ffi::LinkList {
		let list = self.element_list(MarkerType::Link, position);
		ffi::LinkList {
			items: list
				.items
				.iter()
				.map(|item| ffi::LinkListItem { offset: item.offset, text: item.text.clone() })
				.collect(),
			closest_index: list.closest_index,
		}
	}

	#[must_use]
	pub fn heading_tree(&self, position: DocPos) -> ffi::HeadingTree {
		let pos = position.min(self.handle.document().buffer.end());
		let mut items = Vec::new();
		let mut closest_index = -1;
		let mut min_distance: Option<DocLen> = None;
		let markers = &self.handle.document().buffer.markers;
		let mut item_stack: Vec<(i32, i32)> = Vec::new(); // (level, index)
		for marker in markers {
			if !document::is_heading_marker(marker.mtype) {
				continue;
			}
			let level = marker.level;
			while item_stack.last().is_some_and(|(l, _)| *l >= level) {
				item_stack.pop();
			}
			let parent_index = item_stack.last().map_or(-1, |(_, idx)| *idx);
			let current_index = i32::try_from(items.len()).unwrap_or(-1);
			let text = if marker.text.is_empty() { self.get_line_text(marker.position) } else { marker.text.clone() };
			items.push(ffi::HeadingTreeItem { offset: marker.position, text, parent_index });
			item_stack.push((level, current_index));
			if marker.position <= pos {
				let dist = pos - marker.position;
				if min_distance.is_none_or(|min| dist < min) {
					min_distance = Some(dist);
					closest_index = current_index;
				}
			}
		}
		ffi::HeadingTree { items, closest_index }
	}
}
