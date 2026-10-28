use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, document::DocPos, session::NavigationResult};
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::{DocumentManager, DocumentTab},
	reader_ctrl::ReaderCtrl,
	text_render::reload_window_around,
};
use crate::{audio_player::AudioPlayer, text_window::CtrlPos};

mod announce;

#[cfg(not(target_os = "macos"))]
pub use announce::announce_for_selection_command;
pub use announce::{announce, announce_for_command, page_announcement};
use announce::{format_nav_found_message, nav_announcements};

/// Owned so it outlives the `DocumentManager` lock: built while a tab is borrowed, persisted after the lock drops.
pub(super) type HistoryUpdate = (String, Vec<DocPos>, usize);

// For moves within the existing history (`history_go_forward`/`history_go_back`), which record no new entry.
fn tracked_history_update(tab: &DocumentTab) -> Option<HistoryUpdate> {
	if !tab.track {
		return None;
	}
	let (history, history_index) = tab.session.get_history();
	Some((tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index))
}

fn record_history(tab: &mut DocumentTab, from: DocPos, to: DocPos) -> HistoryUpdate {
	tab.session.record_jump(from, to);
	let (history, history_index) = tab.session.get_history();
	(tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index)
}

/// The control's own insertion point is relative to the loaded window, not the document.
pub fn doc_caret(tab: &DocumentTab) -> DocPos {
	tab.window.to_doc(tab.text_ctrl.get_insertion_point())
}

/// By time, not text: whole-file recordings have one blank line of text per file, so a text percentage picks a file by count rather than duration.
pub fn seek_audio_to_percent(tab: &mut DocumentTab, percent: i32) -> Option<DocPos> {
	let target_ms = tab.session.audio_elapsed_for_percent(percent)?;
	let player = tab.audio_player.as_mut()?;
	player.seek_to_ms(target_ms);
	let cursor = player.timeline().cursor_at_elapsed(target_ms)?;
	player.timeline().clip(cursor.clip).map(|clip| clip.start)
}

/// Audio progress where known, since an audiobook's text spine gives every file an equal share however long it runs.
pub fn reading_percent(tab: &DocumentTab, position: DocPos) -> i32 {
	let audio_percent = tab
		.audio_player
		.as_ref()
		.and_then(AudioPlayer::resume_point_ms)
		.and_then(|elapsed_ms| tab.session.audio_progress_percent(elapsed_ms));
	audio_percent.unwrap_or_else(|| tab.session.get_status_info(position).percentage).clamp(0, 100)
}

pub fn doc_selected_range(tab: &DocumentTab) -> (DocPos, DocPos) {
	let (start, end) = selected_range(tab.text_ctrl);
	(tab.window.to_doc(start), tab.window.to_doc(end))
}

/// Leaves audio alone, for audio driving the caret (`handle_seek_audio`), where re-seeking would lose a mid-clip or spilled-over position.
pub(super) fn set_caret_to_doc_offset(tab: &mut DocumentTab, offset: DocPos) {
	if tab.window.needs_reload_for(offset, tab.session.document_len()) {
		reload_window_around(tab, offset, "caret jump");
	}
	let local = tab.window.to_local(offset);
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(local);
	tab.text_ctrl.show_position(local);
}

// Every caret jump routes through here so no caller has to think about the loaded window or audio.
fn jump_to_doc_offset(tab: &mut DocumentTab, offset: DocPos) {
	set_caret_to_doc_offset(tab, offset);
	seek_audio_to_position(tab, offset);
}

/// Without `highlight` nothing is selected, so a screen reader reads the line rather than announcing a selected string; the caret goes to `start` because screen readers speak the character after it.
/// Only `start` is checked against the window, since a match is far shorter than `RELOAD_MARGIN`.
pub fn reveal_doc_range(tab: &mut DocumentTab, start: DocPos, end: DocPos, highlight: bool) {
	if tab.window.needs_reload_for(start, tab.session.document_len()) {
		reload_window_around(tab, start, "selection jump");
	}
	let local_start = tab.window.to_local(start);
	let local_end = tab.window.to_local(end);
	tab.text_ctrl.set_focus();
	if highlight {
		tab.text_ctrl.set_selection(local_start, local_end);
	} else {
		// Also clears the highlight from a match found while the setting was still on.
		tab.text_ctrl.set_insertion_point(local_start);
	}
	tab.text_ctrl.show_position(local_start);
	seek_audio_to_position(tab, start);
}

/// The caret is read before it moves, because going back must return there (#835: anything else landed at the top of the last section jumped into).
pub fn move_to_offset_and_record_history(tab: &mut DocumentTab, offset: DocPos) -> HistoryUpdate {
	let from = doc_caret(tab);
	jump_to_doc_offset(tab, offset);
	record_history(tab, from, offset)
}

fn seek_audio_to_position(tab: &mut DocumentTab, offset: DocPos) {
	let offset = offset.min(tab.session.handle().document().buffer.end());
	if let Some(player) = tab.audio_player.as_mut() {
		player.seek_to_position(offset);
	}
}

pub fn persist_navigation_history(config: &Rc<Mutex<ConfigManager>>, update: Option<&HistoryUpdate>) {
	if let Some((path_str, history, history_index)) = update {
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(path_str, history, *history_index);
	}
}

#[derive(Clone, Copy)]
pub enum MarkerNavTarget {
	Section,
	Page,
	Heading(i32),
	Link,
	Table,
	Formula,
	Separator,
	List,
	ListItem,
	Image,
	Figure,
}

fn apply_navigation_result(
	tab: &mut DocumentTab,
	result: &NavigationResult,
	target: MarkerNavTarget,
	next: bool,
	live_region_label: StaticText,
	from_keyboard: bool,
) -> bool {
	let level_filter = match target {
		MarkerNavTarget::Heading(level) => level,
		_ => 0,
	};
	let ann = nav_announcements(target, level_filter);
	if result.not_supported {
		announce_for_command(live_region_label, from_keyboard, &ann.not_supported);
		return false;
	}
	if !result.found {
		let message = if next { &ann.not_found_next } else { &ann.not_found_prev };
		announce_for_command(live_region_label, from_keyboard, message);
		return false;
	}
	let mut context_text = match target {
		// PDF page markers carry a made-up "Page N" label, which would read the number twice beside the formatter's own.
		MarkerNavTarget::Page => tab.session.first_content_line_after(result.offset),
		_ => result.marker_text.clone(),
	};
	if context_text.is_empty() && !matches!(target, MarkerNavTarget::Page) {
		context_text = tab.session.get_line_text(result.offset);
	}
	let context_index = match target {
		MarkerNavTarget::Heading(_) => result.marker_level,
		MarkerNavTarget::Page | MarkerNavTarget::Image | MarkerNavTarget::Figure => result.marker_index,
		_ => 0,
	};
	let message = format_nav_found_message(&ann, &context_text, context_index, result.wrapped, next);
	announce_for_command(live_region_label, from_keyboard, &message);
	jump_to_doc_offset(tab, result.offset);
	true
}

pub fn handle_history_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
	from_keyboard: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let (message, history_update) = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = doc_caret(tab);
		let result = if forward {
			tab.session.history_go_forward(current_pos)
		} else {
			tab.session.history_go_back(current_pos)
		};
		if result.found {
			// TRANSLATORS: Announced when moving forward/backward through the caret position history
			let message = if forward { t("Navigated to next position.") } else { t("Navigated to previous position.") };
			jump_to_doc_offset(tab, result.offset);
			let history_update = tracked_history_update(tab);
			(message, history_update)
		} else {
			// TRANSLATORS: Announced when there is no next/previous position in the caret position history
			let message = if forward { t("No next position.") } else { t("No previous position.") };
			(message, None)
		}
	};
	drop(dm);
	announce_for_command(live_region_label, from_keyboard, &message);
	persist_navigation_history(config, history_update.as_ref());
}

pub fn handle_marker_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	target: MarkerNavTarget,
	next: bool,
	from_keyboard: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let history_update = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = doc_caret(tab);
		let result = match target {
			MarkerNavTarget::Section => tab.session.navigate_section(current_pos, wrap, next),
			MarkerNavTarget::Page => tab.session.navigate_page(current_pos, wrap, next),
			MarkerNavTarget::Heading(level) => tab.session.navigate_heading(current_pos, wrap, next, level),
			MarkerNavTarget::Link => tab.session.navigate_link(current_pos, wrap, next),
			MarkerNavTarget::Table => tab.session.navigate_table(current_pos, wrap, next),
			MarkerNavTarget::Formula => tab.session.navigate_formula(current_pos, wrap, next),
			MarkerNavTarget::Separator => tab.session.navigate_separator(current_pos, wrap, next),
			MarkerNavTarget::List => tab.session.navigate_list(current_pos, wrap, next),
			MarkerNavTarget::ListItem => tab.session.navigate_list_item(current_pos, wrap, next),
			MarkerNavTarget::Image => tab.session.navigate_image(current_pos, wrap, next),
			MarkerNavTarget::Figure => tab.session.navigate_figure(current_pos, wrap, next),
		};
		let target_offset = result.offset;
		if apply_navigation_result(tab, &result, target, next, live_region_label, from_keyboard) {
			let update = record_history(tab, current_pos, target_offset);
			tab.track.then_some(update)
		} else {
			None
		}
	};
	drop(dm);
	persist_navigation_history(config, history_update.as_ref());
}

pub fn handle_container_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	to_end: bool,
	from_keyboard: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let history_update = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = doc_caret(tab);
		let result = tab.session.navigate_container(current_pos, to_end);
		if result.not_supported {
			// TRANSLATORS: Announced when the document has no containers (lists/tables) to navigate
			announce_for_command(live_region_label, from_keyboard, t("No containers."));
			None
		} else if !result.found {
			// TRANSLATORS: Announced when the caret is not currently inside a container (list/table)
			announce_for_command(live_region_label, from_keyboard, t("Not in a container."));
			None
		} else {
			let offset = result.offset;
			let line = tab.session.get_line_text(offset);
			let message = if line.trim().is_empty() {
				// TRANSLATORS: Announced when jumping to the start/end of the container (list/table) the caret is inside, and the target line is blank
				if to_end { t("Past end of container.") } else { t("Start of container.") }
			} else {
				line
			};
			announce_for_command(live_region_label, from_keyboard, &message);
			let update = move_to_offset_and_record_history(tab, offset);
			tab.track.then_some(update)
		}
	};
	drop(dm);
	persist_navigation_history(config, history_update.as_ref());
}

pub fn selected_range(text_ctrl: ReaderCtrl) -> (CtrlPos, CtrlPos) {
	let (start, end) = text_ctrl.get_selection();
	if start == end {
		let pos = text_ctrl.get_insertion_point();
		(pos, pos)
	} else if start <= end {
		(start, end)
	} else {
		(end, start)
	}
}
