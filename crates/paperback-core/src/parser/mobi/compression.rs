//! PalmDOC (compression mode 2) decompression, plus the trailing-byte-entry size decoder
//! used to strip MOBI's optional multibyte/index trailing data off the end of each raw
//! content record before it's decompressed.

pub(super) fn get_trailing_size(data: &[u8]) -> usize {
	let mut size = 0usize;
	let mut pos = data.len() - 1;
	let mut shift = 0u32;
	loop {
		let b = data[pos];
		if shift < 32 {
			size |= ((b & 0x7f) as usize) << shift;
		}
		if b & 0x80 != 0 || pos == 0 {
			break;
		}
		pos -= 1;
		shift += 7;
	}
	size
}

pub(super) fn decompress_palmdoc(data: &[u8]) -> Vec<u8> {
	let mut pos: usize = 0;
	let mut text: Vec<u8> = Vec::new();
	let mut prev: Option<u8> = None;
	while pos < data.len() {
		let byte = data[pos];
		pos += 1;
		match byte {
			new if prev.is_some() => {
				let old = prev.take().unwrap();
				let token = u16::from_be_bytes([old, new]) & 0x3fff;
				let offset = (token >> 3) as usize;
				let len = ((token & 0x0007) + 3) as usize;
				if offset == 0 || offset > text.len() {
					continue;
				}
				let start = text.len() - offset;
				for j in 0..len {
					let src = start + j;
					if src >= text.len() {
						break;
					}
					let b = text[src];
					text.push(b);
				}
			}
			0x00 | 0x09..=0x7f => text.push(byte),
			0x01..=0x08 => {
				let count = byte as usize;
				if pos + count <= data.len() {
					text.extend_from_slice(&data[pos..pos + count]);
					pos += count;
				}
			}
			0x80..=0xbf => {
				if pos >= data.len() {
					break;
				}
				prev = Some(byte);
			}
			_ => {
				text.push(b' ');
				text.push(byte ^ 0x80);
			}
		}
	}
	text
}
