use patois::nt;

use super::{ChapterProgressFfi, DocumentSession};
use crate::document::DocPos;

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	/// Empty where [`Self::chapter_announcement`] has no answer; `elapsed_ms` is negative when no recording is playing.
	#[must_use]
	pub fn chapter_announcement_ffi(&self, position: DocPos, elapsed_ms: i64) -> String {
		self.chapter_announcement(position, u64::try_from(elapsed_ms).ok()).unwrap_or_default()
	}

	/// What [`Self::chapter_announcement_ffi`] says, in parts, for an app that words it in its own translations.
	#[must_use]
	pub fn chapter_progress_ffi(&self, position: DocPos, elapsed_ms: i64) -> ChapterProgressFfi {
		let by_time = u64::try_from(elapsed_ms).ok().and_then(|ms| self.audio_chapter_span(ms));
		if let Some((title, elapsed_ms, length_ms)) = by_time {
			let percent = (elapsed_ms * 100).checked_div(length_ms).unwrap_or(100).min(100);
			return ChapterProgressFfi {
				found: true,
				title,
				percent: i32::try_from(percent).unwrap_or(100),
				words_left: -1,
				elapsed_ms: i64::try_from(elapsed_ms).unwrap_or(i64::MAX),
				total_ms: i64::try_from(length_ms).unwrap_or(i64::MAX),
			};
		}
		if self.handle.document().audio_only {
			return ChapterProgressFfi::default();
		}
		let (title, percent, words_left) = self.text_chapter_span(position);
		ChapterProgressFfi {
			found: true,
			title,
			percent,
			words_left: i64::try_from(words_left).unwrap_or(i64::MAX),
			elapsed_ms: -1,
			total_ms: -1,
		}
	}
}

impl DocumentSession {
	/// How far into the chapter the reader is and how much of it is left: by time where the recording at `elapsed_ms` can be placed, otherwise by the text at `position`. `None` only for a book that is all audio whose files' lengths aren't known, as its text is a placeholder line per file.
	#[must_use]
	pub fn chapter_announcement(&self, position: DocPos, elapsed_ms: Option<u64>) -> Option<String> {
		if let Some(time) = elapsed_ms.and_then(|ms| self.audio_chapter_announcement(ms)) {
			return Some(time);
		}
		if self.handle.document().audio_only {
			return None;
		}
		Some(self.text_chapter_announcement(position))
	}

	/// The chapter `position` is in, how far through it by characters and how many words of it are left, such as "Chapter 3, 42%, 1850 words left".
	#[must_use]
	pub fn text_chapter_announcement(&self, position: DocPos) -> String {
		let (title, percent, words_left) = self.text_chapter_span(position);
		let mut parts = Vec::with_capacity(3);
		if !title.is_empty() {
			parts.push(title);
		}
		parts.push(format!("{percent}%"));
		// TRANSLATORS: Announced with how far through a book's chapter the reader is: the number of words left in it. %d is the number.
		let words = nt("%d word left", "%d words left", u64::try_from(words_left).unwrap_or(u64::MAX));
		parts.push(words.replacen("%d", &words_left.to_string(), 1));
		parts.join(", ")
	}

	/// The title of the chapter `position` is in, the percentage through it by characters and the words left in it. Chapters are placed as [`Self::audio_chapter_span`] places them: the table of contents' entries, and of entries starting together the deepest. Before the first entry, or in a book without any, the whole stretch is the chapter, untitled.
	fn text_chapter_span(&self, position: DocPos) -> (String, i32, usize) {
		let end_of_book = self.handle.document().buffer.end();
		let position = position.min(end_of_book);
		let toc = self.get_toc();
		let current =
			toc.iter().filter(|entry| entry.position <= position).max_by_key(|entry| (entry.position, entry.level));
		let start = current.map_or(DocPos::ZERO, |entry| entry.position);
		let end = toc.iter().map(|entry| entry.position).filter(|next| *next > start).min().unwrap_or(end_of_book);
		let end = end.min(end_of_book);
		let read = position.get().saturating_sub(start.get());
		let length = end.get().saturating_sub(start.get());
		let percent = (read * 100).checked_div(length).unwrap_or(100).min(100);
		let words_left = self.text_range(position, end).split_whitespace().count();
		let title = current.map_or("", |entry| entry.title.trim()).to_owned();
		(title, i32::try_from(percent).unwrap_or(100), words_left)
	}
}
