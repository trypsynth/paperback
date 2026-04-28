#[cfg(target_os = "linux")]
use std::collections::HashMap;
use std::{
	cell::Cell,
	fs,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{Mutex, atomic::Ordering},
	time::Instant,
};

use wxdragon::{
	color::Colour,
	event::{EventType, WindowEventData},
	prelude::*,
	translations::translate as t,
};

use super::{
	main_window::{SLEEP_TIMER_DURATION_MINUTES, SLEEP_TIMER_START_MS},
	menu_ids, status,
};
use crate::{
	config::{ConfigManager, ReadabilityFont},
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	session::DocumentSession,
};

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	pub file_path: PathBuf,
}

const POSITION_SAVE_INTERVAL_SECS: u64 = 3;
const WXK_F10: i32 = 349;
const WXK_WINDOWS_MENU: i32 = 395;

pub struct DocumentManager {
	frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
	last_sound_position: Cell<Option<i64>>,
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
			recently_closed: Vec::new(),
			#[cfg(target_os = "linux")]
			navigation_key_map: Rc::new(build_navigation_key_map()),
		}
	}

	pub fn open_file(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path) -> bool {
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
		let (password, forced_extension) = {
			let config = self.config.lock().unwrap();
			let path_str = path.to_string_lossy();
			config.refresh_document_hash(&path_str);
			config.import_document_settings(&path_str);
			let forced_extension = config.get_document_format(&path_str);
			let password = config.get_document_password(&path_str);
			drop(config);
			(password, forced_extension)
		};
		let path_str = path.to_string_lossy().to_string();
		match DocumentSession::new(&path_str, &password, &forced_extension) {
			Ok(session) => self.add_session_tab(self_rc, path, session, &password),
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
					match DocumentSession::new(&path_str, &password, &forced_extension) {
						Ok(session) => self.add_session_tab(self_rc, path, session, &password),
						Err(retry_error) => {
							let message = build_document_load_error_message(path, &retry_error);
							show_error_dialog(&self.notebook, &message, &t("Error"));
							false
						}
					}
				} else {
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
	) -> bool {
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}
		let title = session.title();
		let title = if title.is_empty() {
			path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
		} else {
			title
		};
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
		fill_text_ctrl(text_ctrl, &content);
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
		self.tabs.push(DocumentTab { panel, text_ctrl, session, file_path: path.to_path_buf() });
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
		config.add_recent_document(&path_str);
		config.set_document_opened(&path_str, true);
		config.add_opened_document(&path_str);
		config.flush();
		true
	}

	pub fn close_document(&mut self, index: usize) -> bool {
		if index >= self.tabs.len() {
			return false;
		}
		if let Some(tab) = self.tabs.get(index) {
			self.recently_closed.push(tab.file_path.clone());
			let position = tab.text_ctrl.get_insertion_point();
			let path_str = tab.file_path.to_string_lossy();
			let config = self.config.lock().unwrap();
			config.set_document_position(&path_str, position);
			let (history, history_index) = tab.session.get_history();
			config.set_navigation_history(&path_str, history, history_index);
			config.set_document_opened(&path_str, false);
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

	pub fn close_all_documents(&mut self) {
		while !self.tabs.is_empty() {
			self.close_document(0);
		}
	}

	pub fn save_all_positions(&self) {
		let config = self.config.lock().unwrap();
		for tab in &self.tabs {
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
		if let Some(last_save) = self.last_position_save.get() {
			if now.duration_since(last_save).as_secs() < POSITION_SAVE_INTERVAL_SECS {
				return;
			}
		}
		if let Some(tab) = self.active_tab() {
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
					crate::session::LinkAction::Internal => {
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(result.offset);
						tab.text_ctrl.show_position(result.offset);
						tab.session.check_and_record_history(result.offset);
						live_region::announce(self.live_region_label, &t("Navigated to internal link."));
					}
					crate::session::LinkAction::External => {
						wxdragon::utils::launch_default_browser(
							&result.url,
							wxdragon::utils::BrowserLaunchFlags::Default,
						);
					}
					crate::session::LinkAction::NotFound => {}
				}
			}
		}
	}

	pub fn activate_current_table(&self) {
		let table_html = self.active_tab().and_then(|tab| {
			let pos = tab.text_ctrl.get_insertion_point();
			tab.session.get_table_at_position(pos)
		});
		if let Some(html) = table_html {
			super::dialogs::show_web_view_dialog(&self.frame, &t("Table View"), &html, false, None);
		}
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
				if !bm.note.is_empty() {
					has_note = true;
				} else {
					has_bookmark = true;
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
			fill_text_ctrl(text_ctrl, &content);
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
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			tab.panel.layout();
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
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
					let mut dm = dm_for_enter.lock().unwrap();
					dm.activate_current_table();
					dm.activate_current_link();
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
				dm.update_status_bar();
				dm.save_position_throttled();
				dm.check_bookmark_sounds();
			}
		});
		let text_ctrl_for_menu = text_ctrl;
		#[cfg(target_os = "linux")]
		let key_map = navigation_key_map;
		#[cfg(target_os = "linux")]
		let frame_for_keys = frame;
		text_ctrl.on_key_down(move |event| {
			if let WindowEventData::Keyboard(kbd) = &event {
				if let Some(key) = kbd.get_key_code() {
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

fn normalized_path_key(path: &Path) -> String {
	let normalized = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
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

#[cfg(target_os = "windows")]
pub fn apply_line_spacing_to_ctrl(text_ctrl: TextCtrl, line_spacing: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{PARAFORMAT2, PFM_LINESPACING},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_SETSEL: u32 = 177;
	const EM_SETPARAFORMAT: u32 = 1095;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	unsafe {
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut pf = PARAFORMAT2::default();
		pf.Base.cbSize = std::mem::size_of::<PARAFORMAT2>() as u32;
		pf.Base.dwMask = PFM_LINESPACING;
		pf.bLineSpacingRule = line_spacing.clamp(0, 2) as u8;
		SendMessageW(hwnd, EM_SETPARAFORMAT, None, Some(LPARAM(&raw const pf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(0)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_line_spacing_to_ctrl(_text_ctrl: TextCtrl, _line_spacing: i32) {}

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

#[cfg(target_os = "windows")]
pub fn apply_text_alignment_to_ctrl(text_ctrl: TextCtrl, alignment: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{PARAFORMAT2, PFA_CENTER, PFA_JUSTIFY, PFA_LEFT, PFA_RIGHT, PFM_ALIGNMENT},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_SETSEL: u32 = 177;
	const EM_SETPARAFORMAT: u32 = 1095;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	let pfa = match alignment {
		1 => PFA_CENTER,
		2 => PFA_RIGHT,
		3 => PFA_JUSTIFY,
		_ => PFA_LEFT,
	};
	unsafe {
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut pf = PARAFORMAT2::default();
		pf.Base.cbSize = std::mem::size_of::<PARAFORMAT2>() as u32;
		pf.Base.dwMask = PFM_ALIGNMENT;
		pf.Base.wAlignment = pfa;
		SendMessageW(hwnd, EM_SETPARAFORMAT, None, Some(LPARAM(&raw const pf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(0)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_text_alignment_to_ctrl(_text_ctrl: TextCtrl, _alignment: i32) {}

#[cfg(target_os = "windows")]
pub fn apply_letter_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CFM_SPACING, CHARFORMAT2W},
			WindowsAndMessaging::SendMessageW,
		},
	};
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
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut cf = std::mem::zeroed::<CHARFORMAT2W>();
		cf.Base.cbSize = std::mem::size_of::<CHARFORMAT2W>() as u32;
		cf.Base.dwMask = CFM_SPACING;
		cf.sSpacing = spacing_twips;
		SendMessageW(hwnd, EM_SETCHARFORMAT, Some(WPARAM(SCF_ALL as usize)), Some(LPARAM(&raw const cf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(0)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_letter_spacing_to_ctrl(_text_ctrl: TextCtrl, _spacing: i32) {}

#[cfg(target_os = "windows")]
pub fn apply_paragraph_spacing_to_ctrl(text_ctrl: TextCtrl, spacing: i32) {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{PARAFORMAT2, PFM_SPACEAFTER},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_SETSEL: u32 = 177;
	const EM_SETPARAFORMAT: u32 = 1095;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	// spacing in twips: 0=none, 1=120 twips (~6pt), 2=240 twips (~12pt)
	let space_after: i32 = match spacing {
		1 => 120,
		2 => 240,
		_ => 0,
	};
	unsafe {
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut pf = PARAFORMAT2::default();
		pf.Base.cbSize = std::mem::size_of::<PARAFORMAT2>() as u32;
		pf.Base.dwMask = PFM_SPACEAFTER;
		pf.dySpaceAfter = space_after;
		SendMessageW(hwnd, EM_SETPARAFORMAT, None, Some(LPARAM(&raw const pf as isize)));
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(0)));
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_paragraph_spacing_to_ctrl(_text_ctrl: TextCtrl, _spacing: i32) {}

/// Applies all paragraph/character readability formats in one batched operation.
/// Returns immediately with no Windows messages when all values are at their defaults (all 0).
/// For non-default values, suppresses redraws across both format passes so the control
/// only repaints once at the end.
#[cfg(target_os = "windows")]
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
	use windows::Win32::{
		Foundation::{HWND, LPARAM, RECT, WPARAM},
		Graphics::Gdi::InvalidateRect,
		UI::{
			Controls::RichEdit::{
				CFM_SPACING, CHARFORMAT2W, PARAFORMAT2, PFA_CENTER, PFA_JUSTIFY, PFA_LEFT, PFA_RIGHT, PFM_ALIGNMENT,
				PFM_LINESPACING, PFM_SPACEAFTER,
			},
			WindowsAndMessaging::SendMessageW,
		},
	};
	const EM_SETSEL: u32 = 177;
	const EM_SETPARAFORMAT: u32 = 1095;
	const EM_SETCHARFORMAT: u32 = 1092;
	const SCF_ALL: u32 = 4;
	const WM_SETREDRAW: u32 = 11;
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return;
	}
	let hwnd = HWND(hwnd_ptr);
	unsafe {
		SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(0)), None);
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));

		// Combine line spacing + paragraph spacing + alignment into one EM_SETPARAFORMAT
		let mut pf = PARAFORMAT2::default();
		pf.Base.cbSize = std::mem::size_of::<PARAFORMAT2>() as u32;
		pf.Base.dwMask = PFM_LINESPACING | PFM_SPACEAFTER | PFM_ALIGNMENT;
		pf.bLineSpacingRule = line_spacing.clamp(0, 2) as u8;
		pf.dySpaceAfter = match para_spacing {
			1 => 120,
			2 => 240,
			_ => 0,
		};
		pf.Base.wAlignment = match alignment {
			1 => PFA_CENTER,
			2 => PFA_RIGHT,
			3 => PFA_JUSTIFY,
			_ => PFA_LEFT,
		};
		SendMessageW(hwnd, EM_SETPARAFORMAT, None, Some(LPARAM(&raw const pf as isize)));

		if letter_spacing != 0 {
			let spacing_twips: i16 = match letter_spacing {
				1 => 20,
				2 => 40,
				_ => 0,
			};
			let mut cf = std::mem::zeroed::<CHARFORMAT2W>();
			cf.Base.cbSize = std::mem::size_of::<CHARFORMAT2W>() as u32;
			cf.Base.dwMask = CFM_SPACING;
			cf.sSpacing = spacing_twips;
			SendMessageW(hwnd, EM_SETCHARFORMAT, Some(WPARAM(SCF_ALL as usize)), Some(LPARAM(&raw const cf as isize)));
		}

		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(0)));
		SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(1)), None);
		let _ = InvalidateRect(Some(hwnd), None::<*const RECT>, true);
	}
}

#[cfg(not(target_os = "windows"))]
pub fn apply_readability_format_to_ctrl(
	_text_ctrl: TextCtrl,
	_line_spacing: i32,
	_para_spacing: i32,
	_letter_spacing: i32,
	_alignment: i32,
) {
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
