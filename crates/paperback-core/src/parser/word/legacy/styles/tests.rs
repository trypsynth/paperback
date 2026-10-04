use super::{ISTD_NIL, ParagraphStyle, paragraph_styles};

fn heading_levels(word_document: &[u8], table: &[u8]) -> Vec<Option<u8>> {
	paragraph_styles(word_document, table).iter().map(|style| style.heading).collect()
}

/// A paragraph style as Word writes one with an 18-byte Stdf: sti, kind and base, one UPX
/// (paragraph properties) and no character properties to speak of, its name, then the PAPX.
fn style(sti: u16, base: u16, name: &str, papx_sprms: &[u8]) -> Vec<u8> {
	let mut std = Vec::new();
	std.extend_from_slice(&sti.to_le_bytes());
	std.extend_from_slice(&((base << 4) | 1).to_le_bytes());
	std.extend_from_slice(&2u16.to_le_bytes()); // cupx 2
	std.extend_from_slice(&[0; 12]); // rest of StdfBase and StdfPost2000
	let name: Vec<u16> = name.encode_utf16().collect();
	std.extend_from_slice(&u16::try_from(name.len()).unwrap().to_le_bytes());
	for unit in name {
		std.extend_from_slice(&unit.to_le_bytes());
	}
	std.extend_from_slice(&[0, 0]);
	let cb_upx = u16::try_from(2 + papx_sprms.len()).unwrap();
	std.extend_from_slice(&cb_upx.to_le_bytes());
	std.extend_from_slice(&[0, 0]); // istd
	std.extend_from_slice(papx_sprms);
	if cb_upx % 2 == 1 {
		std.push(0);
	}
	std.extend_from_slice(&[0, 0]); // empty character UPX
	std
}

/// A FIB pointing at a stylesheet of `styles` (None for an empty slot) at the start of the table.
fn stylesheet(styles: &[Option<Vec<u8>>]) -> (Vec<u8>, Vec<u8>) {
	let mut table = Vec::new();
	table.extend_from_slice(&18u16.to_le_bytes()); // cbStshi
	table.extend_from_slice(&u16::try_from(styles.len()).unwrap().to_le_bytes());
	table.extend_from_slice(&18u16.to_le_bytes()); // cbSTDBaseInFile
	table.extend_from_slice(&[0; 14]);
	for std in styles {
		let std = std.as_deref().unwrap_or_default();
		table.extend_from_slice(&u16::try_from(std.len()).unwrap().to_le_bytes());
		table.extend_from_slice(std);
	}
	let mut word_document = vec![0u8; 0x200];
	word_document[0xA6..0xAA].copy_from_slice(&u32::try_from(table.len()).unwrap().to_le_bytes());
	(word_document, table)
}

#[test]
fn built_in_heading_styles_are_headings_whatever_they_are_called() {
	let (word_document, table) = stylesheet(&[
		Some(style(0, ISTD_NIL, "Normal", &[])),
		Some(style(1, 0, "Überschrift 1", &[])),
		Some(style(3, 0, "heading 3", &[])),
		None,
		Some(style(0x0FFE, 0, "Body Text", &[])),
	]);
	assert_eq!(heading_levels(&word_document, &table), vec![None, Some(1), Some(3), None, None]);
}

#[test]
fn an_outline_level_makes_a_heading_and_is_inherited_by_styles_based_on_it() {
	let (word_document, table) = stylesheet(&[
		Some(style(0, ISTD_NIL, "Normal", &[])),
		Some(style(0x0FFE, 0, "Chapter", &[0x40, 0x26, 0])),
		Some(style(0x0FFE, 1, "Chapter Unnumbered", &[0x0A, 0x26, 0])),
		Some(style(2, 0, "Heading 2 as body", &[0x40, 0x26, 9])),
		Some(style(0x0FFE, 4, "Loops", &[])),
	]);
	assert_eq!(heading_levels(&word_document, &table), vec![None, Some(1), Some(1), None, None]);
}

#[test]
fn a_missing_or_truncated_stylesheet_has_no_headings() {
	assert!(heading_levels(&[0; 0x200], &[]).is_empty());
	let (word_document, mut table) = stylesheet(&[Some(style(1, ISTD_NIL, "Heading 1", &[]))]);
	table.truncate(table.len() - 4);
	assert!(heading_levels(&word_document, &table).is_empty());
}

#[test]
fn a_style_s_list_is_inherited_and_a_based_on_style_can_change_the_level() {
	let (word_document, table) = stylesheet(&[
		Some(style(0, ISTD_NIL, "Normal", &[])),
		// Heading 1 numbered from list instance 2, level 0; Heading 2 based on it, at level 1.
		Some(style(1, 0, "heading 1", &[0x0B, 0x46, 2, 0, 0x0A, 0x26, 0])),
		Some(style(2, 1, "heading 2", &[0x0A, 0x26, 1])),
	]);
	let styles = paragraph_styles(&word_document, &table);
	assert_eq!(styles[0], ParagraphStyle::default());
	assert_eq!(styles[1], ParagraphStyle { heading: Some(1), ilfo: Some(2), ilvl: Some(0) });
	assert_eq!(styles[2], ParagraphStyle { heading: Some(2), ilfo: Some(2), ilvl: Some(1) });
}
