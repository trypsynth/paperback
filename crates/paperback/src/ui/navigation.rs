use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, session::NavigationResult};
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::{DocumentManager, DocumentTab},
	text_render::reload_window_around,
};
use crate::audio_player::AudioPlayer;

mod announce;

pub use announce::{announce, announce_for_command, page_announcement};
use announce::{format_nav_found_message, nav_announcements};

/// `(file_path, history_positions, history_index)` snapshot to persist via
/// [`persist_navigation_history`]. Kept as an owned tuple (rather than borrowing
/// from the tab) so it can outlive the `DocumentManager` lock: callers build it
/// while a tab is borrowed, drop the lock, then persist it against `config`.
pub(super) type HistoryUpdate = (String, Vec<i64>, usize);

/// Snapshots `tab`'s current position history as a [`HistoryUpdate`], if `tab.track`
/// is set. Use this when the caller has already updated the history itself (e.g. via
/// `history_go_forward`/`history_go_back`, which navigate *within* existing history
/// rather than recording a new entry). For the common "record a new position, then
/// snapshot" case, use [`record_history`] or [`move_to_offset_and_record_history`]
/// instead.
fn tracked_history_update(tab: &DocumentTab) -> Option<HistoryUpdate> {
	if !tab.track {
		return None;
	}
	let (history, history_index) = tab.session.get_history();
	Some((tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index))
}

/// Records a jump from `from` to `to` in `tab`'s position history and returns the resulting
/// snapshot unconditionally. Callers that should only persist it when `tab.track` is set
/// (most callers) should gate with `tab.track.then_some(update)`.
fn record_history(tab: &mut DocumentTab, from: i64, to: i64) -> HistoryUpdate {
	tab.session.record_jump(from, to);
	let (history, history_index) = tab.session.get_history();
	(tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index)
}

/// Reads the caret's document-absolute position, translating it out of `tab`'s currently
/// loaded window. Use this instead of `tab.text_ctrl.get_insertion_point()` directly - that
/// method only knows about whatever window is currently loaded, not the document as a whole.
pub fn doc_caret(tab: &DocumentTab) -> i64 {
	tab.window.to_doc(tab.text_ctrl.get_insertion_point())
}

/// Seeks `tab`'s audio to `percent` of the recording, reporting the text position that lands on.
///
/// `None` when the document has no audio, or its file lengths are not all known, leaving the
/// caller to fall back to the character-count mapping. Seeking by time rather than by text
/// position is the whole point: the text of a bundle of whole-file recordings is one blank line
/// per file, so mapping a percentage through it lands on a file boundary chosen by file count
/// rather than by how long any of them runs.
pub fn seek_audio_to_percent(tab: &mut DocumentTab, percent: i32) -> Option<i64> {
	let target_ms = tab.session.audio_elapsed_for_percent(percent)?;
	let player = tab.audio_player.as_mut()?;
	player.seek_to_ms(target_ms);
	let cursor = player.timeline().cursor_at_elapsed(target_ms)?;
	let start = player.timeline().clip(cursor.clip).map(|clip| clip.start)?;
	i64::try_from(start).ok()
}

/// How far the reader has got through `tab`, as a whole percent.
///
/// For a document with audio whose files all have a known length, this is progress through the
/// recording, taken from where the player actually is. That is what a listener means by how far
/// in they are: the character-count percentage measures the text spine, which for an audiobook
/// is a handful of chapter headings and, for a bundle of whole-file recordings, one blank line
/// per file, so it reports every file as an equal share however long it runs.
///
/// Falls back to the character-count percentage for text, for audio the player has not started,
/// and for audio whose lengths could not all be established.
pub fn reading_percent(tab: &DocumentTab, position: i64) -> i32 {
	let audio_percent = tab
		.audio_player
		.as_ref()
		.and_then(AudioPlayer::resume_point_ms)
		.and_then(|elapsed_ms| tab.session.audio_progress_percent(elapsed_ms));
	audio_percent.unwrap_or_else(|| tab.session.get_status_info(position).percentage).clamp(0, 100)
}

/// The current selection (or, absent one, the caret collapsed to a zero-length range), as
/// document-absolute positions. Use this instead of `selected_range(tab.text_ctrl)` directly
/// for anything that persists the result (bookmarks) or otherwise treats it as a document
/// position rather than a ctrl-local one.
pub fn doc_selected_range(tab: &DocumentTab) -> (i64, i64) {
	let (start, end) = selected_range(tab.text_ctrl);
	(tab.window.to_doc(start), tab.window.to_doc(end))
}

/// Moves the caret to document-absolute `offset`, focusing the document and reloading `tab`'s
/// window first if `offset` falls outside it (see `DocumentTab::window`). Does not touch audio -
/// use [`jump_to_doc_offset`] for the common case where a caret jump should also drive audio to
/// match. This lower-level form exists for the reverse flow (audio driving the caret, e.g.
/// `handle_seek_audio`), where re-seeking audio from the position we just derived it from would
/// undo whatever precision produced that position (a mid-clip offset, or one "spilled" into the
/// next file).
pub(super) fn set_caret_to_doc_offset(tab: &mut DocumentTab, offset: i64) {
	if tab.window.needs_reload_for(offset, tab.session.document_len()) {
		reload_window_around(tab, offset, "caret jump");
	}
	let local = tab.window.to_local(offset);
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(local);
	tab.text_ctrl.show_position(local);
}

/// Moves the caret to document-absolute `offset`, focuses the document, shows the position,
/// and keeps audio in sync - reloading `tab`'s window first if `offset` falls outside it (see
/// `DocumentTab::window`). This is the one chokepoint every caret-jump in the app routes
/// through, directly or via [`move_to_offset_and_record_history`], so callers never need to
/// think about the window themselves.
fn jump_to_doc_offset(tab: &mut DocumentTab, offset: i64) {
	set_caret_to_doc_offset(tab, offset);
	seek_audio_to_position(tab, offset);
}

/// Reveals the document-absolute range `[start, end)` - a Find match - focusing the document and
/// reloading `tab`'s window first if `start` falls outside it (see `DocumentTab::window`). The
/// range form of [`jump_to_doc_offset`], for jumps to a span of text rather than to a single
/// position. Keeps audio in sync with `start`, like `jump_to_doc_offset`.
///
/// With `highlight` the range is selected, which is what a sighted reader expects: the match is
/// picked out on screen. Without it only the caret is moved, to the end of the range, so nothing
/// is selected and a screen reader reads the line around the caret as ordinary text instead of
/// announcing a selected string. The caret lands at `end` either way, since that is where
/// selecting the range leaves it.
///
/// Only `start` is checked against the window: a match is expected to be far shorter than a
/// window's `RELOAD_MARGIN`, so `end` lands safely inside whatever window `start` triggers.
pub fn reveal_doc_range(tab: &mut DocumentTab, start: i64, end: i64, highlight: bool) {
	if tab.window.needs_reload_for(start, tab.session.document_len()) {
		reload_window_around(tab, start, "selection jump");
	}
	let local_start = tab.window.to_local(start);
	let local_end = tab.window.to_local(end);
	tab.text_ctrl.set_focus();
	if highlight {
		tab.text_ctrl.set_selection(local_start, local_end);
	} else {
		// Moving the insertion point collapses any selection, so this also takes the highlight
		// off a match found while the setting was still on.
		tab.text_ctrl.set_insertion_point(local_end);
	}
	tab.text_ctrl.show_position(local_start);
	seek_audio_to_position(tab, start);
}

/// Moves the caret to `offset`, focuses the document, shows the position, and records the
/// jump in the session's position history. Returns the resulting history snapshot
/// unconditionally; gate on `tab.track` at the call site if the update should only be
/// persisted when history tracking is enabled for this tab.
///
/// Where the caret was is read before it moves, because that is the position going back has
/// to return to. #835: recording it from anywhere else lands the reader at the top of
/// whatever section they had jumped into last, rather than at the line they left.
pub fn move_to_offset_and_record_history(tab: &mut DocumentTab, offset: i64) -> HistoryUpdate {
	let from = doc_caret(tab);
	jump_to_doc_offset(tab, offset);
	record_history(tab, from, offset)
}

/// Keeps a document's audio in step with the caret, called after every jump that moves the
/// insertion point. A no-op for documents with no recorded audio.
fn seek_audio_to_position(tab: &mut DocumentTab, offset: i64) {
	if let Some(player) = tab.audio_player.as_mut() {
		player.seek_to_position(usize::try_from(offset).unwrap_or(0));
	}
}

/// Persists a [`HistoryUpdate`] built by [`move_to_offset_and_record_history`] (or
/// similar) to `config`. No-op if `update` is `None`, so callers can pass the gated
/// result directly.
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
		// PDFs fabricate a "Page N" label on page markers; announcing it alongside the page
		// number the formatter adds would read the number twice, so use the page's first line
		// of real content instead.
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

/// Navigate relative to the container (list/table) the caret is currently inside: `to_end` jumps
/// just past its end, otherwise to its start. Announces "Not in a container." when the caret is
/// not inside any container.
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

pub fn selected_range(text_ctrl: TextCtrl) -> (i64, i64) {
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
