use super::{
	DocumentSession, ReadingAheadFfi, ReadingStepFfi, ReadingUnitFfi, SearchOptionsFfi, SegmentDirectionFfi,
	SegmentTypeFfi,
};
use crate::document::{DocLen, DocPos, TocItem};

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	/// One press of the mobile reading bar's previous or next, or the segment under `position` for `Current`.
	#[must_use]
	pub fn reading_step(
		&self,
		position: DocPos,
		unit: ReadingUnitFfi,
		direction: SegmentDirectionFfi,
	) -> ReadingStepFfi {
		let landed = match unit {
			ReadingUnitFfi::Segment { segment_type } => {
				let segment = self.get_text_segment(position, segment_type, direction);
				segment.found.then_some((segment.start_pos, segment.text))
			}
			ReadingUnitFfi::Find { query, options } => self.find_step(position, &query, options, direction),
		};
		let Some((landed_at, text)) = landed else { return ReadingStepFfi::not_found(position) };
		// A previous that lands where it started, at the top of the book, is no move; reading it again would sound like one.
		if landed_at == position && !matches!(direction, SegmentDirectionFfi::Current) {
			return ReadingStepFfi::not_found(position);
		}
		let text = if text.trim().is_empty() { self.section_title_at(landed_at) } else { text };
		ReadingStepFfi { found: true, position: landed_at, text }
	}

	/// What continuous reading speaks around `position`, for the speech engine to prepare ahead; always by paragraph, since a heading or link step's text is only the marker's title.
	#[must_use]
	pub fn reading_ahead(&self, position: DocPos) -> ReadingAheadFfi {
		let paragraph = |at, direction| {
			let segment = self.get_text_segment(at, SegmentTypeFfi::Paragraph, direction);
			(segment.found && !segment.text.trim().is_empty()).then_some(segment)
		};
		let next = paragraph(position, SegmentDirectionFfi::Next);
		let after_next = next.as_ref().and_then(|next| paragraph(next.start_pos, SegmentDirectionFfi::Next));
		let previous =
			paragraph(position, SegmentDirectionFfi::Previous).filter(|previous| previous.start_pos != position);
		ReadingAheadFfi {
			next: next.map(|s| s.text).unwrap_or_default(),
			after_next: after_next.map(|s| s.text).unwrap_or_default(),
			previous: previous.map(|s| s.text).unwrap_or_default(),
		}
	}

	/// The title of the table of contents entry `position` is in: the latest one starting at or before it, and of entries sharing a start the deepest, as it names the most.
	#[must_use]
	pub fn section_title_at(&self, position: DocPos) -> String {
		fn walk<'a>(items: &'a [TocItem], position: DocPos, best: &mut Option<&'a TocItem>) {
			for item in items {
				if item.offset <= position && best.is_none_or(|b| item.offset >= b.offset) {
					*best = Some(item);
				}
				walk(&item.children, position, best);
			}
		}
		let mut best = None;
		walk(&self.handle.document().toc_items, position, &mut best);
		best.map(|item| item.name.clone()).unwrap_or_default()
	}
}

impl DocumentSession {
	/// Lands on the match itself rather than its paragraph's start, so the next search starts past it.
	fn find_step(
		&self,
		position: DocPos,
		query: &str,
		options: SearchOptionsFfi,
		direction: SegmentDirectionFfi,
	) -> Option<(DocPos, String)> {
		let (start, forward) = match direction {
			SegmentDirectionFfi::Current => (position, true),
			// A forward search counts a match at its start position, which would find the current match again.
			SegmentDirectionFfi::Next => (position + DocLen::ONE, true),
			SegmentDirectionFfi::Previous => (position, false),
		};
		let found = self.search_ffi(query.to_string(), start, SearchOptionsFfi { forward, ..options });
		if !found.found {
			return None;
		}
		let paragraph = self.get_text_segment(found.position, SegmentTypeFfi::Paragraph, SegmentDirectionFfi::Current);
		Some((found.position, paragraph.text))
	}
}

impl ReadingStepFfi {
	const fn not_found(position: DocPos) -> Self {
		Self { found: false, position, text: String::new() }
	}
}
