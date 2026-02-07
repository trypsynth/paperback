use std::str;

use encoding_rs::{UTF_16BE, UTF_16LE, WINDOWS_1252};

#[must_use]
pub fn convert_to_utf8(input: &[u8]) -> String {
	if input.len() >= 4 {
		// UTF-32 LE BOM
		if input[0] == 0xFF && input[1] == 0xFE && input[2] == 0x00 && input[3] == 0x00 {
			return decode_utf32_le(&input[4..]);
		}
		// UTF-32 BE BOM
		if input[0] == 0x00 && input[1] == 0x00 && input[2] == 0xFE && input[3] == 0xFF {
			return decode_utf32_be(&input[4..]);
		}
	}
	if input.len() >= 3 {
		// UTF-8 BOM
		if input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
			return String::from_utf8_lossy(&input[3..]).to_string();
		}
	}
	if input.len() >= 2 {
		// UTF-16 LE BOM
		if input[0] == 0xFF && input[1] == 0xFE {
			let (decoded, _, _) = UTF_16LE.decode(&input[2..]);
			return decoded.to_string();
		}
		// UTF-16 BE BOM
		if input[0] == 0xFE && input[1] == 0xFF {
			let (decoded, _, _) = UTF_16BE.decode(&input[2..]);
			return decoded.to_string();
		}
	}
	// UTF-8 without BOM
	if let Ok(s) = str::from_utf8(input) {
		return s.to_string();
	}
	// UTF-16 without BOM (only if data looks like UTF-16)
	if looks_like_utf16(input) {
		let (decoded, encoding, had_errors) = UTF_16LE.decode(input);
		if !had_errors && encoding == UTF_16LE {
			return decoded.to_string();
		}
		let (decoded, encoding, had_errors) = UTF_16BE.decode(input);
		if !had_errors && encoding == UTF_16BE {
			return decoded.to_string();
		}
	}
	// Windows-1252
	let (decoded, _, _) = WINDOWS_1252.decode(input);
	if decoded.chars().any(|c| !c.is_control() || c.is_whitespace()) {
		return decoded.to_string();
	}
	// Give up
	String::from_utf8_lossy(input).to_string()
}

fn decode_utf32_le(input: &[u8]) -> String {
	input
		.chunks_exact(4)
		.filter_map(|chunk| {
			let code_point = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
			char::from_u32(code_point)
		})
		.collect()
}

fn decode_utf32_be(input: &[u8]) -> String {
	input
		.chunks_exact(4)
		.filter_map(|chunk| {
			let code_point = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
			char::from_u32(code_point)
		})
		.collect()
}

fn looks_like_utf16(input: &[u8]) -> bool {
	if input.len() < 2 {
		return false;
	}
	let mut even_nulls = 0usize;
	let mut odd_nulls = 0usize;
	for (idx, byte) in input.iter().enumerate() {
		if *byte == 0 {
			if idx % 2 == 0 {
				even_nulls += 1;
			} else {
				odd_nulls += 1;
			}
		}
	}
	let total_nulls = even_nulls + odd_nulls;
	if total_nulls == 0 {
		return false;
	}
	let pairs = input.len() / 2;
	total_nulls >= pairs
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_utf8_with_bom() {
		let input = b"\xEF\xBB\xBFHello";
		assert_eq!(convert_to_utf8(input), "Hello");
	}

	#[test]
	fn test_utf8_without_bom() {
		let input = b"Hello World";
		assert_eq!(convert_to_utf8(input), "Hello World");
	}

	#[test]
	fn test_utf16le_with_bom() {
		let input = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
		assert_eq!(convert_to_utf8(input), "Hello");
	}

	#[test]
	fn test_utf16be_with_bom() {
		let input = b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o";
		assert_eq!(convert_to_utf8(input), "Hello");
	}

	#[test]
	fn test_windows1252() {
		let input = b"caf\xE9";
		assert_eq!(convert_to_utf8(input), "café");
	}

	#[test]
	fn test_iso_8859_1() {
		let input = b"Test\xA0String";
		let result = convert_to_utf8(input);
		assert!(result.contains("Test"));
		assert!(result.contains("String"));
	}

	#[test]
	fn test_utf32le_with_bom() {
		let input = b"\xFF\xFE\x00\x00H\x00\x00\x00i\x00\x00\x00";
		assert_eq!(convert_to_utf8(input), "Hi");
	}

	#[test]
	fn test_utf32be_with_bom() {
		let input = b"\x00\x00\xFE\xFF\x00\x00\x00H\x00\x00\x00i";
		assert_eq!(convert_to_utf8(input), "Hi");
	}

	#[test]
	fn test_empty_input() {
		let input = b"";
		assert_eq!(convert_to_utf8(input), "");
	}
}
