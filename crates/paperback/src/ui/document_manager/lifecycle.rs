use std::{cell::Cell, path::Path, rc::Rc, sync::Mutex, time::Instant};

use paperback_core::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession};
use patois::t;
use wxdragon::prelude::*;

use super::{
	DocumentManager, DocumentTab, POSITION_SAVE_INTERVAL_SECS,
	ocr::{joins_recognized_text, restore_saved_pages, saved_pages_of},
	parse_settings, read_fingerprint, reading_path, title_or_filename,
};
use crate::{
	audio_player::AudioPlayer,
	shell,
	ui::{
		readability::{
			apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_readability_format_to_ctrl,
			build_font_from_readability,
		},
		reader_input,
		text_render::load_window_into_ctrl,
	},
	working_copy::WorkingCopy,
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

	pub fn open_source_file(&mut self, self_rc: &Rc<Mutex<Self>>, path: &Path, title: &str) -> bool {
		self.open_file_impl(self_rc, path, false, false, Some(title))
	}

	pub fn open_link(&mut self, self_rc: &Rc<Mutex<Self>>, link: &str, copy: WorkingCopy) -> bool {
		self.parse_and_add_tab(self_rc, Path::new(link), Some(copy), true, None)
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
		self.parse_and_add_tab(self_rc, path, None, track, title_override)
	}

	fn parse_and_add_tab(
		&mut self,
		self_rc: &Rc<Mutex<Self>>,
		path: &Path,
		working_copy: Option<WorkingCopy>,
		track: bool,
		title_override: Option<&str>,
	) -> bool {
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
		let parse_path = reading_path(path, working_copy.as_ref()).to_string_lossy().into_owned();
		let open = |password: &str| DocumentSession::new(&parse_path, password, &forced_extension, settings);
		tracing::info!(path = %path.display(), "opening document");
		match open(&password) {
			Ok(session) => self.add_session_tab(self_rc, path, working_copy, session, &password, track, title_override),
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
					match open(&password) {
						Ok(session) => {
							self.add_session_tab(self_rc, path, working_copy, session, &password, track, title_override)
						}
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

	fn add_session_tab(
		&mut self,
		self_rc: &Rc<Mutex<Self>>,
		path: &Path,
		working_copy: Option<WorkingCopy>,
		session: DocumentSession,
		password: &str,
		track: bool,
		title_override: Option<&str>,
	) -> bool {
		if let Some(index) = self.find_tab_by_path(path) {
			self.notebook.set_selection(index);
			return true;
		}
		let local_path = reading_path(path, working_copy.as_ref()).to_path_buf();
		let title = title_override.map_or_else(|| title_or_filename(session.title(), &local_path), ToString::to_string);
		let panel = Panel::builder(&self.notebook).build();
		let config = self.config.lock().unwrap();
		let mut session = session;
		// Before the position is read, since it was saved against the text OCR put in.
		restore_saved_pages(&mut session, &saved_pages_of(&local_path), joins_recognized_text(&local_path, &config));
		let word_wrap = config.get_app_bool("word_wrap", false);
		let text_ctrl =
			reader_input::build_text_ctrl(panel, word_wrap, self_rc, self.frame, self.from_keyboard.clone());
		let rf = config.get_readability_font();
		if let Some(font) = build_font_from_readability(&rf) {
			text_ctrl.set_font(&font);
		}
		apply_foreground_color_to_ctrl(text_ctrl.widget(), rf.color);
		apply_bg_color_to_ctrl(text_ctrl.widget(), config.get_bg_color());
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&text_ctrl.widget(), 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		let path_str = path.to_string_lossy();
		let doc_len = session.document_len();
		let initial_pos = config.get_validated_document_position(&path_str, doc_len.as_pos()).unwrap_or_default();
		let window = load_window_into_ctrl(text_ctrl, &session, initial_pos, doc_len);
		apply_readability_format_to_ctrl(
			text_ctrl.widget(),
			config.get_line_spacing(),
			config.get_paragraph_spacing(),
			config.get_letter_spacing(),
			config.get_text_alignment(),
		);
		// `add_page(select: true)` can synchronously fire page-changed, whose handler locks `self.config` again; `Mutex` isn't reentrant, so holding it here deadlocked the app.
		drop(config);
		self.notebook.add_page(&panel, &title, true, None);
		let config = self.config.lock().unwrap();
		let nav_history = config.get_navigation_history(&path_str);
		session.set_history(&nav_history.positions, nav_history.index);
		let audio_player = session.audio().cloned().and_then(|timeline| match AudioPlayer::new(timeline) {
			Ok(mut player) => {
				player.set_speed(crate::audio_player::saved_speed(&config));
				Some(player)
			}
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
			working_copy,
			track,
			audio_player,
			disk_fingerprint: read_fingerprint(&local_path),
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
		// Resume from the saved time rather than the caret, which only lands on a clip start and is stale if the caret wasn't following the audio.
		let saved_audio_time = config.get_document_audio_time(&path_str);
		if let Some(player) = self.tabs[tab_index].audio_player.as_mut() {
			match saved_audio_time {
				Some(time_ms) => {
					player.seek_to_ms(time_ms);
				}
				None => {
					player.seek_to_position(initial_pos);
				}
			}
		}
		if track {
			config.add_recent_document(&path_str);
			// The jump list opens files; a link's working copy is gone once its tab closes.
			if self.tabs[tab_index].working_copy.is_none() {
				shell::add_recent_document(path);
			}
			config.set_document_opened(&path_str, true);
			config.add_opened_document(&path_str);
			let tab = &self.tabs[tab_index];
			if let (Some(copy), Some(fingerprint)) = (&tab.working_copy, tab.disk_fingerprint) {
				config.set_document_download(&path_str, &copy.file_name(), fingerprint.len());
			}
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
			// Reopening an untracked document would give it the wrong title and make it tracked.
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
		// Before the tab goes, or the worker keeps its own clone of the cancel flag and reads on for a closed document; quiet, since the count would describe a document no longer open.
		self.stop_ocr_at(index, true);
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
		let config = self.config.lock().unwrap();
		config.clear_opened_documents();
		config.flush();
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

	/// The app exits without dropping its tabs, so link copies would otherwise stay in the temporary folder.
	pub fn remove_working_copies(&mut self) {
		for tab in &mut self.tabs {
			tab.working_copy = None;
		}
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

pub(super) fn prompt_for_password(parent: &dyn WxWidget) -> Option<String> {
	// TRANSLATORS: Label for the password entry field in the "Document Password" prompt dialog
	let dialog = TextEntryDialog::builder(parent, &t("&Password:"), &t("Document Password")).password().build();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	dialog.get_value().filter(|value| !value.trim().is_empty())
}

pub(super) fn show_error_dialog(parent: &dyn WxWidget, message: &str, title: &str) {
	let dialog = MessageDialog::builder(parent, message, title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
	dialog.show_modal();
}

pub(super) fn build_document_load_error_message(path: &Path, error: &str) -> String {
	let details = error.trim().strip_prefix(PASSWORD_REQUIRED_ERROR_PREFIX).map_or_else(|| error.trim(), str::trim);
	if details.is_empty() {
		// TRANSLATORS: Generic error message shown when a document fails to load with no further detail available
		return t("Failed to load document.");
	}
	// TRANSLATORS: "File" label prefix in the document-load error dialog; {} is the file path
	let file_line = t("File: {}").replace("{}", &path.display().to_string());
	// TRANSLATORS: "Details" label prefix in the document-load error dialog; {} is the underlying error message
	let details_line = t("Details: {}").replace("{}", details);
	// TRANSLATORS: Generic error message shown when a document fails to load, followed by file and detail lines
	format!("{}\n\n{file_line}\n{details_line}", t("Failed to load document."))
}
