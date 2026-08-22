use crate::{
	document::{self, DocumentHandle, MarkerType, ParserContext, ParserFlags},
	parser,
	reader_core::record_history_position,
	types::{self as ffi},
};

mod export;
mod links;
mod navigation;
mod search;
mod stats;

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

#[derive(Debug, Clone)]
pub struct TextSegmentFfi {
	pub text: String,
	pub start_pos: i64,
	pub end_pos: i64,
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
		render_tables_inline: bool,
	) -> Result<Self, String> {
		let mut context = ParserContext::new(file_path.to_string());
		if !password.is_empty() {
			context = context.with_password(password.to_string());
		}
		if !forced_extension.is_empty() {
			context = context.with_forced_extension(forced_extension.to_string());
		}
		context = context.with_render_tables_inline(render_tables_inline);
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
		Self::new(&file_path, &password, &forced_extension, render_tables_inline).map_err(DocumentError::ParseError)
	}

	/// The parsed document handle backing this session.
	#[must_use]
	pub const fn handle(&self) -> &DocumentHandle {
		&self.handle
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
mod tests {
	use std::{
		fs::{self, File},
		io::Write,
		path::Path,
	};

	use super::*;
	use crate::{
		config::ConfigManager,
		document::{Document, DocumentBuffer, Marker},
		types::{NavDirection, NavTarget},
	};

	fn sample_session(parser_flags: ParserFlags) -> DocumentSession {
		let mut buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0).with_reference("chapter1.xhtml".to_string()));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 0));
		buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
		buffer.add_marker(
			Marker::new(MarkerType::Link, 6)
				.with_text("line2".to_string())
				.with_reference("https://example.com".to_string()),
		);
		buffer.add_marker(Marker::new(MarkerType::List, 6).with_level(1));
		buffer.add_marker(Marker::new(MarkerType::ListItem, 6).with_level(1).with_text("item".to_string()));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, 8));
		buffer.add_marker(
			Marker::new(MarkerType::Table, 12)
				.with_length(5)
				.with_text("line3".to_string())
				.with_reference("<table/>".to_string()),
		);
		buffer.add_marker(Marker::new(MarkerType::Separator, 5).with_length(1));
		let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
		doc.set_buffer(buffer);
		DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags,
			last_stable_position: None,
		}
	}

	fn session_with_content(content: &str) -> DocumentSession {
		let buffer = DocumentBuffer::with_content(content.to_string());
		let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
		doc.set_buffer(buffer);
		DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::empty(),
			last_stable_position: None,
		}
	}

	#[test]
	fn first_content_line_after_skips_headers_and_blank_lines() {
		let session = session_with_content("27\n\nChapter One\nThe old house stood at the end of the lane.\n");
		assert_eq!(session.first_content_line_after(0), "Chapter One");
	}

	#[test]
	fn first_content_line_after_skips_bare_page_number_header() {
		let session = session_with_content("27\n\nThe old house stood at the end of the lane.\n");
		assert_eq!(session.first_content_line_after(0), "The old house stood at the end of the lane.");
	}

	#[test]
	fn first_content_line_after_returns_empty_without_content() {
		let session = session_with_content("27\n\n");
		assert_eq!(session.first_content_line_after(0), "");
	}

	#[test]
	fn navigation_result_constructors_have_expected_flags() {
		let not_found = NavigationResult::not_found();
		assert!(!not_found.found);
		assert!(!not_found.not_supported);
		let not_supported = NavigationResult::not_supported();
		assert!(!not_supported.found);
		assert!(not_supported.not_supported);
	}

	#[test]
	fn link_activation_result_not_found_defaults() {
		let result = LinkActivationResult::not_found();
		assert!(!result.found);
		assert_eq!(result.action, LinkAction::NotFound);
		assert_eq!(result.offset, 0);
		assert_eq!(result.url, "");
	}

	#[test]
	fn set_history_clamps_out_of_range_index() {
		let mut session = sample_session(ParserFlags::NONE);
		session.set_history(&[10, 20], 99);
		let (history, index) = session.get_history();
		assert_eq!(history, &[10, 20]);
		assert_eq!(index, 1);
	}

	#[test]
	fn set_history_empty_resets_index_to_zero() {
		let mut session = sample_session(ParserFlags::NONE);
		session.set_history(&[], 99);
		let (history, index) = session.get_history();
		assert!(history.is_empty());
		assert_eq!(index, 0);
	}

	#[test]
	fn check_and_record_history_records_only_after_threshold() {
		let mut session = sample_session(ParserFlags::NONE);
		session.check_and_record_history(100);
		session.check_and_record_history(200);
		session.check_and_record_history(450);
		session.check_and_record_history(900);
		let (history, index) = session.get_history();
		assert_eq!(history, &[100, 450]);
		assert_eq!(index, 1);
	}

	#[test]
	fn nav_helpers_build_expected_request() {
		assert_eq!(DocumentSession::nav_direction(true), NavDirection::Next);
		assert_eq!(DocumentSession::nav_direction(false), NavDirection::Previous);
		let req = DocumentSession::nav_request(7, true, false, NavTarget::Heading, 2);
		assert_eq!(req.position, 7);
		assert!(req.wrap);
		assert_eq!(req.direction, NavDirection::Previous);
		assert_eq!(req.target, NavTarget::Heading);
		assert_eq!(req.level_filter, 2);
	}

	#[test]
	fn navigate_section_returns_not_supported_without_flag() {
		let session = sample_session(ParserFlags::NONE);
		let result = session.navigate_section(0, false, true);
		assert!(!result.found);
		assert!(result.not_supported);
	}

	#[test]
	fn navigate_list_and_list_item_require_support_flag() {
		let session = sample_session(ParserFlags::NONE);
		assert!(session.navigate_list(0, false, true).not_supported);
		assert!(session.navigate_list_item(0, false, true).not_supported);
		let session = sample_session(ParserFlags::SUPPORTS_LISTS);
		assert!(!session.navigate_list(0, false, true).not_supported);
		assert!(!session.navigate_list_item(0, false, true).not_supported);
	}

	#[test]
	fn status_and_percent_helpers_handle_bounds() {
		let session = sample_session(ParserFlags::NONE);
		let start = session.get_status_info(-5);
		assert_eq!(start.line_number, 1);
		assert_eq!(start.character_number, 1);
		assert_eq!(start.percentage, 0);
		let end = session.get_status_info(999);
		assert_eq!(end.percentage, 100);
		assert_eq!(session.position_from_percent(-10), 0);
		assert_eq!(session.position_from_percent(101), 17);
		assert_eq!(session.position_from_percent(1), 1);
	}

	#[test]
	fn line_and_position_helpers_are_consistent() {
		let session = sample_session(ParserFlags::NONE);
		assert_eq!(session.line_count(), 3);
		assert_eq!(session.position_from_line(1), 0);
		assert_eq!(session.position_from_line(2), 6);
		assert_eq!(session.position_from_line(999), 17);
	}

	#[test]
	fn page_helpers_report_counts_and_offsets() {
		let session = sample_session(ParserFlags::NONE);
		assert_eq!(session.page_count(), 2);
		assert!(session.current_page(0) > 0);
		assert!(session.current_page(8) >= session.current_page(0));
		assert_eq!(session.page_offset(1), 0);
		assert_eq!(session.page_offset(2), 8);
		assert_eq!(session.page_offset(0), -1);
		assert_eq!(session.page_offset(-1), -1);
	}

	#[test]
	fn text_range_and_line_text_extract_expected_content() {
		let session = sample_session(ParserFlags::NONE);
		assert_eq!(session.get_text_range(0, 5), "line1");
		assert_eq!(session.get_text_range(5, 5), "");
		assert_eq!(session.get_line_text(0), "line1");
		assert_eq!(session.get_line_text(7), "line2");
		assert_eq!(session.get_line_text(999), "line3");
	}

	#[test]
	fn has_headings_checks_specific_and_any_levels() {
		let session = sample_session(ParserFlags::NONE);
		assert!(session.has_headings(None));
		assert!(session.has_headings(Some(1)));
		assert!(!session.has_headings(Some(2)));
		assert!(!session.has_headings(Some(99)));
	}

	#[test]
	fn get_formatting_markers_returns_only_bold_italic_underline_markers() {
		let mut buffer = DocumentBuffer::with_content("line1\nline2\nline3".to_string());
		buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::Bold, 0).with_length(5));
		buffer.add_marker(Marker::new(MarkerType::Italic, 6).with_length(5));
		buffer.add_marker(Marker::new(MarkerType::Underline, 12).with_length(5));
		let mut doc = Document::new().with_title("Title".to_string()).with_author("Author".to_string());
		doc.set_buffer(buffer);
		let session = DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};

		let markers = session.get_formatting_markers();

		assert_eq!(markers.len(), 3);
		assert_eq!(markers[0].mtype, MarkerType::Bold);
		assert_eq!(markers[0].position, 0);
		assert_eq!(markers[0].length, 5);
		assert_eq!(markers[1].mtype, MarkerType::Italic);
		assert_eq!(markers[1].position, 6);
		assert_eq!(markers[1].length, 5);
		assert_eq!(markers[2].mtype, MarkerType::Underline);
		assert_eq!(markers[2].position, 12);
		assert_eq!(markers[2].length, 5);
	}

	#[test]
	fn table_and_section_accessors_require_in_range_and_reference() {
		let session = sample_session(ParserFlags::NONE);
		assert_eq!(session.get_table_at_position(13).as_deref(), Some("<table/>"));
		assert!(session.get_table_at_position(2).is_none());
		assert_eq!(session.get_current_section_path(0).as_deref(), Some("chapter1.xhtml"));
	}

	#[test]
	fn activate_link_returns_not_found_outside_link_text() {
		let session = sample_session(ParserFlags::NONE);
		let result = session.activate_link(2);
		assert!(!result.found);
		assert_eq!(result.action, LinkAction::NotFound);
	}

	#[test]
	fn activate_link_resolves_external_links() {
		let session = sample_session(ParserFlags::NONE);
		let result = session.activate_link(7);
		assert!(result.found);
		assert_eq!(result.action, LinkAction::External);
		assert_eq!(result.url, "https://example.com");
	}

	#[test]
	fn link_list_reports_closest_index_and_text() {
		let session = sample_session(ParserFlags::NONE);
		let list = session.link_list(7);
		assert_eq!(list.items.len(), 1);
		assert_eq!(list.items[0].offset, 6);
		assert_eq!(list.items[0].text, "line2");
		assert_eq!(list.closest_index, 0);
	}

	#[test]
	fn get_text_segment_current_direction_finds_enclosing_paragraph_start() {
		// "line2" spans bytes 6..11; position 8 lands mid-paragraph, e.g. where a link marker
		// embedded in the middle of a sentence would sit. The Current direction must still
		// return the full enclosing paragraph, not a suffix truncated from the given position.
		let session = sample_session(ParserFlags::NONE);
		let seg = session.get_text_segment(8, SegmentTypeFfi::Paragraph, SegmentDirectionFfi::Current);
		assert_eq!(seg.text, "line2");
		assert_eq!(seg.start_pos, 6);
		assert_eq!(seg.end_pos, 11);
	}

	#[test]
	fn heading_tree_builds_parent_links_and_closest_index() {
		let mut buffer = DocumentBuffer::with_content("a\nb\nc".to_string());
		buffer.add_marker(Marker::new(MarkerType::Heading1, 0).with_level(1).with_text("H1".to_string()));
		buffer.add_marker(Marker::new(MarkerType::Heading2, 2).with_level(2).with_text("H2".to_string()));
		buffer.add_marker(Marker::new(MarkerType::Heading1, 4).with_level(1).with_text("H1b".to_string()));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		let session = DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		let tree = session.heading_tree(3);
		assert_eq!(tree.items.len(), 3);
		assert_eq!(tree.items[0].parent_index, -1);
		assert_eq!(tree.items[1].parent_index, 0);
		assert_eq!(tree.items[2].parent_index, -1);
		assert_eq!(tree.closest_index, 1);
	}

	#[test]
	fn history_navigation_returns_not_found_when_empty() {
		let mut session = sample_session(ParserFlags::NONE);
		assert!(!session.history_go_back(0).found);
		assert!(!session.history_go_forward(0).found);
	}

	#[test]
	fn history_navigation_updates_index_and_returns_targets() {
		let mut session = sample_session(ParserFlags::NONE);
		session.set_history(&[10, 20, 30], 2);
		let back = session.history_go_back(30);
		assert!(back.found);
		assert_eq!(back.offset, 20);
		let forward = session.history_go_forward(20);
		assert!(forward.found);
		assert_eq!(forward.offset, 30);
	}

	#[test]
	fn webview_target_path_returns_none_for_missing_markdown_file() {
		let session = DocumentSession {
			handle: sample_session(ParserFlags::NONE).handle,
			file_path: "C:\\docs\\chapter.md".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		assert!(session.webview_target_path(0, "C:\\temp").is_none());
	}

	#[test]
	fn webview_target_path_returns_none_for_non_webview_extensions() {
		let session = sample_session(ParserFlags::NONE);
		assert!(session.webview_target_path(0, "C:\\temp").is_none());
	}

	#[test]
	fn extract_resource_returns_false_for_non_epub_files() {
		let session = DocumentSession {
			handle: sample_session(ParserFlags::NONE).handle,
			file_path: "C:\\docs\\chapter.txt".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		assert_eq!(session.extract_resource("anything", "out.file").ok(), Some(false));
	}

	fn session_with_path(file_path: &str) -> DocumentSession {
		DocumentSession {
			handle: sample_session(ParserFlags::NONE).handle,
			file_path: file_path.to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		}
	}

	fn unique_temp_dir() -> std::path::PathBuf {
		let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
		std::env::temp_dir().join(format!("paperback_source_test_{nanos}"))
	}

	#[test]
	fn source_view_available_matches_text_source_formats() {
		assert!(session_with_path("book.epub").source_view_available());
		assert!(session_with_path("page.html").source_view_available());
		assert!(session_with_path("page.htm").source_view_available());
		assert!(session_with_path("page.xhtml").source_view_available());
		assert!(session_with_path("notes.md").source_view_available());
		assert!(session_with_path("notes.markdown").source_view_available());
		assert!(!session_with_path("doc.pdf").source_view_available());
		assert!(!session_with_path("doc.docx").source_view_available());
		assert!(!session_with_path("plain.txt").source_view_available());
	}

	#[test]
	fn view_source_returns_none_for_unsupported_format() {
		let dir = unique_temp_dir();
		let src = dir.join("doc.pdf");
		fs::create_dir_all(&dir).unwrap();
		fs::write(&src, b"%PDF-1.7").unwrap();
		let session = session_with_path(&src.to_string_lossy());
		assert!(session.view_source(0, &dir.to_string_lossy()).is_none());
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn view_source_writes_html_source_and_maps_caret_forward() {
		let dir = unique_temp_dir();
		fs::create_dir_all(&dir).unwrap();
		let html = "<html><body><p id=\"a\">Alpha</p><p id=\"b\">Bravo</p></body></html>";
		let src = dir.join("page.html");
		fs::write(&src, html.as_bytes()).unwrap();
		let session = session_with_path(&src.to_string_lossy());

		let at_start = session.view_source(0, &dir.to_string_lossy()).expect("source at start");
		// Source written verbatim to a .txt file.
		assert!(at_start.path.ends_with("page.html.source.txt"));
		assert_eq!(fs::read_to_string(&at_start.path).unwrap(), html);

		// A later reading position maps to a caret deeper in the source.
		let at_bravo = session.view_source(6, &dir.to_string_lossy()).expect("source at bravo");
		assert!(at_bravo.caret > at_start.caret);
		let tail: String = html.chars().skip(usize::try_from(at_bravo.caret).unwrap()).collect();
		assert!(tail.contains("Bravo"), "caret should land at/before the second paragraph: {tail}");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn view_source_for_markdown_maps_caret_to_current_block() {
		let dir = unique_temp_dir();
		fs::create_dir_all(&dir).unwrap();
		let md = "# Title\n\nFirst paragraph.\n\nSecond paragraph.\n";
		let src = dir.join("notes.md");
		fs::write(&src, md.as_bytes()).unwrap();
		// A real session populates id_positions with pb-block-N anchors.
		let session = DocumentSession::new(&src.to_string_lossy(), "", "", false).expect("open markdown");

		let rendered = session.content();
		let pos = i64::try_from(rendered.find("Second").expect("second block rendered")).unwrap();
		let view = session.view_source(pos, &dir.to_string_lossy()).expect("markdown source");

		assert!(view.path.ends_with("notes.md.source.txt"));
		assert_eq!(fs::read_to_string(&view.path).unwrap(), md);
		// Caret lands at the start of the second paragraph in the raw Markdown.
		let tail: String = md.chars().skip(usize::try_from(view.caret).unwrap()).collect();
		assert!(tail.starts_with("Second paragraph."), "caret should be at the current block: {tail}");
		let _ = fs::remove_dir_all(&dir);
	}

	#[test]
	fn navigate_page_returns_found_and_page_marker_index() {
		let session = sample_session(ParserFlags::NONE);
		let result = session.navigate_page(0, false, true);
		assert!(result.found);
		assert!(!result.not_supported);
		assert_eq!(result.offset, 8);
		assert!(result.marker_index >= 0);
	}

	#[test]
	fn navigate_link_returns_found_when_link_exists() {
		let session = sample_session(ParserFlags::NONE);
		let result = session.navigate_link(0, false, true);
		assert!(result.found);
		assert!(!result.not_supported);
		assert_eq!(result.offset, 6);
	}

	#[test]
	fn navigate_table_and_separator_return_found() {
		let session = sample_session(ParserFlags::NONE);
		let table = session.navigate_table(0, false, true);
		assert!(table.found);
		assert_eq!(table.offset, 12);
		let separator = session.navigate_separator(0, false, true);
		assert!(separator.found);
		assert_eq!(separator.offset, 5);
	}

	#[test]
	fn navigate_heading_respects_level_support() {
		let session = sample_session(ParserFlags::NONE);
		let any_level = session.navigate_heading(-1, false, true, 0);
		assert!(!any_level.not_supported);
		assert!(any_level.found);
		let missing_level = session.navigate_heading(-1, false, true, 2);
		assert!(missing_level.not_supported);
		assert!(!missing_level.found);
	}

	#[test]
	fn navigate_section_returns_found_when_flag_enabled() {
		let session = sample_session(ParserFlags::SUPPORTS_SECTIONS);
		let result = session.navigate_section(-1, false, true);
		assert!(result.found);
		assert!(!result.not_supported);
	}

	#[test]
	fn navigate_bookmark_and_note_return_not_found_with_empty_config() {
		let session = sample_session(ParserFlags::NONE);
		let config = ConfigManager::new();
		assert!(!session.navigate_bookmark(&config, 0, false, true).found);
		assert!(!session.navigate_note(&config, 0, false, true).found);
	}

	#[test]
	fn bookmark_display_at_position_returns_not_found_without_data() {
		let session = sample_session(ParserFlags::NONE);
		let config = ConfigManager::new();
		let display = session.bookmark_display_at_position(&config, 0);
		assert!(!display.found);
		assert_eq!(display.note, "");
		assert_eq!(display.snippet, "");
	}

	#[test]
	fn get_current_section_path_returns_none_when_reference_empty() {
		let mut buffer = DocumentBuffer::with_content("line".to_string());
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		let session = DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		assert!(session.get_current_section_path(0).is_none());
	}

	#[test]
	fn extract_resource_for_missing_epub_returns_error() {
		let session = DocumentSession {
			handle: sample_session(ParserFlags::NONE).handle,
			file_path: "C:\\path\\does\\not\\exist.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		assert!(session.extract_resource("x", "y").is_err());
	}

	/// Builds a session whose buffer contains a table marker spanning a display range, used to
	/// exercise `get_table_at_position`'s half-open `[position, position + length)` check.
	fn table_session() -> DocumentSession {
		// Layout (display units): "before\n" (0..7), table span "tbl\n" (7..11), "after\n" (11..17).
		let html = "<table><tr><td>a</td><td>b</td></tr></table>";
		let mut buffer = DocumentBuffer::with_content("before\ntbl\nafter\n".to_string());
		// Table marker length is the DISPLAY extent of the emitted span ("tbl\n" -> 4).
		buffer.add_marker(
			Marker::new(MarkerType::Table, 7)
				.with_length(4)
				.with_text("tbl".to_string())
				.with_reference(html.to_string()),
		);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.compute_stats();
		DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		}
	}

	#[test]
	fn get_table_at_position_uses_display_length() {
		let session = table_session();
		// Table marker at display position 7 with length 4 -> half-open range [7, 11).
		assert_eq!(session.get_table_at_position(7).as_deref(), Some("<table><tr><td>a</td><td>b</td></tr></table>"));
		assert_eq!(session.get_table_at_position(10).as_deref(), Some("<table><tr><td>a</td><td>b</td></tr></table>"));
		assert!(session.get_table_at_position(11).is_none());
		assert!(session.get_table_at_position(2).is_none());
	}

	#[test]
	fn get_table_at_position_handles_multibyte_extent() {
		// A table marker whose display extent exceeds its char-count would have been mis-measured
		// by the old `text.chars().count()` logic. Here the displayed text is shorter (in chars)
		// than the display extent, so the caret near the end is only inside the table when using
		// `marker.length`.
		let mut buffer = DocumentBuffer::with_content("\u{1F600}\u{1F600}\u{1F600}".to_string());
		// Three non-BMP emoji: 3 chars but 6 display (UTF-16) units. Marker spans the whole range.
		buffer.add_marker(
			Marker::new(MarkerType::Table, 0)
				.with_length(6)
				.with_text("x".to_string())
				.with_reference("<table/>".to_string()),
		);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.compute_stats();
		let session = DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		// Position 5 is within [0, 6) by display length but would be outside [0, 1) by char count.
		assert_eq!(session.get_table_at_position(5).as_deref(), Some("<table/>"));
		assert!(session.get_table_at_position(6).is_none());
	}

	#[test]
	fn activate_link_returns_not_found_when_reference_missing() {
		let mut buffer = DocumentBuffer::with_content("line1\nline2".to_string());
		buffer.add_marker(Marker::new(MarkerType::Link, 6).with_text("line2".to_string()));
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		let session = DocumentSession {
			handle: DocumentHandle::new(doc),
			file_path: "book.epub".to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags: ParserFlags::NONE,
			last_stable_position: None,
		};
		let result = session.activate_link(7);
		assert!(!result.found);
		assert_eq!(result.action, LinkAction::NotFound);
	}

	/// Builds a minimal real EPUB on disk whose spine chapter references an image
	/// via a relative path that only resolves if sibling directory structure is
	/// preserved on extraction, then returns its path.
	fn build_epub_with_relative_image(dir: &Path) -> std::path::PathBuf {
		use zip::{ZipWriter, write::FileOptions};

		let epub_path = dir.join("book.epub");
		let file = File::create(&epub_path).expect("create epub file");
		let mut writer = ZipWriter::new(file);
		let opts = FileOptions::<()>::default();

		writer.start_file("mimetype", opts).unwrap();
		writer.write_all(b"application/epub+zip").unwrap();

		writer.start_file("META-INF/container.xml", opts).unwrap();
		writer
			.write_all(
				br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
	<rootfiles>
		<rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
	</rootfiles>
</container>"#,
			)
			.unwrap();

		writer.start_file("OEBPS/content.opf", opts).unwrap();
		writer
			.write_all(
				br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="bookid">
	<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
		<dc:title>Test Book</dc:title>
		<dc:identifier id="bookid">test-book</dc:identifier>
	</metadata>
	<manifest>
		<item id="chapter1" href="Text/chapter1.xhtml" media-type="application/xhtml+xml"/>
		<item id="cover-img" href="Images/cover.jpg" media-type="image/jpeg"/>
	</manifest>
	<spine>
		<itemref idref="chapter1"/>
	</spine>
</package>"#,
			)
			.unwrap();

		writer.start_file("OEBPS/Text/chapter1.xhtml", opts).unwrap();
		writer
			.write_all(
				br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
	<p>Chapter text.</p>
	<img src="../Images/cover.jpg" alt="Cover"/>
</body></html>"#,
			)
			.unwrap();

		writer.start_file("OEBPS/Images/cover.jpg", opts).unwrap();
		// The filler text intentionally avoids starting with a hex digit right after the
		// \xNN escapes above: `gen-pot`'s xgettext pass parses this file in C mode, where
		// \x escapes are greedy and would otherwise swallow leading hex-looking characters
		// (e.g. "fake" starting with a valid hex digit) into a wildly out-of-range escape.
		writer.write_all(b"\xFF\xD8\xFF\xE0placeholder-jpeg-bytes").unwrap();

		writer.finish().unwrap();
		epub_path
	}

	#[test]
	fn webview_target_path_extracts_sibling_image_resources() {
		let temp_root = std::env::temp_dir().join(format!(
			"paperback_webview_test_{}",
			std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
		));
		fs::create_dir_all(&temp_root).unwrap();
		let epub_path = build_epub_with_relative_image(&temp_root);

		let session = DocumentSession::new(&epub_path.to_string_lossy(), "", "", false).expect("parse test epub");
		let target = session.webview_target_path(0, &temp_root.to_string_lossy()).expect("webview target");

		let section_content = fs::read_to_string(&target.path).expect("read extracted section");
		assert!(section_content.contains("Images/cover.jpg"));

		// The image referenced relatively from the section must have been
		// extracted alongside it at the same relative location.
		let image_path = Path::new(&target.path).parent().unwrap().parent().unwrap().join("Images/cover.jpg");
		assert!(image_path.exists(), "expected image extracted at {}", image_path.display());

		fs::remove_dir_all(&temp_root).ok();
	}
}
