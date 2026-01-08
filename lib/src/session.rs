use std::{
	fs::{self, File},
	io::{BufReader, Write},
	path::Path,
};

use sha1::{Digest, Sha1};
use zip::ZipArchive;

use crate::{
	bridge::ffi::{self, NavDirection, NavTarget},
	config::ConfigManager,
	document::{DocumentHandle, MarkerType, ParserContext, ParserFlags},
	parser,
	reader_core::{
		bookmark_navigate, get_filtered_bookmarks, history_go_next, history_go_previous, reader_navigate, resolve_link,
	},
	utils::zip as zip_utils,
};

const MAX_HISTORY_LEN: usize = 10;

/// Status information for a position in a document.
#[derive(Debug, Clone, Copy, Default)]
pub struct StatusInfo {
	/// Line number (1-based)
	pub line_number: i64,
	/// Character number (1-based)
	pub character_number: i64,
	/// Reading percentage (0-100)
	pub percentage: i32,
	/// Total character count in the document
	pub total_chars: i64,
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
}

impl DocumentSession {
	/// Creates a new document session by parsing the document at the given path.
	///
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
		let mut doc = parser::parse_document(&context).map_err(|e| e.to_string())?;
		doc.compute_stats();
		Ok(Self {
			handle: DocumentHandle::new(doc),
			file_path: file_path.to_string(),
			history: Vec::new(),
			history_index: 0,
			parser_flags,
		})
	}

	#[must_use]
	pub const fn handle(&self) -> &DocumentHandle {
		&self.handle
	}

	pub const fn handle_mut(&mut self) -> &mut DocumentHandle {
		&mut self.handle
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
	pub const fn parser_flags(&self) -> ParserFlags {
		self.parser_flags
	}

	#[must_use]
	pub fn get_history(&self) -> (&[i64], usize) {
		(&self.history, self.history_index)
	}

	pub fn set_history(&mut self, positions: &[i64], index: usize) {
		self.history = positions.to_vec();
		self.history_index = index.min(self.history.len().saturating_sub(1));
	}

	pub fn record_position(&mut self, position: i64) {
		record_position_internal(&mut self.history, &mut self.history_index, position, MAX_HISTORY_LEN);
	}

	#[must_use]
	pub fn navigate_section(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		if !self.parser_flags.contains(ParserFlags::SUPPORTS_SECTIONS) {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::Section,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_heading(&self, position: i64, wrap: bool, next: bool, level: i32) -> NavigationResult {
		if !self.has_headings(if level > 0 { Some(level) } else { None }) {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::Heading,
			level_filter: level,
		};
		let result = reader_navigate(&self.handle, &req);
		NavigationResult::from_nav_result(&result)
	}

	#[must_use]
	pub fn navigate_page(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let count = self.handle.count_markers_by_type(MarkerType::PageBreak);
		if count == 0 {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::Page,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found {
			let offset = usize::try_from(nav_result.offset).unwrap_or(0);
			nav_result.marker_index = self.handle.page_index(offset).unwrap_or(-1);
			if nav_result.marker_text.is_empty() {
				nav_result.marker_text = self.get_line_text(nav_result.offset);
			}
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_link(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let count = self.handle.count_markers_by_type(MarkerType::Link);
		if count == 0 {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::Link,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_list(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		if !self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) {
			return NavigationResult::not_supported();
		}
		let count = self.handle.count_markers_by_type(MarkerType::List);
		if count == 0 {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::List,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_list_item(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		if !self.parser_flags.contains(ParserFlags::SUPPORTS_LISTS) {
			return NavigationResult::not_supported();
		}
		let count = self.handle.count_markers_by_type(MarkerType::ListItem);
		if count == 0 {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::ListItem,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_table(&self, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let count = self.handle.count_markers_by_type(MarkerType::Table);
		if count == 0 {
			return NavigationResult::not_supported();
		}
		let req = ffi::NavRequest {
			position,
			wrap,
			direction: if next { NavDirection::Next } else { NavDirection::Previous },
			target: NavTarget::Table,
			level_filter: 0,
		};
		let result = reader_navigate(&self.handle, &req);
		let mut nav_result = NavigationResult::from_nav_result(&result);
		if nav_result.found && nav_result.marker_text.is_empty() {
			nav_result.marker_text = self.get_line_text(nav_result.offset);
		}
		nav_result
	}

	#[must_use]
	pub fn navigate_bookmark(&self, config: &ConfigManager, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let result = bookmark_navigate(config, &self.file_path, position, wrap, next, false);
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
	pub fn navigate_note(&self, config: &ConfigManager, position: i64, wrap: bool, next: bool) -> NavigationResult {
		let result = bookmark_navigate(config, &self.file_path, position, wrap, next, true);
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
	pub fn navigate_bookmark_display(
		&self,
		config: &ConfigManager,
		position: i64,
		wrap: bool,
		next: bool,
		notes_only: bool,
	) -> ffi::FfiBookmarkNavDisplay {
		let result = bookmark_navigate(config, &self.file_path, position, wrap, next, notes_only);
		if !result.found {
			return ffi::FfiBookmarkNavDisplay {
				found: false,
				wrapped: false,
				start: -1,
				end: -1,
				note: String::new(),
				snippet: String::new(),
				index: -1,
			};
		}
		let snippet = if result.start == result.end {
			self.get_line_text(result.start)
		} else {
			self.get_text_range(result.start, result.end)
		};
		ffi::FfiBookmarkNavDisplay {
			found: true,
			wrapped: result.wrapped,
			start: result.start,
			end: result.end,
			note: result.note,
			snippet,
			index: result.index,
		}
	}

	#[must_use]
	pub fn bookmark_display_at_position(
		&self,
		config: &ConfigManager,
		position: i64,
	) -> ffi::FfiBookmarkDisplayAtPosition {
		let bookmark = config.get_bookmarks(&self.file_path).into_iter().find(|bm| bm.start == position);
		let Some(bookmark) = bookmark else {
			return ffi::FfiBookmarkDisplayAtPosition { found: false, note: String::new(), snippet: String::new() };
		};
		let snippet = if bookmark.start == bookmark.end {
			self.get_line_text(bookmark.start)
		} else {
			self.get_text_range(bookmark.start, bookmark.end)
		};
		ffi::FfiBookmarkDisplayAtPosition { found: true, note: bookmark.note, snippet }
	}

	#[must_use]
	pub fn link_list(&self, position: i64) -> ffi::FfiLinkList {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let mut closest_index = -1;
		let mut items = Vec::new();
		for marker in
			self.handle.document().buffer.markers.iter().filter(|marker| marker.marker_type == MarkerType::Link)
		{
			let text = if marker.text.is_empty() {
				self.get_line_text(i64::try_from(marker.position).unwrap_or(0))
			} else {
				marker.text.clone()
			};
			if marker.position <= pos {
				closest_index = i32::try_from(items.len()).unwrap_or(-1);
			}
			items.push(ffi::FfiLinkListItem { offset: marker.position, text });
		}
		ffi::FfiLinkList { items, closest_index }
	}

	pub fn history_go_back(&mut self, current_pos: i64) -> NavigationResult {
		if self.history.is_empty() {
			return NavigationResult::not_found();
		}
		let result = history_go_previous(&self.history, self.history_index, current_pos, MAX_HISTORY_LEN);
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

	pub fn history_go_forward(&mut self, current_pos: i64) -> NavigationResult {
		if self.history.is_empty() {
			return NavigationResult::not_found();
		}
		let result = history_go_next(&self.history, self.history_index, current_pos, MAX_HISTORY_LEN);
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

	pub fn activate_link(&mut self, position: i64) -> LinkActivationResult {
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
		self.record_position(position);
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

	/// Gets the table marker at the current position.
	/// Returns None if no table marker is found at the position.
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

	/// Extracts a resource to the given output path.
	///
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
	pub fn export_content(&self, output_path: &str) -> std::io::Result<()> {
		let content = self.content();
		let mut file = File::create(output_path)?;
		file.write_all(content.as_bytes())?;
		file.flush()?;
		Ok(())
	}

	#[must_use]
	pub fn get_filtered_bookmark_display_items(
		&self,
		config: &ConfigManager,
		path: &str,
		current_pos: i64,
		filter: ffi::BookmarkFilterType,
	) -> ffi::FfiFilteredBookmarkDisplay {
		let filtered = get_filtered_bookmarks(config, path, current_pos, filter);
		let items = filtered
			.items
			.into_iter()
			.map(|item| {
				let snippet = if item.is_whole_line {
					self.get_line_text(item.start)
				} else {
					self.get_text_range(item.start, item.end)
				};
				ffi::FfiBookmarkDisplayEntry {
					start: item.start,
					end: item.end,
					note: item.note,
					snippet,
					is_whole_line: item.is_whole_line,
					index: item.index,
				}
			})
			.collect();
		ffi::FfiFilteredBookmarkDisplay { items, closest_index: filtered.closest_index }
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
			total_chars: i64::try_from(total_chars).unwrap_or(0),
		}
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

fn normalize_index(positions: &[i64], index: usize) -> usize {
	if positions.is_empty() {
		return 0;
	}
	index.min(positions.len().saturating_sub(1))
}

fn trim_history(positions: &mut Vec<i64>, index: &mut usize, max_len: usize) {
	if max_len == 0 {
		return;
	}
	while positions.len() > max_len {
		positions.remove(0);
		if *index > 0 {
			*index -= 1;
		}
	}
}

fn record_position_internal(positions: &mut Vec<i64>, index: &mut usize, current_pos: i64, max_len: usize) {
	if positions.is_empty() {
		positions.push(current_pos);
		*index = 0;
		trim_history(positions, index, max_len);
		return;
	}
	*index = normalize_index(positions, *index);
	if positions[*index] != current_pos {
		if *index + 1 < positions.len() {
			if positions[*index + 1] != current_pos {
				positions.truncate(*index + 1);
				positions.push(current_pos);
			}
		} else {
			positions.push(current_pos);
		}
		*index += 1;
	}
	trim_history(positions, index, max_len);
}
