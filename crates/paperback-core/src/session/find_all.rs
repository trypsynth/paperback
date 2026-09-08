//! The Find All result rows: every line of a document that holds at least one match for a
//! query, in document order, each with the page it sits on and the full list of that line's
//! match spans. Built by [`DocumentSession::find_all_lines`] from
//! [`crate::reader_core::reader_search_all`]; the UI renders one row per line and jumps to the
//! chosen match span.

use super::DocumentSession;
use crate::{document::MarkerType, reader_core::SearchOptions};

/// One match's extent, in the same display units (UTF-16 code units on Windows/macOS) as the
/// GUI caret and page markers, so a caller can hand it straight to a range selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindSpan {
	pub start: i64,
	pub end: i64,
}

/// One result line: the trimmed text of a line holding at least one match, the page it is on
/// (`0` when the document has no page markers), and every match's span on that line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindAllLine {
	pub text: String,
	pub page: i32,
	pub matches: Vec<FindSpan>,
}

impl DocumentSession {
	/// Every line that contains at least one match of `query`, one row per distinct line in
	/// document order (a line holding several matches appears once). Match spans and the `page`
	/// are computed in display units, so a caller can compare spans against the caret and select
	/// ranges without further conversion. Returns an empty list for an empty query, an invalid
	/// regex, or a document with no matches.
	#[must_use]
	pub fn find_all_lines(&self, query: &str, options: SearchOptions) -> Vec<FindAllLine> {
		let buffer = &self.handle.document().buffer;
		let matches = crate::reader_core::reader_search_all(&buffer.content, query, options);
		if matches.is_empty() {
			return Vec::new();
		}
		let newline_bytes: Vec<usize> = buffer.content.match_indices('\n').map(|(idx, _)| idx).collect();
		let content_len = buffer.content.len();
		// The page-break offsets once, so each result line's page is a binary search instead of
		// recounting every marker per line (which would scale with lines x markers).
		let pagebreak_offsets: Vec<i64> = buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| i64::try_from(marker.position).unwrap_or(i64::MAX))
			.collect();
		let mut rows: Vec<FindAllLine> = Vec::new();
		let mut open_line_start: Option<usize> = None;
		for (start, end) in matches {
			let start_byte = buffer.byte_index_for_display(usize::try_from(start.max(0)).unwrap_or(0)).min(content_len);
			let line_idx = newline_bytes.partition_point(|&p| p < start_byte);
			let line_start = if line_idx == 0 { 0 } else { newline_bytes[line_idx - 1] + 1 };
			let line_end = newline_bytes.get(line_idx).copied().unwrap_or(content_len);
			if open_line_start != Some(line_start) {
				let text = buffer.content[line_start..line_end].trim().to_string();
				if text.is_empty() {
					continue;
				}
				// The page number is how many page-break markers sit at or before the line; a line
				// before the first marker still belongs to page 1.
				let page = if pagebreak_offsets.is_empty() {
					0
				} else {
					i32::try_from(pagebreak_offsets.partition_point(|&offset| offset <= start)).unwrap_or(1).max(1)
				};
				rows.push(FindAllLine { text, page, matches: Vec::new() });
				open_line_start = Some(line_start);
			}
			if let Some(row) = rows.last_mut() {
				row.matches.push(FindSpan { start, end });
			}
		}
		rows
	}
}
