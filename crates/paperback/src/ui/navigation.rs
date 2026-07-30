use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, reader_core, session::NavigationResult, types::BookmarkFilterType};
use patois::t;
use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager};

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
	Image,
	Figure,
}

enum NavFoundFormat {
	TextOnly,
	TextWithLevel,
	PageFormat,
	LinkFormat,
	ImageFormat,
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
			// TRANSLATORS: Announced when the document has no sections to navigate
			not_supported: t("No sections."),
			// TRANSLATORS: Announced when there is no next section from the current position
			not_found_next: t("No next section"),
			// TRANSLATORS: Announced when there is no previous section from the current position
			not_found_prev: t("No previous section"),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Heading(level) => {
			if level_filter > 0 {
				// TRANSLATORS: Announced when the document has no headings at the given level; %d is the heading level number
				let no_headings = t("No headings at level %d.");
				// TRANSLATORS: Announced when there is no next heading at the given level; %d is the heading level number
				let no_next = t("No next heading at level %d.");
				// TRANSLATORS: Announced when there is no previous heading at the given level; %d is the heading level number
				let no_prev = t("No previous heading at level %d.");
				NavAnnouncements {
					not_supported: no_headings.replacen("%d", &level.to_string(), 1),
					not_found_next: no_next.replacen("%d", &level.to_string(), 1),
					not_found_prev: no_prev.replacen("%d", &level.to_string(), 1),
					format: NavFoundFormat::TextWithLevel,
				}
			} else {
				NavAnnouncements {
					// TRANSLATORS: Announced when the document has no headings at all (no level filter applied)
					not_supported: t("No headings."),
					// TRANSLATORS: Announced when there is no next heading (no level filter applied)
					not_found_next: t("No next heading."),
					// TRANSLATORS: Announced when there is no previous heading (no level filter applied)
					not_found_prev: t("No previous heading."),
					format: NavFoundFormat::TextWithLevel,
				}
			}
		}
		MarkerNavTarget::Page => NavAnnouncements {
			// TRANSLATORS: Announced when "Go to Page" is used on a document that has no page numbers
			not_supported: t("No pages."),
			// TRANSLATORS: Announced when there is no next page from the current position
			not_found_next: t("No next page."),
			// TRANSLATORS: Announced when there is no previous page from the current position
			not_found_prev: t("No previous page."),
			format: NavFoundFormat::PageFormat,
		},
		MarkerNavTarget::Link => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no links to navigate
			not_supported: t("No links."),
			// TRANSLATORS: Announced when there is no next link from the current position
			not_found_next: t("No next link."),
			// TRANSLATORS: Announced when there is no previous link from the current position
			not_found_prev: t("No previous link."),
			format: NavFoundFormat::LinkFormat,
		},
		MarkerNavTarget::List => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no lists to navigate
			not_supported: t("No lists."),
			// TRANSLATORS: Announced when there is no next list from the current position
			not_found_next: t("No next list."),
			// TRANSLATORS: Announced when there is no previous list from the current position
			not_found_prev: t("No previous list."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::ListItem => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no list items to navigate
			not_supported: t("No list items."),
			// TRANSLATORS: Announced when there is no next list item from the current position
			not_found_next: t("No next list item."),
			// TRANSLATORS: Announced when there is no previous list item from the current position
			not_found_prev: t("No previous list item."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Table => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no tables to navigate
			not_supported: t("No tables."),
			// TRANSLATORS: Announced when there is no next table from the current position
			not_found_next: t("No next table."),
			// TRANSLATORS: Announced when there is no previous table from the current position
			not_found_prev: t("No previous table."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Separator => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no separators to navigate
			not_supported: t("No separators."),
			// TRANSLATORS: Announced when there is no next separator from the current position
			not_found_next: t("No next separator."),
			// TRANSLATORS: Announced when there is no previous separator from the current position
			not_found_prev: t("No previous separator."),
			format: NavFoundFormat::TextOnly,
		},
		MarkerNavTarget::Image => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no images to navigate
			not_supported: t("No images."),
			// TRANSLATORS: Announced when there is no next image from the current position
			not_found_next: t("No next image."),
			// TRANSLATORS: Announced when there is no previous image from the current position
			not_found_prev: t("No previous image."),
			format: NavFoundFormat::ImageFormat,
		},
		MarkerNavTarget::Figure => NavAnnouncements {
			// TRANSLATORS: Announced when the document has no figures to navigate
			not_supported: t("No figures."),
			// TRANSLATORS: Announced when there is no next figure from the current position
			not_found_next: t("No next figure."),
			// TRANSLATORS: Announced when there is no previous figure from the current position
			not_found_prev: t("No previous figure."),
			format: NavFoundFormat::ImageFormat,
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
		// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
		if wrapped { if next { t("Wrapping to start. ") } else { t("Wrapping to end. ") } } else { String::new() };
	match ann.format {
		NavFoundFormat::TextOnly => format!("{wrap_prefix}{context_text}"),
		NavFoundFormat::TextWithLevel => {
			// TRANSLATORS: Announcement when landing on a heading; %s is the heading text, %d is the heading level number
			let template = t("%s Heading level %d");
			let message = template.replacen("%s", context_text, 1).replacen("%d", &context_index.to_string(), 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::PageFormat => {
			// TRANSLATORS: Announcement when landing on a page; %d is the page number, %s is the page text
			let template = t("Page %d: %s");
			let page_text = (context_index + 1).to_string();
			let message = template.replacen("%d", &page_text, 1).replacen("%s", context_text, 1);
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::LinkFormat => {
			// TRANSLATORS: Suffix appended after a link's text when announcing navigation to a link; the leading space is significant
			let message = format!("{context_text}{}", t(" link"));
			format!("{wrap_prefix}{message}")
		}
		NavFoundFormat::ImageFormat => {
			let message = context_text.to_string();
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
		MarkerNavTarget::Image => result.marker_index,
		MarkerNavTarget::Figure => result.marker_index,
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
			// TRANSLATORS: Announced when moving forward/backward through the caret position history
			let message = if forward { t("Navigated to next position.") } else { t("Navigated to previous position.") };
			tab.text_ctrl.set_focus();
			tab.text_ctrl.set_insertion_point(result.offset);
			tab.text_ctrl.show_position(result.offset);
			tab.session.set_stable_position(result.offset);
			let history_update = if tab.track {
				let (history, history_index) = tab.session.get_history();
				let path_str = tab.file_path.to_string_lossy().to_string();
				Some((path_str, history.to_vec(), history_index))
			} else {
				None
			};
			(message, history_update)
		} else {
			// TRANSLATORS: Announced when there is no next/previous position in the caret position history
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
			MarkerNavTarget::Image => tab.session.navigate_image(current_pos, wrap, next),
			MarkerNavTarget::Figure => tab.session.navigate_figure(current_pos, wrap, next),
		};
		let target_offset = result.offset;
		if apply_navigation_result(tab, &result, target, next, live_region_label) {
			tab.session.check_and_record_history(target_offset);
			if tab.track {
				let (history, history_index) = tab.session.get_history();
				let path_str = tab.file_path.to_string_lossy().to_string();
				Some((path_str, history.to_vec(), history_index))
			} else {
				None
			}
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

/// Navigate relative to the container (list/table) the caret is currently inside: `to_end` jumps
/// just past its end, otherwise to its start. Announces "Not in a container." when the caret is
/// not inside any container.
pub fn handle_container_navigation(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	to_end: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let history_update = {
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		let current_pos = tab.text_ctrl.get_insertion_point();
		let result = tab.session.navigate_container(current_pos, to_end);
		if result.not_supported {
			// TRANSLATORS: Announced when the document has no containers (lists/tables) to navigate
			live_region::announce(live_region_label, &t("No containers."));
			None
		} else if !result.found {
			// TRANSLATORS: Announced when the caret is not currently inside a container (list/table)
			live_region::announce(live_region_label, &t("Not in a container."));
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
			live_region::announce(live_region_label, &message);
			tab.text_ctrl.set_focus();
			tab.text_ctrl.set_insertion_point(offset);
			tab.text_ctrl.show_position(offset);
			tab.session.check_and_record_history(offset);
			if tab.track {
				let (history, history_index) = tab.session.get_history();
				let path_str = tab.file_path.to_string_lossy().to_string();
				Some((path_str, history.to_vec(), history_index))
			} else {
				None
			}
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
			let history_update = if tab.track {
				let (history, history_index) = tab.session.get_history();
				Some((path_str, history.to_vec(), history_index))
			} else {
				None
			};
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
			// TRANSLATORS: Fallback announcement when viewing a bookmark that has no note text or line snippet
			if text.is_empty() { t("Bookmark.") } else { text }
		} else {
			// TRANSLATORS: Fallback announcement when viewing a bookmark that has no note text or line snippet
			t("Bookmark.")
		};
		let history_update = if tab.track {
			let (history, history_index) = tab.session.get_history();
			let path_str = tab.file_path.to_string_lossy().to_string();
			Some((path_str, history.to_vec(), history_index))
		} else {
			None
		};
		(message, history_update)
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
		// TRANSLATORS: Message shown when trying to view a bookmark note but the current position has none
		let dialog = MessageDialog::builder(frame, &t("No note at the current position."), &t("View Note"))
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return;
	}
	dialogs::show_view_note_dialog(frame, &note);
}
