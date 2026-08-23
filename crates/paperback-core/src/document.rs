use std::collections::HashMap;

/// Re-exported so `crate::document::ParserFlags` keeps working; the flags themselves are
/// declared alongside the rest of each format's metadata in `paperback-formats`.
pub use paperback_formats::ParserFlags;

use crate::{
	audio::AudioTimeline,
	types::HeadingInfo,
	util::text::{ch_width, display_len, is_space_like},
};

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
	Image = 14,
	Figure = 15,
	Bold = 16,
	Italic = 17,
	Underline = 18,
}

impl From<MarkerType> for i32 {
	fn from(marker: MarkerType) -> Self {
		marker as Self
	}
}

/// Yields the character-formatting marker types implied by the given
/// bold/italic/underline flags, in a stable order. Shared by the parsers so
/// the flag-triple → marker fan-out lives in one place.
pub(crate) fn format_marker_types(bold: bool, italic: bool, underline: bool) -> impl Iterator<Item = MarkerType> {
	[(bold, MarkerType::Bold), (italic, MarkerType::Italic), (underline, MarkerType::Underline)]
		.into_iter()
		.filter_map(|(on, kind)| on.then_some(kind))
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
			14 => Ok(Self::Image),
			15 => Ok(Self::Figure),
			16 => Ok(Self::Bold),
			17 => Ok(Self::Italic),
			18 => Ok(Self::Underline),
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
	content_char_count: usize,
	newline_char_positions: Vec<usize>,
	char_to_byte_map: Vec<usize>,
	/// `display_len_at_char[i]` is the display-unit offset (UTF-16 code units on
	/// Windows/macOS, Unicode scalars on GTK — see `util::text::display_len`) of the `i`-th
	/// char, with a trailing end-boundary entry equal to `content_display_len`, mirroring
	/// `char_to_byte_map`'s shape. Needed because `Marker.position`/the GUI caret use display
	/// units while `char_to_byte_map`/`newline_char_positions` use char units — on Windows and
	/// macOS these two diverge for any character outside the Basic Multilingual Plane.
	display_len_at_char: Vec<usize>,
}

impl DocumentBuffer {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			content: String::new(),
			markers: Vec::new(),
			content_display_len: 0,
			content_char_count: 0,
			newline_char_positions: Vec::new(),
			char_to_byte_map: Vec::new(),
			display_len_at_char: Vec::new(),
		}
	}

	#[must_use]
	pub fn with_content(content: String) -> Self {
		let mut char_count = 0usize;
		let mut display_count = 0usize;
		let mut newline_char_positions = Vec::new();
		let mut char_to_byte_map = Vec::with_capacity(content.len().min(1024));
		let mut display_len_at_char = Vec::with_capacity(content.len().min(1024));
		for (byte_idx, c) in content.char_indices() {
			char_to_byte_map.push(byte_idx);
			display_len_at_char.push(display_count);
			if c == '\n' {
				newline_char_positions.push(char_count);
			}
			char_count += 1;
			display_count += ch_width(c);
		}
		char_to_byte_map.push(content.len()); // append end boundary
		display_len_at_char.push(display_count);
		debug_assert_eq!(display_count, display_len(&content));
		Self {
			content,
			markers: Vec::new(),
			content_display_len: display_count,
			content_char_count: char_count,
			newline_char_positions,
			char_to_byte_map,
			display_len_at_char,
		}
	}

	pub fn add_marker(&mut self, marker: Marker) {
		self.markers.push(marker);
	}

	pub fn append(&mut self, text: &str) {
		let base = self.content_char_count;
		let mut count = 0usize;
		let mut display_count = self.content_display_len;

		// Remove the end boundary temporarily
		if !self.char_to_byte_map.is_empty() {
			self.char_to_byte_map.pop();
		}
		if !self.display_len_at_char.is_empty() {
			self.display_len_at_char.pop();
		}

		let start_byte = self.content.len();
		for (byte_idx, c) in text.char_indices() {
			self.char_to_byte_map.push(start_byte + byte_idx);
			self.display_len_at_char.push(display_count);
			if c == '\n' {
				self.newline_char_positions.push(base + count);
			}
			count += 1;
			display_count += ch_width(c);
		}
		self.content.push_str(text);
		self.char_to_byte_map.push(self.content.len()); // append end boundary back
		self.display_len_at_char.push(display_count);
		self.content_display_len = display_count;
		self.content_char_count += count;
	}

	#[must_use]
	pub fn byte_index_for_char(&self, char_index: usize) -> usize {
		self.char_to_byte_map.get(char_index).copied().unwrap_or(self.content.len())
	}

	#[must_use]
	pub fn char_index_for_byte(&self, byte_index: usize) -> usize {
		self.char_to_byte_map.binary_search(&byte_index).unwrap_or_else(|idx| idx)
	}

	/// The display-unit offset of the `char_index`-th char (or the document's total display
	/// length, for a char index at or past the end).
	#[must_use]
	pub fn display_index_for_char(&self, char_index: usize) -> usize {
		self.display_len_at_char.get(char_index).copied().unwrap_or(self.content_display_len)
	}

	/// The char index whose char covers `display_index`, or the nearest one before it if
	/// `display_index` lands inside a surrogate pair (shouldn't happen for a well-formed
	/// caller, but this stays well-defined rather than panicking).
	#[must_use]
	pub fn char_index_for_display(&self, display_index: usize) -> usize {
		self.display_len_at_char.binary_search(&display_index).unwrap_or_else(|idx| idx)
	}

	/// The byte offset corresponding to `display_index`, composing [`Self::char_index_for_display`]
	/// with [`Self::byte_index_for_char`].
	#[must_use]
	pub fn byte_index_for_display(&self, display_index: usize) -> usize {
		self.byte_index_for_char(self.char_index_for_display(display_index))
	}

	/// The display-unit offset corresponding to `byte_index`, composing
	/// [`Self::char_index_for_byte`] with [`Self::display_index_for_char`].
	#[must_use]
	pub fn display_index_for_byte(&self, byte_index: usize) -> usize {
		self.display_index_for_char(self.char_index_for_byte(byte_index))
	}

	#[must_use]
	pub const fn current_position(&self) -> usize {
		self.content_display_len
	}

	/// Total document length in display units (UTF-16 code units on Windows/macOS, Unicode
	/// scalars on GTK) — the same unit the GUI caret and `Marker.position` use. An alias of
	/// [`Self::current_position`] under the name callers building a window/range API actually
	/// want; `current_position` is kept for the parser-cursor callers already using that name.
	#[must_use]
	pub const fn total_display_len(&self) -> usize {
		self.content_display_len
	}

	#[must_use]
	pub const fn char_count(&self) -> usize {
		self.content_char_count
	}

	#[must_use]
	pub fn newline_positions(&self) -> &[usize] {
		&self.newline_char_positions
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
	/// Recorded audio for this document, when it has any.
	pub audio: Option<AudioTimeline>,
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
			audio: None,
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

	pub fn set_audio(&mut self, audio: AudioTimeline) {
		self.audio = Some(audio);
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

/// Whether a marker type denotes a navigable container (an element the caret can be *inside* of,
/// such as a list or table). The single place to extend the set of container types.
#[must_use]
pub const fn is_container_marker(marker_type: MarkerType) -> bool {
	matches!(marker_type, MarkerType::List | MarkerType::Table)
}

/// The display-unit span of a container marker: `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerSpan {
	pub start: usize,
	pub end: usize,
	pub mtype: MarkerType,
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

#[derive(Debug, Clone)]
pub struct ParserContext {
	pub file_path: String,
	pub password: Option<String>,
	pub forced_extension: Option<String>,
	/// When `true`, parsers emit each table's full tab-separated rendering inline; when `false`,
	/// they emit a `"[Table]: <first row>"` placeholder. Threaded into each parser at parse time.
	pub render_tables_inline: bool,
}

impl ParserContext {
	#[must_use]
	pub const fn new(file_path: String) -> Self {
		Self { file_path, password: None, forced_extension: None, render_tables_inline: true }
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

	#[must_use]
	pub const fn with_render_tables_inline(mut self, value: bool) -> Self {
		self.render_tables_inline = value;
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
	fn marker_type_round_trip_for_all_known_values() {
		for raw in 0..=18 {
			let marker = MarkerType::try_from(raw).unwrap();
			assert_eq!(i32::from(marker), raw);
		}
		assert!(MarkerType::try_from(19).is_err());
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
		assert_eq!(handle.section_index(61), Some(1));
		assert_eq!(handle.page_index(25), Some(0));
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

	// `ch_width`/`display_len` return UTF-16 code-unit counts on Windows/macOS and Unicode
	// scalar counts on GTK, so a BMP-only string ("abc") behaves identically on every
	// platform and is the right fixture for testing the display<->char<->byte bridge itself
	// without also depending on which platform the test runs on.
	#[test]
	fn display_char_byte_indices_agree_for_bmp_only_content() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		for i in 0..=3 {
			assert_eq!(buffer.display_index_for_char(i), i);
			assert_eq!(buffer.char_index_for_display(i), i);
			assert_eq!(buffer.byte_index_for_display(i), i);
			assert_eq!(buffer.display_index_for_byte(i), i);
		}
		assert_eq!(buffer.total_display_len(), 3);
	}

	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn display_index_accounts_for_surrogate_pairs_on_windows_and_macos() {
		// U+1F600 (an astral-plane emoji) is 1 char, 4 UTF-8 bytes, but 2 UTF-16 code units.
		let buffer = DocumentBuffer::with_content("a\u{1F600}b".to_string());
		// chars: 'a' (char 0), emoji (char 1), 'b' (char 2), end boundary (char 3)
		assert_eq!(buffer.display_index_for_char(0), 0); // 'a' starts at display 0
		assert_eq!(buffer.display_index_for_char(1), 1); // emoji starts at display 1
		assert_eq!(buffer.display_index_for_char(2), 3); // 'b' starts at display 3 (emoji took 2 units)
		assert_eq!(buffer.total_display_len(), 4);

		assert_eq!(buffer.char_index_for_display(0), 0);
		assert_eq!(buffer.char_index_for_display(1), 1);
		assert_eq!(buffer.char_index_for_display(3), 2);

		// byte<->display composition round-trips through the char index for every char start.
		assert_eq!(buffer.byte_index_for_display(0), 0);
		assert_eq!(buffer.byte_index_for_display(1), 1);
		assert_eq!(buffer.byte_index_for_display(3), 5); // 1 byte 'a' + 4 byte emoji
		assert_eq!(buffer.display_index_for_byte(0), 0);
		assert_eq!(buffer.display_index_for_byte(1), 1);
		assert_eq!(buffer.display_index_for_byte(5), 3);
	}

	#[test]
	fn display_index_for_char_clamps_past_the_end() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		assert_eq!(buffer.display_index_for_char(999), buffer.total_display_len());
	}

	#[test]
	fn append_extends_the_display_index_incrementally() {
		let mut buffer = DocumentBuffer::new();
		buffer.append("ab");
		buffer.append("cd");
		assert_eq!(buffer.total_display_len(), 4);
		for i in 0..=4 {
			assert_eq!(buffer.display_index_for_char(i), i, "char {i}");
		}
	}
}
