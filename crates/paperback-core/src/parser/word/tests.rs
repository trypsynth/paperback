use super::WordParser;
use crate::{document::ParserContext, parser::Parser, util::test_support::TempDir};

fn read(name: &str, bytes: &[u8]) -> String {
	let dir = TempDir::new("word-routing");
	let path = dir.write_str(name, bytes);
	WordParser.parse(&ParserContext::new(path)).unwrap().buffer.content
}

#[test]
fn a_word_for_dos_file_named_doc_is_read_as_write() {
	let text = b"Word for DOS text.\r\n";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0x31;
	file[1] = 0xBE;
	file[0x0E..0x12].copy_from_slice(&u32::try_from(128 + text.len()).unwrap().to_le_bytes());
	file[128..].copy_from_slice(text);
	assert_eq!(read("dos.doc", &file), "Word for DOS text.\n");
}

#[test]
fn a_word_for_windows_2_file_named_doc_is_read_as_one() {
	let text = b"Word 2 text.\r";
	let mut file = vec![0u8; 0x180];
	file[0..2].copy_from_slice(&0xA5DBu16.to_le_bytes());
	file[0x18..0x1C].copy_from_slice(&0x180u32.to_le_bytes());
	file[0x1C..0x20].copy_from_slice(&u32::try_from(0x180 + text.len()).unwrap().to_le_bytes());
	file.extend_from_slice(text);
	assert_eq!(read("winword.doc", &file).trim_end(), "Word 2 text.");
}
