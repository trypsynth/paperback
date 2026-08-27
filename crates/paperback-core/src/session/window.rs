//! Slices a bounded window of a document's text (plus the formatting markers within it) for a
//! GUI text control that can't hold an entire huge document without perceptible cost - see
//! `crates/paperback/src/ui/text_window.rs` for the caller-side half of this.

use super::{DocumentSession, LineMarker};
use crate::document::MarkerType;

/// A contiguous slice of a document's text, in display units, plus the Bold/Italic/Underline
/// markers that fall within it, rebased to window-local display-unit positions.
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
}

impl DocumentSession {
	/// Total document length in display units - the same unit [`Self::get_window`]'s bounds,
	/// the GUI caret, and `Marker.position` all use.
	#[must_use]
	pub fn document_len(&self) -> i64 {
		i64::try_from(self.handle.document().buffer.total_display_len()).unwrap_or(i64::MAX)
	}

	/// Slices `[raw_start, raw_end)` (display units, clamped to the document) out of the
	/// content, snapping both edges outward to the nearest paragraph boundary so a window
	/// never starts or ends mid-paragraph, then collects the Bold/Italic/Underline markers
	/// that overlap it, clipped and rebased to window-local positions.
	#[must_use]
	pub fn get_window(&self, raw_start: i64, raw_end: i64) -> WindowSlice {
		let buf = &self.handle.document().buffer;
		let doc_len = buf.total_display_len();
		let raw_start = usize::try_from(raw_start.max(0)).unwrap_or(0).min(doc_len);
		let raw_end = usize::try_from(raw_end.max(0)).unwrap_or(0).clamp(raw_start, doc_len);
		let byte_start = snap_start_to_paragraph_boundary(&buf.content, buf.byte_index_for_display(raw_start));
		let byte_end = snap_end_to_paragraph_boundary(&buf.content, buf.byte_index_for_display(raw_end));
		let start = buf.display_index_for_byte(byte_start);
		let end = buf.display_index_for_byte(byte_end);
		let text = buf.content[byte_start..byte_end].to_string();
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
		WindowSlice { start: i64::try_from(start).unwrap_or(0), end: i64::try_from(end).unwrap_or(0), text, markers }
	}
}

/// Extends `byte_idx` backward to the start of the paragraph it's in - the byte right after
/// the nearest preceding `\n`, or 0 if there isn't one - so a window never starts mid-paragraph.
fn snap_start_to_paragraph_boundary(content: &str, byte_idx: usize) -> usize {
	content[..byte_idx].rfind('\n').map_or(0, |i| i + 1)
}

/// Extends `byte_idx` forward to the end of the paragraph it's in - one past the nearest
/// following `\n`, or the end of the content if there isn't one.
fn snap_end_to_paragraph_boundary(content: &str, byte_idx: usize) -> usize {
	content[byte_idx..].find('\n').map_or(content.len(), |i| byte_idx + i + 1)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, DocumentHandle, Marker, MarkerType, ParserFlags};

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
			last_stable_position: None,
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
}
