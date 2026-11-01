use bitflags::bitflags;
use regex::{Regex, RegexBuilder};

use crate::{
	document::{ContentBytePos, DocPos, DocumentBuffer},
	types as ffi,
};

bitflags! {
	#[derive(Copy, Clone)]
	pub struct SearchOptions: u8 {
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const WHOLE_WORD = 1 << 2;
		const REGEX = 1 << 3;
	}
}

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
pub fn reader_search(buffer: &DocumentBuffer, needle: &str, start: DocPos, options: SearchOptions) -> Option<DocPos> {
	search_range(buffer, needle, start, options).map(|(found, _)| found)
}

fn search_range(
	buffer: &DocumentBuffer,
	needle: &str,
	start: DocPos,
	options: SearchOptions,
) -> Option<(DocPos, DocPos)> {
	if needle.is_empty() {
		return None;
	}
	let start_byte = buffer.byte_at(start);
	// A case-insensitive regex avoids copying and lowercasing the whole haystack.
	let re = build_matcher(needle, options)?;
	let found = if options.contains(SearchOptions::FORWARD) {
		re.find(buffer.slice(start_byte..)).map(|m| (start_byte + m.start(), start_byte + m.end()))
	} else {
		re.find_iter(buffer.slice(..start_byte))
			.last()
			.map(|m| (ContentBytePos::from_raw(m.start()), ContentBytePos::from_raw(m.end())))
	};
	found.map(|(from, to)| (buffer.pos_at_byte(from), buffer.pos_at_byte(to)))
}

#[must_use]
pub fn reader_search_all(buffer: &DocumentBuffer, needle: &str, options: SearchOptions) -> Vec<(DocPos, DocPos)> {
	if needle.is_empty() {
		return Vec::new();
	}
	let Some(re) = build_matcher(needle, options) else {
		return Vec::new();
	};
	re.find_iter(&buffer.content)
		.filter(|m| !m.is_empty())
		.map(|m| {
			(
				buffer.pos_at_byte(ContentBytePos::from_raw(m.start())),
				buffer.pos_at_byte(ContentBytePos::from_raw(m.end())),
			)
		})
		.collect()
}

#[must_use]
pub fn reader_search_with_wrap(
	buffer: &DocumentBuffer,
	needle: &str,
	start: DocPos,
	options: SearchOptions,
) -> ffi::SearchResult {
	if let Some((position, end)) = search_range(buffer, needle, start, options) {
		return ffi::SearchResult { found: true, wrapped: false, position, end };
	}
	let wrap_pos = if options.contains(SearchOptions::FORWARD) { DocPos::ZERO } else { buffer.end() };
	if let Some((position, end)) = search_range(buffer, needle, wrap_pos, options) {
		return ffi::SearchResult { found: true, wrapped: true, position, end };
	}
	ffi::SearchResult { found: false, wrapped: false, position: DocPos::ZERO, end: DocPos::ZERO }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::DocLen;

	#[test]
	fn reader_search_handles_basic_and_whole_word() {
		let buffer = DocumentBuffer::with_content("Hello world".to_string());
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "hello", 0.into(), options), Some(0.into()));
		let buffer = DocumentBuffer::with_content("hello_world".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(&buffer, "hello", 0.into(), options), None);
	}

	#[test]
	fn reader_search_returns_positions() {
		let buffer = DocumentBuffer::with_content("a😀b".to_string());
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "b", 0.into(), options), Some(DocLen::of("a😀").as_pos()));
	}

	#[test]
	fn reader_search_starts_from_a_position() {
		let buffer = DocumentBuffer::with_content("😀b😀b".to_string());
		let options = SearchOptions::FORWARD;
		let start = DocLen::of("😀b").as_pos();
		assert_eq!(reader_search(&buffer, "b", start, options), Some(DocLen::of("😀b😀").as_pos()));
	}

	#[test]
	fn reader_search_with_wrap_backward_starts_from_the_end() {
		let buffer = DocumentBuffer::with_content("😀😀a".to_string());
		let result = reader_search_with_wrap(&buffer, "a", 0.into(), SearchOptions::empty());
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, DocLen::of("😀😀").as_pos());
	}

	/// Find selects to here, so a regex must report what it matched rather than the query's own length.
	#[test]
	fn a_regex_match_ends_where_its_text_does() {
		let buffer = DocumentBuffer::with_content("😀 page 1234 of 9".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::REGEX;
		let result = reader_search_with_wrap(&buffer, r"\d+", 0.into(), options);
		assert_eq!(result.position, DocLen::of("😀 page ").as_pos());
		assert_eq!(result.end, DocLen::of("😀 page 1234").as_pos());
		let back = reader_search_with_wrap(&buffer, r"\d+", result.end, SearchOptions::REGEX);
		assert_eq!((back.position, back.end), (result.position, result.end));
	}

	#[test]
	fn reader_search_all_returns_position_spans() {
		let buffer = DocumentBuffer::with_content("😀ab 😀ab".to_string());
		let spans = reader_search_all(&buffer, "ab", SearchOptions::empty());
		let first = DocLen::of("😀").as_pos();
		let second = DocLen::of("😀ab 😀").as_pos();
		let ab = DocLen::of("ab");
		assert_eq!(spans, vec![(first, first + ab), (second, second + ab)]);
	}

	#[test]
	fn reader_search_counts_an_astral_char_as_one_position() {
		let buffer = DocumentBuffer::with_content("a😀b".to_string());
		assert_eq!(reader_search(&buffer, "b", 0.into(), SearchOptions::FORWARD), Some(2.into()));
	}

	#[test]
	fn reader_search_handles_match_case() {
		let buffer = DocumentBuffer::with_content("Hello hello".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::MATCH_CASE;
		assert_eq!(reader_search(&buffer, "hello", 0.into(), options), Some(6.into()));
		assert_eq!(reader_search(&buffer, "Hello", 0.into(), options), Some(0.into()));
		let options = SearchOptions::FORWARD;
		assert_eq!(reader_search(&buffer, "HELLO", 0.into(), options), Some(0.into()));
	}

	#[test]
	fn reader_search_with_wrap_wraps_forward() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::FORWARD;
		let result = reader_search_with_wrap(&buffer, "a", 1.into(), options);
		assert!(result.found);
		assert!(result.wrapped);
		assert_eq!(result.position, 0);
	}

	#[test]
	fn reader_search_backward_finds_previous_match() {
		let buffer = DocumentBuffer::with_content("one two one".to_string());
		let options = SearchOptions::empty();
		assert_eq!(reader_search(&buffer, "one", 11.into(), options), Some(8.into()));
	}

	#[test]
	fn reader_search_with_regex_invalid_pattern_returns_not_found() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::REGEX;
		assert_eq!(reader_search(&buffer, "(", 0.into(), options), None);
	}

	#[test]
	fn reader_search_whole_word_positive_case() {
		let buffer = DocumentBuffer::with_content("alpha beta gamma".to_string());
		let options = SearchOptions::FORWARD | SearchOptions::WHOLE_WORD;
		assert_eq!(reader_search(&buffer, "beta", 0.into(), options), Some(6.into()));
	}

	#[test]
	fn reader_search_clamps_start_past_the_document_end() {
		let buffer = DocumentBuffer::with_content("😀abc".to_string());
		let past_the_end = DocPos::from_raw(usize::MAX);
		assert_eq!(reader_search(&buffer, "a", past_the_end, SearchOptions::FORWARD), None);
		assert_eq!(reader_search(&buffer, "a", past_the_end, SearchOptions::empty()), Some(DocLen::of("😀").as_pos()));
	}

	#[test]
	fn reader_search_handles_an_empty_buffer() {
		let buffer = DocumentBuffer::new();
		assert_eq!(reader_search(&buffer, "a", 0.into(), SearchOptions::FORWARD), None);
		assert_eq!(reader_search(&buffer, "a", DocPos::from_raw(usize::MAX), SearchOptions::empty()), None);
		assert!(reader_search_all(&buffer, "a", SearchOptions::empty()).is_empty());
		assert!(!reader_search_with_wrap(&buffer, "a", 0.into(), SearchOptions::empty()).found);
	}

	#[test]
	fn reader_search_with_wrap_wraps_backward() {
		let buffer = DocumentBuffer::with_content("abca".to_string());
		let options = SearchOptions::empty();
		let result = reader_search_with_wrap(&buffer, "a", 0.into(), options);
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
		assert!(spans.iter().all(|&(start, end)| end - start == 4));
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
		assert_eq!((spans[0].1 - spans[0].0), 2);
		assert_eq!((spans[1].1 - spans[1].0), 3);
		assert_eq!((spans[2].1 - spans[2].0), 1);
	}

	#[test]
	fn reader_search_all_skips_zero_length_matches() {
		let buffer = DocumentBuffer::with_content("aaab".to_string());
		let options = SearchOptions::REGEX;
		let spans = reader_search_all(&buffer, "a*", options);
		assert_eq!(spans.len(), 1);
		assert_eq!((spans[0].0, spans[0].1), (0.into(), 3.into()));
	}

	#[test]
	fn reader_search_all_invalid_regex_yields_no_spans() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		let options = SearchOptions::REGEX;
		assert!(reader_search_all(&buffer, "(", options).is_empty());
		assert!(reader_search_all(&buffer, "", options).is_empty());
	}
}
