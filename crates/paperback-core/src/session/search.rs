//! Content search: [`DocumentSession::search_ffi`] runs an in-memory search over the
//! rendered document text with the reader's case/whole-word/regex/direction options.

use super::{DocumentSession, SearchOptionsFfi, SearchResultFfi};
use crate::reader_core::{SearchOptions, reader_search_with_wrap};

impl DocumentSession {
	// `query: String` (not `&str`) because paperback.udl dictates this signature for UniFFI scaffolding.
	#[must_use]
	#[allow(clippy::needless_pass_by_value)]
	pub fn search_ffi(&self, query: String, start_position: i64, options: SearchOptionsFfi) -> SearchResultFfi {
		let mut search_options = SearchOptions::empty();
		if options.match_case {
			search_options.insert(SearchOptions::MATCH_CASE);
		}
		if options.whole_word {
			search_options.insert(SearchOptions::WHOLE_WORD);
		}
		if options.regex {
			search_options.insert(SearchOptions::REGEX);
		}
		if options.forward {
			search_options.insert(SearchOptions::FORWARD);
		}

		let result =
			reader_search_with_wrap(&self.handle.document().buffer.content, &query, start_position, search_options);
		SearchResultFfi { found: result.found, wrapped: result.wrapped, position: result.position }
	}
}
