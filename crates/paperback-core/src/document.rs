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

pub use buffer::{DocumentBuffer, Edit, PartSpan, ReplaceOutcome};
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
		let mut stats = DocumentStats::from_text(&self.buffer.content);
		if let Some(audio) = &self.audio {
			stats.audio_file_count = audio.sources().len();
			stats.audio_total_duration_ms = audio.total_duration_ms();
		}
		self.stats = stats;
	}
}

impl Default for Document {
	fn default() -> Self {
		Self::new()
	}
}

/// The parse-time toggles a reader can change, gathered so a caller can pass them together.
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

/// The pages a caller asked for, as the one stretch that covers all of them.
///
/// A caller converting a page range out of a long document does not want the rest of it read, and
/// for a PDF reading a page is most of what parsing costs: a thousand-page extract from a 45,000
/// page file takes about a second against the forty-odd it takes to read every page and throw
/// away all but the thousand. So a parser that can start partway through is told where to start and
/// where to stop, and one that cannot ignores this.
///
/// Pages are 0-based and `end` is exclusive, unlike the 1-based ranges a reader types, because
/// this is what a parser indexes pages by. `end` is `None` for "through to the last page", since a
/// reader who writes `--pages 80-end` is naming a page count neither they nor the caller knows
/// before the document is open.
///
/// This is a hint about what is worth reading, not an instruction about what the document
/// contains: a parser still emits whatever it emits for every page, so page numbering and
/// everything downstream of it are unchanged. What it may change is any measurement taken across
/// the document -- the PDF parser's survey of repeated page-edge lines and of the body font size
/// sees the window rather than the whole book -- which is unavoidable, since measuring a
/// document-wide fact means reading the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageWindow {
	/// The first page wanted, 0-based.
	pub start: usize,
	/// One past the last page wanted, 0-based, or `None` for through to the last page.
	pub end: Option<usize>,
}

impl PageWindow {
	#[must_use]
	pub const fn new(start: usize, end: Option<usize>) -> Self {
		Self { start, end }
	}

	/// Whether a 0-based page is one the caller asked for.
	#[must_use]
	pub const fn contains(&self, page: usize) -> bool {
		page >= self.start
			&& match self.end {
				Some(end) => page < end,
				None => true,
			}
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
	/// When `true`, the PDF parser joins the lines of an untagged page back into the
	/// paragraphs they were wrapped from; when `false`, every line stands on its own. Off is
	/// for documents whose line breaks are the content, such as code listings and poetry.
	pub join_pdf_paragraphs: bool,
	/// Whether the lines a document repeats at its pages' edges are taken out of the text.
	///
	/// On by default, which is what a reader of the app expects and what every document read so
	/// far has been given. It is a judgement rather than a fact, though: a line is taken out for
	/// repeating on four or more pages, and on some documents that catches the tail of a sentence
	/// that happens to fall at a page break, which is content rather than furniture. A caller that
	/// would rather keep every word than have a tidy page edge can turn it off, and should expect
	/// a running head on every page in exchange.
	pub strip_running_text: bool,
	/// Whether Markdown text between dollar signs, as in `$x^2$` and `$$x^2$$`, is read as a TeX formula. Off is for
	/// documents whose dollars are not math, such as `$HOME/bin:$PATH` in a README: Markdown can't express which reading
	/// the writer meant.
	pub markdown_dollar_math: bool,
	/// Whether a PDF's line-end hyphens are left exactly as the document has them.
	///
	/// pdfium reports a hyphen at the end of a line without saying whether it belongs to the word
	/// (`no-one` wrapped after the hyphen) or only splits it (`sugges-tion` for `suggestion`), so
	/// the parser decides, and it decides from what the document writes elsewhere -- which means
	/// knowing how the whole book spells a word before it can set one page of it. A caller that
	/// would rather have the document's characters as they stand than that guess can turn the
	/// decision off: every hyphen at a line end is kept, and nothing about the rest of the
	/// document is gathered or consulted.
	///
	/// Off by default, and the guess is right far more often than not. On is for a caller that
	/// wants a page's text to depend on that page alone -- which is what lets a parser skip the
	/// pages it was not asked for and still produce the same text for the ones it was. See
	/// [`PageWindow`] for the other half of that.
	pub no_hyphenation: bool,
	/// The pages the caller wants, when it wants only some of them. See [`PageWindow`].
	///
	/// `None` means the whole document, which is what every caller that has not asked for a range
	/// gets, and what the desktop app always gets: it sets this nowhere, so an app session reads
	/// exactly the pages it read before this existed.
	pub page_window: Option<PageWindow>,
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
			no_hyphenation: false,
			page_window: None,
		}
	}

	/// Sets whether a PDF's line-end hyphens are left as the document has them. See
	/// [`ParserContext::no_hyphenation`].
	#[must_use]
	pub const fn with_no_hyphenation(mut self, no_hyphenation: bool) -> Self {
		self.no_hyphenation = no_hyphenation;
		self
	}

	/// Sets the pages worth reading, for a caller converting a range out of a long document. See
	/// [`PageWindow`].
	#[must_use]
	pub const fn with_page_window(mut self, window: PageWindow) -> Self {
		self.page_window = Some(window);
		self
	}

	#[must_use]
	pub fn with_password(mut self, password: String) -> Self {
		self.password = Some(password);
		self
	}

	/// Sets whether repeated page-edge lines are taken out. See
	/// [`ParserContext::strip_running_text`].
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
	fn document_compute_stats_uses_buffer_content() {
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("one two".to_string()));
		doc.compute_stats();
		assert_eq!(doc.stats.word_count, 2);
		assert_eq!(doc.stats.line_count, 1);
		assert_eq!(doc.stats.audio_file_count, 0);
		assert_eq!(doc.stats.audio_total_duration_ms, 0);
	}

	#[test]
	fn document_compute_stats_folds_in_audio_file_count_and_duration() {
		let mut builder = crate::audio::AudioTimelineBuilder::new();
		let source0 = builder.add_source(crate::audio::AudioLocation::File("a.mp3".to_string()), Some(2000));
		let source1 = builder.add_source(crate::audio::AudioLocation::File("b.mp3".to_string()), Some(3000));
		builder.add_clip(source0, 0, 2000, 0, 1);
		builder.add_clip(source1, 0, 3000, 1, 2);
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("a b".to_string()));
		doc.set_audio(builder.build());
		doc.compute_stats();
		assert_eq!(doc.stats.audio_file_count, 2);
		assert_eq!(doc.stats.audio_total_duration_ms, 5000);
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
		// The others move with it, so a partial override cannot quietly drop them.
		assert!(context.render_tables_inline);
		assert!(context.join_pdf_paragraphs);
		assert!(context.markdown_dollar_math);
	}
}
