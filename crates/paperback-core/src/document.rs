use std::collections::HashMap;

/// Re-exported so `crate::document::ParserFlags` keeps working; the flags themselves are
/// declared alongside the rest of each format's metadata in `paperback-formats`.
pub use paperback_formats::ParserFlags;

use crate::audio::AudioTimeline;

mod buffer;
mod handle;
mod marker;
mod stats;
mod toc;

pub use buffer::{DocumentBuffer, PartSpan};
pub use handle::DocumentHandle;
pub(crate) use marker::format_marker_types;
pub use marker::{ContainerSpan, Marker, MarkerType, is_container_marker, is_heading_marker};
pub use stats::DocumentStats;
pub use toc::TocItem;

#[derive(Debug, Clone)]
pub struct Document {
	pub title: String,
	pub author: String,
	pub buffer: DocumentBuffer,
	pub toc_items: Vec<TocItem>,
	pub id_positions: HashMap<String, usize>,
	pub spine_items: Vec<String>,
	pub manifest_items: HashMap<String, String>,
	pub stats: DocumentStats,
	/// Recorded audio for this document, when it has any.
	pub audio: Option<AudioTimeline>,
	/// True when the text spine exists only to anchor audio, carrying no prose of its own -
	/// a zip of bare narration files, say. Read-aloud UIs use this to offer time-based
	/// navigation instead of paragraph/section units that would have nothing to land on.
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
			stats: DocumentStats::default(),
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
	}

	pub fn set_audio(&mut self, audio: AudioTimeline) {
		self.audio = Some(audio);
	}

	pub fn compute_stats(&mut self) {
		self.stats = DocumentStats::from_text(&self.buffer.content);
	}
}

impl Default for Document {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone)]
pub struct ParserContext {
	pub file_path: String,
	pub password: Option<String>,
	pub forced_extension: Option<String>,
	/// When `true`, parsers emit each table's full tab-separated rendering inline; when `false`,
	/// they emit a `"[Table]: <first row>"` placeholder. Threaded into each parser at parse time.
	pub render_tables_inline: bool,
}

impl ParserContext {
	#[must_use]
	pub const fn new(file_path: String) -> Self {
		Self { file_path, password: None, forced_extension: None, render_tables_inline: true }
	}

	#[must_use]
	pub fn with_password(mut self, password: String) -> Self {
		self.password = Some(password);
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn document_compute_stats_uses_buffer_content() {
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("one two".to_string()));
		doc.compute_stats();
		assert_eq!(doc.stats.word_count, 2);
		assert_eq!(doc.stats.line_count, 1);
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
}
