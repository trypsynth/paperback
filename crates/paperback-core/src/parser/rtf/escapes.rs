use std::{borrow::Cow, collections::HashMap, fmt::Write as _, str};

use encoding_rs::Encoding;

/// Plenty of writers put raw codepage bytes in 7-bit RTF; as `\'xx` escapes they decode under the active font. Valid UTF-8 is left alone, since a codepage byte run practically never validates.
pub(super) fn escape_raw_bytes(bytes: &[u8]) -> Cow<'_, str> {
	if let Ok(text) = str::from_utf8(bytes) {
		return Cow::Borrowed(text);
	}
	let mut out = String::with_capacity(bytes.len() + bytes.len() / 8);
	for &byte in bytes {
		if byte.is_ascii() {
			out.push(byte as char);
		} else {
			let _ = write!(out, "\\'{byte:02x}");
		}
	}
	Cow::Owned(out)
}

/// Some writers hard-wrap an inter-word space onto its own line (`word\r\n \r\nnext`), which must stay a space or the words merge.
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
				// Only a run standing for an inter-word space is swallowed: dropping the break after a lone backslash (a Cocoa paragraph end) left it against the next control word, so the two read as an escaped backslash and a literal \f0 landed in the text.
				if left && right && !out.ends_with(' ') {
					out.push(' ');
					i = k;
					continue;
				}
			}
		}
		let ch = input[i..].chars().next().unwrap_or_default();
		out.push(ch);
		i += ch.len_utf8();
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

/// Once the escape that ended a control word (`{\leveltext\'01-;}`) is decoded, the lexer, which ends control words only at whitespace, fails on the `-` as a numeric parameter.
/// Word writes this for every dash-bulleted list, so the whole document refused to open.
fn end_open_control_word(out: &mut String) {
	let bytes = out.as_bytes();
	let mut start = bytes.len();
	while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'-') {
		start -= 1;
	}
	if start == bytes.len() || start == 0 || !bytes[start].is_ascii_alphabetic() {
		return;
	}
	// An odd run of backslashes before the letters starts a control word; an even run is escaped backslashes followed by ordinary text.
	let backslashes = bytes[..start].iter().rev().take_while(|&&b| b == b'\\').count();
	if backslashes % 2 == 1 {
		out.push(' ');
	}
}

/// Allows the leading `-` that `\uN` uses for codepoints above 0x7FFF.
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

/// Only the fallbacks seen in the wild are skipped, a `\'xx` escape (`LibreOffice`) and a literal `?` (Word and most ebook converters): writers that disagree with their own `\ucN` are commoner than exotic fallbacks, and swallowing real text is far worse than a stray character.
fn skip_unicode_fallback(bytes: &[u8], mut i: usize, uc: usize) -> usize {
	for _ in 0..uc {
		// A hex escape is unambiguously fallback, even across the space that delimits the control word.
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
		// A bare `?` counts only directly against the digits, where it cannot be document text: it is what terminates the parameter.
		if bytes.get(i) == Some(&b'?') {
			i += 1;
			continue;
		}
		break;
	}
	i
}

/// `rtf_parser` conflates `\'xx` and `\uN` into `ControlWord::Unicode`, so hex escapes are decoded here, except `\'7b`, `\'7d` and `\'5c`, which the lexer must still see as escaped braces and backslash.
/// The lexer ends a control word only at whitespace, so `\uN` gets a delimiting space and loses its fallback; Word's `Majesty\uNNNN?s First` otherwise lost both the apostrophe and the `s`.
/// Literal tabs become `\tab`, since the lexer trims whitespace off its slices.
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
	// RTF scopes \ucN to its group; tracking it flat is an approximation that can only leave a stray fallback character, never eat real text.
	let mut uc_count = 1usize;
	while i < len {
		// A literal tab means \tab to every other reader, so promote it rather than let the lexer trim it away.
		if bytes[i] == b'\t' {
			result.push_str("\\tab ");
			i += 1;
			continue;
		}
		if bytes[i] == b'\\' && bytes.get(i + 1) == Some(&b'u') {
			// Falls through after reading \ucN so the control word still reaches the lexer.
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
				// The space after the digits only ends the control word; with \uc0 nothing stepped over it, and leaving it turned "\u347 roda" into "ś roda".
				if i == digits_end && bytes.get(i) == Some(&b' ') {
					i += 1;
				}
				continue;
			}
		}
		// \fcharset, \fbidi, \froman and the like are \f plus a non-digit, so only font switches match.
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
		}
		if bytes[i] == b'\\' && i + 1 < len {
			match bytes[i + 1] {
				// RTF non-breaking space
				b'~' => {
					end_open_control_word(&mut result);
					result.push(' ');
					i += 2;
					continue;
				}
				// Optional / non-breaking hyphen
				b'-' | b'_' => {
					end_open_control_word(&mut result);
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
				end_open_control_word(&mut result);
				result.push_str(&decoded);
				i += 4;
				continue;
			}
		}
		let ch = rtf[i..].chars().next().unwrap_or_default();
		result.push(ch);
		i += ch.len_utf8();
	}
	result
}

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
mod tests;
