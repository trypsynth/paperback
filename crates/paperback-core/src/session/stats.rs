//! Position/line/page bookkeeping and the read-aloud/status-bar queries built on it: status
//! bar text, percent-to-position mapping, line and page lookups, formatting-marker spans,
//! and [`DocumentSession::get_text_segment`] (the paragraph/line/element text used by TTS
//! and "read current element" commands).

use super::{
	DocumentSession, DocumentStatsFfi, LineMarker, SegmentDirectionFfi, SegmentTypeFfi, StatusInfo, TextSegmentFfi,
};
use crate::{
	document::{MarkerType, ParserFlags},
	reader_core::reader_navigate,
	types::{self as ffi, NavDirection, NavTarget},
};

impl DocumentSession {
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
				position: i64::try_from(m.position).unwrap_or(0),
				text: String::new(),
				reference: String::new(),
				level: 0,
				length: i64::try_from(m.length).unwrap_or(0),
			})
			.collect()
	}

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
		supported
	}

	#[must_use]
	pub fn page_count_ffi(&self) -> i32 {
		i32::try_from(self.page_count()).unwrap_or(0)
	}

	#[must_use]
	pub fn get_table_at_position(&self, position: i64) -> Option<String> {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let table_index = self.handle.current_marker_index(pos_usize, MarkerType::Table)?;
		let marker = self.handle.document().buffer.markers.get(table_index)?;
		// `length` is the display extent (Tasks 2-3); valid range is the half-open `[position, end)`.
		let table_end = marker.position + marker.length;
		if pos_usize < marker.position || pos_usize >= table_end {
			return None;
		}
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn get_current_section_path(&self, position: i64) -> Option<String> {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let section_index = self.handle.current_marker_index(pos_usize, MarkerType::SectionBreak)?;
		let marker = self.handle.document().buffer.markers.get(section_index)?;
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn get_text_segment(
		&self,
		position: i64,
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
				// `offset` is the marker's own real position and must never move, regardless of
				// what the fallback below does for display text: a plain-audio DAISY section, for
				// instance, has no newlines anywhere in its buffer, so `find_paragraph_boundaries`
				// collapses to the whole buffer (0..len) at every position, and re-deriving
				// `start_pos` from that would silently discard the section actually found and
				// snap every result back to position 0.
				let offset = res.offset as i64;
				let mut text = res.marker_text.clone();
				let mut end_pos = offset;
				if text.trim().is_empty() {
					let content = &self.handle.document().buffer.content;
					let total_chars = self.handle.document().buffer.char_count();
					let start_pos_char = usize::try_from(offset.max(0)).unwrap_or(0).min(total_chars);
					let byte_idx = self.handle.document().buffer.byte_index_for_char(start_pos_char);
					let (start_byte, end_byte) =
						self.find_paragraph_boundaries(content, byte_idx, SegmentDirectionFfi::Current);
					text = content[start_byte..end_byte].trim().to_string();
					end_pos = offset + i64::try_from(text.chars().count()).unwrap_or(0);
				} else {
					end_pos += i64::try_from(text.chars().count()).unwrap_or(0);
				}
				return TextSegmentFfi { text, start_pos: offset, end_pos, found: true };
			}
			return TextSegmentFfi { text: String::new(), start_pos: position, end_pos: position, found: false };
		}
		let content = &self.handle.document().buffer.content;
		let total_chars = self.handle.document().buffer.char_count();
		let start_pos_char = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let byte_idx = self.handle.document().buffer.byte_index_for_char(start_pos_char);
		let (start_byte, end_byte) = if matches!(segment_type, SegmentTypeFfi::Line) {
			let line_num = self.line_from_position(start_pos_char as i64);
			let target_line = match direction {
				SegmentDirectionFfi::Previous => (line_num - 1).max(1),
				SegmentDirectionFfi::Next => line_num + 1,
				SegmentDirectionFfi::Current => line_num,
			};
			let start_char_idx = usize::try_from(self.position_from_line(target_line)).unwrap_or(0);
			let end_char_idx = usize::try_from(self.position_from_line(target_line + 1)).unwrap_or(0);
			let sb = self.handle.document().buffer.byte_index_for_char(start_char_idx);
			let eb = self.handle.document().buffer.byte_index_for_char(end_char_idx);
			(sb, eb)
		} else {
			self.find_paragraph_boundaries(content, byte_idx, direction)
		};
		let text = content[start_byte..end_byte].trim().to_string();
		let start_char = self.handle.document().buffer.char_index_for_byte(start_byte);
		let end_char = self.handle.document().buffer.char_index_for_byte(end_byte);
		TextSegmentFfi {
			text,
			start_pos: i64::try_from(start_char).unwrap_or(0),
			end_pos: i64::try_from(end_char).unwrap_or(0),
			// Next past the last paragraph/line (or Previous before the first) yields an empty
			// range here rather than an error; treat that the same as any other not-found miss
			// so callers don't stop on it and speak/persist a bogus end-of-buffer position.
			found: start_byte < end_byte,
		}
	}

	fn find_paragraph_boundaries(
		&self,
		content: &str,
		byte_idx: usize,
		direction: SegmentDirectionFfi,
	) -> (usize, usize) {
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
		(start, end)
	}

	#[must_use]
	pub fn get_status_info(&self, position: i64) -> StatusInfo {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		let character_number = pos + 1;
		let percentage = (pos * 100).checked_div(total_chars).unwrap_or(0);
		StatusInfo {
			line_number: i64::try_from(line_number).unwrap_or(1),
			character_number: i64::try_from(character_number).unwrap_or(1),
			percentage: i32::try_from(percentage).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn position_from_percent(&self, percent: i32) -> i64 {
		let total_chars = i64::try_from(self.handle.document().buffer.char_count()).unwrap_or(0);
		let percent = i64::from(percent.clamp(0, 100));
		if total_chars == 0 {
			return 0;
		}
		// Ceiling division: (percent * total_chars + 99) / 100
		(percent * total_chars + 99) / 100
	}

	#[must_use]
	pub fn line_count(&self) -> i64 {
		let newline_count = self.handle.document().buffer.newline_positions().len();
		// Line count is newlines + 1 (last line may not have trailing newline)
		i64::try_from(newline_count + 1).unwrap_or(1)
	}

	#[must_use]
	pub fn position_from_line(&self, line: i64) -> i64 {
		if line <= 1 {
			return 0;
		}
		let buf = &self.handle.document().buffer;
		let target_newlines = usize::try_from(line - 1).unwrap_or(0);
		let newlines = buf.newline_positions();
		if target_newlines <= newlines.len() {
			i64::try_from(newlines[target_newlines - 1] + 1).unwrap_or(0)
		} else {
			i64::try_from(buf.char_count()).unwrap_or(0)
		}
	}

	#[must_use]
	pub fn line_from_position(&self, position: i64) -> i64 {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		i64::try_from(line_number).unwrap_or(1)
	}

	#[must_use]
	pub fn page_count(&self) -> usize {
		self.handle.count_markers_by_type(MarkerType::PageBreak)
	}

	#[must_use]
	pub fn current_page(&self, position: i64) -> i32 {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		self.handle.page_index(pos).map_or(0, |idx| idx + 1)
	}

	#[must_use]
	pub fn page_offset(&self, page_number: i32) -> i64 {
		let index = page_number - 1;
		if index < 0 {
			return -1;
		}
		self.handle
			.get_marker_position_by_index(MarkerType::PageBreak, index)
			.map_or(-1, |offset| i64::try_from(offset).unwrap_or(-1))
	}

	/// Returns the text between two positions (start inclusive, end exclusive).
	#[must_use]
	pub fn get_text_range(&self, start: i64, end: i64) -> String {
		let total_chars = self.handle.document().buffer.char_count();
		let start_pos = usize::try_from(start.max(0)).unwrap_or(0).min(total_chars);
		let end_pos = usize::try_from(end.max(0)).unwrap_or(0).min(total_chars);
		if start_pos >= end_pos {
			return String::new();
		}
		let start_byte = self.handle.document().buffer.byte_index_for_char(start_pos);
		let end_byte = self.handle.document().buffer.byte_index_for_char(end_pos);
		self.handle.document().buffer.content[start_byte..end_byte].to_string()
	}

	#[must_use]
	pub fn get_line_text(&self, position: i64) -> String {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let newlines = buf.newline_positions();
		let line_start = match newlines.partition_point(|&p| p < pos) {
			0 => 0,
			idx => newlines[idx - 1] + 1,
		};
		let start_byte = buf.byte_index_for_char(line_start);
		let line_end_byte = buf.content[start_byte..].find('\n').map_or(buf.content.len(), |i| start_byte + i);
		buf.content[start_byte..line_end_byte].to_string()
	}

	/// Returns the first non-blank line of real content at or after `position`, skipping blank
	/// lines and bare page-number headers (e.g. "27" or "Page 27"). Returns an empty string if
	/// no such line exists. Used for page-navigation announcements so the page number isn't
	/// announced twice (the page marker's own text is a "Page N" label).
	#[must_use]
	pub fn first_content_line_after(&self, position: i64) -> String {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let mut pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let newlines = buf.newline_positions();
		loop {
			let idx = newlines.partition_point(|&p| p < pos);
			let line_start = if idx == 0 { 0 } else { newlines[idx - 1] + 1 };
			let line_end = newlines.get(idx).copied().unwrap_or(total_chars);
			let line =
				self.get_text_range(i64::try_from(line_start).unwrap_or(0), i64::try_from(line_end).unwrap_or(0));
			if is_content_line(line.trim()) {
				return line;
			}
			if line_end >= total_chars {
				return String::new();
			}
			pos = line_end + 1;
		}
	}

	#[must_use]
	pub fn get_line_markers(&self, line: i64) -> Vec<LineMarker> {
		let start_pos = self.position_from_line(line);
		let end_pos = self.position_from_line(line + 1);
		let start_usize = usize::try_from(start_pos.max(0)).unwrap_or(0);
		// If line + 1 overflows or is the end, end_pos might be equal to start_pos
		let end_usize = if start_pos == end_pos { usize::MAX } else { usize::try_from(end_pos.max(0)).unwrap_or(0) };
		let mut res = Vec::new();
		for marker in &self.handle.document().buffer.markers {
			if marker.position >= start_usize && marker.position < end_usize {
				res.push(LineMarker {
					mtype: marker.mtype,
					position: i64::try_from(marker.position).unwrap_or(0),
					text: marker.text.clone(),
					reference: marker.reference.clone(),
					level: marker.level,
					length: i64::try_from(marker.length).unwrap_or(0),
				});
			} else if marker.position > end_usize {
				break;
			}
		}
		res
	}
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
