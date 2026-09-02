//! The heavier "Go" menu handlers: prompting for a line/page/percent and jumping there. The
//! rest of the Go menu (marker/bookmark/history navigation, Find) is thin enough to stay
//! inlined in `bind_menu_events`'s dispatch match.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, dialogs, navigation};

pub(super) fn handle_go_to_line(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
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
	if let Some(line) = dialogs::show_go_to_line_dialog(frame, current_line, max_lines) {
		let update = {
			let mut dm_guard = dm.lock().unwrap();
			let update = {
				let Some(tab) = dm_guard.active_tab_mut() else {
					return;
				};
				let target_pos = tab.session.position_from_line(i64::from(line));
				navigation::move_to_offset_and_record_history(tab, target_pos)
			};
			drop(dm_guard);
			update
		};
		navigation::persist_navigation_history(config, Some(&update));
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
				live_region::announce(live_region_label, &t("No pages."));
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
		navigation::announce_after_delay(frame, live_region_label, message);
	}
}
pub(super) fn handle_go_to_percent(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
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
	if let Some(percent) = dialogs::show_go_to_percent_dialog(frame, current_percent) {
		let update = {
			let mut dm_guard = dm.lock().unwrap();
			let update = {
				let Some(tab) = dm_guard.active_tab_mut() else {
					return;
				};
				// An audio document seeks the recording; the caret follows the clip that lands
				// on. Everything else maps the percentage through the text as before.
				let target_pos = navigation::seek_audio_to_percent(tab, percent)
					.unwrap_or_else(|| tab.session.position_from_percent(percent));
				navigation::move_to_offset_and_record_history(tab, target_pos)
			};
			drop(dm_guard);
			update
		};
		navigation::persist_navigation_history(config, Some(&update));
	}
}
