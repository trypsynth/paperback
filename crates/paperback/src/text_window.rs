//! The document-absolute <-> currently-loaded-window translation for a huge document's text
//! control. See `paperback_core::session::window` for the content-slicing half of this.
//!
//! `text_ctrl` never holds more than `TARGET_WINDOW_SIZE` positions of a document at once
//! for huge documents (`RichEdit`'s resize/relayout cost scales with how far into the *loaded*
//! content the caret sits, independent of document size - so bounding what's loaded bounds
//! that cost). Everything else in the app keeps working in document-absolute positions
//! (bookmarks, saved position, navigation history, `Marker.position` are all untouched); only
//! `text_ctrl`'s own position calls need translating through a `TextWindow`.

use paperback_core::{
	document::{DocLen, DocPos},
	session::WindowSlice,
};

/// Target size of a loaded window. A starting hypothesis, not a measured
/// optimum - the original perf problem scaled roughly linearly with loaded-content position
/// (30-90ms at the very start of a 16.5M-char document, 13s at 50% in, 27s at the end), so a
/// 500K window's worst case should land in the tens-of-ms range. Re-tune against a real
/// maximize/restore benchmark once this is in use.
const TARGET_WINDOW_SIZE: DocLen = paperback_core::WINDOW_DISPLAY_LEN;

/// How close to a loaded edge the caret has to get before a reload is triggered. A quarter of
/// the window size, so a single caret-right press at the boundary doesn't cause a reload on
/// literally every keystroke - the new window is recentered with headroom on both sides.
const RELOAD_MARGIN: DocLen = DocLen::from_raw(TARGET_WINDOW_SIZE.get() / 4);

/// How much text a single forward extension appends. Comfortably wider than
/// [`RELOAD_MARGIN`] so a steady forward read crosses the trigger once per chunk rather
/// than appending on nearly every tick.
const EXTEND_CHUNK: DocLen = DocLen::from_raw(TARGET_WINDOW_SIZE.get() / 2);

/// Below this document length, just load the whole thing as one window - identical to the
/// app's behavior before windowing existed, so ordinary documents are completely unaffected by
/// this feature (new-path risk is isolated to documents that actually need it). Shared with
/// the web view, which has to agree about what counts as a whole document.
const WHOLE_DOCUMENT_THRESHOLD: DocLen = paperback_core::WHOLE_DOCUMENT_DISPLAY_LEN;

/// A position in a tab's `text_ctrl`: counted from the start of what is loaded into it, in the control's own units. Only [`TextWindow`] turns one into a document position or back, so a position of one kind can't be passed where the other is meant (#1062).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CtrlPos(i64);

impl CtrlPos {
	pub const ZERO: Self = Self(0);

	/// Wraps a position the control or a native API on it gave out.
	pub const fn from_raw(raw: i64) -> Self {
		Self(raw)
	}

	pub const fn get(self) -> i64 {
		self.0
	}
}

/// Moves along the control by a count of its units, such as a column within a line it measured.
impl std::ops::Add<i64> for CtrlPos {
	type Output = Self;

	fn add(self, units: i64) -> Self {
		Self(self.0 + units)
	}
}

/// The document-absolute bounds of whatever's currently loaded into a tab's `text_ctrl`, and what it takes to translate a position between the two.
///
/// A document counts one position for every char, on every platform (#1062). The control on Windows and macOS counts UTF-16 code units instead, two for every char outside the Basic Multilingual Plane, so the window notes where each of those sits in what it holds. Most text has none, and then a control position is just an offset from `start`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextWindow {
	start: DocPos,
	end: DocPos,
	/// Each loaded char that takes two of the control's units, as its offset from `start` in positions and in the control's units.
	wide: Vec<(DocLen, i64)>,
}

impl TextWindow {
	#[cfg(test)]
	pub const fn new(start: DocPos, end: DocPos) -> Self {
		Self { start, end, wide: Vec::new() }
	}

	/// A window holding `slice`, as it is loaded into the control.
	pub fn of(slice: &WindowSlice) -> Self {
		let mut window = Self { start: slice.start, end: slice.start, wide: Vec::new() };
		window.extend_with(slice);
		window
	}

	/// A window covering the entire document - the pre-windowing behavior, used for documents
	/// under `WHOLE_DOCUMENT_THRESHOLD` and as the fallback when a window can't be computed.
	#[cfg(test)]
	pub const fn whole(doc_len: DocLen) -> Self {
		Self { start: DocPos::ZERO, end: doc_len.as_pos(), wide: Vec::new() }
	}

	pub const fn start(&self) -> DocPos {
		self.start
	}

	pub const fn end(&self) -> DocPos {
		self.end
	}

	/// Whether more text should be appended to the end of this window to stay ahead of a
	/// forward reader: the caret is within [`RELOAD_MARGIN`] of the loaded end and there is
	/// more document past it.
	///
	/// Extension is the *safe* half of window maintenance. Appending leaves every existing
	/// offset into the control pointing at the same text, so a screen reader part-way through
	/// a Say-All - which holds its own offsets, advances them itself, and has no way to learn
	/// the text changed - simply finds more text ahead of it and reads on.
	pub fn wants_extension_for(&self, doc_pos: DocPos, doc_len: DocLen) -> bool {
		self.end < doc_len.as_pos() && doc_pos >= self.start && self.end.since(doc_pos) < RELOAD_MARGIN
	}

	/// Whether this window has to be rebuilt around `doc_pos`, discarding what is loaded.
	///
	/// Compaction is the *destructive* half: it moves `start`, which shifts every offset in
	/// the control and silently invalidates the position a Say-All is reading from. Only two
	/// things justify it from the caret's own movement - the caret leaving the loaded range
	/// outright (nothing can be rendered otherwise), and running out of text *behind* the
	/// caret, which cannot happen to a reader that only ever moves forward.
	pub fn needs_compaction_for(&self, doc_pos: DocPos, _doc_len: DocLen) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		self.start > DocPos::ZERO && doc_pos - self.start < RELOAD_MARGIN
	}

	/// The `[start, end)` range a forward extension should ask the session for.
	///
	/// Starts exactly at the current `end`, which is always a paragraph boundary (the session
	/// snapped it there when it produced this window), and re-snapping a boundary is a no-op -
	/// so the appended slice abuts the loaded text with no gap and no repeated paragraph.
	pub fn extend_bounds(&self, doc_len: DocLen) -> (DocPos, DocPos) {
		(self.end, (self.end + EXTEND_CHUNK).min(doc_len.as_pos()))
	}

	/// Takes in `slice`, which has just been appended to the control and starts where this window ends. Never moves `start`.
	pub fn extend_with(&mut self, slice: &WindowSlice) {
		if slice.end <= self.end {
			return;
		}
		if cfg!(any(target_os = "windows", target_os = "macos")) {
			let (mut position, mut units) = (slice.start.since(self.start), self.to_local(slice.start).0);
			for c in slice.text.chars() {
				if c.len_utf16() == 2 {
					self.wide.push((position, units));
				}
				position += DocLen::ONE;
				units += i64::try_from(c.len_utf16()).unwrap_or(1);
			}
		}
		self.extend_end_to(slice.end);
	}

	/// Moves the loaded end forward after an append. Never moves `start`.
	fn extend_end_to(&mut self, end: DocPos) {
		if end > self.end {
			self.end = end;
		}
	}

	/// How much text is currently loaded.
	pub fn loaded_len(&self) -> DocLen {
		self.end - self.start
	}

	/// Whether this window already covers the whole document, given its current length (which
	/// the caller re-reads fresh from the session rather than this type caching a stale copy -
	/// see the doc comment on `needs_reload_for`).
	pub fn is_whole_document(&self, doc_len: DocLen) -> bool {
		self.start == DocPos::ZERO && self.end >= doc_len.as_pos()
	}

	/// Translates a document-absolute position into `text_ctrl`'s own coordinate space,
	/// clamped into this window's bounds (a position outside the window has no valid
	/// ctrl-local representation until the window is reloaded - callers must check
	/// `needs_reload_for` first if that matters).
	pub fn to_local(&self, doc_pos: DocPos) -> CtrlPos {
		let position = doc_pos.since(self.start).min(self.loaded_len());
		let wide_before = self.wide.partition_point(|&(at, _)| at < position);
		CtrlPos(i64::try_from(position.get() + wide_before).unwrap_or(i64::MAX))
	}

	/// Translates a `text_ctrl`-local position back to document-absolute. A position between the two units of one char, where no caret can sit, rounds forward to the char after it, as the document does.
	pub fn to_doc(&self, local_pos: CtrlPos) -> DocPos {
		let passed = self.wide.partition_point(|&(_, at)| at + 2 <= local_pos.0);
		self.start + DocLen::from_raw(usize::try_from(local_pos.0).unwrap_or(0).saturating_sub(passed))
	}

	/// Whether reaching `doc_pos` requires loading a different window first: either it's
	/// outside `[start, end]` outright, or it's within `RELOAD_MARGIN` of an edge that isn't
	/// already the actual start/end of the document (no point reloading to "get more headroom"
	/// past an edge that can't move). `doc_len` is always read fresh from the session by the
	/// caller rather than cached on this type, since the only time it can go stale is a
	/// reparse, which already rebuilds the whole `TextWindow` from scratch.
	pub fn needs_reload_for(&self, doc_pos: DocPos, doc_len: DocLen) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		let near_start = self.start > DocPos::ZERO && doc_pos - self.start < RELOAD_MARGIN;
		let near_end = self.end < doc_len.as_pos() && doc_len.as_pos() > doc_pos && self.end - doc_pos < RELOAD_MARGIN;
		near_start || near_end
	}
}

/// How large the loaded window may grow before a relayout is worth compacting for.
///
/// Twice the target, so a read that has crossed one or two extension boundaries is left alone
/// and only a genuinely long one pays for a rebuild.
pub const fn compaction_threshold() -> DocLen {
	DocLen::from_raw(TARGET_WINDOW_SIZE.get() * 2)
}

/// Whether a document `doc_len` long should just be loaded whole rather than
/// windowed.
pub fn should_use_whole_document(doc_len: DocLen) -> bool {
	doc_len <= WHOLE_DOCUMENT_THRESHOLD
}

/// The raw `[start, end)` a fresh window should request (before `DocumentSession::get_window`'s
/// paragraph-boundary snapping) to contain `doc_pos` with headroom on both sides.
///
/// Stays `TARGET_WINDOW_SIZE` wide even when `doc_pos` sits within half a window of either end
/// of the document, sliding the window inward rather than truncating it: a jump to the very
/// last character (Ctrl+End) would otherwise load only the trailing half-window, leaving the
/// caret `RELOAD_MARGIN` from a start edge that can still move and so re-triggering a reload on
/// the first Up/Page Up afterwards.
pub fn target_window_bounds(doc_pos: DocPos, doc_len: DocLen) -> (DocPos, DocPos) {
	let half = DocLen::from_raw(TARGET_WINDOW_SIZE.get() / 2);
	let raw_end = (doc_pos + half).min(doc_len.as_pos()).max(TARGET_WINDOW_SIZE.min(doc_len).as_pos());
	(raw_end.saturating_sub(TARGET_WINDOW_SIZE), raw_end)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn p(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	fn l(raw: usize) -> DocLen {
		DocLen::from_raw(raw)
	}

	#[test]
	fn to_local_and_to_doc_round_trip_within_bounds() {
		let window = TextWindow::new(p(1000), p(2000));
		assert_eq!(window.to_local(p(1500)), CtrlPos(500));
		assert_eq!(window.to_doc(CtrlPos(500)), p(1500));
	}

	fn slice(start: usize, text: &str) -> WindowSlice {
		let start = p(start);
		let end = start + DocLen::of(text);
		WindowSlice { start, end, text: text.to_string(), markers: Vec::new(), paragraphs: Vec::new() }
	}

	/// The control units at each of five positions: on Windows and macOS an emoji before a position takes two, elsewhere one, as the document counts.
	fn units(utf16: [i64; 5], scalars: [i64; 5]) -> [i64; 5] {
		if cfg!(any(target_os = "windows", target_os = "macos")) { utf16 } else { scalars }
	}

	#[test]
	fn a_char_outside_the_bmp_is_one_position_and_as_many_units_as_the_control_counts() {
		// "a😀b😀c" from position 100: a=100, 😀=101, b=102, 😀=103, c=104.
		let window = TextWindow::of(&slice(100, "a\u{1F600}b\u{1F600}c"));
		let expected = units([0, 1, 3, 4, 6], [0, 1, 2, 3, 4]);
		for (position, units) in (100..105).map(p).zip(expected) {
			assert_eq!(window.to_local(position), CtrlPos(units), "position {position}");
			assert_eq!(window.to_doc(CtrlPos(units)), position, "units {units}");
		}
		assert_eq!(window.to_local(p(105)), CtrlPos(units([7, 0, 0, 0, 0], [5, 0, 0, 0, 0])[0]));
		if cfg!(any(target_os = "windows", target_os = "macos")) {
			// Between the two units of an emoji, where no caret can sit: the char after it.
			assert_eq!(window.to_doc(CtrlPos(2)), p(102));
			assert_eq!(window.to_doc(CtrlPos(5)), p(104));
		}
	}

	/// Text appended to a window is placed after everything already loaded, emoji included.
	#[test]
	fn an_extended_window_counts_what_was_loaded_before() {
		let mut window = TextWindow::of(&slice(0, "a\u{1F600}"));
		window.extend_with(&slice(2, "b\u{1F600}c"));
		let expected = units([0, 1, 3, 4, 6], [0, 1, 2, 3, 4]);
		for (position, units) in (0..5).map(p).zip(expected) {
			assert_eq!(window.to_local(position), CtrlPos(units), "position {position}");
			assert_eq!(window.to_doc(CtrlPos(units)), position, "units {units}");
		}
		assert_eq!(window, TextWindow::of(&slice(0, "a\u{1F600}b\u{1F600}c")));
	}

	#[test]
	fn to_local_clamps_outside_the_window() {
		let window = TextWindow::new(p(1000), p(2000));
		assert_eq!(window.to_local(p(0)), CtrlPos(0));
		assert_eq!(window.to_local(p(5000)), CtrlPos(1000));
	}

	#[test]
	fn is_whole_document_true_only_when_covering_everything() {
		assert!(TextWindow::whole(l(10_000)).is_whole_document(l(10_000)));
		assert!(!TextWindow::new(p(0), p(5000)).is_whole_document(l(10_000)));
		assert!(!TextWindow::new(p(100), p(10_000)).is_whole_document(l(10_000)));
	}

	#[test]
	fn needs_reload_for_outside_the_window() {
		let window = TextWindow::new(p(1000), p(2000));
		assert!(window.needs_reload_for(p(500), l(10_000)));
		assert!(window.needs_reload_for(p(2500), l(10_000)));
	}

	#[test]
	fn needs_reload_for_near_an_edge_that_can_still_move() {
		// A window sized like a real one (RELOAD_MARGIN is a quarter of TARGET_WINDOW_SIZE),
		// with room on both sides for it to move.
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + TARGET_WINDOW_SIZE);
		assert!(window.needs_reload_for(window.start + RELOAD_MARGIN - DocLen::ONE, l(10_000_000)));
		assert!(window.needs_reload_for(window.end - RELOAD_MARGIN + DocLen::ONE, l(10_000_000)));
		let middle = window.start + l(TARGET_WINDOW_SIZE.get() / 2);
		assert!(!window.needs_reload_for(middle, l(10_000_000))); // comfortably in the middle
	}

	#[test]
	fn needs_reload_for_is_false_near_an_edge_that_is_the_real_document_boundary() {
		let window = TextWindow::new(p(0), TARGET_WINDOW_SIZE.as_pos());
		// Near the start, but start == 0 (the real document start) so there's nowhere to go.
		assert!(!window.needs_reload_for(p(10), l(10_000_000)));
		let window = TextWindow::new(p(10_000_000) - TARGET_WINDOW_SIZE, p(10_000_000));
		// Near the end, but end == doc_len (the real document end).
		assert!(!window.needs_reload_for(p(10_000_000 - 10), l(10_000_000)));
	}

	#[test]
	fn should_use_whole_document_below_threshold_only() {
		assert!(should_use_whole_document(l(1000)));
		assert!(!should_use_whole_document(l(10_000_000)));
	}

	#[test]
	fn target_window_bounds_centers_with_headroom_and_clamps_to_document() {
		let (start, end) = target_window_bounds(p(1_000_000), l(10_000_000));
		assert_eq!(start, p(1_000_000 - TARGET_WINDOW_SIZE.get() / 2));
		assert_eq!(end, start + TARGET_WINDOW_SIZE);
		// Near the very start: can't go negative.
		let (start, end) = target_window_bounds(p(100), l(10_000_000));
		assert_eq!(start, p(0));
		assert_eq!(end, TARGET_WINDOW_SIZE.as_pos());
		// Near the very end: end clamps to the document length, and the window slides back
		// rather than shrinking.
		let (start, end) = target_window_bounds(p(9_999_000), l(10_000_000));
		assert_eq!(end, p(10_000_000));
		assert_eq!(start, p(10_000_000) - TARGET_WINDOW_SIZE);
		// At the very last character: same, and the caret is far enough from the (movable)
		// start edge that landing there doesn't immediately ask for another reload.
		let (start, end) = target_window_bounds(p(10_000_000), l(10_000_000));
		assert_eq!((start, end), (p(10_000_000) - TARGET_WINDOW_SIZE, p(10_000_000)));
		assert!(!TextWindow::new(start, end).needs_reload_for(p(10_000_000), l(10_000_000)));
		// A document shorter than a full window is covered whole, not slid past its start.
		assert_eq!(target_window_bounds(p(300), l(1000)), (p(0), p(1000)));
	}
	// Extension is the operation a Say-All can survive, so it must be what a forward-moving
	// caret triggers - never the rebuild that moves `start` and invalidates the reader's offsets.
	#[test]
	fn a_caret_approaching_the_loaded_end_extends_rather_than_compacts() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + TARGET_WINDOW_SIZE);
		let near_end = window.end - RELOAD_MARGIN + DocLen::ONE;
		assert!(window.wants_extension_for(near_end, l(10_000_000)));
		assert!(!window.needs_compaction_for(near_end, l(10_000_000)));
	}

	#[test]
	fn a_caret_in_the_middle_wants_nothing() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + TARGET_WINDOW_SIZE);
		let middle = window.start + l(TARGET_WINDOW_SIZE.get() / 2);
		assert!(!window.wants_extension_for(middle, l(10_000_000)));
		assert!(!window.needs_compaction_for(middle, l(10_000_000)));
	}

	// Reading backwards off the front is the case compaction still exists for. A Say-All cannot
	// reach it, because it only ever moves forward.
	#[test]
	fn a_caret_approaching_a_movable_start_compacts() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + TARGET_WINDOW_SIZE);
		assert!(window.needs_compaction_for(window.start + RELOAD_MARGIN - DocLen::ONE, l(10_000_000)));
	}

	#[test]
	fn nothing_is_wanted_at_the_real_end_of_the_document() {
		let doc_len = l(10_000_000);
		let window = TextWindow::new(doc_len.as_pos() - TARGET_WINDOW_SIZE, doc_len.as_pos());
		assert!(!window.wants_extension_for(doc_len.as_pos() - l(10), doc_len));
		assert!(!window.needs_compaction_for(doc_len.as_pos() - l(10), doc_len));
	}

	// The appended range has to begin exactly where the loaded one ends: a gap would drop text
	// and an overlap would repeat a paragraph, and either would shift every later offset.
	#[test]
	fn extension_starts_exactly_at_the_loaded_end() {
		let window = TextWindow::new(p(1_000_000), p(1_500_000));
		let (from, to) = window.extend_bounds(l(10_000_000));
		assert_eq!(from, p(1_500_000));
		assert_eq!(to, p(1_500_000) + EXTEND_CHUNK);
	}

	#[test]
	fn extension_stops_at_the_end_of_the_document() {
		let window = TextWindow::new(p(0), p(900_000));
		assert_eq!(window.extend_bounds(l(1_000_000)), (p(900_000), p(1_000_000)));
	}

	// Extending must only ever move `end`; `start` moving is what breaks a reader.
	#[test]
	fn extending_never_moves_the_start() {
		let mut window = TextWindow::new(p(1_000_000), p(1_500_000));
		window.extend_end_to(p(1_750_000));
		assert_eq!(window.start, p(1_000_000));
		assert_eq!(window.end, p(1_750_000));
		assert_eq!(window.loaded_len(), l(750_000));
		// An out-of-order or repeated append must not shrink what is loaded.
		window.extend_end_to(p(1_600_000));
		assert_eq!(window.end, p(1_750_000));
	}

	// A window that grew across several extensions is what a resize has to pay for.
	#[test]
	fn compaction_threshold_tolerates_a_couple_of_extensions() {
		let one = TextWindow::new(p(0), (TARGET_WINDOW_SIZE + EXTEND_CHUNK).as_pos());
		assert!(one.loaded_len() <= compaction_threshold());
		let many = TextWindow::new(p(0), (TARGET_WINDOW_SIZE + l(EXTEND_CHUNK.get() * 6)).as_pos());
		assert!(many.loaded_len() > compaction_threshold());
	}

	// The failure this guards against, reported as a Say-All that restarted from the top of a
	// large file mid-read. A screen reader holds its own offsets into the control and advances
	// them itself, so anything the window pump does on a timer has to leave those offsets meaning
	// the same character. Extension does. Compaction does not, and it is self sustaining: moving
	// the loaded start leaves a stale offset sitting near the new start, which is the very
	// condition to compact again. Driving that policy from a tick walked the window back to the
	// top of the document in twelve ticks, three seconds at the pump's cadence.
	//
	// So the pump extends and nothing else, and this pins that: a reader that only moves forward
	// must find the ground under it unchanged, however long it reads for.
	#[test]
	fn the_pump_never_moves_the_ground_under_a_reader() {
		let doc_len = l(5_000_000);
		let start = p(1_750_000);
		let mut window = TextWindow::new(start, start + TARGET_WINDOW_SIZE);
		// Seeded just inside RELOAD_MARGIN of the loaded start, the position most likely to look
		// like it needs a compaction and so the worst case for a pump that must not do one.
		let mut reader_local = CtrlPos(i64::try_from(RELOAD_MARGIN.get() - 25_000).unwrap());
		for _ in 0..500 {
			let before = window.to_doc(reader_local);
			if window.wants_extension_for(before, doc_len) {
				let (from, to) = window.extend_bounds(doc_len);
				assert_eq!(from, window.end, "an extension must begin exactly at the loaded end");
				window.extend_end_to(to);
			}
			assert_eq!(window.to_doc(CtrlPos::ZERO), start, "the pump must never move the loaded start");
			assert_eq!(window.to_doc(reader_local), before, "a held offset must still mean the same character");
			// The reader speaks a line and advances its own offset.
			reader_local = reader_local + 200;
		}
		// And it stayed ahead of the reader the whole way rather than running out of text.
		assert!(window.to_doc(reader_local) <= window.end, "extension must keep up with the reader");
	}

	// Compaction is still the right answer for a caret that has run out of text behind it; it just
	// may only happen where the user has already stopped any read. This is the case it exists for.
	#[test]
	fn a_caret_that_has_run_out_of_text_behind_it_still_asks_to_compact() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + TARGET_WINDOW_SIZE);
		assert!(window.needs_compaction_for(p(1_000_000) + RELOAD_MARGIN - DocLen::ONE, l(10_000_000)));
	}

	// Why the split exists at all, as a property rather than a comment: the two operations differ
	// exactly in whether an offset someone else is holding still means the same character.
	#[test]
	fn extension_leaves_a_held_offset_alone_and_compaction_moves_it() {
		let doc_len = l(5_000_000);
		let window = TextWindow::new(p(1_750_000), p(1_750_000) + TARGET_WINDOW_SIZE);
		let held_local = CtrlPos(i64::try_from(RELOAD_MARGIN.get() - 25_000).unwrap());
		let before = window.to_doc(held_local);
		let mut extended = window.clone();
		extended.extend_end_to(window.end + EXTEND_CHUNK);
		assert_eq!(extended.to_doc(held_local), before, "extension must leave a held offset meaning the same place");
		let (start, end) = target_window_bounds(before, doc_len);
		let compacted = TextWindow::new(start, end);
		assert_ne!(
			compacted.to_doc(held_local),
			before,
			"compaction moves a held offset, which is why only a user action may trigger it"
		);
	}
}
