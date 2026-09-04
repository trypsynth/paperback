use std::collections::HashSet;

use anyhow::{Context, Result, bail};
use mp4ameta::{Chapter, ChplTimescale, ReadConfig, Tag};

use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

pub struct M4bParser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedChapter {
	start_ms: u64,
	end_ms: u64,
	title: String,
}

impl Parser for M4bParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let read_config = ReadConfig {
			read_meta_items: true,
			read_image_data: false,
			read_chapter_list: true,
			read_chapter_track: true,
			read_audio_info: true,
			chpl_timescale: ChplTimescale::DEFAULT,
		};
		let tag = Tag::read_with_path(&context.file_path, &read_config)
			.with_context(|| format!("failed to read M4B metadata from {}", context.file_path))?;
		let duration_ms = u64::try_from(tag.duration().as_millis()).context("M4B duration is too large")?;
		if duration_ms == 0 {
			bail!("M4B file has no positive audio duration");
		}
		let (title, author) = document_metadata(&tag, &context.file_path);
		let chapters = normalize_chapters(tag.chapters(), duration_ms, &title);
		Ok(build_document(&context.file_path, title, author, duration_ms, &chapters))
	}
}

fn first_nonempty<'a>(values: impl IntoIterator<Item = Option<&'a str>>) -> Option<&'a str> {
	values.into_iter().flatten().map(str::trim).find(|value| !value.is_empty())
}

fn document_metadata(tag: &Tag, file_path: &str) -> (String, String) {
	let fallback_title = extract_title_from_path(file_path);
	let title = first_nonempty([tag.title(), tag.album()]).unwrap_or(&fallback_title).to_string();
	let author = first_nonempty([tag.album_artist(), tag.artist()]).unwrap_or_default().to_string();
	(title, author)
}

fn normalize_chapters(chapters: &[Chapter], duration_ms: u64, document_title: &str) -> Vec<NormalizedChapter> {
	let mut starts: Vec<(u64, String)> = chapters
		.iter()
		.filter_map(|chapter| {
			let start_ms = u64::try_from(chapter.start.as_millis()).ok()?;
			(start_ms < duration_ms).then(|| (start_ms, chapter.title.trim().to_string()))
		})
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
	use std::time::Duration;

	use mp4ameta::Chapter;

	use super::*;

	fn chapter(start_ms: u64, title: &str) -> Chapter {
		Chapter::new(Duration::from_millis(start_ms), title)
	}

	#[test]
	fn first_nonempty_trims_and_uses_the_first_present_value() {
		assert_eq!(first_nonempty([Some("  "), None, Some(" Book "), Some("Album")]), Some("Book"));
		assert_eq!(first_nonempty([None, Some(" ")]), None);
	}

	#[test]
	fn document_metadata_uses_tag_priority_and_path_fallbacks() {
		let mut tag = Tag::default();
		assert_eq!(document_metadata(&tag, "/books/fallback.m4b"), ("fallback".to_string(), String::new()));
		tag.set_album(" Album ");
		tag.set_artist("Artist");
		assert_eq!(document_metadata(&tag, "fallback.m4b"), ("Album".to_string(), "Artist".to_string()));
		tag.set_title("Title");
		tag.set_album_artist("Album Artist");
		assert_eq!(document_metadata(&tag, "fallback.m4b"), ("Title".to_string(), "Album Artist".to_string()));
	}

	#[test]
	fn normalizes_chapters_into_sorted_contiguous_clips() {
		let chapters = vec![
			chapter(5000, "Second"),
			chapter(1000, " First "),
			chapter(5000, "Duplicate"),
			chapter(9000, ""),
			chapter(12_000, "Past end"),
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
	fn accepts_chapter_tracks_and_prefers_chapter_lists_when_both_exist() {
		let mut tag = Tag::default();
		tag.chapter_track_mut().extend([chapter(0, "Track One"), chapter(5000, "Track Two")]);
		assert_eq!(normalize_chapters(tag.chapters(), 10_000, "Book")[0].title, "Track One");
		tag.chapter_list_mut().extend([chapter(0, "List One"), chapter(4000, "List Two")]);
		let chapters = normalize_chapters(tag.chapters(), 10_000, "Book");
		assert_eq!(chapters[0].title, "List One");
		assert_eq!(chapters[0].end_ms, 4000);
	}

	#[test]
	fn chapterless_book_becomes_one_full_length_section() {
		assert_eq!(
			normalize_chapters(&[], 10_000, "Book"),
			vec![NormalizedChapter { start_ms: 0, end_ms: 10_000, title: "Book".to_string() }]
		);
	}

	#[test]
	fn builds_audio_only_document_with_navigation_anchors() {
		let chapters = vec![
			NormalizedChapter { start_ms: 0, end_ms: 4000, title: "One".to_string() },
			NormalizedChapter { start_ms: 4000, end_ms: 10_000, title: "Two".to_string() },
		];
		let document = build_document("book.m4b", "Book".to_string(), "Author".to_string(), 10_000, &chapters);
		let audio = document.audio.as_ref().expect("audio timeline");
		assert_eq!(document.title, "Book");
		assert_eq!(document.author, "Author");
		assert_eq!(document.buffer.content, "\n\n");
		assert_eq!(document.toc_items.len(), 2);
		assert_eq!(document.toc_items[0].name, "One");
		assert_eq!(document.toc_items[1].offset, 1);
		assert!(document.audio_only);
		assert_eq!(audio.sources().len(), 1);
		assert_eq!(audio.clips().len(), 2);
		assert_eq!(audio.clips()[0].clip_end_ms, 4000);
		assert_eq!(audio.clips()[1].clip_begin_ms, 4000);
		assert_eq!(audio.total_duration_ms(), 10_000);
		assert_eq!(audio.source(0).and_then(|source| source.duration_ms), Some(10_000));
		assert_eq!(audio.source(0).map(|source| &source.location), Some(&AudioLocation::File("book.m4b".to_string())));
	}
}
