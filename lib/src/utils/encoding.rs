use std::str;

use encoding_rs::{UTF_16BE, UTF_16LE, WINDOWS_1252};

#[must_use]
pub fn convert_to_utf8(input: &[u8]) -> String {
	if input.len() >= 4 {
		// UTF-32 LE.
		if input[0] == 0xFF && input[1] == 0xFE && input[2] == 0x00 && input[3] == 0x00 {
			return decode_utf32_le(&input[4..]);
		}
		// UTF-32 BE.
		if input[0] == 0x00 && input[1] == 0x00 && input[2] == 0xFE && input[3] == 0xFF {
			return decode_utf32_be(&input[4..]);
		}
	}
	if input.len() >= 3 {
		// UTF-8.
		if input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
			return String::from_utf8_lossy(&input[3..]).to_string();
		}
	}
	if input.len() >= 2 {
		// UTF-16 LE.
		if input[0] == 0xFF && input[1] == 0xFE {
			let (decoded, _, _) = UTF_16LE.decode(&input[2..]);
			return decoded.to_string();
		}
		// UTF-16 BE.
		if input[0] == 0xFE && input[1] == 0xFF {
			let (decoded, _, _) = UTF_16BE.decode(&input[2..]);
			return decoded.to_string();
		}
	}
	// Try UTF-8 without BOM
	if let Ok(s) = str::from_utf8(input) {
		return s.to_string();
	}
	// Try UTF-16 LE without BOM
	let (decoded, encoding, had_errors) = UTF_16LE.decode(input);
	if !had_errors && encoding == UTF_16LE {
		return decoded.to_string();
	}
	// Try UTF-16 BE without BOM
	let (decoded, encoding, had_errors) = UTF_16BE.decode(input);
	if !had_errors && encoding == UTF_16BE {
		return decoded.to_string();
	}
	// Try Windows-1252
	let (decoded, _, _) = WINDOWS_1252.decode(input);
	if decoded.chars().any(|c| !c.is_control() || c.is_whitespace()) {
		return decoded.to_string();
	}
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
}
