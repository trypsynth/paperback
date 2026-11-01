use crate::{
	audio::AudioTimeline,
	document::{
		self, DocLen, DocPos, DocumentHandle, Edit, MarkerType, ParseSettings, ParserContext, ParserFlags,
		ReplaceOutcome,
	},
	parser,
	reader_core::record_history_position,
	types::{self as ffi},
};

mod audio;
mod export;
mod find_all;
mod images;
mod links;
mod navigation;
mod reading_step;
mod search;
mod stats;
mod window;

pub use find_all::{FindAllLine, FindSpan};
pub use window::{ParagraphFormat, WHOLE_DOCUMENT_LEN, WINDOW_LEN, WindowSlice};

const MAX_HISTORY_LEN: usize = 10;
const HISTORY_DISTANCE_THRESHOLD: DocLen = DocLen::from_raw(300);
// Past one seek keypress (10 s by default) but short enough to catch a jump to the next track.
const AUDIO_HISTORY_DISTANCE_THRESHOLD_MS: u64 = 30_000;

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptionsFfi {
	pub match_case: bool,
	pub whole_word: bool,
	pub regex: bool,
	pub forward: bool,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, Default)]
pub struct SearchResultFfi {
	pub found: bool,
	pub wrapped: bool,
	pub position: DocPos,
}

#[derive(Debug, Clone)]
pub struct WebviewTarget {
	pub path: String,
	pub fragment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceView {
	pub path: String,
	/// A position in the source, which opens as a document of its own, not in this document.
	pub caret: DocPos,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
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
	pub offset: DocPos,
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
			offset: DocPos::ZERO,
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
			offset: DocPos::ZERO,
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
			offset: result.offset,
			marker_text: result.marker_text.clone(),
			marker_level: result.marker_level,
			marker_index: -1,
			not_supported: false,
		}
	}
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct LinkActivationResult {
	pub found: bool,
	pub action: LinkAction,
	pub offset: DocPos,
	pub url: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkAction {
	Internal,
	External,
	#[default]
	NotFound,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
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
	Formula,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy)]
pub enum SegmentDirectionFfi {
	Current,
	Next,
	Previous,
}

/// Check `found`, not blank `text`: a plain-audio DAISY section is found but its text is a placeholder space.
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct TextSegmentFfi {
	pub text: String,
	pub start_pos: DocPos,
	pub end_pos: DocPos,
	pub found: bool,
}

/// What the mobile reading bar steps by; stepping by time is a seek on the app's own audio player.
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone)]
pub enum StepUnitFfi {
	Segment {
		segment_type: SegmentTypeFfi,
	},
	/// `options.forward` is ignored: the step's direction decides it.
	Find {
		query: String,
		options: SearchOptionsFfi,
	},
}

/// `text` is never blank when `found`: a segment with no text of its own reads as its section's title.
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingStepFfi {
	pub found: bool,
	pub position: DocPos,
	pub text: String,
}

/// Empty where there is no such paragraph.
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadingAheadFfi {
	pub next: String,
	pub after_next: String,
	pub previous: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioClipFfi {
	pub found: bool,
	pub source: i32,
	pub clip_begin_ms: i64,
	pub clip_end_ms: i64,
	pub start: DocPos,
	pub end: DocPos,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioCursorFfi {
	pub found: bool,
	pub clip_index: i32,
	pub seek_ms: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioPointFfi {
	pub found: bool,
	pub position: DocPos,
	pub time_ms: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Error), uniffi(flat_error))]
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
		Self { found: false, action: LinkAction::NotFound, offset: DocPos::ZERO, url: String::new() }
	}
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct DocumentSession {
	handle: DocumentHandle,
	file_path: String,
	history: Vec<DocPos>,
	history_index: usize,
	parser_flags: ParserFlags,
	/// Kept so views that re-render the source parse it the same way.
	parse_settings: ParseSettings,
	/// Kept so a second read of the file (the picture behind an image line) gets past the same lock.
	password: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct TocEntry {
	pub title: String,
	pub position: DocPos,
	pub level: i32,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct LineMarker {
	pub mtype: MarkerType,
	pub position: DocPos,
	pub text: String,
	pub reference: String,
	pub level: i32,
	pub length: DocLen,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct DocumentStatsFfi {
	pub word_count: i64,
	pub line_count: i64,
	pub char_count: i64,
	pub char_count_no_whitespace: i64,
	pub audio_file_count: i64,
	/// `-1` when there is no audio or any file's length is unknown.
	pub audio_total_duration_ms: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct HeadingTreeItemFfi {
	pub offset: DocPos,
	pub text: String,
	pub parent_index: i32,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct HeadingTreeFfi {
	pub items: Vec<HeadingTreeItemFfi>,
	pub closest_index: i32,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct LinkListItemFfi {
	pub offset: DocPos,
	pub text: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone)]
pub struct LinkListFfi {
	pub items: Vec<LinkListItemFfi>,
	pub closest_index: i32,
}

// uniffi::export reads attributes literally and does not see through cfg_attr, so the whole block is behind the feature.
#[cfg(feature = "uniffi")]
#[uniffi::export]
impl DocumentSession {
	// Owned `String` params (not `&str`) because that is the signature UniFFI generates for.
	#[uniffi::constructor(name = "new_ffi")]
	#[allow(clippy::needless_pass_by_value)]
	pub fn new_ffi(
		file_path: String,
		password: String,
		forced_extension: String,
		render_tables_inline: bool,
	) -> Result<Self, DocumentError> {
		// The mobile front ends have a reader-facing switch only for table rendering, so they take the other defaults.
		let settings = ParseSettings { render_tables_inline, ..ParseSettings::default() };
		Self::new(&file_path, &password, &forced_extension, settings).map_err(DocumentError::ParseError)
	}
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
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
}

impl DocumentSession {
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
			parse_settings: settings,
			password: password.to_string(),
		})
	}

	#[must_use]
	pub const fn handle(&self) -> &DocumentHandle {
		&self.handle
	}

	#[must_use]
	pub fn nearest_pos(&self, raw: i64) -> DocPos {
		self.handle.document().buffer.nearest_pos(raw)
	}

	/// Returns how many positions longer the document got, negative when shorter.
	pub fn replace_range(&mut self, start: DocPos, end: DocPos, text: &str) -> i64 {
		let doc_end = self.handle.document().buffer.end();
		self.handle.replace_range(start.min(doc_end)..end.min(doc_end), text)
	}

	/// Pages are 1-based; [`Self::replace_image_only_pages`] consumes the markers, so a recognized page never comes back.
	#[must_use]
	pub fn image_only_pages(&self) -> Vec<(i32, DocPos)> {
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::ImageOnlyPage)
			.map(|marker| (self.current_page(marker.position), marker.position))
			.collect()
	}

	/// Matches anywhere on the placeholder line, so Enter works wherever the caret is on it.
	#[must_use]
	pub fn image_only_page_at(&self, position: DocPos) -> Option<DocPos> {
		let (start, end) = self.line_bounds_at(position)?;
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.find(|marker| {
				marker.mtype == MarkerType::ImageOnlyPage && marker.position >= start && marker.position <= end
			})
			.map(|marker| marker.position)
	}

	pub fn replace_image_only_pages(&mut self, pages: &[(DocPos, String)]) -> ReplaceOutcome {
		let edits: Vec<Edit> = pages
			.iter()
			.filter(|(offset, _)| self.image_only_page_at(*offset).is_some())
			.filter_map(|(offset, text)| {
				let (start, end) = self.line_bounds_at(*offset)?;
				Some(Edit { start, end, text: text.clone() })
			})
			.collect();
		self.handle.replace_ranges(edits)
	}

	/// Unlike [`Self::line_bounds_at`] the span includes the trailing newline, so a replacement must put one back or the next page's `PageBreak` no longer starts a line.
	/// Starting on the page-break marker is what keeps the marker alive across a replace.
	#[must_use]
	pub fn page_text_bounds(&self, page: i32) -> Option<(DocPos, DocPos)> {
		let index = usize::try_from(page.checked_sub(1)?).ok()?;
		let offsets = self.page_offsets();
		let start = *offsets.get(index)?;
		let end = offsets.get(index + 1).copied().unwrap_or_else(|| self.handle.document().buffer.end());
		Some((start, end.max(start)))
	}

	/// Keyed by 1-based page, not offset, because each applied page moves every offset after it; spans are re-resolved from live markers.
	/// Re-OCR of a text page drops the markers inside it, since markers pointing into replaced text would be worse than none.
	pub fn replace_ocr_pages(&mut self, pages: &[(i32, String)]) -> ReplaceOutcome {
		let edits: Vec<Edit> = pages
			.iter()
			.filter_map(|(page, text)| {
				let (start, end) = self.page_text_bounds(*page)?;
				if let Some(placeholder) = self.image_only_page_at(start) {
					// `line_bounds_at` stops before the newline, so the text goes in without one.
					let (line_start, line_end) = self.line_bounds_at(placeholder)?;
					return Some(Edit { start: line_start, end: line_end, text: text.clone() });
				}
				// The page span includes its newline, so the replacement must carry one.
				let text = if text.ends_with('\n') { text.clone() } else { format!("{text}\n") };
				let doc_end = self.handle.document().buffer.end();
				Some(Edit { start: start.min(doc_end), end: end.min(doc_end), text })
			})
			.collect();
		self.handle.replace_ranges(edits)
	}

	#[must_use]
	pub fn audio(&self) -> Option<&AudioTimeline> {
		self.handle.document().audio.as_ref()
	}

	#[must_use]
	pub fn file_path(&self) -> &str {
		&self.file_path
	}

	#[must_use]
	pub fn stats(&self) -> &document::DocumentStats {
		self.handle.document().stats()
	}

	#[must_use]
	pub fn get_history(&self) -> (&[DocPos], usize) {
		(&self.history, self.history_index)
	}

	pub fn set_history(&mut self, positions: &[DocPos], index: usize) {
		self.history = positions.to_vec();
		self.history_index = index.min(self.history.len().saturating_sub(1));
	}

	/// A short jump records nothing, since a spot inside the paragraph being read is not worth returning to.
	/// A plain audio bundle has one placeholder character per file, so audio distance counts too.
	pub fn record_jump(&mut self, from: DocPos, to: DocPos) {
		let text_distance_worth_recording = to.since(from).max(from.since(to)) >= HISTORY_DISTANCE_THRESHOLD;
		let audio_distance_worth_recording =
			self.audio_distance_ms(from, to).is_some_and(|ms| ms >= AUDIO_HISTORY_DISTANCE_THRESHOLD_MS);
		if text_distance_worth_recording || audio_distance_worth_recording {
			record_history_position(&mut self.history, &mut self.history_index, from, MAX_HISTORY_LEN);
		}
	}

	fn audio_distance_ms(&self, from: DocPos, to: DocPos) -> Option<u64> {
		let audio = self.audio()?;
		let from_ms = audio.point_for_position(from)?.time_ms;
		let to_ms = audio.point_for_position(to)?.time_ms;
		Some(from_ms.abs_diff(to_ms))
	}
}

#[cfg(test)]
mod tests;
