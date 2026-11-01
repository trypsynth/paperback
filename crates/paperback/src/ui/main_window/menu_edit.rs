use std::{rc::Rc, sync::Mutex};

#[cfg(not(target_os = "macos"))]
use patois::nt;
use patois::t;
use wxdragon::prelude::*;

use super::DocumentManager;
#[cfg(not(target_os = "macos"))]
use crate::ui::navigation::announce_for_selection_command;
use crate::ui::{
	navigation::announce_for_command,
	selection::{copied_announcement, copy_to_clipboard},
};

#[derive(Clone, Copy)]
enum CopyOutcome {
	Nothing,
	Copied(usize),
	Failed,
}

/// Announces because a copy otherwise gives no feedback, so one that copied nothing would go unnoticed until the reader pastes.
pub fn handle_copy(dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText, from_keyboard: bool) {
	// The clipboard write goes through OLE on Windows, which can pump messages, and the document manager's mutex is not reentrant, so the write and the announcement stay outside the lock.
	let text = dm.lock().unwrap().text_to_copy();
	let outcome = text.map_or(CopyOutcome::Nothing, |text| {
		let count = text.chars().count();
		if copy_to_clipboard(&text) { CopyOutcome::Copied(count) } else { CopyOutcome::Failed }
	});
	announce_for_command(live_region_label, from_keyboard, copy_message(outcome));
}

/// Separate so a test can pin the wording, since a test cannot build a document.
fn copy_message(outcome: CopyOutcome) -> String {
	match outcome {
		// TRANSLATORS: Announced when the copy shortcut is pressed with nothing selected, so there was nothing to put on the clipboard.
		CopyOutcome::Nothing => t("Nothing to copy."),
		// TRANSLATORS: Announced when the copy ran but the text could not be put on the clipboard, so nothing was copied.
		CopyOutcome::Failed => t("Could not copy to the clipboard."),
		CopyOutcome::Copied(count) => copied_announcement(count),
	}
}

/// Announces because after a menu click the focus return reads over the screen reader's own selection report; the shortcut waits too, or the message would leave that report hanging on the end.
#[cfg(not(target_os = "macos"))]
pub fn handle_select_all(dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText, from_keyboard: bool) {
	let Some(count) = dm.lock().unwrap().select_all_and_count() else {
		return;
	};
	announce_for_selection_command(live_region_label, from_keyboard, selected_message(count));
}

/// Separate so a test can pin the wording, since a test cannot build a document.
#[cfg(not(target_os = "macos"))]
fn selected_message(count: usize) -> String {
	// TRANSLATORS: Announced after Select All, from the Edit menu or with Ctrl+A. The %d placeholder is replaced with the number of characters in the document. Plural form is chosen by that count.
	nt("%d character selected.", "%d characters selected.", u64::try_from(count).unwrap_or(0)).replacen(
		"%d",
		&count.to_string(),
		1,
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The same wording as the mark-based copy, so the two commands are learned once.
	#[test]
	fn a_copy_says_how_many_characters_it_copied() {
		assert_eq!(copy_message(CopyOutcome::Copied(42)), "42 characters copied.");
	}

	#[test]
	fn a_copy_of_one_character_is_not_plural() {
		assert_eq!(copy_message(CopyOutcome::Copied(1)), "1 character copied.");
	}

	#[test]
	fn a_copy_with_nothing_selected_says_so() {
		assert_eq!(copy_message(CopyOutcome::Nothing), "Nothing to copy.");
	}

	#[test]
	fn a_clipboard_that_refused_the_text_says_so() {
		assert_eq!(copy_message(CopyOutcome::Failed), "Could not copy to the clipboard.");
	}

	#[test]
	#[cfg(not(target_os = "macos"))]
	fn a_select_all_says_how_many_characters_the_document_has() {
		assert_eq!(selected_message(42), "42 characters selected.");
	}

	#[test]
	#[cfg(not(target_os = "macos"))]
	fn a_select_all_of_one_character_is_not_plural() {
		assert_eq!(selected_message(1), "1 character selected.");
	}
}
