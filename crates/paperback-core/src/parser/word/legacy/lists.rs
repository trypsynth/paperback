//! Word 97-2003 automatic numbering: the `1.`, `a)` and bullets a `.doc` generates rather than
//! stores in its text, read from the binary structures [MS-DOC] describes.
//!
//! * A paragraph is in a list when its properties (see [`super::properties`]) carry `sprmPIlfo`
//!   (which list instance) and `sprmPIlvl` (which level).
//! * The list definitions (`PlfLst`) give each list's levels: number format, start value and the
//!   label template, in which a character 0-8 stands for that level's current number.
//! * The list instances (`PlfLfo`) point paragraphs at a definition, optionally restarting levels.
//!
//! These are translated into the same model `numbering.xml` is read into, so the counting and the
//! label formatting are shared with `.docx`.

use std::collections::HashMap;

use super::{
	super::numbering::{Instance, Level, ListLabel, Numbering},
	properties::fc_lcb,
};

/// Offsets in the FIB of the (fc, lcb) pairs this needs, in `FibRgFcLcb97`.
const FIB_FC_PLF_LST: usize = 0x02E2;
const FIB_FC_PLF_LFO: usize = 0x02EA;
const LSTF_SIZE: usize = 28;
const LVLF_SIZE: usize = 28;
const LFO_SIZE: usize = 16;

/// A document's lists: the numbering its list paragraphs refer to.
pub(super) struct DocLists {
	numbering: Numbering,
}

impl DocLists {
	/// Reads the list definitions and instances. `None` when the document has no lists or they
	/// cannot be read, in which case it is read without them.
	pub(super) fn read(word_document: &[u8], table: &[u8]) -> Option<Self> {
		let lists = read_plf_lst(table, fc_lcb(word_document, FIB_FC_PLF_LST)?)?;
		let instances = read_plf_lfo(table, fc_lcb(word_document, FIB_FC_PLF_LFO)?)?;
		(!lists.is_empty() && !instances.is_empty())
			.then(|| Self { numbering: Numbering::from_definitions(lists, instances) })
	}

	/// The label for the next paragraph in list instance `ilfo` at level `ilvl`, advancing the
	/// count of that list.
	pub(super) fn label(&mut self, ilfo: i16, ilvl: u8) -> Option<ListLabel> {
		self.numbering.label_of(&ilfo.to_string(), usize::from(ilvl))
	}
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

#[cfg(test)]
mod tests;
