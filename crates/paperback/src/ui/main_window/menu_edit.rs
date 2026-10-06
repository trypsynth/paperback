//! Edit > Copy and Edit > Select All on Windows and Linux.

use std::{rc::Rc, sync::Mutex};

use patois::t;
use wxdragon::{clipboard::Clipboard, prelude::*};

use super::DocumentManager;
use crate::ui::{navigation::announce_for_command, selection::copied_announcement};

/// What the copy shortcut found when it went to copy.
#[derive(Clone, Copy)]
enum CopyOutcome {
	/// Nothing was selected, or what was selected is empty.
	Nothing,
	/// This many characters went on the clipboard.
	Copied(usize),
	/// The clipboard would not take the text.
	Failed,
}

/// Puts the selection, or the whole document after a Select All, on the clipboard, and says what it did. The document manager is unlocked before the clipboard write.
///
/// Announces because it is the only feedback a copy gives this reader: there is no beep and nothing changes on screen, so a copy that copied nothing is otherwise silent and the reader finds out later, by pasting. The copy from a marked selection already announces, in these same words.
pub fn handle_copy(dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText, from_keyboard: bool) {
	// `Clipboard::set_text` goes through OLE on Windows, which can pump messages, and the document manager's mutex is not reentrant, so the write and the announcement stay outside the lock.
	let text = dm.lock().unwrap().text_to_copy();
	let outcome = text.map_or(CopyOutcome::Nothing, |text| {
		let count = text.chars().count();
		if Clipboard::get().set_text(&text) { CopyOutcome::Copied(count) } else { CopyOutcome::Failed }
	});
	announce_for_command(live_region_label, from_keyboard, copy_message(outcome));
}

/// Its own function of the outcome so the wording can be pinned without a document, which a test cannot build.
fn copy_message(outcome: CopyOutcome) -> String {
	match outcome {
		// TRANSLATORS: Announced when the copy shortcut is pressed with nothing selected, so there was nothing to put on the clipboard.
		CopyOutcome::Nothing => t("Nothing to copy."),
		// TRANSLATORS: Announced when the copy ran but the text could not be put on the clipboard, so nothing was copied.
		CopyOutcome::Failed => t("Could not copy to the clipboard."),
		CopyOutcome::Copied(count) => copied_announcement(count),
	}
}

pub fn handle_select_all(dm: &Rc<Mutex<DocumentManager>>) {
	if let Some(tab) = dm.lock().unwrap().active_tab() {
		tab.text_ctrl.select_all();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The same wording as the mark-based copy, so the two commands are learned once.
	#[test]
	fn a_copy_says_how_many_characters_it_copied() {
		assert_eq!(copy_message(CopyOutcome::Copied(42)), "Copied 42 characters.");
	}

	/// One character is read aloud by a person, not measured by a buffer.
	#[test]
	fn a_copy_of_one_character_is_not_plural() {
		assert_eq!(copy_message(CopyOutcome::Copied(1)), "Copied 1 character.");
	}

	/// A copy that copied nothing has to say so rather than leaving the reader to paste and find out.
	#[test]
	fn a_copy_with_nothing_selected_says_so() {
		assert_eq!(copy_message(CopyOutcome::Nothing), "Nothing to copy.");
	}

	#[test]
	fn a_clipboard_that_refused_the_text_says_so() {
		assert_eq!(copy_message(CopyOutcome::Failed), "Could not copy to the clipboard.");
	}
}
