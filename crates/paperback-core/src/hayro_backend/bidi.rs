//! pdfium's `CFX_BidiChar` and `CFX_BidiString` (`core/fxcrt/fx_bidi.cpp`), which split a line into runs of one direction and decide which way the line as a whole reads.
//!
//! This is much simpler than the Unicode bidirectional algorithm, and on purpose: the text layer orders right-to-left runs the way pdfium does, and [`crate::parser::util::bidi`] was written against the result.
//!
//! Portions are derived from `PDFium`, used under its BSD-style license:
//! Copyright 2014 The `PDFium` Authors. Original code copyright 2014 Foxit Software Inc. <http://www.foxitsoftware.com>

use icu_properties::{
	CodePointMapData,
	props::{BidiClass, BidiMirroringGlyph},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
	Neutral,
	Left,
	LeftWeak,
	Right,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Segment {
	pub(super) start: usize,
	pub(super) count: usize,
	pub(super) direction: Direction,
}

fn direction_of(c: u32) -> Direction {
	let Some(ch) = char::from_u32(c) else { return Direction::Neutral };
	match CodePointMapData::<BidiClass>::new().get(ch) {
		BidiClass::LeftToRight => Direction::Left,
		BidiClass::ArabicNumber
		| BidiClass::EuropeanNumber
		| BidiClass::NonspacingMark
		| BidiClass::CommonSeparator
		| BidiClass::EuropeanSeparator
		| BidiClass::EuropeanTerminator
		| BidiClass::BoundaryNeutral => Direction::LeftWeak,
		BidiClass::RightToLeft | BidiClass::ArabicLetter => Direction::Right,
		_ => Direction::Neutral,
	}
}

/// A line split into runs of one direction, in the order the line reads.
pub(super) struct BidiString {
	order: Vec<Segment>,
	overall: Direction,
}

impl BidiString {
	/// Splits `text`, reading it right to left when it has more right-to-left runs than left-to-right ones.
	pub(super) fn new(text: &[u32]) -> Self {
		let mut order = Vec::new();
		let mut current = Segment { start: 0, count: 0, direction: Direction::Neutral };
		for &c in text {
			let direction = direction_of(c);
			if direction != current.direction {
				order.push(current);
				current = Segment { start: current.start + current.count, count: 0, direction };
			}
			current.count += 1;
		}
		order.push(current);
		order.retain(|segment| segment.count > 0);
		let right = order.iter().filter(|segment| segment.direction == Direction::Right).count();
		let left = order.iter().filter(|segment| segment.direction == Direction::Left).count();
		let mut string = Self { order, overall: Direction::Left };
		if right > left {
			string.order.reverse();
			string.overall = Direction::Right;
		}
		string
	}

	pub(super) const fn overall_direction(&self) -> Direction {
		self.overall
	}

	pub(super) fn segments(&self) -> &[Segment] {
		&self.order
	}
}

pub(super) fn overall_direction(text: &[u32]) -> Direction {
	BidiString::new(text).overall_direction()
}

/// The character's mirror image, such as `)` for `(`, which is what a right-to-left run shows in its place.
pub(super) fn mirror(c: u32) -> u32 {
	char::from_u32(c)
		.and_then(|ch| CodePointMapData::<BidiMirroringGlyph>::new().get(ch).mirroring_glyph)
		.map_or(c, u32::from)
}

#[cfg(test)]
mod tests {
	use super::{BidiString, Direction};

	fn text(s: &str) -> Vec<u32> {
		s.chars().map(u32::from).collect()
	}

	#[test]
	fn a_latin_line_reads_left_to_right() {
		assert_eq!(BidiString::new(&text("abc def")).overall_direction(), Direction::Left);
	}

	/// Two Hebrew runs outnumber the one Latin run, so the line reads right to left and its runs come in reverse.
	#[test]
	fn a_mostly_hebrew_line_reads_right_to_left() {
		let line = BidiString::new(&text("שלום a עולם"));
		assert_eq!(line.overall_direction(), Direction::Right);
		assert_eq!(line.segments().first().map(|segment| segment.direction), Some(Direction::Right));
	}
}
