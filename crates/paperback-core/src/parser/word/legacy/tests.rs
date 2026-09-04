use super::{looks_like_text_content, normalize_doc_text, parse_doc_clx, parse_doc_piece_table};

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
	clx.extend_from_slice(&(piece_table.len() as u32).to_le_bytes());
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
