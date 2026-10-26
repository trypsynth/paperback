//! Position/line/page bookkeeping and the read-aloud/status-bar queries built on it: status
//! bar text, percent-to-position mapping, line and page lookups, formatting-marker spans,
//! and [`DocumentSession::get_text_segment`] (the paragraph/line/element text used by TTS
//! and "read current element" commands).

use super::{
	DocumentSession, DocumentStatsFfi, LineMarker, SegmentDirectionFfi, SegmentTypeFfi, StatusInfo, TextSegmentFfi,
};
use crate::{
	document::{ContentBytePos, DocLen, DocPos, DocumentBuffer, MarkerType, ParserFlags},
	reader_core::reader_navigate,
	types::{self as ffi, NavDirection, NavTarget},
};

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	#[must_use]
	pub fn get_stats_ffi(&self) -> DocumentStatsFfi {
		let s = self.stats();
		DocumentStatsFfi {
			word_count: i64::try_from(s.word_count).unwrap_or(0),
			line_count: i64::try_from(s.line_count).unwrap_or(0),
			char_count: i64::try_from(s.char_count).unwrap_or(0),
			char_count_no_whitespace: i64::try_from(s.char_count_no_whitespace).unwrap_or(0),
			audio_file_count: i64::try_from(s.audio_file_count).unwrap_or(0),
			audio_total_duration_ms: i64::try_from(s.audio_total_duration_ms).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn get_supported_segment_types_ffi(&self) -> Vec<SegmentTypeFfi> {
		let mut supported = vec![SegmentTypeFfi::Paragraph, SegmentTypeFfi::Line];
		let has_heading = (0..=5).any(|level| {
			let mtype = match level {
				0 => MarkerType::Heading1,
				1 => MarkerType::Heading2,
				2 => MarkerType::Heading3,
				3 => MarkerType::Heading4,
				4 => MarkerType::Heading5,
				_ => MarkerType::Heading6,
			};
			self.has_marker(mtype)
		});
		if has_heading {
			supported.push(SegmentTypeFfi::Heading);
		}
		if self.has_marker(MarkerType::Link) {
			supported.push(SegmentTypeFfi::Link);
		}
		if self.parser_flags.contains(ParserFlags::SUPPORTS_SECTIONS) && self.has_marker(MarkerType::SectionBreak) {
			supported.push(SegmentTypeFfi::Section);
		}
		if self.parser_flags.contains(ParserFlags::SUPPORTS_PAGES) && self.has_marker(MarkerType::PageBreak) {
			supported.push(SegmentTypeFfi::Page);
		}
		if self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::List) {
			supported.push(SegmentTypeFfi::List);
		}
		if self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::ListItem) {
			supported.push(SegmentTypeFfi::ListItem);
		}
		if self.has_marker(MarkerType::Table) {
			supported.push(SegmentTypeFfi::Table);
		}
		if self.has_marker(MarkerType::Separator) {
			supported.push(SegmentTypeFfi::Separator);
		}
		if self.has_marker(MarkerType::Image) {
			supported.push(SegmentTypeFfi::Image);
		}
		if self.has_marker(MarkerType::Figure) {
			supported.push(SegmentTypeFfi::Figure);
		}
		if self.has_marker(MarkerType::Formula) {
			supported.push(SegmentTypeFfi::Formula);
		}
		supported
	}

	#[must_use]
	pub fn page_count_ffi(&self) -> i32 {
		i32::try_from(self.page_count()).unwrap_or(0)
	}

	#[must_use]
	pub fn get_text_segment(
		&self,
		position: DocPos,
		segment_type: SegmentTypeFfi,
		direction: SegmentDirectionFfi,
	) -> TextSegmentFfi {
		let nav_target = match segment_type {
			SegmentTypeFfi::Heading => Some(NavTarget::Heading),
			SegmentTypeFfi::Link => Some(NavTarget::Link),
			SegmentTypeFfi::Section => Some(NavTarget::Section),
			SegmentTypeFfi::Page => Some(NavTarget::Page),
			SegmentTypeFfi::List => Some(NavTarget::List),
			SegmentTypeFfi::ListItem => Some(NavTarget::ListItem),
			SegmentTypeFfi::Table => Some(NavTarget::Table),
			SegmentTypeFfi::Separator => Some(NavTarget::Separator),
			SegmentTypeFfi::Image => Some(NavTarget::Image),
			SegmentTypeFfi::Figure => Some(NavTarget::Figure),
			SegmentTypeFfi::Formula => Some(NavTarget::Formula),
			_ => None,
		};
		if let Some(target) = nav_target {
			let direction_nav = match direction {
				SegmentDirectionFfi::Previous => NavDirection::Previous,
				_ => NavDirection::Next,
			};
			let nav_req = ffi::NavRequest { position, wrap: false, direction: direction_nav, target, level_filter: 0 };
			let res = reader_navigate(&self.handle, &nav_req);
			if res.found {
				// `res.offset` is the marker's own real position and must never move, regardless of
				// what the fallback below does for display text: a plain-audio DAISY section, for
				// instance, has no newlines anywhere in its buffer, so `find_paragraph_boundaries`
				// collapses to the whole buffer (0..len) at every position, and re-deriving
				// `start_pos` from that would silently discard the section actually found and
				// snap every result back to position 0.
				let mut text = res.marker_text.clone();
				if text.trim().is_empty() {
					let buffer = &self.handle.document().buffer;
					let byte_idx = buffer.byte_at(res.offset);
					let (start_byte, end_byte) = self.find_paragraph_boundaries(byte_idx, SegmentDirectionFfi::Current);
					text = buffer.slice(start_byte..end_byte).trim().to_string();
				}
				let end_pos = res.offset + DocLen::of(&text);
				return TextSegmentFfi { text, start_pos: res.offset, end_pos, found: true };
			}
			return TextSegmentFfi { text: String::new(), start_pos: position, end_pos: position, found: false };
		}
		let buffer = &self.handle.document().buffer;
		let start = position.min(buffer.end());
		let byte_idx = buffer.byte_at(start);
		let (start_byte, end_byte) = if matches!(segment_type, SegmentTypeFfi::Line) {
			let line_num = self.line_from_position(start);
			let target_line = match direction {
				SegmentDirectionFfi::Previous => (line_num - 1).max(1),
				SegmentDirectionFfi::Next => line_num + 1,
				SegmentDirectionFfi::Current => line_num,
			};
			(
				buffer.byte_at(self.position_from_line(target_line)),
				buffer.byte_at(self.position_from_line(target_line + 1)),
			)
		} else {
			self.find_paragraph_boundaries(byte_idx, direction)
		};
		let text = buffer.slice(start_byte..end_byte).trim().to_string();
		TextSegmentFfi {
			text,
			start_pos: buffer.pos_at_byte(start_byte),
			end_pos: buffer.pos_at_byte(end_byte),
			// Next past the last paragraph/line (or Previous before the first) yields an empty
			// range here rather than an error; treat that the same as any other not-found miss
			// so callers don't stop on it and speak/persist a bogus end-of-buffer position.
			found: start_byte < end_byte,
		}
	}

	#[must_use]
	pub fn get_status_info(&self, position: DocPos) -> StatusInfo {
		let buf = &self.handle.document().buffer;
		let pos = position.min(buf.end());
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		let character_number = pos.get() + 1;
		let percentage = (pos.get() * 100).checked_div(buf.len().get()).unwrap_or(0);
		StatusInfo {
			line_number: i64::try_from(line_number).unwrap_or(1),
			character_number: i64::try_from(character_number).unwrap_or(1),
			percentage: i32::try_from(percentage).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn position_from_percent(&self, percent: i32) -> DocPos {
		let buf = &self.handle.document().buffer;
		let percent = usize::try_from(percent.clamp(0, 100)).unwrap_or(0);
		DocPos::from_raw((percent * buf.len().get()).div_ceil(100)).min(buf.end())
	}

	#[must_use]
	pub fn line_count(&self) -> i64 {
		let newline_count = self.handle.document().buffer.newline_positions().len();
		// Line count is newlines + 1 (last line may not have trailing newline)
		i64::try_from(newline_count + 1).unwrap_or(1)
	}

	#[must_use]
	pub fn position_from_line(&self, line: i64) -> DocPos {
		if line <= 1 {
			return DocPos::ZERO;
		}
		let buf = &self.handle.document().buffer;
		let target_newlines = usize::try_from(line - 1).unwrap_or(0);
		let newlines = buf.newline_positions();
		if target_newlines <= newlines.len() { newlines[target_newlines - 1] + DocLen::ONE } else { buf.end() }
	}

	#[must_use]
	pub fn line_from_position(&self, position: DocPos) -> i64 {
		let buf = &self.handle.document().buffer;
		let pos = position.min(buf.end());
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		i64::try_from(line_number).unwrap_or(1)
	}

	#[must_use]
	pub fn current_page(&self, position: DocPos) -> i32 {
		self.handle.page_index(position.min(self.handle.document().buffer.end())).map_or(0, |idx| idx + 1)
	}

	/// Where `page` (1-based) starts, or -1 when the document has no such page.
	#[must_use]
	pub fn page_offset(&self, page: i32) -> i64 {
		// The apps read the -1 as it is, so it stays; Rust callers have `page_position`.
		self.page_position(page).map_or(-1, |position| i64::try_from(position).unwrap_or(-1))
	}

	#[must_use]
	pub fn get_line_text(&self, position: DocPos) -> String {
		self.line_text_at(position)
	}

	#[must_use]
	pub fn get_line_markers(&self, line: i64) -> Vec<LineMarker> {
		let buf = &self.handle.document().buffer;
		let start = self.position_from_line(line);
		let end_pos = self.position_from_line(line + 1);
		// If line + 1 overflows or is the end, end_pos might be equal to start
		let end = (start != end_pos).then_some(end_pos);
		let mut res = Vec::new();
		for marker in &buf.markers {
			if marker.position >= start && end.is_none_or(|end| marker.position < end) {
				res.push(LineMarker {
					mtype: marker.mtype,
					position: marker.position,
					text: marker.text.clone(),
					reference: marker.reference.clone(),
					level: marker.level,
					length: marker.length,
				});
			} else if end.is_some_and(|end| marker.position > end) {
				break;
			}
		}
		res
	}
}

impl DocumentSession {
	/// Where `page` (1-based) starts, if the document has that page.
	#[must_use]
	pub fn page_position(&self, page: i32) -> Option<DocPos> {
		self.handle.get_marker_position_by_index(MarkerType::PageBreak, page.checked_sub(1)?)
	}

	#[must_use]
	pub fn get_formatting_markers(&self) -> Vec<LineMarker> {
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.filter(|m| matches!(m.mtype, MarkerType::Bold | MarkerType::Italic | MarkerType::Underline))
			.map(|m| LineMarker {
				mtype: m.mtype,
				position: m.position,
				text: String::new(),
				reference: String::new(),
				level: 0,
				length: m.length,
			})
			.collect()
	}

	#[must_use]
	pub fn get_table_at_position(&self, position: DocPos) -> Option<String> {
		self.marker_reference_at(position, MarkerType::Table)
	}

	#[must_use]
	pub fn get_formula_at_position(&self, position: DocPos) -> Option<String> {
		self.marker_reference_at(position, MarkerType::Formula)
	}

	/// Reference of the marker whose half-open extent contains `position`.
	fn marker_reference_at(&self, position: DocPos, mtype: MarkerType) -> Option<String> {
		let pos = position.min(self.handle.document().buffer.end());
		let index = self.handle.current_marker_index(pos, mtype)?;
		let marker = self.handle.document().buffer.markers.get(index)?;
		if pos < marker.position || pos >= marker.position + marker.length {
			return None;
		}
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn get_current_section_path(&self, position: DocPos) -> Option<String> {
		let pos = position.min(self.handle.document().buffer.end());
		let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
		let marker = self.handle.document().buffer.markers.get(section_index)?;
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	fn find_paragraph_boundaries(
		&self,
		byte_idx: ContentBytePos,
		direction: SegmentDirectionFfi,
	) -> (ContentBytePos, ContentBytePos) {
		let content = self.handle.document().buffer.content.as_str();
		let byte_idx = byte_idx.get();
		let mut start = byte_idx;
		if matches!(direction, SegmentDirectionFfi::Previous) {
			let mut search_end = byte_idx;
			while search_end > 0
				&& (content.as_bytes()[search_end - 1] == b'\n' || content.as_bytes()[search_end - 1] == b'\r')
			{
				search_end -= 1;
			}
			start = content[..search_end].rfind('\n').map_or(0, |i| i + 1);
		} else if matches!(direction, SegmentDirectionFfi::Next) {
			if let Some(next) = content[byte_idx..].find('\n') {
				start = byte_idx + next;
				while start < content.len()
					&& (content.as_bytes()[start] == b'\n' || content.as_bytes()[start] == b'\r')
				{
					start += 1;
				}
			} else {
				start = content.len();
			}
		} else {
			// Current: byte_idx may land anywhere inside the enclosing paragraph (e.g. a link
			// marker mid-sentence), not just at its start, so search backward for the nearest
			// preceding newline rather than only trimming forward from byte_idx.
			while start < content.len() && (content.as_bytes()[start] == b'\n' || content.as_bytes()[start] == b'\r') {
				start += 1;
			}
			start = content[..start].rfind('\n').map_or(0, |i| i + 1);
		}
		let end = content[start..].find('\n').map_or(content.len(), |i| start + i);
		(ContentBytePos::from_raw(start), ContentBytePos::from_raw(end))
	}

	#[must_use]
	pub fn page_count(&self) -> usize {
		self.handle.count_markers_by_type(MarkerType::PageBreak)
	}

	/// The position of every page-break marker in document order — one entry per page.
	/// Collected in a single O(markers) pass; [`Self::page_position`] is O(markers) per call, so
	/// callers that want every page should use this once instead of calling it in a loop.
	#[must_use]
	pub fn page_offsets(&self) -> Vec<DocPos> {
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect()
	}

	/// The `[start, end)` span of the line containing `position`, where `end` is the position of
	/// the line's terminating `\n` (exclusive of it). Returns `None` for an empty document.
	#[must_use]
	pub fn line_bounds_at(&self, position: DocPos) -> Option<(DocPos, DocPos)> {
		let buf = &self.handle.document().buffer;
		if buf.is_empty() {
			return None;
		}
		Some(line_around(buf, position.min(buf.end())))
	}

	/// The text of the line containing `position` (see [`Self::line_bounds_at`]); an empty string at the end of the document.
	#[must_use]
	pub fn line_text_at(&self, position: DocPos) -> String {
		let Some((start, end)) = self.line_bounds_at(position) else {
			return String::new();
		};
		self.text_range(start, end)
	}

	/// Returns the first non-blank line of real content at or after `position`, skipping blank
	/// lines and bare page-number headers (e.g. "27" or "Page 27"). Returns an empty string if
	/// no such line exists. Used for page-navigation announcements so the page number isn't
	/// announced twice (the page marker's own text is a "Page N" label).
	#[must_use]
	pub fn first_content_line_after(&self, position: DocPos) -> String {
		let buf = &self.handle.document().buffer;
		let mut pos = position.min(buf.end());
		loop {
			let (line_start, line_end) = line_around(buf, pos);
			let line = buf.text(line_start..line_end);
			if is_content_line(line.trim()) {
				return line.to_string();
			}
			if line_end >= buf.end() {
				return String::new();
			}
			pos = line_end + DocLen::ONE;
		}
	}
}

/// The start of the line holding `pos` and the position of the `\n` that ends it, or of the end of the document for the last line.
fn line_around(buf: &DocumentBuffer, pos: DocPos) -> (DocPos, DocPos) {
	let newlines = buf.newline_positions();
	let idx = newlines.partition_point(|&p| p < pos);
	let start = if idx == 0 { DocPos::ZERO } else { newlines[idx - 1] + DocLen::ONE };
	(start, newlines.get(idx).copied().unwrap_or_else(|| buf.end()))
}

/// Whether a trimmed line looks like real page content rather than a blank line, a bare page
/// number ("27", "27 / 300"), or a "Page N" header.
fn is_content_line(trimmed: &str) -> bool {
	if trimmed.is_empty() {
		return false;
	}
	let lower = trimmed.to_ascii_lowercase();
	let body = lower.strip_prefix("page ").unwrap_or(&lower);
	body.chars().any(char::is_alphabetic)
}
