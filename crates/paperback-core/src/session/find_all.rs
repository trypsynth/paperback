use super::DocumentSession;
use crate::{
	document::{ContentBytePos, DocPos, MarkerType},
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
		let newline_bytes: Vec<ContentBytePos> =
			buffer.content.match_indices('\n').map(|(idx, _)| ContentBytePos::from_raw(idx)).collect();
		let content_end = buffer.content_end();
		// Collected once, so each line's page is a binary search rather than a walk over every marker.
		let pagebreak_offsets: Vec<DocPos> = buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.position)
			.collect();
		let mut rows: Vec<FindAllLine> = Vec::new();
		let mut open_line_start: Option<ContentBytePos> = None;
		for (start, end) in matches {
			let start_byte = buffer.byte_at(start);
			let line_idx = newline_bytes.partition_point(|&p| p < start_byte);
			let line_start = if line_idx == 0 { ContentBytePos::ZERO } else { newline_bytes[line_idx - 1] + 1 };
			let line_end = newline_bytes.get(line_idx).copied().unwrap_or(content_end);
			if open_line_start != Some(line_start) {
				let text = buffer.slice(line_start..line_end).trim().to_string();
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
