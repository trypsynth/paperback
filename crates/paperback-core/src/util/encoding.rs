use std::str;

use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};
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
	// Legacy multi-byte encodings (GBK, Big5, Shift_JIS, EUC-KR, etc.) have no BOM and
	// aren't valid UTF-8, so they can only be guessed heuristically. chardetng is the
	// statistical detector Mozilla built specifically to pair with encoding_rs for this.
	// Only trust its guess when it picks a multi-byte encoding: those have real
	// structural constraints, so a clean decode is a meaningful signal. Single-byte
	// encodings decode almost any input "successfully", so a confident-looking guess on
	// a short/ambiguous sample is more likely wrong than not. That case is left to the
	// Windows-1252-or-give-up fallback below, unchanged.
	let mut detector = EncodingDetector::new(Iso2022JpDetection::Allow);
	detector.feed(input, true);
	let detected = detector.guess(None, Utf8Detection::Allow);
	if !detected.is_single_byte() {
		let (decoded, _, had_errors) = detected.decode(input);
		if !had_errors {
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
		.as_chunks::<4>()
		.0
		.iter()
		.filter_map(|chunk| {
			let code_point = u32::from_le_bytes(*chunk);
			char::from_u32(code_point)
		})
		.collect()
}

fn decode_utf32_be(input: &[u8]) -> String {
	input
		.as_chunks::<4>()
		.0
		.iter()
		.filter_map(|chunk| {
			let code_point = u32::from_be_bytes(*chunk);
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
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case(b"\xEF\xBB\xBFHello", "Hello")]
	#[case(b"Hello World", "Hello World")]
	#[case(b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00", "Hello")]
	#[case(b"\xFE\xFF\x00H\x00e\x00l\x00l\x00o", "Hello")]
	#[case(b"\xFF\xFE\x00\x00H\x00\x00\x00i\x00\x00\x00", "Hi")]
	#[case(b"\x00\x00\xFE\xFF\x00\x00\x00H\x00\x00\x00i", "Hi")]
	#[case(b"", "")]
	fn test_convert_to_utf8_known_inputs(#[case] input: &[u8], #[case] expected: &str) {
		assert_eq!(convert_to_utf8(input), expected);
	}

	/// <https://github.com/trypsynth/paperback/issues/632>: GBK-encoded Chinese text
	/// (no BOM, not valid UTF-8) was falling through to the Windows-1252 fallback and
	/// coming out as mojibake instead of being detected.
	#[test]
	fn detects_gbk_encoded_chinese_text() {
		let original = "你好，世界。这是一个测试。";
		let (encoded, _, had_errors) = encoding_rs::GBK.encode(original);
		assert!(!had_errors, "test setup: GBK must be able to encode this sample");
		assert_eq!(convert_to_utf8(&encoded), original);
	}

	#[test]
	fn detects_big5_encoded_chinese_text() {
		let original = "你好，世界。這是一個測試。";
		let (encoded, _, had_errors) = encoding_rs::BIG5.encode(original);
		assert!(!had_errors, "test setup: Big5 must be able to encode this sample");
		assert_eq!(convert_to_utf8(&encoded), original);
	}

	#[test]
	fn detects_shift_jis_encoded_japanese_text() {
		let original = "こんにちは世界。これはテストです。";
		let (encoded, _, had_errors) = encoding_rs::SHIFT_JIS.encode(original);
		assert!(!had_errors, "test setup: Shift_JIS must be able to encode this sample");
		assert_eq!(convert_to_utf8(&encoded), original);
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
	fn test_utf16le_without_bom_when_pattern_matches() {
		let input = b"H\x00i\x00";
		assert_eq!(convert_to_utf8(input), "H\0i\0");
	}

	#[test]
	fn test_utf32le_ignores_incomplete_trailing_bytes() {
		let input = b"\xFF\xFE\x00\x00A\x00\x00\x00\x99";
		assert_eq!(convert_to_utf8(input), "A");
	}

	#[test]
	fn test_decode_utf32_le_skips_invalid_code_points() {
		let input = [0x00, 0xD8, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00];
		assert_eq!(decode_utf32_le(&input), "A");
	}

	#[test]
	fn test_decode_utf32_be_ignores_incomplete_chunks() {
		let input = [0x00, 0x00, 0x00, 0x41, 0x99];
		assert_eq!(decode_utf32_be(&input), "A");
	}

	#[rstest]
	#[case(b"H\x00i\x00", true)]
	#[case(b"\x00H\x00i", true)]
	#[case(b"Hello", false)]
	#[case(b"", false)]
	fn test_looks_like_utf16_heuristic(#[case] input: &[u8], #[case] expected: bool) {
		assert_eq!(looks_like_utf16(input), expected);
	}

	#[test]
	fn test_convert_to_utf8_falls_back_to_lossy_when_no_viable_decode() {
		let input = b"\x81\x8D";
		assert_eq!(convert_to_utf8(input), "\u{FFFD}\u{FFFD}");
	}
}
