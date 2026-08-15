//! Maps recorded audio onto a document's text spine.
//!
//! An `AudioTimeline` is an overlay: it stores no text, duplicates no markers,
//! and is rebuilt from the source document on every open, exactly like markers
//! are. Nothing here is serialized — only `TimelinePoint` reaches disk.
//!
//! A point in an audio document has two coordinates, and each is authoritative
//! for its own axis: `position` locates the text caret, `time_ms` locates the
//! audio. Playback always resolves from `time_ms`; text navigation always uses
//! `position`. Converting between them is what this module is for.
//!
//! The granularity of `position` follows the source material, so the same model
//! covers all three cases. DAISY narration binds a clip per sentence, making
//! `position` precise and `time_ms` a refinement within it. An audiobook binds
//! one clip per chapter, making `position` a chapter anchor and leaving
//! `time_ms` to carry everything else. Text-only documents have no timeline at
//! all.

/// Where a source's bytes live. Paths are final, not relative to the document:
/// the timeline is rebuilt on every open, so there is nothing to keep portable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioLocation {
	File(String),
	ZipEntry { archive: String, entry: String },
}

/// One audio file, referenced by index from the clips that play parts of it.
/// Interning matters to players: consecutive clips sharing a source are a seek
/// on an open decoder, a change of source is an open.
#[derive(Debug, Clone)]
pub struct AudioSource {
	pub location: AudioLocation,
	/// Full length of the file, when the parser knows it. Clip bounds do not
	/// depend on this.
	pub duration_ms: Option<u64>,
}

/// A span of audio bound to a span of text: source time `[clip_begin_ms,
/// clip_end_ms)` narrates display units `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioClip {
	pub source: usize,
	pub clip_begin_ms: u64,
	pub clip_end_ms: u64,
	pub start: usize,
	pub end: usize,
}

impl AudioClip {
	#[must_use]
	pub const fn duration_ms(&self) -> u64 {
		self.clip_end_ms.saturating_sub(self.clip_begin_ms)
	}
}

/// A resumable point in an audio document. `time_ms` is elapsed time from the
/// start of the document, not an offset into any one source, so it stays
/// meaningful when several clips share a text position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimelinePoint {
	pub position: usize,
	pub time_ms: u64,
}

impl TimelinePoint {
	#[must_use]
	pub const fn new(position: usize, time_ms: u64) -> Self {
		Self { position, time_ms }
	}
}

/// What a player needs to start at some point: which clip, and where to seek
/// within that clip's source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackCursor {
	pub clip: usize,
	pub seek_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct AudioTimeline {
	sources: Vec<AudioSource>,
	clips: Vec<AudioClip>,
	/// Elapsed time at the start of each clip, parallel to `clips`.
	elapsed_ms: Vec<u64>,
	total_duration_ms: u64,
}

impl AudioTimeline {
	#[must_use]
	pub fn sources(&self) -> &[AudioSource] {
		&self.sources
	}

	#[must_use]
	pub fn source(&self, index: usize) -> Option<&AudioSource> {
		self.sources.get(index)
	}

	#[must_use]
	pub fn clips(&self) -> &[AudioClip] {
		&self.clips
	}

	#[must_use]
	pub fn clip(&self, index: usize) -> Option<&AudioClip> {
		self.clips.get(index)
	}

	#[must_use]
	pub const fn total_duration_ms(&self) -> u64 {
		self.total_duration_ms
	}

	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.clips.is_empty()
	}

	/// Elapsed time at the start of the clip at `index`.
	#[must_use]
	pub fn clip_start_ms(&self, index: usize) -> Option<u64> {
		self.elapsed_ms.get(index).copied()
	}

	/// The clip playing at `elapsed_ms`, with the offset into its source to seek
	/// to. Times past the end of the document resolve to the end of the last
	/// clip rather than to nothing.
	#[must_use]
	pub fn cursor_at_elapsed(&self, elapsed_ms: u64) -> Option<PlaybackCursor> {
		if self.clips.is_empty() {
			return None;
		}
		let index = self.elapsed_ms.partition_point(|start| *start <= elapsed_ms).saturating_sub(1);
		let clip = &self.clips[index];
		let into_clip = (elapsed_ms - self.elapsed_ms[index]).min(clip.duration_ms());
		Some(PlaybackCursor { clip: index, seek_ms: clip.clip_begin_ms + into_clip })
	}

	/// Where playback should resume for `point`. Resolves from `time_ms` alone;
	/// `position` is the text caret and says nothing about the audio.
	#[must_use]
	pub fn resolve(&self, point: TimelinePoint) -> Option<PlaybackCursor> {
		self.cursor_at_elapsed(point.time_ms)
	}

	/// The clip narrating `position`, or `None` where nothing does — gaps are
	/// ordinary, since front matter and unrecorded sections are common. Where
	/// several clips share a text anchor, this is the first of them.
	#[must_use]
	pub fn clip_index_at_position(&self, position: usize) -> Option<usize> {
		let searched = self.clips.partition_point(|clip| clip.start <= position);
		let mut found: Option<usize> = None;
		// Walking back, starts are non-increasing, so the first containing clip
		// has the greatest start. Keep walking through that start's group to
		// reach its earliest clip.
		for index in (0..searched).rev() {
			let clip = &self.clips[index];
			if found.is_some_and(|best| self.clips[best].start != clip.start) {
				break;
			}
			if position < clip.end {
				found = Some(index);
			}
		}
		found
	}

	/// The point to jump audio to when the reader moves the caret to `position`.
	#[must_use]
	pub fn point_for_position(&self, position: usize) -> Option<TimelinePoint> {
		let index = self.clip_index_at_position(position)?;
		Some(TimelinePoint::new(self.clips[index].start, self.elapsed_ms[index]))
	}

	/// The document-wide elapsed time corresponding to `raw_ms` within `source`'s own audio
	/// file — the reverse of `resolve`, used to translate a decoder's reported position back
	/// into document coordinates. Falls back to the nearest preceding clip's end when
	/// `raw_ms` lands in an unrecorded gap.
	#[must_use]
	pub fn elapsed_for_source_position(&self, source: usize, raw_ms: u64) -> Option<u64> {
		let mut nearest_preceding: Option<u64> = None;
		for (index, clip) in self.clips.iter().enumerate() {
			if clip.source != source || clip.clip_begin_ms > raw_ms {
				continue;
			}
			if raw_ms < clip.clip_end_ms {
				return Some(self.elapsed_ms[index] + (raw_ms - clip.clip_begin_ms));
			}
			let end_elapsed = self.elapsed_ms[index] + clip.duration_ms();
			nearest_preceding = Some(nearest_preceding.map_or(end_elapsed, |best| best.max(end_elapsed)));
		}
		nearest_preceding
	}

	/// How far through the recording `point` is, in `0.0..=1.0`. Audio documents
	/// should prefer this to a character-count percentage, which measures only
	/// the text spine — a few chapter headings, for an audiobook.
	#[must_use]
	pub fn progress(&self, point: TimelinePoint) -> f64 {
		if self.total_duration_ms == 0 {
			return 0.0;
		}
		(point.time_ms.min(self.total_duration_ms) as f64) / (self.total_duration_ms as f64)
	}

	/// Finds whichever source's narration picks up where `current`'s leaves off, for
	/// auto-advancing playback across a chapter boundary (each DAISY chapter is typically
	/// its own audio source file).
	#[must_use]
	pub fn next_source_after(&self, current: usize) -> Option<usize> {
		let last_end_ms = self
			.clips
			.iter()
			.enumerate()
			.filter(|(_, clip)| clip.source == current)
			.filter_map(|(index, clip)| Some(self.clip_start_ms(index)? + clip.duration_ms()))
			.max()?;
		self.clips
			.iter()
			.enumerate()
			.filter(|(index, clip)| {
				clip.source != current && self.clip_start_ms(*index).is_some_and(|start| start >= last_end_ms)
			})
			.min_by_key(|(index, _)| self.clip_start_ms(*index).unwrap_or(u64::MAX))
			.map(|(_, clip)| clip.source)
	}
}

#[derive(Debug, Clone, Default)]
pub struct AudioTimelineBuilder {
	sources: Vec<AudioSource>,
	clips: Vec<AudioClip>,
}

impl AudioTimelineBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self { sources: Vec::new(), clips: Vec::new() }
	}

	/// Registers a source and returns the index to pass to `add_clip`.
	pub fn add_source(&mut self, location: AudioLocation, duration_ms: Option<u64>) -> usize {
		self.sources.push(AudioSource { location, duration_ms });
		self.sources.len() - 1
	}

	/// Binds `[clip_begin_ms, clip_end_ms)` of `source` to display units
	/// `[start, end)`. Positions must be display units, from the same counting
	/// as marker positions — see `util::text::display_len`, which is UTF-16 code
	/// units on Windows and macOS.
	pub fn add_clip(&mut self, source: usize, clip_begin_ms: u64, clip_end_ms: u64, start: usize, end: usize) {
		self.clips.push(AudioClip { source, clip_begin_ms, clip_end_ms, start, end });
	}

	/// Sorts the clips and indexes them. Clips of no duration, and clips naming
	/// a source that was never registered, are dropped rather than rejected;
	/// authored SMIL is routinely a little ragged and one bad `<par>` should not
	/// cost the reader the whole book.
	#[must_use]
	pub fn build(self) -> AudioTimeline {
		let source_count = self.sources.len();
		let mut clips: Vec<AudioClip> = self
			.clips
			.into_iter()
			.filter(|clip| clip.source < source_count && clip.clip_end_ms > clip.clip_begin_ms)
			.collect();
		clips.sort_by_key(|clip| (clip.start, clip.clip_begin_ms));
		let mut elapsed_ms = Vec::with_capacity(clips.len());
		let mut total_duration_ms = 0;
		for clip in &clips {
			elapsed_ms.push(total_duration_ms);
			total_duration_ms += clip.duration_ms();
		}
		AudioTimeline { sources: self.sources, clips, elapsed_ms, total_duration_ms }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Narration bound per sentence, the DAISY shape: one source, clips in step
	/// with the text.
	fn narrated_timeline() -> AudioTimeline {
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File("chapter1.mp3".to_string()), Some(9000));
		builder.add_clip(source, 0, 2000, 0, 40);
		builder.add_clip(source, 2000, 5000, 40, 95);
		builder.add_clip(source, 5000, 9000, 95, 160);
		builder.build()
	}

	/// An audiobook chapter with no text of its own beyond its heading, split
	/// across two files: two clips sharing one anchor.
	fn audiobook_timeline() -> AudioTimeline {
		let mut builder = AudioTimelineBuilder::new();
		let part1 = builder.add_source(AudioLocation::File("part1.mp3".to_string()), Some(600_000));
		let part2 = builder.add_source(AudioLocation::File("part2.mp3".to_string()), Some(300_000));
		builder.add_clip(part1, 0, 600_000, 12, 30);
		builder.add_clip(part2, 0, 300_000, 12, 30);
		builder.build()
	}

	#[test]
	fn build_sorts_clips_and_accumulates_elapsed_time() {
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File("a.mp3".to_string()), None);
		builder.add_clip(source, 2000, 5000, 40, 95);
		builder.add_clip(source, 0, 2000, 0, 40);
		let timeline = builder.build();
		assert_eq!(timeline.clips().iter().map(|c| c.start).collect::<Vec<_>>(), vec![0, 40]);
		assert_eq!(timeline.clip_start_ms(0), Some(0));
		assert_eq!(timeline.clip_start_ms(1), Some(2000));
		assert_eq!(timeline.total_duration_ms(), 5000);
	}

	#[test]
	fn build_drops_degenerate_and_dangling_clips() {
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File("a.mp3".to_string()), None);
		builder.add_clip(source, 0, 1000, 0, 10);
		builder.add_clip(source, 500, 500, 10, 20); // no duration
		builder.add_clip(source + 1, 0, 1000, 20, 30); // no such source
		let timeline = builder.build();
		assert_eq!(timeline.clips().len(), 1);
		assert_eq!(timeline.total_duration_ms(), 1000);
	}

	#[test]
	fn cursor_at_elapsed_seeks_within_the_containing_clip() {
		let timeline = narrated_timeline();
		let cursor = timeline.cursor_at_elapsed(3500).unwrap();
		assert_eq!(cursor, PlaybackCursor { clip: 1, seek_ms: 3500 });
	}

	#[test]
	fn cursor_at_elapsed_offsets_from_the_clip_begin_not_the_file_start() {
		let timeline = audiobook_timeline();
		// 700s in is 100s into the second file, which begins at its own zero.
		let cursor = timeline.cursor_at_elapsed(700_000).unwrap();
		assert_eq!(cursor, PlaybackCursor { clip: 1, seek_ms: 100_000 });
	}

	#[test]
	fn cursor_at_elapsed_clamps_past_the_end() {
		let timeline = narrated_timeline();
		let cursor = timeline.cursor_at_elapsed(u64::MAX).unwrap();
		assert_eq!(cursor, PlaybackCursor { clip: 2, seek_ms: 9000 });
	}

	#[test]
	fn cursor_at_elapsed_is_none_when_there_is_no_audio() {
		assert!(AudioTimeline::default().cursor_at_elapsed(0).is_none());
	}

	#[test]
	fn clip_index_at_position_is_start_inclusive_end_exclusive() {
		let timeline = narrated_timeline();
		assert_eq!(timeline.clip_index_at_position(40), Some(1));
		assert_eq!(timeline.clip_index_at_position(94), Some(1));
		assert_eq!(timeline.clip_index_at_position(95), Some(2));
		assert_eq!(timeline.clip_index_at_position(160), None);
	}

	#[test]
	fn clip_index_at_position_returns_the_first_clip_sharing_an_anchor() {
		let timeline = audiobook_timeline();
		assert_eq!(timeline.clip_index_at_position(20), Some(0));
	}

	#[test]
	fn clip_index_at_position_is_none_in_a_gap() {
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File("a.mp3".to_string()), None);
		builder.add_clip(source, 0, 1000, 0, 10);
		builder.add_clip(source, 1000, 2000, 50, 60);
		let timeline = builder.build();
		assert!(timeline.clip_index_at_position(30).is_none());
	}

	#[test]
	fn point_for_position_anchors_to_the_start_of_its_clip() {
		let timeline = narrated_timeline();
		assert_eq!(timeline.point_for_position(60), Some(TimelinePoint::new(40, 2000)));
	}

	#[test]
	fn a_point_round_trips_from_text_to_audio_and_back() {
		let timeline = narrated_timeline();
		let point = timeline.point_for_position(100).unwrap();
		let cursor = timeline.resolve(point).unwrap();
		assert_eq!(timeline.clip(cursor.clip).unwrap().start, point.position);
	}

	#[test]
	fn elapsed_for_source_position_maps_raw_file_position_back_to_document_time() {
		let timeline = narrated_timeline();
		// The source's single file holds all three clips back to back.
		assert_eq!(timeline.elapsed_for_source_position(0, 0), Some(0));
		assert_eq!(timeline.elapsed_for_source_position(0, 3500), Some(3500));
		assert_eq!(timeline.elapsed_for_source_position(0, 7000), Some(7000));
	}

	#[test]
	fn elapsed_for_source_position_falls_back_to_nearest_preceding_clip_in_a_gap() {
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File("a.mp3".to_string()), None);
		builder.add_clip(source, 0, 1000, 0, 10);
		builder.add_clip(source, 2000, 3000, 10, 20);
		let timeline = builder.build();
		// 1500ms falls between the two clips' raw ranges; nearest preceding clip ended at 1000ms.
		assert_eq!(timeline.elapsed_for_source_position(0, 1500), Some(1000));
	}

	#[test]
	fn elapsed_for_source_position_is_none_for_an_unknown_source() {
		let timeline = narrated_timeline();
		assert_eq!(timeline.elapsed_for_source_position(9, 0), None);
	}

	#[test]
	fn elapsed_for_source_position_distinguishes_sources_sharing_a_text_anchor() {
		let timeline = audiobook_timeline();
		assert_eq!(timeline.elapsed_for_source_position(1, 50_000), Some(650_000));
	}

	#[test]
	fn next_source_after_finds_the_source_that_continues_the_narration() {
		let timeline = audiobook_timeline();
		// part1 (source 0) ends where part2 (source 1) begins.
		assert_eq!(timeline.next_source_after(0), Some(1));
	}

	#[test]
	fn next_source_after_is_none_for_the_last_source() {
		let timeline = audiobook_timeline();
		assert_eq!(timeline.next_source_after(1), None);
	}

	#[test]
	fn next_source_after_is_none_for_an_unknown_source() {
		let timeline = narrated_timeline();
		assert_eq!(timeline.next_source_after(9), None);
	}

	#[test]
	fn progress_measures_time_not_text() {
		let timeline = audiobook_timeline();
		// Half the recording, but the text spine is one heading throughout.
		assert!((timeline.progress(TimelinePoint::new(12, 450_000)) - 0.5).abs() < f64::EPSILON);
		assert!((timeline.progress(TimelinePoint::new(12, u64::MAX)) - 1.0).abs() < f64::EPSILON);
		assert!((AudioTimeline::default().progress(TimelinePoint::default())).abs() < f64::EPSILON);
	}
}
