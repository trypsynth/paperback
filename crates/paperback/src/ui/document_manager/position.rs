//! The reader's place in the active document: the status bar, the spoken position, and the temporary bookmark. Split out of the main `DocumentManager` impl.

use patois::t;

use super::{DocumentManager, DocumentTab};
use crate::ui::{
	navigation::{self, announce_for_command, move_to_offset_and_record_history, persist_navigation_history},
	sleep_timer, status,
};

impl DocumentManager {
	pub fn update_status_bar(&self) {
		let sleep_start = sleep_timer::start_ms();
		let sleep_duration = sleep_timer::duration_minutes();
		if self.tabs.is_empty() {
			// TRANSLATORS: Default status bar text when no document is open
			let mut status_text = t("Ready");
			if sleep_start > 0 {
				let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
				if remaining > 0 {
					status_text = status::format_sleep_timer_status(&status_text, remaining);
				}
			}
			self.frame.set_status_text(&status_text, 0);
			return;
		}
		if let Some(tab) = self.active_tab() {
			let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let status_info = tab.session.get_status_info(position);
			let mut status_text = status::format_status_text(&status_info);
			if sleep_start > 0 {
				let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
				if remaining > 0 {
					status_text = status::format_sleep_timer_status(&status_text, remaining);
				}
			}
			self.frame.set_status_text(&status_text, 0);
		}
	}

	/// Announces the current caret position as a percentage of the document, and the page it falls
	/// on where the document has pages, via the live region.
	pub fn announce_current_percent(&self, from_keyboard: bool) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let percent = navigation::reading_percent(tab, position);
		let page = page_at(tab, position);
		announce_for_command(self.live_region_label, from_keyboard, position_announcement(percent, page));
	}

	/// Sets the temporary bookmark at the current caret position and announces it.
	pub fn set_temporary_bookmark(&self, from_keyboard: bool) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let path_str = tab.file_path.to_string_lossy().to_string();
		let config = self.config.lock().unwrap();
		config.set_temporary_bookmark(&path_str, Some(position));
		config.flush();
		drop(config);
		// TRANSLATORS: Announced after setting a temporary bookmark at the current position
		announce_for_command(self.live_region_label, from_keyboard, t("Temporary bookmark set."));
	}

	/// Jumps to the temporary bookmark, announcing the line text there, or "No temporary bookmark."
	/// if none has been set.
	pub fn jump_to_temporary_bookmark(&mut self, from_keyboard: bool) {
		let path_str = {
			let Some(tab) = self.active_tab() else {
				return;
			};
			tab.file_path.to_string_lossy().to_string()
		};
		let position = {
			let config = self.config.lock().unwrap();
			config.get_temporary_bookmark(&path_str)
		};
		let Some(position) = position else {
			// TRANSLATORS: Announced when jumping to a temporary bookmark but none has been set
			announce_for_command(self.live_region_label, from_keyboard, t("No temporary bookmark."));
			return;
		};
		let (message, track, update) = {
			let tab = self.active_tab_mut().unwrap();
			let position = position.clamp(0, tab.session.document_len().max(0));
			let line_text = tab.session.get_line_text(position);
			let message = if line_text.trim().is_empty() {
				// TRANSLATORS: Fallback announcement when jumping to a temporary bookmark on a blank line
				t("Temporary bookmark.")
			} else {
				line_text
			};
			let update = move_to_offset_and_record_history(tab, position);
			(message, tab.track, update)
		};
		announce_for_command(self.live_region_label, from_keyboard, message);
		persist_navigation_history(&self.config, track.then_some(&update));
	}
}

/// The 1-based page `position` falls on, or `None` for a document that has no pages at all -
/// meaning one with no page-break markers, which is every plain text and markdown document and any
/// EPUB that carries no page list.
///
/// The `.max(1)` covers a hole in `current_page`, which answers 0 both for a document with no pages
/// and for a position ahead of the first marker: the first is what the guard above has already
/// turned into `None`, the second is not a page number to say out loud. Only a PDF escapes the
/// second case, since its first page marker sits at offset 0. An RTF's and a DAISY book's first
/// break lands mid-document, and an EPUB page list starts at the first *content* page, leaving the
/// cover and title pages in front of every marker.
///
/// Every caller of `current_page` patches that 0 differently or not at all - Go to Page clamps it
/// (`dialogs::show_go_to_page_dialog`), the Elements view reads it as "no closest page"
/// (`dialogs::elements`), and iOS's Go To sheet shows it to the reader as a plain 0. This agrees
/// with Go to Page, the one other place that speaks a page number; the fix that would stop it being
/// forgotten belongs in `current_page` itself.
fn page_at(tab: &DocumentTab, position: i64) -> Option<i32> {
	if tab.session.page_count() == 0 {
		return None;
	}
	Some(tab.session.current_page(position).max(1))
}

/// What the "announce percentage" shortcut says for `percent` through the document, on `page` where
/// it has pages - "15%, page 30" - or the bare percentage where it has none.
pub(super) fn position_announcement(percent: i32, page: Option<i32>) -> String {
	let Some(page) = page else {
		return format!("{percent}%");
	};
	// TRANSLATORS: Announced by the shortcut that reports the reading position. %s is how far through the document the reader is, e.g. "15%"; %d is the page number.
	t("%s, page %d").replacen("%d", &page.to_string(), 1).replacen("%s", &format!("{percent}%"), 1)
}
