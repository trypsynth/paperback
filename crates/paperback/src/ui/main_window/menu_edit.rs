//! Edit > Copy and Edit > Select All on Windows and Linux.

use std::{rc::Rc, sync::Mutex};

use patois::{nt, t};
use wxdragon::{clipboard::Clipboard, prelude::*};

use super::DocumentManager;
use crate::ui::{
	navigation::{announce_for_command, announce_for_selection_command},
	selection::copied_announcement,
};

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

/// Selects everything the reading control holds, and says how much text the document has.
///
/// Announces for the same reason the copy above does, and one more. As the only feedback the command
/// gave the reader, a screen reader's own report of the selection change went out the instant the
/// menu closed and the focus-return chain read over it; saying it here gives a menu click the same
/// delayed announcement every other command gets.
///
/// The shortcut waits as well, which the copy's does not. Selecting is not silent: the change is
/// itself reported, and a message raised the moment the command runs is spoken first and leaves that
/// report hanging on the end.
pub fn handle_select_all(dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText, from_keyboard: bool) {
	let Some(count) = dm.lock().unwrap().select_all_and_count() else {
		return;
	};
	announce_for_selection_command(live_region_label, from_keyboard, selected_message(count));
}

/// Its own function of the count so the wording can be pinned without a document, which a test cannot build.
fn selected_message(count: usize) -> String {
	// TRANSLATORS: Announced after Select All, from the Edit menu or with Ctrl+A. The %d placeholder is replaced with the number of characters in the document. Plural form is chosen by that count.
	nt("Selected %d character.", "Selected %d characters.", u64::try_from(count).unwrap_or(0)).replacen(
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

	#[test]
	fn a_select_all_says_how_many_characters_the_document_has() {
		assert_eq!(selected_message(42), "Selected 42 characters.");
	}

	/// One is read aloud by a person, not measured by a buffer.
	#[test]
	fn a_select_all_of_one_character_is_not_plural() {
		assert_eq!(selected_message(1), "Selected 1 character.");
	}
}
