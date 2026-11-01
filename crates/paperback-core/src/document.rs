use std::{collections::HashMap, sync::OnceLock};

pub use paperback_formats::ParserFlags;

use crate::audio::AudioTimeline;

mod buffer;
mod handle;
mod marker;
mod position;
mod stats;
mod toc;

pub use buffer::{DocumentBuffer, Edit, PartSpan, ReplaceOutcome};
pub use handle::DocumentHandle;
pub(crate) use marker::format_marker_types;
pub use marker::{ContainerSpan, Marker, MarkerType, is_container_marker, is_heading_marker};
pub use position::{ContentBytePos, DocLen, DocPos};
pub use stats::DocumentStats;
pub use toc::TocItem;

#[derive(Debug, Clone)]
pub struct Document {
	pub title: String,
	pub author: String,
	pub buffer: DocumentBuffer,
	pub toc_items: Vec<TocItem>,
	pub id_positions: HashMap<String, DocPos>,
	pub spine_items: Vec<String>,
	pub manifest_items: HashMap<String, String>,
	pub(crate) stats: OnceLock<DocumentStats>,
	pub audio: Option<AudioTimeline>,
	/// The text only anchors audio (a zip of bare narration files), so read-aloud UIs navigate by time, as text units would have nothing to land on.
	pub audio_only: bool,
}

impl Document {
	#[must_use]
	pub fn new() -> Self {
		Self {
			title: String::new(),
			author: String::new(),
			buffer: DocumentBuffer::new(),
			toc_items: Vec::new(),
			id_positions: HashMap::new(),
			spine_items: Vec::new(),
			manifest_items: HashMap::new(),
			stats: OnceLock::new(),
			audio: None,
			audio_only: false,
		}
	}

	#[must_use]
	pub fn with_title(mut self, title: String) -> Self {
		self.title = title;
		self
	}

	#[must_use]
	pub fn with_author(mut self, author: String) -> Self {
		self.author = author;
		self
	}

	pub fn set_buffer(&mut self, buffer: DocumentBuffer) {
		self.buffer = buffer;
		self.stats.take();
	}

	pub fn set_audio(&mut self, audio: AudioTimeline) {
		self.audio = Some(audio);
		self.stats.take();
	}

	/// Counted on first use rather than at open, since it is a pass over the whole text that only the word count and document info need.
	pub fn stats(&self) -> &DocumentStats {
		self.stats.get_or_init(|| {
			let mut stats = DocumentStats::from_text(&self.buffer.content);
			if let Some(audio) = &self.audio {
				stats.audio_file_count = audio.sources().len();
				stats.audio_total_duration_ms = audio.durations_are_known().then(|| audio.total_duration_ms());
			}
			stats
		})
	}
}

impl Default for Document {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ParseSettings {
	pub render_tables_inline: bool,
	pub join_pdf_paragraphs: bool,
	pub strip_running_text: bool,
	pub markdown_dollar_math: bool,
}

impl Default for ParseSettings {
	fn default() -> Self {
		Self {
			render_tables_inline: true,
			join_pdf_paragraphs: true,
			strip_running_text: true,
			markdown_dollar_math: true,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ParserContext {
	pub file_path: String,
	pub password: Option<String>,
	pub forced_extension: Option<String>,
	pub render_tables_inline: bool,
	/// Off is for documents whose line breaks are the content, such as code listings and poetry.
	pub join_pdf_paragraphs: bool,
	/// A judgement, not a fact: a line repeating on four or more pages can be the tail of a sentence that falls at a page break.
	pub strip_running_text: bool,
	/// Off is for dollars that are not math, such as `$HOME/bin:$PATH` in a README, since Markdown can't say which the writer meant.
	pub markdown_dollar_math: bool,
}

impl ParserContext {
	#[must_use]
	pub const fn new(file_path: String) -> Self {
		Self {
			file_path,
			password: None,
			forced_extension: None,
			render_tables_inline: true,
			join_pdf_paragraphs: true,
			strip_running_text: true,
			markdown_dollar_math: true,
		}
	}

	#[must_use]
	pub fn with_password(mut self, password: String) -> Self {
		self.password = Some(password);
		self
	}

	#[must_use]
	pub const fn with_strip_running_text(mut self, strip: bool) -> Self {
		self.strip_running_text = strip;
		self
	}

	#[must_use]
	pub fn with_forced_extension(mut self, extension: String) -> Self {
		self.forced_extension = Some(extension);
		self
	}

	#[must_use]
	pub const fn with_render_tables_inline(mut self, value: bool) -> Self {
		self.render_tables_inline = value;
		self
	}

	#[must_use]
	pub const fn with_join_pdf_paragraphs(mut self, value: bool) -> Self {
		self.join_pdf_paragraphs = value;
		self
	}

	#[must_use]
	pub const fn with_parse_settings(mut self, settings: ParseSettings) -> Self {
		self.render_tables_inline = settings.render_tables_inline;
		self.join_pdf_paragraphs = settings.join_pdf_paragraphs;
		self.strip_running_text = settings.strip_running_text;
		self.markdown_dollar_math = settings.markdown_dollar_math;
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn document_stats_uses_buffer_content() {
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("one two".to_string()));
		assert_eq!(doc.stats().word_count, 2);
		assert_eq!(doc.stats().line_count, 1);
		assert_eq!(doc.stats().audio_file_count, 0);
		assert_eq!(doc.stats().audio_total_duration_ms, None);
	}

	#[test]
	fn document_stats_folds_in_audio_file_count_and_duration() {
		let mut builder = crate::audio::AudioTimelineBuilder::new();
		let source0 = builder.add_source(crate::audio::AudioLocation::File("a.mp3".to_string()), Some(2000));
		let source1 = builder.add_source(crate::audio::AudioLocation::File("b.mp3".to_string()), Some(3000));
		builder.add_clip(source0, 0, 2000, 0.into(), 1.into());
		builder.add_clip(source1, 0, 3000, 1.into(), 2.into());
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("a b".to_string()));
		doc.set_audio(builder.build());
		assert_eq!(doc.stats().audio_file_count, 2);
		assert_eq!(doc.stats().audio_total_duration_ms, Some(5000));
	}

	#[test]
	fn document_stats_has_no_audio_duration_when_a_file_length_is_unknown() {
		let mut builder = crate::audio::AudioTimelineBuilder::new();
		let source0 = builder.add_source(crate::audio::AudioLocation::File("a.mp3".to_string()), Some(2000));
		let source1 = builder.add_source(crate::audio::AudioLocation::File("b.mp3".to_string()), None);
		builder.add_clip(source0, 0, 2000, 0.into(), 1.into());
		builder.add_clip(source1, 0, 3000, 1.into(), 2.into());
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("a b".to_string()));
		doc.set_audio(builder.build());
		assert_eq!(doc.stats().audio_file_count, 2);
		assert_eq!(doc.stats().audio_total_duration_ms, None);
	}

	#[test]
	fn parser_context_builder_sets_optional_fields() {
		let context = ParserContext::new("book.epub".to_string())
			.with_password("secret".to_string())
			.with_forced_extension("txt".to_string());
		assert_eq!(context.file_path, "book.epub");
		assert_eq!(context.password.as_deref(), Some("secret"));
		assert_eq!(context.forced_extension.as_deref(), Some("txt"));
	}

	#[test]
	fn parse_settings_reach_the_parser_context() {
		let settings = ParseSettings { strip_running_text: false, ..ParseSettings::default() };
		let context = ParserContext::new("book.pdf".to_string()).with_parse_settings(settings);
		assert!(!context.strip_running_text);
		assert!(context.render_tables_inline);
		assert!(context.join_pdf_paragraphs);
		assert!(context.markdown_dollar_math);
	}
}
