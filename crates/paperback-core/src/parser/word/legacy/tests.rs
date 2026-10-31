use super::{
	extract_word6_text, looks_like_text_content, normalize_doc_text, parse_doc_clx, parse_doc_piece_table, winword_text,
};

#[test]
fn parse_doc_piece_table_extracts_ansi_text() {
	let mut word_document = vec![0u8; 64];
	word_document[16..21].copy_from_slice(b"Hello");
	let mut piece_table = Vec::new();
	piece_table.extend_from_slice(&0u32.to_le_bytes());
	piece_table.extend_from_slice(&5u32.to_le_bytes());
	piece_table.extend_from_slice(&0u16.to_le_bytes());
	let fc_raw = 0x4000_0000u32 | 32u32;
	piece_table.extend_from_slice(&fc_raw.to_le_bytes());
	piece_table.extend_from_slice(&0u16.to_le_bytes());
	let text = parse_doc_piece_table(&piece_table, &word_document).expect("text");
	assert_eq!(text, "Hello");
}

#[test]
fn parse_doc_clx_extracts_piece_table_text() {
	let mut word_document = vec![0u8; 64];
	word_document[16..21].copy_from_slice(b"Hello");
	let mut piece_table = Vec::new();
	piece_table.extend_from_slice(&0u32.to_le_bytes());
	piece_table.extend_from_slice(&5u32.to_le_bytes());
	piece_table.extend_from_slice(&0u16.to_le_bytes());
	let fc_raw = 0x4000_0000u32 | 32u32;
	piece_table.extend_from_slice(&fc_raw.to_le_bytes());
	piece_table.extend_from_slice(&0u16.to_le_bytes());
	let mut clx = Vec::new();
	clx.push(0x02);
	clx.extend_from_slice(&u32::try_from(piece_table.len()).unwrap().to_le_bytes());
	clx.extend_from_slice(&piece_table);
	let text = parse_doc_clx(&clx, &word_document).expect("clx text");
	assert_eq!(text, "Hello");
}

#[test]
fn normalize_doc_text_cleans_control_markers() {
	let text = "A\r\nB\u{13}\u{14}C\u{15}\n\n\nD";
	assert_eq!(normalize_doc_text(text), "A\nBC\n\nD");
}

#[test]
fn looks_like_text_content_detects_textual_data() {
	assert!(looks_like_text_content("Manual Title\nLine 2\nLine 3"));
	assert!(!looks_like_text_content("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}"));
}

#[test]
fn extract_word6_text_reads_the_fcmin_to_fcmac_run() {
	use super::extract_word6_text;
	let text = b"Harold Koplow, Gainesville FL.";
	let fc_min = 0x300usize;
	let mut wd = vec![0u8; fc_min + text.len()];
	// nFib 101 (Word 6) and the magic are not read by this function, only fcMin/fcMac are.
	wd[0x18..0x1C].copy_from_slice(&u32::try_from(fc_min).unwrap().to_le_bytes());
	wd[0x1C..0x20].copy_from_slice(&u32::try_from(fc_min + text.len()).unwrap().to_le_bytes());
	wd[fc_min..].copy_from_slice(text);
	assert_eq!(extract_word6_text(&wd).trim(), "Harold Koplow, Gainesville FL.");
}

#[test]
fn extract_word6_text_survives_nonsense_bounds() {
	use super::extract_word6_text;
	let mut wd = vec![0u8; 0x40];
	// fcMac before fcMin.
	wd[0x18..0x1C].copy_from_slice(&0x30u32.to_le_bytes());
	wd[0x1C..0x20].copy_from_slice(&0x10u32.to_le_bytes());
	let _ = extract_word6_text(&wd);
}

fn winword2(text: &[u8]) -> Vec<u8> {
	let mut file = vec![0u8; 0x180];
	file[0..2].copy_from_slice(&0xA5DBu16.to_le_bytes());
	file[0x18..0x1C].copy_from_slice(&0x180u32.to_le_bytes());
	file[0x1C..0x20].copy_from_slice(&u32::try_from(0x180 + text.len()).unwrap().to_le_bytes());
	file.extend_from_slice(text);
	file
}

#[test]
fn a_word_for_windows_2_file_reads_its_text_run() {
	assert_eq!(winword_text(&winword2(b"Hello\rWorld\r")).as_deref(), Some("Hello\rWorld\r"));
}

#[test]
fn a_fast_saved_word_for_windows_file_reads_its_pieces_in_order() {
	let mut file = winword2(b"World\rHello ");
	file[0x0A] = 0x04;
	let clx_at = u32::try_from(file.len()).unwrap();
	let mut clx = vec![2u8];
	clx.extend_from_slice(&28u16.to_le_bytes());
	for cp in [0u32, 6, 12] {
		clx.extend_from_slice(&cp.to_le_bytes());
	}
	for fc in [0x180u32 + 6, 0x180] {
		clx.extend_from_slice(&0u16.to_le_bytes());
		clx.extend_from_slice(&fc.to_le_bytes());
		clx.extend_from_slice(&0u16.to_le_bytes());
	}
	file.extend_from_slice(&clx);
	file[0x11E..0x122].copy_from_slice(&clx_at.to_le_bytes());
	file[0x122..0x124].copy_from_slice(&u16::try_from(clx.len()).unwrap().to_le_bytes());
	assert_eq!(winword_text(&file).as_deref(), Some("Hello World\r"));
}

#[test]
fn a_fast_saved_file_whose_pieces_make_no_sense_falls_back_to_its_run() {
	let mut file = winword2(b"Last full save\r");
	file[0x0A] = 0x04;
	file[0x11E..0x122].copy_from_slice(&9_999_999u32.to_le_bytes());
	assert_eq!(winword_text(&file).as_deref(), Some("Last full save\r"));
}

#[test]
fn a_word_for_windows_file_reads_its_text_and_footnotes_but_not_its_headers() {
	let mut file = winword2(b"Body\rNote\rRunning head\r");
	file[0x34..0x38].copy_from_slice(&5u32.to_le_bytes());
	file[0x38..0x3C].copy_from_slice(&5u32.to_le_bytes());
	file[0x3C..0x40].copy_from_slice(&13u32.to_le_bytes());
	assert_eq!(winword_text(&file).as_deref(), Some("Body\rNote\r"));
}

#[test]
fn a_binary_header_ahead_of_mostly_text_is_not_plain_text() {
	let mut content = "\0".repeat(200);
	content.push_str(&"Readable words follow the header. ".repeat(100));
	assert!(!looks_like_text_content(&content));
}

#[test]
fn a_word_for_the_macintosh_file_reads_its_big_endian_run_as_mac_roman() {
	let mut file = vec![0u8; 0x100];
	file[0] = 0xFE;
	file[1] = 0x37;
	file[0x14..0x18].copy_from_slice(&0x100u32.to_be_bytes());
	file[0x18..0x1C].copy_from_slice(&0x10Bu32.to_be_bytes());
	file.extend_from_slice(b"It\xd5s a map\r");
	assert_eq!(winword_text(&file).as_deref(), Some("It\u{2019}s a map\r"));
}

#[test]
fn a_word_6_unicode_file_reads_its_run_as_utf16() {
	let text: Vec<u8> = "Date Logger\r".encode_utf16().flat_map(u16::to_le_bytes).collect();
	let mut stream = vec![0u8; 0x200];
	stream[0..2].copy_from_slice(&0xA697u16.to_le_bytes());
	stream[0x18..0x1C].copy_from_slice(&0x200u32.to_le_bytes());
	stream[0x1C..0x20].copy_from_slice(&u32::try_from(0x200 + text.len()).unwrap().to_le_bytes());
	stream.extend_from_slice(&text);
	assert_eq!(extract_word6_text(&stream), "Date Logger\r");
}
