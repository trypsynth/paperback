//! Slices a bounded window of a document's text (plus the formatting markers within it) for a
//! GUI text control that can't hold an entire huge document without perceptible cost - see
//! `crates/paperback/src/ui/text_window.rs` for the caller-side half of this.

use std::collections::BTreeMap;

use super::{DocumentSession, LineMarker};
use crate::{
	document::{ContentBytePos, DocumentBuffer, MarkerType, is_heading_marker},
	parser::util::toc::marker_type_to_heading_level,
};

/// How much of a document, in display units, is shown at once when it is too long to show
/// whole: the width of the reader's loaded window, and of the slice the web view is handed.
pub const WINDOW_DISPLAY_LEN: i64 = 500_000;

/// The document length up to which the whole thing is shown rather than a window around the
/// reading position. One extension chunk above [`WINDOW_DISPLAY_LEN`], so a document a little
/// over a window's width is still shown whole rather than windowed from the outset.
///
/// Shared between the reader and the web view deliberately. They used to decide separately,
/// the reader against this and the web view against [`WINDOW_DISPLAY_LEN`], so a document
/// between the two was loaded whole by one and sliced by the other: the web view dropped the
/// text past its slice, dropped any link whose target lay outside it, and announced itself as
/// a partial view of a document the reader was showing in full.
pub const WHOLE_DOCUMENT_DISPLAY_LEN: i64 = WINDOW_DISPLAY_LEN + WINDOW_DISPLAY_LEN / 2;

/// A contiguous slice of a document's text, in display units, plus the Bold/Italic/Underline
/// markers and the heading and list paragraphs that fall within it, rebased to window-local
/// display-unit positions.
#[derive(Debug, Clone)]
pub struct WindowSlice {
	/// Document-absolute display-unit offset of the first char of `text`, after boundary
	/// snapping - may differ from whatever raw start [`DocumentSession::get_window`] was asked
	/// for.
	pub start: i64,
	/// Document-absolute display-unit offset one past the last char of `text`.
	pub end: i64,
	pub text: String,
	/// Positions/lengths are window-local (relative to `start`), not document-absolute.
	pub markers: Vec<LineMarker>,
	/// The headings and lists in `text`, as the whole paragraphs they cover, sorted and
	/// non-overlapping. Paragraphs that are neither have no entry.
	pub paragraphs: Vec<ParagraphFormat>,
}

/// A run of one or more whole paragraphs of a [`WindowSlice`] sharing a heading level and list
/// membership, so the reading control can carry them as paragraph formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParagraphFormat {
	/// Window-local display-unit offset of the first paragraph's start.
	pub start: i64,
	/// Window-local display-unit offset one past the last paragraph's `\n`, or the window's end.
	pub end: i64,
	pub heading_level: Option<u8>,
	pub list_item: bool,
}

impl DocumentSession {
	/// Total document length in display units - the same unit [`Self::get_window`]'s bounds,
	/// the GUI caret, and `Marker.position` all use.
	#[must_use]
	pub fn document_len(&self) -> i64 {
		i64::try_from(self.handle.document().buffer.total_display_len()).unwrap_or(i64::MAX)
	}

	/// The text between two display-unit positions, `start` inclusive and `end` exclusive - the
	/// same unit [`Self::document_len`], the GUI caret and `Marker.position` all use.
	///
	/// Converts through [`crate::document::DocumentBuffer::byte_index_for_display`] so astral-plane
	/// characters (one char, two display units on Windows and macOS) preserve caret and marker bounds.
	/// Nothing is snapped to a paragraph boundary: a bound landing mid-word cuts the word, which
	/// is the point when the caller is naming two caret positions.
	///
	/// Both bounds are clamped to the document; an empty or reversed range is an empty string.
	#[must_use]
	pub fn get_text_range_display(&self, start: i64, end: i64) -> String {
		let buf = &self.handle.document().buffer;
		let doc_len = buf.total_display_len();
		let start_display = usize::try_from(start.max(0)).unwrap_or(0).min(doc_len);
		let end_display = usize::try_from(end.max(0)).unwrap_or(0).min(doc_len);
		if start_display >= end_display {
			return String::new();
		}
		// The buffer answers with the start of a char, so both bounds are char boundaries.
		let start_byte = buf.byte_index_for_display(start_display);
		let end_byte = buf.byte_index_for_display(end_display);
		buf.slice(start_byte..end_byte).to_string()
	}

	/// The display-unit position just past the character at `display_pos`, so that
	/// `get_text_range_display(x, session.display_pos_after_char_at(y))` covers every character
	/// from `x` through the one a caret at `y` is sitting on.
	///
	/// A caret sits at the *start* of the character it names - which is the character a screen
	/// reader is speaking - so an exclusive end bound drops it. This spans the whole character,
	/// an astral-plane pair included, rather than the single display unit its start sits on.
	/// Past the last character it is the document length, so "the character at the caret" at the
	/// very end of a document stays clamped instead of running off it.
	#[must_use]
	pub fn display_pos_after_char_at(&self, display_pos: i64) -> i64 {
		let buf = &self.handle.document().buffer;
		let total = buf.total_display_len();
		let pos = usize::try_from(display_pos.max(0)).unwrap_or(0).min(total);
		let char_index = buf.char_index_for_display(pos);
		i64::try_from(buf.display_index_for_char(char_index + 1)).unwrap_or(i64::MAX)
	}

	/// Slices `[raw_start, raw_end)` (display units, clamped to the document) out of the
	/// content, snapping both edges outward to the nearest paragraph boundary so a window
	/// never starts or ends mid-paragraph, then collects the Bold/Italic/Underline markers
	/// that overlap it, clipped and rebased to window-local positions, and its heading and list
	/// paragraphs.
	#[must_use]
	pub fn get_window(&self, raw_start: i64, raw_end: i64) -> WindowSlice {
		let buf = &self.handle.document().buffer;
		let doc_len = buf.total_display_len();
		let raw_start = usize::try_from(raw_start.max(0)).unwrap_or(0).min(doc_len);
		let raw_end = usize::try_from(raw_end.max(0)).unwrap_or(0).clamp(raw_start, doc_len);
		let byte_start = snap_start_to_paragraph_boundary(buf, buf.byte_index_for_display(raw_start));
		let byte_end = snap_end_to_paragraph_boundary(buf, buf.byte_index_for_display(raw_end));
		let start = buf.display_index_for_byte(byte_start);
		let end = buf.display_index_for_byte(byte_end);
		let text = buf.slice(byte_start..byte_end).to_string();
		let markers = buf
			.markers
			.iter()
			.filter(|m| matches!(m.mtype, MarkerType::Bold | MarkerType::Italic | MarkerType::Underline))
			.filter_map(|m| {
				let marker_start = m.position;
				let marker_end = m.position.saturating_add(m.length);
				if marker_end <= start || marker_start >= end {
					return None; // fully outside the window
				}
				let clipped_start = marker_start.max(start);
				let clipped_end = marker_end.min(end);
				(clipped_end > clipped_start).then(|| LineMarker {
					mtype: m.mtype,
					position: i64::try_from(clipped_start - start).unwrap_or(0),
					text: String::new(),
					reference: String::new(),
					level: 0,
					length: i64::try_from(clipped_end - clipped_start).unwrap_or(0),
				})
			})
			.collect();
		WindowSlice {
			start: i64::try_from(start).unwrap_or(0),
			end: i64::try_from(end).unwrap_or(0),
			text,
			markers,
			paragraphs: paragraph_formats(buf, start, end),
		}
	}
}

/// The paragraphs within the display-unit window `[start, end)` of `buf` that are headings or part
/// of a list, merged into runs and rebased to window-local positions. Both bounds must sit on
/// paragraph boundaries, which [`DocumentSession::get_window`]'s snapping guarantees.
///
/// A heading or list item marks the whole paragraph its position falls in: headings resolved from
/// a table of contents can point partway into a line. A list marks every paragraph its span
/// overlaps, so a line continuing an item stays inside the list.
fn paragraph_formats(buf: &DocumentBuffer, start: usize, end: usize) -> Vec<ParagraphFormat> {
	// Keyed by paragraph start byte: (paragraph end byte, heading level, list item).
	let mut by_paragraph: BTreeMap<ContentBytePos, (ContentBytePos, Option<u8>, bool)> = BTreeMap::new();
	for marker in &buf.markers {
		let (length, heading_level, list_item) = match marker.mtype {
			MarkerType::List => (marker.length, None, true),
			MarkerType::ListItem => (1, None, true),
			mtype if is_heading_marker(mtype) => (1, u8::try_from(marker_type_to_heading_level(mtype)).ok(), false),
			_ => continue,
		};
		let span_start = marker.position.max(start);
		let span_end = marker.position.saturating_add(length).min(end);
		if span_start >= span_end {
			continue;
		}
		let span_end = buf.byte_index_for_display(span_end);
		let mut paragraph_start = snap_start_to_paragraph_boundary(buf, buf.byte_index_for_display(span_start));
		loop {
			let paragraph_end = snap_end_to_paragraph_boundary(buf, paragraph_start);
			let (_, level, item) = by_paragraph.entry(paragraph_start).or_insert((paragraph_end, None, false));
			*level = level.or(heading_level);
			*item |= list_item;
			if paragraph_end >= span_end {
				break;
			}
			paragraph_start = paragraph_end;
		}
	}
	let local = |byte| i64::try_from(buf.display_index_for_byte(byte) - start).unwrap_or(0);
	let mut runs: Vec<ParagraphFormat> = Vec::new();
	for (paragraph_start, (paragraph_end, heading_level, list_item)) in by_paragraph {
		let (run_start, run_end) = (local(paragraph_start), local(paragraph_end));
		match runs.last_mut() {
			Some(last)
				if last.end == run_start && last.heading_level == heading_level && last.list_item == list_item =>
			{
				last.end = run_end;
			}
			_ => runs.push(ParagraphFormat { start: run_start, end: run_end, heading_level, list_item }),
		}
	}
	runs
}

/// Extends `byte_idx` backward to the start of the paragraph it's in - the byte right after
/// the nearest preceding `\n`, or 0 if there isn't one - so a window never starts mid-paragraph.
pub(super) fn snap_start_to_paragraph_boundary(buffer: &DocumentBuffer, byte_idx: ContentBytePos) -> ContentBytePos {
	buffer.slice(..byte_idx).rfind('\n').map_or(ContentBytePos::ZERO, |i| ContentBytePos::from_raw(i + 1))
}

/// Extends `byte_idx` forward to the end of the paragraph it's in - one past the nearest
/// following `\n`, or the end of the content if there isn't one.
pub(super) fn snap_end_to_paragraph_boundary(buffer: &DocumentBuffer, byte_idx: ContentBytePos) -> ContentBytePos {
	buffer.slice(byte_idx..).find('\n').map_or_else(|| buffer.content_end(), |i| byte_idx + i + 1)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, DocumentHandle, Marker, MarkerType, ParseSettings, ParserFlags};

	fn session_with(content: &str, markers: &[Marker]) -> DocumentSession {
		let mut buffer = DocumentBuffer::with_content(content.to_string());
		for m in markers {
			buffer.add_marker(m.clone());
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::empty(),
			parse_settings: ParseSettings::default(),
			password: String::new(),
		}
	}

	#[test]
	fn get_window_full_range_round_trips_whole_document() {
		let session = session_with("first paragraph\nsecond paragraph\nthird paragraph", &[]);
		let len = session.document_len();
		let slice = session.get_window(0, len);
		assert_eq!(slice.start, 0);
		assert_eq!(slice.end, len);
		assert_eq!(slice.text, "first paragraph\nsecond paragraph\nthird paragraph");
	}

	#[test]
	fn get_window_snaps_edges_outward_to_paragraph_boundaries() {
		let content = "first paragraph\nsecond paragraph\nthird paragraph";
		let session = session_with(content, &[]);
		// Ask for a range landing mid-word in both "second paragraph" (start) and itself (end).
		let mid_second_start = content.find("second").unwrap() as i64 + 3;
		let mid_second_end = content.find("second paragraph").unwrap() as i64 + 10;
		let slice = session.get_window(mid_second_start, mid_second_end);
		assert_eq!(slice.text, "second paragraph\n");
	}

	#[test]
	fn get_window_clamps_out_of_range_bounds() {
		let session = session_with("hello world", &[]);
		let len = session.document_len();
		let slice = session.get_window(-100, 100_000);
		assert_eq!(slice.start, 0);
		assert_eq!(slice.end, len);
		assert_eq!(slice.text, "hello world");
	}

	#[test]
	fn get_window_clips_and_rebases_markers_straddling_the_window() {
		let content = "alpha\nbeta gamma\ndelta";
		// Bold spans "ha\nbeta g" - partway into "alpha" through partway into "beta gamma",
		// straddling the paragraph 1/2 boundary.
		let marker_start = 3;
		let marker_len = 9;
		assert_eq!(&content[marker_start..marker_start + marker_len], "ha\nbeta g");
		let markers = [Marker::new(MarkerType::Bold, marker_start).with_length(marker_len)];
		let session = session_with(content, &markers);
		// A request landing inside "beta gamma" should snap to just that paragraph, not pull
		// in "alpha" too.
		let raw_start = content.find("beta").unwrap() as i64;
		let slice = session.get_window(raw_start, raw_start + 2);
		assert_eq!(slice.text, "beta gamma\n");
		// The marker is clipped to only the portion that falls inside this window.
		assert_eq!(slice.markers.len(), 1);
		let m = &slice.markers[0];
		let clipped = &slice.text[m.position as usize..(m.position + m.length) as usize];
		assert_eq!(clipped, "beta g");
	}

	#[test]
	fn get_window_excludes_markers_entirely_outside_the_window() {
		let content = "alpha\nbeta gamma\ndelta";
		let markers = [Marker::new(MarkerType::Italic, 0).with_length(5)]; // "alpha"
		let session = session_with(content, &markers);
		let raw_start = content.find("delta").unwrap() as i64;
		let slice = session.get_window(raw_start, raw_start + 2);
		assert_eq!(slice.text, "delta");
		assert!(slice.markers.is_empty());
	}

	#[test]
	fn get_window_filters_to_bold_italic_underline_only() {
		let content = "some heading text";
		let markers =
			[Marker::new(MarkerType::Heading1, 0).with_length(4), Marker::new(MarkerType::Bold, 0).with_length(4)];
		let session = session_with(content, &markers);
		let slice = session.get_window(0, session.document_len());
		assert_eq!(slice.markers.len(), 1);
		assert_eq!(slice.markers[0].mtype, MarkerType::Bold);
	}

	fn paragraph(start: i64, end: i64, heading_level: Option<u8>, list_item: bool) -> ParagraphFormat {
		ParagraphFormat { start, end, heading_level, list_item }
	}

	fn whole_window_paragraphs(content: &str, markers: &[Marker]) -> Vec<ParagraphFormat> {
		let session = session_with(content, markers);
		session.get_window(0, session.document_len()).paragraphs
	}

	#[test]
	fn heading_marks_its_whole_paragraph_with_its_level() {
		let content = "intro\nTitle\nbody\n";
		let markers = [Marker::new(MarkerType::Heading2, 6)];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(6, 12, Some(2), false)]);
	}

	// Headings found through the table of contents sit wherever their anchor was, which can be
	// partway into a line.
	#[test]
	fn heading_marker_mid_line_marks_the_containing_paragraph() {
		let content = "intro\nsome Title here\nbody";
		let markers = [Marker::new(MarkerType::Heading1, content.find("Title").unwrap())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(6, 22, Some(1), false)]);
	}

	#[test]
	fn list_span_marks_every_paragraph_it_covers_as_one_run() {
		// The "b" line continues the first item: it is in the list without a bullet of its own.
		let content = "before\n• a\nb\n• c\nafter\n";
		let markers = [Marker::new(MarkerType::List, 7).with_length(10)];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(7, 17, None, true)]);
	}

	// Tagged PDF lists carry no length, so their items are all there is to go on.
	#[test]
	fn list_item_without_a_list_span_marks_its_paragraph() {
		let content = "x\n• item\ny";
		let markers = [Marker::new(MarkerType::ListItem, 2)];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(2, 9, None, true)]);
	}

	#[test]
	fn heading_inside_a_list_carries_both() {
		let content = "• a\nHead\n• b\n";
		let markers = [Marker::new(MarkerType::List, 0).with_length(13), Marker::new(MarkerType::Heading3, 4)];
		assert_eq!(
			whole_window_paragraphs(content, &markers),
			vec![paragraph(0, 4, None, true), paragraph(4, 9, Some(3), true), paragraph(9, 13, None, true)]
		);
	}

	#[test]
	fn paragraph_formats_outside_the_window_are_dropped_and_straddling_lists_are_clipped() {
		let content = "Head\n• a\n• b\n• c\ntail\n";
		let markers = [Marker::new(MarkerType::Heading1, 0), Marker::new(MarkerType::List, 5).with_length(12)];
		let session = session_with(content, &markers);
		// Display units, not bytes: each bullet is one unit but three bytes.
		let slice = session.get_window(9, 10);
		assert_eq!(slice.text, "• b\n");
		assert_eq!(slice.paragraphs, vec![paragraph(0, 4, None, true)]);
	}

	#[test]
	fn paragraph_formats_are_window_local() {
		let content = "aaa\nbbb\nHead\nccc";
		let markers = [Marker::new(MarkerType::Heading6, 8)];
		let session = session_with(content, &markers);
		let slice = session.get_window(4, session.document_len());
		assert_eq!(slice.text, "bbb\nHead\nccc");
		assert_eq!(slice.paragraphs, vec![paragraph(4, 9, Some(6), false)]);
	}

	/// Gated to the platforms where an astral-plane character is two display units.
	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn paragraph_positions_count_astral_characters_as_two_units() {
		let content = "a\u{1F600}b\nHead\n";
		let markers = [Marker::new(MarkerType::Heading1, 5)];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(5, 10, Some(1), false)]);
	}
	// The UI grows a document window forward by asking for the range starting at the one it
	// already has loaded, and appending the result. That only works because an existing window's
	// `end` is already on a paragraph boundary, so snapping it again as a `start` is a no-op. If
	// that ever stopped holding, every extension would repeat or drop a paragraph and shift every
	// offset the text control has already handed out.
	#[test]
	fn a_windows_end_is_a_boundary_that_the_next_window_starts_exactly_on() {
		let content = "alpha paragraph
beta paragraph
gamma paragraph
delta paragraph
epsilon";
		let session = session_with(content, &[]);
		let len = session.document_len();
		let first = session.get_window(0, 20);
		assert!(first.end > 0 && first.end < len, "need a window that stops short of the end");
		let second = session.get_window(first.end, len);
		assert_eq!(second.start, first.end, "the next window must begin where the last one ended");
		assert_eq!(
			format!("{}{}", first.text, second.text),
			session.get_window(0, len).text,
			"appending the two must reconstruct the document exactly"
		);
	}

	// Same property across several hops, which is what a long read actually does.
	#[test]
	fn repeated_extensions_reconstruct_the_document() {
		let mut content = String::new();
		for i in 0..50 {
			use std::fmt::Write as _;
			let _ = writeln!(content, "paragraph number {i} with some words in it");
		}
		let session = session_with(&content, &[]);
		let len = session.document_len();
		let mut assembled = String::new();
		let mut cursor = 0;
		while cursor < len {
			let slice = session.get_window(cursor, cursor + 30);
			assert_eq!(slice.start, cursor, "extension must abut the previous window");
			assert!(slice.end > cursor, "extension must make progress");
			assembled.push_str(&slice.text);
			cursor = slice.end;
		}
		assert_eq!(assembled, content);
	}

	#[test]
	fn get_text_range_display_round_trips_the_whole_document() {
		let content = "first paragraph\nsecond paragraph\nthird paragraph";
		let session = session_with(content, &[]);
		assert_eq!(session.get_text_range_display(0, session.document_len()), content);
	}

	/// The property that separates this from `get_window`: what was asked for is what comes back,
	/// even when it starts and ends mid-word.
	#[test]
	fn get_text_range_display_extracts_an_interior_range_without_snapping() {
		let content = "first paragraph\nsecond paragraph\nthird paragraph";
		let session = session_with(content, &[]);
		let start = content.find("second").unwrap() as i64 + 3;
		let end = content.find("second paragraph").unwrap() as i64 + 10;
		assert_eq!(session.get_text_range_display(start, end), "ond par");
		// The window call on the same bounds swallows both paragraphs whole.
		assert_eq!(session.get_window(start, end).text, "second paragraph\n");
	}

	#[test]
	fn get_text_range_display_clamps_and_handles_empty_or_reversed_ranges() {
		let session = session_with("hello world", &[]);
		let len = session.document_len();
		assert_eq!(session.get_text_range_display(-100, 100_000), "hello world");
		assert_eq!(session.get_text_range_display(0, len), "hello world");
		assert_eq!(session.get_text_range_display(3, 3), "");
		assert_eq!(session.get_text_range_display(7, 2), "");
		assert_eq!(session.get_text_range_display(len + 5, len + 9), "");
	}

	/// The inclusive end bound: a caret names the character it sits on, so a range ending at the
	/// caret has to reach past that character rather than up to its start.
	#[test]
	fn display_pos_after_char_at_covers_the_character_the_caret_is_on() {
		let session = session_with("hello world", &[]);
		let end = |pos| session.display_pos_after_char_at(pos);
		assert_eq!(session.get_text_range_display(0, end(0)), "h");
		assert_eq!(session.get_text_range_display(0, end(6)), "hello w");
		// Past the last character it is the document length, never one beyond it.
		assert_eq!(end(session.document_len()), session.document_len());
		assert_eq!(end(10_000), session.document_len());
	}

	/// An astral-plane character is one character the reader hears and two display units, so the
	/// end bound has to clear the whole pair rather than land inside it.
	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn display_pos_after_char_at_spans_a_whole_astral_pair() {
		let session = session_with("a\u{1F600}bc", &[]);
		let end = |pos| session.display_pos_after_char_at(pos);
		assert_eq!(session.get_text_range_display(0, end(0)), "a");
		assert_eq!(session.get_text_range_display(1, end(1)), "\u{1F600}");
		assert_eq!(session.get_text_range_display(3, end(3)), "b");
		assert_eq!(session.get_text_range_display(0, end(4)), "a\u{1F600}bc");
	}

	/// An astral-plane character is one char but two display units on Windows and macOS.
	/// Gated to the platforms where the two units differ; GTK display units are scalars.
	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn get_text_range_display_agrees_with_the_buffer_on_astral_characters() {
		let session = session_with("a\u{1F600}bc", &[]);
		// Display units: 'a' at 0, the emoji across 1..3, 'b' at 3, 'c' at 4.
		assert_eq!(session.document_len(), 5);
		assert_eq!(session.get_text_range_display(0, 5), "a\u{1F600}bc");
		assert_eq!(session.get_text_range_display(1, 3), "\u{1F600}");
		assert_eq!(session.get_text_range_display(3, 5), "bc");
	}
}
