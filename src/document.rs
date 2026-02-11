use std::collections::HashMap;

use bitflags::bitflags;

use crate::text::{display_len, is_space_like};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MarkerType {
	Heading1 = 0,
	Heading2 = 1,
	Heading3 = 2,
	Heading4 = 3,
	Heading5 = 4,
	Heading6 = 5,
	PageBreak = 6,
	SectionBreak = 7,
	TocItem = 8,
	Link = 9,
	List = 10,
	ListItem = 11,
	Table = 12,
	Separator = 13,
}

impl From<MarkerType> for i32 {
	fn from(marker: MarkerType) -> Self {
		marker as Self
	}
}

impl TryFrom<i32> for MarkerType {
	type Error = ();

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Heading1),
			1 => Ok(Self::Heading2),
			2 => Ok(Self::Heading3),
			3 => Ok(Self::Heading4),
			4 => Ok(Self::Heading5),
			5 => Ok(Self::Heading6),
			6 => Ok(Self::PageBreak),
			7 => Ok(Self::SectionBreak),
			8 => Ok(Self::TocItem),
			9 => Ok(Self::Link),
			10 => Ok(Self::List),
			11 => Ok(Self::ListItem),
			12 => Ok(Self::Table),
			13 => Ok(Self::Separator),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Marker {
	pub mtype: MarkerType,
	pub position: usize,
	pub text: String,
	pub reference: String,
	pub level: i32,
	pub length: usize,
}

impl Marker {
	#[must_use]
	pub const fn new(mtype: MarkerType, position: usize) -> Self {
		Self { mtype, position, text: String::new(), reference: String::new(), level: 0, length: 0 }
	}

	#[must_use]
	pub fn with_text(mut self, text: String) -> Self {
		self.text = text;
		self
	}

	#[must_use]
	pub fn with_reference(mut self, reference: String) -> Self {
		self.reference = reference;
		self
	}

	#[must_use]
	pub const fn with_level(mut self, level: i32) -> Self {
		self.level = level;
		self
	}

	#[must_use]
	pub const fn with_length(mut self, length: usize) -> Self {
		self.length = length;
		self
	}
}

#[derive(Debug, Clone)]
pub struct DocumentBuffer {
	pub content: String,
	pub markers: Vec<Marker>,
	content_display_len: usize,
}

impl DocumentBuffer {
	#[must_use]
	pub const fn new() -> Self {
		Self { content: String::new(), markers: Vec::new(), content_display_len: 0 }
	}

	#[must_use]
	pub fn with_content(content: String) -> Self {
		let len = display_len(&content);
		Self { content, markers: Vec::new(), content_display_len: len }
	}

	pub fn add_marker(&mut self, marker: Marker) {
		self.markers.push(marker);
	}

	pub fn append(&mut self, text: &str) {
		self.content.push_str(text);
		self.content_display_len += display_len(text);
	}

	#[must_use]
	pub const fn current_position(&self) -> usize {
		self.content_display_len
	}
}

impl Default for DocumentBuffer {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone)]
pub struct TocItem {
	pub name: String,
	pub reference: String,
	pub offset: usize,
	pub children: Vec<Self>,
}

impl TocItem {
	#[must_use]
	pub const fn new(name: String, reference: String, offset: usize) -> Self {
		Self { name, reference, offset, children: Vec::new() }
	}
}

#[derive(Debug, Clone, Default)]
pub struct DocumentStats {
	pub word_count: usize,
	pub line_count: usize,
	pub char_count: usize,
	pub char_count_no_whitespace: usize,
}

impl DocumentStats {
	#[must_use]
	pub fn from_text(text: &str) -> Self {
		let char_count = text.chars().count();
		let line_count = text.lines().count();
		let word_count = text.split_whitespace().count();
		let char_count_no_whitespace = text.chars().filter(|c| !is_space_like(*c)).count();
		Self { word_count, line_count, char_count, char_count_no_whitespace }
	}
}

#[derive(Debug, Clone)]
pub struct Document {
	pub title: String,
	pub author: String,
	pub buffer: DocumentBuffer,
	pub toc_items: Vec<TocItem>,
	pub id_positions: HashMap<String, usize>,
	pub spine_items: Vec<String>,
	pub manifest_items: HashMap<String, String>,
	pub stats: DocumentStats,
}

impl Document {
	#[must_use]
	pub fn new() -> Self {
		Self {
			title: String::new(),
			author: String::new(),
			buffer: DocumentBuffer::new(),
			toc_items: Vec::new(),
			id_positions: HashMap::new(),
			spine_items: Vec::new(),
			manifest_items: HashMap::new(),
			stats: DocumentStats::default(),
		}
	}

	#[must_use]
	pub fn with_title(mut self, title: String) -> Self {
		self.title = title;
		self
	}

	#[must_use]
	pub fn with_author(mut self, author: String) -> Self {
		self.author = author;
		self
	}

	pub fn set_buffer(&mut self, buffer: DocumentBuffer) {
		self.buffer = buffer;
	}

	pub fn compute_stats(&mut self) {
		self.stats = DocumentStats::from_text(&self.buffer.content);
	}
}

impl Default for Document {
	fn default() -> Self {
		Self::new()
	}
}

#[must_use]
pub const fn is_heading_marker(marker_type: MarkerType) -> bool {
	matches!(
		marker_type,
		MarkerType::Heading1
			| MarkerType::Heading2
			| MarkerType::Heading3
			| MarkerType::Heading4
			| MarkerType::Heading5
			| MarkerType::Heading6
	)
}

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
	pub fn heading_info(&self, heading_index: i32) -> Option<crate::types::HeadingInfo> {
		let idx = usize::try_from(heading_index).ok()?;
		let heading_markers = self.heading_markers(None);
		let (_, marker) = heading_markers.get(idx)?;
		Some(crate::types::HeadingInfo { offset: marker.position, level: marker.level, text: marker.text.clone() })
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
		self.current_marker_index(position, MarkerType::SectionBreak).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn page_index(&self, position: usize) -> Option<i32> {
		self.current_marker_index(position, MarkerType::PageBreak).and_then(|idx| i32::try_from(idx).ok())
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

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct ParserFlags: u32 {
		const NONE = 0;
		const SUPPORTS_SECTIONS = 1 << 0;
		const SUPPORTS_TOC = 1 << 1;
		const SUPPORTS_PAGES = 1 << 2;
		const SUPPORTS_LISTS = 1 << 3;
	}
}

#[derive(Debug, Clone)]
pub struct ParserContext {
	pub file_path: String,
	pub password: Option<String>,
	pub forced_extension: Option<String>,
}

impl ParserContext {
	#[must_use]
	pub const fn new(file_path: String) -> Self {
		Self { file_path, password: None, forced_extension: None }
	}

	#[must_use]
	pub fn with_password(mut self, password: String) -> Self {
		self.password = Some(password);
		self
	}

	#[must_use]
	pub fn with_forced_extension(mut self, extension: String) -> Self {
		self.forced_extension = Some(extension);
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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

	#[test]
	fn marker_type_round_trip_for_all_known_values() {
		for raw in 0..=13 {
			let marker = MarkerType::try_from(raw).unwrap();
			assert_eq!(i32::from(marker), raw);
		}
		assert!(MarkerType::try_from(14).is_err());
		assert!(MarkerType::try_from(-1).is_err());
	}

	#[test]
	fn marker_builder_helpers_set_all_fields() {
		let marker = Marker::new(MarkerType::Table, 42)
			.with_text("Title".to_string())
			.with_reference("ref".to_string())
			.with_level(3)
			.with_length(9);
		assert_eq!(marker.position, 42);
		assert_eq!(marker.text, "Title");
		assert_eq!(marker.reference, "ref");
		assert_eq!(marker.level, 3);
		assert_eq!(marker.length, 9);
	}

	#[test]
	fn document_buffer_append_updates_position() {
		let mut buffer = DocumentBuffer::new();
		assert_eq!(buffer.current_position(), 0);
		buffer.append("abc");
		buffer.append("de");
		assert_eq!(buffer.current_position(), 5);
	}

	#[test]
	fn document_stats_counts_words_lines_and_chars() {
		let stats = DocumentStats::from_text("a b\nc");
		assert_eq!(stats.word_count, 3);
		assert_eq!(stats.line_count, 2);
		assert_eq!(stats.char_count, 5);
		assert_eq!(stats.char_count_no_whitespace, 3);
	}

	#[test]
	fn document_compute_stats_uses_buffer_content() {
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("one two".to_string()));
		doc.compute_stats();
		assert_eq!(doc.stats.word_count, 2);
		assert_eq!(doc.stats.line_count, 1);
	}

	#[test]
	fn heading_marker_helper_matches_heading_types_only() {
		assert!(is_heading_marker(MarkerType::Heading1));
		assert!(is_heading_marker(MarkerType::Heading6));
		assert!(!is_heading_marker(MarkerType::Link));
		assert!(!is_heading_marker(MarkerType::SectionBreak));
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
		assert_eq!(handle.section_index(61), Some(5));
		assert_eq!(handle.page_index(25), Some(2));
		assert_eq!(handle.next_heading_index(0, None), Some(1));
		assert_eq!(handle.previous_heading_index(100, None), Some(3));
	}

	#[test]
	fn parser_context_builder_sets_optional_fields() {
		let context = ParserContext::new("book.epub".to_string())
			.with_password("secret".to_string())
			.with_forced_extension("txt".to_string());
		assert_eq!(context.file_path, "book.epub");
		assert_eq!(context.password.as_deref(), Some("secret"));
		assert_eq!(context.forced_extension.as_deref(), Some("txt"));
	}
}
