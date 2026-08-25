//! FFI-facing queries over a document's recorded narration ([`super::DocumentSession::audio`]):
//! source lookup/extraction, clip lookup, and the elapsed-time/position conversions a platform
//! player needs to keep the caret and playback in sync.

use std::{
	fs::{self, File},
	io::BufReader,
	path::Path,
};

use zip::ZipArchive;

use super::{AudioClipFfi, AudioCursorFfi, AudioPointFfi, DocumentSession};
use crate::{audio::AudioLocation, util::zip as zip_utils};

impl DocumentSession {
	#[must_use]
	pub fn has_audio_ffi(&self) -> bool {
		self.audio().is_some_and(|timeline| !timeline.is_empty())
	}

	/// True when this document's text spine is only there to anchor audio (see
	/// `Document::audio_only`), so a read-aloud UI should navigate it by elapsed time rather
	/// than by paragraph, section, or any other text unit.
	#[must_use]
	pub fn is_audio_only_ffi(&self) -> bool {
		self.handle().document().audio_only && self.has_audio_ffi()
	}

	#[must_use]
	pub fn audio_source_count_ffi(&self) -> i32 {
		self.audio().map_or(0, |timeline| i32::try_from(timeline.sources().len()).unwrap_or(0))
	}

	/// Empty for a zip-embedded source, an out-of-range index, or a document with no audio;
	/// otherwise callers can play the path directly without `audio_extract_source_ffi`.
	#[must_use]
	pub fn audio_source_direct_path_ffi(&self, index: i32) -> String {
		let Some(timeline) = self.audio() else { return String::new() };
		let Ok(index) = usize::try_from(index) else { return String::new() };
		match timeline.source(index).map(|source| &source.location) {
			Some(AudioLocation::File(path)) => path.clone(),
			_ => String::new(),
		}
	}

	/// Writes source `index`'s audio bytes to `output_path`: extracted from its archive when
	/// zip-embedded, copied otherwise, for platforms whose player needs a real local path.
	#[must_use]
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

	fn extract_zip_audio_entry(archive: &str, entry: &str, password: Option<&str>, output_path: &str) -> bool {
		let Ok(file) = File::open(archive) else { return false };
		let Ok(mut zip) = ZipArchive::new(BufReader::new(file)) else { return false };
		zip_utils::extract_zip_entry_to_file_with_password(&mut zip, entry, Path::new(output_path), password).is_ok()
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
			start: i64::try_from(clip.start).unwrap_or(0),
			end: i64::try_from(clip.end).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn audio_total_duration_ms_ffi(&self) -> i64 {
		self.audio().map_or(0, |timeline| i64::try_from(timeline.total_duration_ms()).unwrap_or(0))
	}

	/// See `AudioTimeline::cursor_at_elapsed`.
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

	/// See `AudioTimeline::point_for_position`.
	#[must_use]
	pub fn audio_point_for_position_ffi(&self, position: i64) -> AudioPointFfi {
		let Some(timeline) = self.audio() else { return AudioPointFfi::default() };
		let position = usize::try_from(position.max(0)).unwrap_or(0);
		let Some(point) = timeline.point_for_position(position) else { return AudioPointFfi::default() };
		AudioPointFfi {
			found: true,
			position: i64::try_from(point.position).unwrap_or(0),
			time_ms: i64::try_from(point.time_ms).unwrap_or(0),
		}
	}

	/// `-1` when the source is unknown or the position falls before any recorded clip. See
	/// `AudioTimeline::elapsed_for_source_position`.
	#[must_use]
	pub fn audio_elapsed_for_source_position_ffi(&self, source: i32, raw_ms: i64) -> i64 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(source) = usize::try_from(source) else { return -1 };
		let Ok(raw_ms) = u64::try_from(raw_ms) else { return -1 };
		timeline.elapsed_for_source_position(source, raw_ms).and_then(|ms| i64::try_from(ms).ok()).unwrap_or(-1)
	}

	/// `-1` at the end of the book. See `AudioTimeline::next_source_after`.
	#[must_use]
	pub fn audio_next_source_after_ffi(&self, current_source: i32) -> i32 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(current_source) = usize::try_from(current_source) else { return -1 };
		timeline.next_source_after(current_source).and_then(|source| i32::try_from(source).ok()).unwrap_or(-1)
	}

	/// `-1` at the start of the book. See `AudioTimeline::previous_source_before`.
	#[must_use]
	pub fn audio_previous_source_before_ffi(&self, current_source: i32) -> i32 {
		let Some(timeline) = self.audio() else { return -1 };
		let Ok(current_source) = usize::try_from(current_source) else { return -1 };
		timeline.previous_source_before(current_source).and_then(|source| i32::try_from(source).ok()).unwrap_or(-1)
	}
}
