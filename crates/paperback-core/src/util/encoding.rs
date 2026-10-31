use std::str;

use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};
use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE, WINDOWS_1252};

/// A file doesn't change encoding part-way, so a prefix answers as well as the whole; scanning all of a 60 MB GB18030 book made it open several times slower than in UTF-8 (#886).
const DETECTION_SAMPLE_BYTES: usize = 64 * 1024;

#[must_use]
pub fn convert_to_utf8(input: &[u8]) -> String {
	if input.len() >= 4 {
		if input[0] == 0xFF && input[1] == 0xFE && input[2] == 0x00 && input[3] == 0x00 {
			return decode_utf32_le(&input[4..]);
		}
		if input[0] == 0x00 && input[1] == 0x00 && input[2] == 0xFE && input[3] == 0xFF {
			return decode_utf32_be(&input[4..]);
		}
	}
	if input.len() >= 3 && input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
		return String::from_utf8_lossy(&input[3..]).to_string();
	}
	if input.len() >= 2 {
		if input[0] == 0xFF && input[1] == 0xFE {
			let (decoded, _, _) = UTF_16LE.decode(&input[2..]);
			return decoded.to_string();
		}
		if input[0] == 0xFE && input[1] == 0xFF {
			let (decoded, _, _) = UTF_16BE.decode(&input[2..]);
			return decoded.to_string();
		}
	}
	if let Ok(s) = str::from_utf8(input) {
		return s.to_string();
	}
	if looks_like_utf16(sample_of(input)) {
		let (decoded, encoding, had_errors) = UTF_16LE.decode(input);
		if !had_errors && encoding == UTF_16LE {
			return decoded.to_string();
		}
		let (decoded, encoding, had_errors) = UTF_16BE.decode(input);
		if !had_errors && encoding == UTF_16BE {
			return decoded.to_string();
		}
	}
	// Only a multi-byte guess is trusted: their structure makes a clean decode meaningful, while single-byte encodings decode almost anything.
	let mut detector = EncodingDetector::new(Iso2022JpDetection::Allow);
	let sample = sample_of(input);
	detector.feed(sample, sample.len() == input.len());
	let detected = detector.guess(None, Utf8Detection::Allow);
	if !detected.is_single_byte() {
		let (decoded, _, had_errors) = detected.decode(input);
		if !had_errors {
			return decoded.to_string();
		}
	}
	let (decoded, _, _) = WINDOWS_1252.decode(input);
	if decoded.chars().any(|c| !c.is_control() || c.is_whitespace()) {
		return decoded.to_string();
	}
	String::from_utf8_lossy(input).to_string()
}

/// Text from an ANSI-era format. A few symbol-font bytes (checkboxes, bullets) in Western text read to the detector as a CJK encoding, which then swallows the letter after each, so text that is almost all ASCII is taken as Windows-1252.
#[must_use]
pub fn decode_ansi(input: &[u8]) -> String {
	let high = input.iter().filter(|&&byte| byte >= 0x80).count();
	if high * 20 < input.len() { WINDOWS_1252.decode(input).0.into_owned() } else { convert_to_utf8(input) }
}

/// The HTML specification's prescan length for `<meta charset>`.
const HTML_PRESCAN_BYTES: usize = 1024;

/// The XML declaration has to come first, so anything past the first line is not one.
const XML_DECLARATION_SCAN_BYTES: usize = 200;

#[must_use]
pub fn xml_declared_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
	let head = String::from_utf8_lossy(&bytes[..bytes.len().min(XML_DECLARATION_SCAN_BYTES)]);
	let declaration = head.split_once("?>")?.0;
	let after_key = declaration.split_once("encoding")?.1;
	let quoted = after_key.trim_start().strip_prefix('=')?.trim_start();
	let quote = quoted.chars().next().filter(|c| *c == '"' || *c == '\'')?;
	let label = quoted[1..].split(quote).next()?;
	Encoding::for_label(label.as_bytes())
}

#[must_use]
pub fn html_declared_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
	if let Some(encoding) = xml_declared_encoding(bytes) {
		return Some(encoding);
	}
	let head = String::from_utf8_lossy(&bytes[..bytes.len().min(HTML_PRESCAN_BYTES)]).to_ascii_lowercase();
	let value = head.split("charset").skip(1).find_map(|rest| rest.trim_start().strip_prefix('='))?;
	let value = value.trim_start().trim_start_matches(['"', '\'']);
	let label = value.split(|c: char| c.is_ascii_whitespace() || matches!(c, '"' | '\'' | ';' | '>' | '/')).next()?;
	let encoding = Encoding::for_label(label.as_bytes())?;
	// A file readable far enough to find the declaration is not UTF-16, so the specification reads such a claim as UTF-8.
	Some(if encoding == UTF_16LE || encoding == UTF_16BE { UTF_8 } else { encoding })
}

/// A UTF-8 claim on bytes that are not UTF-8 is ignored, as old editors stamped it on whatever they saved.
#[must_use]
pub fn decode_html(bytes: &[u8], fallback: Option<&'static Encoding>) -> String {
	let has_bom =
		bytes.starts_with(&[0xEF, 0xBB, 0xBF]) || bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]);
	if has_bom {
		return convert_to_utf8(bytes);
	}
	if let Ok(text) = str::from_utf8(bytes) {
		return text.to_owned();
	}
	html_declared_encoding(bytes)
		.filter(|encoding| *encoding != UTF_8)
		.or(fallback)
		.map_or_else(|| convert_to_utf8(bytes), |encoding| encoding.decode_without_bom_handling(bytes).0.into_owned())
}

/// Takes ownership so UTF-8 becomes the text without a copy, which for a whole text file would cost as much memory again.
#[must_use]
pub fn convert_owned_to_utf8(mut bytes: Vec<u8>) -> String {
	let bom = bytes.starts_with(&[0xEF, 0xBB, 0xBF]);
	if bom {
		bytes.drain(..3);
	}
	match String::from_utf8(bytes) {
		Ok(text) => text,
		Err(error) if bom => String::from_utf8_lossy(error.as_bytes()).into_owned(),
		Err(error) => convert_to_utf8(error.as_bytes()),
	}
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

/// The detector must not be told a cut sample is the end of input, or it reads the cut character as malformed and rules out the right encoding (#907).
fn sample_of(input: &[u8]) -> &[u8] {
	&input[..input.len().min(DETECTION_SAMPLE_BYTES)]
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

	#[rstest]
	#[case(b"\xEF\xBB\xBFHello")]
	#[case(b"\xEF\xBB\xBFbad \xFF byte")]
	#[case(b"Hello World")]
	#[case(b"caf\xE9")]
	#[case(b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00")]
	#[case(b"\x00\x00\xFE\xFF\x00\x00\x00H\x00\x00\x00i")]
	#[case(b"\xC4\xE3\xBA\xC3\xA3\xAC\xCA\xC0\xBD\xE7")]
	#[case(b"")]
	fn owned_bytes_decode_as_borrowed_ones_do(#[case] input: &[u8]) {
		assert_eq!(convert_owned_to_utf8(input.to_vec()), convert_to_utf8(input));
	}

	/// #886: detection reads only a prefix, so text past it must still come out whole and correct.
	#[test]
	fn a_book_longer_than_the_detection_sample_still_decodes_whole() {
		let paragraph = "这是一本很长的中文电子书，用来确认取样检测之后的文字仍然正确。
";
		let repeats = (DETECTION_SAMPLE_BYTES / paragraph.len()) * 4;
		let text: String = paragraph.repeat(repeats);
		let (encoded, _, had_errors) = encoding_rs::GB18030.encode(&text);
		assert!(!had_errors, "the fixture itself must be representable in GB18030");
		assert!(encoded.len() > DETECTION_SAMPLE_BYTES * 2, "the fixture must outrun the sample");
		assert_eq!(convert_to_utf8(&encoded), text);
	}

	/// #907: a sample cut mid-character made the detector rule out GBK and fall back to Windows-1252.
	#[test]
	fn a_sample_ending_mid_character_still_detects_gbk() {
		let text = format!("a{}", "这是一本很长的中文电子书用来确认取样检测".repeat(DETECTION_SAMPLE_BYTES / 20));
		let (encoded, _, had_errors) = encoding_rs::GBK.encode(&text);
		assert!(!had_errors, "the fixture itself must be representable in GBK");
		assert!(encoded[DETECTION_SAMPLE_BYTES - 1] >= 0x81, "the sample must end on a lead byte");
		assert_eq!(convert_to_utf8(&encoded), text);
	}

	/// #632: GBK text fell through to the Windows-1252 fallback and came out as mojibake.
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

	#[rstest]
	#[case(b"<meta charset=\"windows-1251\">", Some("windows-1251"))]
	#[case(b"<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=Windows-1251\">", Some("windows-1251"))]
	#[case(b"<meta charset=gb2312>", Some("GBK"))]
	#[case(b"<meta charset='shift_jis'/>", Some("Shift_JIS"))]
	#[case(b"<meta charset=\"utf-16\">", Some("UTF-8"))]
	#[case(b"<meta charset=\"klingon\">", None)]
	#[case(b"<p>no declaration</p>", None)]
	#[case(b"<?xml version=\"1.0\" encoding=\"windows-1250\"?><html>", Some("windows-1250"))]
	fn reads_the_charset_an_html_file_declares(#[case] input: &[u8], #[case] expected: Option<&str>) {
		assert_eq!(html_declared_encoding(input).map(Encoding::name), expected);
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
