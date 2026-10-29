use std::collections::HashSet;

use crate::t;

pub struct RawChapter {
	pub start_ms: u64,
	pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedChapter {
	pub start_ms: u64,
	pub end_ms: u64,
	pub title: String,
}

/// The first chapter is forced to start at 0 so the sections cover the whole file without gaps.
#[must_use]
pub fn normalize_chapters(chapters: &[RawChapter], duration_ms: u64, document_title: &str) -> Vec<NormalizedChapter> {
	let mut starts: Vec<(u64, String)> = chapters
		.iter()
		.filter(|chapter| chapter.start_ms < duration_ms)
		.map(|chapter| (chapter.start_ms, chapter.title.trim().to_string()))
		.collect();
	starts.sort_by_key(|(start_ms, _)| *start_ms);
	let mut seen = HashSet::new();
	starts.retain(|(start_ms, _)| seen.insert(*start_ms));
	if starts.is_empty() {
		return vec![NormalizedChapter { start_ms: 0, end_ms: duration_ms, title: document_title.to_string() }];
	}
	starts[0].0 = 0;
	starts
		.iter()
		.enumerate()
		.map(|(index, (start_ms, title))| {
			let title = if title.is_empty() {
				// TRANSLATORS: Fallback label for an audiobook chapter whose embedded title is empty; {} is the chapter number
				t("Chapter {}").replace("{}", &(index + 1).to_string())
			} else {
				title.clone()
			};
			let end_ms = starts.get(index + 1).map_or(duration_ms, |(next_start_ms, _)| *next_start_ms);
			NormalizedChapter { start_ms: *start_ms, end_ms, title }
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	fn raw(start_ms: u64, title: &str) -> RawChapter {
		RawChapter { start_ms, title: title.to_string() }
	}

	#[test]
	fn normalizes_chapters_into_sorted_contiguous_clips() {
		let chapters = vec![
			raw(5000, "Second"),
			raw(1000, " First "),
			raw(5000, "Duplicate"),
			raw(9000, ""),
			raw(12_000, "Past end"),
		];
		assert_eq!(
			normalize_chapters(&chapters, 10_000, "Book"),
			vec![
				NormalizedChapter { start_ms: 0, end_ms: 5000, title: "First".to_string() },
				NormalizedChapter { start_ms: 5000, end_ms: 9000, title: "Second".to_string() },
				NormalizedChapter { start_ms: 9000, end_ms: 10_000, title: "Chapter 3".to_string() },
			]
		);
	}

	#[test]
	fn chapterless_book_becomes_one_full_length_section() {
		assert_eq!(
			normalize_chapters(&[], 10_000, "Book"),
			vec![NormalizedChapter { start_ms: 0, end_ms: 10_000, title: "Book".to_string() }]
		);
	}
}
