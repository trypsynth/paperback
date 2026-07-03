use std::{
	fs::{self, File},
	io::{self, BufReader, Write},
	path::Path,
};

use base64::Engine;
use zip::ZipArchive;

use crate::{
	config::ConfigManager,
	document::{self, DocumentHandle, MarkerType, ParserContext, ParserFlags},
	parser,
	reader_core::{
		SearchOptions, bookmark_navigate, encode_url_fragment, history_go_next, history_go_previous,
		nearest_fragment_before, reader_container_navigate, reader_navigate, reader_search_with_wrap,
		record_history_position, resolve_link,
	},
	types::{self as ffi, NavDirection, NavTarget},
	util::{encoding::convert_to_utf8, zip as zip_utils},
};

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

#[derive(Copy, Clone)]
struct NavigateParams {
	position: i64,
	wrap: bool,
	next: bool,
	target: NavTarget,
	level_filter: i32,
}

#[derive(Debug, Clone)]
pub struct TocEntry {
	pub title: String,
	pub position: i64,
	pub level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerTypeFfi {
	Heading1,
	Heading2,
	Heading3,
	Heading4,
	Heading5,
	Heading6,
	PageBreak,
	SectionBreak,
	TocItem,
	Link,
	List,
	ListItem,
	Table,
	Separator,
	Image,
	Figure,
	Bold,
	Italic,
	Underline,
}

impl From<crate::document::MarkerType> for MarkerTypeFfi {
	fn from(m: crate::document::MarkerType) -> Self {
		match m {
			crate::document::MarkerType::Heading1 => Self::Heading1,
			crate::document::MarkerType::Heading2 => Self::Heading2,
			crate::document::MarkerType::Heading3 => Self::Heading3,
			crate::document::MarkerType::Heading4 => Self::Heading4,
			crate::document::MarkerType::Heading5 => Self::Heading5,
			crate::document::MarkerType::Heading6 => Self::Heading6,
			crate::document::MarkerType::PageBreak => Self::PageBreak,
			crate::document::MarkerType::SectionBreak => Self::SectionBreak,
			crate::document::MarkerType::TocItem => Self::TocItem,
			crate::document::MarkerType::Link => Self::Link,
			crate::document::MarkerType::List => Self::List,
			crate::document::MarkerType::ListItem => Self::ListItem,
			crate::document::MarkerType::Table => Self::Table,
			crate::document::MarkerType::Separator => Self::Separator,
			crate::document::MarkerType::Image => Self::Image,
			crate::document::MarkerType::Figure => Self::Figure,
			crate::document::MarkerType::Bold => Self::Bold,
			crate::document::MarkerType::Italic => Self::Italic,
			crate::document::MarkerType::Underline => Self::Underline,
		}
	}
}

#[derive(Debug, Clone)]
pub struct LineMarker {
	pub mtype: MarkerTypeFfi,
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
	pub const fn stats(&self) -> &crate::document::DocumentStats {
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

	const fn nav_direction(next: bool) -> NavDirection {
		if next { NavDirection::Next } else { NavDirection::Previous }
	}

	const fn nav_request(
		position: i64,
		wrap: bool,
		next: bool,
		target: NavTarget,
		level_filter: i32,
	) -> ffi::NavRequest {
		ffi::NavRequest { position, wrap, direction: Self::nav_direction(next), target, level_filter }
	}

	fn has_marker(&self, marker_type: MarkerType) -> bool {
		self.handle.count_markers_by_type(marker_type) > 0
	}

	fn fill_marker_text_if_empty(&self, nav_result: &mut NavigationResult) {
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
	}

	fn navigate_with_post(
		&self,
		params: NavigateParams,
		is_supported: bool,
		post: impl FnOnce(&Self, &mut NavigationResult),
	) -> NavigationResult {
		if !is_supported {
			return NavigationResult::not_supported();
		}
		let req = Self::nav_request(params.position, params.wrap, params.next, params.target, params.level_filter);
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		post(self, &mut nav_result);
		nav_result
	}

	#[must_use]
	pub fn navigate_section(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.parser_flags.contains(ParserFlags::SUPPORTS_SECTIONS);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Section, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_heading(&self, position: i64, wrap: bool, next: bool, level: i32) -> NavigationResult {
		let is_supported = self.has_headings(if level > 0 { Some(level) } else { None });
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Heading, level_filter: level },
			is_supported,
			|_s, _nav_result| {},
		)
	}

	#[must_use]
	pub fn navigate_page(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::PageBreak);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Page, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				if nav_result.found {
					let offset = usize::try_from(nav_result.offset).unwrap_or(0);
					nav_result.marker_index = s.handle.page_index(offset).unwrap_or(-1);
				}
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_link(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::Link);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Link, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_list(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::List);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::List, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_list_item(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported =
			self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::ListItem);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::ListItem, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_table(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::Table);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Table, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	/// Move relative to the container (list/table) the caret is currently inside: `to_end` jumps
	/// just past its end, otherwise to its start. Not found when the caret is not in a container.
	#[must_use]
	pub fn navigate_container(&self, position: i64, to_end: bool) -> NavigationResult {
		if !(self.has_marker(MarkerType::List) || self.has_marker(MarkerType::Table)) {
			return NavigationResult::not_supported();
		}
		let result = reader_container_navigate(&self.handle, position, to_end);
		NavigationResult::from_nav_result(&result)
	}

	#[must_use]
	pub fn navigate_separator(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::Separator);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Separator, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_image(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::Image);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Image, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	#[must_use]
	pub fn navigate_figure(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let is_supported = self.has_marker(MarkerType::Figure);
		self.navigate_with_post(
			NavigateParams { position, wrap, next, target: NavTarget::Figure, level_filter: 0 },
			is_supported,
			|s, nav_result| {
				s.fill_marker_text_if_empty(nav_result);
			},
		)
	}

	fn navigate_bookmark_inner(
		&self,
		config: &ConfigManager,
		position: i64,
		wrap: bool,
		next: bool,
		notes_only: bool,
	) -> NavigationResult {
		let result = bookmark_navigate(config, &self.file_path, position, wrap, next, notes_only);
		if result.found {
			NavigationResult {
				found: true,
				wrapped: result.wrapped,
				offset: result.start,
				marker_text: result.note.clone(),
				marker_level: 0,
				marker_index: result.index,
				not_supported: false,
			}
		} else {
			NavigationResult::not_found()
		}
	}

	#[must_use]
	pub fn navigate_bookmark(&self, config: &ConfigManager, position: i64, wrap: bool, next: bool) -> NavigationResult {
		self.navigate_bookmark_inner(config, position, wrap, next, false)
	}

	#[must_use]
	pub fn navigate_note(&self, config: &ConfigManager, position: i64, wrap: bool, next: bool) -> NavigationResult {
		self.navigate_bookmark_inner(config, position, wrap, next, true)
	}

	#[must_use]
	pub fn bookmark_display_at_position(
		&self,
		config: &ConfigManager,
		position: i64,
	) -> ffi::BookmarkDisplayAtPosition {
		let bookmark = config.get_bookmarks(&self.file_path).into_iter().find(|bm| bm.start == position);
		let Some(bookmark) = bookmark else {
			return ffi::BookmarkDisplayAtPosition { found: false, note: String::new(), snippet: String::new() };
		};
		let snippet = if bookmark.start == bookmark.end {
			self.get_line_text(bookmark.start)
		} else {
			self.get_text_range(bookmark.start, bookmark.end)
		};
		ffi::BookmarkDisplayAtPosition { found: true, note: bookmark.note, snippet }
	}

	#[must_use]
	pub fn link_list(&self, position: i64) -> ffi::LinkList {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let mut closest_index = -1;
		let mut items = Vec::new();
		for marker in self.handle.document().buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Link) {
			let text = if marker.text.is_empty() {
				self.get_line_text(i64::try_from(marker.position).unwrap_or(0))
			} else {
				marker.text.clone()
			};
			if marker.position <= pos {
				closest_index = i32::try_from(items.len()).unwrap_or(-1);
			}
			items.push(ffi::LinkListItem { offset: marker.position, text });
		}
		ffi::LinkList { items, closest_index }
	}

	#[must_use]
	pub fn get_formatting_markers(&self) -> Vec<LineMarker> {
		self.handle
			.document()
			.buffer
			.markers
			.iter()
			.filter(|m| matches!(m.mtype, MarkerType::Bold | MarkerType::Italic | MarkerType::Underline))
			.map(|m| LineMarker {
				mtype: m.mtype.into(),
				position: i64::try_from(m.position).unwrap_or(0),
				text: String::new(),
				reference: String::new(),
				level: 0,
				length: i64::try_from(m.length).unwrap_or(0),
			})
			.collect()
	}

	#[must_use]
	pub fn heading_tree(&self, position: i64) -> ffi::HeadingTree {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let mut items = Vec::new();
		let mut closest_index = -1;
		let mut min_distance = usize::MAX;
		let markers = &self.handle.document().buffer.markers;
		let mut item_stack: Vec<(i32, i32)> = Vec::new(); // (level, index)
		for marker in markers {
			if !document::is_heading_marker(marker.mtype) {
				continue;
			}
			let level = marker.level;
			while item_stack.last().is_some_and(|(l, _)| *l >= level) {
				item_stack.pop();
			}
			let parent_index = item_stack.last().map_or(-1, |(_, idx)| *idx);
			let current_index = i32::try_from(items.len()).unwrap_or(-1);
			let text = if marker.text.is_empty() {
				self.get_line_text(i64::try_from(marker.position).unwrap_or(0))
			} else {
				marker.text.clone()
			};
			items.push(ffi::HeadingTreeItem { offset: marker.position, text, parent_index });
			item_stack.push((level, current_index));
			if marker.position <= pos {
				let dist = pos - marker.position;
				if dist < min_distance {
					min_distance = dist;
					closest_index = current_index;
				}
			}
		}
		ffi::HeadingTree { items, closest_index }
	}

	#[must_use]
	pub fn get_heading_tree_ffi(&self, position: i64) -> HeadingTreeFfi {
		let tree = self.heading_tree(position);
		HeadingTreeFfi {
			items: tree
				.items
				.into_iter()
				.map(|i| HeadingTreeItemFfi {
					offset: i64::try_from(i.offset).unwrap_or(i64::MAX),
					text: i.text,
					parent_index: i.parent_index,
				})
				.collect(),
			closest_index: tree.closest_index,
		}
	}

	#[must_use]
	pub fn get_link_list_ffi(&self, position: i64) -> LinkListFfi {
		let list = self.link_list(position);
		LinkListFfi {
			items: list
				.items
				.into_iter()
				.map(|i| LinkListItemFfi { offset: i64::try_from(i.offset).unwrap_or(i64::MAX), text: i.text })
				.collect(),
			closest_index: list.closest_index,
		}
	}

	fn history_navigate(&mut self, current_pos: i64, forward: bool) -> NavigationResult {
		if self.history.is_empty() {
			return NavigationResult::not_found();
		}
		let result = if forward {
			history_go_next(&self.history, self.history_index, current_pos, MAX_HISTORY_LEN)
		} else {
			history_go_previous(&self.history, self.history_index, current_pos, MAX_HISTORY_LEN)
		};
		self.history = result.positions;
		self.history_index = result.index;
		if result.found {
			NavigationResult {
				found: true,
				wrapped: false,
				offset: result.target,
				marker_text: String::new(),
				marker_level: 0,
				marker_index: -1,
				not_supported: false,
			}
		} else {
			NavigationResult::not_found()
		}
	}

	pub fn history_go_back(&mut self, current_pos: i64) -> NavigationResult {
		self.history_navigate(current_pos, false)
	}

	pub fn history_go_forward(&mut self, current_pos: i64) -> NavigationResult {
		self.history_navigate(current_pos, true)
	}

	#[must_use]
	pub fn activate_link(&self, position: i64) -> LinkActivationResult {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let href = {
			let link_index = self.handle.current_marker_index(pos_usize, MarkerType::Link);
			let Some(link_index) = link_index else {
				return LinkActivationResult::not_found();
			};
			let Some(marker) = self.handle.document().buffer.markers.get(link_index) else {
				return LinkActivationResult::not_found();
			};
			let link_end = marker.position + marker.text.chars().count();
			if pos_usize < marker.position || pos_usize > link_end {
				return LinkActivationResult::not_found();
			}
			if marker.reference.is_empty() {
				return LinkActivationResult::not_found();
			}
			// Clone the href so we can drop the borrow on self.handle.
			marker.reference.clone()
		};
		let resolution = resolve_link(&self.handle, &href, position);
		if !resolution.found {
			LinkActivationResult::not_found()
		} else if resolution.is_external {
			LinkActivationResult { found: true, action: LinkAction::External, offset: 0, url: resolution.url }
		} else {
			LinkActivationResult {
				found: true,
				action: LinkAction::Internal,
				offset: i64::try_from(resolution.offset).unwrap_or(0),
				url: String::new(),
			}
		}
	}

	#[must_use]
	pub fn activate_link_ffi(&self, position: i64) -> LinkActivationResult {
		self.activate_link(position)
	}

	#[must_use]
	pub fn get_stats_ffi(&self) -> DocumentStatsFfi {
		let s = self.stats();
		DocumentStatsFfi {
			word_count: i64::try_from(s.word_count).unwrap_or(0),
			line_count: i64::try_from(s.line_count).unwrap_or(0),
			char_count: i64::try_from(s.char_count).unwrap_or(0),
			char_count_no_whitespace: i64::try_from(s.char_count_no_whitespace).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn get_supported_segment_types_ffi(&self) -> Vec<SegmentTypeFfi> {
		let mut supported = vec![SegmentTypeFfi::Paragraph, SegmentTypeFfi::Line];

		let has_heading = (0..=5).any(|level| {
			let mtype = match level {
				0 => MarkerType::Heading1,
				1 => MarkerType::Heading2,
				2 => MarkerType::Heading3,
				3 => MarkerType::Heading4,
				4 => MarkerType::Heading5,
				_ => MarkerType::Heading6,
			};
			self.has_marker(mtype)
		});
		if has_heading {
			supported.push(SegmentTypeFfi::Heading);
		}

		if self.has_marker(MarkerType::Link) {
			supported.push(SegmentTypeFfi::Link);
		}

		if self.parser_flags.contains(ParserFlags::SUPPORTS_SECTIONS) && self.has_marker(MarkerType::SectionBreak) {
			supported.push(SegmentTypeFfi::Section);
		}

		if self.parser_flags.contains(ParserFlags::SUPPORTS_PAGES) && self.has_marker(MarkerType::PageBreak) {
			supported.push(SegmentTypeFfi::Page);
		}

		if self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::List) {
			supported.push(SegmentTypeFfi::List);
		}

		if self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) && self.has_marker(MarkerType::ListItem) {
			supported.push(SegmentTypeFfi::ListItem);
		}

		if self.has_marker(MarkerType::Table) {
			supported.push(SegmentTypeFfi::Table);
		}

		if self.has_marker(MarkerType::Separator) {
			supported.push(SegmentTypeFfi::Separator);
		}

		if self.has_marker(MarkerType::Image) {
			supported.push(SegmentTypeFfi::Image);
		}

		if self.has_marker(MarkerType::Figure) {
			supported.push(SegmentTypeFfi::Figure);
		}

		supported
	}

	#[must_use]
	pub fn search_ffi(&self, query: String, start_position: i64, options: SearchOptionsFfi) -> SearchResultFfi {
		let mut search_options = SearchOptions::empty();
		if options.match_case {
			search_options.insert(SearchOptions::MATCH_CASE);
		}
		if options.whole_word {
			search_options.insert(SearchOptions::WHOLE_WORD);
		}
		if options.regex {
			search_options.insert(SearchOptions::REGEX);
		}
		if options.forward {
			search_options.insert(SearchOptions::FORWARD);
		}

		let result =
			reader_search_with_wrap(&self.handle.document().buffer.content, &query, start_position, search_options);
		SearchResultFfi { found: result.found, wrapped: result.wrapped, position: result.position }
	}

	#[must_use]
	pub fn get_status_info_ffi(&self, position: i64) -> StatusInfo {
		self.get_status_info(position)
	}

	#[must_use]
	pub fn position_from_percent_ffi(&self, percent: i32) -> i64 {
		self.position_from_percent(percent)
	}

	#[must_use]
	pub fn current_page_ffi(&self, position: i64) -> i32 {
		i32::try_from(self.current_page(position)).unwrap_or(0)
	}

	#[must_use]
	pub fn page_count_ffi(&self) -> i32 {
		i32::try_from(self.page_count()).unwrap_or(0)
	}

	#[must_use]
	pub fn page_offset_ffi(&self, page: i32) -> i64 {
		self.page_offset(page)
	}

	#[must_use]
	pub fn get_table_at_position(&self, position: i64) -> Option<String> {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let table_index = self.handle.current_marker_index(pos_usize, MarkerType::Table)?;
		let marker = self.handle.document().buffer.markers.get(table_index)?;
		// `length` is the display extent (Tasks 2-3); valid range is the half-open `[position, end)`.
		let table_end = marker.position + marker.length;
		if pos_usize < marker.position || pos_usize >= table_end {
			return None;
		}
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn get_current_section_path(&self, position: i64) -> Option<String> {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let section_index = self.handle.current_marker_index(pos_usize, MarkerType::SectionBreak)?;
		let marker = self.handle.document().buffer.markers.get(section_index)?;
		if marker.reference.is_empty() {
			return None;
		}
		Some(marker.reference.clone())
	}

	#[must_use]
	pub fn webview_target_path(&self, position: i64, temp_dir: &str) -> Option<WebviewTarget> {
		let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty());
		if let Some(section_path) = section_path {
			let digest = crate::config::compute_document_hash(&self.file_path);
			let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
			let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
			if fs::create_dir_all(&doc_temp_dir).is_ok() {
				let file_name = Path::new(&section_path).file_name()?.to_string_lossy().to_string();
				let output_path = doc_temp_dir.join(file_name);
				let output_str = output_path.to_string_lossy().to_string();
				if self.extract_resource(&section_path, &output_str).ok() == Some(true) {
					let fragment = self.inject_reading_anchor(position, &output_str);
					return Some(WebviewTarget { path: output_str, fragment });
				}
			}
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		match ext.as_deref() {
			Some("html" | "htm" | "xhtml") => Some(WebviewTarget { path: self.file_path.clone(), fragment: None }),
			Some("md" | "markdown") => {
				let digest = crate::config::compute_document_hash(&self.file_path);
				let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
				let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
				if fs::create_dir_all(&doc_temp_dir).is_ok() {
					let html_path = doc_temp_dir.join("document.html");
					if let Ok(bytes) = fs::read(&self.file_path) {
						let markdown_text = convert_to_utf8(&bytes);
						let html_body = parser::markdown::markdown_to_html(&markdown_text);
						let full_html =
							format!("<html><head><meta charset=\"utf-8\"></head><body>{html_body}</body></html>");
						if fs::write(&html_path, full_html.as_bytes()).is_ok() {
							return Some(WebviewTarget {
								path: html_path.to_string_lossy().to_string(),
								fragment: None,
							});
						}
					}
				}
				None
			}
			_ => None,
		}
	}

	/// Inserts an empty anchor element at the current reading position into the
	/// extracted section file and returns its id, for use as a URL `#fragment`.
	fn inject_reading_anchor(&self, position: i64, file_path: &str) -> Option<String> {
		const READING_POS_ANCHOR_ID: &str = "paperback-reading-pos";
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
		let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
		let relative = pos.checked_sub(section_start)?;
		let content = convert_to_utf8(&fs::read(file_path).ok()?);
		let injected = parser::xml_to_text::inject_anchor_at_position(&content, relative, READING_POS_ANCHOR_ID)?;
		fs::write(file_path, injected.as_bytes()).ok()?;
		Some(READING_POS_ANCHOR_ID.to_string())
	}

	/// Returns the id of the element closest at-or-before `position` in the current
	/// section, for use as a `#fragment` when opening the section in a web view.
	#[must_use]
	pub fn webview_fragment_for_position(&self, position: i64) -> Option<String> {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		nearest_fragment_before(&self.handle, pos).map(|id| encode_url_fragment(&id))
	}

	/// Returns true when the document's underlying source can be shown as text.
	#[must_use]
	pub fn source_view_available(&self) -> bool {
		if self.file_path.to_lowercase().ends_with(".epub") {
			return true;
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		matches!(ext.as_deref(), Some("html" | "htm" | "xhtml" | "md" | "markdown"))
	}

	/// Writes the underlying source of the document at `position` to a temp `.txt`
	/// file and returns its path plus the caret offset matching the reading position.
	///
	/// For EPUB the current spine section is used; for standalone HTML/XHTML and
	/// Markdown the original file is used. The caret is mapped to the source the
	/// same way the web view positions it: HTML/XHTML/EPUB via the source byte
	/// offset of the element at the reading position, Markdown via the nearest
	/// block anchor. Returns `None` for formats without a meaningful text source.
	#[must_use]
	pub fn view_source(&self, position: i64, temp_dir: &str) -> Option<SourceView> {
		let (content, caret, name) = self.source_content_for_position(position)?;
		let digest = crate::config::compute_document_hash(&self.file_path);
		let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
		let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
		fs::create_dir_all(&doc_temp_dir).ok()?;
		let output_path = doc_temp_dir.join(format!("{name}.source.txt"));
		fs::write(&output_path, content.as_bytes()).ok()?;
		Some(SourceView { path: output_path.to_string_lossy().to_string(), caret: i64::try_from(caret).unwrap_or(0) })
	}

	/// Returns `(source_text, caret_char_offset, file_name)` for the document at
	/// `position`. The caret is mapped into the returned source text.
	fn source_content_for_position(&self, position: i64) -> Option<(String, usize, String)> {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		if self.file_path.to_lowercase().ends_with(".epub") {
			let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty())?;
			let file = File::open(&self.file_path).ok()?;
			let mut archive = ZipArchive::new(BufReader::new(file)).ok()?;
			let content = zip_utils::read_zip_entry_by_name(&mut archive, &section_path).ok()?;
			let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
			let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
			let relative = pos.saturating_sub(section_start);
			let caret = Self::xml_caret(&content, relative);
			let name = Path::new(&section_path).file_name()?.to_string_lossy().to_string();
			return Some((content, caret, name));
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		let name = Path::new(&self.file_path).file_name()?.to_string_lossy().to_string();
		let content = convert_to_utf8(&fs::read(&self.file_path).ok()?);
		let caret = match ext.as_deref() {
			Some("html" | "htm" | "xhtml") => Self::xml_caret(&content, pos),
			Some("md" | "markdown") => self.markdown_caret(&content, pos),
			_ => return None,
		};
		Some((content, caret, name))
	}

	/// Maps a rendered character position to a caret offset in XML/HTML source
	/// via the byte offset of the element at that position.
	fn xml_caret(content: &str, relative: usize) -> usize {
		parser::xml_to_text::XmlToText::new()
			.find_anchor_byte_offset(content, relative)
			.and_then(|byte| Some(content.get(..byte)?.chars().count()))
			.unwrap_or(0)
	}

	/// Maps a rendered character position to a caret offset in Markdown source
	/// via the nearest `pb-block-N` anchor recorded during parsing.
	fn markdown_caret(&self, content: &str, pos: usize) -> usize {
		nearest_fragment_before(&self.handle, pos)
			.and_then(|id| id.strip_prefix("pb-block-").and_then(|n| n.parse::<usize>().ok()))
			.and_then(|index| parser::markdown::block_source_offset(content, index))
			.and_then(|byte| Some(content.get(..byte)?.chars().count()))
			.unwrap_or(0)
	}

	/// # Errors
	///
	/// Returns an error if the EPUB cannot be opened or the resource cannot be written.
	pub fn extract_resource(&self, resource_path: &str, output_path: &str) -> anyhow::Result<bool> {
		if self.file_path.to_lowercase().ends_with(".epub") {
			let file = File::open(&self.file_path)?;
			let mut archive = ZipArchive::new(BufReader::new(file))?;
			zip_utils::extract_zip_entry_to_file(&mut archive, resource_path, Path::new(output_path))?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Exports the document content to a file.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be written.
	pub fn export_as(&self, output_path: &str, format: crate::export::ExportFormat) -> io::Result<()> {
		let content = crate::export::render(&self.handle, format);
		let mut file = File::create(output_path)?;
		file.write_all(content.as_bytes())?;
		file.flush()?;
		Ok(())
	}

	#[must_use]
	pub fn get_text_segment(
		&self,
		position: i64,
		segment_type: SegmentTypeFfi,
		direction: SegmentDirectionFfi,
	) -> TextSegmentFfi {
		let nav_target = match segment_type {
			SegmentTypeFfi::Heading => Some(ffi::NavTarget::Heading),
			SegmentTypeFfi::Link => Some(ffi::NavTarget::Link),
			SegmentTypeFfi::Section => Some(ffi::NavTarget::Section),
			SegmentTypeFfi::Page => Some(ffi::NavTarget::Page),
			SegmentTypeFfi::List => Some(ffi::NavTarget::List),
			SegmentTypeFfi::ListItem => Some(ffi::NavTarget::ListItem),
			SegmentTypeFfi::Table => Some(ffi::NavTarget::Table),
			SegmentTypeFfi::Separator => Some(ffi::NavTarget::Separator),
			SegmentTypeFfi::Image => Some(ffi::NavTarget::Image),
			SegmentTypeFfi::Figure => Some(ffi::NavTarget::Figure),
			_ => None,
		};

		if let Some(target) = nav_target {
			let direction_nav = match direction {
				SegmentDirectionFfi::Previous => ffi::NavDirection::Previous,
				_ => ffi::NavDirection::Next,
			};
			let nav_req = ffi::NavRequest { position, wrap: false, direction: direction_nav, target, level_filter: 0 };
			let res = crate::reader_core::reader_navigate(&self.handle, &nav_req);
			if res.found {
				let mut text = res.marker_text.clone();
				let mut offset = res.offset as i64;
				let mut end_pos = offset;

				if text.trim().is_empty() {
					let content = &self.handle.document().buffer.content;
					let total_chars = self.handle.document().buffer.char_count();
					let start_pos_char = usize::try_from(offset.max(0)).unwrap_or(0).min(total_chars);
					let byte_idx = self.handle.document().buffer.byte_index_for_char(start_pos_char);

					let (start_byte, end_byte) =
						self.find_paragraph_boundaries(content, byte_idx, SegmentDirectionFfi::Current);
					text = content[start_byte..end_byte].trim().to_string();
					let start_char = self.handle.document().buffer.char_index_for_byte(start_byte) as i64;
					let end_char = self.handle.document().buffer.char_index_for_byte(end_byte) as i64;

					offset = start_char;
					end_pos = end_char;
				} else {
					end_pos += i64::try_from(text.chars().count()).unwrap_or(0);
				}

				return TextSegmentFfi { text, start_pos: offset, end_pos };
			}
			return TextSegmentFfi { text: String::new(), start_pos: position, end_pos: position };
		}

		let content = &self.handle.document().buffer.content;
		let total_chars = self.handle.document().buffer.char_count();
		let start_pos_char = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let byte_idx = self.handle.document().buffer.byte_index_for_char(start_pos_char);

		let (start_byte, end_byte) = if matches!(segment_type, SegmentTypeFfi::Line) {
			let line_num = self.line_from_position(start_pos_char as i64);
			let target_line = match direction {
				SegmentDirectionFfi::Previous => (line_num - 1).max(1),
				SegmentDirectionFfi::Next => line_num + 1,
				SegmentDirectionFfi::Current => line_num,
			};
			let start_char_idx = usize::try_from(self.position_from_line(target_line)).unwrap_or(0);
			let end_char_idx = usize::try_from(self.position_from_line(target_line + 1)).unwrap_or(0);

			let sb = self.handle.document().buffer.byte_index_for_char(start_char_idx);
			let eb = self.handle.document().buffer.byte_index_for_char(end_char_idx);
			(sb, eb)
		} else {
			self.find_paragraph_boundaries(content, byte_idx, direction)
		};
		let text = content[start_byte..end_byte].trim().to_string();
		let start_char = self.handle.document().buffer.char_index_for_byte(start_byte);
		let end_char = self.handle.document().buffer.char_index_for_byte(end_byte);
		TextSegmentFfi {
			text,
			start_pos: i64::try_from(start_char).unwrap_or(0),
			end_pos: i64::try_from(end_char).unwrap_or(0),
		}
	}

	fn find_paragraph_boundaries(
		&self,
		content: &str,
		byte_idx: usize,
		direction: SegmentDirectionFfi,
	) -> (usize, usize) {
		let mut start = byte_idx;

		if matches!(direction, SegmentDirectionFfi::Previous) {
			let mut search_end = byte_idx;
			while search_end > 0 && content.as_bytes()[search_end - 1] == b'\n' {
				search_end -= 1;
			}
			start = content[..search_end].rfind('\n').map_or(0, |i| i + 1);
		} else if matches!(direction, SegmentDirectionFfi::Next) {
			if let Some(next) = content[byte_idx..].find('\n') {
				start = byte_idx + next;
				while start < content.len() && content.as_bytes()[start] == b'\n' {
					start += 1;
				}
			} else {
				start = content.len();
			}
		} else {
			while start < content.len() && content.as_bytes()[start] == b'\n' {
				start += 1;
			}
		}

		let end = content[start..].find('\n').map_or(content.len(), |i| start + i);
		(start, end)
	}

	#[must_use]
	pub fn get_status_info(&self, position: i64) -> StatusInfo {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		let character_number = pos + 1;
		let percentage = if total_chars > 0 { (pos * 100) / total_chars } else { 0 };
		StatusInfo {
			line_number: i64::try_from(line_number).unwrap_or(1),
			character_number: i64::try_from(character_number).unwrap_or(1),
			percentage: i32::try_from(percentage).unwrap_or(0),
		}
	}

	#[must_use]
	pub fn position_from_percent(&self, percent: i32) -> i64 {
		let total_chars = i64::try_from(self.handle.document().buffer.char_count()).unwrap_or(0);
		let percent = i64::from(percent.clamp(0, 100));
		if total_chars == 0 {
			return 0;
		}
		// Ceiling division: (percent * total_chars + 99) / 100
		(percent * total_chars + 99) / 100
	}

	#[must_use]
	pub fn line_count(&self) -> i64 {
		let newline_count = self.handle.document().buffer.newline_positions().len();
		// Line count is newlines + 1 (last line may not have trailing newline)
		i64::try_from(newline_count + 1).unwrap_or(1)
	}

	#[must_use]
	pub fn position_from_line(&self, line: i64) -> i64 {
		if line <= 1 {
			return 0;
		}
		let buf = &self.handle.document().buffer;
		let target_newlines = usize::try_from(line - 1).unwrap_or(0);
		let newlines = buf.newline_positions();
		if target_newlines <= newlines.len() {
			i64::try_from(newlines[target_newlines - 1] + 1).unwrap_or(0)
		} else {
			i64::try_from(buf.char_count()).unwrap_or(0)
		}
	}

	#[must_use]
	pub fn line_from_position(&self, position: i64) -> i64 {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_number = buf.newline_positions().partition_point(|&p| p < pos) + 1;
		i64::try_from(line_number).unwrap_or(1)
	}

	#[must_use]
	pub fn page_count(&self) -> usize {
		self.handle.count_markers_by_type(MarkerType::PageBreak)
	}

	#[must_use]
	pub fn current_page(&self, position: i64) -> i32 {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		self.handle.page_index(pos).map_or(0, |idx| idx + 1)
	}

	#[must_use]
	pub fn page_offset(&self, page_number: i32) -> i64 {
		let index = page_number - 1;
		if index < 0 {
			return -1;
		}
		self.handle
			.get_marker_position_by_index(MarkerType::PageBreak, index)
			.map_or(-1, |offset| i64::try_from(offset).unwrap_or(-1))
	}

	/// Returns the text between two positions (start inclusive, end exclusive).
	#[must_use]
	pub fn get_text_range(&self, start: i64, end: i64) -> String {
		let total_chars = self.handle.document().buffer.char_count();
		let start_pos = usize::try_from(start.max(0)).unwrap_or(0).min(total_chars);
		let end_pos = usize::try_from(end.max(0)).unwrap_or(0).min(total_chars);
		if start_pos >= end_pos {
			return String::new();
		}

		let start_byte = self.handle.document().buffer.byte_index_for_char(start_pos);
		let end_byte = self.handle.document().buffer.byte_index_for_char(end_pos);

		self.handle.document().buffer.content[start_byte..end_byte].to_string()
	}

	#[must_use]
	pub fn get_line_text(&self, position: i64) -> String {
		let buf = &self.handle.document().buffer;
		let total_chars = buf.char_count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let newlines = buf.newline_positions();
		let line_start = match newlines.partition_point(|&p| p < pos) {
			0 => 0,
			idx => newlines[idx - 1] + 1,
		};

		let start_byte = buf.byte_index_for_char(line_start);
		let line_end_byte = buf.content[start_byte..].find('\n').map_or(buf.content.len(), |i| start_byte + i);

		buf.content[start_byte..line_end_byte].to_string()
	}

	#[must_use]
	pub fn get_line_markers(&self, line: i64) -> Vec<LineMarker> {
		let start_pos = self.position_from_line(line);
		let end_pos = self.position_from_line(line + 1);
		let start_usize = usize::try_from(start_pos.max(0)).unwrap_or(0);
		// If line + 1 overflows or is the end, end_pos might be equal to start_pos
		let end_usize = if start_pos == end_pos { usize::MAX } else { usize::try_from(end_pos.max(0)).unwrap_or(0) };

		let mut res = Vec::new();
		for marker in &self.handle.document().buffer.markers {
			if marker.position >= start_usize && marker.position < end_usize {
				res.push(LineMarker {
					mtype: MarkerTypeFfi::from(marker.mtype),
					position: i64::try_from(marker.position).unwrap_or(0),
					text: marker.text.clone(),
					reference: marker.reference.clone(),
					level: marker.level,
					length: i64::try_from(marker.length).unwrap_or(0),
				});
			} else if marker.position > end_usize {
				break;
			}
		}
		res
	}

	fn has_headings(&self, level: Option<i32>) -> bool {
		if let Some(lvl) = level {
			let marker_type = match lvl {
				1 => MarkerType::Heading1,
				2 => MarkerType::Heading2,
				3 => MarkerType::Heading3,
				4 => MarkerType::Heading4,
				5 => MarkerType::Heading5,
				6 => MarkerType::Heading6,
				_ => return false,
			};
			self.handle.count_markers_by_type(marker_type) > 0
		} else {
			self.handle.count_markers_by_type(MarkerType::Heading1) > 0
				|| self.handle.count_markers_by_type(MarkerType::Heading2) > 0
				|| self.handle.count_markers_by_type(MarkerType::Heading3) > 0
				|| self.handle.count_markers_by_type(MarkerType::Heading4) > 0
				|| self.handle.count_markers_by_type(MarkerType::Heading5) > 0
				|| self.handle.count_markers_by_type(MarkerType::Heading6) > 0
		}
	}

	#[must_use]
	pub fn get_toc(&self) -> Vec<TocEntry> {
		let mut flat = Vec::new();
		fn flatten(items: &[crate::document::TocItem], level: i32, flat: &mut Vec<TocEntry>) {
			for item in items {
				flat.push(TocEntry {
					title: item.name.clone(),
					position: i64::try_from(item.offset).unwrap_or(0),
					level,
				});
				flatten(&item.children, level + 1, flat);
			}
		}
		flatten(&self.handle.document().toc_items, 0, &mut flat);
		flat
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker};

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
		assert_eq!(markers[0].mtype, MarkerTypeFfi::Bold);
		assert_eq!(markers[0].position, 0);
		assert_eq!(markers[0].length, 5);
		assert_eq!(markers[1].mtype, MarkerTypeFfi::Italic);
		assert_eq!(markers[1].position, 6);
		assert_eq!(markers[1].length, 5);
		assert_eq!(markers[2].mtype, MarkerTypeFfi::Underline);
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
			handle: sample_session(ParserFlags::NONE).handle.clone(),
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
			handle: sample_session(ParserFlags::NONE).handle.clone(),
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
			handle: sample_session(ParserFlags::NONE).handle.clone(),
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
			handle: sample_session(ParserFlags::NONE).handle.clone(),
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
}
