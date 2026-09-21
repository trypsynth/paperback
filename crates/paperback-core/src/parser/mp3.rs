//! A loose MP3 file opened directly, as opposed to an M4B (which carries its own chapter list
//! in a different way, see `parser::m4b`) or a DAISY "plain audio" zip bundle of several files
//! (see `parser::daisy::plain_audio`).
//!
//! MP3 has a standard, widely-supported way to embed a chapter list: ID3v2's `CHAP`/`CTOC`
//! frames, the same mechanism podcast apps use. A chapterless file becomes one full-length
//! section, same as a chapterless M4B.

use std::{fs::File, io::BufReader};

use anyhow::{Context, Result, bail};
use lofty::{config::ParseOptions, id3::v2::Frame, mpeg::MpegFile, prelude::*};

use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		util::{
			chapters::{NormalizedChapter, RawChapter, normalize_chapters},
			path::extract_title_from_path,
		},
	},
};

pub struct Mp3Parser;

impl Parser for Mp3Parser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let tagged_file = lofty::read_from_path(&context.file_path)
			.with_context(|| format!("failed to read audio metadata from {}", context.file_path))?;
		let duration_ms =
			u64::try_from(tagged_file.properties().duration().as_millis()).context("audio duration is too large")?;
		if duration_ms == 0 {
			bail!("audio file has no positive duration");
		}
		let (title, author) = document_metadata(&tagged_file, &context.file_path);
		let chapters = normalize_chapters(&read_id3v2_chapters(&context.file_path), duration_ms, &title);
		Ok(build_document(&context.file_path, title, author, duration_ms, &chapters))
	}
}

fn document_metadata(tagged_file: &lofty::file::TaggedFile, file_path: &str) -> (String, String) {
	let fallback_title = extract_title_from_path(file_path);
	let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());
	let title = tag
		.and_then(Accessor::title)
		.map(|title| title.trim().to_string())
		.filter(|title| !title.is_empty())
		.unwrap_or(fallback_title);
	let author = tag.and_then(Accessor::artist).map(|artist| artist.trim().to_string()).unwrap_or_default();
	(title, author)
}

/// ID3v2 `CHAP` frames from the file's tag, if it has any. Reading them needs the format-specific
/// `MpegFile` (via its own `AudioFile::read_from`, a second, separate read of the file), since
/// the generic `TaggedFile`/`Tag` API `document_metadata` uses normalizes tags across formats
/// and drops frames, like `CHAP`, that have no cross-format equivalent. Empty (rather than an
/// error) whenever the file can't be opened, isn't valid MPEG audio after all, or carries no
/// ID3v2 tag at all — `normalize_chapters` treats an empty list as "no chapters" regardless of
/// why, falling back to one full-length section, which is the right response to all three.
///
/// Chapters are read in whatever order the tag stores them in; `normalize_chapters` sorts them
/// by start time regardless. A `CTOC` frame's own ordering is not consulted: for the single flat
/// chapter list every real-world MP3 audiobook or podcast actually has, start time sorts them
/// identically, and consulting `CTOC` besides would only matter for a nested chapter tree, which
/// no known producer writes and this reader has no representation for anyway.
fn read_id3v2_chapters(file_path: &str) -> Vec<RawChapter> {
	let Ok(file) = File::open(file_path) else { return Vec::new() };
	let mut reader = BufReader::new(file);
	let Ok(mpeg_file) = MpegFile::read_from(&mut reader, ParseOptions::new()) else { return Vec::new() };
	let Some(id3v2) = mpeg_file.id3v2() else { return Vec::new() };
	id3v2
		.iter()
		.filter_map(|frame| match frame {
			Frame::Chapter(chapter) => Some(chapter),
			_ => None,
		})
		.map(|chapter| RawChapter { start_ms: u64::from(chapter.times.start), title: chapter_title(chapter) })
		.collect()
}

/// A chapter frame's human-readable name, from its embedded `TIT2` child frame. Blank when
/// absent, which `normalize_chapters` turns into a "Chapter N" fallback.
fn chapter_title(chapter: &lofty::id3::v2::ChapterFrame<'_>) -> String {
	chapter
		.children
		.iter()
		.find_map(|frame| match frame {
			Frame::Text(text) if frame.id().as_str() == "TIT2" => Some(text.value.trim().to_string()),
			_ => None,
		})
		.unwrap_or_default()
}

fn build_document(
	file_path: &str,
	title: String,
	author: String,
	duration_ms: u64,
	chapters: &[NormalizedChapter],
) -> Document {
	let mut buffer = DocumentBuffer::new();
	let mut toc_items = Vec::with_capacity(chapters.len());
	let mut audio_builder = AudioTimelineBuilder::new();
	let source = audio_builder.add_source(AudioLocation::File(file_path.to_string()), Some(duration_ms));
	for chapter in chapters {
		let position = buffer.current_position();
		buffer.append("\n");
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, position).with_text(chapter.title.clone()));
		audio_builder.add_clip(source, chapter.start_ms, chapter.end_ms, position, position + 1);
		toc_items.push(TocItem::new(chapter.title.clone(), chapter.start_ms.to_string(), position));
	}

	Document {
		title,
		author,
		buffer,
		toc_items,
		audio: Some(audio_builder.build()),
		audio_only: true,
		..Document::default()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builds_a_single_full_length_section_when_chapterless() {
		let chapters = normalize_chapters(&[], 10_000, "Book");
		let document = build_document("book.mp3", "Book".to_string(), "Author".to_string(), 10_000, &chapters);
		let audio = document.audio.as_ref().expect("audio timeline");
		assert_eq!(document.title, "Book");
		assert_eq!(document.author, "Author");
		assert_eq!(document.buffer.content, "\n");
		assert_eq!(document.toc_items.len(), 1);
		assert_eq!(document.toc_items[0].name, "Book");
		assert!(document.audio_only);
		assert_eq!(audio.sources().len(), 1);
		assert_eq!(audio.clips().len(), 1);
		assert_eq!(audio.clips()[0].clip_end_ms, 10_000);
		assert_eq!(audio.total_duration_ms(), 10_000);
		assert_eq!(audio.source(0).map(|source| &source.location), Some(&AudioLocation::File("book.mp3".to_string())));
	}

	#[test]
	fn builds_multiple_sections_from_chapters() {
		let raw = vec![
			RawChapter { start_ms: 0, title: "One".to_string() },
			RawChapter { start_ms: 4000, title: "Two".to_string() },
		];
		let chapters = normalize_chapters(&raw, 10_000, "Book");
		let document = build_document("book.mp3", "Book".to_string(), "Author".to_string(), 10_000, &chapters);
		let audio = document.audio.as_ref().expect("audio timeline");
		assert_eq!(document.toc_items.len(), 2);
		assert_eq!(document.toc_items[0].name, "One");
		assert_eq!(document.toc_items[1].offset, 1);
		assert_eq!(audio.sources().len(), 1);
		assert_eq!(audio.clips().len(), 2);
		assert_eq!(audio.clips()[0].clip_end_ms, 4000);
		assert_eq!(audio.clips()[1].clip_begin_ms, 4000);
		assert_eq!(audio.total_duration_ms(), 10_000);
	}

	/// Reads a chapter's title back from a real, self-contained `ChapterFrame` built in memory
	/// (as `read_id3v2_chapters` would find embedded in an MP3's ID3v2 tag), rather than from a
	/// file on disk, so this doesn't depend on a real chaptered MP3 fixture existing in the repo.
	#[test]
	fn chapter_title_reads_the_embedded_tit2_child_frame() {
		use lofty::{
			TextEncoding,
			id3::v2::{ChapterFrame, FrameId, FrameList, TextInformationFrame},
		};

		let title_frame = TextInformationFrame::new(FrameId::new("TIT2").unwrap(), TextEncoding::UTF8, "Introduction");
		let mut children = FrameList::new();
		children.insert(Frame::Text(title_frame));
		let titled = ChapterFrame::new("chp0", 0..5000, u32::MAX..u32::MAX, children);
		assert_eq!(chapter_title(&titled), "Introduction");

		let untitled = ChapterFrame::new("chp1", 5000..10_000, u32::MAX..u32::MAX, FrameList::new());
		assert_eq!(chapter_title(&untitled), "");
	}
}
