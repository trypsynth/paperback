//! Merging the parser's bold, italic and underline markers into the non-overlapping runs the
//! reading control and the RTF fast path both draw from. Neither the merge nor the segments it
//! produces touch a widget, so both sides can be tested without one.

/// A non-overlapping run of text with the union of bold/italic/underline
/// styles active over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FormatSegment {
	pub start: i64,
	pub end: i64,
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
}

/// Merges bold/italic/underline markers (which may overlap, e.g. a bold word
/// inside an italic sentence) into a sequence of non-overlapping segments, each
/// carrying the union of the styles active over that range.
///
/// This is required because wxMSW's `wxTextCtrl::SetStyle` rewrites the *entire*
/// font for a range whenever any font attribute is present in the `wxTextAttr`
/// (it masks `CFM_FACE | CFM_SIZE | ...` unconditionally and fills unset fields
/// from a default font, Arial 10pt). Applying overlapping single-style markers
/// one at a time would therefore both reset the face/size and clobber each
/// other's styles. Producing one combined style per non-overlapping segment
/// avoids both problems and is correct on every platform.
///
/// Implemented as a sweep over +1/-1 events per style so it's O(n log n) instead
/// of the naive O(n^2) "rescan every marker at every boundary" approach, which
/// took several seconds on books with tens of thousands of formatting spans.
pub fn merge_formatting_markers(markers: &[paperback_core::session::LineMarker]) -> Vec<FormatSegment> {
	use paperback_core::document::MarkerType;
	#[derive(Clone, Copy)]
	struct Event {
		position: i64,
		delta: i32,
		style_idx: usize,
	}
	let mut events: Vec<Event> = Vec::new();
	for m in markers {
		if m.length <= 0 {
			continue;
		}
		let style_idx = match m.mtype {
			MarkerType::Bold => 0,
			MarkerType::Italic => 1,
			MarkerType::Underline => 2,
			_ => continue,
		};
		events.push(Event { position: m.position, delta: 1, style_idx });
		events.push(Event { position: m.position + m.length, delta: -1, style_idx });
	}
	events.sort_unstable_by_key(|e| e.position);
	let mut active = [0i32; 3];
	let mut segments: Vec<FormatSegment> = Vec::new();
	// The segment currently being extended, if the active style set is non-empty.
	let mut open: Option<FormatSegment> = None;
	let mut idx = 0;
	while idx < events.len() {
		let position = events[idx].position;
		while idx < events.len() && events[idx].position == position {
			active[events[idx].style_idx] += events[idx].delta;
			idx += 1;
		}
		let (bold, italic, underline) = (active[0] > 0, active[1] > 0, active[2] > 0);
		let same_style = open.is_some_and(|seg| seg.bold == bold && seg.italic == italic && seg.underline == underline);
		if same_style {
			// Style unchanged across this boundary: keep extending the open segment
			// instead of splitting it into an adjacent duplicate.
			open.as_mut().expect("same_style implies open is Some").end = position;
		} else {
			if let Some(mut seg) = open.take() {
				seg.end = position;
				if seg.bold || seg.italic || seg.underline {
					segments.push(seg);
				}
			}
			if bold || italic || underline {
				open = Some(FormatSegment { start: position, end: position, bold, italic, underline });
			}
		}
	}
	if let Some(seg) = open
		&& (seg.bold || seg.italic || seg.underline)
	{
		segments.push(seg);
	}
	segments
}

#[cfg(test)]
mod tests {
	use paperback_core::{document::MarkerType, session::LineMarker};

	use super::{FormatSegment, merge_formatting_markers};

	fn marker(mtype: MarkerType, position: i64, length: i64) -> LineMarker {
		LineMarker { mtype, position, text: String::new(), reference: String::new(), level: 0, length }
	}

	#[test]
	fn no_markers_yields_no_segments() {
		assert_eq!(merge_formatting_markers(&[]), Vec::new());
	}

	#[test]
	fn zero_length_markers_are_ignored() {
		let markers = [marker(MarkerType::Bold, 5, 0)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn non_format_markers_are_ignored() {
		let markers = [marker(MarkerType::Heading1, 0, 10), marker(MarkerType::Link, 2, 3)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn single_bold_marker_produces_one_segment() {
		let markers = [marker(MarkerType::Bold, 0, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 4, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn overlapping_bold_and_italic_keep_both_on_the_intersection() {
		// Bold over [0,10), italic over [4,7): the middle run must carry both.
		let markers = [marker(MarkerType::Bold, 0, 10), marker(MarkerType::Italic, 4, 3)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![
				FormatSegment { start: 0, end: 4, bold: true, italic: false, underline: false },
				FormatSegment { start: 4, end: 7, bold: true, italic: true, underline: false },
				FormatSegment { start: 7, end: 10, bold: true, italic: false, underline: false },
			]
		);
	}

	#[test]
	fn adjacent_identical_segments_are_coalesced() {
		let markers = [marker(MarkerType::Bold, 0, 4), marker(MarkerType::Bold, 4, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 8, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn all_three_styles_can_stack() {
		let markers =
			[marker(MarkerType::Bold, 0, 6), marker(MarkerType::Italic, 0, 6), marker(MarkerType::Underline, 0, 6)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 6, bold: true, italic: true, underline: true }]
		);
	}
}
