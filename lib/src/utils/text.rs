#[must_use]
pub fn remove_soft_hyphens(input: &str) -> String {
	input.replace("\u{00AD}", "")
}

#[must_use]
pub fn url_decode(input: &str) -> String {
	percent_encoding::percent_decode_str(input).decode_utf8_lossy().into_owned()
}

#[must_use]
pub fn collapse_whitespace(input: &str) -> String {
	let mut result = String::with_capacity(input.len());
	let mut prev_was_space = false;
	for ch in input.chars() {
		let is_space = is_space_like(ch);
		if is_space {
			if !prev_was_space {
				result.push(' ');
				prev_was_space = true;
			}
		} else {
			result.push(ch);
			prev_was_space = false;
		}
	}
	result
}

#[must_use]
pub fn trim_string(s: &str) -> String {
	s.trim_matches(is_space_like).to_string()
}

#[must_use]
pub fn display_len(s: &str) -> usize {
	#[cfg(windows)]
	{
		s.encode_utf16().count()
	}
	#[cfg(not(windows))]
	{
	s.chars().count()
}
}

pub const fn is_space_like(ch: char) -> bool {
	ch.is_whitespace() || matches!(ch, '\u{00A0}' | '\u{200B}')
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_remove_soft_hyphens() {
		assert_eq!(remove_soft_hyphens("hel\u{00AD}lo"), "hello");
		assert_eq!(remove_soft_hyphens("no hyphens"), "no hyphens");
		assert_eq!(remove_soft_hyphens("mul\u{00AD}ti\u{00AD}ple"), "multiple");
	}

	#[test]
	fn test_url_decode() {
		assert_eq!(url_decode("hello%20world"), "hello world");
		assert_eq!(url_decode("test%2Fpath"), "test/path");
		assert_eq!(url_decode("100%25"), "100%");
		// Test UTF-8 encoded characters.
		assert_eq!(url_decode("caf%C3%A9"), "café");
	}

	#[test]
	fn test_collapse_whitespace() {
		assert_eq!(collapse_whitespace("hello   world"), "hello world");
		assert_eq!(collapse_whitespace("hello\n\nworld"), "hello world");
		assert_eq!(collapse_whitespace("hello\t\tworld"), "hello world");
		assert_eq!(collapse_whitespace("  spaces  "), "  spaces ");
		assert_eq!(collapse_whitespace("hello\u{00A0}\u{00A0}world"), "hello world");
		assert_eq!(collapse_whitespace("hello\u{200B}\u{200B}world"), "hello world");
	}

	#[test]
	fn test_trim_string() {
		assert_eq!(trim_string("  hello  "), "hello");
		assert_eq!(trim_string("\n\nhello\n\n"), "hello");
		assert_eq!(trim_string("\u{00A0}hello\u{00A0}"), "hello");
		assert_eq!(trim_string("\u{200B}hello\u{200B}"), "hello");
		assert_eq!(trim_string("hello"), "hello");
	}

	#[cfg(windows)]
	#[test]
	fn test_display_len_windows() {
		assert_eq!(display_len("abc"), 3);
		assert_eq!(display_len("💖"), 2);
		assert_eq!(display_len("line\nwrap"), 9);
	}

	#[cfg(not(windows))]
	#[test]
	fn test_display_len_non_windows() {
		assert_eq!(display_len("abc"), 3);
		assert_eq!(display_len("💖"), 1);
		assert_eq!(display_len("line\nwrap"), 9);
	}
}
