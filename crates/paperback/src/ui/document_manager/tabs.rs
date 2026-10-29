use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, display_title};
use crate::ui::{main_window::update_title_from_manager, navigation::announce};

impl DocumentManager {
	/// Opening a document is a request to read it, so an earlier focus preference doesn't apply.
	pub fn focus_document_text(&self) {
		if let Some(tab) = self.active_tab() {
			self.last_focus_in_text.set(true);
			tab.text_ctrl.set_focus();
		} else {
			self.notebook.set_focus();
		}
	}

	pub fn restore_focus(&self) {
		if self.last_focus_in_text.get() {
			if let Some(tab) = self.active_tab() {
				tab.text_ctrl.set_focus();
			} else {
				self.notebook.set_focus();
			}
		} else {
			self.notebook.set_focus();
		}
	}

	#[cfg(target_os = "windows")]
	pub fn record_focus_target(&self) {
		// With no documents the notebook has focus by default, not by choice; recording it made the next document open onto its tab strip.
		if self.tabs.is_empty() {
			return;
		}
		if self.active_tab().is_some_and(|tab| tab.text_ctrl.has_focus()) {
			self.last_focus_in_text.set(true);
		} else if self.notebook.has_focus() {
			self.last_focus_in_text.set(false);
		}
	}

	/// The read-only `RichEdit` emits no focus event on re-activation and `SetFocus` on an already focused window emits nothing; the notebook is skipped because its native control announces its tab and a whole-control event would swallow that.
	#[cfg(target_os = "windows")]
	pub fn announce_focus(&self) {
		if !self.last_focus_in_text.get() {
			return;
		}
		if let Some(tab) = self.active_tab() {
			let hwnd = windows::Win32::Foundation::HWND(tab.text_ctrl.get_handle());
			// EVENT_OBJECT_FOCUS = 0x8005, OBJID_CLIENT = -4, CHILDID_SELF = 0
			unsafe {
				windows::Win32::UI::Accessibility::NotifyWinEvent(0x8005, hwnd, -4, 0);
			}
		}
	}

	#[cfg(not(target_os = "windows"))]
	pub fn cycle_tab(&mut self, forward: bool) {
		let count = self.tabs.len();
		let Some(active) = self.active_tab_index() else { return };
		let index = if forward { (active + 1) % count } else { (active + count - 1) % count };
		self.switch_to_tab(index);
	}

	/// Every caller holds the manager lock, so the page-changed handler (synchronous on Windows) can't get in; its follow-up and the title announcement happen here instead, or the switch is heard only as "edit read only".
	pub fn switch_to_tab(&mut self, index: usize) -> bool {
		// Silent past the last tab: a new translatable string isn't worth it for a key mostly pressed by habit.
		let Some(tab) = self.tabs.get(index) else {
			return false;
		};
		if self.active_tab_index() == Some(index) {
			// "Already on this tab" and "the shortcut did nothing" are indistinguishable to a screen reader.
			announce(self.live_region_label, display_title(tab));
			return true;
		}
		// Through the notebook so the tab strip repaints exactly as for its own Ctrl+Tab.
		self.notebook.set_selection(index);
		self.finish_page_change(true);
		// From the tab strip the native control announces the selection itself; announcing after the page change, via the delayed live region, lands the title on top of the focus event rather than under it.
		if !self.notebook.has_focus() {
			announce(self.live_region_label, display_title(&self.tabs[index]));
		}
		true
	}

	/// Running twice for one switch, where the notebook delivers page-changed after the lock is released, is harmless.
	pub fn finish_page_change(&mut self, check_disk: bool) {
		if check_disk
			&& let Some(index) = self.active_tab_index()
			&& self.reload_tab_if_changed(index)
		{
			// Medium so an unrequested reload waits for the current utterance instead of cutting the reader off.
			// TRANSLATORS: Announced by screen readers after a document was re-read from disk, either because the reader asked for it with F5 or because its file changed on disk
			live_region::announce_with_priority(
				self.live_region_label,
				&t("Document reloaded."),
				live_region::Priority::Medium,
			);
		}
		update_title_from_manager(&self.frame, self);
		self.reset_sound_line();
		self.pause_inactive_audio();
	}
}
