//! Plain-text and regex search over the rendered document, with UTF-16 offset conversion
//! (display positions are UTF-16 code units) and wrap-around retry.

use bitflags::bitflags;
use regex::RegexBuilder;

use crate::types as ffi;

bitflags! {
	#[derive(Copy, Clone)]
	pub struct SearchOptions: u8 {
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const WHOLE_WORD = 1 << 2;
		const REGEX = 1 << 3;
	}
}

#[must_use]
pub fn reader_search(haystack: &str, needle: &str, start: i64, options: SearchOptions) -> i64 {
	if needle.is_empty() {
		return -1;
	}
	let start_utf16 = usize::try_from(start.clamp(0, i64::MAX)).unwrap_or(0);

	let utf16_to_byte_index = |s: &str, utf16_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (byte_idx, ch) in s.char_indices() {
			let len16 = ch.len_utf16();
			if utf16_count >= utf16_idx {
				return byte_idx;
			}
			utf16_count += len16;
		}
		s.len()
	};
	let byte_to_utf16_index = |s: &str, byte_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (idx, ch) in s.char_indices() {
			if idx >= byte_idx {
				break;
			}
			utf16_count += ch.len_utf16();
		}
		utf16_count
	};
	let start_byte = utf16_to_byte_index(haystack, start_utf16);

	// Build regex for search - this avoids copying/lowercasing the entire haystack
	let escaped_needle =
		if options.contains(SearchOptions::REGEX) { needle.to_string() } else { regex::escape(needle) };
	let pattern =
		if options.contains(SearchOptions::WHOLE_WORD) { format!(r"\b{escaped_needle}\b") } else { escaped_needle };
	let mut builder = RegexBuilder::new(&pattern);
	if !options.contains(SearchOptions::MATCH_CASE) {
		builder.case_insensitive(true);
	}
	let Ok(re) = builder.build() else {
		return -1;
	};

	if options.contains(SearchOptions::FORWARD) {
		if let Some(m) = re.find(&haystack[start_byte..]) {
			let byte_pos = start_byte + m.start();
			let utf16_pos = byte_to_utf16_index(haystack, byte_pos);
			return i64::try_from(utf16_pos).unwrap_or(-1);
		}
	} else {
		let mut last: Option<usize> = None;
		let end_byte = start_byte.min(haystack.len());
		for m in re.find_iter(&haystack[..end_byte]) {
			last = Some(m.start());
		}
		if let Some(pos) = last {
			let utf16_pos = byte_to_utf16_index(haystack, pos);
			return i64::try_from(utf16_pos).unwrap_or(-1);
		}
	}
	-1
}

#[must_use]
pub fn reader_search_with_wrap(haystack: &str, needle: &str, start: i64, options: SearchOptions) -> ffi::SearchResult {
	let position = reader_search(haystack, needle, start, options);
	if position >= 0 {
		return ffi::SearchResult { found: true, wrapped: false, position };
	}
	let wrap_pos = if options.contains(SearchOptions::FORWARD) {
		0
	} else {
		i64::try_from(haystack.encode_utf16().count()).unwrap_or(0)
	};
	let wrapped_position = reader_search(haystack, needle, wrap_pos, options);
	if wrapped_position >= 0 {
		return ffi::SearchResult { found: true, wrapped: true, position: wrapped_position };
	}
	ffi::SearchResult { found: false, wrapped: false, position: -1 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reader_search_handles_basic_and_whole_word() {
		let haystack = "Hello world";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "hello", 0, options), 0);
		let haystack = "hello_world";
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(haystack, "hello", 0, options), -1);
	}

	#[test]
	fn reader_search_handles_utf16_offsets() {
		let haystack = "a😀b";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "b", 0, options), 3);
	}

	#[test]
	fn reader_search_handles_match_case() {
		let haystack = "Hello hello";
		let options = SearchOptions::FORWARD | SearchOptions::MATCH_CASE;
		assert_eq!(reader_search(haystack, "hello", 0, options), 6);
		assert_eq!(reader_search(haystack, "Hello", 0, options), 0);
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "HELLO", 0, options), 0);
	}

	#[test]
	fn reader_search_with_wrap_wraps_forward() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD;
		let result = reader_search_with_wrap(haystack, "a", 1, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 0);
	}

	#[test]
	fn reader_search_backward_finds_previous_match() {
		let haystack = "one two one";
		let options = SearchOptions::empty();
		assert_eq!(reader_search(haystack, "one", 11, options), 8);
	}

	#[test]
	fn reader_search_with_regex_invalid_pattern_returns_not_found() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD | SearchOptions::REGEX;
		assert_eq!(reader_search(haystack, "(", 0, options), -1);
	}

	#[test]
	fn reader_search_whole_word_positive_case() {
		let haystack = "alpha beta gamma";
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(haystack, "beta", 0, options), 6);
	}

	#[test]
	fn reader_search_clamps_negative_start_to_zero() {
		let haystack = "abc";
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(haystack, "a", -500, options), 0);
	}

	#[test]
	fn reader_search_with_wrap_wraps_backward() {
		let haystack = "abca";
		let options = SearchOptions::empty();
		let result = reader_search_with_wrap(haystack, "a", 0, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 3);
	}
}
