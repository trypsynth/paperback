//! Opening and closing documents: turning a path into a parsed `DocumentSession` and a tab (with
//! its password prompt, settings import, and format/password lookups), building the tab's text
//! control, closing tabs, and saving reading positions. Split out of the main `DocumentManager`
//! impl.

use std::{cell::Cell, path::Path, rc::Rc, sync::Mutex, time::Instant};

use paperback_core::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession};
use patois::t;
use wxdragon::prelude::*;

use super::{
	DocumentManager, DocumentTab, POSITION_SAVE_INTERVAL_SECS, build_document_load_error_message, parse_settings,
	prompt_for_password, read_fingerprint, show_error_dialog, title_or_filename,
};
use crate::{
	audio_player::AudioPlayer,
	ui::{
		readability::{
			apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_readability_format_to_ctrl,
			build_font_from_readability,
		},
		reader_input, shell,
		text_render::load_window_into_ctrl,
	},
};

impl DocumentManager {
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
			// TRANSLATORS: Error message shown when the requested document file does not exist; {} is the file path
			let template = t("File not found: {}");
			let message = template.replace("{}", &path.to_string_lossy());
			// TRANSLATORS: Generic error dialog title
			show_error_dialog(&self.notebook, &message, &t("Error"));
			return false;
		}
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}
		let import_path = path.with_extension("paperback");
		if !is_restore && import_path.exists() {
			// TRANSLATORS: Prompt asking whether to import a document's previously saved settings and bookmarks found alongside it
			let message = t("A .paperback file was found for this document. Would you like to import it?");
			// TRANSLATORS: Title of the dialog prompting to import a document's saved settings and bookmarks
			let title = t("Import document data");
			let dialog = MessageDialog::builder(&self.notebook, &message, &title)
				.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconQuestion | MessageDialogStyle::Centre)
				.build();
			if dialog.show_modal() == ID_YES {
				let config = self.config.lock().unwrap();
				config.import_settings_from_file(&path.to_string_lossy(), import_path.to_str().unwrap());
			}
		}
		let (password, forced_extension, settings) = {
			let config = self.config.lock().unwrap();
			let path_str = path.to_string_lossy();
			config.refresh_document_hash(&path_str);
			let forced_extension = config.get_document_format(&path_str);
			let password = config.get_document_password(&path_str);
			let settings = parse_settings(&config);
			drop(config);
			(password, forced_extension, settings)
		};
		let path_str = path.to_string_lossy().to_string();
		tracing::info!(path = %path.display(), "opening document");
		match DocumentSession::new(&path_str, &password, &forced_extension, settings) {
			Ok(session) => self.add_session_tab(self_rc, path, session, &password, track, title_override),
			Err(err) => {
				if err.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					let config = self.config.lock().unwrap();
					config.set_document_password(&path_str, "");
					drop(config);
					let password = prompt_for_password(&self.notebook);
					let Some(password) = password else {
						// TRANSLATORS: Error shown when the user dismisses the password prompt for an encrypted document without entering one
						show_error_dialog(&self.notebook, &t("Password is required."), &t("Error"));
						return false;
					};
					match DocumentSession::new(&path_str, &password, &forced_extension, settings) {
						Ok(session) => self.add_session_tab(self_rc, path, session, &password, track, title_override),
						Err(retry_error) => {
							tracing::error!(path = %path.display(), error = %retry_error, "failed to open document");
							let message = build_document_load_error_message(path, &retry_error);
							// TRANSLATORS: Generic error dialog title
							show_error_dialog(&self.notebook, &message, &t("Error"));
							false
						}
					}
				} else {
					tracing::error!(path = %path.display(), error = %err, "failed to open document");
					let message = build_document_load_error_message(path, &err);
					// TRANSLATORS: Generic error dialog title
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
		let title = title_override.map_or_else(|| title_or_filename(session.title(), path), ToString::to_string);
		let panel = Panel::builder(&self.notebook).build();
		let config = self.config.lock().unwrap();
		let mut session = session;
		let word_wrap = config.get_app_bool("word_wrap", false);
		let text_ctrl = reader_input::build_text_ctrl(panel, word_wrap, self_rc, self.frame);
		let rf = config.get_readability_font();
		if let Some(font) = build_font_from_readability(&rf) {
			text_ctrl.set_font(&font);
		}
		apply_foreground_color_to_ctrl(text_ctrl, rf.color);
		apply_bg_color_to_ctrl(text_ctrl, config.get_bg_color());
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		let path_str = path.to_string_lossy();
		let doc_len = session.document_len();
		let saved_pos = config.get_validated_document_position(&path_str, doc_len);
		let initial_pos = if saved_pos >= 0 { saved_pos } else { 0 };
		let window = load_window_into_ctrl(text_ctrl, &session, initial_pos, doc_len);
		apply_readability_format_to_ctrl(
			text_ctrl,
			config.get_line_spacing(),
			config.get_paragraph_spacing(),
			config.get_letter_spacing(),
			config.get_text_alignment(),
		);
		// `add_page(select: true)` can synchronously fire the notebook's page-changed event
		// before returning (e.g. while it's still the only page, or otherwise reentering here
		// on this same thread), and that handler path can itself want `self.config` again.
		// `Mutex` isn't reentrant, so holding this lock across the call self-deadlocks the app
		// forever on documents whose formatting makes that reentrant path reachable - drop it
		// first and reacquire once `add_page` returns.
		drop(config);
		self.notebook.add_page(&panel, &title, true, None);
		let config = self.config.lock().unwrap();
		let nav_history = config.get_navigation_history(&path_str);
		session.set_history(&nav_history.positions, nav_history.index);
		let audio_player = session.audio().cloned().and_then(|timeline| match AudioPlayer::new(timeline) {
			Ok(player) => Some(player),
			Err(err) => {
				tracing::warn!(error = %err, "failed to initialize audio playback for this document");
				None
			}
		});
		self.tabs.push(DocumentTab {
			panel,
			text_ctrl,
			session,
			file_path: path.to_path_buf(),
			track,
			audio_player,
			disk_fingerprint: read_fingerprint(path),
			preferred_column: Cell::new(None),
			selection_mark: Cell::new(None),
			window,
			ocr_job: None,
		});
		if !password.is_empty() {
			config.set_document_password(&path_str, password);
		}
		let tab_index = self.tabs.len() - 1;
		{
			let tab = &self.tabs[tab_index];
			let local = tab.window.to_local(initial_pos);
			tab.text_ctrl.set_insertion_point(local);
			tab.text_ctrl.show_position(local);
		}
		// Resume the narration from the time that was actually reached, not from the caret.
		// Deriving it from the caret only lands on the start of whichever clip contains that
		// position, so it loses however much of that clip had already played, and loses
		// everything since the last explicit jump if the caret wasn't following the audio.
		let saved_audio_time = config.get_document_audio_time(&path_str);
		if let Some(player) = self.tabs[tab_index].audio_player.as_mut() {
			match saved_audio_time {
				Some(time_ms) => {
					player.seek_to_ms(time_ms);
				}
				None => {
					player.seek_to_position(usize::try_from(initial_pos).unwrap_or(0));
				}
			}
		}
		if track {
			config.add_recent_document(&path_str);
			shell::add_recent_document(path);
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
			// don't make an untracked document reopenable; reopening would give it the wrong title and make it tracked.
			if tab.track {
				self.recently_closed.push(tab.file_path.clone());
			}
			let path_str = tab.file_path.to_string_lossy();
			let config = self.config.lock().unwrap();
			if save_state && tab.track {
				let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
				config.set_document_position(&path_str, position);
				config.set_document_audio_time(
					&path_str,
					tab.audio_player.as_ref().and_then(AudioPlayer::resume_point_ms),
				);
				let (history, history_index) = tab.session.get_history();
				config.set_navigation_history(&path_str, history, history_index);
				config.set_document_opened(&path_str, false);
			}
			config.remove_opened_document(&path_str);
			config.flush();
		}
		if let Some(tab) = self.tabs.get_mut(index)
			&& let Some(player) = tab.audio_player.as_mut()
		{
			player.stop();
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
			let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let path_str = tab.file_path.to_string_lossy();
			config.set_document_position(&path_str, position);
			config.set_document_audio_time(&path_str, tab.audio_player.as_ref().and_then(AudioPlayer::resume_point_ms));
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
			let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let path_str = tab.file_path.to_string_lossy();
			let config = self.config.lock().unwrap();
			config.set_document_position(&path_str, position);
			config.set_document_audio_time(&path_str, tab.audio_player.as_ref().and_then(AudioPlayer::resume_point_ms));
			config.flush();
		}
		self.last_position_save.set(Some(now));
	}
}
