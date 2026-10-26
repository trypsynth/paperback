//! Slices a bounded window of a document's text (plus the formatting markers within it) for a
//! GUI text control that can't hold an entire huge document without perceptible cost - see
//! `crates/paperback/src/ui/text_window.rs` for the caller-side half of this.

use std::collections::BTreeMap;

use super::{DocumentSession, LineMarker};
use crate::{
	document::{ContentBytePos, DocLen, DocPos, DocumentBuffer, MarkerType, is_heading_marker},
	parser::util::toc::marker_type_to_heading_level,
};

/// How much of a document is shown at once when it is too long to show
/// whole: the width of the reader's loaded window, and of the slice the web view is handed.
pub const WINDOW_DISPLAY_LEN: DocLen = DocLen::from_raw(500_000);

/// The document length up to which the whole thing is shown rather than a window around the
/// reading position. One extension chunk above [`WINDOW_DISPLAY_LEN`], so a document a little
/// over a window's width is still shown whole rather than windowed from the outset.
///
/// Shared between the reader and the web view deliberately. They used to decide separately,
/// the reader against this and the web view against [`WINDOW_DISPLAY_LEN`], so a document
/// between the two was loaded whole by one and sliced by the other: the web view dropped the
/// text past its slice, dropped any link whose target lay outside it, and announced itself as
/// a partial view of a document the reader was showing in full.
pub const WHOLE_DOCUMENT_DISPLAY_LEN: DocLen =
	DocLen::from_raw(WINDOW_DISPLAY_LEN.get() + WINDOW_DISPLAY_LEN.get() / 2);

/// A contiguous slice of a document's text, plus the Bold/Italic/Underline
/// markers and the heading and list paragraphs that fall within it, rebased to positions in
/// `text` itself.
#[derive(Debug, Clone)]
pub struct WindowSlice {
	/// Document position of the first char of `text`, after boundary
	/// snapping - may differ from whatever raw start [`DocumentSession::get_window`] was asked
	/// for.
	pub start: DocPos,
	/// Document position one past the last char of `text`.
	pub end: DocPos,
	pub text: String,
	/// Positions are in `text` (`start` is the document position of 0), not in the document.
	pub markers: Vec<LineMarker>,
	/// The headings and lists in `text`, as the whole paragraphs they cover, sorted and
	/// non-overlapping. Paragraphs that are neither have no entry.
	pub paragraphs: Vec<ParagraphFormat>,
}

/// A run of one or more whole paragraphs of a [`WindowSlice`] sharing a heading level and list
/// membership, so the reading control can carry them as paragraph formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParagraphFormat {
	/// Position in the slice's text of the first paragraph's start.
	pub start: DocPos,
	/// Position in the slice's text one past the last paragraph's `\n`, or the window's end.
	pub end: DocPos,
	pub heading_level: Option<u8>,
	pub list_item: bool,
}

impl DocumentSession {
	/// How long the document is.
	#[must_use]
	pub const fn document_len(&self) -> DocLen {
		self.handle.document().buffer.len()
	}

	/// The text between two positions, `start` inclusive and `end` exclusive.
	///
	/// Nothing is snapped to a paragraph boundary: a bound landing mid-word cuts the word, which
	/// is the point when the caller is naming two caret positions.
	///
	/// Both bounds are clamped to the document; an empty or reversed range is an empty string.
	#[must_use]
	pub fn get_text_range_display(&self, start: DocPos, end: DocPos) -> String {
		self.handle.document().buffer.text(start..end).to_string()
	}

	/// The position just past the character at `display_pos`, so that
	/// `get_text_range_display(x, session.display_pos_after_char_at(y))` covers every character
	/// from `x` through the one a caret at `y` is sitting on.
	///
	/// A caret sits at the *start* of the character it names - which is the character a screen
	/// reader is speaking - so an exclusive end bound drops it.
	/// Past the last character it is the document length, so "the character at the caret" at the
	/// very end of a document stays clamped instead of running off it.
	#[must_use]
	pub fn display_pos_after_char_at(&self, display_pos: DocPos) -> DocPos {
		let buf = &self.handle.document().buffer;
		(display_pos.min(buf.end()) + DocLen::ONE).min(buf.end())
	}

	/// Slices `[raw_start, raw_end)` (clamped to the document) out of the
	/// content, snapping both edges outward to the nearest paragraph boundary so a window
	/// never starts or ends mid-paragraph, then collects the Bold/Italic/Underline markers
	/// that overlap it, clipped and rebased to positions in the slice's text, and its heading and list
	/// paragraphs.
	#[must_use]
	pub fn get_window(&self, raw_start: DocPos, raw_end: DocPos) -> WindowSlice {
		let buf = &self.handle.document().buffer;
		let raw_start = raw_start.min(buf.end());
		let raw_end = raw_end.min(buf.end()).max(raw_start);
		let byte_start = snap_start_to_paragraph_boundary(buf, buf.byte_at(raw_start));
		let byte_end = snap_end_to_paragraph_boundary(buf, buf.byte_at(raw_end));
		let start = buf.pos_at_byte(byte_start);
		let end = buf.pos_at_byte(byte_end);
		let text = buf.slice(byte_start..byte_end).to_string();
		let markers = buf
			.markers
			.iter()
			.filter(|m| matches!(m.mtype, MarkerType::Bold | MarkerType::Italic | MarkerType::Underline))
			.filter_map(|m| {
				let marker_start = m.position;
				let marker_end = m.position + m.length;
				if marker_end <= start || marker_start >= end {
					return None; // fully outside the window
				}
				let clipped_start = marker_start.max(start);
				let clipped_end = marker_end.min(end);
				(clipped_end > clipped_start).then(|| LineMarker {
					mtype: m.mtype,
					position: (clipped_start - start).as_pos(),
					text: String::new(),
					reference: String::new(),
					level: 0,
					length: clipped_end - clipped_start,
				})
			})
			.collect();
		WindowSlice { start, end, text, markers, paragraphs: paragraph_formats(buf, start, end) }
	}
}

/// The paragraphs within the window `[start, end)` of `buf` that are headings or part
/// of a list, merged into runs and rebased to positions in the window's text. Both bounds must sit on
/// paragraph boundaries, which [`DocumentSession::get_window`]'s snapping guarantees.
///
/// A heading or list item marks the whole paragraph its position falls in: headings resolved from
/// a table of contents can point partway into a line. A list marks every paragraph its span
/// overlaps, so a line continuing an item stays inside the list.
fn paragraph_formats(buf: &DocumentBuffer, start: DocPos, end: DocPos) -> Vec<ParagraphFormat> {
	// Keyed by paragraph start byte: (paragraph end byte, heading level, list item).
	let mut by_paragraph: BTreeMap<ContentBytePos, (ContentBytePos, Option<u8>, bool)> = BTreeMap::new();
	for marker in &buf.markers {
		let (length, heading_level, list_item) = match marker.mtype {
			MarkerType::List => (marker.length, None, true),
			MarkerType::ListItem => (DocLen::ONE, None, true),
			mtype if is_heading_marker(mtype) => {
				(DocLen::ONE, u8::try_from(marker_type_to_heading_level(mtype)).ok(), false)
			}
			_ => continue,
		};
		let span_start = marker.position.max(start);
		let span_end = (marker.position + length).min(end);
		if span_start >= span_end {
			continue;
		}
		let span_end = buf.byte_at(span_end);
		let mut paragraph_start = snap_start_to_paragraph_boundary(buf, buf.byte_at(span_start));
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
	let local = |byte| (buf.pos_at_byte(byte) - start).as_pos();
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
		let len = session.document_len().as_pos();
		let slice = session.get_window(0.into(), len);
		assert_eq!(slice.start, 0);
		assert_eq!(slice.end, len);
		assert_eq!(slice.text, "first paragraph\nsecond paragraph\nthird paragraph");
	}

	#[test]
	fn get_window_snaps_edges_outward_to_paragraph_boundaries() {
		let content = "first paragraph\nsecond paragraph\nthird paragraph";
		let session = session_with(content, &[]);
		// Ask for a range landing mid-word in both "second paragraph" (start) and itself (end).
		let mid_second_start = DocPos::from(content.find("second").unwrap() + 3);
		let mid_second_end = DocPos::from(content.find("second paragraph").unwrap() + 10);
		let slice = session.get_window(mid_second_start, mid_second_end);
		assert_eq!(slice.text, "second paragraph\n");
	}

	#[test]
	fn get_window_clamps_out_of_range_bounds() {
		let session = session_with("hello world", &[]);
		let len = session.document_len().as_pos();
		let slice = session.get_window(0.into(), 100_000.into());
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
		let markers = [Marker::new(MarkerType::Bold, marker_start.into()).with_length(marker_len.into())];
		let session = session_with(content, &markers);
		// A request landing inside "beta gamma" should snap to just that paragraph, not pull
		// in "alpha" too.
		let raw_start = DocPos::from(content.find("beta").unwrap());
		let slice = session.get_window(raw_start, raw_start + 2.into());
		assert_eq!(slice.text, "beta gamma\n");
		// The marker is clipped to only the portion that falls inside this window.
		assert_eq!(slice.markers.len(), 1);
		let m = &slice.markers[0];
		let clipped = &slice.text[m.position.get()..(m.position + m.length).get()];
		assert_eq!(clipped, "beta g");
	}

	#[test]
	fn get_window_excludes_markers_entirely_outside_the_window() {
		let content = "alpha\nbeta gamma\ndelta";
		let markers = [Marker::new(MarkerType::Italic, 0.into()).with_length(5.into())]; // "alpha"
		let session = session_with(content, &markers);
		let raw_start = DocPos::from(content.find("delta").unwrap());
		let slice = session.get_window(raw_start, raw_start + 2.into());
		assert_eq!(slice.text, "delta");
		assert!(slice.markers.is_empty());
	}

	#[test]
	fn get_window_filters_to_bold_italic_underline_only() {
		let content = "some heading text";
		let markers = [
			Marker::new(MarkerType::Heading1, 0.into()).with_length(4.into()),
			Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
		];
		let session = session_with(content, &markers);
		let slice = session.get_window(0.into(), session.document_len().as_pos());
		assert_eq!(slice.markers.len(), 1);
		assert_eq!(slice.markers[0].mtype, MarkerType::Bold);
	}

	fn paragraph(start: usize, end: usize, heading_level: Option<u8>, list_item: bool) -> ParagraphFormat {
		ParagraphFormat { start: start.into(), end: end.into(), heading_level, list_item }
	}

	fn whole_window_paragraphs(content: &str, markers: &[Marker]) -> Vec<ParagraphFormat> {
		let session = session_with(content, markers);
		session.get_window(0.into(), session.document_len().as_pos()).paragraphs
	}

	#[test]
	fn heading_marks_its_whole_paragraph_with_its_level() {
		let content = "intro\nTitle\nbody\n";
		let markers = [Marker::new(MarkerType::Heading2, 6.into())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(6, 12, Some(2), false)]);
	}

	// Headings found through the table of contents sit wherever their anchor was, which can be
	// partway into a line.
	#[test]
	fn heading_marker_mid_line_marks_the_containing_paragraph() {
		let content = "intro\nsome Title here\nbody";
		let markers = [Marker::new(MarkerType::Heading1, content.find("Title").unwrap().into())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(6, 22, Some(1), false)]);
	}

	#[test]
	fn list_span_marks_every_paragraph_it_covers_as_one_run() {
		// The "b" line continues the first item: it is in the list without a bullet of its own.
		let content = "before\n• a\nb\n• c\nafter\n";
		let markers = [Marker::new(MarkerType::List, 7.into()).with_length(10.into())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(7, 17, None, true)]);
	}

	// Tagged PDF lists carry no length, so their items are all there is to go on.
	#[test]
	fn list_item_without_a_list_span_marks_its_paragraph() {
		let content = "x\n• item\ny";
		let markers = [Marker::new(MarkerType::ListItem, 2.into())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(2, 9, None, true)]);
	}

	#[test]
	fn heading_inside_a_list_carries_both() {
		let content = "• a\nHead\n• b\n";
		let markers = [
			Marker::new(MarkerType::List, 0.into()).with_length(13.into()),
			Marker::new(MarkerType::Heading3, 4.into()),
		];
		assert_eq!(
			whole_window_paragraphs(content, &markers),
			vec![paragraph(0, 4, None, true), paragraph(4, 9, Some(3), true), paragraph(9, 13, None, true)]
		);
	}

	#[test]
	fn paragraph_formats_outside_the_window_are_dropped_and_straddling_lists_are_clipped() {
		let content = "Head\n• a\n• b\n• c\ntail\n";
		let markers = [
			Marker::new(MarkerType::Heading1, 0.into()),
			Marker::new(MarkerType::List, 5.into()).with_length(12.into()),
		];
		let session = session_with(content, &markers);
		// Positions, not bytes: each bullet is one position but three bytes.
		let slice = session.get_window(9.into(), 10.into());
		assert_eq!(slice.text, "• b\n");
		assert_eq!(slice.paragraphs, vec![paragraph(0, 4, None, true)]);
	}

	#[test]
	fn paragraph_formats_are_window_local() {
		let content = "aaa\nbbb\nHead\nccc";
		let markers = [Marker::new(MarkerType::Heading6, 8.into())];
		let session = session_with(content, &markers);
		let slice = session.get_window(4.into(), session.document_len().as_pos());
		assert_eq!(slice.text, "bbb\nHead\nccc");
		assert_eq!(slice.paragraphs, vec![paragraph(4, 9, Some(6), false)]);
	}

	#[test]
	fn paragraph_positions_count_an_astral_character_as_one() {
		let content = "a\u{1F600}b\nHead\n";
		let markers = [Marker::new(MarkerType::Heading1, 4.into())];
		assert_eq!(whole_window_paragraphs(content, &markers), vec![paragraph(4, 9, Some(1), false)]);
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
		let len = session.document_len().as_pos();
		let first = session.get_window(0.into(), 20.into());
		assert!(first.end > 0.into() && first.end < len, "need a window that stops short of the end");
		let second = session.get_window(first.end, len);
		assert_eq!(second.start, first.end, "the next window must begin where the last one ended");
		assert_eq!(
			format!("{}{}", first.text, second.text),
			session.get_window(0.into(), len).text,
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
		let len = session.document_len().as_pos();
		let mut assembled = String::new();
		let mut cursor = DocPos::ZERO;
		while cursor < len {
			let slice = session.get_window(cursor, cursor + 30.into());
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
		assert_eq!(session.get_text_range_display(0.into(), session.document_len().as_pos()), content);
	}

	/// The property that separates this from `get_window`: what was asked for is what comes back,
	/// even when it starts and ends mid-word.
	#[test]
	fn get_text_range_display_extracts_an_interior_range_without_snapping() {
		let content = "first paragraph\nsecond paragraph\nthird paragraph";
		let session = session_with(content, &[]);
		let start = DocPos::from(content.find("second").unwrap() + 3);
		let end = DocPos::from(content.find("second paragraph").unwrap() + 10);
		assert_eq!(session.get_text_range_display(start, end), "ond par");
		// The window call on the same bounds swallows both paragraphs whole.
		assert_eq!(session.get_window(start, end).text, "second paragraph\n");
	}

	#[test]
	fn get_text_range_display_clamps_and_handles_empty_or_reversed_ranges() {
		let session = session_with("hello world", &[]);
		let len = session.document_len().as_pos();
		assert_eq!(session.get_text_range_display(0.into(), 100_000.into()), "hello world");
		assert_eq!(session.get_text_range_display(0.into(), len), "hello world");
		assert_eq!(session.get_text_range_display(3.into(), 3.into()), "");
		assert_eq!(session.get_text_range_display(7.into(), 2.into()), "");
		assert_eq!(session.get_text_range_display(len + 5.into(), len + 9.into()), "");
	}

	/// The inclusive end bound: a caret names the character it sits on, so a range ending at the
	/// caret has to reach past that character rather than up to its start.
	#[test]
	fn display_pos_after_char_at_covers_the_character_the_caret_is_on() {
		let session = session_with("hello world", &[]);
		let end = |pos| session.display_pos_after_char_at(pos);
		assert_eq!(session.get_text_range_display(0.into(), end(0.into())), "h");
		assert_eq!(session.get_text_range_display(0.into(), end(6.into())), "hello w");
		// Past the last character it is the document length, never one beyond it.
		assert_eq!(end(session.document_len().as_pos()), session.document_len().as_pos());
		assert_eq!(end(10_000.into()), session.document_len().as_pos());
	}

	/// An astral-plane character is one character the reader hears, and one position.
	#[test]
	fn display_pos_after_char_at_spans_an_astral_character() {
		let session = session_with("a\u{1F600}bc", &[]);
		let end = |pos| session.display_pos_after_char_at(pos);
		assert_eq!(session.get_text_range_display(0.into(), end(0.into())), "a");
		assert_eq!(session.get_text_range_display(1.into(), end(1.into())), "\u{1F600}");
		assert_eq!(session.get_text_range_display(2.into(), end(2.into())), "b");
		assert_eq!(session.get_text_range_display(0.into(), end(3.into())), "a\u{1F600}bc");
	}

	#[test]
	fn get_text_range_display_agrees_with_the_buffer_on_astral_characters() {
		let session = session_with("a\u{1F600}bc", &[]);
		// Positions: 'a' at 0, the emoji at 1, 'b' at 2, 'c' at 3.
		assert_eq!(session.document_len(), 4);
		assert_eq!(session.get_text_range_display(0.into(), 4.into()), "a\u{1F600}bc");
		assert_eq!(session.get_text_range_display(1.into(), 2.into()), "\u{1F600}");
		assert_eq!(session.get_text_range_display(2.into(), 4.into()), "bc");
	}
}
