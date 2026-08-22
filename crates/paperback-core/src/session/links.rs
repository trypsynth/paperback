//! Reference lists and jump targets built from document markers: the link list and
//! heading tree feeding the "Elements"/"Table of Contents" dialogs, link activation for
//! the reader's Enter-key handling, and bookmark note lookup for the status bar.

use super::{
	DocumentSession, HeadingTreeFfi, HeadingTreeItemFfi, LinkAction, LinkActivationResult, LinkListFfi, LinkListItemFfi,
};
use crate::{
	config::ConfigManager,
	document::{self, MarkerType},
	reader_core::resolve_link,
	types::{self as ffi},
};

impl DocumentSession {
	#[must_use]
	pub fn bookmark_display_at_position(
		&self,
		config: &ConfigManager,
		position: i64,
	) -> ffi::BookmarkDisplayAtPosition {
		let bookmark = config.get_bookmarks(&self.file_path).into_iter().find(|bm| bm.start == position);
		let Some(bookmark) = bookmark else {
			return ffi::BookmarkDisplayAtPosition { found: false, note: String::new(), snippet: String::new() };
		};
		let snippet = if bookmark.start == bookmark.end {
			self.get_line_text(bookmark.start)
		} else {
			self.get_text_range(bookmark.start, bookmark.end)
		};
		ffi::BookmarkDisplayAtPosition { found: true, note: bookmark.note, snippet }
	}

	#[must_use]
	pub fn link_list(&self, position: i64) -> ffi::LinkList {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let mut closest_index = -1;
		let mut items = Vec::new();
		for marker in self.handle.document().buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Link) {
			let text = if marker.text.is_empty() {
				self.get_line_text(i64::try_from(marker.position).unwrap_or(0))
			} else {
				marker.text.clone()
			};
			if marker.position <= pos {
				closest_index = i32::try_from(items.len()).unwrap_or(-1);
			}
			items.push(ffi::LinkListItem { offset: marker.position, text });
		}
		ffi::LinkList { items, closest_index }
	}

	#[must_use]
	pub fn heading_tree(&self, position: i64) -> ffi::HeadingTree {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let mut items = Vec::new();
		let mut closest_index = -1;
		let mut min_distance = usize::MAX;
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
			let text = if marker.text.is_empty() {
				self.get_line_text(i64::try_from(marker.position).unwrap_or(0))
			} else {
				marker.text.clone()
			};
			items.push(ffi::HeadingTreeItem { offset: marker.position, text, parent_index });
			item_stack.push((level, current_index));
			if marker.position <= pos {
				let dist = pos - marker.position;
				if dist < min_distance {
					min_distance = dist;
					closest_index = current_index;
				}
			}
		}
		ffi::HeadingTree { items, closest_index }
	}

	#[must_use]
	pub fn get_heading_tree_ffi(&self, position: i64) -> HeadingTreeFfi {
		let tree = self.heading_tree(position);
		HeadingTreeFfi {
			items: tree
				.items
				.into_iter()
				.map(|i| HeadingTreeItemFfi {
					offset: i64::try_from(i.offset).unwrap_or(i64::MAX),
					text: i.text,
					parent_index: i.parent_index,
				})
				.collect(),
			closest_index: tree.closest_index,
		}
	}

	#[must_use]
	pub fn get_link_list_ffi(&self, position: i64) -> LinkListFfi {
		let list = self.link_list(position);
		LinkListFfi {
			items: list
				.items
				.into_iter()
				.map(|i| LinkListItemFfi { offset: i64::try_from(i.offset).unwrap_or(i64::MAX), text: i.text })
				.collect(),
			closest_index: list.closest_index,
		}
	}

	#[must_use]
	pub fn activate_link(&self, position: i64) -> LinkActivationResult {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let href = {
			let link_index = self.handle.current_marker_index(pos_usize, MarkerType::Link);
			let Some(link_index) = link_index else {
				return LinkActivationResult::not_found();
			};
			let Some(marker) = self.handle.document().buffer.markers.get(link_index) else {
				return LinkActivationResult::not_found();
			};
			let link_end = marker.position + marker.text.chars().count();
			if pos_usize < marker.position || pos_usize > link_end {
				return LinkActivationResult::not_found();
			}
			if marker.reference.is_empty() {
				return LinkActivationResult::not_found();
			}
			// Clone the href so we can drop the borrow on self.handle.
			marker.reference.clone()
		};
		let resolution = resolve_link(&self.handle, &href, position);
		if !resolution.found {
			LinkActivationResult::not_found()
		} else if resolution.is_external {
			LinkActivationResult { found: true, action: LinkAction::External, offset: 0, url: resolution.url }
		} else {
			LinkActivationResult {
				found: true,
				action: LinkAction::Internal,
				offset: i64::try_from(resolution.offset).unwrap_or(0),
				url: String::new(),
			}
		}
	}

	#[must_use]
	pub fn activate_link_ffi(&self, position: i64) -> LinkActivationResult {
		self.activate_link(position)
	}
}
