use paperback_core::document::DocPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FormatSegment {
	pub start: DocPos,
	pub end: DocPos,
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
}

/// wxMSW's `wxTextCtrl::SetStyle` rewrites the whole font for a range (unset fields become Arial 10pt), so overlapping markers applied one at a time reset the face and clobber each other.
/// A sweep over +1/-1 events, because rescanning every marker at every boundary took seconds on books with tens of thousands of spans.
pub fn merge_formatting_markers(markers: &[paperback_core::session::LineMarker]) -> Vec<FormatSegment> {
	use paperback_core::document::MarkerType;
	#[derive(Clone, Copy)]
	struct Event {
		position: DocPos,
		delta: i32,
		style_idx: usize,
	}
	let mut events: Vec<Event> = Vec::new();
	for m in markers {
		if m.length.is_zero() {
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
	use paperback_core::{
		document::{DocLen, DocPos, MarkerType},
		session::LineMarker,
	};

	use super::{FormatSegment, merge_formatting_markers};

	fn marker(mtype: MarkerType, position: usize, length: usize) -> LineMarker {
		LineMarker {
			mtype,
			position: DocPos::from_raw(position),
			text: String::new(),
			reference: String::new(),
			level: 0,
			length: DocLen::from_raw(length),
		}
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
			vec![FormatSegment {
				start: DocPos::from_raw(0),
				end: DocPos::from_raw(4),
				bold: true,
				italic: false,
				underline: false
			}]
		);
	}

	#[test]
	fn overlapping_bold_and_italic_keep_both_on_the_intersection() {
		let markers = [marker(MarkerType::Bold, 0, 10), marker(MarkerType::Italic, 4, 3)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![
				FormatSegment {
					start: DocPos::from_raw(0),
					end: DocPos::from_raw(4),
					bold: true,
					italic: false,
					underline: false
				},
				FormatSegment {
					start: DocPos::from_raw(4),
					end: DocPos::from_raw(7),
					bold: true,
					italic: true,
					underline: false
				},
				FormatSegment {
					start: DocPos::from_raw(7),
					end: DocPos::from_raw(10),
					bold: true,
					italic: false,
					underline: false
				},
			]
		);
	}

	#[test]
	fn adjacent_identical_segments_are_coalesced() {
		let markers = [marker(MarkerType::Bold, 0, 4), marker(MarkerType::Bold, 4, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment {
				start: DocPos::from_raw(0),
				end: DocPos::from_raw(8),
				bold: true,
				italic: false,
				underline: false
			}]
		);
	}

	#[test]
	fn all_three_styles_can_stack() {
		let markers =
			[marker(MarkerType::Bold, 0, 6), marker(MarkerType::Italic, 0, 6), marker(MarkerType::Underline, 0, 6)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment {
				start: DocPos::from_raw(0),
				end: DocPos::from_raw(6),
				bold: true,
				italic: true,
				underline: true
			}]
		);
	}
}
