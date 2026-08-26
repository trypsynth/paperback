//! Variable-width integer and base-32 decoding shared by MOBI's fragment-offset table
//! ([`super::links`]) and NCX index ([`super::toc`]) parsing, both of which pack values
//! using these same encodings.

pub(super) fn decode_vwi(data: &[u8], mut pos: usize) -> (usize, usize) {
	let mut val: usize = 0;
	while pos < data.len() {
		let b = data[pos];
		pos += 1;
		val = (val << 7) | (b & 0x7F) as usize;
		if (b & 0x80) != 0 {
			break;
		}
	}
	(val, pos)
}

pub(super) fn base32_decode(s: &str) -> usize {
	let mut val = 0;
	for c in s.chars() {
		val = (val << 5) | (c.to_digit(32).unwrap_or(0) as usize);
	}
	val
}
