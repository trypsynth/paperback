//! Word 97-2003 paragraph styles, read from the stylesheet (`STSH`) in the table stream, for the
//! headings and list numbering a paragraph takes from its style.
//!
//! A paragraph is a heading when its style is one of Word's built-in Heading 1-9 styles, whose
//! style identifier (`sti`) is the level in every language Word is localized into, or when the
//! style, or a style it is based on, gives it an outline level. A style can also put its
//! paragraphs in a list, which is how Word numbers headings `1`, `1.1`, `1.1.1`. Whatever the
//! paragraph sets itself (see [`super::properties`]) wins over its style.

use super::properties::{ParagraphProperties, fc_lcb};

/// Offset in the FIB of the stylesheet's (fc, lcb) pair, in `FibRgFcLcb97`.
const FIB_FC_STSHF: usize = 0x00A2;
/// Style identifiers of the built-in Heading 1 to Heading 9.
const STI_HEADINGS: std::ops::RangeInclusive<u16> = 1..=9;
/// `istdBase` when a style is based on no other.
const ISTD_NIL: u16 = 0x0FFF;
/// The style kind of a paragraph style, the only kind that carries paragraph properties.
const STK_PARAGRAPH: u16 = 1;
/// The outline level that means body text rather than a heading.
pub(super) const OUTLINE_BODY_TEXT: u8 = 9;
/// How far a chain of based-on styles is followed, against a stylesheet that loops.
const MAX_BASE_DEPTH: usize = 16;

/// One style, as the stylesheet gives it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Style {
	sti: u16,
	base: u16,
	/// The style's own paragraph properties, before anything it is based on.
	properties: ParagraphProperties,
}

/// What a paragraph style gives its paragraphs, once what it is based on is taken into account.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ParagraphStyle {
	/// The heading level (1-9), for a heading style.
	pub heading: Option<u8>,
	/// The list instance and level, as [`ParagraphProperties`] has them.
	pub ilfo: Option<i16>,
	pub ilvl: Option<u8>,
}

/// Each style's [`ParagraphStyle`], indexed by `istd`. Empty when the document has no stylesheet
/// or it cannot be read.
pub(super) fn paragraph_styles(word_document: &[u8], table: &[u8]) -> Vec<ParagraphStyle> {
	let styles = fc_lcb(word_document, FIB_FC_STSHF).and_then(|at| read_stylesheet(table, at)).unwrap_or_default();
	(0..styles.len())
		.map(|istd| {
			let chain = || based_on_chain(&styles, istd);
			ParagraphStyle {
				// The nearest style that says anything about an outline level decides it: its own
				// outline level if it sets one, else its built-in heading level.
				heading: chain()
					.find_map(|style| {
						style.properties.outline.map_or_else(
							|| STI_HEADINGS.contains(&style.sti).then(|| u8::try_from(style.sti).ok()),
							|outline| Some((outline < OUTLINE_BODY_TEXT).then_some(outline + 1)),
						)
					})
					.flatten(),
				ilfo: chain().find_map(|style| style.properties.ilfo),
				ilvl: chain().find_map(|style| style.properties.ilvl),
			}
		})
		.collect()
}

/// The style at `istd`, then the style it is based on, and so on.
fn based_on_chain(styles: &[Option<Style>], istd: usize) -> impl Iterator<Item = Style> + '_ {
	let mut next = Some(istd);
	std::iter::from_fn(move || {
		let style = styles.get(next?).copied().flatten()?;
		next = (style.base != ISTD_NIL).then_some(usize::from(style.base));
		Some(style)
	})
	.take(MAX_BASE_DEPTH)
}

/// `STSH`: the stylesheet header (`LPStshi`), whose first field counts the styles and whose second
/// sizes the fixed part of each, then one `LPStd` per style: a size, and the style itself (none
/// for an unused slot).
fn read_stylesheet(table: &[u8], (fc, lcb): (usize, usize)) -> Option<Vec<Option<Style>>> {
	let stsh = table.get(fc..fc.checked_add(lcb)?)?;
	let cb_stshi = usize::from(read_u16(stsh, 0)?);
	let count = usize::from(read_u16(stsh, 2)?);
	let cb_base = usize::from(read_u16(stsh, 4)?);
	let mut styles = Vec::with_capacity(count);
	let mut at = 2 + cb_stshi;
	for _ in 0..count {
		let cb_std = usize::from(read_u16(stsh, at)?);
		let std = stsh.get(at + 2..at + 2 + cb_std)?;
		styles.push((cb_std > 0).then(|| read_style(std, cb_base)).flatten());
		at += 2 + cb_std;
	}
	Some(styles)
}

/// `STD`: the fixed `Stdf` (`cb_base` bytes, of which the first ten are `StdfBase`), the style's
/// name, then for a paragraph style its paragraph properties as the first `UPX`.
fn read_style(std: &[u8], cb_base: usize) -> Option<Style> {
	let sti = read_u16(std, 0)? & 0x0FFF;
	let kind_and_base = read_u16(std, 2)?;
	let base = kind_and_base >> 4;
	let mut style = Style { sti, base, properties: ParagraphProperties::default() };
	let upx_count = read_u16(std, 4)? & 0xF;
	if kind_and_base & 0xF != STK_PARAGRAPH || upx_count == 0 {
		return Some(style);
	}
	// xstzName: a count of UTF-16 units, the units, and a terminating null.
	let name_units = usize::from(read_u16(std, cb_base)?);
	let upx_at = cb_base + 2 + (name_units + 1) * 2;
	// LPUpxPapx: a size, then the paragraph style's istd and its property modifiers.
	let cb_upx = usize::from(read_u16(std, upx_at)?);
	style.properties = ParagraphProperties::read(std.get(upx_at + 2..upx_at + 2 + cb_upx)?)?;
	Some(style)
}

fn read_u16(data: &[u8], at: usize) -> Option<u16> {
	data.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]))
}

#[cfg(test)]
mod tests;
