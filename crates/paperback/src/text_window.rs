//! Maps document positions to the control's own positions. A document counts one position per char, while Scintilla counts UTF-8
//! bytes. The document buffer already keeps that map at checkpoint granularity, so the whole document is not scanned again here.

use paperback_core::{
	document::{ContentBytePos, DocLen, DocPos},
	session::{DocumentSession, WindowSlice},
};

/// Only this module converts to or from a document position, so the two kinds can't be mixed up (#1062).
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

/// The byte position in the control for a document position.
pub fn control_pos(session: &DocumentSession, doc_pos: DocPos) -> CtrlPos {
	let byte = session.handle().document().buffer.byte_at(doc_pos).get();
	CtrlPos::from_raw(i64::try_from(byte).unwrap_or(i64::MAX))
}

/// The document position for a control byte. A byte inside a character rounds forward to the character after it, as the document does.
pub fn document_pos(session: &DocumentSession, ctrl_pos: CtrlPos) -> DocPos {
	let byte = usize::try_from(ctrl_pos.get().max(0)).unwrap_or(0);
	session.handle().document().buffer.pos_at_byte(ContentBytePos::from_raw(byte))
}

/// The range loaded into the control. With the whole document loaded this is just its bounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextWindow {
	start: DocPos,
	end: DocPos,
}

impl TextWindow {
	#[cfg(test)]
	pub const fn new(start: DocPos, end: DocPos) -> Self {
		Self { start, end }
	}

	pub const fn of(slice: &WindowSlice<'_>) -> Self {
		Self { start: slice.start, end: slice.end }
	}

	#[cfg(test)]
	pub const fn whole(doc_len: DocLen) -> Self {
		Self { start: DocPos::ZERO, end: doc_len.as_pos() }
	}

	pub const fn end(&self) -> DocPos {
		self.end
	}

	pub fn is_whole_document(&self, doc_len: DocLen) -> bool {
		self.start == DocPos::ZERO && self.end >= doc_len.as_pos()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn p(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	#[test]
	fn is_whole_document_true_only_when_covering_everything() {
		assert!(TextWindow::whole(DocLen::from_raw(10_000)).is_whole_document(DocLen::from_raw(10_000)));
		assert!(!TextWindow::new(p(0), p(5000)).is_whole_document(DocLen::from_raw(10_000)));
		assert!(!TextWindow::new(p(100), p(10_000)).is_whole_document(DocLen::from_raw(10_000)));
	}
}
