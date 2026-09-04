use std::collections::HashMap;

use rstest::rstest;

use super::{hex_digit, normalize_escapes, normalize_wrapped_space_lines, parse_hex_pair};

#[rstest]
#[case(b'0', Some(0))]
#[case(b'9', Some(9))]
#[case(b'a', Some(10))]
#[case(b'f', Some(15))]
#[case(b'A', Some(10))]
#[case(b'F', Some(15))]
#[case(b'g', None)]
#[case(b'/', None)]
fn hex_digit_classifies_ascii_hex(#[case] input: u8, #[case] expected: Option<u8>) {
	assert_eq!(hex_digit(input), expected);
}

#[rstest]
#[case(b'4', b'1', Some(0x41))]
#[case(b'e', b'9', Some(0xE9))]
#[case(b'E', b'9', Some(0xE9))]
#[case(b'Z', b'9', None)]
#[case(b'1', b'X', None)]
fn parse_hex_pair_parses_and_rejects_invalid(#[case] h1: u8, #[case] h2: u8, #[case] expected: Option<u8>) {
	assert_eq!(parse_hex_pair(h1, h2), expected);
}

#[test]
fn normalize_escapes_decodes_non_structural_escapes() {
	let input = "Don\\'27t say Caf\\'e9";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "Don't say Café");
}

#[test]
fn normalize_escapes_keeps_ascii_escape_sequences() {
	let input = "Escaped brace: \\'7b and slash: \\'5c";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, input);
}

#[test]
fn normalize_escapes_ignores_invalid_hex_sequences() {
	let input = "Broken: \\'zz and mixed: \\'G1";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, input);
}
#[test]
fn normalize_escapes_keeps_u_fallback_hex_sequences() {
	let input = "Ju\\u237\\'edzo";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "Ju\\u237 zo");
}

#[test]
fn normalize_escapes_maps_nonbreaking_space_and_hyphen_symbols() {
	let input = "A\\~B C\\_D E\\-F";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "A B C-D E-F");
}

#[test]
fn normalize_escapes_drops_question_mark_unicode_fallback() {
	// Word writes the fallback with no delimiter, so the `?` terminates the digits.
	let input = "Majesty\\u8217?s First";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "Majesty\\u8217 s First");
}

#[test]
fn normalize_escapes_keeps_question_mark_that_follows_a_delimiter() {
	// Here the space already delimits the control word, so the `?` is real text.
	let input = "vraiment\\u8230 ? Non";
	let output = normalize_escapes(input, encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "vraiment\\u8230  ? Non");
}

#[test]
fn normalize_escapes_honours_uc_zero_and_multi_character_fallback() {
	let no_fallback = normalize_escapes("\\uc0 a\\u8217?b", encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(no_fallback, "\\uc0 a\\u8217 ?b");
	let two_chars = normalize_escapes("\\uc2 a\\u8217??b", encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(two_chars, "\\uc2 a\\u8217 b");
}

#[test]
fn normalize_escapes_preserves_negative_unicode_parameters() {
	let output = normalize_escapes("a\\u-3891?b", encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "a\\u-3891 b");
}

#[test]
fn normalize_escapes_promotes_literal_tabs_to_control_words() {
	let output = normalize_escapes("{\t\nFor Mum", encoding_rs::WINDOWS_1252, &HashMap::new());
	assert_eq!(output, "{\\tab \nFor Mum");
}

#[test]
fn normalize_wrapped_space_lines_preserves_inter_word_space_on_its_own_line() {
	let input = "The older man was\r\n \r\nwordless";
	assert_eq!(normalize_wrapped_space_lines(input), "The older man was wordless");
}

#[test]
fn normalize_escapes_uses_font_charset_for_encoding() {
	// \f2 switches to charset 238 (Windows-1250); \'c6 = Ć in that encoding, Æ in 1252.
	let rtf = r"{\rtf1\ansi\ansicpg1252{\fonttbl{\f1\fcharset0 Arial;}{\f2\fcharset238 CE;}}\pard\f2 \'c6ao}";
	let default_enc = encoding_rs::WINDOWS_1252;
	let font_table = super::super::encoding::extract_font_table(rtf, default_enc);
	let out = normalize_escapes(rtf, default_enc, &font_table);
	assert!(out.contains('Ć'), "expected Ć (Windows-1250 0xC6), got: {out}");
	assert!(!out.contains('Æ'), "should not contain Æ (Windows-1252 0xC6)");
}
