use crate::{
	bridge::ffi::{self, NavDirection, NavTarget},
	config::ConfigManager,
	document::{DocumentHandle, MarkerType, ParserContext, ParserFlags},
	parser,
	reader_core::{bookmark_navigate, history_go_next, history_go_previous, reader_navigate, resolve_link},
};

const MAX_HISTORY_LEN: usize = 10;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkAction {
	Internal,
	External,
	NotFound,
}

impl Default for LinkAction {
	fn default() -> Self {
		Self::NotFound
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

	pub fn handle_mut(&mut self) -> &mut DocumentHandle {
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
		NavigationResult::from_nav_result(&result)
	}

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
		}
		nav_result
	}

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
		NavigationResult::from_nav_result(&result)
	}

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
		NavigationResult::from_nav_result(&result)
	}

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
		NavigationResult::from_nav_result(&result)
	}

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
		NavigationResult::from_nav_result(&result)
	}

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
			let link_index = match link_index {
				Some(idx) => idx,
				None => return LinkActivationResult::not_found(),
			};
			let marker = match self.handle.document().buffer.markers.get(link_index) {
				Some(m) => m,
				None => return LinkActivationResult::not_found(),
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
			if positions[*index + 1] == current_pos {
				*index += 1;
			} else {
				positions.truncate(*index + 1);
				positions.push(current_pos);
				*index += 1;
			}
		} else {
			positions.push(current_pos);
			*index += 1;
		}
	}
	trim_history(positions, index, max_len);
}
