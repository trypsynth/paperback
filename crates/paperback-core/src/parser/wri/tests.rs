use super::{is_rtf, write_binary_text};

#[test]
fn rtf_is_recognised_past_a_bom_or_whitespace() {
	assert!(is_rtf(br"{\rtf1\ansi hello}"));
	assert!(is_rtf(b"\xEF\xBB\xBF{\\rtf1 x}"), "a byte order mark before the brace");
	assert!(is_rtf(b"  \r\n{\\rtf1 x}"), "leading whitespace");
	assert!(!is_rtf(b"Just plain text."));
	assert!(!is_rtf(b"\x31\xBE\x00\x00"), "the write binary magic is not rtf");
}

/// A genuine Windows Write binary file: the magic, `fcMac` at offset 14 pointing past the text,
/// and the text starting at byte 128.
#[test]
fn write_binary_text_reads_the_run_to_fcmac() {
	let text = b"Hello from Windows Write.";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0xBE;
	file[1] = 0x31; // magic 0x31BE
	let fc_mac = u32::try_from(128 + text.len()).unwrap();
	file[0x0E..0x12].copy_from_slice(&fc_mac.to_le_bytes());
	file[128..].copy_from_slice(text);
	assert_eq!(write_binary_text(&file).as_deref(), Some("Hello from Windows Write."));
}

#[test]
fn the_ole_write_magic_is_accepted() {
	let text = b"With objects.";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0xBE;
	file[1] = 0x32; // magic 0x32BE
	file[0x0E..0x12].copy_from_slice(&u32::try_from(128 + text.len()).unwrap().to_le_bytes());
	file[128..].copy_from_slice(text);
	assert_eq!(write_binary_text(&file).as_deref(), Some("With objects."));
}

#[test]
fn non_write_bytes_are_declined_so_the_caller_reads_them_as_text() {
	// No magic: a plain-text .wri.
	assert!(write_binary_text(b"Just some notes saved with a .wri name.").is_none());
	// The magic but a nonsense fcMac (past the file): not treated as Write.
	let mut file = vec![0u8; 200];
	file[0] = 0xBE;
	file[1] = 0x31;
	file[0x0E..0x12].copy_from_slice(&9_999_999u32.to_le_bytes());
	assert!(write_binary_text(&file).is_none());
	// Too short to hold a header at all.
	assert!(write_binary_text(b"\xBE\x31").is_none());
}
