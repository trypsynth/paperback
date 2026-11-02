//! `RichEdit`'s relayout cost scales with how far into the loaded content the caret sits, so a huge document loads a bounded window; everything outside `text_ctrl` stays in document positions.

use paperback_core::{
	document::{DocLen, DocPos},
	session::WindowSlice,
};

/// The window size the tests work in, which is the shipping default. A starting hypothesis, not measured: relayout went from 30-90ms at the start of a 16.5M-char document to 27s at its end.
#[cfg(test)]
pub const DEFAULT_TARGET: DocLen = paperback_core::WINDOW_LEN;

/// Sizes the window around the whole document, so there is never another one to load.
pub const UNLIMITED: DocLen = DocLen::ZERO;

/// The target a stored setting picks. Zero and anything negative mean the whole document at once.
pub fn target_from_char_count(chars: i32) -> DocLen {
	usize::try_from(chars).map_or(UNLIMITED, DocLen::from_raw)
}

/// Only [`TextWindow`] converts to or from a document position, so the two kinds can't be mixed up (#1062).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CtrlPos(i64);

impl CtrlPos {
	#[cfg(any(test, target_os = "windows", target_os = "macos"))]
	pub const ZERO: Self = Self(0);

	pub const fn from_raw(raw: i64) -> Self {
		Self(raw)
	}

	pub const fn get(self) -> i64 {
		self.0
	}
}

impl std::ops::Add<i64> for CtrlPos {
	type Output = Self;

	fn add(self, units: i64) -> Self {
		Self(self.0 + units)
	}
}

/// A document counts one position per char (#1062), but the Windows and macOS controls count UTF-16 units, so the window notes where each two-unit char sits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextWindow {
	start: DocPos,
	end: DocPos,
	/// Offset from `start` in positions and in control units.
	wide: Vec<(DocLen, i64)>,
	/// How much text the window is sized around, as Settings asks; zero means the whole document.
	target: DocLen,
}

impl TextWindow {
	#[cfg(test)]
	pub const fn new(start: DocPos, end: DocPos) -> Self {
		Self { start, end, wide: Vec::new(), target: DEFAULT_TARGET }
	}

	#[cfg(test)]
	pub const fn with_target(start: DocPos, end: DocPos, target: DocLen) -> Self {
		Self { start, end, wide: Vec::new(), target }
	}

	pub fn of(slice: &WindowSlice, target: DocLen) -> Self {
		let mut window = Self { start: slice.start, end: slice.start, wide: Vec::new(), target };
		window.extend_with(slice);
		window
	}

	#[cfg(test)]
	pub const fn whole(doc_len: DocLen) -> Self {
		Self { start: DocPos::ZERO, end: doc_len.as_pos(), wide: Vec::new(), target: UNLIMITED }
	}

	pub const fn start(&self) -> DocPos {
		self.start
	}

	pub const fn end(&self) -> DocPos {
		self.end
	}

	pub const fn target(&self) -> DocLen {
		self.target
	}

	/// Takes effect on the next reload; the text already loaded is left where it is so applying a setting never moves a reader.
	pub fn set_target(&mut self, target: DocLen) {
		self.target = target;
	}

	/// A quarter of the window, so a caret step at the boundary doesn't reload on every keystroke.
	fn reload_margin(&self) -> DocLen {
		DocLen::from_raw(self.target.get() / 4)
	}

	/// Wider than the reload margin so a steady forward read crosses the trigger once per chunk rather than on nearly every tick.
	fn extend_chunk(&self) -> DocLen {
		DocLen::from_raw(self.target.get() / 2)
	}

	/// Appending leaves every offset meaning the same text, so a screen reader mid Say-All, which holds and advances its own offsets, just reads on.
	pub fn wants_extension_for(&self, doc_pos: DocPos, doc_len: DocLen) -> bool {
		self.end < doc_len.as_pos() && doc_pos >= self.start && self.end.since(doc_pos) < self.reload_margin()
	}

	/// Compaction moves `start`, shifting every offset and invalidating a Say-All's position; only the caret leaving the range or running out of text behind it justifies that.
	pub fn needs_compaction_for(&self, doc_pos: DocPos, _doc_len: DocLen) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		self.start > DocPos::ZERO && doc_pos - self.start < self.reload_margin()
	}

	/// `end` is always a paragraph boundary and re-snapping one is a no-op, so the appended slice abuts with no gap or repeated paragraph.
	pub fn extend_bounds(&self, doc_len: DocLen) -> (DocPos, DocPos) {
		(self.end, (self.end + self.extend_chunk()).min(doc_len.as_pos()))
	}

	/// `slice` must start where this window ends.
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

	fn extend_end_to(&mut self, end: DocPos) {
		if end > self.end {
			self.end = end;
		}
	}

	pub fn loaded_len(&self) -> DocLen {
		self.end - self.start
	}

	pub fn is_whole_document(&self, doc_len: DocLen) -> bool {
		self.start == DocPos::ZERO && self.end >= doc_len.as_pos()
	}

	/// Clamped, since a position outside the window has no control position until it reloads.
	pub fn to_local(&self, doc_pos: DocPos) -> CtrlPos {
		let position = doc_pos.since(self.start).min(self.loaded_len());
		let wide_before = self.wide.partition_point(|&(at, _)| at < position);
		CtrlPos(i64::try_from(position.get() + wide_before).unwrap_or(i64::MAX))
	}

	/// A position between the two units of one char rounds forward to the char after it, as the document does.
	pub fn to_doc(&self, local_pos: CtrlPos) -> DocPos {
		let passed = self.wide.partition_point(|&(_, at)| at + 2 <= local_pos.0);
		self.start + DocLen::from_raw(usize::try_from(local_pos.0).unwrap_or(0).saturating_sub(passed))
	}

	/// `doc_len` isn't cached here because it only goes stale on a reparse, which rebuilds the window anyway.
	pub fn needs_reload_for(&self, doc_pos: DocPos, doc_len: DocLen) -> bool {
		if doc_pos < self.start || doc_pos > self.end {
			return true;
		}
		let margin = self.reload_margin();
		let near_start = self.start > DocPos::ZERO && doc_pos - self.start < margin;
		let near_end = self.end < doc_len.as_pos() && doc_len.as_pos() > doc_pos && self.end - doc_pos < margin;
		near_start || near_end
	}

	/// Twice the target, so a read that has crossed one or two extensions is left alone. A whole-document window never compacts, since it is exactly as large as its document.
	pub fn compaction_threshold(&self) -> DocLen {
		if self.target.is_zero() {
			DocLen::from_raw(usize::MAX)
		} else {
			DocLen::from_raw(self.target.get().saturating_mul(2))
		}
	}
}

/// A window and a half: a document only slightly larger than one window loads whole rather than reloading at the first step.
fn whole_document_threshold(target: DocLen) -> DocLen {
	DocLen::from_raw(target.get().saturating_add(target.get() / 2))
}

pub fn should_use_whole_document(doc_len: DocLen, target: DocLen) -> bool {
	target.is_zero() || doc_len <= whole_document_threshold(target)
}

/// Slides inward near a document edge rather than truncating, or Ctrl+End would load half a window and reload again on the first Up.
pub fn target_window_bounds(doc_pos: DocPos, doc_len: DocLen, target: DocLen) -> (DocPos, DocPos) {
	if target.is_zero() {
		return (DocPos::ZERO, doc_len.as_pos());
	}
	let half = DocLen::from_raw(target.get() / 2);
	let raw_end = (doc_pos + half).min(doc_len.as_pos()).max(target.min(doc_len).as_pos());
	(raw_end.saturating_sub(target), raw_end)
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

	fn margin() -> DocLen {
		DocLen::from_raw(DEFAULT_TARGET.get() / 4)
	}

	fn chunk() -> DocLen {
		DocLen::from_raw(DEFAULT_TARGET.get() / 2)
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

	/// Only the Windows and macOS controls count an emoji as two units.
	fn units(utf16: [i64; 5], scalars: [i64; 5]) -> [i64; 5] {
		if cfg!(any(target_os = "windows", target_os = "macos")) { utf16 } else { scalars }
	}

	#[test]
	fn a_char_outside_the_bmp_is_one_position_and_as_many_units_as_the_control_counts() {
		// "a😀b😀c" from position 100: a=100, 😀=101, b=102, 😀=103, c=104.
		let window = TextWindow::of(&slice(100, "a\u{1F600}b\u{1F600}c"), DEFAULT_TARGET);
		let expected = units([0, 1, 3, 4, 6], [0, 1, 2, 3, 4]);
		for (position, units) in (100..105).map(p).zip(expected) {
			assert_eq!(window.to_local(position), CtrlPos(units), "position {position}");
			assert_eq!(window.to_doc(CtrlPos(units)), position, "units {units}");
		}
		assert_eq!(window.to_local(p(105)), CtrlPos(units([7, 0, 0, 0, 0], [5, 0, 0, 0, 0])[0]));
		if cfg!(any(target_os = "windows", target_os = "macos")) {
			assert_eq!(window.to_doc(CtrlPos(2)), p(102));
			assert_eq!(window.to_doc(CtrlPos(5)), p(104));
		}
	}

	#[test]
	fn an_extended_window_counts_what_was_loaded_before() {
		let mut window = TextWindow::of(&slice(0, "a\u{1F600}"), DEFAULT_TARGET);
		window.extend_with(&slice(2, "b\u{1F600}c"));
		let expected = units([0, 1, 3, 4, 6], [0, 1, 2, 3, 4]);
		for (position, units) in (0..5).map(p).zip(expected) {
			assert_eq!(window.to_local(position), CtrlPos(units), "position {position}");
			assert_eq!(window.to_doc(CtrlPos(units)), position, "units {units}");
		}
		assert_eq!(window, TextWindow::of(&slice(0, "a\u{1F600}b\u{1F600}c"), DEFAULT_TARGET));
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
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + DEFAULT_TARGET);
		assert!(
			window.needs_reload_for(
				window.start + DocLen::from_raw(DEFAULT_TARGET.get() / 4) - DocLen::ONE,
				l(10_000_000)
			)
		);
		assert!(
			window
				.needs_reload_for(window.end - DocLen::from_raw(DEFAULT_TARGET.get() / 4) + DocLen::ONE, l(10_000_000))
		);
		let middle = window.start + l(DEFAULT_TARGET.get() / 2);
		assert!(!window.needs_reload_for(middle, l(10_000_000)));
	}

	#[test]
	fn needs_reload_for_is_false_near_an_edge_that_is_the_real_document_boundary() {
		let window = TextWindow::new(p(0), DEFAULT_TARGET.as_pos());
		assert!(!window.needs_reload_for(p(10), l(10_000_000)));
		let window = TextWindow::new(p(10_000_000) - DEFAULT_TARGET, p(10_000_000));
		assert!(!window.needs_reload_for(p(10_000_000 - 10), l(10_000_000)));
	}

	#[test]
	fn should_use_whole_document_below_threshold_only() {
		assert!(should_use_whole_document(l(1000), DEFAULT_TARGET));
		assert!(!should_use_whole_document(l(10_000_000), DEFAULT_TARGET));
	}

	#[test]
	fn target_window_bounds_centers_with_headroom_and_clamps_to_document() {
		let (start, end) = target_window_bounds(p(1_000_000), l(10_000_000), DEFAULT_TARGET);
		assert_eq!(start, p(1_000_000 - DEFAULT_TARGET.get() / 2));
		assert_eq!(end, start + DEFAULT_TARGET);
		let (start, end) = target_window_bounds(p(100), l(10_000_000), DEFAULT_TARGET);
		assert_eq!(start, p(0));
		assert_eq!(end, DEFAULT_TARGET.as_pos());
		let (start, end) = target_window_bounds(p(9_999_000), l(10_000_000), DEFAULT_TARGET);
		assert_eq!(end, p(10_000_000));
		assert_eq!(start, p(10_000_000) - DEFAULT_TARGET);
		let (start, end) = target_window_bounds(p(10_000_000), l(10_000_000), DEFAULT_TARGET);
		assert_eq!((start, end), (p(10_000_000) - DEFAULT_TARGET, p(10_000_000)));
		assert!(!TextWindow::new(start, end).needs_reload_for(p(10_000_000), l(10_000_000)));
		assert_eq!(target_window_bounds(p(300), l(1000), DEFAULT_TARGET), (p(0), p(1000)));
	}
	// A forward-moving caret must extend, the one operation a Say-All survives.
	#[test]
	fn a_caret_approaching_the_loaded_end_extends_rather_than_compacts() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + DEFAULT_TARGET);
		let near_end = window.end - margin() + DocLen::ONE;
		assert!(window.wants_extension_for(near_end, l(10_000_000)));
		assert!(!window.needs_compaction_for(near_end, l(10_000_000)));
	}

	#[test]
	fn a_caret_in_the_middle_wants_nothing() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + DEFAULT_TARGET);
		let middle = window.start + l(DEFAULT_TARGET.get() / 2);
		assert!(!window.wants_extension_for(middle, l(10_000_000)));
		assert!(!window.needs_compaction_for(middle, l(10_000_000)));
	}

	// A Say-All only moves forward, so it never reaches this.
	#[test]
	fn a_caret_approaching_a_movable_start_compacts() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + DEFAULT_TARGET);
		assert!(window.needs_compaction_for(window.start + margin() - DocLen::ONE, l(10_000_000)));
	}

	#[test]
	fn nothing_is_wanted_at_the_real_end_of_the_document() {
		let doc_len = l(10_000_000);
		let window = TextWindow::new(doc_len.as_pos() - DEFAULT_TARGET, doc_len.as_pos());
		assert!(!window.wants_extension_for(doc_len.as_pos() - l(10), doc_len));
		assert!(!window.needs_compaction_for(doc_len.as_pos() - l(10), doc_len));
	}

	// A gap would drop text and an overlap repeat a paragraph, either shifting every later offset.
	#[test]
	fn extension_starts_exactly_at_the_loaded_end() {
		let window = TextWindow::new(p(1_000_000), p(1_500_000));
		let (from, to) = window.extend_bounds(l(10_000_000));
		assert_eq!(from, p(1_500_000));
		assert_eq!(to, p(1_500_000) + chunk());
	}

	#[test]
	fn extension_stops_at_the_end_of_the_document() {
		let window = TextWindow::new(p(0), p(900_000));
		assert_eq!(window.extend_bounds(l(1_000_000)), (p(900_000), p(1_000_000)));
	}

	// `start` moving is what breaks a reader.
	#[test]
	fn extending_never_moves_the_start() {
		let mut window = TextWindow::new(p(1_000_000), p(1_500_000));
		window.extend_end_to(p(1_750_000));
		assert_eq!(window.start, p(1_000_000));
		assert_eq!(window.end, p(1_750_000));
		assert_eq!(window.loaded_len(), l(750_000));
		window.extend_end_to(p(1_600_000));
		assert_eq!(window.end, p(1_750_000));
	}

	#[test]
	fn compaction_threshold_tolerates_a_couple_of_extensions() {
		let one = TextWindow::new(p(0), (DEFAULT_TARGET + chunk()).as_pos());
		assert!(one.loaded_len() <= one.compaction_threshold());
		let many = TextWindow::new(p(0), (DEFAULT_TARGET + l(chunk().get() * 6)).as_pos());
		assert!(many.loaded_len() > many.compaction_threshold());
	}

	// Reported as a Say-All restarting from the top of a large file: compaction on a timer is self-sustaining, since the stale offset lands near the new start and triggers another, and walked the window to the top in twelve ticks.
	#[test]
	fn the_pump_never_moves_the_ground_under_a_reader() {
		let doc_len = l(5_000_000);
		let start = p(1_750_000);
		let mut window = TextWindow::new(start, start + DEFAULT_TARGET);
		// Just inside the reload margin of the start, the worst case for a pump that must not compact.
		let mut reader_local = CtrlPos(i64::try_from(margin().get() - 25_000).unwrap());
		for _ in 0..500 {
			let before = window.to_doc(reader_local);
			if window.wants_extension_for(before, doc_len) {
				let (from, to) = window.extend_bounds(doc_len);
				assert_eq!(from, window.end, "an extension must begin exactly at the loaded end");
				window.extend_end_to(to);
			}
			assert_eq!(window.to_doc(CtrlPos::ZERO), start, "the pump must never move the loaded start");
			assert_eq!(window.to_doc(reader_local), before, "a held offset must still mean the same character");
			reader_local = reader_local + 200;
		}
		assert!(window.to_doc(reader_local) <= window.end, "extension must keep up with the reader");
	}

	// Still right here, but only where the user has already stopped any read.
	#[test]
	fn a_caret_that_has_run_out_of_text_behind_it_still_asks_to_compact() {
		let window = TextWindow::new(p(1_000_000), p(1_000_000) + DEFAULT_TARGET);
		assert!(window.needs_compaction_for(p(1_000_000) + margin() - DocLen::ONE, l(10_000_000)));
	}

	#[test]
	fn extension_leaves_a_held_offset_alone_and_compaction_moves_it() {
		let doc_len = l(5_000_000);
		let window = TextWindow::new(p(1_750_000), p(1_750_000) + DEFAULT_TARGET);
		let held_local = CtrlPos(i64::try_from(margin().get() - 25_000).unwrap());
		let before = window.to_doc(held_local);
		let mut extended = window.clone();
		extended.extend_end_to(window.end + chunk());
		assert_eq!(extended.to_doc(held_local), before, "extension must leave a held offset meaning the same place");
		let (start, end) = target_window_bounds(before, doc_len, DEFAULT_TARGET);
		let compacted = TextWindow::new(start, end);
		assert_ne!(
			compacted.to_doc(held_local),
			before,
			"compaction moves a held offset, which is why only a user action may trigger it"
		);
	}

	#[test]
	fn an_unlimited_target_loads_the_whole_document_and_never_reloads() {
		assert!(should_use_whole_document(l(10_000_000), UNLIMITED));
		assert_eq!(target_window_bounds(p(5_000_000), l(10_000_000), UNLIMITED), (p(0), p(10_000_000)));
		let mut window = TextWindow::with_target(p(0), p(10_000_000), UNLIMITED);
		assert!(!window.wants_extension_for(p(9_999_999), l(10_000_000)));
		assert!(!window.needs_compaction_for(p(1), l(10_000_000)));
		assert!(!window.needs_reload_for(p(9_999_999), l(10_000_000)));
		assert_eq!(window.extend_bounds(l(10_000_000)), (p(10_000_000), p(10_000_000)));
		assert!(window.loaded_len() <= window.compaction_threshold());
		window.set_target(DEFAULT_TARGET);
		assert!(window.loaded_len() > window.compaction_threshold());
	}

	#[test]
	fn a_larger_target_widens_the_window_and_its_reload_margin() {
		let target = l(2_000_000);
		let (start, end) = target_window_bounds(p(5_000_000), l(20_000_000), target);
		assert_eq!(end - start, target);
		assert_eq!((start, end), (p(4_000_000), p(6_000_000)));
		let window = TextWindow::with_target(start, end, target);
		assert!(!window.needs_reload_for(p(5_000_000), l(20_000_000)));
		assert!(window.needs_reload_for(p(4_100_000), l(20_000_000)));
	}
}
