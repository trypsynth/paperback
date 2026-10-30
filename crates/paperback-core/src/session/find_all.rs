use super::DocumentSession;
use crate::{
	document::{DocLen, DocPos, MarkerType},
	reader_core::SearchOptions,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindSpan {
	pub start: DocPos,
	pub end: DocPos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindAllLine {
	pub text: String,
	/// `0` when the document has no page markers.
	pub page: i32,
	pub matches: Vec<FindSpan>,
}

impl DocumentSession {
	#[must_use]
	pub fn find_all_lines(&self, query: &str, options: SearchOptions) -> Vec<FindAllLine> {
		let buffer = &self.handle.document().buffer;
		let matches = crate::reader_core::reader_search_all(buffer, query, options);
		if matches.is_empty() {
			return Vec::new();
		}
		let newlines = buffer.newline_positions();
		// Collected once, so each line's page is a binary search rather than a walk over every marker.
		let pagebreak_offsets: Vec<DocPos> = buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		let mut rows: Vec<FindAllLine> = Vec::new();
		let mut open_line_start: Option<DocPos> = None;
		for (start, end) in matches {
			let line_idx = newlines.partition_point(|&p| p < start);
			let line_start = if line_idx == 0 { DocPos::ZERO } else { newlines[line_idx - 1] + DocLen::ONE };
			let line_end = newlines.get(line_idx).copied().unwrap_or_else(|| buffer.end());
			if open_line_start != Some(line_start) {
				let text = buffer.text(line_start..line_end).trim().to_string();
				if text.is_empty() {
					continue;
				}
				// A line before the first page break still belongs to page 1.
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
