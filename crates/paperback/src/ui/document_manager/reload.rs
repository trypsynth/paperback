use std::{fs, path::Path, time::SystemTime};

use paperback_core::{
	config::ConfigManager,
	document::{DocPos, ParseSettings},
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	session::DocumentSession,
};
use patois::t;
use wxdragon::prelude::*;

use super::{
	DocumentManager, DocumentTab,
	lifecycle::{build_document_load_error_message, prompt_for_password, show_error_dialog},
	ocr::{is_pdf, restore_saved_pages, saved_pages_of},
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
	working_copy::WorkingCopy,
};

/// Metadata-only, since `config::compute_document_hash` reads up to 2 MiB per check (every activation and tab switch) and still misses mid-file edits in larger files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct FileFingerprint {
	modified: SystemTime,
	len: u64,
}

impl FileFingerprint {
	pub(super) const fn len(self) -> u64 {
		self.len
	}
}

pub(super) fn read_fingerprint(path: &Path) -> Option<FileFingerprint> {
	let meta = fs::metadata(path).ok()?;
	Some(FileFingerprint { modified: meta.modified().ok()?, len: meta.len() })
}

impl DocumentManager {
	/// Re-parses rather than transforming in place so every format's table rendering stays identical via the shared parse-time helper.
	pub fn apply_parse_settings(&mut self, settings: ParseSettings) {
		let (style, parse_inputs) = {
			let cfg = self.config.lock().unwrap();
			let parse_inputs: Vec<(String, String)> = self
				.tabs
				.iter()
				.map(|tab| {
					let path_str = tab.file_path.to_string_lossy();
					(cfg.get_document_password(&path_str), cfg.get_document_format(&path_str))
				})
				.collect();
			(readability_style(&cfg), parse_inputs)
		};
		for (tab, (password, forced_extension)) in self.tabs.iter_mut().zip(parse_inputs) {
			let _ = reparse_tab_in_place(tab, &password, &forced_extension, settings, &style);
		}
	}

	pub fn reload_link(&mut self, index: usize, copy: WorkingCopy) -> bool {
		let Some(tab) = self.tabs.get_mut(index) else {
			return false;
		};
		let previous = (tab.working_copy.replace(copy), tab.disk_fingerprint);
		if !self.reload_tab(index, true) {
			let tab = &mut self.tabs[index];
			(tab.working_copy, tab.disk_fingerprint) = previous;
			return false;
		}
		let tab = &self.tabs[index];
		if let (Some(copy), Some(size)) = (&tab.working_copy, tab.disk_fingerprint.map(FileFingerprint::len)) {
			let link = tab.file_path.to_string_lossy();
			let name = copy.file_name();
			let config = self.config.lock().unwrap();
			if config.get_document_download_size(&link) != Some(size)
				|| config.get_document_download_name(&link).as_deref() != Some(name.as_str())
			{
				config.set_document_download(&link, &name, size);
				config.flush();
			}
		}
		true
	}

	pub fn reload_tab_if_changed(&mut self, index: usize) -> bool {
		self.reload_tab(index, false)
	}

	/// `forced` (F5) skips the changed-file and auto-reload checks, since turning that setting off asks not to be reloaded unasked and F5 is asking.
	pub fn reload_tab(&mut self, index: usize, forced: bool) -> bool {
		let Some(tab) = self.tabs.get(index) else {
			return false;
		};
		// A source view's title is applied once at open, so a re-parse would leave "Source: ..." over an ordinary reading.
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
		if forced {
			// Or its pages would land in the new text; quietly, because its page count would describe the replaced text.
			self.stop_ocr_at(index, true);
		}
		// `try_lock` because a frame-activation caller may be inside a nested modal loop whose opener holds the lock.
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
		let reloaded = match reparse_tab_in_place(tab, &password, &forced_extension, settings, &style) {
			Ok(()) => true,
			Err(err) if err.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
				// Recorded before the prompt so a re-entrant call during its modal loop sees the file as unchanged and skips a second prompt.
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
		// Dismissing keeps the old content without an error, since the reload wasn't the reader's doing.
		let Some(password) = prompt_for_password(&self.notebook) else {
			return false;
		};
		let tab = &mut self.tabs[index];
		match reparse_tab_in_place(tab, &password, forced_extension, settings, style) {
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

pub(super) fn parse_settings(cfg: &ConfigManager) -> ParseSettings {
	ParseSettings {
		render_tables_inline: cfg.get_app_bool("render_tables_inline", true),
		join_pdf_paragraphs: cfg.get_app_bool("join_pdf_paragraphs", true),
		strip_running_text: cfg.get_app_bool("strip_running_text", true),
		markdown_dollar_math: cfg.get_app_bool("markdown_dollar_math", true),
	}
}

fn reparse_tab_in_place(
	tab: &mut DocumentTab,
	password: &str,
	forced_extension: &str,
	settings: ParseSettings,
	style: &ReadabilityStyle,
) -> Result<(), String> {
	let new_fingerprint = read_fingerprint(tab.local_path());
	let current_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
	let pos = current_pos.min(tab.session.handle().document().buffer.end());
	// Uses the full `id_positions` key, not `nearest_fragment_before`, which strips EPUB's "path#" prefix and so fails the lookup after reparsing.
	let stable_anchor = {
		let id_positions = &tab.session.handle().document().id_positions;
		id_positions
			.iter()
			.filter(|&(_, &off)| off <= pos)
			.max_by_key(|&(_, &off)| off)
			.map(|(key, &anchor_off)| (key.clone(), pos - anchor_off))
	};
	let fallback_percent = tab.session.get_status_info(current_pos).percentage;
	let parse_path = tab.local_path().to_string_lossy().into_owned();
	let new_session = match DocumentSession::new(&parse_path, password, forced_extension, settings) {
		Ok(session) => session,
		Err(err) => {
			tracing::error!(path = %parse_path, error = %err, "failed to re-parse document");
			return Err(err);
		}
	};
	tab.session = new_session;
	let local_path = tab.local_path().to_path_buf();
	restore_saved_pages(
		&mut tab.session,
		&saved_pages_of(&local_path),
		is_pdf(&local_path) && settings.join_pdf_paragraphs,
	);
	// TODO(windowing): still whole-document on every reparse; should load a window centred on `restored_pos` instead.
	let doc_len = tab.session.document_len();
	let slice = tab.session.get_window(DocPos::ZERO, doc_len.as_pos());
	tab.window = TextWindow::of(&slice);
	fill_text_ctrl_with_formatting(tab.text_ctrl, &slice, &tab.window);
	if let Some(font) = build_font_from_readability(&style.rf) {
		tab.text_ctrl.set_font(&font);
	}
	apply_foreground_color_to_ctrl(tab.text_ctrl.widget(), style.rf.color);
	apply_bg_color_to_ctrl(tab.text_ctrl.widget(), style.bg_color);
	apply_readability_format_to_ctrl(
		tab.text_ctrl.widget(),
		style.line_spacing,
		style.paragraph_spacing,
		style.letter_spacing,
		style.text_alignment,
	);
	tab.panel.layout();
	let restored_pos = if let Some((ref key, within)) = stable_anchor {
		match tab.session.handle().document().id_positions.get(key) {
			Some(&new_anchor_off) => new_anchor_off + within,
			None => tab.session.position_from_percent(fallback_percent),
		}
	} else {
		tab.session.position_from_percent(fallback_percent)
	};
	let restored = tab.window.to_local(restored_pos);
	tab.text_ctrl.set_insertion_point(restored);
	tab.text_ctrl.show_position(restored);
	tab.disk_fingerprint = new_fingerprint;
	// The caret was re-derived rather than carried over, so there's nothing honest to map the old mark onto.
	tab.selection_mark.set(None);
	Ok(())
}
