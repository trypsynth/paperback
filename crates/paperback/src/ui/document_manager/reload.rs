//! Re-reading open documents: after the parse settings change, and when a file changes on disk. Split out of the main `DocumentManager` impl.

use std::{fs, path::Path, time::SystemTime};

use paperback_core::{
	config::ConfigManager, document::ParseSettings, parser::PASSWORD_REQUIRED_ERROR_PREFIX, session::DocumentSession,
};
use patois::t;
use wxdragon::prelude::*;

use super::{
	DocumentManager, DocumentTab,
	lifecycle::{build_document_load_error_message, prompt_for_password, show_error_dialog},
};
use crate::{
	text_window::TextWindow,
	ui::{
		readability::{
			ReadabilityStyle, apply_bg_color_to_ctrl, apply_foreground_color_to_ctrl, apply_readability_format_to_ctrl,
			build_font_from_readability, readability_style,
		},
		text_render::fill_text_ctrl_with_formatting,
	},
};

/// Change-detection stamp for an open document's file, compared on every frame activation and
/// tab switch. Metadata-only on purpose: `config::compute_document_hash` would read up to 2 MiB
/// from disk per check and still miss mid-file edits in files larger than that (it hashes only
/// head, tail and size), whereas a single `fs::metadata` call catches any completed write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct FileFingerprint {
	modified: SystemTime,
	len: u64,
}

pub(super) fn read_fingerprint(path: &Path) -> Option<FileFingerprint> {
	let meta = fs::metadata(path).ok()?;
	Some(FileFingerprint { modified: meta.modified().ok()?, len: meta.len() })
}

impl DocumentManager {
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

	/// Reloads the tab at `index` if its file changed on disk since it was last parsed and automatic reloading is on.
	pub fn reload_tab_if_changed(&mut self, index: usize) -> bool {
		self.reload_tab(index, false)
	}

	/// Re-reads the tab at `index` from disk. Returns true only when the tab content was
	/// actually replaced.
	///
	/// `forced` is the reader pressing F5, which skips the checks for a changed file and for the automatic reloading setting: turning that setting off asks not to be reloaded unasked, and F5 is asking. It does not reload an untracked tab, a help file or a source view, since a source view's title is applied once when it opens and a re-parse would leave "Source: ..." over an ordinary reading.
	///
	/// If the stored password no longer decrypts the file, prompts for a new one and retries once. Uses `try_lock` on the config: the caller may be a frame-activation handler running inside a nested modal event loop whose opener already holds the lock.
	pub fn reload_tab(&mut self, index: usize, forced: bool) -> bool {
		let Some(tab) = self.tabs.get(index) else {
			return false;
		};
		if !tab.track {
			return false;
		}
		let Some(current) = read_fingerprint(tab.local_path()) else {
			return false;
		};
		if !forced && tab.disk_fingerprint == Some(current) {
			return false;
		}
		let path_str = tab.file_path.to_string_lossy().to_string();
		let Ok(cfg) = self.config.try_lock() else {
			return false;
		};
		if !forced && !cfg.get_app_bool("auto_reload_documents", true) {
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
			tracing::info!(path = %path_str, forced, "document reloaded");
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

/// The parse-time toggles as the reader has them set.
pub(super) fn parse_settings(cfg: &ConfigManager) -> ParseSettings {
	ParseSettings {
		render_tables_inline: cfg.get_app_bool("render_tables_inline", true),
		join_pdf_paragraphs: cfg.get_app_bool("join_pdf_paragraphs", true),
		strip_running_text: cfg.get_app_bool("strip_running_text", true),
		markdown_dollar_math: cfg.get_app_bool("markdown_dollar_math", true),
	}
}

/// Builds a fresh session for `tab`'s file and refills its text control, restoring the reading
/// position. Returns the parse error and leaves the tab unchanged if the re-parse fails.
///
/// `path_str` is where the document's settings are kept, which for a document opened from a link
/// is the link; the file parsed is always the one the tab reads from.
fn reparse_tab_in_place(
	tab: &mut DocumentTab,
	path_str: &str,
	password: &str,
	forced_extension: &str,
	settings: ParseSettings,
	style: &ReadabilityStyle,
) -> Result<(), String> {
	let new_fingerprint = read_fingerprint(tab.local_path());
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
	let parse_path = tab.local_path().to_string_lossy().into_owned();
	let new_session = match DocumentSession::new(&parse_path, password, forced_extension, settings) {
		Ok(mut session) => {
			session.set_settings_path(path_str);
			session
		}
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
