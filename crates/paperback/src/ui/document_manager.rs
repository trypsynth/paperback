#[cfg(target_os = "linux")]
use std::collections::HashMap;
use std::{
	cell::Cell,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{Mutex, atomic::Ordering},
	time::Instant,
};

use paperback_core::{
	config::{ConfigManager, ReadabilityFont},
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	session::DocumentSession,
};
use patois::t;
use wxdragon::{
	color::Colour,
	event::{EventType, WindowEventData},
	prelude::*,
};

#[cfg(target_os = "windows")]
use super::rtf_write::{self, RtfFontInfo};
use super::{
	main_window::{SLEEP_TIMER_DURATION_MINUTES, SLEEP_TIMER_START_MS},
	menu_ids, status,
};

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	pub file_path: PathBuf,
	pub track: bool,
}

pub fn title_or_filename(title: String, path: &Path) -> String {
	if title.is_empty() {
		path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
	} else {
		title
	}
}

pub fn display_title(tab: &DocumentTab) -> String {
	title_or_filename(tab.session.title(), &tab.file_path)
}

const POSITION_SAVE_INTERVAL_SECS: u64 = 3;
const WXK_F10: i32 = 349;
const WXK_WINDOWS_MENU: i32 = 395;
#[cfg(target_os = "windows")]
const WXK_UP: i32 = 315;
#[cfg(target_os = "windows")]
const WXK_DOWN: i32 = 317;

pub struct DocumentManager {
	frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
	last_sound_position: Cell<Option<i64>>,
	preferred_column: Cell<Option<i64>>,
	recently_closed: Vec<PathBuf>,
	#[cfg(target_os = "linux")]
	navigation_key_map: Rc<HashMap<(i32, bool), i32>>,
}

impl DocumentManager {
	pub fn new(
		frame: Frame,
		notebook: Notebook,
		config: Rc<Mutex<ConfigManager>>,
		live_region_label: StaticText,
	) -> Self {
		Self {
			frame,
			notebook,
			tabs: Vec::new(),
			config,
			live_region_label,
			last_position_save: Cell::new(None),
			last_sound_position: Cell::new(None),
			preferred_column: Cell::new(None),
			recently_closed: Vec::new(),
			#[cfg(target_os = "linux")]
			navigation_key_map: Rc::new(build_navigation_key_map()),
		}
	}

	pub fn open_file(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path) -> bool {
		self.open_file_impl(self_rc, path, true, false, None)
	}

	pub fn open_file_restore(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path) -> bool {
		self.open_file_impl(self_rc, path, true, true, None)
	}

	pub fn open_help_file(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path) -> bool {
		self.open_file_impl(self_rc, path, false, false, None)
	}

	/// Opens a synthetic source-view document (untracked) with an explicit tab title.
	pub fn open_source_file(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path, title: &str) -> bool {
		self.open_file_impl(self_rc, path, false, false, Some(title))
	}

	fn open_file_impl(
		&mut self,
		self_rc: &Rc<Mutex<Self>>,
		path: &Path,
		track: bool,
		is_restore: bool,
		title_override: Option<&str>,
	) -> bool {
		if !path.exists() {
			let template = t("File not found: {}");
			let message = template.replace("{}", &path.to_string_lossy());
			show_error_dialog(&self.notebook, &message, &t("Error"));
			return false;
		}
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}

		let import_path = path.with_extension("paperback");
		if !is_restore && import_path.exists() {
			let message = t("A .paperback file was found for this document. Would you like to import it?");
			let title = t("Import document data");
			let dialog = MessageDialog::builder(&self.notebook, &message, &title)
				.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconQuestion | MessageDialogStyle::Centre)
				.build();
			if dialog.show_modal() == wxdragon::id::ID_YES {
				let config = self.config.lock().unwrap();
				config.import_settings_from_file(&path.to_string_lossy(), import_path.to_str().unwrap());
			}
		}

		let (password, forced_extension, render_tables_inline) = {
			let config = self.config.lock().unwrap();
			let path_str = path.to_string_lossy();
			config.refresh_document_hash(&path_str);
			let forced_extension = config.get_document_format(&path_str);
			let password = config.get_document_password(&path_str);
			let render_tables_inline = config.get_app_bool("render_tables_inline", true);
			drop(config);
			(password, forced_extension, render_tables_inline)
		};
		let path_str = path.to_string_lossy().to_string();
		tracing::info!(path = %path.display(), "opening document");
		match DocumentSession::new(&path_str, &password, &forced_extension, render_tables_inline) {
			Ok(session) => self.add_session_tab(self_rc, path, session, &password, track, title_override),
			Err(err) => {
				if err.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					let config = self.config.lock().unwrap();
					config.set_document_password(&path_str, "");
					drop(config);
					let password = prompt_for_password(&self.notebook);
					let Some(password) = password else {
						show_error_dialog(&self.notebook, &t("Password is required."), &t("Error"));
						return false;
					};
					match DocumentSession::new(&path_str, &password, &forced_extension, render_tables_inline) {
						Ok(session) => self.add_session_tab(self_rc, path, session, &password, track, title_override),
						Err(retry_error) => {
							tracing::error!(path = %path.display(), error = %retry_error, "failed to open document");
							let message = build_document_load_error_message(path, &retry_error);
							show_error_dialog(&self.notebook, &message, &t("Error"));
							false
						}
					}
				} else {
					tracing::error!(path = %path.display(), error = %err, "failed to open document");
					let message = build_document_load_error_message(path, &err);
					show_error_dialog(&self.notebook, &message, &t("Error"));
					false
				}
			}
		}
	}

	pub fn add_session_tab(
		&mut self,
		self_rc: &Rc<Mutex<Self>>,
		path: &Path,
		session: DocumentSession,
		password: &str,
		track: bool,
		title_override: Option<&str>,
	) -> bool {
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}
		let title =
			title_override.map_or_else(|| title_or_filename(session.title(), path), std::string::ToString::to_string);
		let panel = Panel::builder(&self.notebook).build();
		let config = self.config.lock().unwrap();
		let mut session = session;
		let word_wrap = config.get_app_bool("word_wrap", false);
		#[cfg(target_os = "linux")]
		let text_ctrl = Self::build_text_ctrl(panel, word_wrap, self_rc, self.frame, Rc::clone(&self.navigation_key_map));
		#[cfg(not(target_os = "linux"))]
		let text_ctrl = Self::build_text_ctrl(panel, word_wrap, self_rc);
		let rf = config.get_readability_font();
		if let Some(font) = build_font_from_readability(&rf) {
			text_ctrl.set_font(&font);
		}
		apply_foreground_color_to_ctrl(text_ctrl, rf.color);
		apply_bg_color_to_ctrl(text_ctrl, config.get_bg_color());
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		let content = session.content();
		fill_text_ctrl_with_formatting(text_ctrl, &session, &content);
		apply_readability_format_to_ctrl(
			text_ctrl,
			config.get_line_spacing(),
			config.get_paragraph_spacing(),
			config.get_letter_spacing(),
			config.get_text_alignment(),
		);
		self.notebook.add_page(&panel, &title, true, None);
		let path_str = path.to_string_lossy();
		let nav_history = config.get_navigation_history(&path_str);
		session.set_history(&nav_history.positions, nav_history.index);
		self.tabs.push(DocumentTab { panel, text_ctrl, session, file_path: path.to_path_buf(), track });
		if !password.is_empty() {
			config.set_document_password(&path_str, password);
		}
		let tab_index = self.tabs.len() - 1;
		let max_pos = self.tabs[tab_index].text_ctrl.get_last_position();
		let saved_pos = config.get_validated_document_position(&path_str, max_pos);
		let initial_pos = if saved_pos >= 0 {
			self.tabs[tab_index].text_ctrl.set_insertion_point(saved_pos);
			self.tabs[tab_index].text_ctrl.show_position(saved_pos);
			saved_pos
		} else {
			self.tabs[tab_index].text_ctrl.set_insertion_point(0);
			self.tabs[tab_index].text_ctrl.show_position(0);
			0
		};
		self.tabs[tab_index].session.set_stable_position(initial_pos);
		if track {
			config.add_recent_document(&path_str);
			config.set_document_opened(&path_str, true);
			config.add_opened_document(&path_str);
		}
		config.flush();
		true
	}

	pub fn close_document(&mut self, index: usize, save_state: bool) -> bool {
		if index >= self.tabs.len() {
			return false;
		}
		if let Some(tab) = self.tabs.get(index) {
			tracing::info!(path = %tab.file_path.display(), "closing document");
			self.recently_closed.push(tab.file_path.clone());
			let path_str = tab.file_path.to_string_lossy();
			let config = self.config.lock().unwrap();
			if save_state && tab.track {
				let position = tab.text_ctrl.get_insertion_point();
				config.set_document_position(&path_str, position);
				let (history, history_index) = tab.session.get_history();
				config.set_navigation_history(&path_str, history, history_index);
				config.set_document_opened(&path_str, false);
			}
			config.remove_opened_document(&path_str);
			config.flush();
		}
		let _page = self.notebook.get_page(index);
		self.notebook.remove_page(index);
		self.tabs.remove(index);
		let count = self.tabs.len();
		if count > 0 {
			let new_index = index.min(count - 1);
			self.notebook.set_selection(new_index);
		}
		true
	}

	pub fn active_index_after_closing(&self, index: usize) -> Option<usize> {
		let count = self.tabs.len();
		if index >= count || count <= 1 {
			return None;
		}
		let new_index = index.min(count - 2);
		Some(if new_index < index { new_index } else { new_index + 1 })
	}

	pub fn close_all_documents(&mut self) {
		while !self.tabs.is_empty() {
			self.close_document(0, true);
		}
	}

	pub fn save_all_positions(&self) {
		let config = self.config.lock().unwrap();
		for tab in &self.tabs {
			if !tab.track {
				continue;
			}
			let position = tab.text_ctrl.get_insertion_point();
			let path_str = tab.file_path.to_string_lossy();
			config.set_document_position(&path_str, position);
			let (history, history_index) = tab.session.get_history();
			config.set_navigation_history(&path_str, history, history_index);
		}
		config.flush();
	}

	pub fn save_position_throttled(&self) {
		let now = Instant::now();
		if let Some(last_save) = self.last_position_save.get()
			&& now.duration_since(last_save).as_secs() < POSITION_SAVE_INTERVAL_SECS
		{
			return;
		}
		if let Some(tab) = self.active_tab()
			&& tab.track
		{
			let position = tab.text_ctrl.get_insertion_point();
			let path_str = tab.file_path.to_string_lossy();
			let config = self.config.lock().unwrap();
			config.set_document_position(&path_str, position);
			config.flush();
		}
		self.last_position_save.set(Some(now));
	}

	pub fn active_tab_index(&self) -> Option<usize> {
		let selection = self.notebook.selection();
		if selection >= 0 { usize::try_from(selection).ok() } else { None }
	}

	pub fn active_tab(&self) -> Option<&DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get(i))
	}

	pub fn active_tab_mut(&mut self) -> Option<&mut DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get_mut(i))
	}

	pub fn get_tab(&self, index: usize) -> Option<&DocumentTab> {
		self.tabs.get(index)
	}

	pub const fn tab_count(&self) -> usize {
		self.tabs.len()
	}

	pub fn open_paths(&self) -> Vec<String> {
		self.tabs.iter().map(|tab| tab.file_path.to_string_lossy().to_string()).collect()
	}

	pub fn find_tab_by_path(&self, path: &Path) -> Option<usize> {
		let target = normalized_path_key(path);
		self.tabs.iter().position(|tab| normalized_path_key(&tab.file_path) == target)
	}

	pub fn restore_focus(&self) {
		if let Some(tab) = self.active_tab() {
			tab.text_ctrl.set_focus();
		} else {
			self.notebook.set_focus();
		}
	}

	pub fn pop_recently_closed(&mut self) -> Option<PathBuf> {
		self.recently_closed.pop()
	}

	pub fn push_recently_closed(&mut self, path: PathBuf) {
		self.recently_closed.push(path);
	}

	pub const fn has_recently_closed(&self) -> bool {
		!self.recently_closed.is_empty()
	}

	pub const fn notebook(&self) -> &Notebook {
		&self.notebook
	}

	pub fn activate_current_link(&mut self) {
		if let Some(tab) = self.active_tab_mut() {
			let pos = tab.text_ctrl.get_insertion_point();
			let result = tab.session.activate_link(pos);
			if result.found {
				match result.action {
					paperback_core::session::LinkAction::Internal => {
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(result.offset);
						tab.text_ctrl.show_position(result.offset);
						tab.session.check_and_record_history(result.offset);
						live_region::announce(self.live_region_label, &t("Navigated to internal link."));
					}
					paperback_core::session::LinkAction::External => {
						wxdragon::utils::launch_default_browser(
							&result.url,
							wxdragon::utils::BrowserLaunchFlags::Default,
						);
					}
					paperback_core::session::LinkAction::NotFound => {}
				}
			}
		}
	}
	pub fn activate_current_table(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.text_ctrl.get_insertion_point();
			tab.session.get_table_at_position(pos)
		})
	}

	pub fn update_status_bar(&self) {
		let sleep_start = SLEEP_TIMER_START_MS.load(Ordering::SeqCst);
		let sleep_duration = SLEEP_TIMER_DURATION_MINUTES.load(Ordering::SeqCst);
		if self.tabs.is_empty() {
			let mut status_text = t("Ready");
			if sleep_start > 0 {
				let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
				if remaining > 0 {
					status_text = status::format_sleep_timer_status(&status_text, remaining);
				}
			}
			self.frame.set_status_text(&status_text, 0);
			return;
		}
		if let Some(tab) = self.active_tab() {
			let position = tab.text_ctrl.get_insertion_point();
			let status_info = tab.session.get_status_info(position);
			let mut status_text = status::format_status_text(&status_info);
			if sleep_start > 0 {
				let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
				if remaining > 0 {
					status_text = status::format_sleep_timer_status(&status_text, remaining);
				}
			}
			self.frame.set_status_text(&status_text, 0);
		}
	}

	fn check_bookmark_sounds(&self) {
		let config = self.config.lock().unwrap();
		if !config.get_app_bool("bookmark_sounds", true) {
			return;
		}
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.text_ctrl.get_insertion_point();
		let prev = self.last_sound_position.get().unwrap_or(position);
		self.last_sound_position.set(Some(position));
		if prev == position {
			return;
		}
		let path_str = tab.file_path.to_string_lossy().to_string();
		let bookmarks = config.get_bookmarks(&path_str);
		drop(config);
		let mut has_note = false;
		let mut has_bookmark = false;
		for bm in &bookmarks {
			let triggered = if position > prev {
				bm.start > prev && bm.start <= position
			} else {
				bm.start >= position && bm.start < prev
			};
			if triggered {
				if bm.note.is_empty() {
					has_bookmark = true;
				} else {
					has_note = true;
				}
			}
		}
		if has_note || has_bookmark {
			super::sounds::play_bookmark_sound(has_note);
		}
	}

	pub fn reset_sound_line(&self) {
		self.last_sound_position.set(None);
	}

	pub fn apply_font(&self, font: &Font) {
		for tab in &self.tabs {
			tab.text_ctrl.set_font(font);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_color(&self, color: i32) {
		for tab in &self.tabs {
			apply_foreground_color_to_ctrl(tab.text_ctrl, color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_bg_color(&self, color: i32) {
		for tab in &self.tabs {
			apply_bg_color_to_ctrl(tab.text_ctrl, color);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_text_alignment(&self, alignment: i32) {
		for tab in &self.tabs {
			apply_text_alignment_to_ctrl(tab.text_ctrl, alignment);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_letter_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_letter_spacing_to_ctrl(tab.text_ctrl, spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_paragraph_spacing(&self, spacing: i32) {
		for tab in &self.tabs {
			apply_paragraph_spacing_to_ctrl(tab.text_ctrl, spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_line_spacing(&self, line_spacing: i32) {
		for tab in &self.tabs {
			apply_line_spacing_to_ctrl(tab.text_ctrl, line_spacing);
			tab.text_ctrl.refresh(true, None);
		}
	}

	pub fn apply_word_wrap(&mut self, self_rc: &Rc<Mutex<Self>>, word_wrap: bool) {
		let (rf, line_spacing, bg_color, text_alignment, letter_spacing, paragraph_spacing) = {
			let cfg = self.config.lock().unwrap();
			(
				cfg.get_readability_font(),
				cfg.get_line_spacing(),
				cfg.get_bg_color(),
				cfg.get_text_alignment(),
				cfg.get_letter_spacing(),
				cfg.get_paragraph_spacing(),
			)
		};
		for tab in &mut self.tabs {
			let old_ctrl = tab.text_ctrl;
			let current_pos = old_ctrl.get_insertion_point();
			let content = old_ctrl.get_value();
			#[cfg(target_os = "linux")]
			let text_ctrl =
				Self::build_text_ctrl(tab.panel, word_wrap, self_rc, self.frame, Rc::clone(&self.navigation_key_map));
			#[cfg(not(target_os = "linux"))]
			let text_ctrl = Self::build_text_ctrl(tab.panel, word_wrap, self_rc);
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
			tab.panel.set_sizer(sizer, true);
			fill_text_ctrl_with_formatting(text_ctrl, &tab.session, &content);
			if let Some(font) = build_font_from_readability(&rf) {
				text_ctrl.set_font(&font);
			}
			apply_foreground_color_to_ctrl(text_ctrl, rf.color);
			apply_bg_color_to_ctrl(text_ctrl, bg_color);
			apply_line_spacing_to_ctrl(text_ctrl, line_spacing);
			apply_paragraph_spacing_to_ctrl(text_ctrl, paragraph_spacing);
			apply_letter_spacing_to_ctrl(text_ctrl, letter_spacing);
			apply_text_alignment_to_ctrl(text_ctrl, text_alignment);
			let max_pos = text_ctrl.get_last_position();
			let pos = current_pos.clamp(0, max_pos);
			tab.panel.layout();
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
		}
	}

	/// Re-parses every open document with the new `render_tables_inline` setting and refills its
	/// text control. Re-parsing (rather than transforming in place) keeps every format's table
	/// rendering identical via the shared parse-time helper. A tab whose re-parse fails is left
	/// unchanged.
	pub fn apply_render_tables_inline(&mut self, render_tables_inline: bool) {
		// Read readability settings and collect each tab's parse inputs (path, password, forced
		// format) under a single config lock, so we don't re-lock per tab while mutating the tabs.
		let (rf, line_spacing, bg_color, text_alignment, letter_spacing, paragraph_spacing, parse_inputs) = {
			let cfg = self.config.lock().unwrap();
			let parse_inputs: Vec<(String, String, String)> = self
				.tabs
				.iter()
				.map(|tab| {
					let path_str = tab.file_path.to_string_lossy().to_string();
					let password = cfg.get_document_password(&path_str);
					let forced_extension = cfg.get_document_format(&path_str);
					(path_str, password, forced_extension)
				})
				.collect();
			(
				cfg.get_readability_font(),
				cfg.get_line_spacing(),
				cfg.get_bg_color(),
				cfg.get_text_alignment(),
				cfg.get_letter_spacing(),
				cfg.get_paragraph_spacing(),
				parse_inputs,
			)
		};
		for (tab, (path_str, password, forced_extension)) in self.tabs.iter_mut().zip(parse_inputs) {
			let current_pos = tab.text_ctrl.get_insertion_point();
			let pos = usize::try_from(current_pos.max(0)).unwrap_or(0);

			// Find the nearest anchor at-or-before the cursor using the full id_positions key
			// (unlike nearest_fragment_before, which strips the "path#" prefix for epub keys
			// making the subsequent lookup fail). Record the within-block offset so the cursor
			// lands at the same structural position after reparsing. Fallback: percentage-based
			// position for formats with no anchors.
			let stable_anchor = {
				let id_positions = &tab.session.handle().document().id_positions;
				id_positions
					.iter()
					.filter(|&(_, &off)| off <= pos)
					.max_by_key(|&(_, &off)| off)
					.map(|(key, &anchor_off)| (key.clone(), pos.saturating_sub(anchor_off)))
			};
			let fallback_percent = tab.session.get_status_info(current_pos).percentage;

			let new_session = match DocumentSession::new(&path_str, &password, &forced_extension, render_tables_inline)
			{
				Ok(session) => session,
				Err(err) => {
					tracing::error!(path = %path_str, error = %err, "failed to re-parse document for render_tables_inline toggle");
					continue;
				}
			};
			tab.session = new_session;
			let content = tab.session.content();
			fill_text_ctrl_with_formatting(tab.text_ctrl, &tab.session, &content);
			if let Some(font) = build_font_from_readability(&rf) {
				tab.text_ctrl.set_font(&font);
			}
			apply_foreground_color_to_ctrl(tab.text_ctrl, rf.color);
			apply_bg_color_to_ctrl(tab.text_ctrl, bg_color);
			apply_readability_format_to_ctrl(
				tab.text_ctrl,
				line_spacing,
				paragraph_spacing,
				letter_spacing,
				text_alignment,
			);
			tab.panel.layout();
			let max_pos = tab.text_ctrl.get_last_position();

			let restored_pos = if let Some((ref key, within)) = stable_anchor {
				match tab.session.handle().document().id_positions.get(key) {
					Some(&new_anchor_off) => i64::try_from(new_anchor_off + within).unwrap_or(0).clamp(0, max_pos),
					None => tab.session.position_from_percent(fallback_percent).clamp(0, max_pos),
				}
			} else {
				tab.session.position_from_percent(fallback_percent).clamp(0, max_pos)
			};

			tab.text_ctrl.set_insertion_point(restored_pos);
			tab.text_ctrl.show_position(restored_pos);
			tab.session.set_stable_position(restored_pos);
		}
	}

	fn build_text_ctrl(
		panel: Panel,
		word_wrap: bool,
		self_rc: &Rc<Mutex<Self>>,
		#[cfg(target_os = "linux")] frame: Frame,
		#[cfg(target_os = "linux")] navigation_key_map: Rc<HashMap<(i32, bool), i32>>,
	) -> TextCtrl {
		let style = TextCtrlStyle::MultiLine
			| TextCtrlStyle::ReadOnly
			| TextCtrlStyle::Rich2
			| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
		let text_ctrl = TextCtrl::builder(&panel).with_style(style).build();
		let dm_for_enter = Rc::clone(self_rc);
		text_ctrl.on_char(move |event| {
			if let WindowEventData::Keyboard(kbd) = event {
				if kbd.get_key_code() == Some(13) || kbd.get_key_code() == Some(32) {
					// 13 is KEY_RETURN, 32 is space
					let table_html = {
						let dm = dm_for_enter.lock().unwrap();
						dm.activate_current_table()
					};
					if let Some(html) = table_html {
						let frame = dm_for_enter.lock().unwrap().frame;
						super::dialogs::show_web_view_dialog(&frame, &t("Table View"), &html, false, None);
					} else {
						let mut dm = dm_for_enter.lock().unwrap();
						dm.activate_current_link();
					}
				} else {
					kbd.event.skip(true);
				}
			}
		});
		let dm_for_key_up = Rc::clone(self_rc);
		text_ctrl.bind_internal(EventType::KEY_UP, move |event| {
			event.skip(true);
			if let Ok(dm) = dm_for_key_up.try_lock() {
				dm.update_status_bar();
				dm.save_position_throttled();
				dm.check_bookmark_sounds();
			}
		});
		let dm_for_mouse = Rc::clone(self_rc);
		text_ctrl.bind_internal(wxdragon::event::EventType::LEFT_UP, move |event| {
			event.skip(true);
			if let Ok(dm) = dm_for_mouse.try_lock() {
				dm.preferred_column.set(None);
				dm.update_status_bar();
				dm.save_position_throttled();
				dm.check_bookmark_sounds();
			}
		});
		let text_ctrl_for_menu = text_ctrl;
		#[cfg(target_os = "windows")]
		let dm_for_nav = Rc::clone(self_rc);
		#[cfg(target_os = "linux")]
		let key_map = navigation_key_map;
		#[cfg(target_os = "linux")]
		let frame_for_keys = frame;
		text_ctrl.on_key_down(move |event| {
			if let WindowEventData::Keyboard(kbd) = &event
				&& let Some(key) = kbd.get_key_code()
			{
				if (key == WXK_F10 && kbd.shift_down()) || key == WXK_WINDOWS_MENU {
					kbd.event.skip(false);
					show_reader_context_menu(text_ctrl_for_menu);
					return;
				}
				#[cfg(target_os = "linux")]
				if !kbd.control_down() && !kbd.alt_down() {
					if let Some(&menu_id) = key_map.get(&(key, kbd.shift_down())) {
						kbd.event.skip(false);
						frame_for_keys.process_menu_command(menu_id);
						return;
					}
				}
				#[cfg(target_os = "windows")]
				if (key == WXK_DOWN || key == WXK_UP) && !kbd.shift_down() && !kbd.control_down() {
					let going_down = key == WXK_DOWN;
					let nav_result = dm_for_nav.try_lock().ok().and_then(|dm| {
						navigate_line_by_column(text_ctrl_for_menu, going_down, dm.preferred_column.get())
					});
					if let Some((new_pos, new_col)) = nav_result {
						kbd.event.skip(false);
						text_ctrl_for_menu.set_insertion_point(new_pos);
						text_ctrl_for_menu.show_position(new_pos);
						if let Ok(dm) = dm_for_nav.try_lock() {
							dm.preferred_column.set(Some(new_col));
							dm.update_status_bar();
						}
					} else {
						kbd.event.skip(true);
					}
					return;
				}
				#[cfg(target_os = "windows")]
				if let Ok(dm) = dm_for_nav.try_lock() {
					dm.preferred_column.set(None);
				}
			}
			event.skip(true);
		});
		let text_ctrl_for_right_click = text_ctrl;
		text_ctrl.bind_internal(EventType::RIGHT_UP, move |event| {
			event.skip(false);
			show_reader_context_menu(text_ctrl_for_right_click);
		});
		text_ctrl
	}
}

/// Returns (`new_position`, `preferred_column`) for character-column-based vertical navigation.
/// Uses wxdragon `PositionToXY`, `XYToPosition`, and `GetLineLength` so the cursor lands on the same
/// character column (not pixel column) on the target visual line.
#[cfg(target_os = "windows")]
fn navigate_line_by_column(text_ctrl: TextCtrl, going_down: bool, pref_col: Option<i64>) -> Option<(i64, i64)> {
	let current_pos = text_ctrl.get_insertion_point().max(0);
	let (current_col, current_line) = text_ctrl.position_to_xy(current_pos)?;
	let col = pref_col.unwrap_or(current_col);
	let target_line = if going_down { current_line + 1 } else { current_line - 1 };
	if target_line < 0 {
		return None;
	}
	let target_line_start = text_ctrl.xy_to_position(0, target_line);
	if target_line_start < 0 {
		return None;
	}
	let target_line_len = i64::from(text_ctrl.get_line_length(target_line));
	let new_pos = target_line_start + col.min(target_line_len);
	Some((new_pos, col))
}

fn normalized_path_key(path: &Path) -> String {
	let normalized = dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
	let value = normalized.to_string_lossy().to_string();
	#[cfg(target_os = "windows")]
	{
		value.to_ascii_lowercase()
	}
	#[cfg(not(target_os = "windows"))]
	{
		value
	}
}

fn prompt_for_password(parent: &dyn WxWidget) -> Option<String> {
	let dialog = TextEntryDialog::builder(parent, &t("&Password:"), &t("Document Password")).password().build();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	dialog.get_value().filter(|value| !value.trim().is_empty())
}

fn show_error_dialog(parent: &dyn WxWidget, message: &str, title: &str) {
	let dialog = MessageDialog::builder(parent, message, title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
	dialog.show_modal();
}

fn build_document_load_error_message(path: &Path, error: &str) -> String {
	let details = error.trim().strip_prefix(PASSWORD_REQUIRED_ERROR_PREFIX).map_or_else(|| error.trim(), str::trim);
	if details.is_empty() {
		return t("Failed to load document.");
	}
	format!("{}\n\nFile: {}\nDetails: {}", t("Failed to load document."), path.display(), details)
}

fn fill_text_ctrl(text_ctrl: TextCtrl, content: &str) {
	text_ctrl.set_value(content);
}

/// Sets `content` on `text_ctrl` and applies its bold/italic/underline markers.
///
/// On Windows this streams a single RTF blob into the native RichEdit control
/// via `EM_STREAMIN` (see `stream_rtf_into_ctrl`) instead of issuing one
/// `SetStyle` call per formatting span, which is far cheaper on documents with
/// thousands of spans. `wxTextCtrl::SetValue` can't be used for this — it does
/// not forward to the native `WM_SETTEXT` handler that auto-detects a `{\rtf`
/// prefix, so it would just store the markup as literal text. If streaming
/// doesn't round-trip back to the original content, this falls back to the
/// plain-text + per-segment path used on every other platform.
fn fill_text_ctrl_with_formatting(text_ctrl: TextCtrl, session: &DocumentSession, content: &str) {
	let markers = session.get_formatting_markers();
	let segments = merge_formatting_markers(&markers);

	#[cfg(target_os = "windows")]
	if !segments.is_empty() {
		if let Some(font) = text_ctrl.get_font() {
			let rtf = rtf_write::build_rtf(
				content,
				&segments,
				&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
			);
			if stream_rtf_into_ctrl(text_ctrl, &rtf) {
				let round_tripped = text_ctrl.get_value();
				// RichEdit's document model implicitly terminates the buffer, so a
				// wholly-trailing "\par" (with no content after it) doesn't manifest
				// as a stored character. Tolerate exactly that one known, harmless
				// discrepancy rather than falling back over it: the very last
				// position of the document ends up one short of `content`, which
				// only matters at the literal last character of the book.
				let matched = round_tripped == content
					|| (content.ends_with('\n')
						&& round_tripped.len() + 1 == content.len()
						&& content.starts_with(round_tripped.as_str()));
				if matched {
					return;
				}
			}
			// Never leave raw RTF markup on screen for an accessibility user;
			// fall back below to the plain-text + segment-loop path.
			tracing::warn!("RTF fast path for formatting markers did not round-trip; falling back");
		}
	}

	fill_text_ctrl(text_ctrl, content);
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &segments);
}

#[cfg(target_os = "windows")]
struct RtfStreamCursor<'a> {
	data: &'a [u8],
	pos: usize,
}

/// `EDITSTREAMCALLBACK` for `EM_STREAMIN`: RichEdit calls this repeatedly,
/// asking for up to `cb` bytes each time, until we report 0 bytes written
/// (end of stream) or return a nonzero error code. Called synchronously
/// within `SendMessageW` on the same thread, so the `RtfStreamCursor` borrow
/// in `stream_rtf_into_ctrl` stays valid for every call.
#[cfg(target_os = "windows")]
unsafe extern "system" fn rtf_stream_read_callback(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32 {
	if pbbuff.is_null() || pcb.is_null() || dwcookie == 0 {
		return 1;
	}
	let cursor = unsafe { &mut *(dwcookie as *mut RtfStreamCursor<'_>) };
	let remaining = cursor.data.len() - cursor.pos;
	let to_copy = remaining.min(usize::try_from(cb.max(0)).unwrap_or(0));
	if to_copy > 0 {
		unsafe { std::ptr::copy_nonoverlapping(cursor.data[cursor.pos..].as_ptr(), pbbuff, to_copy) };
		cursor.pos += to_copy;
	}
	unsafe { *pcb = i32::try_from(to_copy).unwrap_or(i32::MAX) };
	0
}

/// Feeds `rtf` into the native RichEdit control behind `text_ctrl` via the
/// Win32 `EM_STREAMIN` message. `wxTextCtrl::SetValue` cannot be used for this:
/// it does not forward to the native `WM_SETTEXT` handler that auto-detects a
/// `{\rtf` prefix, so it just stores the markup as literal text (confirmed by
/// a round-trip mismatch where `GetValue()` returned the raw RTF source
/// unchanged). `EM_STREAMIN` is the documented, explicit way to load RTF into
/// a RichEdit control, and is why this needs a raw `SendMessageW` call rather
/// than a wx-level API — the same pattern already used for letter-spacing
/// (`EM_SETCHARFORMAT`) in `apply_readability_format_to_ctrl`.
///
/// Returns `false` if the control has no native handle yet or the stream
/// didn't fully complete, in which case callers should fall back to the
/// plain-text + segment-loop path rather than trust partial content.
#[cfg(target_os = "windows")]
fn stream_rtf_into_ctrl(text_ctrl: TextCtrl, rtf: &str) -> bool {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{EDITSTREAM, EM_STREAMIN, SF_RTF},
			WindowsAndMessaging::SendMessageW,
		},
	};

	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return false;
	}
	let hwnd = HWND(hwnd_ptr);
	let mut cursor = RtfStreamCursor { data: rtf.as_bytes(), pos: 0 };
	let mut stream = EDITSTREAM {
		dwCookie: std::ptr::addr_of_mut!(cursor) as usize,
		dwError: 0,
		pfnCallback: Some(rtf_stream_read_callback),
	};
	unsafe {
		SendMessageW(
			hwnd,
			EM_STREAMIN,
			Some(WPARAM(SF_RTF as usize)),
			Some(LPARAM(std::ptr::addr_of_mut!(stream) as isize)),
		);
	}
	stream.dwError == 0 && cursor.pos == cursor.data.len()
}

pub fn apply_line_spacing_to_ctrl(text_ctrl: TextCtrl, line_spacing: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_line_spacing(match line_spacing {
		1 => 15,
		2 => 20,
		_ => 10,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

pub fn build_font_from_readability(rf: &ReadabilityFont) -> Option<Font> {
	if rf.is_default() {
		return None;
	}
	let point_size = if rf.point_size > 0 { rf.point_size } else { 10 };
	let mut font = Font::new_with_details(
		point_size,
		FontFamily::Default.as_i32(),
		rf.style,
		rf.weight,
		rf.underlined,
		&rf.face_name,
	)?;
	if rf.strikethrough {
		font.set_strikethrough(true);
	}
	if rf.encoding != 0 {
		font.set_encoding(rf.encoding);
	}
	Some(font)
}

pub fn apply_foreground_color_to_ctrl(text_ctrl: TextCtrl, color: i32) {
	if color >= 0 {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		text_ctrl.set_foreground_color(Colour::rgb(r, g, b));
	}
}

pub fn apply_bg_color_to_ctrl(text_ctrl: TextCtrl, color: i32) {
	if color >= 0 {
		let r = ((color >> 16) & 0xFF) as u8;
		let g = ((color >> 8) & 0xFF) as u8;
		let b = (color & 0xFF) as u8;
		text_ctrl.set_background_color(Colour::rgb(r, g, b));
	}
}

pub fn apply_text_alignment_to_ctrl(text_ctrl: TextCtrl, alignment: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_alignment(match alignment {
		1 => 2,
		2 => 3,
		3 => 4,
		_ => 1,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

#[cfg(target_os = "windows")]
pub fn apply_letter_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CFM_SPACING, CHARFORMAT2W},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_GETSEL: u32 = 176;
	const EM_SETSEL: u32 = 177;
	const EM_SETCHARFORMAT: u32 = 1092;
	const SCF_ALL: u32 = 4;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	// spacing_twips: 0=normal, 1=20 twips (~1pt extra), 2=40 twips (~2pt extra)
	let spacing_twips: i16 = match spacing {
		1 => 20,
		2 => 40,
		_ => 0,
	};
	unsafe {
		let mut caret: u32 = 0;
		SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(std::ptr::addr_of_mut!(caret) as usize)), None);
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut cf = CHARFORMAT2W::default();
		cf.Base.cbSize = std::mem::size_of::<CHARFORMAT2W>() as u32;
		cf.Base.dwMask = CFM_SPACING;
		cf.sSpacing = spacing_twips;
		SendMessageW(hwnd, EM_SETCHARFORMAT, Some(WPARAM(SCF_ALL as usize)), Some(LPARAM(&raw const cf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(caret as usize)), Some(LPARAM(caret as isize)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_letter_spacing_to_ctrl(_text_ctrl: TextCtrl, _spacing: i32) {}

pub fn apply_paragraph_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	attr.set_paragraph_spacing_after(match spacing {
		1 => 120,
		2 => 240,
		_ => 0,
	});
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
}

pub fn apply_readability_format_to_ctrl(
	text_ctrl: TextCtrl,
	line_spacing: i32,
	para_spacing: i32,
	letter_spacing: i32,
	alignment: i32,
) {
	if line_spacing == 0 && para_spacing == 0 && letter_spacing == 0 && alignment == 0 {
		return;
	}
	#[cfg(not(target_os = "windows"))]
	let _ = letter_spacing;
	#[cfg(target_os = "windows")]
	let windows_data = {
		use windows::Win32::{
			Foundation::{HWND, LPARAM, WPARAM},
			UI::WindowsAndMessaging::SendMessageW,
		};
		const EM_GETSEL: u32 = 176;
		const EM_SETSEL: u32 = 177;
		const WM_SETREDRAW: u32 = 11;
		let hwnd_ptr = text_ctrl.get_handle();
		if hwnd_ptr.is_null() {
			None
		} else {
			let hwnd = HWND(hwnd_ptr);
			let mut caret: u32 = 0;
			unsafe {
				SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(std::ptr::addr_of_mut!(caret) as usize)), None);
				SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(0)), None);
				SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
			}
			Some((hwnd, caret))
		}
	};
	let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
	if line_spacing > 0 {
		attr.set_line_spacing(match line_spacing {
			1 => 15,
			2 => 20,
			_ => 10,
		});
	}
	if para_spacing > 0 {
		attr.set_paragraph_spacing_after(match para_spacing {
			1 => 120,
			2 => 240,
			_ => 0,
		});
	}
	if alignment > 0 {
		attr.set_alignment(match alignment {
			1 => 2,
			2 => 3,
			3 => 4,
			_ => 1,
		});
	}
	text_ctrl.set_style(0, text_ctrl.get_last_position(), &attr);
	#[cfg(target_os = "windows")]
	if let Some((hwnd, caret)) = windows_data {
		unsafe {
			use windows::Win32::{
				Foundation::{LPARAM, RECT, WPARAM},
				Graphics::Gdi::InvalidateRect,
				UI::{
					Controls::RichEdit::{CFM_SPACING, CHARFORMAT2W},
					WindowsAndMessaging::SendMessageW,
				},
			};
			const EM_SETSEL: u32 = 177;
			const EM_SETCHARFORMAT: u32 = 1092;
			const SCF_ALL: u32 = 4;
			const WM_SETREDRAW: u32 = 11;
			if letter_spacing != 0 {
				let spacing_twips: i16 = match letter_spacing {
					1 => 20,
					2 => 40,
					_ => 0,
				};
				let mut cf = CHARFORMAT2W::default();
				cf.Base.cbSize = std::mem::size_of::<CHARFORMAT2W>() as u32;
				cf.Base.dwMask = CFM_SPACING;
				cf.sSpacing = spacing_twips;
				SendMessageW(
					hwnd,
					EM_SETCHARFORMAT,
					Some(WPARAM(SCF_ALL as usize)),
					Some(LPARAM(&raw const cf as isize)),
				);
			}
			SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(caret as usize)), Some(LPARAM(caret as isize)));
			SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(1)), None);
			let _ = InvalidateRect(Some(hwnd), None::<*const RECT>, true);
		}
	}
}

/// A non-overlapping run of text with the union of bold/italic/underline
/// styles active over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FormatSegment {
	pub start: i64,
	pub end: i64,
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
}

/// Merges bold/italic/underline markers (which may overlap, e.g. a bold word
/// inside an italic sentence) into a sequence of non-overlapping segments, each
/// carrying the union of the styles active over that range.
///
/// This is required because wxMSW's `wxTextCtrl::SetStyle` rewrites the *entire*
/// font for a range whenever any font attribute is present in the `wxTextAttr`
/// (it masks `CFM_FACE | CFM_SIZE | ...` unconditionally and fills unset fields
/// from a default font — Arial 10pt). Applying overlapping single-style markers
/// one at a time would therefore both reset the face/size and clobber each
/// other's styles. Producing one combined style per non-overlapping segment
/// avoids both problems and is correct on every platform.
///
/// Implemented as a sweep over +1/-1 events per style so it's O(n log n) instead
/// of the naive O(n^2) "rescan every marker at every boundary" approach, which
/// took several seconds on books with tens of thousands of formatting spans.
pub fn merge_formatting_markers(markers: &[paperback_core::session::LineMarker]) -> Vec<FormatSegment> {
	use paperback_core::session::MarkerTypeFfi;

	#[derive(Clone, Copy)]
	struct Event {
		position: i64,
		delta: i32,
		style_idx: usize,
	}

	let mut events: Vec<Event> = Vec::new();
	for m in markers {
		if m.length <= 0 {
			continue;
		}
		let style_idx = match m.mtype {
			MarkerTypeFfi::Bold => 0,
			MarkerTypeFfi::Italic => 1,
			MarkerTypeFfi::Underline => 2,
			_ => continue,
		};
		events.push(Event { position: m.position, delta: 1, style_idx });
		events.push(Event { position: m.position + m.length, delta: -1, style_idx });
	}
	events.sort_unstable_by_key(|e| e.position);

	let mut active = [0i32; 3];
	let mut segments: Vec<FormatSegment> = Vec::new();
	// The segment currently being extended, if the active style set is non-empty.
	let mut open: Option<FormatSegment> = None;
	let mut idx = 0;
	while idx < events.len() {
		let position = events[idx].position;
		while idx < events.len() && events[idx].position == position {
			active[events[idx].style_idx] += events[idx].delta;
			idx += 1;
		}
		let (bold, italic, underline) = (active[0] > 0, active[1] > 0, active[2] > 0);
		let same_style = open.is_some_and(|seg| seg.bold == bold && seg.italic == italic && seg.underline == underline);
		if same_style {
			// Style unchanged across this boundary: keep extending the open segment
			// instead of splitting it into an adjacent duplicate.
			open.as_mut().expect("same_style implies open is Some").end = position;
		} else {
			if let Some(mut seg) = open.take() {
				seg.end = position;
				if seg.bold || seg.italic || seg.underline {
					segments.push(seg);
				}
			}
			if bold || italic || underline {
				open = Some(FormatSegment { start: position, end: position, bold, italic, underline });
			}
		}
	}
	if let Some(seg) = open {
		if seg.bold || seg.italic || seg.underline {
			segments.push(seg);
		}
	}
	segments
}

fn apply_formatting_markers_to_ctrl_from_segments(text_ctrl: TextCtrl, segments: &[FormatSegment]) {
	if segments.is_empty() {
		return;
	}
	let base_font = text_ctrl.get_font();
	text_ctrl.freeze();
	for seg in segments {
		let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
		if let Some(base) = &base_font {
			let style = if seg.italic { FontStyle::Italic } else { base.get_style() };
			let weight = if seg.bold { FontWeight::Bold } else { base.get_weight() };
			let underlined = seg.underline || base.is_underlined();
			if let Some(mut font) = Font::new_with_details(
				base.get_point_size(),
				base.get_family().as_i32(),
				style.as_i32(),
				weight.as_i32(),
				underlined,
				&base.get_face_name(),
			) {
				if base.is_strikethrough() {
					font.set_strikethrough(true);
				}
				let encoding = base.get_encoding();
				if encoding != 0 {
					font.set_encoding(encoding);
				}
				attr.set_font(&font);
			}
		} else {
			// No base font to preserve; fall back to per-attribute flags.
			if seg.bold {
				attr.set_font_weight(FontWeight::Bold);
			}
			if seg.italic {
				attr.set_font_style(FontStyle::Italic);
			}
			if seg.underline {
				attr.set_font_underlined(true);
			}
		}
		text_ctrl.set_style(seg.start, seg.end, &attr);
	}
	text_ctrl.thaw();
}

fn show_reader_context_menu(text_ctrl: TextCtrl) {
	text_ctrl.set_focus();
	let mut menu = Menu::builder()
		.append_item(menu_ids::TOGGLE_BOOKMARK, &t("Create &bookmark"), &t("Create bookmark"))
		.append_item(menu_ids::BOOKMARK_WITH_NOTE, &t("Bookmark with &note"), &t("Create bookmark with note"))
		.append_separator()
		.append_item(menu_ids::FIND, &t("&Find"), &t("Find text"))
		.append_item(menu_ids::FIND_NEXT, &t("Find &next"), &t("Find next match"))
		.append_item(menu_ids::FIND_PREVIOUS, &t("Find &previous"), &t("Find previous match"))
		.append_separator()
		.append_item(menu_ids::GO_TO_PAGE, &t("Go to &page"), &t("Go to page"))
		.append_item(menu_ids::GO_TO_LINE, &t("Go to &line"), &t("Go to line"))
		.append_item(menu_ids::GO_TO_PERCENT, &t("Go to &percent"), &t("Go to percent"))
		.build();
	text_ctrl.popup_menu(&mut menu, None);
}

/// Build a map from (key_code, shift) to menu ID for single-key navigation shortcuts.
/// Parses shortcut strings from menu entry labels to stay in sync with menu definitions.
#[cfg(target_os = "linux")]
fn build_navigation_key_map() -> HashMap<(i32, bool), i32> {
	use super::menu::{self, MenuEntry};

	let mut map = HashMap::new();
	let all_entries = [
		menu::headings_entries(),
		menu::sections_entries(),
		menu::pages_entries(),
		menu::links_entries(),
		menu::tables_entries(),
		menu::separators_entries(),
		menu::lists_entries(),
		menu::containers_entries(),
		menu::bookmarks_entries(),
	];
	for entries in &all_entries {
		for entry in entries {
			if let MenuEntry::Item(spec) = entry {
				if let Some((key, shift)) = parse_single_key_shortcut(&spec.label) {
					map.insert((key, shift), spec.id);
				}
			}
		}
	}
	map
}

/// Parse a single-key or Shift+key shortcut from a menu label like `"&Next Heading\tH"`.
/// Returns None for shortcuts involving Ctrl, Alt, or function keys.
#[cfg(target_os = "linux")]
fn parse_single_key_shortcut(label: &str) -> Option<(i32, bool)> {
	let shortcut = label.split('\t').nth(1)?;
	if shortcut.contains("Ctrl") || shortcut.contains("Alt") {
		return None;
	}
	let shift = shortcut.contains("Shift+");
	let key_name = shortcut.rsplit('+').next()?;
	if key_name.starts_with('F') && key_name.len() > 1 && key_name[1..].chars().all(|c| c.is_ascii_digit()) {
		return None;
	}
	if key_name.len() == 1 {
		let key = key_name.as_bytes()[0].to_ascii_uppercase() as i32;
		return Some((key, shift));
	}
	None
}

#[cfg(test)]
mod tests {
	use paperback_core::session::{LineMarker, MarkerTypeFfi};

	use super::{FormatSegment, merge_formatting_markers};

	fn marker(mtype: MarkerTypeFfi, position: i64, length: i64) -> LineMarker {
		LineMarker { mtype, position, text: String::new(), reference: String::new(), level: 0, length }
	}

	#[test]
	fn no_markers_yields_no_segments() {
		assert_eq!(merge_formatting_markers(&[]), Vec::new());
	}

	#[test]
	fn zero_length_markers_are_ignored() {
		let markers = [marker(MarkerTypeFfi::Bold, 5, 0)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn non_format_markers_are_ignored() {
		let markers = [marker(MarkerTypeFfi::Heading1, 0, 10), marker(MarkerTypeFfi::Link, 2, 3)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn single_bold_marker_produces_one_segment() {
		let markers = [marker(MarkerTypeFfi::Bold, 0, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 4, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn overlapping_bold_and_italic_keep_both_on_the_intersection() {
		// Bold over [0,10), italic over [4,7): the middle run must carry both.
		let markers = [marker(MarkerTypeFfi::Bold, 0, 10), marker(MarkerTypeFfi::Italic, 4, 3)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![
				FormatSegment { start: 0, end: 4, bold: true, italic: false, underline: false },
				FormatSegment { start: 4, end: 7, bold: true, italic: true, underline: false },
				FormatSegment { start: 7, end: 10, bold: true, italic: false, underline: false },
			]
		);
	}

	#[test]
	fn adjacent_identical_segments_are_coalesced() {
		let markers = [marker(MarkerTypeFfi::Bold, 0, 4), marker(MarkerTypeFfi::Bold, 4, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 8, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn all_three_styles_can_stack() {
		let markers = [
			marker(MarkerTypeFfi::Bold, 0, 6),
			marker(MarkerTypeFfi::Italic, 0, 6),
			marker(MarkerTypeFfi::Underline, 0, 6),
		];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 6, bold: true, italic: true, underline: true }]
		);
	}
}
