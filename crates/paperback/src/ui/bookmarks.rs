//! Setting, listing and jumping to bookmarks, and the notes attached to them.
//!
//! In a document that is only audio, a bookmark remembers where in the recording it was set as
//! well as its chapter. An audiobook's text is a blank line per chapter, so a text position alone
//! can only say which chapter a bookmark is in: in a chapterless MP3 every bookmark would land on
//! the same spot, and setting a second would find the first and remove it.

use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, reader_core, types::BookmarkFilterType};
use patois::t;
use wx_utils::format_duration_ms;
use wxdragon::prelude::*;

use super::{
	dialogs,
	document_manager::{DocumentManager, DocumentTab},
	navigation::{
		HistoryUpdate, doc_caret, doc_selected_range, move_to_offset_and_record_history, persist_navigation_history,
	},
};

/// How close to the playback time a bookmark has to be for toggling to treat it as the one here.
/// Playback moves on between one press and the next, so an exact match would never happen and the
/// second press would add a second bookmark instead of removing the first.
const TOGGLE_REACH_MS: u64 = 1500;

/// The playback time a bookmark set now should record, for a document that is only audio.
///
/// `None` for a document with text, whose bookmarks stay text positions. Before anything has
/// played the reader is at the start, which is time zero.
fn audio_bookmark_time(tab: &DocumentTab) -> Option<u64> {
	if !tab.session.handle().document().audio_only {
		return None;
	}
	let player = tab.audio_player.as_ref()?;
	Some(player.resume_point_ms().unwrap_or(0))
}

/// Seeks the recording to an audio bookmark. Moving the caret to the bookmark's chapter seeks the
/// audio to the start of that chapter, so this has to come after it.
fn seek_to_audio_bookmark(tab: &mut DocumentTab, audio_ms: u64) {
	if let Some(player) = tab.audio_player.as_mut() {
		player.seek_to_ms(audio_ms);
	}
}

/// What to announce for an audio bookmark: its note if it has one, and where in the recording.
fn audio_bookmark_label(note: &str, audio_ms: u64) -> String {
	let time = format_duration_ms(audio_ms);
	if note.is_empty() { time } else { format!("{note}, {time}") }
}

fn wrap_prefix(wrapped: bool, next: bool) -> String {
	if !wrapped {
		return String::new();
	}
	// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
	if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") }
}

fn bookmark_announcement(content: &str, number: usize) -> String {
	// TRANSLATORS: Announcement when landing on a bookmark; %s is the bookmark/line text, %d is the bookmark's 1-based index
	t("%s - Bookmark %d").replacen("%s", content, 1).replacen("%d", &number.to_string(), 1)
}

fn nothing_to_navigate_to(has_items: bool, next: bool, notes_only: bool) -> String {
	if !has_items {
		// TRANSLATORS: Announced when there are no bookmarks/notes at all to navigate to
		if notes_only { t("No notes.") } else { t("No bookmarks.") }
	} else if next {
		// TRANSLATORS: Announced when there is no next bookmark/note from the current position
		if notes_only { t("No next note.") } else { t("No next bookmark.") }
	} else if notes_only {
		// TRANSLATORS: Announced when there is no previous note from the current position
		t("No previous note.")
	} else {
		// TRANSLATORS: Announced when there is no previous bookmark from the current position
		t("No previous bookmark.")
	}
}

/// Next or previous bookmark in a document that is only audio, ordered by playback time.
fn navigate_audio_bookmark(
	tab: &mut DocumentTab,
	config: &ConfigManager,
	current_ms: u64,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> (String, Option<HistoryUpdate>) {
	let path_str = tab.file_path.to_string_lossy().to_string();
	let Some(hit) = reader_core::audio_bookmark_navigate(config, &path_str, current_ms, wrap, next, notes_only) else {
		let has_items = config
			.get_bookmarks(&path_str)
			.iter()
			.any(|bm| bm.audio_ms.is_some() && (!notes_only || !bm.note.is_empty()));
		return (nothing_to_navigate_to(has_items, next, notes_only), None);
	};
	let update = move_to_offset_and_record_history(tab, hit.start);
	seek_to_audio_bookmark(tab, hit.audio_ms);
	if config.get_app_bool("bookmark_sounds", true) {
		super::sounds::play_bookmark_sound(!hit.note.is_empty());
	}
	let label = audio_bookmark_label(&hit.note, hit.audio_ms);
	let message = format!("{}{}", wrap_prefix(hit.wrapped, next), bookmark_announcement(&label, hit.index + 1));
	(message, tab.track.then_some(update))
}

fn navigate_text_bookmark(
	tab: &mut DocumentTab,
	config: &Rc<Mutex<ConfigManager>>,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> (String, Option<HistoryUpdate>) {
	let current_pos = doc_caret(tab);
	let path_str = tab.file_path.to_string_lossy().to_string();
	let (result, has_items) = {
		let cfg = config.lock().unwrap();
		let bookmarks = cfg.get_bookmarks(&path_str);
		let has_items = if notes_only { bookmarks.iter().any(|bm| !bm.note.is_empty()) } else { !bookmarks.is_empty() };
		let result = if notes_only {
			tab.session.navigate_note(&cfg, current_pos, wrap, next)
		} else {
			tab.session.navigate_bookmark(&cfg, current_pos, wrap, next)
		};
		(result, has_items)
	};
	if !result.found {
		return (nothing_to_navigate_to(has_items, next, notes_only), None);
	}
	let update = move_to_offset_and_record_history(tab, result.offset);
	if config.lock().unwrap().get_app_bool("bookmark_sounds", true) {
		super::sounds::play_bookmark_sound(!result.marker_text.is_empty());
	}
	let note_text = result.marker_text;
	let line_text = tab.session.get_line_text(result.offset);
	let content_text = if note_text.is_empty() { line_text } else { format!("{note_text}, {line_text}") };
	let number = usize::try_from(result.marker_index + 1).unwrap_or(1);
	let message = format!("{}{}", wrap_prefix(result.wrapped, next), bookmark_announcement(&content_text, number));
	(message, tab.track.then_some(update))
}

pub fn handle_bookmark_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	next: bool,
	notes_only: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	let (message, history_update) = match audio_bookmark_time(tab) {
		Some(current_ms) => {
			let cfg = config.lock().unwrap();
			navigate_audio_bookmark(tab, &cfg, current_ms, wrap, next, notes_only)
		}
		None => navigate_text_bookmark(tab, config, wrap, next, notes_only),
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	persist_navigation_history(config, history_update.as_ref());
}

pub fn handle_bookmark_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	filter: BookmarkFilterType,
) {
	let mut dm = doc_manager.lock().unwrap();
	let (message, history_update) = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = doc_caret(tab);
		let selection = dialogs::show_bookmark_dialog(frame, &tab.session, &Rc::clone(config), current_pos, filter);
		let Some(selection) = selection else {
			return;
		};
		let update = move_to_offset_and_record_history(tab, selection.start);
		let message = if let Some(audio_ms) = selection.audio_ms {
			seek_to_audio_bookmark(tab, audio_ms);
			let path_str = tab.file_path.to_string_lossy().to_string();
			let note = config
				.lock()
				.unwrap()
				.get_bookmarks(&path_str)
				.into_iter()
				.find(|bm| bm.audio_ms == Some(audio_ms))
				.map(|bm| bm.note)
				.unwrap_or_default();
			audio_bookmark_label(&note, audio_ms)
		} else {
			let info = {
				let cfg = config.lock().unwrap();
				tab.session.bookmark_display_at_position(&cfg, selection.start)
			};
			let text =
				if info.found { if info.note.is_empty() { info.snippet } else { info.note } } else { String::new() };
			// TRANSLATORS: Fallback announcement when viewing a bookmark that has no note text or line snippet
			if text.is_empty() { t("Bookmark.") } else { text }
		};
		(message, tab.track.then_some(update))
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	persist_navigation_history(config, history_update.as_ref());
}

/// Where a new bookmark would go: the selection, and the playback time in an audio-only document.
fn bookmark_target(doc_manager: &Rc<Mutex<DocumentManager>>) -> Option<(i64, i64, String, Option<u64>)> {
	let mut dm = doc_manager.lock().unwrap();
	let tab = dm.active_tab_mut()?;
	let (start, end) = doc_selected_range(tab);
	Some((start, end, tab.file_path.to_string_lossy().to_string(), audio_bookmark_time(tab)))
}

pub fn handle_toggle_bookmark(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let Some((start, end, path_str, audio_ms)) = bookmark_target(doc_manager) else {
		return;
	};
	let cfg = config.lock().unwrap();
	let existed = if let Some(ms) = audio_ms {
		let here = cfg.audio_bookmark_near(&path_str, ms, TOGGLE_REACH_MS).and_then(|bm| bm.audio_ms);
		match here {
			Some(existing) => cfg.remove_audio_bookmark(&path_str, existing),
			None => cfg.add_audio_bookmark(&path_str, start, ms, ""),
		}
		here.is_some()
	} else {
		let existed =
			cfg.get_bookmarks(&path_str).iter().any(|bm| bm.audio_ms.is_none() && bm.start == start && bm.end == end);
		cfg.toggle_bookmark(&path_str, start, end, "");
		existed
	};
	cfg.flush();
	drop(cfg);
	// TRANSLATORS: Announced after toggling a bookmark at the current selection off/on
	let message = if existed { t("Bookmark removed.") } else { t("Bookmark added.") };
	live_region::announce(live_region_label, &message);
}

pub fn handle_bookmark_with_note(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let Some((start, end, path_str, audio_ms)) = bookmark_target(doc_manager) else {
		return;
	};
	let existing = {
		let cfg = config.lock().unwrap();
		match audio_ms {
			Some(ms) => cfg.audio_bookmark_near(&path_str, ms, TOGGLE_REACH_MS),
			None => cfg
				.get_bookmarks(&path_str)
				.into_iter()
				.find(|bm| bm.audio_ms.is_none() && bm.start == start && bm.end == end),
		}
	};
	let existing_note = existing.as_ref().map(|bm| bm.note.clone()).unwrap_or_default();
	// TRANSLATORS: Title of the dialog for adding or editing a bookmark note
	let bookmark_note_title = t("Bookmark Note");
	// TRANSLATORS: Prompt label in the bookmark note dialog asking the user to type their note
	let bookmark_note_prompt = t("Enter bookmark note:");
	let Some(note) =
		dialogs::show_note_entry_dialog(frame, &bookmark_note_title, &bookmark_note_prompt, &existing_note)
	else {
		return;
	};
	let cfg = config.lock().unwrap();
	match (audio_ms, existing.as_ref().and_then(|bm| bm.audio_ms)) {
		(Some(_), Some(existing_ms)) => cfg.update_audio_bookmark_note(&path_str, existing_ms, &note),
		(Some(ms), None) => cfg.add_audio_bookmark(&path_str, start, ms, &note),
		(None, _) if existing.is_some() => cfg.update_bookmark_note(&path_str, start, end, &note),
		(None, _) => cfg.add_bookmark(&path_str, start, end, &note),
	}
	cfg.flush();
	drop(cfg);
	// TRANSLATORS: Announced after saving a bookmark's note text
	live_region::announce(live_region_label, &t("Bookmark saved."));
}

pub fn handle_view_note_text(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let (current_pos, path_str, audio_ms) = {
		let dm = doc_manager.lock().unwrap();
		let Some(tab) = dm.active_tab() else {
			return;
		};
		(doc_caret(tab), tab.file_path.to_string_lossy().to_string(), audio_bookmark_time(tab))
	};
	let note = {
		let cfg = config.lock().unwrap();
		match audio_ms {
			Some(ms) => cfg.audio_bookmark_near(&path_str, ms, TOGGLE_REACH_MS).map(|bm| bm.note).unwrap_or_default(),
			None => reader_core::bookmark_note_at_position(&cfg, &path_str, current_pos),
		}
	};
	if note.is_empty() {
		// TRANSLATORS: Message shown when trying to view a bookmark note but the current position has none
		let dialog = MessageDialog::builder(frame, &t("No note at the current position."), &t("View Note"))
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return;
	}
	dialogs::show_view_note_dialog(frame, &note);
}
