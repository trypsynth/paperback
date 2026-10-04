//! The marked selection and the copy it makes possible: a reader's stand-in for dragging a
//! selection across text they cannot see, in place of shift-arrowing through it a line at a time.
//!
//! The mark is a document-absolute display-unit position rather than a text-control position, so
//! it survives the window reloads a huge document goes through on the way, and the text is read
//! back out of the session rather than out of the control, which only ever holds a slice of one.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::{clipboard::Clipboard, prelude::*};

use super::{document_manager::DocumentManager, navigation};
use crate::ui::navigation::announce_for_command;

/// What the copy shortcut found when it went to copy.
enum CopyOutcome {
	/// No beginning has been marked.
	NoMark,
	/// The mark and the caret are the same place, so there is nothing between them.
	Empty,
	/// The text to put on the clipboard.
	Text(String),
}

/// Orders the mark and the caret into document order.
///
/// Marking a place, reading back past it and pressing the copy key there gets the text between
/// the two rather than nothing: the two presses name a range, the same way dragging a selection
/// upwards does. Both of these are positions the reader is *on* rather than bounds - the caller
/// turns them into a range that can be extracted.
const fn ordered_ends(mark: i64, caret: i64) -> (i64, i64) {
	if mark <= caret { (mark, caret) } else { (caret, mark) }
}

/// What the copy shortcut says once it has copied `count` characters.
fn copied_announcement(count: usize) -> String {
	// TRANSLATORS: Announced after copying from the beginning of a marked selection. %d is the number of characters copied to the clipboard.
	t("Copied (%d chars).").replacen("%d", &count.to_string(), 1)
}

/// Marks the reader's current position as the beginning of a selection to copy from later.
pub fn handle_set_selection_start(dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText, from_keyboard: bool) {
	{
		let dm = dm.lock().unwrap();
		let Some(position) = dm.active_tab().map(navigation::doc_caret) else {
			return;
		};
		dm.set_selection_mark(Some(position));
	}
	// TRANSLATORS: Announced when the reader marks the beginning of a selection to copy from later.
	announce_for_command(live_region_label, from_keyboard, t("Beginning of selection set."));
}

/// Copies everything between the marked beginning and the reader's current position.
pub fn handle_copy_from_selection_start(
	dm: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
	// Everything that touches the document happens under the lock; the clipboard write and the
	// announcement deliberately do not. `Clipboard::set_text` goes through OLE on Windows, which
	// can pump messages, and the document manager's mutex is not reentrant - holding it across a
	// pumped message that comes back into a handler locking it would deadlock the app.
	let outcome = {
		let dm = dm.lock().unwrap();
		let Some(tab) = dm.active_tab() else {
			return;
		};
		match dm.selection_mark() {
			None => CopyOutcome::NoMark,
			Some(mark) => {
				// The gesture is spent either way, so a second press with no fresh mark says so
				// rather than quietly copying again from a mark the reader has moved past.
				dm.set_selection_mark(None);
				let (from, to) = ordered_ends(mark, navigation::doc_caret(tab));
				// Both ends are included, because both name a character the reader is on. A
				// caret sits at the start of the character a screen reader is speaking, so
				// ending the range there would drop the last character read - the one the
				// reader deliberately stopped on. This reaches past it instead, spanning the
				// whole character where one is an astral-plane pair.
				let end = tab.session.display_pos_after_char_at(to);
				let text = tab.session.get_text_range_display(from, end);
				if text.is_empty() { CopyOutcome::Empty } else { CopyOutcome::Text(text) }
			}
		}
	};
	let message = match outcome {
		// TRANSLATORS: Announced when the copy-from-selection shortcut is pressed before any beginning of a selection has been marked.
		CopyOutcome::NoMark => t("Cannot copy; set beginning of selection first."),
		// TRANSLATORS: Announced when the beginning of the selection was marked at the reader's current position, leaving nothing between the two to copy.
		CopyOutcome::Empty => t("Nothing to copy."),
		CopyOutcome::Text(text) => {
			// Counted in characters, not in the display units the range is measured in: a spoken
			// count is for a person listening, and an astral-plane character is two display units
			// but one character to everyone who is not a text buffer.
			let count = text.chars().count();
			if Clipboard::get().set_text(&text) {
				copied_announcement(count)
			} else {
				// TRANSLATORS: Announced when the copy ran but the text could not be put on the clipboard, so nothing was copied.
				t("Could not copy to the clipboard.")
			}
		}
	};
	announce_for_command(live_region_label, from_keyboard, message);
}

/// Moves the reader back to the marked beginning, leaving the mark in place so the copy and the
/// jump can be used in either order, as often as they like.
pub fn handle_jump_to_selection_start(
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
	let (message, history_update) = {
		let mut dm = dm.lock().unwrap();
		let Some(tab) = dm.active_tab_mut() else {
			return;
		};
		match tab.selection_mark.get() {
			None => (
				// TRANSLATORS: Announced when the shortcut that jumps back to the beginning of a marked selection is pressed with no beginning marked.
				t("Cannot jump; set beginning of selection first."),
				None,
			),
			Some(mark) => {
				let update = navigation::move_to_offset_and_record_history(tab, mark);
				// TRANSLATORS: Announced after jumping back to the marked beginning of the selection.
				(t("Jumped to beginning of selection."), tab.track.then_some(update))
			}
		}
	};
	announce_for_command(live_region_label, from_keyboard, message);
	navigation::persist_navigation_history(config, history_update.as_ref());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ordered_ends_normalizes_to_document_order() {
		assert_eq!(ordered_ends(5, 9), (5, 9));
		assert_eq!(ordered_ends(9, 5), (5, 9));
		assert_eq!(ordered_ends(4, 4), (4, 4));
	}

	#[test]
	fn copied_announcement_names_the_character_count() {
		assert_eq!(copied_announcement(42), "Copied (42 chars).");
		assert_eq!(copied_announcement(0), "Copied (0 chars).");
	}
}
