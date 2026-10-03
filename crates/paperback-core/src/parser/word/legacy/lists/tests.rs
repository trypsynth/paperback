use super::{number_format, read_lvl, read_plf_lfo, sprms};

/// A level as Word writes one: LVLF, then its paragraph and character modifiers, then the label
/// template. `template` uses 0-8 for the level placeholders, at the 1-based positions in `numbers`.
fn lvl(start: i32, nfc: u8, legal: bool, numbers: &[u8], template: &[u16], grpprl_papx: &[u8]) -> Vec<u8> {
	let mut b = Vec::new();
	b.extend_from_slice(&start.to_le_bytes());
	b.push(nfc);
	b.push(if legal { 0b100 } else { 0 });
	let mut rgbxch = [0u8; 9];
	rgbxch[..numbers.len()].copy_from_slice(numbers);
	b.extend_from_slice(&rgbxch);
	b.push(0); // ixchFollow
	b.extend_from_slice(&[0; 8]); // dxaIndentSav, unused
	b.push(0); // cbGrpprlChpx
	b.push(u8::try_from(grpprl_papx.len()).unwrap());
	b.extend_from_slice(&[0, 0]); // ilvlRestartLim, grfhic
	b.extend_from_slice(grpprl_papx);
	b.extend_from_slice(&u16::try_from(template.len()).unwrap().to_le_bytes());
	for unit in template {
		b.extend_from_slice(&unit.to_le_bytes());
	}
	b
}

#[test]
fn a_level_template_turns_its_placeholders_into_level_references() {
	// "%1.%2." as Word stores it: level numbers 0 and 1 at positions 1 and 3.
	let bytes = lvl(1, 0, false, &[1, 3], &[0, u16::from(b'.'), 1, u16::from(b'.')], &[0x0F, 0x84, 0x68, 0x01]);
	let (level, next) = read_lvl(&bytes, 0).expect("level reads");
	assert_eq!((level.format.as_str(), level.text.as_str(), level.start, level.legal), ("decimal", "%1.%2.", 1, false));
	assert_eq!(next, bytes.len(), "the paragraph modifiers and template are stepped over");
}

#[test]
fn a_bullet_level_keeps_its_symbol_and_a_legal_level_says_so() {
	let bytes = lvl(1, 23, false, &[], &[0xF0B7], &[]);
	let (level, _) = read_lvl(&bytes, 0).expect("level reads");
	assert_eq!((level.format.as_str(), level.text.as_str()), ("bullet", "\u{F0B7}"));
	let bytes = lvl(3, 2, true, &[1], &[0, u16::from(b')')], &[]);
	let (level, _) = read_lvl(&bytes, 0).expect("level reads");
	assert_eq!((level.format.as_str(), level.text.as_str(), level.start, level.legal), ("lowerRoman", "%1)", 3, true));
}

#[test]
fn list_instances_are_numbered_from_one_and_carry_their_restarts() {
	// Two LFOs: the first plain, the second restarting level 0 at 5.
	let mut b = Vec::new();
	b.extend_from_slice(&2i32.to_le_bytes());
	for (lsid, overrides) in [(111i32, 0u8), (222, 1)] {
		b.extend_from_slice(&lsid.to_le_bytes());
		b.extend_from_slice(&[0; 8]);
		b.extend_from_slice(&[overrides, 0, 0, 0]);
	}
	b.extend_from_slice(&0u32.to_le_bytes()); // LFOData of the first: cp only
	b.extend_from_slice(&0u32.to_le_bytes()); // second: cp, then one LFOLVL
	b.extend_from_slice(&5i32.to_le_bytes());
	b.extend_from_slice(&0x10u32.to_le_bytes()); // iLvl 0, fStartAt
	let instances = read_plf_lfo(&b, (0, b.len())).expect("instances read");
	assert_eq!(instances["1"].abstract_id, "111");
	assert_eq!(instances["2"].abstract_id, "222");
	assert_eq!(instances["2"].start_overrides.get(&0), Some(&5));
}

#[test]
fn property_modifiers_are_sized_from_the_sprm_and_unsizable_ones_stop_the_walk() {
	// sprmPIlvl (1 byte) 2, sprmPIlfo (2 bytes) 3, then sprmPChgTabs, which has a size rule of
	// its own and stops the walk rather than being misread.
	let grpprl = [0x0A, 0x26, 0x02, 0x0B, 0x46, 0x03, 0x00, 0x15, 0xC6, 0x03, 9, 9, 9];
	let found: Vec<_> = sprms(&grpprl).iter().map(|(s, o)| (*s, o.to_vec())).collect();
	assert_eq!(found, vec![(0x260A, vec![2]), (0x460B, vec![3, 0])]);
	// A truncated operand is not read past the end.
	assert!(sprms(&[0x0B, 0x46, 0x03]).is_empty());
}

#[test]
fn number_format_codes_map_to_numbering_xml_names() {
	assert_eq!(number_format(0), "decimal");
	assert_eq!(number_format(2), "lowerRoman");
	assert_eq!(number_format(4), "lowerLetter");
	assert_eq!(number_format(23), "bullet");
	assert_eq!(number_format(0xFF), "none");
	assert_eq!(number_format(57), "decimal", "formats with no equivalent fall back to decimal");
}
