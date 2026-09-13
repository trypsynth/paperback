use crate::{
	audio::AudioTimeline,
	document::{self, DocumentHandle, Edit, MarkerType, ParseSettings, ParserContext, ParserFlags, ReplaceOutcome},
	parser,
	reader_core::record_history_position,
	types::{self as ffi},
};

mod audio;
mod export;
mod find_all;
mod links;
mod navigation;
mod search;
mod stats;
mod window;

pub use find_all::{FindAllLine, FindSpan};
pub use window::WindowSlice;

const MAX_HISTORY_LEN: usize = 10;
const HISTORY_DISTANCE_THRESHOLD: i64 = 300;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptionsFfi {
	pub match_case: bool,
	pub whole_word: bool,
	pub regex: bool,
	pub forward: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchResultFfi {
	pub found: bool,
	pub wrapped: bool,
	pub position: i64,
}

#[derive(Debug, Clone)]
pub struct WebviewTarget {
	pub path: String,
	pub fragment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceView {
	pub path: String,
	pub caret: i64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatusInfo {
	pub line_number: i64,
	pub character_number: i64,
	pub percentage: i32,
}

#[derive(Debug, Clone)]
pub struct NavigationResult {
	pub found: bool,
	pub wrapped: bool,
	pub offset: i64,
	pub marker_text: String,
	pub marker_level: i32,
	pub marker_index: i32,
	pub not_supported: bool,
}

impl NavigationResult {
	const fn not_found() -> Self {
		Self {
			found: false,
			wrapped: false,
			offset: 0,
			marker_text: String::new(),
			marker_level: 0,
			marker_index: -1,
			not_supported: false,
		}
	}

	const fn not_supported() -> Self {
		Self {
			found: false,
			wrapped: false,
			offset: 0,
			marker_text: String::new(),
			marker_level: 0,
			marker_index: -1,
			not_supported: true,
		}
	}

	fn from_nav_result(result: &ffi::NavResult) -> Self {
		Self {
			found: result.found,
			wrapped: result.wrapped,
			offset: i64::try_from(result.offset).unwrap_or(0),
			marker_text: result.marker_text.clone(),
			marker_level: result.marker_level,
			marker_index: -1,
			not_supported: false,
		}
	}
}

#[derive(Debug, Clone)]
pub struct LinkActivationResult {
	pub found: bool,
	pub action: LinkAction,
	pub offset: i64,
	pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkAction {
	Internal,
	External,
	#[default]
	NotFound,
}

#[derive(Debug, Clone, Copy)]
pub enum SegmentTypeFfi {
	Paragraph,
	Line,
	Heading,
	Link,
	Section,
	Page,
	List,
	ListItem,
	Table,
	Separator,
	Image,
	Figure,
}

#[derive(Debug, Clone, Copy)]
pub enum SegmentDirectionFfi {
	Current,
	Next,
	Previous,
}

/// `found` is independent of `text`: a segment can be found but have no text of its own (e.g. a
/// plain-audio DAISY section marker, whose buffer content is just a placeholder space), so
/// callers must check `found` rather than treating blank `text` as "not found".
#[derive(Debug, Clone)]
pub struct TextSegmentFfi {
	pub text: String,
	pub start_pos: i64,
	pub end_pos: i64,
	pub found: bool,
}

/// `found` is `false` (other fields zeroed) when the lookup misses, e.g. an out-of-range clip
/// index. Mirrors `AudioClip` from `AudioTimeline` for platforms driving their own player.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioClipFfi {
	pub found: bool,
	pub source: i32,
	pub clip_begin_ms: i64,
	pub clip_end_ms: i64,
	pub start: i64,
	pub end: i64,
}

/// See `AudioTimeline::cursor_at_elapsed`.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioCursorFfi {
	pub found: bool,
	pub clip_index: i32,
	pub seek_ms: i64,
}

/// See `AudioTimeline::point_for_position`.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioPointFfi {
	pub found: bool,
	pub position: i64,
	pub time_ms: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
	#[error("Parse error: {0}")]
	ParseError(String),
}

impl From<String> for DocumentError {
	fn from(s: String) -> Self {
		Self::ParseError(s)
	}
}

impl LinkActivationResult {
	const fn not_found() -> Self {
		Self { found: false, action: LinkAction::NotFound, offset: 0, url: String::new() }
	}
}

pub struct DocumentSession {
	handle: DocumentHandle,
	file_path: String,
	history: Vec<i64>,
	history_index: usize,
	parser_flags: ParserFlags,
	last_stable_position: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct TocEntry {
	pub title: String,
	pub position: i64,
	pub level: i32,
}

#[derive(Debug, Clone)]
pub struct LineMarker {
	pub mtype: MarkerType,
	pub position: i64,
	pub text: String,
	pub reference: String,
	pub level: i32,
	pub length: i64,
}

#[derive(Debug, Clone)]
pub struct DocumentStatsFfi {
	pub word_count: i64,
	pub line_count: i64,
	pub char_count: i64,
	pub char_count_no_whitespace: i64,
	pub audio_file_count: i64,
	pub audio_total_duration_ms: i64,
}

#[derive(Debug, Clone)]
pub struct HeadingTreeItemFfi {
	pub offset: i64,
	pub text: String,
	pub parent_index: i32,
}

#[derive(Debug, Clone)]
pub struct HeadingTreeFfi {
	pub items: Vec<HeadingTreeItemFfi>,
	pub closest_index: i32,
}

#[derive(Debug, Clone)]
pub struct LinkListItemFfi {
	pub offset: i64,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct LinkListFfi {
	pub items: Vec<LinkListItemFfi>,
	pub closest_index: i32,
}

impl DocumentSession {
	/// # Errors
	///
	/// Returns an error if the document cannot be parsed.
	pub fn new(
		file_path: &str,
		password: &str,
		forced_extension: &str,
		settings: ParseSettings,
	) -> Result<Self, String> {
		let mut context = ParserContext::new(file_path.to_string());
		if !password.is_empty() {
			context = context.with_password(password.to_string());
		}
		if !forced_extension.is_empty() {
			context = context.with_forced_extension(forced_extension.to_string());
		}
		context = context.with_parse_settings(settings);
		let parser_flags = parser::get_parser_flags_for_context(&context);
		let doc = parser::parse_document(&context).map_err(|e| e.to_string())?;
		Ok(Self {
			handle: DocumentHandle::new(doc),
			file_path: file_path.to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags,
			last_stable_position: None,
		})
	}

	// Owned `String` params (not `&str`) because paperback.udl dictates this signature for UniFFI scaffolding.
	#[allow(clippy::needless_pass_by_value)]
	pub fn new_ffi(
		file_path: String,
		password: String,
		forced_extension: String,
		render_tables_inline: bool,
	) -> Result<Self, DocumentError> {
		// The mobile front ends have no reader-facing switch for the PDF paragraph joining,
		// so they take its default.
		let settings = ParseSettings { render_tables_inline, ..ParseSettings::default() };
		Self::new(&file_path, &password, &forced_extension, settings).map_err(DocumentError::ParseError)
	}

	/// The parsed document handle backing this session.
	#[must_use]
	pub const fn handle(&self) -> &DocumentHandle {
		&self.handle
	}

	/// Replaces the text spanning `start..end` (display units) with `text`, keeping the buffer
	/// indexes, markers, and `id_positions` in sync. Returns the display-unit length delta so the
	/// caller can adjust its sliding window. Used by the OCR flow to swap an image-only
	/// placeholder line for recognized text.
	pub fn replace_range(&mut self, start: i64, end: i64, text: &str) -> i64 {
		let start = usize::try_from(start.max(0)).unwrap_or(0);
		let end = usize::try_from(end.max(0)).unwrap_or(0);
		self.handle.replace_range(start, end, text)
	}

	/// Every page still awaiting OCR, as `(1-based page number, placeholder offset)` pairs in
	/// document order. Driven by the `ImageOnlyPage` markers the PDF parser leaves behind, which
	/// [`Self::replace_image_only_pages`] removes as it consumes them, so a page already
	/// recognized in this session never appears here twice.
	#[must_use]
	pub fn image_only_pages(&self) -> Vec<(i32, i64)> {
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::ImageOnlyPage)
			.map(|marker| {
				let offset = i64::try_from(marker.position).unwrap_or(0);
				(self.current_page(offset), offset)
			})
			.collect()
	}

	/// The offset of the OCR placeholder on the line containing `position`, if that line is one.
	/// This is what Enter keys off, so pressing it anywhere on the placeholder line works.
	#[must_use]
	pub fn image_only_page_at(&self, position: i64) -> Option<i64> {
		let (start, end) = self.line_bounds_at(position)?;
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.find(|marker| {
				marker.mtype == MarkerType::ImageOnlyPage
					&& i64::try_from(marker.position).is_ok_and(|p| p >= start && p <= end)
			})
			.map(|marker| i64::try_from(marker.position).unwrap_or(start))
	}

	/// Swaps each `(placeholder offset, recognized text)` pair for the whole placeholder line,
	/// in one pass. Offsets must be ones [`Self::image_only_pages`] or
	/// [`Self::image_only_page_at`] reported and must still be placeholders; anything else is
	/// skipped. Returns the outcome so callers can remap positions they hold themselves.
	pub fn replace_image_only_pages(&mut self, pages: &[(i64, String)]) -> ReplaceOutcome {
		let edits: Vec<Edit> = pages
			.iter()
			.filter(|(offset, _)| self.image_only_page_at(*offset).is_some())
			.filter_map(|(offset, text)| {
				let (start, end) = self.line_bounds_at(*offset)?;
				Some(Edit {
					start: usize::try_from(start.max(0)).unwrap_or(0),
					end: usize::try_from(end.max(0)).unwrap_or(0),
					text: text.clone(),
				})
			})
			.collect();
		self.handle.replace_ranges(edits)
	}

	/// This document's recorded audio, when it has any (DAISY audiobooks; text-only
	/// documents have none).
	#[must_use]
	pub fn audio(&self) -> Option<&AudioTimeline> {
		self.handle.document().audio.as_ref()
	}

	#[must_use]
	pub fn file_path(&self) -> &str {
		&self.file_path
	}

	#[must_use]
	pub fn title(&self) -> String {
		self.handle.document().title.clone()
	}

	#[must_use]
	pub fn author(&self) -> String {
		self.handle.document().author.clone()
	}

	#[must_use]
	pub fn content(&self) -> String {
		self.handle.document().buffer.content.clone()
	}

	#[must_use]
	pub const fn stats(&self) -> &document::DocumentStats {
		&self.handle.document().stats
	}

	#[must_use]
	pub fn get_history(&self) -> (&[i64], usize) {
		(&self.history, self.history_index)
	}

	pub fn set_history(&mut self, positions: &[i64], index: usize) {
		self.history = positions.to_vec();
		self.history_index = index.min(self.history.len().saturating_sub(1));
	}

	pub fn check_and_record_history(&mut self, new_position: i64) {
		if let Some(last_pos) = self.last_stable_position {
			let distance = (new_position - last_pos).abs();
			if distance >= HISTORY_DISTANCE_THRESHOLD {
				record_history_position(&mut self.history, &mut self.history_index, last_pos, MAX_HISTORY_LEN);
				self.last_stable_position = Some(new_position);
			}
		} else {
			self.last_stable_position = Some(new_position);
		}
	}

	pub const fn set_stable_position(&mut self, position: i64) {
		self.last_stable_position = Some(position);
	}
}

#[cfg(test)]
mod tests;
