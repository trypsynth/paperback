use pulldown_cmark::{Event, Parser, TagEnd};
use roman::to;

#[must_use]
pub fn markdown_to_text(markdown: &str) -> String {
	let mut text = String::new();
	let parser = Parser::new(markdown);
	for event in parser {
		match event {
			Event::Text(t) | Event::Code(t) => {
				text.push_str(&t);
			}
			Event::End(TagEnd::Paragraph | TagEnd::Heading(_)) => {
				text.push_str("\n\n");
			}
			Event::End(TagEnd::Item) => {
				text.push('\n');
			}
			_ => {}
		}
	}
	text.trim().to_string()
}

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
	if input.is_empty() {
		return String::new();
	}
	let mut result = String::with_capacity(input.len());
	let mut in_leading = true;
	let mut pending_space = false;
	for ch in input.chars() {
		if is_space_like(ch) {
			if in_leading {
				result.push(' ');
			} else {
				pending_space = true;
			}
		} else {
			in_leading = false;
			if pending_space {
				result.push(' ');
				pending_space = false;
			}
			result.push(ch);
		}
	}
	if pending_space {
		result.push(' ');
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

#[must_use]
pub const fn ch_width(ch: char) -> usize {
	#[cfg(windows)]
	{
		ch.len_utf16()
	}
	#[cfg(not(windows))]
	{
		let _ = ch;
		1
	}
}

#[must_use]
pub const fn is_space_like(ch: char) -> bool {
	ch.is_whitespace() || matches!(ch, '\u{00A0}' | '\u{200B}')
}

#[must_use]
pub fn format_list_item(number: i32, list_type: &str) -> String {
	match list_type {
		"a" => to_alpha(number, false),
		"A" => to_alpha(number, true),
		"i" => to(number).map_or_else(|| number.to_string(), |s| s.to_lowercase()),
		"I" => to(number).unwrap_or_else(|| number.to_string()),
		_ => number.to_string(),
	}
}

fn to_alpha(mut n: i32, uppercase: bool) -> String {
	if n <= 0 {
		return n.to_string();
	}
	let mut result = String::new();
	let base = if uppercase { b'A' } else { b'a' };
	while n > 0 {
		n -= 1;
		let offset = u8::try_from(n % 26).unwrap_or(0);
		result.insert(0, (base + offset) as char);
		n /= 26;
	}
	result
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case("hel\u{00AD}lo", "hello")]
	#[case("no hyphens", "no hyphens")]
	#[case("mul\u{00AD}ti\u{00AD}ple", "multiple")]
	fn test_remove_soft_hyphens(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(remove_soft_hyphens(input), expected);
	}

	#[rstest]
	#[case("hello%20world", "hello world")]
	#[case("test%2Fpath", "test/path")]
	#[case("100%25", "100%")]
	#[case("caf%C3%A9", "café")]
	#[case("bad%ZZvalue", "bad%ZZvalue")]
	#[case("%", "%")]
	fn test_url_decode(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(url_decode(input), expected);
	}

	#[rstest]
	#[case("hello   world", "hello world")]
	#[case("hello\n\nworld", "hello world")]
	#[case("hello\t\tworld", "hello world")]
	#[case("  spaces  ", "  spaces ")]
	#[case("hello\u{00A0}\u{00A0}world", "hello world")]
	#[case("hello\u{200B}\u{200B}world", "hello world")]
	#[case("\u{00A0}\u{200B}alpha\t \n beta\u{00A0}", "  alpha beta ")]
	fn test_collapse_whitespace(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(collapse_whitespace(input), expected);
	}

	#[rstest]
	#[case("  hello  ", "hello")]
	#[case("\n\nhello\n\n", "hello")]
	#[case("\u{00A0}hello\u{00A0}", "hello")]
	#[case("\u{200B}hello\u{200B}", "hello")]
	#[case("hello", "hello")]
	#[case("\u{200B}\u{00A0}  hello  \u{00A0}\u{200B}", "hello")]
	fn test_trim_string(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(trim_string(input), expected);
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

	#[rstest]
	#[case(1, "1", "1")]
	#[case(5, "1", "5")]
	#[case(1, "a", "a")]
	#[case(26, "a", "z")]
	#[case(27, "a", "aa")]
	#[case(1, "A", "A")]
	#[case(26, "A", "Z")]
	#[case(27, "A", "AA")]
	#[case(1, "i", "i")]
	#[case(4, "i", "iv")]
	#[case(1994, "i", "mcmxciv")]
	#[case(1, "I", "I")]
	#[case(4, "I", "IV")]
	#[case(1994, "I", "MCMXCIV")]
	#[case(10, "unknown", "10")]
	#[case(0, "a", "0")]
	#[case(-5, "i", "-5")]
	#[case(52, "a", "az")]
	#[case(53, "a", "ba")]
	#[case(52, "A", "AZ")]
	#[case(53, "A", "BA")]
	fn test_format_list_item(#[case] number: i32, #[case] list_type: &str, #[case] expected: &str) {
		assert_eq!(format_list_item(number, list_type), expected);
	}

	#[test]
	fn test_markdown_to_text_paragraphs_and_lists() {
		let md = "# Title\n\nFirst paragraph.\n\n- One\n- Two";
		let text = markdown_to_text(md);
		assert!(text.contains("Title"));
		assert!(text.contains("First paragraph."));
		assert!(text.contains("One\nTwo"));
	}

	#[test]
	fn test_markdown_to_text_preserves_issue_references() {
		let md = "Fixes #12, closes #7, and resolves #312.";
		let text = markdown_to_text(md);
		assert!(text.contains("#12"), "#12 was dropped");
		assert!(text.contains("#7"), "#7 was dropped");
		assert!(text.contains("#312"), "#312 was dropped");
	}

	#[test]
	fn test_markdown_to_text_preserves_inline_code() {
		let md = "Bumps `pdfium` from `969d3b7` to `42b6c95`.";
		let text = markdown_to_text(md);
		assert!(text.contains("969d3b7"), "commit hash before 'to' was dropped");
		assert!(text.contains("42b6c95"), "commit hash after 'to' was dropped");
	}

	#[test]
	fn test_markdown_to_text_preserves_hash_tokens() {
		let md = "Topic #rust and issue #x1 and number #42";
		let text = markdown_to_text(md);
		assert!(text.contains("#rust"));
		assert!(text.contains("#x1"));
		assert!(text.contains("#42"));
	}

	#[rstest]
	#[case(' ', true)]
	#[case('\n', true)]
	#[case('\u{00A0}', true)]
	#[case('\u{200B}', true)]
	#[case('x', false)]
	fn test_is_space_like_variants(#[case] ch: char, #[case] expected: bool) {
		assert_eq!(is_space_like(ch), expected);
	}

	#[test]
	fn collapse_whitespace_empty_returns_empty() {
		assert_eq!(collapse_whitespace(""), "");
	}

	#[test]
	fn collapse_whitespace_no_whitespace_unchanged() {
		assert_eq!(collapse_whitespace("hello"), "hello");
	}

	#[test]
	fn display_len_empty_string_is_zero() {
		assert_eq!(display_len(""), 0);
	}

	// \r and \n are each one code unit on every platform, so \r\n counts as 2.
	// This property underlies the find-navigation coordinate system.
	#[test]
	fn display_len_crlf_counts_as_two_units() {
		assert_eq!(display_len("\r\n"), 2);
	}

	#[test]
	fn display_len_plain_newline_counts_as_one_unit() {
		assert_eq!(display_len("\n"), 1);
	}
}
