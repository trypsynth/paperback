//! Setting, listing and jumping to bookmarks, and the notes attached to them.

use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, reader_core, types::BookmarkFilterType};
use patois::t;
use wxdragon::prelude::*;

use super::{
	dialogs,
	document_manager::DocumentManager,
	navigation::{doc_caret, doc_selected_range, move_to_offset_and_record_history, persist_navigation_history},
};

pub fn handle_bookmark_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	next: bool,
	notes_only: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let (message, history_update) = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = doc_caret(tab);
		let path_str = tab.file_path.to_string_lossy().to_string();
		let (result, has_items) = {
			let cfg = config.lock().unwrap();
			let bookmarks = cfg.get_bookmarks(&path_str);
			let has_items =
				if notes_only { bookmarks.iter().any(|bm| !bm.note.is_empty()) } else { !bookmarks.is_empty() };
			let result = if notes_only {
				tab.session.navigate_note(&cfg, current_pos, wrap, next)
			} else {
				tab.session.navigate_bookmark(&cfg, current_pos, wrap, next)
			};
			drop(cfg);
			(result, has_items)
		};
		if result.found {
			let update = move_to_offset_and_record_history(tab, result.offset);
			if config.lock().unwrap().get_app_bool("bookmark_sounds", true) {
				super::sounds::play_bookmark_sound(!result.marker_text.is_empty());
			}
			let note_text = result.marker_text;
			let line_text = tab.session.get_line_text(result.offset);
			let content_text = if note_text.is_empty() { line_text } else { format!("{note_text}, {line_text}") };
			let wrap_prefix = if result.wrapped {
				// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
				if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") }
			} else {
				String::new()
			};
			// TRANSLATORS: Announcement when landing on a bookmark; %s is the bookmark/line text, %d is the bookmark's 1-based index
			let bookmark_text = t("%s - Bookmark %d").replacen("%s", &content_text, 1).replacen(
				"%d",
				&(result.marker_index + 1).to_string(),
				1,
			);
			let message = format!("{wrap_prefix}{bookmark_text}");
			let history_update = tab.track.then_some(update);
			(message, history_update)
		} else {
			let message = if !has_items {
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
			};
			(message, None)
		}
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
		let info = {
			let cfg = config.lock().unwrap();
			tab.session.bookmark_display_at_position(&cfg, selection.start)
		};
		let message = if info.found {
			let mut text = info.note;
			if text.is_empty() {
				text = info.snippet;
			}
			// TRANSLATORS: Fallback announcement when viewing a bookmark that has no note text or line snippet
			if text.is_empty() { t("Bookmark.") } else { text }
		} else {
			// TRANSLATORS: Fallback announcement when viewing a bookmark that has no note text or line snippet
			t("Bookmark.")
		};
		let history_update = tab.track.then_some(update);
		(message, history_update)
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	persist_navigation_history(config, history_update.as_ref());
}

pub fn handle_toggle_bookmark(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (start, end, path_str) = {
		let mut dm = doc_manager.lock().unwrap();
		let (start, end, path_str) = {
			let Some(tab) = dm.active_tab_mut() else {
				return;
			};
			let (start, end) = doc_selected_range(tab);
			let path_str = tab.file_path.to_string_lossy().to_string();
			(start, end, path_str)
		};
		drop(dm);
		(start, end, path_str)
	};
	let cfg = config.lock().unwrap();
	let existed = cfg.get_bookmarks(&path_str).iter().any(|bm| bm.start == start && bm.end == end);
	cfg.toggle_bookmark(&path_str, start, end, "");
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
	let (start, end, path_str) = {
		let mut dm = doc_manager.lock().unwrap();
		let (start, end, path_str) = {
			let Some(tab) = dm.active_tab_mut() else {
				return;
			};
			let (start, end) = doc_selected_range(tab);
			let path_str = tab.file_path.to_string_lossy().to_string();
			(start, end, path_str)
		};
		drop(dm);
		(start, end, path_str)
	};
	let existing = {
		let cfg = config.lock().unwrap();
		cfg.get_bookmarks(&path_str).into_iter().find(|bm| bm.start == start && bm.end == end)
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
	if existing.is_some() {
		cfg.update_bookmark_note(&path_str, start, end, &note);
	} else {
		cfg.add_bookmark(&path_str, start, end, &note);
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
	let (current_pos, path_str) = {
		let dm = doc_manager.lock().unwrap();
		let (current_pos, path_str) = {
			let Some(tab) = dm.active_tab() else {
				return;
			};
			let current_pos = doc_caret(tab);
			let path_str = tab.file_path.to_string_lossy().to_string();
			(current_pos, path_str)
		};
		drop(dm);
		(current_pos, path_str)
	};
	let note = {
		let cfg = config.lock().unwrap();
		reader_core::bookmark_note_at_position(&cfg, &path_str, current_pos)
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
