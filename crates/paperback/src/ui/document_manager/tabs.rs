//! Which document has focus and which tab is showing. Split out of the main `DocumentManager` impl.

use wxdragon::prelude::*;

use super::{DocumentManager, display_title};
use crate::ui::navigation::announce;

impl DocumentManager {
	/// Puts focus on the freshly opened document's text, and makes that the target a later
	/// [`Self::restore_focus`] will return to.
	///
	/// Opening a document is a request to read it, so it is not a case for restoring an earlier
	/// preference: whatever had focus a moment ago was about a different document, or about
	/// there being none.
	pub fn focus_document_text(&self) {
		if let Some(tab) = self.active_tab() {
			self.last_focus_in_text.set(true);
			tab.text_ctrl.set_focus();
		} else {
			self.notebook.set_focus();
		}
	}

	/// Restores focus to whichever control had it when the window was last active (the text
	/// control or the notebook), falling back to the notebook when there's no active document.
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

	/// Records whether the text control or the notebook currently has focus, so focus can be
	/// restored to the same place when the window is next activated. Only updates when one of
	/// the two is confidently focused (a mid-focus-transition leaves the previous value).
	///
	/// With no documents open there is nothing to record: the notebook is the only focusable
	/// control, so it has focus by default rather than by choice. Writing that down as "the user
	/// prefers the tab strip" made the next document open onto its own tab strip instead of its
	/// text, because [`Self::restore_focus`] honours the preference and could not tell a real
	/// one from the absence of an alternative.
	#[cfg(target_os = "windows")]
	pub fn record_focus_target(&self) {
		if self.tabs.is_empty() {
			return;
		}
		if self.active_tab().is_some_and(|tab| tab.text_ctrl.has_focus()) {
			self.last_focus_in_text.set(true);
		} else if self.notebook.has_focus() {
			self.last_focus_in_text.set(false);
		}
	}

	/// Tells screen readers that the text control has focus, after `restore_focus`. On Windows the
	/// read-only Richedit does not emit its own focus event on re-activation, and `SetFocus` on a
	/// window that already has focus emits nothing at all, so the text control needs the explicit
	/// event; the notebook's native tab control announces its selected tab on its own (firing a
	/// whole-control event there would swallow that), so nothing is sent for it.
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

	/// Moves to the next document, or the previous one, wrapping around at either end.
	#[cfg(not(target_os = "windows"))]
	pub fn cycle_tab(&self, forward: bool) {
		let count = self.tabs.len();
		let Some(active) = self.active_tab_index() else { return };
		let index = if forward { (active + 1) % count } else { (active + count - 1) % count };
		self.switch_to_tab(index);
	}

	/// Selects the tab at `index`, reporting whether there was such a tab.
	///
	/// Goes through the notebook rather than setting state directly, which is what makes this
	/// behave exactly like the notebook's own Ctrl+Tab: the tab strip repaints, the page-changed
	/// handler updates the title bar, pauses the other document's audio and checks whether the
	/// file changed on disk.
	///
	/// The title is announced here rather than left to the page-changing handler, and that is the
	/// part which is easy to get wrong. That handler takes the manager lock before it announces
	/// anything, and every caller of this method already holds it - there is no way to reach a
	/// `DocumentManager` except through its `MutexGuard` - so it takes the lock, fails, and
	/// returns in silence. The jump then announced nothing at all, and what a screen reader said
	/// instead was its own focus event on the newly revealed text control: "edit read only",
	/// then whatever line the caret was on. Ctrl+Tab never had that problem because the notebook
	/// handles it from its own key handler, with the lock free.
	///
	/// Announcing last, and through the delayed live region rather than straight to the reader,
	/// is what makes the title land on top of that focus event instead of under it - the same
	/// thing [`announce`]'s delay is there for everywhere else. It is the one behaviour worth
	/// copying from Ctrl+Tab, and it is a behaviour of the announcement, not of the focus.
	///
	/// `Ctrl+<n>` for an `n` past the last document reports false and does nothing at all. The
	/// alternative - saying so - would mean a new translatable string for a key that is mostly
	/// pressed by habit, and a reader who is nine documents deep is not going to wonder what
	/// went wrong.
	pub fn switch_to_tab(&self, index: usize) -> bool {
		let Some(tab) = self.tabs.get(index) else {
			return false;
		};
		if self.active_tab_index() == Some(index) {
			// Re-announced rather than left silent. "Already on this tab" and "the shortcut did
			// nothing" are indistinguishable to a screen reader, and the shortcut doing nothing
			// is the reading that costs the reader the most.
			announce(self.live_region_label, display_title(tab));
			return true;
		}
		self.notebook.set_selection(index);
		// The same condition the page-changing handler uses. From the reading control the notebook
		// does not have focus, the handler cannot get in to say anything, and the title is the
		// only thing this gesture should be heard saying. From the tab strip the notebook does
		// have focus and its native control announces its own new selection, so saying it again
		// would read the title twice.
		//
		// After the page change, so that the title lands on top of the focus event the switch
		// caused rather than under it.
		if !self.notebook.has_focus() {
			announce(self.live_region_label, display_title(&self.tabs[index]));
		}
		true
	}
}
