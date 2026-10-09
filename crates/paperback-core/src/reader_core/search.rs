//! Plain-text and regex search over the rendered document, with display-unit offset conversion
//! through [`DocumentBuffer`] and wrap-around retry.

use bitflags::bitflags;
use regex::{Regex, RegexBuilder};

use crate::{document::DocumentBuffer, types as ffi};

bitflags! {
	#[derive(Copy, Clone)]
	pub struct SearchOptions: u8 {
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const WHOLE_WORD = 1 << 2;
		const REGEX = 1 << 3;
	}
}

/// The matcher both `reader_search` and `reader_search_all` run, honouring `options`: the needle
/// is escaped unless it is already a regex, wrapped in `\b…\b` for whole-word search, and matched
/// case-insensitively unless `MATCH_CASE` is set. `None` for an invalid regex.
fn build_matcher(needle: &str, options: SearchOptions) -> Option<Regex> {
	let escaped_needle =
		if options.contains(SearchOptions::REGEX) { needle.to_string() } else { regex::escape(needle) };
	let pattern =
		if options.contains(SearchOptions::WHOLE_WORD) { format!(r"\b{escaped_needle}\b") } else { escaped_needle };
	let mut builder = RegexBuilder::new(&pattern);
	if !options.contains(SearchOptions::MATCH_CASE) {
		builder.case_insensitive(true);
	}
	builder.build().ok()
}

#[must_use]
pub fn reader_search(buffer: &DocumentBuffer, needle: &str, start: i64, options: SearchOptions) -> i64 {
	if needle.is_empty() {
		return -1;
	}
	let haystack = &buffer.content;
	let start_display = usize::try_from(start.max(0)).unwrap_or(0).min(buffer.total_display_len());
	let start_byte = buffer.byte_index_for_display(start_display);
	// Build regex for search - this avoids copying/lowercasing the entire haystack.
	let Some(re) = build_matcher(needle, options) else {
		return -1;
	};
	if options.contains(SearchOptions::FORWARD) {
		if let Some(m) = re.find(&haystack[start_byte..]) {
			let byte_pos = start_byte + m.start();
			let display_pos = buffer.display_index_for_byte(byte_pos);
			return i64::try_from(display_pos).unwrap_or(-1);
		}
	} else {
		let mut last: Option<usize> = None;
		for m in re.find_iter(&haystack[..start_byte]) {
			last = Some(m.start());
		}
		if let Some(pos) = last {
			let display_pos = buffer.display_index_for_byte(pos);
			return i64::try_from(display_pos).unwrap_or(-1);
		}
	}
	-1
}

/// Every match of `needle` in `buffer`, as display-unit `(start, end)` spans in document order.
/// Direction and wrap have no meaning here. Zero-length matches (e.g. an empty `x*` regex match)
/// are skipped; an empty needle or an invalid regex yields no spans.
#[must_use]
pub fn reader_search_all(buffer: &DocumentBuffer, needle: &str, options: SearchOptions) -> Vec<(i64, i64)> {
	if needle.is_empty() {
		return Vec::new();
	}
	let Some(re) = build_matcher(needle, options) else {
		return Vec::new();
	};
	re.find_iter(&buffer.content)
		.filter(|m| !m.is_empty())
		.map(|m| {
			let start = i64::try_from(buffer.display_index_for_byte(m.start())).unwrap_or(-1);
			let end = i64::try_from(buffer.display_index_for_byte(m.end())).unwrap_or(-1);
			(start, end)
		})
		.collect()
}

#[must_use]
pub fn reader_search_with_wrap(
	buffer: &DocumentBuffer,
	needle: &str,
	start: i64,
	options: SearchOptions,
) -> ffi::SearchResult {
	let position = reader_search(buffer, needle, start, options);
	if position >= 0 {
		return ffi::SearchResult { found: true, wrapped: false, position };
	}
	let wrap_pos = if options.contains(SearchOptions::FORWARD) {
		0
	} else {
		i64::try_from(buffer.total_display_len()).unwrap_or(0)
	};
	let wrapped_position = reader_search(buffer, needle, wrap_pos, options);
	if wrapped_position >= 0 {
		return ffi::SearchResult { found: true, wrapped: true, position: wrapped_position };
	}
	ffi::SearchResult { found: false, wrapped: false, position: -1 }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::text::display_len;

	#[test]
	fn reader_search_handles_basic_and_whole_word() {
		let buffer = DocumentBuffer::with_content("Hello world".to_string());
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "hello", 0, options), 0);
		let buffer = DocumentBuffer::with_content("hello_world".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(&buffer, "hello", 0, options), -1);
	}

	#[test]
	fn reader_search_returns_display_offsets() {
		let buffer = DocumentBuffer::with_content("a😀b".to_string());
		let options = SearchOptions::FORWARD;
		let expected = i64::try_from(display_len("a😀")).unwrap();
		assert_eq!(reader_search(&buffer, "b", 0, options), expected);
	}

	#[test]
	fn reader_search_starts_from_a_display_offset() {
		let buffer = DocumentBuffer::with_content("😀b😀b".to_string());
		let options = SearchOptions::FORWARD;
		let start = i64::try_from(display_len("😀b")).unwrap();
		assert_eq!(reader_search(&buffer, "b", start, options), i64::try_from(display_len("😀b😀")).unwrap());
	}

	#[test]
	fn reader_search_with_wrap_backward_starts_from_the_display_end() {
		let buffer = DocumentBuffer::with_content("😀😀a".to_string());
		let result = reader_search_with_wrap(&buffer, "a", 0, SearchOptions::empty());
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, i64::try_from(display_len("😀😀")).unwrap());
	}

	#[test]
	fn reader_search_all_returns_display_spans() {
		let buffer = DocumentBuffer::with_content("😀ab 😀ab".to_string());
		let spans = reader_search_all(&buffer, "ab", SearchOptions::empty());
		let first = i64::try_from(display_len("😀")).unwrap();
		let second = i64::try_from(display_len("😀ab 😀")).unwrap();
		assert_eq!(spans, vec![(first, first + 2), (second, second + 2)]);
	}

	#[cfg(not(any(windows, target_os = "macos")))]
	#[test]
	fn reader_search_counts_scalars_off_windows_and_macos() {
		let buffer = DocumentBuffer::with_content("a😀b".to_string());
		assert_eq!(reader_search(&buffer, "b", 0, SearchOptions::FORWARD), 2);
	}

	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn reader_search_counts_utf16_units_on_windows_and_macos() {
		let buffer = DocumentBuffer::with_content("a😀b".to_string());
		assert_eq!(reader_search(&buffer, "b", 0, SearchOptions::FORWARD), 3);
	}

	#[test]
	fn reader_search_handles_match_case() {
		let buffer = DocumentBuffer::with_content("Hello hello".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::MATCH_CASE;
		assert_eq!(reader_search(&buffer, "hello", 0, options), 6);
		assert_eq!(reader_search(&buffer, "Hello", 0, options), 0);
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "HELLO", 0, options), 0);
	}

	#[test]
	fn reader_search_with_wrap_wraps_forward() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::FORWARD;
		let result = reader_search_with_wrap(&buffer, "a", 1, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 0);
	}

	#[test]
	fn reader_search_backward_finds_previous_match() {
		let buffer = DocumentBuffer::with_content("one two one".to_string());
		let options = SearchOptions::empty();
		assert_eq!(reader_search(&buffer, "one", 11, options), 8);
	}

	#[test]
	fn reader_search_with_regex_invalid_pattern_returns_not_found() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::REGEX;
		assert_eq!(reader_search(&buffer, "(", 0, options), -1);
	}

	#[test]
	fn reader_search_whole_word_positive_case() {
		let buffer = DocumentBuffer::with_content("alpha beta gamma".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(&buffer, "beta", 0, options), 6);
	}

	#[test]
	fn reader_search_clamps_negative_start_to_zero() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "a", -500, options), 0);
	}

	#[test]
	fn reader_search_clamps_start_past_the_document_end() {
		let buffer = DocumentBuffer::with_content("😀abc".to_string());
		assert_eq!(reader_search(&buffer, "a", i64::MAX, SearchOptions::FORWARD), -1);
		assert_eq!(
			reader_search(&buffer, "a", i64::MAX, SearchOptions::empty()),
			i64::try_from(display_len("😀")).unwrap()
		);
	}

	#[test]
	fn reader_search_handles_an_empty_buffer() {
		let buffer = DocumentBuffer::new();
		assert_eq!(reader_search(&buffer, "a", 0, SearchOptions::FORWARD), -1);
		assert_eq!(reader_search(&buffer, "a", i64::MAX, SearchOptions::empty()), -1);
		assert!(reader_search_all(&buffer, "a", SearchOptions::empty()).is_empty());
		assert!(!reader_search_with_wrap(&buffer, "a", 0, SearchOptions::empty()).found);
	}

	#[test]
	fn reader_search_with_wrap_wraps_backward() {
		let buffer = DocumentBuffer::with_content("abca".to_string());
		let options = SearchOptions::empty();
		let result = reader_search_with_wrap(&buffer, "a", 0, options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 3);
	}

	#[test]
	fn reader_search_all_returns_every_plain_match_in_order() {
		let buffer = DocumentBuffer::with_content("blah on page one, blah again, and blah".to_string());
		let options = SearchOptions::empty();
		let spans = reader_search_all(&buffer, "blah", options);
		assert_eq!(spans.len(), 3);
		assert_eq!(spans[0].0, 0);
		// "blah" is four ASCII chars, so every span is four display units long.
		assert!(spans.iter().all(|(start, end)| end - start == 4));
		assert!(spans.windows(2).all(|w| w[0].0 < w[1].0));
	}

	#[test]
	fn reader_search_all_respects_match_case() {
		let buffer = DocumentBuffer::with_content("Hello hello HELLO".to_string());
		let case_sensitive = SearchOptions::MATCH_CASE;
		assert_eq!(reader_search_all(&buffer, "hello", case_sensitive).len(), 1);
		let insensitive = SearchOptions::empty();
		assert_eq!(reader_search_all(&buffer, "hello", insensitive).len(), 3);
	}

	#[test]
	fn reader_search_all_respects_whole_word() {
		let buffer = DocumentBuffer::with_content("the cat and the theater".to_string());
		let options = SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search_all(&buffer, "the", options).len(), 2);
		let options = SearchOptions::empty();
		assert_eq!(reader_search_all(&buffer, "the", options).len(), 3);
	}

	#[test]
	fn reader_search_all_regex_spans_the_actual_match() {
		let buffer = DocumentBuffer::with_content("12 and 345 and 6".to_string());
		let options = SearchOptions::REGEX;
		let spans = reader_search_all(&buffer, r"\d+", options);
		assert_eq!(spans.len(), 3);
		// Each span covers the whole run of digits, not just one.
		assert_eq!((spans[0].1 - spans[0].0), 2);
		assert_eq!((spans[1].1 - spans[1].0), 3);
		assert_eq!((spans[2].1 - spans[2].0), 1);
	}

	#[test]
	fn reader_search_all_skips_zero_length_matches() {
		// `a*` also matches the empty gaps between characters; only the real run should survive.
		let buffer = DocumentBuffer::with_content("aaab".to_string());
		let options = SearchOptions::REGEX;
		let spans = reader_search_all(&buffer, "a*", options);
		assert_eq!(spans.len(), 1);
		assert_eq!((spans[0].0, spans[0].1), (0, 3));
	}

	#[test]
	fn reader_search_all_invalid_regex_yields_no_spans() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::REGEX;
		assert!(reader_search_all(&buffer, "(", options).is_empty());
		assert!(reader_search_all(&buffer, "", options).is_empty());
	}
}
