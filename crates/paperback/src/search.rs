use bitflags::bitflags;
use paperback_core::reader_core;

#[derive(Clone, Debug, Default)]
pub struct SearchResult {
	pub found: bool,
	pub wrapped: bool,
	pub position: i64,
}

bitflags! {
	#[derive(Copy, Clone, Default)]
	pub struct FindOptions: u8 {
		const NONE = 0;
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const MATCH_WHOLE_WORD = 1 << 2;
		const USE_REGEX = 1 << 3;
	}
}

pub fn find_text_with_wrap(haystack: &str, needle: &str, start: i64, options: FindOptions) -> SearchResult {
	if needle.is_empty() {
		return SearchResult::default();
	}
	let mut search_options = reader_core::SearchOptions::empty();
	if options.contains(FindOptions::FORWARD) {
		search_options |= reader_core::SearchOptions::FORWARD;
	}
	if options.contains(FindOptions::MATCH_CASE) {
		search_options |= reader_core::SearchOptions::MATCH_CASE;
	}
	if options.contains(FindOptions::MATCH_WHOLE_WORD) {
		search_options |= reader_core::SearchOptions::WHOLE_WORD;
	}
	if options.contains(FindOptions::USE_REGEX) {
		search_options |= reader_core::SearchOptions::REGEX;
	}
	let result = reader_core::reader_search_with_wrap(haystack, needle, start, search_options);
	SearchResult { found: result.found, wrapped: result.wrapped, position: result.position }
}
