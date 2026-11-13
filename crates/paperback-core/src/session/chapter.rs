use patois::nt;

use super::DocumentSession;
use crate::document::DocPos;

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	/// Empty where [`Self::chapter_announcement`] has no answer; `elapsed_ms` is negative when no recording is playing.
	#[must_use]
	pub fn chapter_announcement_ffi(&self, position: DocPos, elapsed_ms: i64) -> String {
		self.chapter_announcement(position, u64::try_from(elapsed_ms).ok()).unwrap_or_default()
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

	/// The chapter `position` is in, how far through it by characters and how many words of it are left, such as "Chapter 3, 42%, 1850 words left". Chapters are placed as [`Self::audio_chapter_announcement`] places them: the table of contents' entries, and of entries starting together the deepest. Before the first entry, or in a book without any, the whole stretch is the chapter, untitled.
	#[must_use]
	pub fn text_chapter_announcement(&self, position: DocPos) -> String {
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
		let mut parts = Vec::with_capacity(3);
		if let Some(title) = current.map(|entry| entry.title.trim()).filter(|title| !title.is_empty()) {
			parts.push(title.to_owned());
		}
		parts.push(format!("{percent}%"));
		// TRANSLATORS: Announced with how far through a book's chapter the reader is: the number of words left in it. %d is the number.
		let words = nt("%d word left", "%d words left", u64::try_from(words_left).unwrap_or(u64::MAX));
		parts.push(words.replacen("%d", &words_left.to_string(), 1));
		parts.join(", ")
	}
}
