//! [`reader_navigate`] (the marker-based Previous/Next navigation dispatch) and
//! [`reader_container_navigate`] (move relative to the enclosing list/table).

use crate::{
	document::{DocumentHandle, MarkerType},
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

#[must_use]
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
				// Section markers only carry a synthetic "Section N" label, not the chapter's real
				// title, so leave the text empty here; the UI falls back to the text of the line at
				// `offset`, which is the section's actual heading. Page markers carry a meaningful
				// label (the page number/name) that should be announced as-is.
				let text = if req.target == NavTarget::Page {
					doc.document().buffer.markers.get(idx).map(|m| m.text.clone()).unwrap_or_default()
				} else {
					String::new()
				};
				return build_nav_result(true, wrapped, offset, 0, text);
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
		NavTarget::List
		| NavTarget::ListItem
		| NavTarget::Link
		| NavTarget::Table
		| NavTarget::Separator
		| NavTarget::Image
		| NavTarget::Figure => {
			let kind = match req.target {
				NavTarget::List => MarkerType::List,
				NavTarget::ListItem => MarkerType::ListItem,
				NavTarget::Link => MarkerType::Link,
				NavTarget::Table => MarkerType::Table,
				NavTarget::Separator => MarkerType::Separator,
				NavTarget::Image => MarkerType::Image,
				NavTarget::Figure => MarkerType::Figure,
				_ => unreachable!(
					"NavTarget should only be List, ListItem, Link, Table, Separator, Image, or Figure in this branch"
				),
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

/// Move relative to the container (list/table) the caret is currently inside.
///
/// `to_end` moves to the start of the line that follows the container; otherwise moves to its
/// start. `found` is false when the caret is not inside any container. `marker_level` carries the
/// container's `MarkerType` as `i32` so the UI can describe what was left.
#[must_use]
pub fn reader_container_navigate(doc: &DocumentHandle, position: i64, to_end: bool) -> ffi::NavResult {
	let pos = usize::try_from(position.max(0)).unwrap_or(0);
	doc.enclosing_container(pos).map_or_else(
		|| build_nav_result(false, false, 0, 0, String::new()),
		|span| {
			let offset = if to_end { line_after(doc, span.end) } else { span.start };
			build_nav_result(true, false, offset, i32::from(span.mtype), String::new())
		},
	)
}

/// The start of the line following the one that ends at-or-after `end`. Container lengths vary in
/// whether they include the terminating newline (lists do, tables do not), so we normalise to the
/// next line: find the newline that terminates the container's final line and step past it. When
/// the container is the last line of the document, the clamped document end is returned.
fn line_after(doc: &DocumentHandle, end: usize) -> usize {
	let buffer = &doc.document().buffer;
	let total = buffer.char_count();
	let probe = end.min(total).saturating_sub(1);
	buffer.newline_positions().iter().find(|&&nl| nl >= probe).map_or(total, |&nl| (nl + 1).min(total))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker};

	#[test]
	fn reader_container_navigate_to_end_lands_on_following_line() {
		// Lines: "AAAA"(0-3) \n(4) "LLLL"(5-8) \n(9) "MMMM"(10-13) \n(14) "BBBB"(15-18).
		let content = "AAAA\nLLLL\nMMMM\nBBBB".to_string();
		// List length 10 includes the trailing newline (end == 15, the next line start).
		let mut list_buf = DocumentBuffer::with_content(content.clone());
		list_buf.add_marker(Marker::new(MarkerType::List, 5).with_length(10));
		let mut list_doc = Document::new();
		list_doc.set_buffer(list_buf);
		let list_handle = DocumentHandle::new(list_doc);
		let list_end = reader_container_navigate(&list_handle, 7, true);
		assert!(list_end.found);
		assert_eq!(list_end.offset, 15); // start of "BBBB"
		assert_eq!(list_end.marker_level, i32::from(MarkerType::List));
		assert_eq!(reader_container_navigate(&list_handle, 7, false).offset, 5);
		// Table length 9 stops at the last visible char (end == 14, the terminating newline).
		let mut table_buf = DocumentBuffer::with_content(content);
		table_buf.add_marker(Marker::new(MarkerType::Table, 5).with_length(9));
		let mut table_doc = Document::new();
		table_doc.set_buffer(table_buf);
		let table_handle = DocumentHandle::new(table_doc);
		let table_end = reader_container_navigate(&table_handle, 7, true);
		assert!(table_end.found);
		assert_eq!(table_end.offset, 15); // also lands on "BBBB", the line after the table
	}

	#[test]
	fn reader_container_navigate_not_in_container() {
		let mut buffer = DocumentBuffer::with_content("x".repeat(120));
		buffer.add_marker(Marker::new(MarkerType::List, 10).with_length(30));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		let handle = DocumentHandle::new(doc);
		assert!(!reader_container_navigate(&handle, 5, true).found);
	}
}
