//! Word/line/char counting for a document's full text, parallelized by splitting the text into
//! roughly-equal chunks (one per core) and merging each chunk's counts, correcting for words
//! split across a chunk boundary (see [`ChunkStats`]).

use rayon::prelude::*;

use crate::util::text::is_space_like;

#[derive(Debug, Clone, Default)]
pub struct DocumentStats {
	pub word_count: usize,
	pub line_count: usize,
	pub char_count: usize,
	pub char_count_no_whitespace: usize,
	/// Number of recorded audio files, when the document has any. Filled in by
	/// [`crate::document::Document::compute_stats`] from the document's audio timeline, not by
	/// [`DocumentStats::from_text`], which only ever sees the text spine.
	pub audio_file_count: usize,
	/// Total playback length across every recorded audio file, in milliseconds. See
	/// `audio_file_count`.
	pub audio_total_duration_ms: u64,
}

/// Per-chunk counts from [`count_chunk`], plus the whitespace-ness of the chunk's first and last
/// chars so [`DocumentStats::from_text`] can correct `word_count` for words split across a chunk
/// boundary (each half otherwise counts as its own word).
struct ChunkStats {
	char_count: usize,
	char_count_no_whitespace: usize,
	newline_count: usize,
	word_count: usize,
	first_is_whitespace: bool,
	last_is_whitespace: bool,
}

fn count_chunk(text: &str) -> ChunkStats {
	let mut char_count = 0usize;
	let mut char_count_no_whitespace = 0usize;
	let mut newline_count = 0usize;
	let mut word_count = 0usize;
	let mut in_word = false;
	let mut first_is_whitespace = false;
	let mut last_is_whitespace = false;
	for (i, c) in text.chars().enumerate() {
		if i == 0 {
			first_is_whitespace = c.is_whitespace();
		}
		last_is_whitespace = c.is_whitespace();
		char_count += 1;
		if !is_space_like(c) {
			char_count_no_whitespace += 1;
		}
		if c == '\n' {
			newline_count += 1;
		}
		if c.is_whitespace() {
			in_word = false;
		} else if !in_word {
			in_word = true;
			word_count += 1;
		}
	}
	ChunkStats {
		char_count,
		char_count_no_whitespace,
		newline_count,
		word_count,
		first_is_whitespace,
		last_is_whitespace,
	}
}

/// Splits `text` into up to `target_chunks` roughly-equal pieces, snapped to char boundaries, for
/// parallel counting. Never splits mid-char, and returns fewer/no chunks for short input.
fn split_into_chunks(text: &str, target_chunks: usize) -> Vec<&str> {
	if text.is_empty() || target_chunks <= 1 {
		return vec![text];
	}
	let chunk_len = text.len().div_ceil(target_chunks).max(1);
	let mut chunks = Vec::with_capacity(target_chunks);
	let mut start = 0;
	while start < text.len() {
		let mut end = (start + chunk_len).min(text.len());
		while end < text.len() && !text.is_char_boundary(end) {
			end += 1;
		}
		chunks.push(&text[start..end]);
		start = end;
	}
	chunks
}

impl DocumentStats {
	#[must_use]
	pub fn from_text(text: &str) -> Self {
		Self::from_text_with_chunk_count(text, rayon::current_num_threads())
	}

	fn from_text_with_chunk_count(text: &str, target_chunks: usize) -> Self {
		let chunks = split_into_chunks(text, target_chunks);
		let counted: Vec<ChunkStats> = chunks.into_par_iter().map(count_chunk).collect();
		let mut char_count = 0usize;
		let mut char_count_no_whitespace = 0usize;
		let mut newline_count = 0usize;
		let mut word_count = 0usize;
		let mut prev_last_is_whitespace = true;
		for chunk in &counted {
			char_count += chunk.char_count;
			char_count_no_whitespace += chunk.char_count_no_whitespace;
			newline_count += chunk.newline_count;
			word_count += chunk.word_count;
			// A word split across the boundary was counted once by each neighboring chunk; merge them.
			if chunk.char_count > 0 && !prev_last_is_whitespace && !chunk.first_is_whitespace {
				word_count -= 1;
			}
			if chunk.char_count > 0 {
				prev_last_is_whitespace = chunk.last_is_whitespace;
			}
		}
		let line_count = if text.is_empty() {
			0
		} else if text.ends_with('\n') {
			newline_count
		} else {
			newline_count + 1
		};
		Self { word_count, line_count, char_count, char_count_no_whitespace, ..Self::default() }
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[test]
	fn document_stats_counts_words_lines_and_chars() {
		let stats = DocumentStats::from_text("a b\nc");
		assert_eq!(stats.word_count, 3);
		assert_eq!(stats.line_count, 2);
		assert_eq!(stats.char_count, 5);
		assert_eq!(stats.char_count_no_whitespace, 3);
	}

	/// Naive reference matching the pre-parallel single-pass implementation, used to check the
	/// chunked version agrees regardless of how the text happens to get split.
	fn naive_stats(text: &str) -> (usize, usize, usize, usize) {
		let char_count = text.chars().count();
		let line_count = text.lines().count();
		let word_count = text.split_whitespace().count();
		let char_count_no_whitespace = text.chars().filter(|c| !is_space_like(*c)).count();
		(word_count, line_count, char_count, char_count_no_whitespace)
	}

	#[rstest]
	#[case("")]
	#[case(" ")]
	#[case("a")]
	#[case("hello world")]
	#[case("a b\nc")]
	#[case("word-boundary straddles the chunk split here on purpose across many words")]
	#[case("   leading and trailing whitespace   ")]
	#[case("line1\nline2\nline3\n")]
	#[case("line1\nline2\nline3")]
	#[case("no newlines at all just words words words")]
	#[case("münchhausen café naïve 日本語 emoji 🎉🎉 straddling")]
	#[case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")]
	#[case("x x x x x x x x x x x x x x x x word y y y y y y y y y y y y y y y y")]
	fn from_text_with_chunk_count_matches_naive_single_pass_for_any_chunk_count(#[case] text: &str) {
		let (word_count, line_count, char_count, char_count_no_whitespace) = naive_stats(text);
		for target_chunks in [1, 2, 3, 4, 7, 16, 64] {
			let stats = DocumentStats::from_text_with_chunk_count(text, target_chunks);
			assert_eq!(
				stats.word_count, word_count,
				"word_count mismatch at target_chunks={target_chunks} for {text:?}"
			);
			assert_eq!(
				stats.line_count, line_count,
				"line_count mismatch at target_chunks={target_chunks} for {text:?}"
			);
			assert_eq!(
				stats.char_count, char_count,
				"char_count mismatch at target_chunks={target_chunks} for {text:?}"
			);
			assert_eq!(
				stats.char_count_no_whitespace, char_count_no_whitespace,
				"char_count_no_whitespace mismatch at target_chunks={target_chunks} for {text:?}"
			);
		}
	}

	#[test]
	fn split_into_chunks_never_splits_mid_char_and_preserves_content() {
		let text = "münchhausen café naïve 日本語 emoji 🎉🎉 straddling boundaries on purpose";
		for target_chunks in [1, 2, 3, 5, 8, 32] {
			let chunks = split_into_chunks(text, target_chunks);
			assert_eq!(chunks.concat(), text);
			for chunk in &chunks {
				assert!(text.contains(chunk));
			}
		}
	}
}
