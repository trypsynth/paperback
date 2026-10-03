//! Word 97-2003 paragraph properties, which the list numbering and the headings are both read from.
//!
//! Each paragraph's properties (PAPX) sit in 512-byte formatting pages (FKPs) of the
//! `WordDocument` stream, found through the paragraph bin table (`PlcBtePapx`) in the table
//! stream. A PAPX names the paragraph's style (`istd`) and carries the property modifiers (sprms)
//! the paragraph applies on top of it.

/// Offset in the FIB of the paragraph bin table's (fc, lcb) pair, in `FibRgFcLcb97`.
const FIB_FC_PLCF_BTE_PAPX: usize = 0x0102;
const SPRM_P_ILVL: u16 = 0x260A;
const SPRM_P_ILFO: u16 = 0x460B;
const SPRM_P_OUT_LVL: u16 = 0x2640;
const SPRM_T_DEF_TABLE: u16 = 0xD608;
const SPRM_P_CHG_TABS: u16 = 0xC615;
const FKP_SIZE: usize = 512;

/// What a paragraph's properties, or a paragraph style's, say about its structure. Each is `None`
/// when these properties leave it to the style beneath.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ParagraphProperties {
	/// The paragraph style, an index into the stylesheet.
	pub istd: u16,
	/// The list instance (1-based) the paragraph is in; 0 takes it out of its style's list.
	pub ilfo: Option<i16>,
	/// The level of that list.
	pub ilvl: Option<u8>,
	/// The outline level (0-8, or 9 for body text).
	pub outline: Option<u8>,
}

impl ParagraphProperties {
	/// Reads a style index and the property modifiers that follow it, as a PAPX and a paragraph
	/// style's UPX both lay them out.
	pub(super) fn read(istd_and_grpprl: &[u8]) -> Option<Self> {
		let istd = istd_and_grpprl.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]]))?;
		let mut properties = Self { istd, ..Self::default() };
		for (sprm, operand) in sprms(&istd_and_grpprl[2..]) {
			match sprm {
				SPRM_P_ILFO => properties.ilfo = operand.get(..2).map(|b| i16::from_le_bytes([b[0], b[1]])),
				SPRM_P_ILVL => properties.ilvl = operand.first().map(|&l| l.min(8)),
				SPRM_P_OUT_LVL => properties.outline = operand.first().copied(),
				_ => {}
			}
		}
		Some(properties)
	}
}

/// Where to find each paragraph's properties.
pub(super) struct Paragraphs {
	/// Paragraph bin table: file-offset ranges and the FKP page holding their properties.
	bins: Vec<(u32, u32, u32)>,
}

impl Paragraphs {
	/// Reads the bin table. `None` when the document has none or it cannot be read.
	pub(super) fn read(word_document: &[u8], table: &[u8]) -> Option<Self> {
		read_bin_table(table, fc_lcb(word_document, FIB_FC_PLCF_BTE_PAPX)?).map(|bins| Self { bins })
	}

	/// The properties of the paragraph whose paragraph mark is at file offset `mark_fc`.
	pub(super) fn at(&self, word_document: &[u8], mark_fc: u32) -> Option<ParagraphProperties> {
		let &(_, _, page) = self.bins.iter().find(|&&(start, end, _)| start <= mark_fc && mark_fc < end)?;
		let base = usize::try_from(page).ok()?.checked_mul(FKP_SIZE)?;
		let fkp = word_document.get(base..base + FKP_SIZE)?;
		let crun = usize::from(fkp[FKP_SIZE - 1]);
		let run = (0..crun).find(|&i| {
			let start = read_u32(fkp, i * 4).unwrap_or(u32::MAX);
			let end = read_u32(fkp, (i + 1) * 4).unwrap_or(0);
			start <= mark_fc && mark_fc < end
		})?;
		// BxPap: one byte giving the PAPX's offset in words, then 12 bytes of layout cache.
		let b_offset = usize::from(*fkp.get((crun + 1) * 4 + run * 13)?) * 2;
		if b_offset == 0 {
			return Some(ParagraphProperties::default()); // default paragraph properties
		}
		let cb = usize::from(*fkp.get(b_offset)?);
		let (start, len) =
			if cb == 0 { (b_offset + 2, usize::from(*fkp.get(b_offset + 1)?) * 2) } else { (b_offset + 1, cb * 2 - 1) };
		// GrpPrlAndIstd: the paragraph style (istd), then the property modifiers.
		ParagraphProperties::read(fkp.get(start..(start + len).min(FKP_SIZE))?)
	}
}

/// The `(fc, lcb)` pair at `offset` in the FIB, when the structure it points at exists.
pub(super) fn fc_lcb(word_document: &[u8], offset: usize) -> Option<(usize, usize)> {
	let fc = usize::try_from(read_u32(word_document, offset)?).ok()?;
	let lcb = usize::try_from(read_u32(word_document, offset + 4)?).ok()?;
	(lcb > 0).then_some((fc, lcb))
}

/// `PlcBtePapx`: n+1 file offsets, then n FKP page numbers (the low 22 bits of each entry).
fn read_bin_table(table: &[u8], (fc, lcb): (usize, usize)) -> Option<Vec<(u32, u32, u32)>> {
	let plc = table.get(fc..fc.checked_add(lcb)?)?;
	let n = lcb.checked_sub(4)? / 8;
	let mut bins = Vec::with_capacity(n);
	for i in 0..n {
		let start = read_u32(plc, i * 4)?;
		let end = read_u32(plc, (i + 1) * 4)?;
		let page = read_u32(plc, (n + 1) * 4 + i * 4)? & 0x003F_FFFF;
		bins.push((start, end, page));
	}
	Some(bins)
}

/// The property modifiers in a grpprl, as `(sprm, operand)`. The operand's size comes from the
/// sprm's own top three bits; a variable-size one says its size in its first byte, except the two
/// that [MS-DOC] gives sizes of their own. Reading stops at anything it cannot size.
fn sprms(grpprl: &[u8]) -> Vec<(u16, &[u8])> {
	let mut out = Vec::new();
	let mut i = 0;
	while i + 2 <= grpprl.len() {
		let sprm = u16::from_le_bytes([grpprl[i], grpprl[i + 1]]);
		i += 2;
		let size = match sprm >> 13 {
			0 | 1 => 1,
			2 | 4 | 5 => 2,
			3 => 4,
			7 => 3,
			_ if sprm == SPRM_T_DEF_TABLE => {
				let Some(cb) = grpprl.get(i..i + 2).map(|b| usize::from(u16::from_le_bytes([b[0], b[1]]))) else {
					break;
				};
				cb + 1
			}
			_ if sprm == SPRM_P_CHG_TABS => break,
			_ => match grpprl.get(i) {
				Some(&cb) => {
					i += 1;
					usize::from(cb)
				}
				None => break,
			},
		};
		let Some(operand) = grpprl.get(i..i + size) else { break };
		out.push((sprm, operand));
		i += size;
	}
	out
}

pub(super) fn read_u32(data: &[u8], at: usize) -> Option<u32> {
	data.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests;
