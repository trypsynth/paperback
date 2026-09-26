use std::{
	cell::Cell,
	fs,
	path::{Path, PathBuf},
	rc::Rc,
	sync::Mutex,
	time::{Instant, SystemTime},
};

use paperback_core::{
	config::ConfigManager, document::ParseSettings, parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession,
};
use patois::t;
use wxdragon::prelude::*;

use super::{
	navigation::{self, move_to_offset_and_record_history, persist_navigation_history},
	readability::{
		ReadabilityStyle, apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_readability_format_to_ctrl,
		build_font_from_readability, readability_style,
	},
	sleep_timer, status,
	text_render::fill_text_ctrl_with_formatting,
};
use crate::{audio_player::AudioPlayer, text_window::TextWindow, ui::navigation::announce};

mod appearance;
mod audio;
mod lifecycle;
mod ocr;
mod window;

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	pub file_path: PathBuf,
	pub track: bool,
	pub audio_player: Option<AudioPlayer>,
	disk_fingerprint: Option<FileFingerprint>,
	/// The column an unbroken run of Up/Down presses is aiming for, so passing through a short
	/// line does not pull the caret left for good. Per tab: each document is read at its own
	/// column, and switching tabs must not carry one document's column into another.
	pub preferred_column: Cell<Option<i64>>,
	/// The document-absolute position marked as the beginning of a selection to copy from, or
	/// `None`. Per tab: a mark names a position in one document, and carrying one document's mark
	/// into another would have it name different text entirely. Document-absolute rather than a
	/// `text_ctrl` position for the same reason [`Self::window`] exists - the loaded window can be
	/// swapped out from under it between marking and copying.
	pub selection_mark: Cell<Option<i64>>,
	/// The document-absolute bounds of whatever's currently loaded into `text_ctrl`. See
	/// `text_window` - for most documents this covers the whole thing, same as before
	/// windowing existed; only huge documents actually get a partial window.
	pub window: TextWindow,
	/// The OCR job running for this tab, if one is. Written and read only on the UI thread.
	ocr_job: Option<ocr::OcrJob>,
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
	pub(super) frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	pub(super) config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
	last_sound_position: Cell<Option<i64>>,
	last_audio_seek_position: Cell<Option<i64>>,
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
			last_focus_in_text: Cell::new(true),
			recently_closed: Vec::new(),
		}
	}

	pub fn active_tab_index(&self) -> Option<usize> {
		let selection = self.notebook.selection();
		if selection >= 0 { usize::try_from(selection).ok() } else { None }
	}

	pub fn active_tab(&self) -> Option<&DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get(i))
	}

	/// The column vertical navigation is aiming for in the active document, if any.
	pub fn preferred_column(&self) -> Option<i64> {
		self.active_tab().and_then(|tab| tab.preferred_column.get())
	}

	/// Records the column vertical navigation should keep aiming for in the active document.
	pub fn set_preferred_column(&self, column: Option<i64>) {
		if let Some(tab) = self.active_tab() {
			tab.preferred_column.set(column);
		}
	}

	/// The position marked as the beginning of a selection to copy from, if one is set.
	pub fn selection_mark(&self) -> Option<i64> {
		self.active_tab().and_then(|tab| tab.selection_mark.get())
	}

	/// Marks a position as the beginning of a selection to copy from, or clears the mark.
	pub fn set_selection_mark(&self, position: Option<i64>) {
		if let Some(tab) = self.active_tab() {
			tab.selection_mark.set(position);
		}
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

	/// Tells screen readers that the text control has focus, after `restore_focus`. On Windows the
	/// read-only Richedit does not emit its own focus event on re-activation, and `SetFocus` on a
	/// window that already has focus emits nothing at all, so the text control needs the explicit
	/// event; the notebook's native tab control announces its selected tab on its own (firing a
	/// whole-control event there would swallow that), so nothing is sent for it.
	#[cfg(target_os = "windows")]
	pub fn announce_focus(&self) {
		if !self.last_focus_in_text.get() {
			return;
		}
		if let Some(tab) = self.active_tab() {
			let hwnd = windows::Win32::Foundation::HWND(tab.text_ctrl.get_handle());
			// EVENT_OBJECT_FOCUS = 0x8005, OBJID_CLIENT = -4, CHILDID_SELF = 0
			unsafe {
				windows::Win32::UI::Accessibility::NotifyWinEvent(0x8005, hwnd, -4, 0);
			}
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
		let history_update = {
			let Some(tab) = self.active_tab_mut() else {
				return;
			};
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let result = tab.session.activate_link(pos);
			if !result.found {
				return;
			}
			match result.action {
				paperback_core::session::LinkAction::Internal => {
					let update = move_to_offset_and_record_history(tab, result.offset);
					tab.track.then_some(update)
				}
				paperback_core::session::LinkAction::External => {
					launch_default_browser(&result.url, BrowserLaunchFlags::Default);
					return;
				}
				paperback_core::session::LinkAction::NotFound => return,
			}
		};
		// TRANSLATORS: Announcement read by screen readers after following an internal link within the document
		announce(self.live_region_label, t("Navigated to internal link."));
		persist_navigation_history(&self.config, history_update.as_ref());
	}
	pub fn activate_current_table(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			tab.session.get_table_at_position(pos)
		})
	}

	pub fn activate_current_formula(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			tab.session.get_formula_at_position(pos).map(|mathml| {
				format!(
					"<style>math {{ font-size: 2.5em; }} body {{ display: flex; justify-content: center; margin-top: 2em; }}</style>{mathml}"
				)
			})
		})
	}

	/// Whether the caret is on an image-only PDF page's OCR placeholder line.
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

	/// Announces the current caret position as a percentage of the document, and the page it falls
	/// on where the document has pages, via the live region.
	pub fn announce_current_percent(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let percent = navigation::reading_percent(tab, position);
		let page = page_at(tab, position);
		announce(self.live_region_label, position_announcement(percent, page));
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
		announce(self.live_region_label, t("Temporary bookmark set."));
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
			announce(self.live_region_label, t("No temporary bookmark."));
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
		announce(self.live_region_label, message);
		persist_navigation_history(&self.config, track.then_some(&update));
	}

	/// Re-parses every open document with the new parse settings and refills its text control.
	/// Re-parsing (rather than transforming in place) keeps every format's table rendering
	/// identical via the shared parse-time helper. A tab whose re-parse fails is left unchanged.
	pub fn apply_parse_settings(&mut self, settings: ParseSettings) {
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
			let _ = reparse_tab_in_place(tab, &path_str, &password, &forced_extension, settings, &style);
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
		let settings = parse_settings(&cfg);
		let style = readability_style(&cfg);
		drop(cfg);
		let tab = &mut self.tabs[index];
		let (positions, history_index) = tab.session.get_history();
		let positions = positions.to_vec();
		let reloaded = match reparse_tab_in_place(tab, &path_str, &password, &forced_extension, settings, &style) {
			Ok(()) => true,
			Err(err) if err.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
				// Recorded before the prompt so a re-entrant call during its modal
				// event loop sees the file as unchanged and skips a second prompt.
				tab.disk_fingerprint = Some(current);
				self.reprompt_password_and_reparse(index, &path_str, &forced_extension, settings, &style)
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
		settings: ParseSettings,
		style: &ReadabilityStyle,
	) -> bool {
		if let Ok(cfg) = self.config.try_lock() {
			cfg.set_document_password(path_str, "");
		}
		let Some(password) = prompt_for_password(&self.notebook) else {
			return false;
		};
		let tab = &mut self.tabs[index];
		match reparse_tab_in_place(tab, path_str, &password, forced_extension, settings, style) {
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
}

/// The 1-based page `position` falls on, or `None` for a document that has no pages at all -
/// meaning one with no page-break markers, which is every plain text and markdown document and any
/// EPUB that carries no page list.
///
/// The `.max(1)` covers a hole in `current_page`, which answers 0 both for a document with no pages
/// and for a position ahead of the first marker: the first is what the guard above has already
/// turned into `None`, the second is not a page number to say out loud. Only a PDF escapes the
/// second case, since its first page marker sits at offset 0. An RTF's and a DAISY book's first
/// break lands mid-document, and an EPUB page list starts at the first *content* page, leaving the
/// cover and title pages in front of every marker.
///
/// Every caller of `current_page` patches that 0 differently or not at all - Go to Page clamps it
/// (`dialogs::show_go_to_page_dialog`), the Elements view reads it as "no closest page"
/// (`dialogs::elements`), and iOS's Go To sheet shows it to the reader as a plain 0. This agrees
/// with Go to Page, the one other place that speaks a page number; the fix that would stop it being
/// forgotten belongs in `current_page` itself.
fn page_at(tab: &DocumentTab, position: i64) -> Option<i32> {
	if tab.session.page_count() == 0 {
		return None;
	}
	Some(tab.session.current_page(position).max(1))
}

/// What the "announce percentage" shortcut says for `percent` through the document, on `page` where
/// it has pages - "15%, page 30" - or the bare percentage where it has none.
fn position_announcement(percent: i32, page: Option<i32>) -> String {
	let Some(page) = page else {
		return format!("{percent}%");
	};
	// TRANSLATORS: Announced by the shortcut that reports the reading position. %s is how far through the document the reader is, e.g. "15%"; %d is the page number.
	t("%s, page %d").replacen("%d", &page.to_string(), 1).replacen("%s", &format!("{percent}%"), 1)
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

/// The parse-time toggles as the reader has them set.
fn parse_settings(cfg: &ConfigManager) -> ParseSettings {
	ParseSettings {
		render_tables_inline: cfg.get_app_bool("render_tables_inline", true),
		join_pdf_paragraphs: cfg.get_app_bool("join_pdf_paragraphs", true),
	}
}

/// Builds a fresh session for `tab`'s file and refills its text control, restoring the reading
/// position. Returns the parse error and leaves the tab unchanged if the re-parse fails.
fn reparse_tab_in_place(
	tab: &mut DocumentTab,
	path_str: &str,
	password: &str,
	forced_extension: &str,
	settings: ParseSettings,
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
	let new_session = match DocumentSession::new(path_str, password, forced_extension, settings) {
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
	tab.disk_fingerprint = new_fingerprint;
	// The buffer was rebuilt, and the caret above was re-derived from an anchor or a percentage
	// rather than carried over, so there is nothing honest to map an old mark onto. Clearing it
	// means the next copy says "set beginning of selection first" instead of copying whatever now
	// happens to sit at the old offset.
	tab.selection_mark.set(None);
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::{env, fs, path::PathBuf, process};

	use super::{position_announcement, read_fingerprint};

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

	/// The percentage is reported on its own for a document with no pages at all, exactly as it was
	/// before pages joined it - so a markdown or plain text reader hears the same "15%" they always
	/// have, rather than a page they do not have.
	#[test]
	fn announcement_without_a_page_is_the_bare_percentage() {
		assert_eq!(position_announcement(15, None), "15%");
	}

	#[test]
	fn announcement_with_a_page_names_both() {
		assert_eq!(position_announcement(15, Some(30)), "15%, page 30");
	}

	/// The percentage's own `%` must survive substitution: it rides in on the `%s` argument rather
	/// than sitting in the msgid, because `t()` is a plain catalog lookup and does no printf-style
	/// unescaping - a `%%` in the format string would reach the listener doubled.
	#[test]
	fn announcement_keeps_the_percentage_sign_intact() {
		assert!(position_announcement(7, Some(412)).starts_with("7%,"));
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
}
