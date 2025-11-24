use std::collections::HashMap;

use bitflags::bitflags;

use crate::utils::text::display_len;

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
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Marker {
	pub marker_type: MarkerType,
	pub position: usize,
	pub text: String,
	pub reference: String,
	pub level: i32,
}

impl Marker {
	#[must_use]
	pub const fn new(marker_type: MarkerType, position: usize) -> Self {
		Self { marker_type, position, text: String::new(), reference: String::new(), level: 0 }
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
	pub children: Vec<TocItem>,
}

impl TocItem {
	#[must_use]
	pub const fn new(name: String, reference: String, offset: usize) -> Self {
		Self { name, reference, offset, children: Vec::new() }
	}

	pub fn add_child(&mut self, child: Self) {
		self.children.push(child);
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
		let char_count_no_whitespace = text.chars().filter(|c| !c.is_whitespace()).count();
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

const fn is_heading_marker(marker_type: MarkerType) -> bool {
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

	#[must_use]
	pub const fn document_mut(&mut self) -> &mut Document {
		&mut self.doc
	}

	fn markers_by_type(&self, marker_type: MarkerType) -> impl Iterator<Item = (usize, &Marker)> {
		self.doc.buffer.markers.iter().enumerate().filter(move |(_, m)| m.marker_type == marker_type)
	}

	fn heading_markers(&self, level: Option<i32>) -> Vec<(usize, &Marker)> {
		let mut result: Vec<(usize, &Marker)> = self
			.doc
			.buffer
			.markers
			.iter()
			.enumerate()
			.filter(|(_, marker)| is_heading_marker(marker.marker_type))
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
				marker.marker_type == marker_type && i64::try_from(marker.position).unwrap_or(i64::MAX) > position
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
				marker.marker_type == marker_type && i64::try_from(marker.position).unwrap_or(i64::MAX) < position
			})
			.map(|(idx, _)| idx)
			.next_back()
	}

	#[must_use]
	pub fn current_marker_index(&self, position: usize, marker_type: MarkerType) -> Option<usize> {
		let mut result = None;
		for (idx, marker) in self.doc.buffer.markers.iter().enumerate() {
			if marker.marker_type == marker_type && marker.position <= position {
				result = Some(idx);
			} else if marker.position > position {
				break;
			}
		}
		result
	}

	#[must_use]
	pub fn find_first_marker_after(&self, position: i64, marker_type: MarkerType) -> Option<usize> {
		self.doc
			.buffer
			.markers
			.iter()
			.enumerate()
			.find(|(_, marker)| {
				marker.marker_type == marker_type && i64::try_from(marker.position).unwrap_or(i64::MAX) >= position
			})
			.map(|(idx, _)| idx)
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
	pub fn heading_info(&self, heading_index: i32) -> Option<crate::html_to_text::HeadingInfo> {
		let idx = usize::try_from(heading_index).ok()?;
		let heading_markers = self.heading_markers(None);
		let (_, marker) = heading_markers.get(idx)?;
		Some(crate::html_to_text::HeadingInfo {
			offset: marker.position,
			level: marker.level,
			text: marker.text.clone(),
		})
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
		self.doc.buffer.markers.iter().filter(|m| m.marker_type == marker_type).count()
	}

	#[must_use]
	pub fn get_marker_position_by_index(&self, marker_type: MarkerType, index: i32) -> Option<usize> {
		let target = usize::try_from(index).ok()?;
		self.markers_by_type(marker_type).nth(target).map(|(_, marker)| marker.position)
	}

	#[must_use]
	pub fn next_section_index(&self, position: i64) -> Option<i32> {
		self.next_marker_index(position, MarkerType::SectionBreak).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn previous_section_index(&self, position: i64) -> Option<i32> {
		self.previous_marker_index(position, MarkerType::SectionBreak).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn section_index(&self, position: usize) -> Option<i32> {
		self.current_marker_index(position, MarkerType::SectionBreak).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn next_page_index(&self, position: i64) -> Option<i32> {
		self.next_marker_index(position, MarkerType::PageBreak).and_then(|idx| i32::try_from(idx).ok())
	}

	#[must_use]
	pub fn previous_page_index(&self, position: i64) -> Option<i32> {
		self.previous_marker_index(position, MarkerType::PageBreak).and_then(|idx| i32::try_from(idx).ok())
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
}

impl ParserContext {
	#[must_use]
	pub const fn new(file_path: String) -> Self {
		Self { file_path, password: None }
	}

	#[must_use]
	pub fn with_password(mut self, password: String) -> Self {
		self.password = Some(password);
		self
	}
}
