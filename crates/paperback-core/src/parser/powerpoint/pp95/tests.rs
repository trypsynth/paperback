use super::{CONTAINER_MARK, DOCUMENT, NOTES, SLIDE, TEXT, TEXT_MARK, is_pp95, slide_texts};

fn record(record_type: u32, mark: u32, last: u32, body: &[u8]) -> Vec<u8> {
	let mut bytes = Vec::new();
	for field in [record_type, mark, u32::try_from(body.len()).unwrap(), last] {
		bytes.extend_from_slice(&field.to_le_bytes());
	}
	bytes.extend_from_slice(body);
	bytes
}

fn container(record_type: u32, children: &[Vec<u8>]) -> Vec<u8> {
	record(record_type, CONTAINER_MARK, 0, &children.concat())
}

fn text(content: &str) -> Vec<u8> {
	record(TEXT, 0, TEXT_MARK, content.as_bytes())
}

#[test]
fn slides_are_read_without_their_notes_or_the_masters() {
	let first =
		container(SLIDE, &[container(0xBB8, &[text("Study Scope")]), container(NOTES, &[text("Speaker notes")])]);
	let second = container(SLIDE, &[container(0xBB8, &[text("Speed limits"), text("Rural roads")])]);
	let master = container(0x3F8, &[text("Click to edit Master title style")]);
	let stream = container(DOCUMENT, &[container(0x3E8, &[container(0x7D0, &[first, second, master])])]);
	assert!(is_pp95(&stream));
	assert_eq!(slide_texts(&stream), ["Study Scope", "Speed limits\rRural roads"]);
}

#[test]
fn a_powerpoint_97_stream_is_not_taken_for_95() {
	assert!(!is_pp95(&[0x0F, 0x00, 0xE8, 0x03, 0x10, 0x00, 0x00, 0x00]));
}
