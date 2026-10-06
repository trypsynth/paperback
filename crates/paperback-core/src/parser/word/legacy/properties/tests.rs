use super::{ParagraphProperties, Paragraphs, sprms};

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

/// A `WordDocument` stream whose second page is an FKP with one paragraph run (file offsets
/// 100-200) carrying `papx`, and a table stream whose bin table points that range at it.
fn one_paragraph(papx: &[u8]) -> (Vec<u8>, Vec<u8>) {
	let mut word_document = vec![0u8; 1024];
	word_document[0x102..0x106].copy_from_slice(&0u32.to_le_bytes());
	word_document[0x106..0x10A].copy_from_slice(&12u32.to_le_bytes());
	let fkp = &mut word_document[512..];
	fkp[0..4].copy_from_slice(&100u32.to_le_bytes());
	fkp[4..8].copy_from_slice(&200u32.to_le_bytes());
	fkp[8] = 10; // BxPap: the PAPX is 10 words (20 bytes) in
	fkp[20..20 + papx.len()].copy_from_slice(papx);
	fkp[511] = 1; // crun
	let mut table = Vec::new();
	for value in [100u32, 200, 1] {
		table.extend_from_slice(&value.to_le_bytes());
	}
	(word_document, table)
}

#[test]
fn a_paragraph_s_style_list_and_outline_level_are_read_from_its_formatting_page() {
	// cb 5: istd 3, sprmPOutLvl 2, sprmPIlfo 1.
	let (word_document, table) = one_paragraph(&[5, 3, 0, 0x40, 0x26, 2, 0x0B, 0x46, 1, 0]);
	let paragraphs = Paragraphs::read(&word_document, &table).expect("bin table reads");
	assert_eq!(
		paragraphs.at(&word_document, 150),
		Some(ParagraphProperties { istd: 3, ilfo: Some(1), ilvl: None, outline: Some(2) })
	);
	assert_eq!(paragraphs.at(&word_document, 250), None, "outside every bin");
}

#[test]
fn a_paragraph_with_no_papx_has_the_default_style() {
	let (mut word_document, table) = one_paragraph(&[]);
	word_document[512 + 8] = 0;
	let paragraphs = Paragraphs::read(&word_document, &table).expect("bin table reads");
	assert_eq!(paragraphs.at(&word_document, 150), Some(ParagraphProperties::default()));
}
