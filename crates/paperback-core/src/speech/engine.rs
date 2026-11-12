//! What a speech engine provides: its voices, and audio for one paragraph at a time.

use anyhow::Result;

/// A voice an engine offers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voice {
	/// The engine's own identifier, saved in settings to find the voice again.
	pub id: String,
	/// The name the user picks it by.
	pub name: String,
	/// A BCP 47 tag, such as `en-US` or `fi`.
	pub language: String,
}

/// One paragraph to synthesize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechRequest {
	pub text: String,
	/// A [`Voice::id`], or `None` for the engine's default voice.
	pub voice: Option<String>,
	/// From 0 to 100, as a share of the engine's own range, since engines scale rate and pitch differently.
	pub rate_percent: u8,
	pub pitch_percent: u8,
}

/// Where a word starts in the audio, and which word it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordBoundary {
	/// Counted from the start of the chunk the word starts in.
	pub sample: usize,
	/// In chars of [`SpeechRequest::text`].
	pub text_offset: usize,
	pub text_len: usize,
}

/// A stretch of a paragraph's audio: mono 16-bit samples at [`Synthesis::sample_rate`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioChunk {
	pub samples: Vec<i16>,
	/// The words that start in this chunk.
	pub words: Vec<WordBoundary>,
}

/// A paragraph's audio, pulled a chunk at a time so playback can start before the engine has finished.
pub trait Synthesis: Send {
	fn sample_rate(&self) -> u32;

	/// The next chunk, or `None` once the paragraph is done.
	fn next_chunk(&mut self) -> Option<Result<AudioChunk>>;
}

/// A speech engine, such as Windows' `OneCore` voices or SAPI 5.
///
/// A speech plugin will implement this too, so it stays free of anything a plugin couldn't provide.
pub trait SpeechEngine: Send + Sync {
	/// Short and stable, such as `onecore`, since settings are kept under it.
	fn id(&self) -> &str;

	/// The voices installed now. They can change while the app runs.
	///
	/// # Errors
	///
	/// Returns why the engine couldn't list its voices.
	fn voices(&self) -> Result<Vec<Voice>>;

	/// Starts synthesizing `request`.
	///
	/// # Errors
	///
	/// Returns why the engine couldn't start, such as a voice that's been uninstalled.
	fn synthesize(&self, request: &SpeechRequest) -> Result<Box<dyn Synthesis>>;
}

/// Engines count text in UTF-16 code units, as their platforms do, and core counts chars, so a word boundary is converted with this. An offset inside a surrogate pair counts as the char it's in.
#[must_use]
pub fn utf16_to_char_offset(text: &str, utf16_offset: usize) -> usize {
	let mut units = 0;
	for (index, c) in text.chars().enumerate() {
		units += c.len_utf16();
		if units > utf16_offset {
			return index;
		}
	}
	text.chars().count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn utf16_offsets_become_char_offsets() {
		assert_eq!(utf16_to_char_offset("hello world", 6), 6);
		// "😀" is one char but two UTF-16 units.
		assert_eq!(utf16_to_char_offset("😀 smile", 3), 2);
		assert_eq!(utf16_to_char_offset("😀 smile", 1), 0);
		assert_eq!(utf16_to_char_offset("café", 100), 4);
	}
}
