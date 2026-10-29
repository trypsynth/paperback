use paperback_core::document::DocPos;
use patois::t;

use super::{DocumentManager, DocumentTab};
use crate::ui::{
	navigation::{self, announce_for_command, move_to_offset_and_record_history, persist_navigation_history},
	sleep_timer, status,
};

impl DocumentManager {
	pub fn image_file_name_at_caret(&self) -> Option<String> {
		let tab = self.active_tab()?;
		tab.session.image_file_name_at(tab.window.to_doc(tab.text_ctrl.get_insertion_point()))
	}

	pub fn link_at_caret(&self) -> bool {
		self.active_tab()
			.is_some_and(|tab| tab.session.is_on_link(tab.window.to_doc(tab.text_ctrl.get_insertion_point())))
	}

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

	pub fn announce_current_percent(&self, from_keyboard: bool) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let percent = navigation::reading_percent(tab, position);
		let page = page_at(tab, position);
		announce_for_command(self.live_region_label, from_keyboard, position_announcement(percent, page));
	}

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
			let position = position.min(tab.session.document_len().as_pos());
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

fn page_at(tab: &DocumentTab, position: DocPos) -> Option<i32> {
	if tab.session.page_count() == 0 {
		return None;
	}
	// `current_page` answers 0 ahead of the first marker (RTF, DAISY, an EPUB's cover pages), which is no page to say; clamped like Go to Page, though the real fix belongs in `current_page`.
	Some(tab.session.current_page(position).max(1))
}

pub(super) fn position_announcement(percent: i32, page: Option<i32>) -> String {
	let Some(page) = page else {
		return format!("{percent}%");
	};
	// TRANSLATORS: Announced by the shortcut that reports the reading position. %s is how far through the document the reader is, e.g. "15%"; %d is the page number.
	t("%s, page %d").replacen("%d", &page.to_string(), 1).replacen("%s", &format!("{percent}%"), 1)
}
