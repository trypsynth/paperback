//! Markers are sorted by position at construction, and every lookup here relies on that order.

use super::{
	Document,
	buffer::{Edit, ReplaceOutcome},
	marker::{ContainerSpan, Marker, MarkerType, is_container_marker, is_heading_marker},
	position::DocPos,
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

	pub fn replace_range(&mut self, range: std::ops::Range<DocPos>, replacement: &str) -> i64 {
		self.replace_ranges(vec![Edit { start: range.start, end: range.end, text: replacement.to_string() }])
			.total_delta
	}

	/// Markers are re-sorted afterwards to keep the order [`Self::new`] establishes.
	pub fn replace_ranges(&mut self, edits: Vec<Edit>) -> ReplaceOutcome {
		let outcome = self.doc.buffer.replace_ranges(edits);
		if outcome.is_empty() {
			return outcome;
		}
		for position in self.doc.id_positions.values_mut() {
			*position = outcome.shift(*position);
		}
		self.doc.buffer.markers.sort_by_key(|m| m.position);
		self.doc.stats.take();
		outcome
	}

	fn markers_by_type(&self, marker_type: MarkerType) -> impl DoubleEndedIterator<Item = (usize, &Marker)> {
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
	pub fn next_marker_index(&self, position: DocPos, marker_type: MarkerType) -> Option<usize> {
		self.markers_by_type(marker_type).find(|(_, marker)| marker.position > position).map(|(idx, _)| idx)
	}

	#[must_use]
	pub fn previous_marker_index(&self, position: DocPos, marker_type: MarkerType) -> Option<usize> {
		self.markers_by_type(marker_type)
			.filter(|(_, marker)| marker.position < position)
			.map(|(idx, _)| idx)
			.next_back()
	}

	#[must_use]
	pub fn first_marker_index(&self, marker_type: MarkerType) -> Option<usize> {
		self.markers_by_type(marker_type).map(|(idx, _)| idx).next()
	}

	#[must_use]
	pub fn last_marker_index(&self, marker_type: MarkerType) -> Option<usize> {
		self.markers_by_type(marker_type).map(|(idx, _)| idx).next_back()
	}

	#[must_use]
	pub fn current_marker_index(&self, position: DocPos, marker_type: MarkerType) -> Option<usize> {
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

	/// A caret exactly at a container's end is past it.
	#[must_use]
	pub fn enclosing_container(&self, position: DocPos) -> Option<ContainerSpan> {
		self.doc
			.buffer
			.markers
			.iter()
			.filter(|m| is_container_marker(m.mtype) && !m.length.is_zero())
			.map(|m| ContainerSpan { start: m.position, end: m.position + m.length, mtype: m.mtype })
			.filter(|span| span.start <= position && position < span.end)
			.min_by(|a, b| b.start.cmp(&a.start).then_with(|| a.end.cmp(&b.end)))
	}

	#[must_use]
	pub fn next_heading_marker_index(&self, position: DocPos, level: Option<i32>) -> Option<usize> {
		self.heading_markers(level).into_iter().find(|(_, m)| m.position > position).map(|(idx, _)| idx)
	}

	#[must_use]
	pub fn previous_heading_marker_index(&self, position: DocPos, level: Option<i32>) -> Option<usize> {
		self.heading_markers(level).into_iter().filter(|(_, m)| m.position < position).map(|(idx, _)| idx).next_back()
	}

	#[must_use]
	pub fn marker_position(&self, marker_index: i32) -> Option<DocPos> {
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
	pub fn find_closest_toc_offset(&self, position: DocPos) -> DocPos {
		fn search(items: &[TocItem], position: DocPos, best_offset: &mut DocPos, best_distance: &mut usize) {
			for item in items {
				if item.offset <= position {
					let distance = (position - item.offset).get();
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
		let mut best_offset = DocPos::ZERO;
		let mut best_distance = usize::MAX;
		search(&self.doc.toc_items, position, &mut best_offset, &mut best_distance);
		best_offset
	}

	#[must_use]
	pub fn count_markers_by_type(&self, marker_type: MarkerType) -> usize {
		self.doc.buffer.markers.iter().filter(|m| m.mtype == marker_type).count()
	}

	#[must_use]
	pub fn get_marker_position_by_index(&self, marker_type: MarkerType, index: i32) -> Option<DocPos> {
		let target = usize::try_from(index).ok()?;
		self.markers_by_type(marker_type).nth(target).map(|(_, marker)| marker.position)
	}

	#[must_use]
	pub fn section_index(&self, position: DocPos) -> Option<i32> {
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
	pub fn page_index(&self, position: DocPos) -> Option<i32> {
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
	pub fn next_heading_index(&self, position: DocPos, level: Option<i32>) -> Option<i32> {
		self.next_heading_marker_index(position, level).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn previous_heading_index(&self, position: DocPos, level: Option<i32>) -> Option<i32> {
		self.previous_heading_marker_index(position, level).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn first_heading_index(&self, level: Option<i32>) -> Option<i32> {
		self.heading_markers(level).first().and_then(|&(idx, _)| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn last_heading_index(&self, level: Option<i32>) -> Option<i32> {
		self.heading_markers(level).last().and_then(|&(idx, _)| i32::try_from(idx).ok())
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
		buffer.add_marker(Marker::new(MarkerType::Link, 40.into()));
		buffer.add_marker(Marker::new(MarkerType::Heading2, 30.into()).with_level(2).with_text("H2".to_string()));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 20.into()));
		buffer.add_marker(Marker::new(MarkerType::Heading1, 10.into()).with_level(1).with_text("H1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 60.into()));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 5.into()));
		let mut parent = TocItem::new("Part 1".to_string(), "p1".to_string(), 10.into());
		parent.children.push(TocItem::new("Chapter 1".to_string(), "c1".to_string(), 26.into()));
		let mut doc = Document::new().with_title("Sample".to_string()).with_author("Author".to_string());
		doc.set_buffer(buffer);
		doc.toc_items = vec![parent, TocItem::new("Part 2".to_string(), "p2".to_string(), 50.into())];
		DocumentHandle::new(doc)
	}

	fn container_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::new();
		buffer.append(&"x".repeat(200));
		buffer.add_marker(Marker::new(MarkerType::List, 10.into()).with_level(3).with_length(30.into()));
		buffer.add_marker(Marker::new(MarkerType::Table, 80.into()).with_length(40.into()));
		buffer.add_marker(Marker::new(MarkerType::List, 45.into()).with_level(2).with_length(55.into()));
		buffer.add_marker(Marker::new(MarkerType::List, 50.into()).with_level(2).with_length(20.into()));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		DocumentHandle::new(doc)
	}

	#[test]
	fn enclosing_container_finds_list_and_table_spans() {
		let handle = container_handle();
		let list = handle.enclosing_container(25.into()).unwrap();
		assert_eq!((list.start, list.end, list.mtype), (10.into(), 40.into(), MarkerType::List));
		let table = handle.enclosing_container(80.into()).unwrap();
		assert_eq!((table.start, table.end, table.mtype), (80.into(), 120.into(), MarkerType::Table));
	}

	#[test]
	fn enclosing_container_start_inclusive_end_exclusive() {
		let handle = container_handle();
		assert_eq!(handle.enclosing_container(10.into()).unwrap().start, 10);
		assert!(handle.enclosing_container(40.into()).is_none());
	}

	#[test]
	fn enclosing_container_returns_innermost_when_nested() {
		let handle = container_handle();
		let inner = handle.enclosing_container(55.into()).unwrap();
		assert_eq!((inner.start, inner.end), (50.into(), 70.into()));
		assert_eq!(handle.enclosing_container(75.into()).unwrap().start, 45);
	}

	#[test]
	fn enclosing_container_none_outside_any_container() {
		let handle = container_handle();
		assert!(handle.enclosing_container(5.into()).is_none());
		assert!(handle.enclosing_container(150.into()).is_none());
	}

	#[test]
	fn document_handle_sorts_markers_on_creation() {
		let handle = sample_handle();
		let positions: Vec<DocPos> = handle.document().buffer.markers.iter().map(|m| m.position).collect();
		assert_eq!(positions, vec![5, 10, 20, 30, 40, 60]);
	}

	#[test]
	fn marker_index_navigation_works_for_next_previous_and_current() {
		let handle = sample_handle();
		assert_eq!(handle.next_marker_index(5.into(), MarkerType::Heading2), Some(3));
		assert_eq!(handle.previous_marker_index(25.into(), MarkerType::Heading1), Some(1));
		assert_eq!(handle.current_marker_index(25.into(), MarkerType::PageBreak), Some(2));
		assert_eq!(handle.current_marker_index(15.into(), MarkerType::PageBreak), None);
	}

	#[test]
	fn heading_navigation_respects_level_filter() {
		let handle = sample_handle();
		assert_eq!(handle.next_heading_marker_index(0.into(), None), Some(1));
		assert_eq!(handle.next_heading_marker_index(0.into(), Some(2)), Some(3));
		assert_eq!(handle.next_heading_marker_index(10.into(), Some(1)), None);
		assert_eq!(handle.previous_heading_marker_index(35.into(), None), Some(3));
		assert_eq!(handle.previous_heading_marker_index(35.into(), Some(1)), Some(1));
	}

	#[test]
	fn marker_position_and_marker_type_lookup_work() {
		let handle = sample_handle();
		assert_eq!(handle.marker_position(2), Some(20.into()));
		assert_eq!(handle.marker_position(-1), None);
		assert_eq!(handle.get_marker_position_by_index(MarkerType::SectionBreak, 0), Some(5.into()));
		assert_eq!(handle.get_marker_position_by_index(MarkerType::SectionBreak, 1), Some(60.into()));
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
		assert_eq!(handle.find_closest_toc_offset(9.into()), 0);
		assert_eq!(handle.find_closest_toc_offset(27.into()), 26);
		assert_eq!(handle.find_closest_toc_offset(49.into()), 26);
		assert_eq!(handle.find_closest_toc_offset(52.into()), 50);
	}

	#[test]
	fn index_helpers_return_expected_indices() {
		let handle = sample_handle();
		assert_eq!(handle.section_index(61.into()), Some(1));
		assert_eq!(handle.page_index(25.into()), Some(0));
		assert_eq!(handle.next_heading_index(0.into(), None), Some(1));
		assert_eq!(handle.previous_heading_index(100.into(), None), Some(3));
	}

	#[test]
	fn find_closest_toc_offset_returns_zero_when_no_toc_items() {
		let doc = Document::new();
		let handle = DocumentHandle::new(doc);
		assert_eq!(handle.find_closest_toc_offset(100.into()), 0);
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
		assert_eq!(handle.section_index(0.into()), None);
		assert_eq!(handle.page_index(0.into()), None);
	}

	#[test]
	fn heading_index_helpers_return_none_when_filtered_level_missing() {
		let handle = sample_handle();
		assert_eq!(handle.next_heading_index(0.into(), Some(6)), None);
		assert_eq!(handle.previous_heading_index(100.into(), Some(6)), None);
	}

	#[test]
	fn recognized_text_is_in_the_word_count() {
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content(
			"one

two"
			.to_string(),
		));
		let mut handle = DocumentHandle::new(doc);
		assert_eq!(handle.document().stats().word_count, 2);
		handle.replace_ranges(vec![Edit { start: 4.into(), end: 4.into(), text: "three four".to_string() }]);
		assert_eq!(handle.document().stats().word_count, 4);
	}
}
