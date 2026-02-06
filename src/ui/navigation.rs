use std::{rc::Rc, sync::Mutex};

use wxdragon::{prelude::*, translations::translate as t};

use super::{dialogs, document_manager::DocumentManager};
use crate::{config::ConfigManager, reader_core, session::NavigationResult, types::BookmarkFilterType};

#[derive(Clone, Copy)]
pub enum MarkerNavTarget {
	Section,
	Page,
	Heading(i32),
	Link,
	Table,
	Separator,
	List,
	ListItem,
}

enum NavFoundFormat {
	TextOnly,
	TextWithLevel,
	PageFormat,
	LinkFormat,
}

struct NavAnnouncements {
	not_supported: String,
	not_found_next: String,
	not_found_prev: String,
	format: NavFoundFormat,
}

fn nav_announcements(target: MarkerNavTarget, level_filter: i32) -> NavAnnouncements {
	match target {
		MarkerNavTarget::Section => NavAnnouncements {
			not_supported: t("No sections."),
			not_found_next: t("No next section"),
			not_found_prev: t("No previous section"),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Heading(level) => {
			if level_filter > 0 {
				let no_headings = t("No headings at level %d.");
				let no_next = t("No next heading at level %d.");
				let no_prev = t("No previous heading at level %d.");
				NavAnnouncements {
					not_supported: no_headings.replacen("%d", &level.to_string(), 1),
					not_found_next: no_next.replacen("%d", &level.to_string(), 1),
					not_found_prev: no_prev.replacen("%d", &level.to_string(), 1),
					format: NavFoundFormat::TextWithLevel,
				}
			} else {
				NavAnnouncements {
					not_supported: t("No headings."),
					not_found_next: t("No next heading."),
					not_found_prev: t("No previous heading."),
					format: NavFoundFormat::TextWithLevel,
				}
			}
		}
		MarkerNavTarget::Page => NavAnnouncements {
			not_supported: t("No pages."),
			not_found_next: t("No next page."),
			not_found_prev: t("No previous page."),
			format: NavFoundFormat::PageFormat,
		},
		MarkerNavTarget::Link => NavAnnouncements {
			not_supported: t("No links."),
			not_found_next: t("No next link."),
			not_found_prev: t("No previous link."),
			format: NavFoundFormat::LinkFormat,
		},
		MarkerNavTarget::List => NavAnnouncements {
			not_supported: t("No lists."),
			not_found_next: t("No next list."),
			not_found_prev: t("No previous list."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::ListItem => NavAnnouncements {
			not_supported: t("No list items."),
			not_found_next: t("No next list item."),
			not_found_prev: t("No previous list item."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Table => NavAnnouncements {
			not_supported: t("No tables."),
			not_found_next: t("No next table."),
			not_found_prev: t("No previous table."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Separator => NavAnnouncements {
			not_supported: t("No separators."),
			not_found_next: t("No next separator."),
			not_found_prev: t("No previous separator."),
			format: NavFoundFormat::TextOnly,
		},
	}
}

fn format_nav_found_message(
	ann: &NavAnnouncements,
	context_text: &str,
	context_index: i32,
	wrapped: bool,
	next: bool,
) -> String {
	let wrap_prefix =
		if wrapped { if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") } } else { String::new() };
	match ann.format {
		NavFoundFormat::TextOnly => format!("{wrap_prefix}{context_text}"),
		NavFoundFormat::TextWithLevel => {
			let template = t("%s Heading level %d");
			let message = template.replacen("%s", context_text, 1).replacen("%d", &context_index.to_string(), 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::PageFormat => {
			let template = t("Page %d: %s");
			let page_text = (context_index + 1).to_string();
			let message = template.replacen("%d", &page_text, 1).replacen("%s", context_text, 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::LinkFormat => {
			let message = format!("{context_text}{}", t(" link"));
			format!("{wrap_prefix}{message}")
		}
	}
}

fn apply_navigation_result(
	tab: &super::document_manager::DocumentTab,
	result: &NavigationResult,
	target: MarkerNavTarget,
	next: bool,
	live_region_label: StaticText,
) -> bool {
	let level_filter = match target {
		MarkerNavTarget::Heading(level) => level,
		_ => 0,
	};
	let ann = nav_announcements(target, level_filter);
	if result.not_supported {
		live_region::announce(live_region_label, &ann.not_supported);
		return false;
	}
	if !result.found {
		let message = if next { &ann.not_found_next } else { &ann.not_found_prev };
		live_region::announce(live_region_label, message);
		return false;
	}
	let mut context_text = result.marker_text.clone();
	if context_text.is_empty() {
		context_text = tab.session.get_line_text(result.offset);
	}
	let context_index = match target {
		MarkerNavTarget::Heading(_) => result.marker_level,
		MarkerNavTarget::Page => result.marker_index,
		_ => 0,
	};
	let message = format_nav_found_message(&ann, &context_text, context_index, result.wrapped, next);
	live_region::announce(live_region_label, &message);
	let offset = result.offset;
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_insertion_point(offset);
	tab.text_ctrl.show_position(offset);
	true
}

pub fn handle_history_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let (message, history_update) = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = tab.text_ctrl.get_insertion_point();
		let result = if forward {
			tab.session.history_go_forward(current_pos)
		} else {
			tab.session.history_go_back(current_pos)
		};
		if result.found {
			let message = if forward { t("Navigated to next position.") } else { t("Navigated to previous position.") };
			tab.text_ctrl.set_focus();
			tab.text_ctrl.set_insertion_point(result.offset);
			tab.text_ctrl.show_position(result.offset);
			tab.session.set_stable_position(result.offset);
			let (history, history_index) = tab.session.get_history();
			let history = history.to_vec();
			let path_str = tab.file_path.to_string_lossy().to_string();
			(message, Some((path_str, history, history_index)))
		} else {
			let message = if forward { t("No next position.") } else { t("No previous position.") };
			(message, None)
		}
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	if let Some((path_str, history, history_index)) = history_update {
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, &history, history_index);
	}
}

pub fn handle_marker_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	target: MarkerNavTarget,
	next: bool,
) {
	let wrap = config.lock().unwrap().get_app_bool("navigation_wrap", false);
	let mut dm = doc_manager.lock().unwrap();
	let history_update = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = tab.text_ctrl.get_insertion_point();
		let result = match target {
			MarkerNavTarget::Section => tab.session.navigate_section(current_pos, wrap, next),
			MarkerNavTarget::Page => tab.session.navigate_page(current_pos, wrap, next),
			MarkerNavTarget::Heading(level) => tab.session.navigate_heading(current_pos, wrap, next, level),
			MarkerNavTarget::Link => tab.session.navigate_link(current_pos, wrap, next),
			MarkerNavTarget::Table => tab.session.navigate_table(current_pos, wrap, next),
			MarkerNavTarget::Separator => tab.session.navigate_separator(current_pos, wrap, next),
			MarkerNavTarget::List => tab.session.navigate_list(current_pos, wrap, next),
			MarkerNavTarget::ListItem => tab.session.navigate_list_item(current_pos, wrap, next),
		};
		let target_offset = result.offset;
		if apply_navigation_result(tab, &result, target, next, live_region_label) {
			tab.session.check_and_record_history(target_offset);
			let (history, history_index) = tab.session.get_history();
			let history = history.to_vec();
			let path_str = tab.file_path.to_string_lossy().to_string();
			Some((path_str, history, history_index))
		} else {
			None
		}
	};
	drop(dm);
	if let Some((path_str, history, history_index)) = history_update {
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, &history, history_index);
	}
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
		let current_pos = tab.text_ctrl.get_insertion_point();
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
			tab.text_ctrl.set_focus();
			tab.text_ctrl.set_insertion_point(result.offset);
			tab.text_ctrl.show_position(result.offset);
			tab.session.check_and_record_history(result.offset);
			let note_text = result.marker_text;
			let line_text = tab.session.get_line_text(result.offset);
			let content_text = if note_text.is_empty() { line_text } else { format!("{note_text}, {line_text}") };
			let wrap_prefix = if result.wrapped {
				if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") }
			} else {
				String::new()
			};
			let bookmark_text = t("%s - Bookmark %d").replacen("%s", &content_text, 1).replacen(
				"%d",
				&(result.marker_index + 1).to_string(),
				1,
			);
			let message = format!("{wrap_prefix}{bookmark_text}");
			let (history, history_index) = tab.session.get_history();
			let history = history.to_vec();
			(message, Some((path_str, history, history_index)))
		} else {
			let message = if !has_items {
				if notes_only { t("No notes.") } else { t("No bookmarks.") }
			} else if next {
				if notes_only { t("No next note.") } else { t("No next bookmark.") }
			} else if notes_only {
				t("No previous note.")
			} else {
				t("No previous bookmark.")
			};
			(message, None)
		}
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	if let Some((path_str, history, history_index)) = history_update {
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, &history, history_index);
	}
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
		let current_pos = tab.text_ctrl.get_insertion_point();
		let selection = dialogs::show_bookmark_dialog(frame, &tab.session, &Rc::clone(config), current_pos, filter);
		let Some(selection) = selection else {
			return;
		};
		tab.text_ctrl.set_focus();
		tab.text_ctrl.set_insertion_point(selection.start);
		tab.text_ctrl.show_position(selection.start);
		tab.session.check_and_record_history(selection.start);
		let info = {
			let cfg = config.lock().unwrap();
			tab.session.bookmark_display_at_position(&cfg, selection.start)
		};
		let message = if info.found {
			let mut text = info.note;
			if text.is_empty() {
				text = info.snippet;
			}
			if text.is_empty() { t("Bookmark.") } else { text }
		} else {
			t("Bookmark.")
		};
		let (history, history_index) = tab.session.get_history();
		let history = history.to_vec();
		let path_str = tab.file_path.to_string_lossy().to_string();
		(message, Some((path_str, history, history_index)))
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	if let Some((path_str, history, history_index)) = history_update {
		let cfg = config.lock().unwrap();
		cfg.set_navigation_history(&path_str, &history, history_index);
	}
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
			let (start, end) = selected_range(tab.text_ctrl);
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
			let (start, end) = selected_range(tab.text_ctrl);
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
	let Some(note) =
		dialogs::show_note_entry_dialog(frame, &t("Bookmark Note"), &t("Enter bookmark note:"), &existing_note)
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
			let current_pos = tab.text_ctrl.get_insertion_point();
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
		let dialog = MessageDialog::builder(frame, &t("No note at the current position."), &t("View Note"))
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return;
	}
	dialogs::show_view_note_dialog(frame, &note);
}
