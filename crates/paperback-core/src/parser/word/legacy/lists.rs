//! Word 97-2003 automatic numbering: the `1.`, `a)` and bullets a `.doc` generates rather than
//! stores in its text, read from the binary structures [MS-DOC] describes.
//!
//! * Each paragraph's properties (PAPX) sit in 512-byte formatting pages (FKPs) of the
//!   `WordDocument` stream, found through the paragraph bin table (`PlcBtePapx`) in the table
//!   stream. A paragraph is in a list when its properties carry `sprmPIlfo` (which list instance)
//!   and `sprmPIlvl` (which level).
//! * The list definitions (`PlfLst`) give each list's levels: number format, start value and the
//!   label template, in which a character 0-8 stands for that level's current number.
//! * The list instances (`PlfLfo`) point paragraphs at a definition, optionally restarting levels.
//!
//! These are translated into the same model `numbering.xml` is read into, so the counting and the
//! label formatting are shared with `.docx`.

use std::collections::HashMap;

use super::super::numbering::{Instance, Level, ListLabel, Numbering};

/// Offsets in the FIB of the (fc, lcb) pairs this needs, in `FibRgFcLcb97`.
const FIB_FC_PLCF_BTE_PAPX: usize = 0x0102;
const FIB_FC_PLF_LST: usize = 0x02E2;
const FIB_FC_PLF_LFO: usize = 0x02EA;
const SPRM_P_ILVL: u16 = 0x260A;
const SPRM_P_ILFO: u16 = 0x460B;
const SPRM_T_DEF_TABLE: u16 = 0xD608;
const SPRM_P_CHG_TABS: u16 = 0xC615;
const FKP_SIZE: usize = 512;
const LSTF_SIZE: usize = 28;
const LVLF_SIZE: usize = 28;
const LFO_SIZE: usize = 16;

/// A document's lists: where to find each paragraph's properties, and the numbering they refer to.
pub(super) struct DocLists {
	/// Paragraph bin table: file-offset ranges and the FKP page holding their properties.
	bins: Vec<(u32, u32, u32)>,
	numbering: Numbering,
}

impl DocLists {
	/// Reads the bin table and the list definitions. `None` when the document has no lists or its
	/// structures cannot be read, in which case it is read exactly as before.
	pub(super) fn read(word_document: &[u8], table: &[u8]) -> Option<Self> {
		let lists = read_plf_lst(table, fc_lcb(word_document, FIB_FC_PLF_LST)?)?;
		let instances = read_plf_lfo(table, fc_lcb(word_document, FIB_FC_PLF_LFO)?)?;
		let bins = read_bin_table(table, fc_lcb(word_document, FIB_FC_PLCF_BTE_PAPX)?)?;
		(!lists.is_empty() && !instances.is_empty())
			.then(|| Self { bins, numbering: Numbering::from_definitions(lists, instances) })
	}

	/// The label for the paragraph whose paragraph mark is at file offset `mark_fc`, advancing the
	/// count of the list it is in. `None` for a paragraph that is not in a list.
	pub(super) fn label_for_paragraph(&mut self, word_document: &[u8], mark_fc: u32) -> Option<ListLabel> {
		let (ilfo, ilvl) = self.paragraph_list(word_document, mark_fc)?;
		self.numbering.label_of(&ilfo.to_string(), usize::from(ilvl))
	}

	/// `(ilfo, ilvl)` from the properties of the paragraph whose mark is at `mark_fc`.
	fn paragraph_list(&self, word_document: &[u8], mark_fc: u32) -> Option<(i16, u8)> {
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
			return None; // default paragraph properties
		}
		let cb = usize::from(*fkp.get(b_offset)?);
		let (start, len) =
			if cb == 0 { (b_offset + 2, usize::from(*fkp.get(b_offset + 1)?) * 2) } else { (b_offset + 1, cb * 2 - 1) };
		// GrpPrlAndIstd: the paragraph style (istd), then the property modifiers.
		let grpprl = fkp.get(start + 2..(start + len).min(FKP_SIZE))?;
		let (mut ilfo, mut ilvl) = (None, 0u8);
		for (sprm, operand) in sprms(grpprl) {
			match sprm {
				SPRM_P_ILFO => ilfo = operand.get(..2).map(|b| i16::from_le_bytes([b[0], b[1]])),
				SPRM_P_ILVL => ilvl = operand.first().copied().unwrap_or(0),
				_ => {}
			}
		}
		ilfo.filter(|&i| i > 0).map(|i| (i, ilvl.min(8)))
	}
}

/// The `(fc, lcb)` pair at `offset` in the FIB, when the structure it points at exists.
fn fc_lcb(word_document: &[u8], offset: usize) -> Option<(usize, usize)> {
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

/// `PlfLst`: the list definitions, followed in the table stream by their levels (one for a
/// simple list, nine otherwise). Keyed by the list's `lsid`, which is what instances refer to.
fn read_plf_lst(table: &[u8], (fc, lcb): (usize, usize)) -> Option<HashMap<String, Vec<Option<Level>>>> {
	table.get(fc..fc.checked_add(lcb)?)?;
	let count = usize::try_from(i16::from_le_bytes(table.get(fc..fc + 2)?.try_into().ok()?)).ok()?;
	let mut lists = HashMap::new();
	let mut levels_at = fc + 2 + count * LSTF_SIZE;
	for i in 0..count {
		let lstf = table.get(fc + 2 + i * LSTF_SIZE..fc + 2 + (i + 1) * LSTF_SIZE)?;
		let lsid = i32::from_le_bytes(lstf[0..4].try_into().ok()?);
		let simple = lstf[26] & 1 != 0;
		let mut levels = vec![None; 9];
		for slot in levels.iter_mut().take(if simple { 1 } else { 9 }) {
			let (level, next) = read_lvl(table, levels_at)?;
			*slot = Some(level);
			levels_at = next;
		}
		lists.insert(lsid.to_string(), levels);
	}
	Some(lists)
}

/// One `LVL`: the fixed LVLF, its paragraph and character property modifiers, then the label
/// template as a counted UTF-16 string. Returns the level and the offset just past it.
fn read_lvl(table: &[u8], at: usize) -> Option<(Level, usize)> {
	let lvlf = table.get(at..at + LVLF_SIZE)?;
	let start = i32::from_le_bytes(lvlf[0..4].try_into().ok()?);
	let nfc = lvlf[4];
	let legal = lvlf[5] & 0b100 != 0;
	let placeholders: Vec<usize> = lvlf[6..15].iter().filter(|&&b| b != 0).map(|&b| usize::from(b)).collect();
	let cb_chpx = usize::from(lvlf[24]);
	let cb_papx = usize::from(lvlf[25]);
	let xst_at = at + LVLF_SIZE + cb_papx + cb_chpx;
	let cch = usize::from(u16::from_le_bytes(table.get(xst_at..xst_at + 2)?.try_into().ok()?));
	let units: Vec<u16> = table
		.get(xst_at + 2..xst_at + 2 + cch * 2)?
		.as_chunks::<2>()
		.0
		.iter()
		.map(|c| u16::from_le_bytes(*c))
		.collect();
	// rgbxchNums holds the 1-based positions in the template of each level-number placeholder; the
	// character there is the level (0-8) whose number goes in.
	let mut text = String::new();
	for (i, &unit) in units.iter().enumerate() {
		if placeholders.contains(&(i + 1)) && unit < 9 {
			text.push('%');
			text.push_str(&(unit + 1).to_string());
		} else {
			text.extend(char::decode_utf16([unit]).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)));
		}
	}
	let level = Level { format: number_format(nfc).to_string(), text, start, legal };
	Some((level, xst_at + 2 + cch * 2))
}

/// `PlfLfo`: the list instances (1-based, which is what `sprmPIlfo` counts), each naming its
/// definition by `lsid`, followed by each one's level overrides.
fn read_plf_lfo(table: &[u8], (fc, lcb): (usize, usize)) -> Option<HashMap<String, Instance>> {
	table.get(fc..fc.checked_add(lcb)?)?;
	let count = usize::try_from(i32::from_le_bytes(table.get(fc..fc + 4)?.try_into().ok()?)).ok()?;
	let mut instances = HashMap::new();
	let mut data_at = fc + 4 + count * LFO_SIZE;
	for i in 0..count {
		let lfo = table.get(fc + 4 + i * LFO_SIZE..fc + 4 + (i + 1) * LFO_SIZE)?;
		let lsid = i32::from_le_bytes(lfo[0..4].try_into().ok()?);
		let overrides = usize::from(lfo[12]);
		let mut instance = Instance { abstract_id: lsid.to_string(), ..Instance::default() };
		// LFOData: a cp, then one LFOLVL per override, each possibly carrying a whole LVL.
		data_at += 4;
		for _ in 0..overrides {
			let lfolvl = table.get(data_at..data_at + 8)?;
			let start = i32::from_le_bytes(lfolvl[0..4].try_into().ok()?);
			let flags = u32::from_le_bytes(lfolvl[4..8].try_into().ok()?);
			let ilvl = usize::try_from(flags & 0xF).ok()?;
			data_at += 8;
			if flags & 0x10 != 0 {
				instance.start_overrides.insert(ilvl, start);
			}
			if flags & 0x20 != 0 {
				let (level, next) = read_lvl(table, data_at)?;
				instance.level_overrides.insert(ilvl, level);
				data_at = next;
			}
		}
		instances.insert((i + 1).to_string(), instance);
	}
	Some(instances)
}

/// [MS-DOC] number format codes, in the terms `numbering.xml` uses.
const fn number_format(nfc: u8) -> &'static str {
	match nfc {
		1 => "upperRoman",
		2 => "lowerRoman",
		3 => "upperLetter",
		4 => "lowerLetter",
		22 => "decimalZero",
		23 => "bullet",
		0xFF => "none",
		_ => "decimal",
	}
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

fn read_u32(data: &[u8], at: usize) -> Option<u32> {
	data.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests;
