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
}
