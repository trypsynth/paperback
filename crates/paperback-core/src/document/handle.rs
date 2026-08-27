//! [`DocumentHandle`]: read-only navigation over a parsed [`super::Document`] — finding the
//! next/previous/current marker of a given type, heading navigation with optional level
//! filtering, the innermost list/table containing a position, and TOC/section/page lookups.
//! Markers are sorted by position once at construction so every lookup below can rely on that
//! order instead of re-sorting per call.

use super::{
	Document,
	marker::{ContainerSpan, Marker, MarkerType, is_container_marker, is_heading_marker},
	toc::TocItem,
};
use crate::types::HeadingInfo;

#[derive(Debug, Clone)]
pub struct DocumentHandle {
	doc: Document,
}

impl DocumentHandle {
	#[must_use]
	pub fn new(mut doc: Document) -> Self {
		doc.buffer.markers.sort_by_key(|m| m.position);
		Self { doc }
	}

	#[must_use]
	pub const fn document(&self) -> &Document {
		&self.doc
	}

	fn markers_by_type(&self, marker_type: MarkerType) -> impl Iterator<Item = (usize, &Marker)> {
		self.doc.buffer.markers.iter().enumerate().filter(move |(_, m)| m.mtype == marker_type)
	}

	fn heading_markers(&self, level: Option<i32>) -> Vec<(usize, &Marker)> {
		let mut result: Vec<(usize, &Marker)> = self
			.doc
			.buffer
			.markers
			.iter()
			.enumerate()
			.filter(|(_, marker)| is_heading_marker(marker.mtype))
			.filter(|(_, marker)| level.is_none_or(|lvl| marker.level == lvl))
			.collect();
		result.sort_by_key(|(_, marker)| marker.position);
		result
	}

	#[must_use]
	pub fn next_marker_index(&self, position: i64, marker_type: MarkerType) -> Option<usize> {
		self.doc
			.buffer
			.markers
			.iter()
			.enumerate()
			.filter(|(_, marker)| {
				marker.mtype == marker_type && i64::try_from(marker.position).unwrap_or(i64::MAX) > position
			})
			.map(|(idx, _)| idx)
			.next()
	}

	#[must_use]
	pub fn previous_marker_index(&self, position: i64, marker_type: MarkerType) -> Option<usize> {
		self.doc
			.buffer
			.markers
			.iter()
			.enumerate()
			.filter(|(_, marker)| {
				marker.mtype == marker_type && i64::try_from(marker.position).unwrap_or(i64::MAX) < position
			})
			.map(|(idx, _)| idx)
			.next_back()
	}

	#[must_use]
	pub fn current_marker_index(&self, position: usize, marker_type: MarkerType) -> Option<usize> {
		let mut result = None;
		for (idx, marker) in self.doc.buffer.markers.iter().enumerate() {
			if marker.mtype == marker_type && marker.position <= position {
				result = Some(idx);
			} else if marker.position > position {
				break;
			}
		}
		result
	}

	/// The innermost container (list/table) whose span contains `position`, or `None` when the
	/// position is not inside any container. A container covers `[start, start + length)`; the
	/// start is inclusive and the end is exclusive (a caret exactly at the end is past it).
	/// When containers nest, the innermost is the candidate with the greatest start (ties broken
	/// by the smallest end).
	#[must_use]
	pub fn enclosing_container(&self, position: usize) -> Option<ContainerSpan> {
		self.doc
			.buffer
			.markers
			.iter()
			.filter(|m| is_container_marker(m.mtype) && m.length > 0)
			.map(|m| ContainerSpan { start: m.position, end: m.position + m.length, mtype: m.mtype })
			.filter(|span| span.start <= position && position < span.end)
			.min_by(|a, b| b.start.cmp(&a.start).then_with(|| a.end.cmp(&b.end)))
	}

	#[must_use]
	pub fn next_heading_marker_index(&self, position: i64, level: Option<i32>) -> Option<usize> {
		let heading_markers = self.heading_markers(level);
		heading_markers
			.into_iter()
			.find(|(_, m)| i64::try_from(m.position).unwrap_or(i64::MAX) > position)
			.map(|(idx, _)| idx)
	}

	#[must_use]
	pub fn previous_heading_marker_index(&self, position: i64, level: Option<i32>) -> Option<usize> {
		let heading_markers = self.heading_markers(level);
		heading_markers
			.into_iter()
			.filter(|(_, m)| i64::try_from(m.position).unwrap_or(i64::MAX) < position)
			.map(|(idx, _)| idx)
			.next_back()
	}

	#[must_use]
	pub fn marker_position(&self, marker_index: i32) -> Option<usize> {
		let idx = usize::try_from(marker_index).ok()?;
		self.doc.buffer.markers.get(idx).map(|m| m.position)
	}

	#[must_use]
	pub fn heading_info(&self, heading_index: i32) -> Option<HeadingInfo> {
		let idx = usize::try_from(heading_index).ok()?;
		let heading_markers = self.heading_markers(None);
		let (_, marker) = heading_markers.get(idx)?;
		Some(HeadingInfo { offset: marker.position, level: marker.level, text: marker.text.clone() })
	}

	#[must_use]
	pub fn find_closest_toc_offset(&self, position: usize) -> usize {
		fn search(items: &[TocItem], position: usize, best_offset: &mut usize, best_distance: &mut usize) {
			for item in items {
				if item.offset <= position {
					let distance = position - item.offset;
					if distance < *best_distance {
						*best_distance = distance;
						*best_offset = item.offset;
					}
				}
				if !item.children.is_empty() {
					search(&item.children, position, best_offset, best_distance);
				}
			}
		}
		let mut best_offset = 0usize;
		let mut best_distance = usize::MAX;
		search(&self.doc.toc_items, position, &mut best_offset, &mut best_distance);
		best_offset
	}

	#[must_use]
	pub fn count_markers_by_type(&self, marker_type: MarkerType) -> usize {
		self.doc.buffer.markers.iter().filter(|m| m.mtype == marker_type).count()
	}

	#[must_use]
	pub fn get_marker_position_by_index(&self, marker_type: MarkerType, index: i32) -> Option<usize> {
		let target = usize::try_from(index).ok()?;
		self.markers_by_type(marker_type).nth(target).map(|(_, marker)| marker.position)
	}

	#[must_use]
	pub fn section_index(&self, position: usize) -> Option<i32> {
		let count = self
			.doc
			.buffer
			.markers
			.iter()
			.filter(|m| m.mtype == MarkerType::SectionBreak && m.position <= position)
			.count();
		if count == 0 { None } else { i32::try_from(count - 1).ok() }
	}

	#[must_use]
	pub fn page_index(&self, position: usize) -> Option<i32> {
		let count = self
			.doc
			.buffer
			.markers
			.iter()
			.filter(|m| m.mtype == MarkerType::PageBreak && m.position <= position)
			.count();
		if count == 0 { None } else { i32::try_from(count - 1).ok() }
	}

	#[must_use]
	pub fn next_heading_index(&self, position: i64, level: Option<i32>) -> Option<i32> {
		self.next_heading_marker_index(position, level).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn previous_heading_index(&self, position: i64, level: Option<i32>) -> Option<i32> {
		self.previous_heading_marker_index(position, level).and_then(|idx| i32::try_from(idx).ok())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::buffer::DocumentBuffer;

	fn sample_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::new();
		let text = "x".repeat(120);
		buffer.append(&text);
		buffer.add_marker(Marker::new(MarkerType::Link, 40));
		buffer.add_marker(Marker::new(MarkerType::Heading2, 30).with_level(2).with_text("H2".to_string()));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 20));
		buffer.add_marker(Marker::new(MarkerType::Heading1, 10).with_level(1).with_text("H1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 60));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 5));
		let mut parent = TocItem::new("Part 1".to_string(), "p1".to_string(), 10);
		parent.children.push(TocItem::new("Chapter 1".to_string(), "c1".to_string(), 26));
		let mut doc = Document::new().with_title("Sample".to_string()).with_author("Author".to_string());
		doc.set_buffer(buffer);
		doc.toc_items = vec![parent, TocItem::new("Part 2".to_string(), "p2".to_string(), 50)];
		DocumentHandle::new(doc)
	}

	fn container_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::new();
		buffer.append(&"x".repeat(200));
		// A list spanning [10, 40) and a table spanning [80, 120).
		buffer.add_marker(Marker::new(MarkerType::List, 10).with_level(3).with_length(30));
		buffer.add_marker(Marker::new(MarkerType::Table, 80).with_length(40));
		// A nested list [50, 70) inside an outer list [45, 100).
		buffer.add_marker(Marker::new(MarkerType::List, 45).with_level(2).with_length(55));
		buffer.add_marker(Marker::new(MarkerType::List, 50).with_level(2).with_length(20));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		DocumentHandle::new(doc)
	}

	#[test]
	fn enclosing_container_finds_list_and_table_spans() {
		let handle = container_handle();
		let list = handle.enclosing_container(25).unwrap();
		assert_eq!((list.start, list.end, list.mtype), (10, 40, MarkerType::List));
		let table = handle.enclosing_container(80).unwrap();
		assert_eq!((table.start, table.end, table.mtype), (80, 120, MarkerType::Table));
	}

	#[test]
	fn enclosing_container_start_inclusive_end_exclusive() {
		let handle = container_handle();
		assert_eq!(handle.enclosing_container(10).unwrap().start, 10); // start is inside
		assert!(handle.enclosing_container(40).is_none()); // end is past the list
	}

	#[test]
	fn enclosing_container_returns_innermost_when_nested() {
		let handle = container_handle();
		// Position 55 is inside both the outer list [45,100) and the nested list [50,70).
		let inner = handle.enclosing_container(55).unwrap();
		assert_eq!((inner.start, inner.end), (50, 70));
		// Position 75 is only inside the outer list.
		assert_eq!(handle.enclosing_container(75).unwrap().start, 45);
	}

	#[test]
	fn enclosing_container_none_outside_any_container() {
		let handle = container_handle();
		assert!(handle.enclosing_container(5).is_none());
		assert!(handle.enclosing_container(150).is_none());
	}

	#[test]
	fn document_handle_sorts_markers_on_creation() {
		let handle = sample_handle();
		let positions: Vec<usize> = handle.document().buffer.markers.iter().map(|m| m.position).collect();
		assert_eq!(positions, vec![5, 10, 20, 30, 40, 60]);
	}

	#[test]
	fn marker_index_navigation_works_for_next_previous_and_current() {
		let handle = sample_handle();
		assert_eq!(handle.next_marker_index(5, MarkerType::Heading2), Some(3));
		assert_eq!(handle.previous_marker_index(25, MarkerType::Heading1), Some(1));
		assert_eq!(handle.current_marker_index(25, MarkerType::PageBreak), Some(2));
		assert_eq!(handle.current_marker_index(15, MarkerType::PageBreak), None);
	}

	#[test]
	fn heading_navigation_respects_level_filter() {
		let handle = sample_handle();
		assert_eq!(handle.next_heading_marker_index(0, None), Some(1));
		assert_eq!(handle.next_heading_marker_index(0, Some(2)), Some(3));
		assert_eq!(handle.next_heading_marker_index(10, Some(1)), None);
		assert_eq!(handle.previous_heading_marker_index(35, None), Some(3));
		assert_eq!(handle.previous_heading_marker_index(35, Some(1)), Some(1));
	}

	#[test]
	fn marker_position_and_marker_type_lookup_work() {
		let handle = sample_handle();
		assert_eq!(handle.marker_position(2), Some(20));
		assert_eq!(handle.marker_position(-1), None);
		assert_eq!(handle.get_marker_position_by_index(MarkerType::SectionBreak, 0), Some(5));
		assert_eq!(handle.get_marker_position_by_index(MarkerType::SectionBreak, 1), Some(60));
		assert_eq!(handle.get_marker_position_by_index(MarkerType::SectionBreak, 2), None);
	}

	#[test]
	fn heading_info_returns_sorted_heading_entries() {
		let handle = sample_handle();
		let first = handle.heading_info(0).unwrap();
		assert_eq!(first.offset, 10);
		assert_eq!(first.level, 1);
		assert_eq!(first.text, "H1");
		let second = handle.heading_info(1).unwrap();
		assert_eq!(second.offset, 30);
		assert_eq!(second.level, 2);
		assert_eq!(second.text, "H2");
		assert!(handle.heading_info(2).is_none());
		assert!(handle.heading_info(-1).is_none());
	}

	#[test]
	fn find_closest_toc_offset_uses_nested_items() {
		let handle = sample_handle();
		assert_eq!(handle.find_closest_toc_offset(9), 0);
		assert_eq!(handle.find_closest_toc_offset(27), 26);
		assert_eq!(handle.find_closest_toc_offset(49), 26);
		assert_eq!(handle.find_closest_toc_offset(52), 50);
	}

	#[test]
	fn index_helpers_return_expected_indices() {
		let handle = sample_handle();
		assert_eq!(handle.section_index(61), Some(1));
		assert_eq!(handle.page_index(25), Some(0));
		assert_eq!(handle.next_heading_index(0, None), Some(1));
		assert_eq!(handle.previous_heading_index(100, None), Some(3));
	}

	#[test]
	fn find_closest_toc_offset_returns_zero_when_no_toc_items() {
		let doc = Document::new();
		let handle = DocumentHandle::new(doc);
		assert_eq!(handle.find_closest_toc_offset(100), 0);
	}

	#[test]
	fn count_markers_by_type_counts_only_matching_markers() {
		let handle = sample_handle();
		assert_eq!(handle.count_markers_by_type(MarkerType::SectionBreak), 2);
		assert_eq!(handle.count_markers_by_type(MarkerType::Link), 1);
		assert_eq!(handle.count_markers_by_type(MarkerType::Table), 0);
	}

	#[test]
	fn section_and_page_index_are_none_before_first_marker() {
		let handle = sample_handle();
		assert_eq!(handle.section_index(0), None);
		assert_eq!(handle.page_index(0), None);
	}

	#[test]
	fn heading_index_helpers_return_none_when_filtered_level_missing() {
		let handle = sample_handle();
		assert_eq!(handle.next_heading_index(0, Some(6)), None);
		assert_eq!(handle.previous_heading_index(100, Some(6)), None);
	}
}
