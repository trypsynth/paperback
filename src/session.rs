use std::{
	fs::{self, File},
	io::{self, BufReader, Write},
	path::Path,
};

use sha1::{Digest, Sha1};
use zip::ZipArchive;

use crate::{
	config::ConfigManager,
	document::{self, DocumentHandle, MarkerType, ParserContext, ParserFlags},
	parser,
	reader_core::{
		bookmark_navigate, history_go_next, history_go_previous, reader_navigate, record_history_position, resolve_link,
	},
	types::{self as ffi, NavDirection, NavTarget},
	zip as zip_utils,
};

const MAX_HISTORY_LEN: usize = 10;
const HISTORY_DISTANCE_THRESHOLD: i64 = 300;

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

impl DocumentSession {
	/// # Errors
	///
	/// Returns an error if the document cannot be parsed.
	pub fn new(file_path: &str, password: &str, forced_extension: &str) -> Result<Self, String> {
		let mut context = ParserContext::new(file_path.to_string());
		if !password.is_empty() {
			context = context.with_password(password.to_string());
		}
		if !forced_extension.is_empty() {
			context = context.with_forced_extension(forced_extension.to_string());
		}
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
	pub fn get_table_at_position(&self, position: i64) -> Option<String> {
		let pos_usize = usize::try_from(position.max(0)).unwrap_or(0);
		let table_index = self.handle.current_marker_index(pos_usize, MarkerType::Table)?;
		let marker = self.handle.document().buffer.markers.get(table_index)?;
		let table_end = marker.position + marker.text.chars().count();
		if pos_usize < marker.position || pos_usize > table_end {
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
	pub fn webview_target_path(&self, position: i64, temp_dir: &str) -> Option<String> {
		let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty());
		if let Some(section_path) = section_path {
			let mut hasher = Sha1::new();
			hasher.update(self.file_path.as_bytes());
			let hash = format!("{:x}", hasher.finalize());
			let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
			if fs::create_dir_all(&doc_temp_dir).is_ok() {
				let file_name = Path::new(&section_path).file_name()?.to_string_lossy().to_string();
				let output_path = doc_temp_dir.join(file_name);
				let output_str = output_path.to_string_lossy().to_string();
				if self.extract_resource(&section_path, &output_str).ok() == Some(true) {
					return Some(output_str);
				}
			}
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		match ext.as_deref() {
			Some("html" | "htm" | "xhtml" | "md" | "markdown") => Some(self.file_path.clone()),
			_ => None,
		}
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
	pub fn export_content(&self, output_path: &str) -> io::Result<()> {
		let content = self.content();
		let mut file = File::create(output_path)?;
		file.write_all(content.as_bytes())?;
		file.flush()?;
		Ok(())
	}

	#[must_use]
	pub fn get_status_info(&self, position: i64) -> StatusInfo {
		let content = &self.handle.document().buffer.content;
		let total_chars = content.chars().count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_number = content.chars().take(pos).filter(|&c| c == '\n').count() + 1;
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
		let content = &self.handle.document().buffer.content;
		let total_chars = i64::try_from(content.chars().count()).unwrap_or(0);
		let percent = i64::from(percent.clamp(0, 100));
		if total_chars == 0 {
			return 0;
		}
		// Ceiling division: (percent * total_chars + 99) / 100
		(percent * total_chars + 99) / 100
	}

	#[must_use]
	pub fn line_count(&self) -> i64 {
		let content = &self.handle.document().buffer.content;
		let newline_count = content.chars().filter(|&c| c == '\n').count();
		// Line count is newlines + 1 (last line may not have trailing newline)
		i64::try_from(newline_count + 1).unwrap_or(1)
	}

	#[must_use]
	pub fn position_from_line(&self, line: i64) -> i64 {
		if line < 1 {
			return 0;
		}
		let content = &self.handle.document().buffer.content;
		if line == 1 {
			return 0;
		}
		let target_newlines = usize::try_from(line - 1).unwrap_or(0);
		let mut newline_count = 0;
		for (i, c) in content.chars().enumerate() {
			if c == '\n' {
				newline_count += 1;
				if newline_count == target_newlines {
					return i64::try_from(i + 1).unwrap_or(0);
				}
			}
		}
		// Line number exceeds actual lines, return end of document.
		i64::try_from(content.chars().count()).unwrap_or(0)
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
	pub fn page_offset(&self, page_index: i32) -> i64 {
		if page_index < 0 {
			return -1;
		}
		self.handle
			.get_marker_position_by_index(MarkerType::PageBreak, page_index)
			.map_or(-1, |offset| i64::try_from(offset).unwrap_or(-1))
	}

	/// Returns the text between two positions (start inclusive, end exclusive).
	#[must_use]
	pub fn get_text_range(&self, start: i64, end: i64) -> String {
		let content = &self.handle.document().buffer.content;
		let total_chars = content.chars().count();
		let start_pos = usize::try_from(start.max(0)).unwrap_or(0).min(total_chars);
		let end_pos = usize::try_from(end.max(0)).unwrap_or(0).min(total_chars);
		if start_pos >= end_pos {
			return String::new();
		}
		content.chars().skip(start_pos).take(end_pos - start_pos).collect()
	}

	#[must_use]
	pub fn get_line_text(&self, position: i64) -> String {
		let content = &self.handle.document().buffer.content;
		let total_chars = content.chars().count();
		let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
		let line_start =
			content.chars().take(pos).collect::<Vec<_>>().iter().rposition(|&c| c == '\n').map_or(0, |idx| idx + 1);
		let chars_after_start: String = content.chars().skip(line_start).collect();
		let line_end = chars_after_start.find('\n').map_or(chars_after_start.len(), |idx| idx);
		chars_after_start.chars().take(line_end).collect()
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
}
