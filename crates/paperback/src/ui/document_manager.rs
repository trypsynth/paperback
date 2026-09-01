#[cfg(target_os = "windows")]
use std::ptr::addr_of_mut;
use std::{
	cell::Cell,
	fs,
	path::{Path, PathBuf},
	rc::Rc,
	sync::Mutex,
	time::{Instant, SystemTime},
};

use paperback_core::{
	config::{ActionId, ConfigManager, ReadabilityFont},
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	session::{DocumentSession, WindowSlice},
};
use patois::t;
use wxdragon::{
	clipboard::Clipboard,
	color::Colour,
	event::{EventType, WindowEventData},
	prelude::*,
};

#[cfg(target_os = "windows")]
use super::rtf::{
	stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
	write::{self, RtfFontInfo},
};
use super::{
	menu_ids,
	navigation::{self, move_to_offset_and_record_history, persist_navigation_history},
	shell, sleep_timer, status,
	text_window::{self, TextWindow},
};
use crate::audio_player::AudioPlayer;

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	pub file_path: PathBuf,
	pub track: bool,
	pub audio_player: Option<AudioPlayer>,
	disk_fingerprint: Option<FileFingerprint>,
	/// The document-absolute bounds of whatever's currently loaded into `text_ctrl`. See
	/// `ui::text_window` - for most documents this covers the whole thing, same as before
	/// windowing existed; only huge documents actually get a partial window.
	pub window: TextWindow,
}

/// Change-detection stamp for an open document's file, compared on every frame activation and
/// tab switch. Metadata-only on purpose: `config::compute_document_hash` would read up to 2 MiB
/// from disk per check and still miss mid-file edits in files larger than that (it hashes only
/// head, tail and size), whereas a single `fs::metadata` call catches any completed write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileFingerprint {
	modified: SystemTime,
	len: u64,
}

fn read_fingerprint(path: &Path) -> Option<FileFingerprint> {
	let meta = fs::metadata(path).ok()?;
	Some(FileFingerprint { modified: meta.modified().ok()?, len: meta.len() })
}

pub fn title_or_filename(title: String, path: &Path) -> String {
	if title.is_empty() {
		// TRANSLATORS: Fallback document title shown in tabs and lists when a document has no title and its file name cannot be determined
		path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
	} else {
		title
	}
}

pub fn display_title(tab: &DocumentTab) -> String {
	title_or_filename(tab.session.title(), &tab.file_path)
}

const POSITION_SAVE_INTERVAL_SECS: u64 = 3;

pub struct DocumentManager {
	frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
	last_sound_position: Cell<Option<i64>>,
	last_audio_seek_position: Cell<Option<i64>>,
	preferred_column: Cell<Option<i64>>,
	last_focus_in_text: Cell<bool>,
	recently_closed: Vec<PathBuf>,
}

impl DocumentManager {
	pub const fn new(
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
			last_audio_seek_position: Cell::new(None),
			preferred_column: Cell::new(None),
			last_focus_in_text: Cell::new(true),
			recently_closed: Vec::new(),
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
						// TRANSLATORS: Error shown when the user dismisses the password prompt for an encrypted document without entering one
						show_error_dialog(&self.notebook, &t("Password is required."), &t("Error"));
						return false;
					};
					match DocumentSession::new(&path_str, &password, &forced_extension, render_tables_inline) {
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
		let text_ctrl = Self::build_text_ctrl(panel, word_wrap, self_rc, self.frame);
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
		let audio_player = session.audio().cloned().and_then(|timeline| match AudioPlayer::new(&panel, timeline) {
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
			window,
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
		self.tabs[tab_index].session.set_stable_position(initial_pos);
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

	/// Stops every tab's audio ahead of the app closing, winding the native media sessions
	/// down deliberately rather than as a side effect of the frame being destroyed.
	pub fn stop_all_audio(&mut self) {
		for tab in &mut self.tabs {
			if let Some(player) = tab.audio_player.as_mut() {
				player.stop();
			}
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

	/// Restores focus to whichever control had it when the window was last active (the text
	/// control or the notebook), falling back to the notebook when there's no active document.
	/// Puts focus on the freshly opened document's text, and makes that the target a later
	/// [`Self::restore_focus`] will return to.
	///
	/// Opening a document is a request to read it, so it is not a case for restoring an earlier
	/// preference: whatever had focus a moment ago was about a different document, or about
	/// there being none.
	pub fn focus_document_text(&self) {
		if let Some(tab) = self.active_tab() {
			self.last_focus_in_text.set(true);
			tab.text_ctrl.set_focus();
		} else {
			self.notebook.set_focus();
		}
	}

	pub fn restore_focus(&self) {
		if self.last_focus_in_text.get() {
			if let Some(tab) = self.active_tab() {
				tab.text_ctrl.set_focus();
			} else {
				self.notebook.set_focus();
			}
		} else {
			self.notebook.set_focus();
		}
	}

	/// Records whether the text control or the notebook currently has focus, so focus can be
	/// restored to the same place when the window is next activated. Only updates when one of
	/// the two is confidently focused (a mid-focus-transition leaves the previous value).
	///
	/// With no documents open there is nothing to record: the notebook is the only focusable
	/// control, so it has focus by default rather than by choice. Writing that down as "the user
	/// prefers the tab strip" made the next document open onto its own tab strip instead of its
	/// text, because [`Self::restore_focus`] honours the preference and could not tell a real
	/// one from the absence of an alternative.
	#[cfg(target_os = "windows")]
	pub fn record_focus_target(&self) {
		if self.tabs.is_empty() {
			return;
		}
		if self.active_tab().is_some_and(|tab| tab.text_ctrl.has_focus()) {
			self.last_focus_in_text.set(true);
		} else if self.notebook.has_focus() {
			self.last_focus_in_text.set(false);
		}
	}

	/// Returns the native handle to fire an accessibility focus event on after `restore_focus`,
	/// or `None` when the control emits its own focus event and no manual event is needed. On
	/// Windows the read-only Richedit does not emit its own focus event on re-activation, so the
	/// text control needs the explicit event; the notebook's native tab control announces its
	/// selected tab on its own (firing a whole-control event here would swallow that).
	#[cfg(target_os = "windows")]
	pub fn focus_target_handle(&self) -> Option<*mut std::ffi::c_void> {
		if self.last_focus_in_text.get() { self.active_tab().map(|tab| tab.text_ctrl.get_handle()) } else { None }
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
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let result = tab.session.activate_link(pos);
			if result.found {
				match result.action {
					paperback_core::session::LinkAction::Internal => {
						if tab.window.needs_reload_for(result.offset, tab.session.document_len()) {
							reload_window_around(tab, result.offset, "reparse");
						}
						let local = tab.window.to_local(result.offset);
						tab.text_ctrl.set_focus();
						tab.text_ctrl.set_insertion_point(local);
						tab.text_ctrl.show_position(local);
						tab.session.check_and_record_history(result.offset);
						// TRANSLATORS: Announcement read by screen readers after following an internal link within the document
						live_region::announce(self.live_region_label, &t("Navigated to internal link."));
					}
					paperback_core::session::LinkAction::External => {
						launch_default_browser(&result.url, BrowserLaunchFlags::Default);
					}
					paperback_core::session::LinkAction::NotFound => {}
				}
			}
		}
	}
	pub fn activate_current_table(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			tab.session.get_table_at_position(pos)
		})
	}

	pub fn update_status_bar(&self) {
		let sleep_start = sleep_timer::start_ms();
		let sleep_duration = sleep_timer::duration_minutes();
		if self.tabs.is_empty() {
			// TRANSLATORS: Default status bar text when no document is open
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
			let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
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
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
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
			let was_inside = if bm.start == bm.end { prev == bm.start } else { prev >= bm.start && prev < bm.end };
			let is_inside =
				if bm.start == bm.end { position == bm.start } else { position >= bm.start && position < bm.end };
			let triggered = is_inside && !was_inside;
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

	/// When "sync caret to audio" is on, moves the caret to follow playback. Called from a
	/// recurring timer; a no-op for documents with no audio.
	///
	/// Uses `try_lock` on `config` rather than `lock`: this runs on the main thread on every
	/// timer tick, and a modal dialog (e.g. Options) pumps the OS message loop while it holds
	/// that same lock across `show_modal`. A blocking `lock` here would deadlock the UI thread
	/// against itself the moment a tick landed mid-dialog; skipping the tick is harmless since
	/// it just retries in 250ms.
	pub fn pump_audio(&mut self) {
		let Ok(config) = self.config.try_lock() else {
			return;
		};
		let sync_enabled = config.get_app_bool("sync_caret_to_audio", true);
		drop(config);
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let Some(player) = tab.audio_player.as_ref() else {
			return;
		};
		if !sync_enabled || !player.is_playing() {
			return;
		}
		let Some(elapsed) = player.elapsed_ms() else {
			tracing::warn!("sync caret to audio: playing but no elapsed position available");
			return;
		};
		let Some(cursor) = player.timeline().cursor_at_elapsed(elapsed) else {
			tracing::warn!(elapsed, "sync caret to audio: no clip covers the current elapsed time");
			return;
		};
		let Some(position) = player.timeline().clip(cursor.clip).map(|clip| clip.start) else {
			tracing::warn!(clip_index = cursor.clip, "sync caret to audio: cursor names a clip that doesn't exist");
			return;
		};
		// TODO(windowing, phase 3): still sets the caret directly rather than through a
		// window-aware jump, see C:\Users\Quin\.claude\plans\fluffy-hugging-crystal.md - a
		// scrub target outside the loaded window is silently clamped instead of reloading.
		let current = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		if i64::try_from(position).ok() != Some(current) {
			let target = i64::try_from(position).unwrap_or(current);
			let local = tab.window.to_local(target);
			tab.text_ctrl.set_insertion_point(local);
			tab.text_ctrl.show_position(local);
		}
	}

	/// Keeps the loaded window ahead of a caret that is moving forward, by appending to it.
	///
	/// A timer rather than an input hook because a screen reader's own text-walking never reaches
	/// this app's key handlers: NVDA's Say-All (and similar continuous-reading features) drives
	/// RichEdit's UI Automation text pattern directly. Without polling, reaching the loaded end
	/// during a Say-All would look exactly like reaching the real end of the document, and reading
	/// would stop mid-paragraph with millions of characters still unread. Reacting to where the
	/// caret actually is sidesteps having to know what put it there. `RELOAD_MARGIN`, a quarter of
	/// the window, gives even fast reading time to load more well before it runs out.
	///
	/// Because it runs on a timer, it must never do anything a reader could notice. Appending
	/// qualifies: every offset already handed out still points at the same character. Rebuilding
	/// the window does not, and this used to do that.
	///
	/// The bug that motivated the split: NVDA's Say-All holds its own offsets into the control,
	/// advances them itself rather than re-reading the caret, and has no handling for the text
	/// changing underneath it (`_TextReader` only checks whether the object died). Recentring the
	/// window swapped the whole buffer and moved `start`, so those offsets silently came to mean a
	/// different place in the book. Say-All read on from the wrong spot, moved the real caret there
	/// with it, and this timer then saw *that* position, decided it was near an edge, and recentred
	/// again - walking the window backwards to the top of the document a few hundred thousand
	/// characters at a time while the user listened. Reported as reading randomly jumping to the
	/// start of large text files.
	///
	/// Splitting extension from compaction was not enough on its own, because compaction stayed
	/// reachable from here for a caret that had run out of text behind it, on the reasoning that a
	/// forward reader could never be in that position. A reader whose offsets have gone stale can:
	/// compaction moves the loaded start out from under offsets their holder never revisits, which
	/// leaves those offsets sitting near the new start, which is the very condition to compact
	/// again. Each tick walked the window another `RELOAD_MARGIN` back, and the read reached the
	/// top of the document in about three seconds. A tick that can only extend cannot start that
	/// loop, so this one cannot.
	///
	/// Compaction happens elsewhere: at the navigation chokepoints, on the key and mouse events
	/// that end a read (see [`Self::compact_window_after_user_move`]), and on a resize, where the
	/// cost it exists to avoid is actually billed.
	pub fn pump_window_extend(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let doc_len = tab.session.document_len();
		if !tab.window.wants_extension_for(doc_pos, doc_len) {
			return;
		}
		if !Self::extend_window_forward(tab) {
			// The append did not land cleanly, so the control and the window no longer agree on
			// what is loaded. Every later position translation would be wrong by the difference;
			// rebuild to get back to a state that is at least consistent. This does move the
			// loaded start, so it is the one thing here a reader can notice, but a control whose
			// contents no one can locate is worse than a read that loses its place.
			reload_window_around(tab, doc_pos, "append failed");
			let local = tab.window.to_local(doc_pos);
			tab.text_ctrl.set_insertion_point(local);
		}
	}

	/// Compacts the window around the caret if the caret has run out of text behind it.
	///
	/// The counterpart to [`Self::pump_window_extend`], called from the key and mouse handlers
	/// rather than the timer. Anything that moves the loaded start invalidates the offsets a
	/// screen reader is reading from, so it may only happen where the user has already done
	/// something that stops a read, and a keypress or a click is exactly that.
	///
	/// Holding Up-arrow is the case this exists for. Auto-repeat sends key *down* events and no
	/// key up until release, so a held key eats into [`RELOAD_MARGIN`] without compacting; that
	/// margin is a quarter of the window, which is over a minute of held arrow before the caret
	/// reaches the loaded start, and releasing the key compacts well before then.
	pub fn compact_window_after_user_move(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let doc_len = tab.session.document_len();
		if !tab.window.needs_compaction_for(doc_pos, doc_len) {
			return;
		}
		reload_window_around(tab, doc_pos, "caret ran out of text behind it");
		let local = tab.window.to_local(doc_pos);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}

	/// Copies the current selection, widening it to the whole document when everything loaded is
	/// selected but only part of the document is.
	///
	/// Select All can only ever reach what the control holds, which for a windowed document is a
	/// slice of the book rather than the book. The window is not something a reader can see, name
	/// or reason about, so a selection covering all of it means all of it in the only sense the
	/// user has - and copying half a chapter when they asked for the book is a silent wrong answer.
	///
	/// Deliberately keyed off the selection rather than a remembered Select All: clicking anywhere
	/// collapses the selection, so this stops applying on its own with no flag to keep in step.
	///
	/// Returns whether it copied. False means nothing special applied and the caller should let
	/// the control's own copy run.
	pub fn copy_whole_document_if_all_selected(&self) -> bool {
		let Some(tab) = self.active_tab() else {
			return false;
		};
		let (from, to) = tab.text_ctrl.get_selection();
		if from >= to {
			return false;
		}
		let doc_len = tab.session.document_len();
		// Only the widened case is handled here. Anything else is left to the control's own copy,
		// which is better tested than anything this could put in its place.
		let covers_everything_loaded = from <= 0 && to >= tab.text_ctrl.get_last_position();
		if !covers_everything_loaded || tab.window.is_whole_document(doc_len) {
			return false;
		}
		let text = tab.session.get_text_range(0, doc_len);
		if text.is_empty() {
			return false;
		}
		Clipboard::get().set_text(&text)
	}

	/// Collapses a window that grew during a long read back to target size around the caret.
	///
	/// Called before a relayout, which is the one place a deep caret in a big loaded buffer is
	/// actually billed. Measured on a 16.5M-character document, the per-line work a screen reader
	/// does costs about a millisecond even at the far end - nothing against the seconds spent
	/// speaking that line - whereas rewrapping the same buffer costs tens of seconds. So growth is
	/// free while reading and only has to be paid back when the layout actually changes.
	///
	/// Resize events arrive continuously during a drag; this is self-limiting because the first
	/// one brings the window back under the threshold and the rest return immediately.
	pub fn compact_window_if_grown(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		if tab.window.loaded_len() <= text_window::compaction_threshold() {
			return;
		}
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		reload_window_around(tab, doc_pos, "resize");
		let local = tab.window.to_local(doc_pos);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}

	/// Appends the next chunk of the document to the end of the loaded window, leaving
	/// `window.start` and every offset already in play untouched.
	///
	/// Returns false if the append did not land cleanly, meaning the caller should compact rather
	/// than carry on with a control and a `TextWindow` that disagree.
	fn extend_window_forward(tab: &mut DocumentTab) -> bool {
		let doc_len = tab.session.document_len();
		let (from, to) = tab.window.extend_bounds(doc_len);
		if to <= from {
			return true;
		}
		// `from` is the current loaded end, which the session snapped to a paragraph boundary when
		// it produced this window. Snapping a boundary again is a no-op, so this slice abuts the
		// loaded text exactly: no gap, no repeated paragraph.
		let slice = tab.session.get_window(from, to);
		if slice.end <= from {
			return true;
		}
		if !append_slice_to_ctrl(tab.text_ctrl, &slice) {
			return false;
		}
		let previous_end = tab.window.end();
		tab.window.extend_end_to(slice.end);
		// The safe half, logged at debug so a report can be read alongside the window moves above.
		// A healthy forward read extends about once per chunk; a burst of these with the caret
		// pinned at the loaded end is the signature of the caret being moved by the append itself,
		// which is what `append_rtf_into_ctrl` saves and restores the selection to prevent.
		tracing::debug!(
			caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point()),
			from_end = previous_end,
			to_end = tab.window.end(),
			loaded = tab.window.loaded_len(),
			"extended loaded window"
		);
		// The control and the window have to agree on how much is loaded, give or take the single
		// display unit RichEdit does not store for a wholly-trailing paragraph mark (see
		// `write::stored_display_len`). Drift past that would offset every translation from here on.
		let drift = tab.text_ctrl.get_last_position() - tab.window.loaded_len();
		if !(-1..=0).contains(&drift) {
			tracing::warn!(drift, "window extension left the control and the window disagreeing");
			return false;
		}
		true
	}

	/// Jumps the caret to the very first or very last character of the *document*, reloading the
	/// text control's window at that edge (see `ui::text_window`).
	///
	/// `text_ctrl`'s own Ctrl+Home/Ctrl+End can only ever reach the ends of whatever window is
	/// currently loaded, which on a huge document is an arbitrary spot mid-book rather than the
	/// start or end of the document - and reaching a loaded edge that way then makes
	/// `pump_window_reload` recentre the window on it, so the keys both landed in the wrong place
	/// and paid for a reload to get there. `build_text_ctrl` intercepts them and routes here
	/// instead, which also keeps them consistent with every other jump in the app (history,
	/// audio sync, focus).
	pub fn jump_to_document_edge(&mut self, to_end: bool) {
		let (track, update) = {
			let Some(tab) = self.active_tab_mut() else {
				return;
			};
			let offset = if to_end { tab.session.document_len().max(0) } else { 0 };
			let update = move_to_offset_and_record_history(tab, offset);
			(tab.track, update)
		};
		persist_navigation_history(&self.config, track.then_some(&update));
	}

	/// Pauses audio on every tab except the active one, so switching tabs can't leave two
	/// documents narrating at once (the active tab may have audio of its own still playing,
	/// which this leaves untouched).
	pub fn pause_inactive_audio(&mut self) {
		let active = self.active_tab_index();
		for (index, tab) in self.tabs.iter_mut().enumerate() {
			if Some(index) != active
				&& let Some(player) = tab.audio_player.as_mut()
				&& player.is_playing()
			{
				player.pause();
			}
		}
	}

	/// Announces the current caret position as a percentage of the document via the live region.
	pub fn announce_current_percent(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let percent = navigation::reading_percent(tab, position);
		live_region::announce(self.live_region_label, &format!("{percent}%"));
	}

	pub fn reset_sound_line(&self) {
		self.last_sound_position.set(None);
		self.last_audio_seek_position.set(None);
	}

	/// Sets the temporary bookmark at the current caret position and announces it.
	pub fn set_temporary_bookmark(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let path_str = tab.file_path.to_string_lossy().to_string();
		let config = self.config.lock().unwrap();
		config.set_temporary_bookmark(&path_str, Some(position));
		config.flush();
		drop(config);
		// TRANSLATORS: Announced after setting a temporary bookmark at the current position
		live_region::announce(self.live_region_label, &t("Temporary bookmark set."));
	}

	/// Jumps to the temporary bookmark, announcing the line text there, or "No temporary bookmark."
	/// if none has been set.
	pub fn jump_to_temporary_bookmark(&mut self) {
		let path_str = {
			let Some(tab) = self.active_tab() else {
				return;
			};
			tab.file_path.to_string_lossy().to_string()
		};
		let position = {
			let config = self.config.lock().unwrap();
			config.get_temporary_bookmark(&path_str)
		};
		let Some(position) = position else {
			// TRANSLATORS: Announced when jumping to a temporary bookmark but none has been set
			live_region::announce(self.live_region_label, &t("No temporary bookmark."));
			return;
		};
		let (message, track, update) = {
			let tab = self.active_tab_mut().unwrap();
			let position = position.clamp(0, tab.session.document_len().max(0));
			let line_text = tab.session.get_line_text(position);
			let message = if line_text.trim().is_empty() {
				// TRANSLATORS: Fallback announcement when jumping to a temporary bookmark on a blank line
				t("Temporary bookmark.")
			} else {
				line_text
			};
			let update = move_to_offset_and_record_history(tab, position);
			(message, tab.track, update)
		};
		live_region::announce(self.live_region_label, &message);
		persist_navigation_history(&self.config, track.then_some(&update));
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
			let current_doc_pos = tab.window.to_doc(old_ctrl.get_insertion_point());
			// TODO(windowing): still whole-document on every wrap toggle, see
			// C:\Users\Quin\.claude\plans\fluffy-hugging-crystal.md Phase 2 - should re-slice just
			// `tab.window`'s existing range instead of reloading the whole document.
			let doc_len = tab.session.document_len();
			let slice = tab.session.get_window(0, doc_len);
			let text_ctrl = Self::build_text_ctrl(tab.panel, word_wrap, self_rc, self.frame);
			let sizer = BoxSizer::builder(Orientation::Vertical).build();
			sizer.add(&text_ctrl, 1, SizerFlag::Expand | SizerFlag::All, 0);
			tab.panel.set_sizer(sizer, true);
			fill_text_ctrl_with_formatting(text_ctrl, &slice);
			tab.window = TextWindow::whole(doc_len);
			if let Some(font) = build_font_from_readability(&rf) {
				text_ctrl.set_font(&font);
			}
			apply_foreground_color_to_ctrl(text_ctrl, rf.color);
			apply_bg_color_to_ctrl(text_ctrl, bg_color);
			apply_readability_format_to_ctrl(
				text_ctrl,
				line_spacing,
				paragraph_spacing,
				letter_spacing,
				text_alignment,
			);
			let max_pos = text_ctrl.get_last_position();
			let pos = tab.window.to_local(current_doc_pos).clamp(0, max_pos);
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
		let (style, parse_inputs) = {
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
			(readability_style(&cfg), parse_inputs)
		};
		for (tab, (path_str, password, forced_extension)) in self.tabs.iter_mut().zip(parse_inputs) {
			let _ = reparse_tab_in_place(tab, &path_str, &password, &forced_extension, render_tables_inline, &style);
		}
	}

	/// Reloads the tab at `index` if its file changed on disk since it was last parsed. Returns
	/// true only when the tab content was actually replaced. If the stored password no longer
	/// decrypts the file, prompts for a new one and retries once. Uses `try_lock` on the config:
	/// the caller may be a frame-activation handler running inside a nested modal event loop
	/// whose opener already holds the lock.
	pub fn reload_tab_if_changed(&mut self, index: usize) -> bool {
		let Some(tab) = self.tabs.get(index) else {
			return false;
		};
		if !tab.track {
			return false;
		}
		let Some(current) = read_fingerprint(&tab.file_path) else {
			return false;
		};
		if tab.disk_fingerprint == Some(current) {
			return false;
		}
		let path_str = tab.file_path.to_string_lossy().to_string();
		let Ok(cfg) = self.config.try_lock() else {
			return false;
		};
		if !cfg.get_app_bool("auto_reload_documents", true) {
			return false;
		}
		let password = cfg.get_document_password(&path_str);
		let forced_extension = cfg.get_document_format(&path_str);
		let render_tables_inline = cfg.get_app_bool("render_tables_inline", true);
		let style = readability_style(&cfg);
		drop(cfg);
		let tab = &mut self.tabs[index];
		let (positions, history_index) = tab.session.get_history();
		let positions = positions.to_vec();
		let reloaded =
			match reparse_tab_in_place(tab, &path_str, &password, &forced_extension, render_tables_inline, &style) {
				Ok(()) => true,
				Err(err) if err.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
					// Recorded before the prompt so a re-entrant call during its modal
					// event loop sees the file as unchanged and skips a second prompt.
					tab.disk_fingerprint = Some(current);
					self.reprompt_password_and_reparse(
						index,
						&path_str,
						&forced_extension,
						render_tables_inline,
						&style,
					)
				}
				Err(_) => false,
			};
		let tab = &mut self.tabs[index];
		if reloaded {
			tab.session.set_history(&positions, history_index);
			tracing::info!(path = %path_str, "document reloaded after on-disk change");
		} else {
			tab.disk_fingerprint = Some(current);
		}
		reloaded
	}

	/// Asks for a fresh password after a reload attempt failed to decrypt the file, then retries
	/// the re-parse once. Dismissing the prompt keeps the old tab content without an error: the
	/// reload was not user-initiated, so there is nothing to recover from. A wrong password shows
	/// the same load-error dialog as the open flow.
	fn reprompt_password_and_reparse(
		&mut self,
		index: usize,
		path_str: &str,
		forced_extension: &str,
		render_tables_inline: bool,
		style: &ReadabilityStyle,
	) -> bool {
		if let Ok(cfg) = self.config.try_lock() {
			cfg.set_document_password(path_str, "");
		}
		let Some(password) = prompt_for_password(&self.notebook) else {
			return false;
		};
		let tab = &mut self.tabs[index];
		match reparse_tab_in_place(tab, path_str, &password, forced_extension, render_tables_inline, style) {
			Ok(()) => {
				if !password.is_empty()
					&& let Ok(cfg) = self.config.try_lock()
				{
					cfg.set_document_password(path_str, &password);
				}
				true
			}
			Err(err) => {
				let message = build_document_load_error_message(&self.tabs[index].file_path, &err);
				// TRANSLATORS: Generic error dialog title
				show_error_dialog(&self.notebook, &message, &t("Error"));
				false
			}
		}
	}

	fn build_text_ctrl(panel: Panel, word_wrap: bool, self_rc: &Rc<Mutex<Self>>, frame: Frame) -> TextCtrl {
		let style = TextCtrlStyle::MultiLine
			| TextCtrlStyle::ReadOnly
			| TextCtrlStyle::Rich2
			| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
		let text_ctrl = TextCtrl::builder(&panel).with_style(style).build();
		let dm_for_enter = Rc::clone(self_rc);
		text_ctrl.on_char(move |event| {
			if let WindowEventData::Keyboard(kbd) = event {
				if kbd.get_key_code() == Some(13) || kbd.get_key_code() == Some(32) {
					let table_html = {
						let dm = dm_for_enter.lock().unwrap();
						dm.activate_current_table()
					};
					if let Some(html) = table_html {
						let frame = dm_for_enter.lock().unwrap().frame;
						// TRANSLATORS: Title of the dialog showing the HTML rendering of a table activated in the document
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
		// Ctrl+C is intercepted here rather than through the Edit menu because that menu only
		// exists on macOS; elsewhere the control handles the key itself and no menu event fires.
		#[cfg(not(target_os = "macos"))]
		{
			let dm_for_copy = Rc::clone(self_rc);
			text_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
				let handled = event.get_key_code() == Some(i32::from(b'C'))
					&& event.control_down()
					&& dm_for_copy.try_lock().is_ok_and(|dm| dm.copy_whole_document_if_all_selected());
				if !handled {
					event.skip(true);
				}
			});
		}
		let dm_for_key_up = Rc::clone(self_rc);
		text_ctrl.bind_internal(EventType::KEY_UP, move |event| {
			event.skip(true);
			if let Ok(mut dm) = dm_for_key_up.try_lock() {
				// Before the status bar reads the position, so it reports the compacted window.
				dm.compact_window_after_user_move();
				dm.update_status_bar();
				dm.save_position_throttled();
				dm.check_bookmark_sounds();
			}
		});
		let dm_for_mouse = Rc::clone(self_rc);
		text_ctrl.bind_internal(wxdragon::event::EventType::LEFT_UP, move |event| {
			event.skip(true);
			if let Ok(mut dm) = dm_for_mouse.try_lock() {
				dm.preferred_column.set(None);
				dm.compact_window_after_user_move();
				dm.update_status_bar();
				dm.save_position_throttled();
				dm.check_bookmark_sounds();
			}
		});
		let text_ctrl_for_menu = text_ctrl;
		let dm_for_keys = Rc::clone(self_rc);
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
				if let Some(to_end) = document_edge_for_key(key, kbd.control_down(), kbd.shift_down(), kbd.alt_down()) {
					kbd.event.skip(false);
					if let Ok(mut dm) = dm_for_keys.try_lock() {
						dm.preferred_column.set(None);
						dm.jump_to_document_edge(to_end);
					}
					return;
				}
				#[cfg(target_os = "windows")]
				if (key == WXK_DOWN || key == WXK_UP) && !kbd.shift_down() && !kbd.control_down() && !kbd.alt_down() {
					let going_down = key == WXK_DOWN;
					let nav_result = dm_for_keys.try_lock().ok().and_then(|mut dm| {
						let start_of_line = dm.config.lock().unwrap().get_app_bool("line_start_navigation", false);
						let pref_col = dm.preferred_column.get();
						dm.active_tab_mut()
							.and_then(|tab| navigate_line_by_column(tab, going_down, pref_col, start_of_line))
					});
					if let Some((new_pos, new_col)) = nav_result {
						kbd.event.skip(false);
						text_ctrl_for_menu.set_insertion_point(new_pos);
						text_ctrl_for_menu.show_position(new_pos);
						if let Ok(dm) = dm_for_keys.try_lock() {
							dm.preferred_column.set(Some(new_col));
							dm.update_status_bar();
						}
					} else {
						kbd.event.skip(true);
					}
					return;
				}
				#[cfg(target_os = "windows")]
				if let Ok(dm) = dm_for_keys.try_lock() {
					dm.preferred_column.set(None);
				}
				let action = {
					if let Ok(dm) = dm_for_keys.try_lock() {
						let config = dm.config.lock().unwrap();
						config.get_shortcuts().find_action(key, kbd.control_down(), kbd.alt_down(), kbd.shift_down())
					} else {
						None
					}
				};
				if let Some(act) = action {
					match act {
						ActionId::AnnouncePercent => {
							kbd.event.skip(false);
							if let Ok(dm) = dm_for_keys.try_lock() {
								dm.announce_current_percent();
							}
							return;
						}
						ActionId::SetTemporaryBookmark => {
							kbd.event.skip(false);
							if let Ok(dm) = dm_for_keys.try_lock() {
								dm.set_temporary_bookmark();
							}
							return;
						}
						ActionId::JumpToTemporaryBookmark => {
							kbd.event.skip(false);
							if let Ok(mut dm) = dm_for_keys.try_lock() {
								dm.jump_to_temporary_bookmark();
							}
							return;
						}
						_ => {
							if !kbd.control_down() && !kbd.alt_down() || cfg!(target_os = "linux") {
								let menu_id = menu_ids::action_to_menu_id(act);
								kbd.event.skip(false);
								frame_for_keys.process_menu_command(menu_id);
								return;
							}
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

/// Which end of the document a key press names as a "jump to the very start/end" gesture, if
/// any: `Some(true)` for the end, `Some(false)` for the start. See
/// `DocumentManager::jump_to_document_edge` for why these are intercepted rather than left to
/// the text control.
///
/// Ctrl+Home/Ctrl+End everywhere - wxWidgets reports macOS's Command key as `control_down`, so
/// that covers Cmd+Home/Cmd+End there - plus Cmd+Up/Cmd+Down on macOS, which is what Mac text
/// views actually bind document start/end to, and the only one of the two most Apple keyboards
/// can even type (they have no Home/End keys). Bare Home/End are deliberately not included on
/// macOS: there they scroll without moving the caret, which is a different gesture.
const fn document_edge_for_key(key: i32, control: bool, shift: bool, alt: bool) -> Option<bool> {
	if !control || shift || alt {
		return None;
	}
	match key {
		WXK_HOME => Some(false),
		WXK_END => Some(true),
		#[cfg(target_os = "macos")]
		WXK_UP => Some(false),
		#[cfg(target_os = "macos")]
		WXK_DOWN => Some(true),
		_ => None,
	}
}

/// One line-vertical-navigation attempt within whatever's currently loaded in `tab.text_ctrl`.
/// Returns `None` (outer) if the current position has no known line/column (shouldn't happen in
/// practice), `Some(None)` if the target line falls outside what's currently loaded - the caller
/// checks whether there's more document in that direction and, if so, reloads and retries - or
/// `Some(Some(..))` on success.
#[cfg(target_os = "windows")]
fn try_navigate_line_by_column(
	tab: &DocumentTab,
	going_down: bool,
	pref_col: Option<i64>,
	start_of_line: bool,
) -> Option<Option<(i64, i64)>> {
	let text_ctrl = tab.text_ctrl;
	let current_pos = text_ctrl.get_insertion_point().max(0);
	let (current_col, current_line) = text_ctrl.position_to_xy(current_pos)?;
	let col = pref_col.unwrap_or(current_col);
	let target_line = if going_down { current_line + 1 } else { current_line - 1 };
	if target_line < 0 {
		return Some(None);
	}
	let target_line_start = text_ctrl.xy_to_position(0, target_line);
	if target_line_start < 0 {
		return Some(None);
	}
	if start_of_line {
		return Some(Some((target_line_start, 0)));
	}
	let target_line_len = i64::from(text_ctrl.get_line_length(target_line));
	let new_pos = target_line_start + col.min(target_line_len);
	Some(Some((new_pos, col)))
}

/// Returns (`new_position`, `preferred_column`) for vertical navigation.
/// With `start_of_line` set, the caret lands at the start of the target visual line. Otherwise it
/// uses character-column-based navigation (`pref_col` or the current column), so the cursor lands on
/// the same character column (not pixel column) on the target visual line.
///
/// Reloads `tab`'s window and retries once if Up/Down would otherwise stop at a loaded-window
/// boundary that isn't the document's actual start/end. Without this, Up/Down (and Page Up/Down,
/// which RichEdit handles natively with no window awareness at all) can strand the caret mid-chapter
/// with no keyboard-only way past it except an explicit jump (heading/bookmark navigation etc.) -
/// found the hard way testing a huge book, not something worth leaving as a TODO.
#[cfg(target_os = "windows")]
fn navigate_line_by_column(
	tab: &mut DocumentTab,
	going_down: bool,
	pref_col: Option<i64>,
	start_of_line: bool,
) -> Option<(i64, i64)> {
	if let Some(result) = try_navigate_line_by_column(tab, going_down, pref_col, start_of_line)? {
		return Some(result);
	}
	let doc_len = tab.session.document_len();
	let has_more = if going_down { tab.window.end() < doc_len } else { tab.window.start() > 0 };
	if !has_more {
		return None;
	}
	let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point().max(0));
	reload_window_around(tab, doc_pos, "line navigation");
	let local = tab.window.to_local(doc_pos);
	tab.text_ctrl.set_insertion_point(local);
	try_navigate_line_by_column(tab, going_down, pref_col, start_of_line)?
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
	// TRANSLATORS: Label for the password entry field in the "Document Password" prompt dialog
	let dialog = TextEntryDialog::builder(parent, &t("&Password:"), &t("Document Password")).password().build();
	if dialog.show_modal() != ID_OK {
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

fn fill_text_ctrl(text_ctrl: TextCtrl, content: &str) {
	text_ctrl.set_value(content);
}

struct ReadabilityStyle {
	rf: ReadabilityFont,
	line_spacing: i32,
	bg_color: i32,
	text_alignment: i32,
	letter_spacing: i32,
	paragraph_spacing: i32,
}

fn readability_style(cfg: &ConfigManager) -> ReadabilityStyle {
	ReadabilityStyle {
		rf: cfg.get_readability_font(),
		line_spacing: cfg.get_line_spacing(),
		bg_color: cfg.get_bg_color(),
		text_alignment: cfg.get_text_alignment(),
		letter_spacing: cfg.get_letter_spacing(),
		paragraph_spacing: cfg.get_paragraph_spacing(),
	}
}

/// Builds a fresh session for `tab`'s file and refills its text control, restoring the reading
/// position. Returns the parse error and leaves the tab unchanged if the re-parse fails.
fn reparse_tab_in_place(
	tab: &mut DocumentTab,
	path_str: &str,
	password: &str,
	forced_extension: &str,
	render_tables_inline: bool,
	style: &ReadabilityStyle,
) -> Result<(), String> {
	let new_fingerprint = read_fingerprint(&tab.file_path);
	let current_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
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
	let new_session = match DocumentSession::new(path_str, password, forced_extension, render_tables_inline) {
		Ok(session) => session,
		Err(err) => {
			tracing::error!(path = %path_str, error = %err, "failed to re-parse document");
			return Err(err);
		}
	};
	tab.session = new_session;
	// TODO(windowing): still whole-document on every reparse, see
	// C:\Users\Quin\.claude\plans\fluffy-hugging-crystal.md Phase 2 - should load a window
	// centered on `restored_pos` instead.
	let doc_len = tab.session.document_len();
	let slice = tab.session.get_window(0, doc_len);
	fill_text_ctrl_with_formatting(tab.text_ctrl, &slice);
	tab.window = TextWindow::whole(doc_len);
	if let Some(font) = build_font_from_readability(&style.rf) {
		tab.text_ctrl.set_font(&font);
	}
	apply_foreground_color_to_ctrl(tab.text_ctrl, style.rf.color);
	apply_bg_color_to_ctrl(tab.text_ctrl, style.bg_color);
	apply_readability_format_to_ctrl(
		tab.text_ctrl,
		style.line_spacing,
		style.paragraph_spacing,
		style.letter_spacing,
		style.text_alignment,
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
	tab.disk_fingerprint = new_fingerprint;
	Ok(())
}

/// Loads into `text_ctrl` whichever window of `session`'s content contains `target_pos`: the
/// whole document for anything under `text_window::should_use_whole_document`'s threshold
/// (identical to the app's pre-windowing behavior), otherwise a bounded window centered on
/// `target_pos`. Returns the `TextWindow` describing what's now actually loaded.
fn load_window_into_ctrl(text_ctrl: TextCtrl, session: &DocumentSession, target_pos: i64, doc_len: i64) -> TextWindow {
	let slice = if text_window::should_use_whole_document(doc_len) {
		session.get_window(0, doc_len)
	} else {
		let (raw_start, raw_end) = text_window::target_window_bounds(target_pos, doc_len);
		session.get_window(raw_start, raw_end)
	};
	let window = TextWindow::new(slice.start, slice.end);
	fill_text_ctrl_with_formatting(text_ctrl, &slice);
	window
}

/// Reloads `tab`'s window to one centered on `doc_offset`. Call sites go through
/// `navigation::jump_to_doc_offset`, which checks `TextWindow::needs_reload_for` first, so this
/// always actually reloads when called.
///
/// TODO(windowing, phase 2): this doesn't reapply readability/font/color formatting the way a
/// full rebuild does (`apply_readability_format_to_ctrl` and friends), since those need
/// `ConfigManager` values that aren't available at the navigation chokepoints this is called
/// from without changing their signatures (defeating the point of routing ~90 call sites
/// through two unchanged chokepoints). Bold/italic/underline markers are unaffected (handled by
/// `load_window_into_ctrl`/`fill_text_ctrl_with_formatting` from the document's own markers,
/// not from readability settings) - this only means a reload can momentarily show default
/// line/paragraph/letter spacing and alignment until something else (a settings change, a
/// word-wrap toggle) reapplies them across all tabs. Fix by caching the last-applied readability
/// values on `DocumentTab` itself, updated wherever `apply_line_spacing`/`apply_paragraph_spacing`/
/// `apply_letter_spacing`/`apply_text_alignment` already loop over every tab.
///
/// `reason` names the call site in the log. Moving the loaded start is the one thing that can
/// pull the ground out from under a screen reader mid-read, so every one of these is recorded:
/// when someone reports a Say-All losing its place, the log says whether the window moved and
/// which path moved it, without anyone having to reproduce it first. Equally, a report with no
/// line here is evidence the window is not what moved, which is worth just as much.
pub fn reload_window_around(tab: &mut DocumentTab, doc_offset: i64, reason: &'static str) {
	let doc_len = tab.session.document_len();
	let before = tab.window;
	tab.window = load_window_into_ctrl(tab.text_ctrl, &tab.session, doc_offset, doc_len);
	if before.start() != tab.window.start() {
		tracing::info!(
			reason,
			caret = doc_offset,
			from_start = before.start(),
			from_end = before.end(),
			to_start = tab.window.start(),
			to_end = tab.window.end(),
			doc_len,
			"loaded window start moved"
		);
	}
}

/// Appends `slice` to whatever the control already holds, preserving existing offsets.
///
/// The counterpart to [`fill_text_ctrl_with_formatting`], which replaces everything. Returns
/// false if the append did not complete, so the caller can rebuild instead of trusting a control
/// that is now part-way through a chunk.
fn append_slice_to_ctrl(text_ctrl: TextCtrl, slice: &WindowSlice) -> bool {
	let content = slice.text.as_str();
	let segments = merge_formatting_markers(&slice.markers);
	#[cfg(target_os = "windows")]
	if !segments.is_empty()
		&& let Some(font) = text_ctrl.get_font()
	{
		let expected = write::sanitize_for_rich_edit(content);
		let rtf = write::build_rtf(
			&expected,
			&segments,
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if append_rtf_into_ctrl(text_ctrl, &rtf) {
			return true;
		}
		tracing::warn!("RTF append did not complete");
		return false;
	}
	// Plain-text path: the same one `fill_text_ctrl_with_formatting` falls back to, with the
	// segments shifted past what is already loaded so they land on the text just appended.
	let base = text_ctrl.get_last_position();
	text_ctrl.append_text(content);
	if text_ctrl.get_last_position() <= base {
		return false;
	}
	let shifted: Vec<FormatSegment> =
		segments.iter().map(|seg| FormatSegment { start: seg.start + base, end: seg.end + base, ..*seg }).collect();
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &shifted);
	true
}

/// Fills `text_ctrl` with `slice`'s text and bold/italic/underline markers. `slice` may be a
/// window into a much larger document (see `ui::text_window`) rather than its full content;
/// this function has no notion of "the whole document" and just fills whatever it's handed.
///
/// On Windows this streams a single RTF blob into the native `RichEdit` control
/// via `EM_STREAMIN` (see `rtf::stream::stream_rtf_into_ctrl`) instead of issuing
/// one `SetStyle` call per formatting span, which is far cheaper on documents
/// with thousands of spans. `wxTextCtrl::SetValue` can't be used for this, since it
/// does not forward to the native `WM_SETTEXT` handler that auto-detects a
/// `{\rtf` prefix, so it would just store the markup as literal text. If
/// streaming doesn't round-trip back to the original content, this falls back
/// to the plain-text + per-segment path used on every other platform.
fn fill_text_ctrl_with_formatting(text_ctrl: TextCtrl, slice: &WindowSlice) {
	let content = slice.text.as_str();
	let segments = merge_formatting_markers(&slice.markers);
	#[cfg(target_os = "windows")]
	if !segments.is_empty()
		&& let Some(font) = text_ctrl.get_font()
	{
		// What RichEdit will actually end up holding, which is not always what it is handed -
		// see `write::sanitize_for_rich_edit`. Everything below compares against this rather
		// than against `content`.
		let expected = write::sanitize_for_rich_edit(content);
		let rtf = write::build_rtf(
			&expected,
			&segments,
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if stream_rtf_into_ctrl(text_ctrl, &rtf) {
			let round_tripped = text_ctrl.get_value();
			// RichEdit's document model implicitly terminates the buffer, so a
			// wholly-trailing "\par" (with no content after it) doesn't manifest
			// as a stored character. Tolerate exactly that one known, harmless
			// discrepancy rather than falling back over it: the very last
			// position of *whatever we streamed in* ends up one short of `expected`,
			// which only matters at its literal last character. This applies the same
			// way to a windowed slice as to the whole document - RichEdit has no notion
			// of "there's more after this that isn't loaded"; from its perspective
			// `expected` (window or not) *is* the whole buffer it was asked to store.
			let matched = round_tripped == *expected
				|| (expected.ends_with('\n')
					&& round_tripped.len() + 1 == expected.len()
					&& expected.starts_with(round_tripped.as_str()));
			if matched {
				return;
			}
			// Not identical, but harmless as long as it cost no display units: every position
			// the app hands the control is an offset into this buffer, so a length change
			// breaks the caret, bookmarks and `ui::text_window`'s translation alike, whereas a
			// same-width substitution is only cosmetic. RichEdit does make a few of those on
			// its own - U+2028 comes back as a vertical tab, U+FDD0..=U+FDEF as spaces - and
			// falling back over those would cost seconds per window load to fix nothing. A
			// length check is still decisive against the failure this guards: unparsed RTF
			// stored as literal text would be tens of thousands of display units longer than
			// the content it encodes.
			let expected_len = write::stored_display_len(&expected);
			let stored_len = text_ctrl.get_last_position();
			if stored_len == expected_len {
				tracing::debug!(expected_len, "RTF round-trip was substituted but not resized; keeping it");
				return;
			}
			tracing::warn!(stored_len, expected_len, "RTF fast path changed the content's length; falling back");
		} else {
			tracing::warn!("RTF stream-in did not complete; falling back");
		}
		// Never leave raw RTF markup on screen for an accessibility user;
		// fall back below to the plain-text + segment-loop path.
	}
	fill_text_ctrl(text_ctrl, content);
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &segments);
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
		SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(addr_of_mut!(caret) as usize)), None);
		SendMessageW(hwnd, EM_SETSEL, Some(WPARAM(0)), Some(LPARAM(-1_isize)));
		let mut cf = CHARFORMAT2W::default();
		cf.Base.cbSize = size_of::<CHARFORMAT2W>() as u32;
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
				SendMessageW(hwnd, EM_GETSEL, Some(WPARAM(addr_of_mut!(caret) as usize)), None);
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
				cf.Base.cbSize = size_of::<CHARFORMAT2W>() as u32;
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
	use paperback_core::document::MarkerType;
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
			MarkerType::Bold => 0,
			MarkerType::Italic => 1,
			MarkerType::Underline => 2,
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
	if let Some(seg) = open
		&& (seg.bold || seg.italic || seg.underline)
	{
		segments.push(seg);
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
		// TRANSLATORS: Right-click context menu item and status text to bookmark the current position
		.append_item(menu_ids::TOGGLE_BOOKMARK, &t("Create &bookmark"), &t("Create bookmark"))
		// TRANSLATORS: Right-click context menu item and status text to bookmark the current position with an attached note
		.append_item(menu_ids::BOOKMARK_WITH_NOTE, &t("Bookmark with &note"), &t("Create bookmark with note"))
		.append_separator()
		// TRANSLATORS: Right-click context menu item and status text to open the find dialog
		.append_item(menu_ids::FIND, &t("&Find"), &t("Find text"))
		// TRANSLATORS: Right-click context menu item and status text to repeat the last search forward
		.append_item(menu_ids::FIND_NEXT, &t("Find &next"), &t("Find next match"))
		// TRANSLATORS: Right-click context menu item and status text to repeat the last search backward
		.append_item(menu_ids::FIND_PREVIOUS, &t("Find &previous"), &t("Find previous match"))
		.append_separator()
		// TRANSLATORS: Right-click context menu item and status text to jump to a specific page
		.append_item(menu_ids::GO_TO_PAGE, &t("Go to &page"), &t("Go to page"))
		// TRANSLATORS: Right-click context menu item and status text to jump to a specific line
		.append_item(menu_ids::GO_TO_LINE, &t("Go to &line"), &t("Go to line"))
		// TRANSLATORS: Right-click context menu item and status text to jump to a percentage through the document
		.append_item(menu_ids::GO_TO_PERCENT, &t("Go to &percent"), &t("Go to percent"))
		.build();
	text_ctrl.popup_menu(&mut menu, None);
}

#[cfg(test)]
mod tests {
	use std::{env, fs, path::PathBuf, process};

	use paperback_core::{document::MarkerType, session::LineMarker};

	use super::{FormatSegment, merge_formatting_markers, read_fingerprint};

	struct TempFile {
		path: PathBuf,
	}

	impl TempFile {
		fn with_content(name: &str, content: &[u8]) -> Self {
			let path = env::temp_dir().join(format!("paperback-fingerprint-{}-{name}", process::id()));
			fs::write(&path, content).unwrap();
			Self { path }
		}
	}

	impl Drop for TempFile {
		fn drop(&mut self) {
			let _ = fs::remove_file(&self.path);
		}
	}

	#[test]
	fn fingerprint_of_missing_path_is_none() {
		let path = env::temp_dir().join(format!("paperback-fingerprint-{}-does-not-exist", process::id()));
		assert_eq!(read_fingerprint(&path), None);
	}

	#[test]
	fn unchanged_file_keeps_the_same_fingerprint() {
		let file = TempFile::with_content("unchanged", b"stable content");
		let first = read_fingerprint(&file.path);
		assert!(first.is_some());
		assert_eq!(first, read_fingerprint(&file.path));
	}

	#[test]
	fn rewriting_with_a_different_length_changes_the_fingerprint() {
		let file = TempFile::with_content("grows", b"short");
		let before = read_fingerprint(&file.path);
		fs::write(&file.path, b"content that is clearly longer").unwrap();
		let after = read_fingerprint(&file.path);
		assert!(before.is_some() && after.is_some());
		assert_ne!(before, after);
	}

	fn marker(mtype: MarkerType, position: i64, length: i64) -> LineMarker {
		LineMarker { mtype, position, text: String::new(), reference: String::new(), level: 0, length }
	}

	#[test]
	fn no_markers_yields_no_segments() {
		assert_eq!(merge_formatting_markers(&[]), Vec::new());
	}

	#[test]
	fn zero_length_markers_are_ignored() {
		let markers = [marker(MarkerType::Bold, 5, 0)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn non_format_markers_are_ignored() {
		let markers = [marker(MarkerType::Heading1, 0, 10), marker(MarkerType::Link, 2, 3)];
		assert_eq!(merge_formatting_markers(&markers), Vec::new());
	}

	#[test]
	fn single_bold_marker_produces_one_segment() {
		let markers = [marker(MarkerType::Bold, 0, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 4, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn overlapping_bold_and_italic_keep_both_on_the_intersection() {
		// Bold over [0,10), italic over [4,7): the middle run must carry both.
		let markers = [marker(MarkerType::Bold, 0, 10), marker(MarkerType::Italic, 4, 3)];
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
		let markers = [marker(MarkerType::Bold, 0, 4), marker(MarkerType::Bold, 4, 4)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 8, bold: true, italic: false, underline: false }]
		);
	}

	#[test]
	fn all_three_styles_can_stack() {
		let markers =
			[marker(MarkerType::Bold, 0, 6), marker(MarkerType::Italic, 0, 6), marker(MarkerType::Underline, 0, 6)];
		assert_eq!(
			merge_formatting_markers(&markers),
			vec![FormatSegment { start: 0, end: 6, bold: true, italic: true, underline: true }]
		);
	}
}
