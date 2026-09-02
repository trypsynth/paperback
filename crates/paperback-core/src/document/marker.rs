//! Marker types: the tagged positions ([`Marker`]) a parser scatters through a
//! [`super::buffer::DocumentBuffer`] to record headings, links, formatting spans, and other
//! structure, plus the small helpers ([`is_heading_marker`], [`is_container_marker`],
//! [`ContainerSpan`]) that classify them.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum MarkerType {
	Heading1 = 0,
	Heading2 = 1,
	Heading3 = 2,
	Heading4 = 3,
	Heading5 = 4,
	Heading6 = 5,
	PageBreak = 6,
	SectionBreak = 7,
	TocItem = 8,
	Link = 9,
	List = 10,
	ListItem = 11,
	Table = 12,
	Separator = 13,
	Image = 14,
	Figure = 15,
	Bold = 16,
	Italic = 17,
	Underline = 18,
}

impl From<MarkerType> for i32 {
	fn from(marker: MarkerType) -> Self {
		marker as Self
	}
}

/// Yields the character-formatting marker types implied by the given
/// bold/italic/underline flags, in a stable order. Shared by the parsers so
/// the flag-triple → marker fan-out lives in one place.
pub(crate) fn format_marker_types(bold: bool, italic: bool, underline: bool) -> impl Iterator<Item = MarkerType> {
	[(bold, MarkerType::Bold), (italic, MarkerType::Italic), (underline, MarkerType::Underline)]
		.into_iter()
		.filter_map(|(on, kind)| on.then_some(kind))
}

impl TryFrom<i32> for MarkerType {
	type Error = ();

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Heading1),
			1 => Ok(Self::Heading2),
			2 => Ok(Self::Heading3),
			3 => Ok(Self::Heading4),
			4 => Ok(Self::Heading5),
			5 => Ok(Self::Heading6),
			6 => Ok(Self::PageBreak),
			7 => Ok(Self::SectionBreak),
			8 => Ok(Self::TocItem),
			9 => Ok(Self::Link),
			10 => Ok(Self::List),
			11 => Ok(Self::ListItem),
			12 => Ok(Self::Table),
			13 => Ok(Self::Separator),
			14 => Ok(Self::Image),
			15 => Ok(Self::Figure),
			16 => Ok(Self::Bold),
			17 => Ok(Self::Italic),
			18 => Ok(Self::Underline),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Marker {
	pub mtype: MarkerType,
	pub position: usize,
	pub text: String,
	pub reference: String,
	pub level: i32,
	pub length: usize,
}

impl Marker {
	#[must_use]
	pub const fn new(mtype: MarkerType, position: usize) -> Self {
		Self { mtype, position, text: String::new(), reference: String::new(), level: 0, length: 0 }
	}

	#[must_use]
	pub fn with_text(mut self, text: String) -> Self {
		self.text = text;
		self
	}

	#[must_use]
	pub fn with_reference(mut self, reference: String) -> Self {
		self.reference = reference;
		self
	}

	#[must_use]
	pub const fn with_level(mut self, level: i32) -> Self {
		self.level = level;
		self
	}

	#[must_use]
	pub const fn with_length(mut self, length: usize) -> Self {
		self.length = length;
		self
	}
}

#[must_use]
pub const fn is_heading_marker(marker_type: MarkerType) -> bool {
	matches!(
		marker_type,
		MarkerType::Heading1
			| MarkerType::Heading2
			| MarkerType::Heading3
			| MarkerType::Heading4
			| MarkerType::Heading5
			| MarkerType::Heading6
	)
}

/// Whether a marker type denotes a navigable container (an element the caret can be *inside* of,
/// such as a list or table). The single place to extend the set of container types.
#[must_use]
pub const fn is_container_marker(marker_type: MarkerType) -> bool {
	matches!(marker_type, MarkerType::List | MarkerType::Table)
}

/// The display-unit span of a container marker: `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerSpan {
	pub start: usize,
	pub end: usize,
	pub mtype: MarkerType,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn marker_type_round_trip_for_all_known_values() {
		for raw in 0..=18 {
			let marker = MarkerType::try_from(raw).unwrap();
			assert_eq!(i32::from(marker), raw);
		}
		assert!(MarkerType::try_from(19).is_err());
		assert!(MarkerType::try_from(-1).is_err());
	}

	#[test]
	fn marker_builder_helpers_set_all_fields() {
		let marker = Marker::new(MarkerType::Table, 42)
			.with_text("Title".to_string())
			.with_reference("ref".to_string())
			.with_level(3)
			.with_length(9);
		assert_eq!(marker.position, 42);
		assert_eq!(marker.text, "Title");
		assert_eq!(marker.reference, "ref");
		assert_eq!(marker.level, 3);
		assert_eq!(marker.length, 9);
	}

	#[test]
	fn heading_marker_helper_matches_heading_types_only() {
		assert!(is_heading_marker(MarkerType::Heading1));
		assert!(is_heading_marker(MarkerType::Heading6));
		assert!(!is_heading_marker(MarkerType::Link));
		assert!(!is_heading_marker(MarkerType::SectionBreak));
	}
}
