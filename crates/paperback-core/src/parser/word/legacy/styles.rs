//! A built-in Heading 1-9 style is recognised by its `sti`, which is the level in every language Word is localized into, not by its name.

use super::properties::{ParagraphProperties, fc_lcb};

/// Offset of the stylesheet's (fc, lcb) pair in `FibRgFcLcb97`.
const FIB_FC_STSHF: usize = 0x00A2;
const STI_HEADINGS: std::ops::RangeInclusive<u16> = 1..=9;
const ISTD_NIL: u16 = 0x0FFF;
/// The only style kind that carries paragraph properties.
const STK_PARAGRAPH: u16 = 1;
pub(super) const OUTLINE_BODY_TEXT: u8 = 9;
/// Bounds the based-on chain against a stylesheet that loops.
const MAX_BASE_DEPTH: usize = 16;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Style {
	sti: u16,
	base: u16,
	/// Before anything the style is based on.
	properties: ParagraphProperties,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ParagraphStyle {
	pub heading: Option<u8>,
	pub ilfo: Option<i16>,
	pub ilvl: Option<u8>,
}

pub(super) fn paragraph_styles(word_document: &[u8], table: &[u8]) -> Vec<ParagraphStyle> {
	let styles = fc_lcb(word_document, FIB_FC_STSHF).and_then(|at| read_stylesheet(table, at)).unwrap_or_default();
	(0..styles.len())
		.map(|istd| {
			let chain = || based_on_chain(&styles, istd);
			ParagraphStyle {
				// The nearest style that says anything about an outline level decides it: its own outline level if it sets one, else its built-in heading level.
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

fn based_on_chain(styles: &[Option<Style>], istd: usize) -> impl Iterator<Item = Style> + '_ {
	let mut next = Some(istd);
	std::iter::from_fn(move || {
		let style = styles.get(next?).copied().flatten()?;
		next = (style.base != ISTD_NIL).then_some(usize::from(style.base));
		Some(style)
	})
	.take(MAX_BASE_DEPTH)
}

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

/// The `Stdf` is `cb_base` bytes, of which the first ten are `StdfBase`; a paragraph style's properties are the first `UPX` after the name.
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
	let cb_upx = usize::from(read_u16(std, upx_at)?);
	style.properties = ParagraphProperties::read(std.get(upx_at + 2..upx_at + 2 + cb_upx)?)?;
	Some(style)
}

fn read_u16(data: &[u8], at: usize) -> Option<u16> {
	data.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]))
}

#[cfg(test)]
mod tests;
