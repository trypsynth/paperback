use std::{
	cell::Cell,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{Mutex, atomic::Ordering},
	time::Instant,
};

use wxdragon::{prelude::*, translations::translate as t};

use super::{
	main_window::{SLEEP_TIMER_DURATION_MINUTES, SLEEP_TIMER_START_MS},
	status,
};
use crate::{config::ConfigManager, live_region, parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession};

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	pub file_path: PathBuf,
}

const POSITION_SAVE_INTERVAL_SECS: u64 = 3;

pub struct DocumentManager {
	frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
}

impl DocumentManager {
	pub const fn new(
		frame: Frame,
		notebook: Notebook,
		config: Rc<Mutex<ConfigManager>>,
		live_region_label: StaticText,
	) -> Self {
		Self { frame, notebook, tabs: Vec::new(), config, live_region_label, last_position_save: Cell::new(None) }
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
					if let Ok(session) = DocumentSession::new(&path_str, &password, &forced_extension) {
						self.add_session_tab(self_rc, path, session, &password)
					} else {
						show_error_dialog(&self.notebook, &t("Failed to load document."), &t("Error"));
						false
					}
				} else {
					show_error_dialog(&self.notebook, &t("Failed to load document."), &t("Error"));
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
		let style = TextCtrlStyle::MultiLine
			| TextCtrlStyle::ReadOnly
			| TextCtrlStyle::Rich2
			| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
		let text_ctrl = TextCtrl::builder(&panel).with_style(style).build();
		let dm_for_enter = Rc::clone(self_rc);
		text_ctrl.on_char(move |event| {
			if let WindowEventData::Keyboard(kbd) = event {
				if kbd.get_key_code() == Some(13) {
					// 13 is KEY_RETURN
					let mut dm = dm_for_enter.lock().unwrap();
					dm.activate_current_table();
					dm.activate_current_link();
				} else {
					kbd.event.skip(true);
				}
			}
		});
		let dm_for_key_up = Rc::clone(self_rc);
		text_ctrl.bind_internal(wxdragon::event::EventType::KEY_UP, move |event| {
			event.skip(true);
			if let Ok(dm) = dm_for_key_up.try_lock() {
				dm.update_status_bar();
				dm.save_position_throttled();
			}
		});
		let dm_for_mouse = Rc::clone(self_rc);
		text_ctrl.bind_internal(wxdragon::event::EventType::LEFT_UP, move |event| {
			event.skip(true);
			if let Ok(dm) = dm_for_mouse.try_lock() {
				dm.update_status_bar();
				dm.save_position_throttled();
			}
		});
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		let content = session.content();
		fill_text_ctrl(text_ctrl, &content);
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

	pub fn apply_word_wrap(&mut self, word_wrap: bool) {
		for tab in &mut self.tabs {
			let old_ctrl = tab.text_ctrl;
			let current_pos = old_ctrl.get_insertion_point();
			let content = old_ctrl.get_value();
			let style = TextCtrlStyle::MultiLine
				| TextCtrlStyle::ReadOnly
				| TextCtrlStyle::Rich2
				| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
			let text_ctrl = TextCtrl::builder(&tab.panel).with_style(style).build();
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
			tab.panel.set_sizer(sizer, true);
			fill_text_ctrl(text_ctrl, &content);
			let max_pos = text_ctrl.get_last_position();
			let pos = current_pos.clamp(0, max_pos);
			text_ctrl.set_insertion_point(pos);
			text_ctrl.show_position(pos);
			tab.panel.layout();
			old_ctrl.destroy();
			tab.text_ctrl = text_ctrl;
		}
	}
}

fn normalized_path_key(path: &Path) -> String {
	let normalized = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
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

fn fill_text_ctrl(text_ctrl: TextCtrl, content: &str) {
	const CHUNK_SIZE: usize = 32 * 1024;
	text_ctrl.clear();
	let mut buf = String::new();
	for ch in content.chars() {
		buf.push(ch);
		if buf.len() >= CHUNK_SIZE {
			text_ctrl.append_text(&buf);
			buf.clear();
		}
	}
	if !buf.is_empty() {
		text_ctrl.append_text(&buf);
	}
}
