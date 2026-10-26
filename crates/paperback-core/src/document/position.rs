//! The units a place in a document is given in: [`DocPos`] and [`DocLen`] count Unicode scalars, and [`ContentBytePos`] counts bytes into [`super::DocumentBuffer::content`]. Each is its own type so that the compiler, not a doc comment, keeps one from being used as another (#1062).

use std::{
	iter::Sum,
	ops::{Add, AddAssign, Sub, SubAssign},
};

/// A place in a document, counted in Unicode scalars from its start: the unit of the caret, markers, bookmarks and every saved position, the same on every platform. A text control that counts otherwise, as Windows and macOS count UTF-16 code units, is converted where the desktop app loads text into it.
///
/// Nothing ties a position to one buffer, since positions outlive the buffers they were taken from (saved positions, history, a reparse), so one can point past the end or be from another document. [`super::DocumentBuffer::nearest_pos`] brings an outside value into range.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DocPos(usize);

/// How much of a document something takes up, in the scalars [`DocPos`] counts: what one position is past another, or the length of a piece of text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DocLen(usize);

impl DocPos {
	pub const ZERO: Self = Self(0);

	/// Wraps a count of scalars from the start of a document. Positions come from the buffer, from [`DocLen::of`], or from where a value enters the core (the FFI, the config, the desktop's text control), which is the only place this should be called.
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}

	/// How far this position is from the start of the document, as a length.
	#[must_use]
	pub const fn from_start(self) -> DocLen {
		DocLen(self.0)
	}

	/// The position `len` before this one, or the start of the document if that is further back.
	#[must_use]
	pub const fn saturating_sub(self, len: DocLen) -> Self {
		Self(self.0.saturating_sub(len.0))
	}

	/// How far past `earlier` this position is, or nothing when it comes before it.
	#[must_use]
	pub const fn since(self, earlier: Self) -> DocLen {
		DocLen(self.0.saturating_sub(earlier.0))
	}
}

impl DocLen {
	pub const ZERO: Self = Self(0);
	/// One char, such as the newline after a line.
	pub const ONE: Self = Self(1);

	/// Wraps a count of scalars that wasn't measured from text, such as one passed in through the FFI.
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}

	/// How many positions `text` takes up in a document: one for each Unicode scalar.
	#[must_use]
	pub fn of(text: &str) -> Self {
		Self(text.chars().count())
	}

	/// The position this far from the start of the document.
	#[must_use]
	pub const fn as_pos(self) -> DocPos {
		DocPos(self.0)
	}

	#[must_use]
	pub const fn is_zero(self) -> bool {
		self.0 == 0
	}

	#[must_use]
	pub const fn saturating_sub(self, other: Self) -> Self {
		Self(self.0.saturating_sub(other.0))
	}
}

impl Add<DocLen> for DocPos {
	type Output = Self;

	fn add(self, len: DocLen) -> Self {
		Self(self.0 + len.0)
	}
}

impl AddAssign<DocLen> for DocPos {
	fn add_assign(&mut self, len: DocLen) {
		self.0 += len.0;
	}
}

impl Sub<DocLen> for DocPos {
	type Output = Self;

	fn sub(self, len: DocLen) -> Self {
		Self(self.0 - len.0)
	}
}

impl SubAssign<DocLen> for DocPos {
	fn sub_assign(&mut self, len: DocLen) {
		self.0 -= len.0;
	}
}

/// How far apart two positions are. There is no `DocPos + DocPos`, as adding two places in a document means nothing.
impl Sub for DocPos {
	type Output = DocLen;

	fn sub(self, earlier: Self) -> DocLen {
		DocLen(self.0 - earlier.0)
	}
}

impl Add for DocLen {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl AddAssign for DocLen {
	fn add_assign(&mut self, other: Self) {
		self.0 += other.0;
	}
}

impl Sub for DocLen {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self(self.0 - other.0)
	}
}

impl Sum for DocLen {
	fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
		Self(iter.map(|len| len.0).sum())
	}
}

impl std::fmt::Display for DocPos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::fmt::Display for DocLen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// Where a position leaves the core, as the `i64` that the FFI, the config and wx take.
impl TryFrom<DocPos> for i64 {
	type Error = std::num::TryFromIntError;

	fn try_from(pos: DocPos) -> Result<Self, Self::Error> {
		Self::try_from(pos.0)
	}
}

/// Where a length leaves the core, as the `i64` that the FFI, the config and wx take.
impl TryFrom<DocLen> for i64 {
	type Error = std::num::TryFromIntError;

	fn try_from(len: DocLen) -> Result<Self, Self::Error> {
		Self::try_from(len.0)
	}
}

/// A byte offset into [`super::DocumentBuffer::content`]. Byte offsets into any other string, such as
/// a parser's own output before it is appended, stay plain `usize`: this type says which string
/// an offset belongs to, so that it can't be used to slice the wrong one.
///
/// Every value comes from the buffer's conversions or from a search of `content` itself, so it
/// sits on a char boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ContentBytePos(usize);

impl ContentBytePos {
	pub const ZERO: Self = Self(0);

	/// Wraps a byte offset taken from a `str` operation on [`super::DocumentBuffer::content`] (a regex
	/// match, a `find`), which is the only kind of `usize` that may become a `ContentBytePos`.
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}
}

/// Moves forward by a byte count, such as the length of a match found from this position.
impl Add<usize> for ContentBytePos {
	type Output = Self;

	fn add(self, bytes: usize) -> Self {
		Self::from_raw(self.get() + bytes)
	}
}

// Conversions from and comparisons with a bare `usize` let tests write positions as integer literals (`5.into()`, `assert_eq!(pos, 5)`), which the compiler infers as `usize` here. They are test-only because outside tests, a bare `usize` could be in any unit, and mixing units is the mistake these types exist to rule out.
macro_rules! test_literals {
	($($unit:ty),*) => {$(
		#[cfg(test)]
		impl From<usize> for $unit {
			fn from(raw: usize) -> Self {
				Self(raw)
			}
		}
		#[cfg(test)]
		impl PartialEq<usize> for $unit {
			fn eq(&self, other: &usize) -> bool {
				self.0 == *other
			}
		}
		#[cfg(test)]
		impl PartialEq<$unit> for usize {
			fn eq(&self, other: &$unit) -> bool {
				*self == other.0
			}
		}
	)*};
}

test_literals!(DocPos, DocLen, ContentBytePos);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_length_counts_scalars() {
		assert_eq!(DocLen::of(""), 0);
		assert_eq!(DocLen::of("abc"), 3);
		assert_eq!(DocLen::of("a\u{1F600}b"), 3);
		assert_eq!(DocLen::of("line\nwrap"), 9);
		assert_eq!(DocLen::of("\r\n"), 2);
	}

	#[test]
	fn positions_and_lengths_add_up() {
		let start = DocPos::from_raw(4);
		let end = start + DocLen::of("abc");
		assert_eq!(end, 7);
		assert_eq!(end - start, 3);
		assert_eq!(start.since(end), 0);
		assert_eq!(start.saturating_sub(DocLen::from_raw(9)), DocPos::ZERO);
	}
}
