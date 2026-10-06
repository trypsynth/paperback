//! The heavier "Go" menu handlers: the table of contents and elements list, and prompting for a
//! line/page/percent and jumping there. The rest of the Go menu (marker and history navigation)
//! is thin enough to stay inlined in `bind_menu_events`'s dispatch match.

use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, session::DocumentSession};
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, dialogs, navigation};
use crate::ui::navigation::announce;

pub(super) fn handle_go_to_line(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (current_line, max_lines) = {
		let mut dm_guard = dm.lock().unwrap();
		let (current_line, max_lines) = {
			let Some(tab) = dm_guard.active_tab_mut() else {
				return;
			};
			let current_pos = navigation::doc_caret(tab);
			let status = tab.session.get_status_info(current_pos);
			let total_lines = tab.session.line_count().max(1);
			let max_lines = i32::try_from(total_lines.min(i64::from(i32::MAX))).unwrap_or(i32::MAX);
			let current_line =
				i32::try_from(status.line_number.clamp(1, total_lines).min(i64::from(i32::MAX))).unwrap_or(i32::MAX);
			(current_line, max_lines)
		};
		drop(dm_guard);
		(current_line, max_lines)
	};
	if let Some(line) = dialogs::show_go_to_line_dialog(frame, current_line, max_lines, live_region_label) {
		let (message, update) = {
			let mut dm_guard = dm.lock().unwrap();
			let (message, update) = {
				let Some(tab) = dm_guard.active_tab_mut() else {
					return;
				};
				let target_pos = tab.session.position_from_line(i64::from(line));
				// Capture the line's announcement while the document lock is held; it is
				// spoken after focus has returned to the book, cutting off the focus chain.
				let content = tab.session.get_line_text(target_pos);
				let message = line_announcement(line, &content);
				let update = navigation::move_to_offset_and_record_history(tab, target_pos);
				(message, update)
			};
			drop(dm_guard);
			(message, update)
		};
		navigation::persist_navigation_history(config, Some(&update));
		announce(live_region_label, message);
	}
}

/// The announcement for landing on line `line`, whose text is `content` (possibly blank).
///
/// Unlike Go to Page, the line number is not prefixed to the text: page numbers carry
/// external meaning and there are few of them, but line numbers can be huge and say
/// nothing on their own, so the user hears the line's text, as Find does. A blank line
/// falls back to the bare line number so the jump is still confirmed.
fn line_announcement(line: i32, content: &str) -> String {
	let content = content.trim();
	if content.is_empty() {
		// TRANSLATORS: Announced when landing on a blank line; %d is the line number
		t("Line %d").replacen("%d", &line.to_string(), 1)
	} else {
		content.to_string()
	}
}

pub(super) fn handle_go_to_page(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (current_page, max_page) = {
		let mut dm_guard = dm.lock().unwrap();
		let (current_page, max_page) = {
			let Some(tab) = dm_guard.active_tab_mut() else {
				return;
			};
			let page_count = tab.session.page_count();
			if page_count == 0 {
				// TRANSLATORS: Announced when "Go to Page" is used on a document that has no page numbers
				// A menu invocation returns focus to the book just like a closed dialog does,
				// so the announcement needs the same delay to cut off NVDA's focus-chain read.
				announce(live_region_label, t("No pages."));
				return;
			}
			let current_pos = navigation::doc_caret(tab);
			let current_page = tab.session.current_page(current_pos);
			let max_page = i32::try_from(page_count.max(1)).unwrap_or(i32::MAX);
			(current_page, max_page)
		};
		drop(dm_guard);
		(current_page, max_page)
	};
	if let Some(page) = dialogs::show_go_to_page_dialog(frame, current_page, max_page, live_region_label) {
		let (message, update) = {
			let mut dm_guard = dm.lock().unwrap();
			let (message, update) = {
				let Some(tab) = dm_guard.active_tab_mut() else {
					return;
				};
				let target_pos = tab.session.page_offset(page);
				// Capture the page's announcement while the document lock is held; it is
				// spoken after focus has returned to the book, cutting off the focus chain.
				let content = tab.session.first_content_line_after(target_pos);
				let message = navigation::page_announcement(page, &content);
				let update = navigation::move_to_offset_and_record_history(tab, target_pos);
				(message, update)
			};
			drop(dm_guard);
			(message, update)
		};
		navigation::persist_navigation_history(config, Some(&update));
		announce(live_region_label, message);
	}
}

pub(super) fn handle_go_to_percent(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let current_percent = {
		let mut dm_guard = dm.lock().unwrap();
		let current_percent = {
			let Some(tab) = dm_guard.active_tab_mut() else {
				return;
			};
			let current_pos = navigation::doc_caret(tab);
			navigation::reading_percent(tab, current_pos)
		};
		drop(dm_guard);
		current_percent
	};
	if let Some(percent) = dialogs::show_go_to_percent_dialog(frame, current_percent, live_region_label) {
		let (message, update) = {
			let mut dm_guard = dm.lock().unwrap();
			let (message, update) = {
				let Some(tab) = dm_guard.active_tab_mut() else {
					return;
				};
				// An audio document seeks the recording; the caret follows the clip that lands
				// on. Everything else maps the percentage through the text as before.
				let target_pos = navigation::seek_audio_to_percent(tab, percent)
					.unwrap_or_else(|| tab.session.position_from_percent(percent));
				// Announce the line the jump lands on, as Go to Line does; a percentage alone
				// says nothing about where it points. Fall back to the percentage when that
				// line is blank.
				let content = tab.session.get_line_text(target_pos);
				let message =
					if content.trim().is_empty() { format!("{percent}%") } else { content.trim().to_string() };
				let update = navigation::move_to_offset_and_record_history(tab, target_pos);
				(message, update)
			};
			drop(dm_guard);
			(message, update)
		};
		navigation::persist_navigation_history(config, Some(&update));
		// Same interrupt as Find/Go to Line/Go to Page: raise the announcement ~30ms after the
		// dialog closes so the focus-return chain is cut off.
		announce(live_region_label, message);
	}
}

pub(super) fn handle_table_of_contents(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (message, update) = {
		let mut dm_guard = dm.lock().unwrap();
		let (message, update) = {
			let Some(tab) = dm_guard.active_tab_mut() else {
				return;
			};
			let toc_items = &tab.session.handle().document().toc_items;
			if toc_items.is_empty() {
				// TRANSLATORS: Announced when opening the Table of Contents for a document that has none
				announce(live_region_label, t("No table of contents."));
				return;
			}
			let current_pos = navigation::doc_caret(tab);
			let current_pos_usize = usize::try_from(current_pos).unwrap_or(0);
			let current_toc_offset = tab.session.handle().find_closest_toc_offset(current_pos_usize);
			let Some(offset) =
				dialogs::show_toc_dialog(frame, toc_items, i32::try_from(current_toc_offset).unwrap_or(i32::MAX))
			else {
				return;
			};
			let offset = i64::from(offset);
			let update = navigation::move_to_offset_and_record_history(tab, offset);
			// Capture the entry's announcement while the document lock is held; it is spoken
			// after focus has returned to the book, cutting off the focus chain.
			let message = toc_announcement(&tab.session, offset);
			(message, update)
		};
		drop(dm_guard);
		(message, update)
	};
	navigation::persist_navigation_history(config, Some(&update));
	announce(live_region_label, message);
}

/// The announcement for landing on document offset `offset` after picking a table of contents
/// entry. Always reads the content line at the destination, never announcing it as a page jump:
/// a section that begins at the top of a page shares its offset with the page marker, and a
/// "Page N:" prefix there would be redundant. Falls back to the bare line number when the
/// destination has no content line.
fn toc_announcement(session: &DocumentSession, offset: i64) -> String {
	let content = session.first_content_line_after(offset);
	if content.trim().is_empty() {
		// TRANSLATORS: Announced when landing on a blank line; %d is the line number
		t("Line %d").replacen("%d", &session.line_from_position(offset).to_string(), 1)
	} else {
		content.trim().to_string()
	}
}

pub(super) fn handle_elements_list(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (message, update) = {
		let mut dm_guard = dm.lock().unwrap();
		let (message, update) = {
			let Some(tab) = dm_guard.active_tab_mut() else {
				return;
			};
			let current_pos = navigation::doc_caret(tab);
			let Some((offset, kind)) = dialogs::show_elements_dialog(frame, &tab.session, current_pos) else {
				return;
			};
			let update = navigation::move_to_offset_and_record_history(tab, offset);
			// Capture the line the jump lands on while the document lock is held; it is
			// spoken after focus has returned to the book, cutting off the focus chain.
			let message = elements_announcement(&tab.session, offset, kind);
			(message, update)
		};
		drop(dm_guard);
		(message, update)
	};
	navigation::persist_navigation_history(config, Some(&update));
	announce(live_region_label, message);
}

/// The announcement for landing on document offset `offset` from the Elements view `kind`.
/// Only a jump from the Pages view reads like page navigation ("Page N: <first content
/// line>", skipping the page's fabricated "Page N" header), since a page row's target is a
/// page marker; a heading or link reads the line it lands on, even when that element happens
/// to begin at the top of a page (there its offset coincides with the page marker, and a
/// "Page N:" prefix would be redundant). Falls back to the bare line number when the
/// destination line is blank.
fn elements_announcement(session: &DocumentSession, offset: i64, kind: dialogs::ElementsKind) -> String {
	if kind == dialogs::ElementsKind::Page {
		let page_count = i32::try_from(session.page_count()).unwrap_or(0);
		if let Some(page) = (1..=page_count).find(|&page| session.page_offset(page) == offset) {
			let content = session.first_content_line_after(offset);
			return navigation::page_announcement(page, &content);
		}
	}
	let content = session.get_line_text(offset).trim().to_string();
	if content.is_empty() {
		// TRANSLATORS: Announced when landing on a blank line; %d is the line number
		t("Line %d").replacen("%d", &session.line_from_position(offset).to_string(), 1)
	} else {
		content
	}
}
