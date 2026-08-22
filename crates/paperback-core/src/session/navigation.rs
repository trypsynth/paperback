//! Structural navigation: jump to the next/previous heading, section, page, list, table,
//! bookmark, etc., plus the linear back/forward history stack and the flattened TOC. Every
//! `navigate_*` method funnels through [`DocumentSession::navigate_with_post`], which turns
//! a raw `reader_navigate` call into a [`super::NavigationResult`] and reports
//! [`super::NavigationResult::not_supported`] for a target the current format can't produce.

use super::{DocumentSession, MAX_HISTORY_LEN, NavigationResult, TocEntry};
use crate::{
	config::ConfigManager,
	document::{self, MarkerType, ParserFlags},
	reader_core::{
		bookmark_navigate, history_go_next, history_go_previous, reader_container_navigate, reader_navigate,
	},
	types::{self as ffi, NavDirection, NavTarget},
};

#[derive(Copy, Clone)]
struct NavigateParams {
	position: i64,
	wrap: bool,
	next: bool,
	target: NavTarget,
	level_filter: i32,
}

impl DocumentSession {
	pub(super) const fn nav_direction(next: bool) -> NavDirection {
		if next { NavDirection::Next } else { NavDirection::Previous }
	}

	pub(super) const fn nav_request(
		position: i64,
		wrap: bool,
		next: bool,
		target: NavTarget,
		level_filter: i32,
	) -> ffi::NavRequest {
		ffi::NavRequest { position, wrap, direction: Self::nav_direction(next), target, level_filter }
	}

	pub(super) fn has_marker(&self, marker_type: MarkerType) -> bool {
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

	pub(super) fn has_headings(&self, level: Option<i32>) -> bool {
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
		fn flatten(items: &[document::TocItem], level: i32, flat: &mut Vec<TocEntry>) {
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
