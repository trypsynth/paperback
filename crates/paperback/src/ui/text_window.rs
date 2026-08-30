//! The document-absolute <-> currently-loaded-window translation for a huge document's text
//! control. See `paperback_core::session::window` for the content-slicing half of this.
//!
//! `text_ctrl` never holds more than `TARGET_WINDOW_SIZE` display units of a document at once
//! for huge documents (`RichEdit`'s resize/relayout cost scales with how far into the *loaded*
//! content the caret sits, independent of document size - so bounding what's loaded bounds
//! that cost). Everything else in the app keeps working in document-absolute positions
//! (bookmarks, saved position, navigation history, `Marker.position` are all untouched); only
//! `text_ctrl`'s own position calls need translating through a `TextWindow`.

/// Target size of a loaded window, in display units. A starting hypothesis, not a measured
/// optimum - the original perf problem scaled roughly linearly with loaded-content position
/// (30-90ms at the very start of a 16.5M-char document, 13s at 50% in, 27s at the end), so a
/// 500K window's worst case should land in the tens-of-ms range. Re-tune against a real
/// maximize/restore benchmark once this is in use.
const TARGET_WINDOW_SIZE: i64 = 500_000;

/// How close to a loaded edge the caret has to get before a reload is triggered. A quarter of
/// the window size, so a single caret-right press at the boundary doesn't cause a reload on
/// literally every keystroke - the new window is recentered with headroom on both sides.
const RELOAD_MARGIN: i64 = TARGET_WINDOW_SIZE / 4;

/// How much text a single forward extension appends. Comfortably wider than
/// [`RELOAD_MARGIN`] so a steady forward read crosses the trigger once per chunk rather
/// than appending on nearly every tick.
const EXTEND_CHUNK: i64 = TARGET_WINDOW_SIZE / 2;

/// Below this document length, just load the whole thing as one window - identical to the
/// app's behavior before windowing existed, so ordinary documents are completely unaffected by
/// this feature (new-path risk is isolated to documents that actually need it).
const WHOLE_DOCUMENT_THRESHOLD: i64 = TARGET_WINDOW_SIZE + TARGET_WINDOW_SIZE / 2;

/// The document-absolute bounds of whatever's currently loaded into a tab's `text_ctrl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextWindow {
	start: i64,
	end: i64,
}

impl TextWindow {
	pub const fn new(start: i64, end: i64) -> Self {
		Self { start, end }
	}

	/// A window covering the entire document - the pre-windowing behavior, used for documents
	/// under `WHOLE_DOCUMENT_THRESHOLD` and as the fallback when a window can't be computed.
	pub const fn whole(doc_len: i64) -> Self {
		Self { start: 0, end: doc_len }
	}

	#[cfg(target_os = "windows")]
	pub const fn start(&self) -> i64 {
		self.start
	}

	#[cfg(target_os = "windows")]
	pub const fn end(&self) -> i64 {
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
	pub const fn wants_extension_for(&self, doc_pos: i64, doc_len: i64) -> bool {
		self.end < doc_len && doc_pos >= self.start && self.end - doc_pos < RELOAD_MARGIN
	}

	/// Whether this window has to be rebuilt around `doc_pos`, discarding what is loaded.
	///
	/// Compaction is the *destructive* half: it moves `start`, which shifts every offset in
	/// the control and silently invalidates the position a Say-All is reading from. Only two
	/// things justify it from the caret's own movement - the caret leaving the loaded range
	/// outright (nothing can be rendered otherwise), and running out of text *behind* the
	/// caret, which cannot happen to a reader that only ever moves forward.
	pub const fn needs_compaction_for(&self, doc_pos: i64, _doc_len: i64) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		self.start > 0 && doc_pos - self.start < RELOAD_MARGIN
	}

	/// The `[start, end)` range a forward extension should ask the session for.
	///
	/// Starts exactly at the current `end`, which is always a paragraph boundary (the session
	/// snapped it there when it produced this window), and re-snapping a boundary is a no-op -
	/// so the appended slice abuts the loaded text with no gap and no repeated paragraph.
	pub fn extend_bounds(&self, doc_len: i64) -> (i64, i64) {
		(self.end, (self.end + EXTEND_CHUNK).min(doc_len))
	}

	/// Moves the loaded end forward after an append. Never moves `start`.
	pub const fn extend_end_to(&mut self, end: i64) {
		if end > self.end {
			self.end = end;
		}
	}

	/// How much text is currently loaded.
	pub const fn loaded_len(&self) -> i64 {
		self.end - self.start
	}

	/// Whether this window already covers the whole document, given its current length (which
	/// the caller re-reads fresh from the session rather than this type caching a stale copy -
	/// see the doc comment on `needs_reload_for`).
	#[allow(dead_code)]
	pub const fn is_whole_document(&self, doc_len: i64) -> bool {
		self.start <= 0 && self.end >= doc_len
	}

	/// Translates a document-absolute position into `text_ctrl`'s own coordinate space,
	/// clamped into this window's bounds (a position outside the window has no valid
	/// ctrl-local representation until the window is reloaded - callers must check
	/// `needs_reload_for` first if that matters).
	pub fn to_local(self, doc_pos: i64) -> i64 {
		(doc_pos - self.start).clamp(0, self.end - self.start)
	}

	/// Translates a `text_ctrl`-local position back to document-absolute.
	pub const fn to_doc(self, local_pos: i64) -> i64 {
		local_pos + self.start
	}

	/// Whether reaching `doc_pos` requires loading a different window first: either it's
	/// outside `[start, end]` outright, or it's within `RELOAD_MARGIN` of an edge that isn't
	/// already the actual start/end of the document (no point reloading to "get more headroom"
	/// past an edge that can't move). `doc_len` is always read fresh from the session by the
	/// caller rather than cached on this type, since the only time it can go stale is a
	/// reparse, which already rebuilds the whole `TextWindow` from scratch.
	pub const fn needs_reload_for(&self, doc_pos: i64, doc_len: i64) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		let near_start = self.start > 0 && doc_pos - self.start < RELOAD_MARGIN;
		let near_end = self.end < doc_len && doc_len - doc_pos > 0 && self.end - doc_pos < RELOAD_MARGIN;
		near_start || near_end
	}
}

/// How large the loaded window may grow before a relayout is worth compacting for.
///
/// Twice the target, so a read that has crossed one or two extension boundaries is left alone
/// and only a genuinely long one pays for a rebuild.
pub const fn compaction_threshold() -> i64 {
	TARGET_WINDOW_SIZE * 2
}

/// Whether a document of `doc_len` display units should just be loaded whole rather than
/// windowed.
pub const fn should_use_whole_document(doc_len: i64) -> bool {
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
pub fn target_window_bounds(doc_pos: i64, doc_len: i64) -> (i64, i64) {
	let half = TARGET_WINDOW_SIZE / 2;
	let raw_end = (doc_pos + half).min(doc_len).max(TARGET_WINDOW_SIZE.min(doc_len));
	let raw_start = (raw_end - TARGET_WINDOW_SIZE).max(0);
	(raw_start, raw_end)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_local_and_to_doc_round_trip_within_bounds() {
		let window = TextWindow::new(1000, 2000);
		assert_eq!(window.to_local(1500), 500);
		assert_eq!(window.to_doc(500), 1500);
	}

	#[test]
	fn to_local_clamps_outside_the_window() {
		let window = TextWindow::new(1000, 2000);
		assert_eq!(window.to_local(0), 0);
		assert_eq!(window.to_local(5000), 1000);
	}

	#[test]
	fn is_whole_document_true_only_when_covering_everything() {
		assert!(TextWindow::whole(10_000).is_whole_document(10_000));
		assert!(!TextWindow::new(0, 5000).is_whole_document(10_000));
		assert!(!TextWindow::new(100, 10_000).is_whole_document(10_000));
	}

	#[test]
	fn needs_reload_for_outside_the_window() {
		let window = TextWindow::new(1000, 2000);
		assert!(window.needs_reload_for(500, 10_000));
		assert!(window.needs_reload_for(2500, 10_000));
	}

	#[test]
	fn needs_reload_for_near_an_edge_that_can_still_move() {
		// A window sized like a real one (RELOAD_MARGIN is a quarter of TARGET_WINDOW_SIZE),
		// with room on both sides for it to move.
		let window = TextWindow::new(1_000_000, 1_000_000 + TARGET_WINDOW_SIZE);
		assert!(window.needs_reload_for(window.start + RELOAD_MARGIN - 1, 10_000_000));
		assert!(window.needs_reload_for(window.end - RELOAD_MARGIN + 1, 10_000_000));
		let middle = window.start + TARGET_WINDOW_SIZE / 2;
		assert!(!window.needs_reload_for(middle, 10_000_000)); // comfortably in the middle
	}

	#[test]
	fn needs_reload_for_is_false_near_an_edge_that_is_the_real_document_boundary() {
		let window = TextWindow::new(0, TARGET_WINDOW_SIZE);
		// Near the start, but start == 0 (the real document start) so there's nowhere to go.
		assert!(!window.needs_reload_for(10, 10_000_000));
		let window = TextWindow::new(10_000_000 - TARGET_WINDOW_SIZE, 10_000_000);
		// Near the end, but end == doc_len (the real document end).
		assert!(!window.needs_reload_for(10_000_000 - 10, 10_000_000));
	}

	#[test]
	fn should_use_whole_document_below_threshold_only() {
		assert!(should_use_whole_document(1000));
		assert!(!should_use_whole_document(10_000_000));
	}

	#[test]
	fn target_window_bounds_centers_with_headroom_and_clamps_to_document() {
		let (start, end) = target_window_bounds(1_000_000, 10_000_000);
		assert_eq!(start, 1_000_000 - TARGET_WINDOW_SIZE / 2);
		assert_eq!(end, start + TARGET_WINDOW_SIZE);
		// Near the very start: can't go negative.
		let (start, end) = target_window_bounds(100, 10_000_000);
		assert_eq!(start, 0);
		assert_eq!(end, TARGET_WINDOW_SIZE);
		// Near the very end: end clamps to the document length, and the window slides back
		// rather than shrinking.
		let (start, end) = target_window_bounds(9_999_000, 10_000_000);
		assert_eq!(end, 10_000_000);
		assert_eq!(start, 10_000_000 - TARGET_WINDOW_SIZE);
		// At the very last character: same, and the caret is far enough from the (movable)
		// start edge that landing there doesn't immediately ask for another reload.
		let (start, end) = target_window_bounds(10_000_000, 10_000_000);
		assert_eq!((start, end), (10_000_000 - TARGET_WINDOW_SIZE, 10_000_000));
		assert!(!TextWindow::new(start, end).needs_reload_for(10_000_000, 10_000_000));
		// A document shorter than a full window is covered whole, not slid past its start.
		assert_eq!(target_window_bounds(300, 1000), (0, 1000));
	}
	// Extension is the operation a Say-All can survive, so it must be what a forward-moving
	// caret triggers - never the rebuild that moves `start` and invalidates the reader's offsets.
	#[test]
	fn a_caret_approaching_the_loaded_end_extends_rather_than_compacts() {
		let window = TextWindow::new(1_000_000, 1_000_000 + TARGET_WINDOW_SIZE);
		let near_end = window.end - RELOAD_MARGIN + 1;
		assert!(window.wants_extension_for(near_end, 10_000_000));
		assert!(!window.needs_compaction_for(near_end, 10_000_000));
	}

	#[test]
	fn a_caret_in_the_middle_wants_nothing() {
		let window = TextWindow::new(1_000_000, 1_000_000 + TARGET_WINDOW_SIZE);
		let middle = window.start + TARGET_WINDOW_SIZE / 2;
		assert!(!window.wants_extension_for(middle, 10_000_000));
		assert!(!window.needs_compaction_for(middle, 10_000_000));
	}

	// Reading backwards off the front is the case compaction still exists for. A Say-All cannot
	// reach it, because it only ever moves forward.
	#[test]
	fn a_caret_approaching_a_movable_start_compacts() {
		let window = TextWindow::new(1_000_000, 1_000_000 + TARGET_WINDOW_SIZE);
		assert!(window.needs_compaction_for(window.start + RELOAD_MARGIN - 1, 10_000_000));
	}

	#[test]
	fn nothing_is_wanted_at_the_real_end_of_the_document() {
		let doc_len = 10_000_000;
		let window = TextWindow::new(doc_len - TARGET_WINDOW_SIZE, doc_len);
		assert!(!window.wants_extension_for(doc_len - 10, doc_len));
		assert!(!window.needs_compaction_for(doc_len - 10, doc_len));
	}

	// The appended range has to begin exactly where the loaded one ends: a gap would drop text
	// and an overlap would repeat a paragraph, and either would shift every later offset.
	#[test]
	fn extension_starts_exactly_at_the_loaded_end() {
		let window = TextWindow::new(1_000_000, 1_500_000);
		let (from, to) = window.extend_bounds(10_000_000);
		assert_eq!(from, 1_500_000);
		assert_eq!(to, 1_500_000 + EXTEND_CHUNK);
	}

	#[test]
	fn extension_stops_at_the_end_of_the_document() {
		let window = TextWindow::new(0, 900_000);
		assert_eq!(window.extend_bounds(1_000_000), (900_000, 1_000_000));
	}

	// Extending must only ever move `end`; `start` moving is what breaks a reader.
	#[test]
	fn extending_never_moves_the_start() {
		let mut window = TextWindow::new(1_000_000, 1_500_000);
		window.extend_end_to(1_750_000);
		assert_eq!(window.start, 1_000_000);
		assert_eq!(window.end, 1_750_000);
		assert_eq!(window.loaded_len(), 750_000);
		// An out-of-order or repeated append must not shrink what is loaded.
		window.extend_end_to(1_600_000);
		assert_eq!(window.end, 1_750_000);
	}

	// A window that grew across several extensions is what a resize has to pay for.
	#[test]
	fn compaction_threshold_tolerates_a_couple_of_extensions() {
		let one = TextWindow::new(0, TARGET_WINDOW_SIZE + EXTEND_CHUNK);
		assert!(one.loaded_len() <= compaction_threshold());
		let many = TextWindow::new(0, TARGET_WINDOW_SIZE + EXTEND_CHUNK * 6);
		assert!(many.loaded_len() > compaction_threshold());
	}
}
