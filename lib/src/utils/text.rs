use pulldown_cmark::{Event, Parser, TagEnd};
use roman::to;

pub fn markdown_to_text(markdown: &str) -> String {
	let mut text = String::new();
	let parser = Parser::new(markdown);
	for event in parser {
		match event {
			Event::Text(t) => {
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
	let mut result = format!(" {}", text.trim());
	loop {
		let original_len = result.len();
		if let Some(start) = result.find(" #") {
			if let Some(substr) = result.get(start + 2..) {
				let num_len = substr.chars().take_while(char::is_ascii_digit).count();
				if num_len > 0 {
					let mut end = start + 2 + num_len;
					if let Some(after_num) = result.get(end..) {
						if after_num.starts_with(',')
							|| (after_num.starts_with('.')
								&& after_num.get(1..).is_none_or(|s| s.starts_with(char::is_whitespace)))
						{
							end += 1;
						}
					}
					result.replace_range(start..end, "");
				}
			}
		}
		if result.len() == original_len {
			break;
		}
	}
	result.trim_start().to_string()
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

pub fn format_list_item(number: i32, list_type: &str) -> String {
	match list_type {
		"a" => to_alpha(number, false),
		"A" => to_alpha(number, true),
		"i" => to(number).map(|s| s.to_lowercase()).unwrap_or_else(|| number.to_string()),
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
		result.insert(0, (base + (n % 26) as u8) as char);
		n /= 26;
	}
	result
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

	#[test]
	fn test_format_list_item() {
		assert_eq!(format_list_item(1, "1"), "1");
		assert_eq!(format_list_item(5, "1"), "5");
		assert_eq!(format_list_item(1, "a"), "a");
		assert_eq!(format_list_item(26, "a"), "z");
		assert_eq!(format_list_item(27, "a"), "aa");
		assert_eq!(format_list_item(1, "A"), "A");
		assert_eq!(format_list_item(26, "A"), "Z");
		assert_eq!(format_list_item(27, "A"), "AA");
		assert_eq!(format_list_item(1, "i"), "i");
		assert_eq!(format_list_item(4, "i"), "iv");
		assert_eq!(format_list_item(1994, "i"), "mcmxciv");
		assert_eq!(format_list_item(1, "I"), "I");
		assert_eq!(format_list_item(4, "I"), "IV");
		assert_eq!(format_list_item(1994, "I"), "MCMXCIV");
		assert_eq!(format_list_item(10, "unknown"), "10");
		assert_eq!(format_list_item(0, "a"), "0");
		assert_eq!(format_list_item(-5, "i"), "-5");
	}
}
