//! Maps document positions to the control's own positions. A document counts one position per char, while Scintilla counts UTF-8
//! bytes, so the whole document is scanned once for the chars that take more than one byte.

use paperback_core::{
	document::{DocLen, DocPos},
	session::WindowSlice,
};

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

/// The whole document as loaded into the control, and what it takes to translate a position between the two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextWindow {
	start: DocPos,
	end: DocPos,
	/// At each boundary around a multi-byte char, the cumulative extra bytes before it: `(char offset, extra)`.
	extras: Vec<(DocLen, i64)>,
}

impl TextWindow {
	#[cfg(test)]
	pub const fn new(start: DocPos, end: DocPos) -> Self {
		Self { start, end, extras: Vec::new() }
	}

	pub fn of(slice: &WindowSlice) -> Self {
		let mut window = Self { start: slice.start, end: slice.end, extras: Vec::new() };
		let mut extra = 0_i64;
		for (position, c) in slice.text.chars().enumerate() {
			let units = c.len_utf8() - 1;
			if units > 0 {
				let units = i64::try_from(units).unwrap_or(i64::MAX);
				let position = DocLen::from_raw(position);
				window.extras.push((position, extra));
				window.extras.push((position + DocLen::ONE, extra + units));
				extra += units;
			}
		}
		window
	}

	#[cfg(test)]
	pub const fn whole(doc_len: DocLen) -> Self {
		Self { start: DocPos::ZERO, end: doc_len.as_pos(), extras: Vec::new() }
	}

	pub const fn end(&self) -> DocPos {
		self.end
	}

	pub fn loaded_len(&self) -> DocLen {
		self.end - self.start
	}

	pub fn is_whole_document(&self, doc_len: DocLen) -> bool {
		self.start == DocPos::ZERO && self.end >= doc_len.as_pos()
	}

	/// The extra bytes accumulated before `position`.
	fn extra_before(&self, position: DocLen) -> i64 {
		let passed = self.extras.partition_point(|&(at, _)| at <= position);
		if passed == 0 { 0 } else { self.extras[passed - 1].1 }
	}

	/// Clamped to the loaded text.
	pub fn to_local(&self, doc_pos: DocPos) -> CtrlPos {
		let position = doc_pos.since(self.start).min(self.loaded_len());
		let extra = self.extra_before(position);
		CtrlPos(i64::try_from(position.get()).unwrap_or(i64::MAX).saturating_add(extra))
	}

	/// A position between the bytes of one char rounds forward to the char after it, as the document does.
	pub fn to_doc(&self, local_pos: CtrlPos) -> DocPos {
		let local = local_pos.0.max(0);
		let last_extra = self.extras.last().map_or(0, |&(_, extra)| extra);
		let control_len = i64::try_from(self.loaded_len().get()).unwrap_or(i64::MAX).saturating_add(last_extra);
		let local = local.min(control_len);
		let at_i64 = |at: DocLen| i64::try_from(at.get()).unwrap_or(i64::MAX);
		let position = match self.extras.partition_point(|&(at, extra)| at_i64(at) + extra <= local) {
			0 => local,
			passed => {
				let (at, extra) = self.extras[passed - 1];
				let at = at_i64(at);
				let control_at = at + extra;
				if local == control_at {
					at
				} else if let Some(&(next_at, next_extra)) = self.extras.get(passed)
					&& at_i64(next_at) == at + 1
					&& local < at_i64(next_at) + next_extra
				{
					at + 1
				} else {
					local - extra
				}
			}
		};
		let position = usize::try_from(position.max(0)).unwrap_or(0).min(self.loaded_len().get());
		self.start + DocLen::from_raw(position)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn p(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	fn slice(start: usize, text: &str) -> WindowSlice {
		let start = p(start);
		let end = start + DocLen::of(text);
		WindowSlice { start, end, text: text.to_string(), markers: Vec::new(), paragraphs: Vec::new() }
	}

	#[test]
	fn to_local_and_to_doc_round_trip_within_bounds() {
		let window = TextWindow::new(p(1000), p(2000));
		assert_eq!(window.to_local(p(1500)), CtrlPos(500));
		assert_eq!(window.to_doc(CtrlPos(500)), p(1500));
	}

	#[test]
	fn utf8_positions_map_scalars_to_bytes_and_back() {
		let window = TextWindow::of(&slice(0, "aé😀b"));
		// a is byte 0, é bytes 1..=2, 😀 bytes 3..=6, b byte 7, end 8.
		for (position, byte) in [(0, 0), (1, 1), (2, 3), (3, 7), (4, 8)] {
			assert_eq!(window.to_local(p(position)), CtrlPos(byte), "position {position}");
		}
		assert_eq!(window.to_doc(CtrlPos(1)), p(1), "start of é");
		assert_eq!(window.to_doc(CtrlPos(2)), p(2), "inside é rounds forward");
		assert_eq!(window.to_doc(CtrlPos(3)), p(2), "start of 😀");
		assert_eq!(window.to_doc(CtrlPos(4)), p(3), "inside 😀 rounds forward");
		assert_eq!(window.to_doc(CtrlPos(7)), p(3), "b");
		assert_eq!(window.to_doc(CtrlPos(8)), p(4), "end");
	}

	#[test]
	fn a_char_outside_the_bmp_takes_its_utf8_bytes() {
		// "a😀b😀c" from position 100: a=100, 😀=101, b=102, 😀=103, c=104.
		let window = TextWindow::of(&slice(100, "a\u{1F600}b\u{1F600}c"));
		let expected = [0, 1, 5, 6, 10];
		for (position, byte) in (100..105).map(p).zip(expected) {
			assert_eq!(window.to_local(position), CtrlPos(byte), "position {position}");
			assert_eq!(window.to_doc(CtrlPos(byte)), position, "byte {byte}");
		}
		assert_eq!(window.to_local(p(105)), CtrlPos(11));
		assert_eq!(window.to_doc(CtrlPos(2)), p(102), "inside the first emoji rounds forward");
		assert_eq!(window.to_doc(CtrlPos(6)), p(103), "start of the second emoji");
	}

	#[test]
	fn to_local_clamps_outside_the_window() {
		let window = TextWindow::new(p(1000), p(2000));
		assert_eq!(window.to_local(p(0)), CtrlPos(0));
		assert_eq!(window.to_local(p(5000)), CtrlPos(1000));
	}

	#[test]
	fn is_whole_document_true_only_when_covering_everything() {
		assert!(TextWindow::whole(DocLen::from_raw(10_000)).is_whole_document(DocLen::from_raw(10_000)));
		assert!(!TextWindow::new(p(0), p(5000)).is_whole_document(DocLen::from_raw(10_000)));
		assert!(!TextWindow::new(p(100), p(10_000)).is_whole_document(DocLen::from_raw(10_000)));
	}
}
