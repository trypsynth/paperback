//! Pre-processing raw RTF text before it reaches `rtf_parser`'s lexer: normalizing
//! hard-wrapped inter-word spaces, and rewriting `\'xx`/`\uN` escapes and literal tabs into
//! forms the lexer tokenizes correctly (see [`normalize_escapes`] for the full rationale).

use std::{collections::HashMap, str};

use encoding_rs::Encoding;

/// Some writers hard-wrap lines and occasionally place an inter-word space on
/// its own line (`word\r\n \r\nnext`). Preserve that as a single space so words
/// don't get merged by downstream tokenization.
pub(super) fn normalize_wrapped_space_lines(input: &str) -> String {
	let mut out = String::with_capacity(input.len());
	let bytes = input.as_bytes();
	let mut i = 0;
	while i < bytes.len() {
		let mut j = i;
		if consume_line_break(bytes, &mut j) {
			while j < bytes.len() && matches!(bytes[j], b' ' | b'\t') {
				j += 1;
			}
			let mut k = j;
			if consume_line_break(bytes, &mut k) {
				let left =
					out.chars().next_back().is_some_and(|ch| !ch.is_whitespace() && !matches!(ch, '\\' | '{' | '}'));
				let right = bytes
					.get(k)
					.copied()
					.is_some_and(|b| !b.is_ascii_whitespace() && !matches!(b, b'\\' | b'{' | b'}'));
				if left && right && !out.ends_with(' ') {
					out.push(' ');
				}
				i = k;
				continue;
			}
		}
		out.push(bytes[i] as char);
		i += 1;
	}
	out
}

fn consume_line_break(bytes: &[u8], idx: &mut usize) -> bool {
	if *idx >= bytes.len() {
		return false;
	}
	match bytes[*idx] {
		b'\r' => {
			*idx += 1;
			if *idx < bytes.len() && bytes[*idx] == b'\n' {
				*idx += 1;
			}
			true
		}
		b'\n' => {
			*idx += 1;
			true
		}
		_ => false,
	}
}

/// Reads an RTF numeric parameter at `start`, allowing the leading `-` that `\uN`
/// uses for codepoints above 0x7FFF. Returns the index just past the last digit.
fn read_param_end(bytes: &[u8], start: usize) -> Option<usize> {
	let mut j = start;
	if bytes.get(j) == Some(&b'-') {
		j += 1;
	}
	let digits_start = j;
	while bytes.get(j).is_some_and(u8::is_ascii_digit) {
		j += 1;
	}
	(j > digits_start).then_some(j)
}

/// Skips the ANSI fallback characters that follow a `\uN` escape, starting at the
/// index just past its parameter digits.
///
/// Each `\uN` is trailed by `uc` characters spelling the same character in the
/// document codepage, which a reader that understands `\uN` must discard. Only the
/// two forms that actually appear in the wild are recognised: a `\'xx` hex escape
/// (`LibreOffice`) and a literal `?` (Word and most ebook converters). Anything else
/// is left where it is — writers that disagree with their own `\ucN` are commoner
/// than exotic fallbacks, and swallowing real text is far worse than leaving a
/// stray character behind.
fn skip_unicode_fallback(bytes: &[u8], mut i: usize, uc: usize) -> usize {
	for _ in 0..uc {
		// A hex escape is unambiguously fallback, even across the space that
		// delimits the control word.
		let mut j = i;
		while bytes.get(j) == Some(&b' ') {
			j += 1;
		}
		if bytes.get(j) == Some(&b'\\')
			&& bytes.get(j + 1) == Some(&b'\'')
			&& let (Some(&h1), Some(&h2)) = (bytes.get(j + 2), bytes.get(j + 3))
			&& parse_hex_pair(h1, h2).is_some()
		{
			i = j + 4;
			continue;
		}
		// A bare `?` counts only when it sits directly against the previous
		// character, where it cannot be document text: something has to terminate
		// the parameter digits, and the `?` is doing that job.
		if bytes.get(i) == Some(&b'?') {
			i += 1;
			continue;
		}
		break;
	}
	i
}

/// Pre-processes RTF text so the lexer sees escapes it can tokenize correctly.
///
/// `\'xx` hex escapes are replaced with their correctly decoded UTF-8 characters.
/// This resolves the ambiguity between `\'xx` (codepage byte) and `\uN` (Unicode)
/// escapes before the lexer sees them, since the `rtf_parser` crate conflates both
/// into `ControlWord::Unicode`.
///
/// Structural ASCII escapes (`\'7b`, `\'7d`, `\'5c`) are left intact so the lexer
/// still handles escaped `{`, `}`, and `\` correctly.
///
/// `\uN` escapes are rewritten as space-delimited control words with their ANSI
/// fallback dropped. `rtf_parser` ends a control word at the first whitespace, so
/// Word's `Majesty\uNNNN?s First` would otherwise lex as one unknown control word
/// running from the backslash to the space, losing both the apostrophe and the `s`.
///
/// Literal tab characters become `\tab` control words, since the lexer trims
/// whitespace off its slices and would otherwise drop them.
///
/// Tracks `\fN` control words to use the charset declared for each font in the
/// font table, so that Central-European or other non-Latin characters encoded as
/// `\'xx` bytes decode correctly even when the document-level `\ansicpg` differs.
pub(super) fn normalize_escapes(
	rtf: &str,
	encoding: &'static Encoding,
	font_table: &HashMap<u32, &'static Encoding>,
) -> String {
	let mut result = String::with_capacity(rtf.len());
	let bytes = rtf.as_bytes();
	let len = bytes.len();
	let mut i = 0;
	let mut current_encoding = encoding;
	// RTF scopes \ucN to its group; tracking it flat is an approximation that can
	// only leave a stray fallback character, never eat real text.
	let mut uc_count = 1usize;
	while i < len {
		// A literal tab in the text stream carries the same meaning as \tab to every
		// other reader, so promote it rather than let the lexer trim it away.
		if bytes[i] == b'\t' {
			result.push_str("\\tab ");
			i += 1;
			continue;
		}
		if bytes[i] == b'\\' && bytes.get(i + 1) == Some(&b'u') {
			// \ucN declares how many fallback characters trail each \uN. Fall through
			// afterwards so the control word still reaches the lexer.
			if bytes.get(i + 2) == Some(&b'c') {
				if let Some(end) = read_param_end(bytes, i + 3)
					&& let Ok(count) = rtf[i + 3..end].parse::<usize>()
				{
					uc_count = count;
				}
			} else if let Some(digits_end) = read_param_end(bytes, i + 2) {
				result.push_str(&rtf[i..digits_end]);
				result.push(' ');
				i = skip_unicode_fallback(bytes, digits_end, uc_count);
				continue;
			}
		}
		// Track \fN font switches to use the right charset for subsequent \'xx escapes.
		// \fcharset, \fbidi, \froman, etc. start with \f + non-digit so won't match.
		if bytes[i] == b'\\' && i + 2 < len && bytes[i + 1] == b'f' && bytes[i + 2].is_ascii_digit() {
			let num_start = i + 2;
			let mut num_end = num_start;
			while num_end < len && bytes[num_end].is_ascii_digit() {
				num_end += 1;
			}
			if let Ok(s) = str::from_utf8(&bytes[num_start..num_end])
				&& let Ok(font_num) = s.parse::<u32>()
			{
				current_encoding = font_table.get(&font_num).copied().unwrap_or(encoding);
			}
			// Fall through — emit the control word bytes as-is for the lexer.
		}
		if bytes[i] == b'\\' && i + 1 < len {
			match bytes[i + 1] {
				// RTF non-breaking space
				b'~' => {
					result.push(' ');
					i += 2;
					continue;
				}
				// Optional / non-breaking hyphen
				b'-' | b'_' => {
					result.push('-');
					i += 2;
					continue;
				}
				_ => {}
			}
		}
		if bytes[i] == b'\\' && i + 3 < len && bytes[i + 1] == b'\'' {
			let h1 = bytes[i + 2];
			let h2 = bytes[i + 3];
			if let Some(byte) = parse_hex_pair(h1, h2)
				&& !matches!(byte, 0x7B | 0x7D | 0x5C)
			{
				let buf = [byte];
				let (decoded, _, _) = current_encoding.decode(&buf);
				result.push_str(&decoded);
				i += 4;
				continue;
			}
		}
		result.push(bytes[i] as char);
		i += 1;
	}
	result
}

/// Parses two ASCII hex digit bytes into a `u8`.
fn parse_hex_pair(h1: u8, h2: u8) -> Option<u8> {
	let d1 = hex_digit(h1)?;
	let d2 = hex_digit(h2)?;
	Some(d1 << 4 | d2)
}

const fn hex_digit(b: u8) -> Option<u8> {
	match b {
		b'0'..=b'9' => Some(b - b'0'),
		b'a'..=b'f' => Some(b - b'a' + 10),
		b'A'..=b'F' => Some(b - b'A' + 10),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
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
}
