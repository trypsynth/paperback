//! A port of pdfium's `CFX_BidiChar` and `CFX_BidiString` (`core/fxcrt/fx_bidi.cpp`), far simpler than the Unicode bidirectional algorithm on purpose: [`crate::parser::util::bidi`] was written against pdfium's ordering.
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

pub(super) struct BidiString {
	order: Vec<Segment>,
	overall: Direction,
}

impl BidiString {
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

	#[test]
	fn a_mostly_hebrew_line_reads_right_to_left() {
		let line = BidiString::new(&text("שלום a עולם"));
		assert_eq!(line.overall_direction(), Direction::Right);
		assert_eq!(line.segments().first().map(|segment| segment.direction), Some(Direction::Right));
	}
}
