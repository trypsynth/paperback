use super::write_binary_text;

#[test]
fn write_binary_text_reads_the_run_to_fcmac() {
	let text = b"Hello from Windows Write.";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0x31;
	file[1] = 0xBE; // magic 0xBE31
	let fc_mac = u32::try_from(128 + text.len()).unwrap();
	file[0x0E..0x12].copy_from_slice(&fc_mac.to_le_bytes());
	file[128..].copy_from_slice(text);
	assert_eq!(write_binary_text(&file).as_deref(), Some("Hello from Windows Write."));
}

#[test]
fn the_ole_write_magic_is_accepted() {
	let text = b"With objects.";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0x32;
	file[1] = 0xBE; // magic 0xBE32
	file[0x0E..0x12].copy_from_slice(&u32::try_from(128 + text.len()).unwrap().to_le_bytes());
	file[128..].copy_from_slice(text);
	assert_eq!(write_binary_text(&file).as_deref(), Some("With objects."));
}

#[test]
fn non_write_bytes_are_declined_so_the_caller_reads_them_as_text() {
	assert!(write_binary_text(b"Just some notes saved with a .wri name.").is_none());
	let mut file = vec![0u8; 200];
	file[0] = 0x31;
	file[1] = 0xBE;
	file[0x0E..0x12].copy_from_slice(&9_999_999u32.to_le_bytes());
	assert!(write_binary_text(&file).is_none());
	assert!(write_binary_text(b"\x31\xBE").is_none());
}

#[test]
fn a_picture_paragraph_is_read_as_an_image_rather_than_its_bytes() {
	let text: &[u8] = b"Before\r\n\x01\x02\xFFbitmap\r\nAfter\r\n";
	let picture = (128 + 8, 128 + 8 + 11);
	let mut file = vec![0u8; 256 + 128];
	file[0] = 0x31;
	file[1] = 0xBE;
	file[0x0E..0x12].copy_from_slice(&u32::try_from(128 + text.len()).unwrap().to_le_bytes());
	file[0x12..0x14].copy_from_slice(&2u16.to_le_bytes());
	file[128..128 + text.len()].copy_from_slice(text);
	let page = &mut file[256..384];
	page[0..4].copy_from_slice(&128u32.to_le_bytes());
	let limits = [picture.0, picture.1, 128 + text.len()];
	for (index, limit) in limits.into_iter().enumerate() {
		let run = &mut page[4 + index * 6..10 + index * 6];
		run[0..4].copy_from_slice(&u32::try_from(limit).unwrap().to_le_bytes());
		let property: u16 = if index == 1 { 30 } else { 0xFFFF };
		run[4..6].copy_from_slice(&property.to_le_bytes());
	}
	page[4 + 30] = 17;
	page[4 + 30 + 1 + 16] = 0x10;
	page[127] = 3;
	assert_eq!(write_binary_text(&file).as_deref(), Some("Before\n[Image]\nAfter\n"));
}

#[test]
fn a_stray_symbol_byte_in_western_text_does_not_swallow_the_next_letter() {
	let text: &[u8] =
		b"\t\x8dSend disk regardless\t\x8dRegular mail\r\nPlease print this form and mail it with your payment.\r\n";
	let mut file = vec![0u8; 128 + text.len()];
	file[0] = 0x31;
	file[1] = 0xBE;
	file[0x0E..0x12].copy_from_slice(&u32::try_from(128 + text.len()).unwrap().to_le_bytes());
	file[128..].copy_from_slice(text);
	let read = write_binary_text(&file).unwrap();
	assert!(read.contains("Send disk regardless") && read.contains("Regular mail"), "{read}");
}
