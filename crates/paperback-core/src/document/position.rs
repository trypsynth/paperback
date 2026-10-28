//! Scalar positions and content byte offsets are separate types so the compiler, not a doc comment, keeps them apart (#1062).

use std::{
	iter::Sum,
	ops::{Add, AddAssign, Sub, SubAssign},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Unicode scalars, the same on every platform; a control counting UTF-16 units is converted where the desktop loads text into it.
/// Positions outlive their buffers (saved positions, history, a reparse), so one can point past the end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DocPos(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DocLen(usize);

impl DocPos {
	pub const ZERO: Self = Self(0);

	/// Only where a value enters the core (the FFI, the config, the desktop's text control); everything else comes from the buffer or [`DocLen::of`].
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}

	#[must_use]
	pub const fn from_start(self) -> DocLen {
		DocLen(self.0)
	}

	#[must_use]
	pub const fn saturating_sub(self, len: DocLen) -> Self {
		Self(self.0.saturating_sub(len.0))
	}

	#[must_use]
	pub const fn since(self, earlier: Self) -> DocLen {
		DocLen(self.0.saturating_sub(earlier.0))
	}
}

impl DocLen {
	pub const ZERO: Self = Self(0);
	pub const ONE: Self = Self(1);

	/// Only for a count not measured from text, such as one from the FFI.
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}

	#[must_use]
	pub fn of(text: &str) -> Self {
		Self(text.chars().count())
	}

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

/// There is no `DocPos + DocPos`, as adding two places in a document means nothing.
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

impl TryFrom<DocPos> for i64 {
	type Error = std::num::TryFromIntError;

	fn try_from(pos: DocPos) -> Result<Self, Self::Error> {
		Self::try_from(pos.0)
	}
}

impl TryFrom<DocLen> for i64 {
	type Error = std::num::TryFromIntError;

	fn try_from(len: DocLen) -> Result<Self, Self::Error> {
		Self::try_from(len.0)
	}
}

fn saturating_usize(raw: i64) -> usize {
	usize::try_from(raw.max(0)).unwrap_or(usize::MAX)
}

/// Saved as the plain integer the config has always held.
impl Serialize for DocPos {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_i64(i64::try_from(self.0).unwrap_or(i64::MAX))
	}
}

/// A negative position in an edited file loads as the start rather than failing the whole config.
impl<'de> Deserialize<'de> for DocPos {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		i64::deserialize(deserializer).map(|raw| Self(saturating_usize(raw)))
	}
}

// `i64` so the apps' bindings don't change; a negative value lifts to the start, since iOS's Go to page can pass back the -1 of a missing page.
#[cfg(feature = "uniffi")]
uniffi::custom_type!(DocPos, i64, { lower: |pos| i64::try_from(pos).unwrap_or(i64::MAX), try_lift: |raw| Ok(DocPos(saturating_usize(raw))) });

#[cfg(feature = "uniffi")]
uniffi::custom_type!(DocLen, i64, { lower: |len| i64::try_from(len).unwrap_or(i64::MAX), try_lift: |raw| Ok(DocLen(saturating_usize(raw))) });

/// Only for [`super::DocumentBuffer::content`]; offsets into other strings stay `usize` so this can't slice the wrong one. Always on a char boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ContentBytePos(usize);

impl ContentBytePos {
	pub const ZERO: Self = Self(0);

	/// Only for an offset from a `str` operation on `content`, such as a regex match or a `find`.
	#[must_use]
	pub const fn from_raw(raw: usize) -> Self {
		Self(raw)
	}

	#[must_use]
	pub const fn get(self) -> usize {
		self.0
	}
}

impl Add<usize> for ContentBytePos {
	type Output = Self;

	fn add(self, bytes: usize) -> Self {
		Self::from_raw(self.get() + bytes)
	}
}

// Test-only, because outside tests a bare `usize` could be in any unit, and mixing units is what these types rule out.
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
