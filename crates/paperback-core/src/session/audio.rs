use std::{
	fs::{self, File},
	io::BufReader,
	path::Path,
};

use zip::ZipArchive;

use super::{AudioClipFfi, AudioCursorFfi, AudioPointFfi, DocumentSession};
use crate::{
	audio::{AudioLocation, AudioTimeline, TimelinePoint, clock},
	document::DocPos,
	t,
	util::zip as zip_utils,
};

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	#[must_use]
	pub fn has_audio_ffi(&self) -> bool {
		self.audio().is_some_and(|timeline| !timeline.is_empty())
	}

	/// The text then only anchors the audio, so a read-aloud UI should navigate by elapsed time rather than by text units.
	#[must_use]
	pub fn is_audio_only_ffi(&self) -> bool {
		self.handle().document().audio_only && self.has_audio_ffi()
	}

	#[must_use]
	pub fn audio_source_count_ffi(&self) -> i32 {
		self.audio().map_or(0, |timeline| i32::try_from(timeline.sources().len()).unwrap_or(0))
	}

	/// Empty for a zip-embedded source, an out-of-range index, or a document with no audio.
	#[must_use]
	pub fn audio_source_direct_path_ffi(&self, index: i32) -> String {
		let Some(timeline) = self.audio() else { return String::new() };
		let Ok(index) = usize::try_from(index) else { return String::new() };
		match timeline.source(index).map(|source| &source.location) {
			Some(AudioLocation::File(path)) => path.clone(),
			_ => String::new(),
		}
	}

	/// For platforms whose player needs a real local path.
	#[must_use]
	#[allow(clippy::needless_pass_by_value, reason = "uniffi hands strings over by value")]
	pub fn audio_extract_source_ffi(&self, index: i32, output_path: String) -> bool {
		let Some(timeline) = self.audio() else { return false };
		let Ok(index) = usize::try_from(index) else { return false };
		let Some(source) = timeline.source(index) else { return false };
		match &source.location {
			AudioLocation::File(path) => fs::copy(path, &output_path).is_ok(),
			AudioLocation::ZipEntry { archive, entry, password } => {
				Self::extract_zip_audio_entry(archive, entry, password.as_deref(), &output_path)
			}
		}
	}

	#[must_use]
	pub fn audio_clip_count_ffi(&self) -> i32 {
		self.audio().map_or(0, |timeline| i32::try_from(timeline.clips().len()).unwrap_or(0))
	}

	#[must_use]
	pub fn audio_clip_ffi(&self, index: i32) -> AudioClipFfi {
		let Some(timeline) = self.audio() else { return AudioClipFfi::default() };
		let Ok(index) = usize::try_from(index) else { return AudioClipFfi::default() };
		let Some(clip) = timeline.clip(index) else { return AudioClipFfi::default() };
		AudioClipFfi {
			found: true,
			source: i32::try_from(clip.source).unwrap_or(0),
			clip_begin_ms: i64::try_from(clip.clip_begin_ms).unwrap_or(0),
			clip_end_ms: i64::try_from(clip.clip_end_ms).unwrap_or(0),
			start: clip.start,
			end: clip.end,
		}
	}

	#[must_use]
	pub fn audio_total_duration_ms_ffi(&self) -> i64 {
		self.audio().map_or(0, |timeline| i64::try_from(timeline.total_duration_ms()).unwrap_or(0))
	}

	#[must_use]
	pub fn audio_cursor_at_elapsed_ffi(&self, elapsed_ms: i64) -> AudioCursorFfi {
		let Some(timeline) = self.audio() else { return AudioCursorFfi::default() };
		let Ok(elapsed_ms) = u64::try_from(elapsed_ms) else { return AudioCursorFfi::default() };
		let Some(cursor) = timeline.cursor_at_elapsed(elapsed_ms) else { return AudioCursorFfi::default() };
		AudioCursorFfi {
			found: true,
			clip_index: i32::try_from(cursor.clip).unwrap_or(0),
			seek_ms: i64::try_from(cursor.seek_ms).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn audio_point_for_position_ffi(&self, position: DocPos) -> AudioPointFfi {
		let Some(timeline) = self.audio() else { return AudioPointFfi::default() };
		let position = position.min(self.handle.document().buffer.end());
		let Some(point) = timeline.point_for_position(position) else { return AudioPointFfi::default() };
		AudioPointFfi { found: true, position: point.position, time_ms: i64::try_from(point.time_ms).unwrap_or(0) }
	}

	/// `-1` when the source is unknown or the position falls before any recorded clip.
	#[must_use]
	pub fn audio_elapsed_for_source_position_ffi(&self, source: i32, raw_ms: i64) -> i64 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(source) = usize::try_from(source) else { return -1 };
		let Ok(raw_ms) = u64::try_from(raw_ms) else { return -1 };
		timeline.elapsed_for_source_position(source, raw_ms).and_then(|ms| i64::try_from(ms).ok()).unwrap_or(-1)
	}

	/// `-1` at the end of the book.
	#[must_use]
	pub fn audio_next_source_after_ffi(&self, current_source: i32) -> i32 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(current_source) = usize::try_from(current_source) else { return -1 };
		timeline.next_source_after(current_source).and_then(|source| i32::try_from(source).ok()).unwrap_or(-1)
	}

	/// `-1` at the start of the book.
	#[must_use]
	pub fn audio_previous_source_before_ffi(&self, current_source: i32) -> i32 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(current_source) = usize::try_from(current_source) else { return -1 };
		timeline.previous_source_before(current_source).and_then(|source| i32::try_from(source).ok()).unwrap_or(-1)
	}

	/// `-1` where [`Self::audio_progress_percent`] has no answer.
	#[must_use]
	pub fn audio_progress_percent_ffi(&self, elapsed_ms: i64) -> i32 {
		let Ok(elapsed_ms) = u64::try_from(elapsed_ms) else { return -1 };
		self.audio_progress_percent(elapsed_ms).unwrap_or(-1)
	}

	/// `-1` where [`Self::audio_elapsed_for_percent`] has no answer.
	#[must_use]
	pub fn audio_elapsed_for_percent_ffi(&self, percent: i32) -> i64 {
		self.audio_elapsed_for_percent(percent).and_then(|ms| i64::try_from(ms).ok()).unwrap_or(-1)
	}

	/// Whether a time in the recording can be placed: every file's length is known.
	#[must_use]
	pub fn audio_time_known_ffi(&self) -> bool {
		self.audio().is_some_and(AudioTimeline::durations_are_known)
	}

	/// A time in the recording written as Go to Time shows it, such as 1:02:03.
	#[must_use]
	pub fn format_audio_time_ffi(&self, ms: i64) -> String {
		clock::format_ms(u64::try_from(ms).unwrap_or(0))
	}

	/// What was typed into Go to Time, in milliseconds from the start: `-1` when it isn't a time, `-2` when it falls outside the recording, and `-3` when the recording's length isn't known. A leading `+` or `-` moves from `current_ms`.
	#[must_use]
	#[allow(clippy::needless_pass_by_value, reason = "uniffi hands strings over by value")]
	pub fn audio_time_entry_ffi(&self, text: String, current_ms: i64) -> i64 {
		let Some(timeline) = self.audio().filter(|timeline| timeline.durations_are_known()) else {
			return -3;
		};
		match clock::resolve_entry_ms(&text, u64::try_from(current_ms).unwrap_or(0), timeline.total_duration_ms()) {
			Ok(ms) => i64::try_from(ms).unwrap_or(-2),
			Err(clock::TimeEntryError::NotATime) => -1,
			Err(clock::TimeEntryError::OutOfRange) => -2,
		}
	}

	/// Empty where [`Self::audio_time_announcement`] has no answer.
	#[must_use]
	pub fn audio_time_announcement_ffi(&self, elapsed_ms: i64) -> String {
		u64::try_from(elapsed_ms).ok().and_then(|ms| self.audio_time_announcement(ms)).unwrap_or_default()
	}

	/// Empty where [`Self::audio_chapter_announcement`] has no answer.
	#[must_use]
	pub fn audio_chapter_announcement_ffi(&self, elapsed_ms: i64) -> String {
		u64::try_from(elapsed_ms).ok().and_then(|ms| self.audio_chapter_announcement(ms)).unwrap_or_default()
	}
}

impl DocumentSession {
	fn extract_zip_audio_entry(archive: &str, entry: &str, password: Option<&str>, output_path: &str) -> bool {
		let Ok(file) = File::open(archive) else { return false };
		let Ok(mut zip) = ZipArchive::new(BufReader::new(file)) else { return false };
		zip_utils::extract_zip_entry_to_file_with_password(&mut zip, entry, Path::new(output_path), password).is_ok()
	}

	/// `None` when any file's length is unknown, and callers fall back to the character percentage.
	/// Takes elapsed time, not a position, because a position only steps at clip boundaries: once per file for whole-file recordings.
	#[must_use]
	pub fn audio_progress_percent(&self, elapsed_ms: u64) -> Option<i32> {
		let timeline = self.audio()?;
		if !timeline.durations_are_known() {
			return None;
		}
		let progress = timeline.progress(TimelinePoint::new(DocPos::ZERO, elapsed_ms));
		// Truncated so a book is not 100% with half a minute left to play.
		#[allow(clippy::cast_possible_truncation, reason = "truncated on purpose, and a float cast saturates")]
		let percent = (progress * 100.0) as i32;
		Some(percent)
	}

	#[must_use]
	pub fn audio_elapsed_for_percent(&self, percent: i32) -> Option<u64> {
		let timeline = self.audio()?;
		if !timeline.durations_are_known() {
			return None;
		}
		let percent = u64::try_from(percent.clamp(0, 100)).unwrap_or(0);
		Some(timeline.total_duration_ms().saturating_mul(percent) / 100)
	}

	/// How far into the whole recording `elapsed_ms` is, such as "1:02:03 of 10:30:00", or `None` while any file's length is unknown.
	#[must_use]
	pub fn audio_time_announcement(&self, elapsed_ms: u64) -> Option<String> {
		let timeline = self.audio()?;
		if !timeline.durations_are_known() {
			return None;
		}
		let total_ms = timeline.total_duration_ms();
		Some(of_total(elapsed_ms.min(total_ms), total_ms))
	}

	/// The chapter `elapsed_ms` is in, how far into it and how much of it is left, such as "Chapter 3, 4:10 of 12:00, 7:50 left". `None` while any file's length is unknown.
	#[must_use]
	pub fn audio_chapter_announcement(&self, elapsed_ms: u64) -> Option<String> {
		let (title, elapsed_ms, length_ms) = self.audio_chapter_span(elapsed_ms)?;
		let mut parts = Vec::with_capacity(3);
		if !title.is_empty() {
			parts.push(title);
		}
		parts.push(of_total(elapsed_ms, length_ms));
		// TRANSLATORS: Announced with how far into an audiobook's chapter the reader is: the time left in it; {} is written like 12:05
		parts.push(t("{} left").replace("{}", &clock::format_ms(length_ms - elapsed_ms)));
		Some(parts.join(", "))
	}

	/// The title of the chapter `elapsed_ms` is in, the time played of it and how long it runs. Chapters are the table of contents' entries, placed by when they play, and of entries starting together the deepest, as it names the most. Before the first entry, or in a book without any, the whole recording is the chapter, untitled. `None` while any file's length is unknown.
	pub(super) fn audio_chapter_span(&self, elapsed_ms: u64) -> Option<(String, u64, u64)> {
		let timeline = self.audio()?;
		if !timeline.durations_are_known() {
			return None;
		}
		let total_ms = timeline.total_duration_ms();
		let elapsed_ms = elapsed_ms.min(total_ms);
		let toc = self.get_toc();
		let starts: Vec<(u64, i32, &str)> = toc
			.iter()
			.filter_map(|entry| {
				timeline
					.point_for_position(entry.position)
					.map(|point| (point.time_ms, entry.level, entry.title.trim()))
			})
			.collect();
		let current = starts
			.iter()
			.filter(|(start_ms, ..)| *start_ms <= elapsed_ms)
			.max_by_key(|(start_ms, level, _)| (*start_ms, *level));
		let start_ms = current.map_or(0, |(start_ms, ..)| *start_ms);
		let end_ms = starts
			.iter()
			.map(|(start_ms, ..)| *start_ms)
			.filter(|next_ms| *next_ms > start_ms)
			.min()
			.unwrap_or(total_ms);
		let title = current.map_or("", |(_, _, title)| *title).to_owned();
		Some((title, elapsed_ms - start_ms, end_ms - start_ms))
	}
}

fn of_total(elapsed_ms: u64, total_ms: u64) -> String {
	// TRANSLATORS: Announced for how far into an audiobook the reader is: the first {} is the time played so far and the second how long it runs, both written like 1:02:03
	t("{} of {}").replacen("{}", &clock::format_ms(elapsed_ms), 1).replacen("{}", &clock::format_ms(total_ms), 1)
}
