use std::{rc::Rc, sync::Mutex};

use paperback_core::{
	audio::AudioTimeline, config::ConfigManager, reader_core, session::NavigationResult, types::BookmarkFilterType,
};
use patois::t;
use wxdragon::prelude::*;

use super::{
	dialogs,
	document_manager::{DocumentManager, DocumentTab, reload_window_around},
};
use crate::audio_player::AudioPlayer;

/// `(file_path, history_positions, history_index)` snapshot to persist via
/// [`persist_navigation_history`]. Kept as an owned tuple (rather than borrowing
/// from the tab) so it can outlive the `DocumentManager` lock: callers build it
/// while a tab is borrowed, drop the lock, then persist it against `config`.
type HistoryUpdate = (String, Vec<i64>, usize);

/// Snapshots `tab`'s current position history as a [`HistoryUpdate`], if `tab.track`
/// is set. Use this when the caller has already updated the history itself (e.g. via
/// `history_go_forward`/`history_go_back`, which navigate *within* existing history
/// rather than recording a new entry) — for the common "record a new position, then
/// snapshot" case, use [`record_history`] or [`move_to_offset_and_record_history`]
/// instead.
fn tracked_history_update(tab: &DocumentTab) -> Option<HistoryUpdate> {
	if !tab.track {
		return None;
	}
	let (history, history_index) = tab.session.get_history();
	Some((tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index))
}

/// Records `offset` as a new entry in `tab`'s position history and returns the
/// resulting snapshot unconditionally. Callers that should only persist it when
/// `tab.track` is set (most callers) should gate with `tab.track.then_some(update)`.
fn record_history(tab: &mut DocumentTab, offset: i64) -> HistoryUpdate {
	tab.session.check_and_record_history(offset);
	let (history, history_index) = tab.session.get_history();
	(tab.file_path.to_string_lossy().to_string(), history.to_vec(), history_index)
}

/// Reads the caret's document-absolute position, translating it out of `tab`'s currently
/// loaded window. Use this instead of `tab.text_ctrl.get_insertion_point()` directly - that
/// method only knows about whatever window is currently loaded, not the document as a whole.
pub fn doc_caret(tab: &DocumentTab) -> i64 {
	tab.window.to_doc(tab.text_ctrl.get_insertion_point())
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
fn set_caret_to_doc_offset(tab: &mut DocumentTab, offset: i64) {
	if tab.window.needs_reload_for(offset, tab.session.document_len()) {
		reload_window_around(tab, offset);
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

/// Selects the document-absolute range `[start, end)`, focusing the document and reloading
/// `tab`'s window first if `start` falls outside it (see `DocumentTab::window`) - the range form
/// of [`jump_to_doc_offset`], for jumps that should highlight a span of text (e.g. a Find match)
/// rather than just place the caret. Keeps audio in sync with `start`, like `jump_to_doc_offset`.
///
/// Only `start` is checked against the window: a match is expected to be far shorter than a
/// window's `RELOAD_MARGIN`, so `end` lands safely inside whatever window `start` triggers.
pub fn select_doc_range(tab: &mut DocumentTab, start: i64, end: i64) {
	if tab.window.needs_reload_for(start, tab.session.document_len()) {
		reload_window_around(tab, start);
	}
	let local_start = tab.window.to_local(start);
	let local_end = tab.window.to_local(end);
	tab.text_ctrl.set_focus();
	tab.text_ctrl.set_selection(local_start, local_end);
	tab.text_ctrl.show_position(local_start);
	seek_audio_to_position(tab, start);
}

/// Moves the caret to `offset`, focuses the document, shows the position, and
/// records the jump in the session's position history. Returns the resulting
/// history snapshot unconditionally — gate on `tab.track` at the call site if the
/// update should only be persisted when history tracking is enabled for this tab.
pub fn move_to_offset_and_record_history(tab: &mut DocumentTab, offset: i64) -> HistoryUpdate {
	jump_to_doc_offset(tab, offset);
	record_history(tab, offset)
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
			not_found_next: t("No next section."),
			// TRANSLATORS: Announced when there is no previous section from the current position
			not_found_prev: t("No previous section."),
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
			let page_text = (context_index + 1).to_string();
			let message = if context_text.is_empty() {
				// TRANSLATORS: Announcement when landing on a page with no extractable text; %d is the page number
				t("Page %d").replacen("%d", &page_text, 1)
			} else {
				// TRANSLATORS: Announcement when landing on a page; %d is the page number, %s is the page text
				t("Page %d: %s").replacen("%d", &page_text, 1).replacen("%s", context_text, 1)
			};
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
	tab: &mut DocumentTab,
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
	live_region::announce(live_region_label, &message);
	jump_to_doc_offset(tab, result.offset);
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
			tab.session.set_stable_position(result.offset);
			let history_update = tracked_history_update(tab);
			(message, history_update)
		} else {
			// TRANSLATORS: Announced when there is no next/previous position in the caret position history
			let message = if forward { t("No next position.") } else { t("No previous position.") };
			(message, None)
		}
	};
	drop(dm);
	live_region::announce(live_region_label, &message);
	persist_navigation_history(config, history_update.as_ref());
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
		let current_pos = doc_caret(tab);
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
			let update = record_history(tab, target_offset);
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

pub fn handle_toggle_play_pause_audio(doc_manager: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText) {
	let mut dm = doc_manager.lock().unwrap();
	let has_audio = {
		let Some(tab) = dm.active_tab_mut() else { return };
		if let Some(player) = tab.audio_player.as_mut() {
			player.toggle();
			true
		} else {
			false
		}
	};
	drop(dm);
	if !has_audio {
		// TRANSLATORS: Announced when trying to play/pause audio on a document that has none
		live_region::announce(live_region_label, &t("This document has no audio."));
	}
}

/// Where seeking `amount_ms` forward from wherever `player` is now would land, if that runs
/// past the real end of the currently loaded file and "continue into the next file" is
/// enabled: the elapsed time in the document that continuing from the very start of whatever
/// comes next resolves to. `None` if the seek doesn't run off the file's real end, there's no
/// next source, or the next source's own narration hasn't started by that raw position yet
/// (an extremely long seek amount over a very short next file) - the caller falls back to the
/// ordinary clamped-at-`total_duration_ms` target in every one of those cases.
fn spilled_seek_target_ms(player: &AudioPlayer, amount_ms: u64) -> Option<u64> {
	let (source, raw_ms, length_ms) = player.current_file_position_and_length_ms()?;
	spill_overflow_into_next_source(&player.timeline(), source, raw_ms, length_ms, amount_ms)
}

/// The arithmetic behind `spilled_seek_target_ms`, split out so it's testable without a real
/// native media control backing `AudioPlayer`.
///
/// This has to go through the *real* decoder-reported file length (`length_ms`) rather than the
/// document's own declared clip duration, since a plain-audio-zip bundle's placeholder clip
/// duration (see `build_plain_audio_zip_document`) is hours longer than the real file, so the
/// ordinary elapsed-time-based target would just resolve back into the same file, past its real
/// end, where the native seek call clamps it to the file's own last frame instead of advancing.
fn spill_overflow_into_next_source(
	timeline: &AudioTimeline,
	source: usize,
	raw_ms: u64,
	length_ms: u64,
	amount_ms: u64,
) -> Option<u64> {
	let naive_in_file_ms = raw_ms.saturating_add(amount_ms);
	if naive_in_file_ms <= length_ms {
		return None;
	}
	let overflow_ms = naive_in_file_ms - length_ms;
	let next_source = timeline.next_source_after(source)?;
	timeline.elapsed_for_source_position(next_source, overflow_ms)
}

/// Skips the active document's audio narration backward or forward by the configured seek
/// amount (`audio_seek_amount_seconds`, default 10). When "sync caret to audio" is on, the
/// caret follows the new audio position, mirroring what `pump_audio` does during playback;
/// this is the one-shot equivalent for an explicit seek rather than passive following.
pub fn handle_seek_audio(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
) {
	let (sync_enabled, amount_seconds, spill_into_next_file) = {
		let cfg = config.lock().unwrap();
		(
			cfg.get_app_bool("sync_caret_to_audio", true),
			cfg.get_app_int("audio_seek_amount_seconds", 10),
			cfg.get_app_bool("audio_seek_continues_into_next_file", false),
		)
	};
	let amount_ms = u64::try_from(amount_seconds.max(1)).unwrap_or(10) * 1000;
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else { return };
	let Some(player) = tab.audio_player.as_mut() else {
		drop(dm);
		// TRANSLATORS: Announced when trying to seek audio on a document that has none
		live_region::announce(live_region_label, &t("This document has no audio."));
		return;
	};
	let Some(current_ms) = player.resume_point_ms() else {
		drop(dm);
		// TRANSLATORS: Announced when trying to seek audio before playback has established a position
		live_region::announce(live_region_label, &t("Audio hasn't started playing yet."));
		return;
	};
	let total_ms = player.timeline().total_duration_ms();
	let target_ms = if forward {
		let clamped_target_ms = current_ms.saturating_add(amount_ms).min(total_ms);
		if spill_into_next_file {
			spilled_seek_target_ms(player, amount_ms).unwrap_or(clamped_target_ms)
		} else {
			clamped_target_ms
		}
	} else {
		current_ms.saturating_sub(amount_ms)
	};
	player.seek_to_ms(target_ms);
	let sync_position = sync_enabled
		.then(|| player.timeline().cursor_at_elapsed(target_ms))
		.flatten()
		.and_then(|cursor| player.timeline().clip(cursor.clip).map(|clip| i64::try_from(clip.start).unwrap_or(0)));
	if let Some(position) = sync_position {
		set_caret_to_doc_offset(tab, position);
	}
}

/// A human-readable label for one of `dialogs::AUDIO_SEEK_AMOUNTS_SECONDS`, matching the text
/// shown for it in the Options dialog's seek-amount dropdown, for the live-region announcement
/// made when the amount changes via keyboard shortcut.
fn seek_amount_label(seconds: i32) -> String {
	match seconds {
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		5 => t("5 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		10 => t("10 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		30 => t("30 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		60 => t("1 minute"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		120 => t("2 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		300 => t("5 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		600 => t("10 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		1800 => t("30 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		3600 => t("1 hour"),
		other => format!("{other}s"),
	}
}

/// Nudges the configured audio seek amount (used by `handle_seek_audio`) one step up or down
/// through the same preset list shown in the Options dialog's dropdown, and announces the new
/// value. A global setting rather than a per-document action, so unlike `handle_seek_audio` this
/// doesn't need an active document or audio player.
pub fn handle_change_seek_amount(config: &Rc<Mutex<ConfigManager>>, live_region_label: StaticText, increase: bool) {
	let presets = dialogs::AUDIO_SEEK_AMOUNTS_SECONDS;
	let cfg = config.lock().unwrap();
	let current = cfg.get_app_int("audio_seek_amount_seconds", 10);
	let index = presets.iter().position(|&secs| secs == current).unwrap_or_else(|| {
		presets
			.iter()
			.enumerate()
			.min_by_key(|&(_, &secs)| (secs - current).abs())
			.map_or(0, |(nearest_index, _)| nearest_index)
	});
	let new_index = if increase { (index + 1).min(presets.len() - 1) } else { index.saturating_sub(1) };
	let new_value = presets[new_index];
	let at_limit = new_index == index;
	cfg.set_app_int("audio_seek_amount_seconds", new_value);
	cfg.flush();
	drop(cfg);
	let label = seek_amount_label(new_value);
	let message = if at_limit && increase {
		// TRANSLATORS: Announced when the audio seek amount is already at its largest preset; {} is the current amount, e.g. "1 hour"
		t("{} (maximum)").replace("{}", &label)
	} else if at_limit {
		// TRANSLATORS: Announced when the audio seek amount is already at its smallest preset; {} is the current amount, e.g. "5 seconds"
		t("{} (minimum)").replace("{}", &label)
	} else {
		label
	};
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

#[cfg(test)]
mod tests {
	use paperback_core::audio::{AudioLocation, AudioTimelineBuilder};

	use super::*;

	/// Two "files", each one placeholder-duration clip covering its whole 24h declared length
	/// (like `build_plain_audio_zip_document`'s clips), so a real file length far shorter than
	/// that placeholder is what has to trigger the spill, not the document's own clip bounds.
	fn plain_audio_zip_timeline() -> AudioTimeline {
		const PLACEHOLDER_MS: u64 = 24 * 60 * 60 * 1000;
		let mut builder = AudioTimelineBuilder::new();
		let file1 = builder.add_source(AudioLocation::File("chapter1.mp3".to_string()), None);
		let file2 = builder.add_source(AudioLocation::File("chapter2.mp3".to_string()), None);
		builder.add_clip(file1, 0, PLACEHOLDER_MS, 0, 1);
		builder.add_clip(file2, 0, PLACEHOLDER_MS, 1, 2);
		builder.build()
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_when_the_seek_stays_within_the_real_file() {
		let timeline = plain_audio_zip_timeline();
		// 30s into a 45s file, seeking 10s: lands at 40s, still short of the real 45s end.
		assert_eq!(spill_overflow_into_next_source(&timeline, 0, 30_000, 45_000, 10_000), None);
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_exactly_at_the_real_end() {
		let timeline = plain_audio_zip_timeline();
		assert_eq!(spill_overflow_into_next_source(&timeline, 0, 35_000, 45_000, 10_000), None);
	}

	#[test]
	fn spill_overflow_into_next_source_lands_the_overflow_into_the_next_files_start() {
		let timeline = plain_audio_zip_timeline();
		// 40s into a 45s file, seeking 10s: 5s of that seek belongs to whatever plays next.
		let target = spill_overflow_into_next_source(&timeline, 0, 40_000, 45_000, 10_000);
		assert_eq!(target, timeline.elapsed_for_source_position(1, 5_000));
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_past_the_last_file() {
		let timeline = plain_audio_zip_timeline();
		assert_eq!(spill_overflow_into_next_source(&timeline, 1, 40_000, 45_000, 10_000), None);
	}
}
