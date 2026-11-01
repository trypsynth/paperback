//! The mark is a document position so it survives window reloads, and text is read from the session since the control only holds a slice.

use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, document::DocPos};
use patois::{nt, t};
use wxdragon::{clipboard::Clipboard, prelude::*};

use super::{document_manager::DocumentManager, navigation};
use crate::ui::navigation::announce_for_command;

enum CopyOutcome {
	NoMark,
	Empty,
	Text(String),
}

fn ordered_ends(mark: DocPos, caret: DocPos) -> (DocPos, DocPos) {
	if mark <= caret { (mark, caret) } else { (caret, mark) }
}

/// Shared with plain Ctrl+C, since both are the same action to the reader.
pub fn copied_announcement(count: usize) -> String {
	// TRANSLATORS: Announced after text is copied, with Ctrl+C or from the marked beginning of a selection. The %d placeholder is replaced with the number of characters copied. Plural form is chosen by that count.
	nt("%d character copied.", "%d characters copied.", u64::try_from(count).unwrap_or(0)).replacen(
		"%d",
		&count.to_string(),
		1,
	)
}

/// Flushed because on Windows wx hands the clipboard a data object Paperback owns, which goes when the app exits; a failed flush still leaves the text while running, so only the write decides success.
pub fn copy_to_clipboard(text: &str) -> bool {
	let clipboard = Clipboard::get();
	let copied = clipboard.set_text(text);
	if copied {
		clipboard.flush();
	}
	copied
}

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

pub fn handle_copy_from_selection_start(
	dm: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
	// The clipboard write stays outside the lock: OLE can pump messages on Windows, and a handler re-locking this non-reentrant mutex would deadlock.
	let outcome = {
		let dm = dm.lock().unwrap();
		let Some(tab) = dm.active_tab() else {
			return;
		};
		match dm.selection_mark() {
			None => CopyOutcome::NoMark,
			Some(mark) => {
				// Spent either way, so a second press says so rather than quietly copying from a mark the reader has moved past.
				dm.set_selection_mark(None);
				let (from, to) = ordered_ends(mark, navigation::doc_caret(tab));
				// Inclusive: the caret sits at the start of the character being spoken, so ending there would drop the one the reader stopped on.
				let end = tab.session.pos_after_char_at(to);
				let text = tab.session.text_range(from, end);
				if text.is_empty() { CopyOutcome::Empty } else { CopyOutcome::Text(text) }
			}
		}
	};
	let message = match outcome {
		// TRANSLATORS: Announced when the copy-from-selection shortcut is pressed before any beginning of a selection has been marked.
		CopyOutcome::NoMark => t("Cannot copy; set beginning of selection first."),
		// TRANSLATORS: Announced when a copy had nothing to copy: no text was selected, or the marked beginning of a selection is at the reader's own position, leaving nothing between the two.
		CopyOutcome::Empty => t("Nothing to copy."),
		CopyOutcome::Text(text) => {
			// Counted in chars: an astral-plane character is one to a listener, though two of a text control's units.
			let count = text.chars().count();
			if copy_to_clipboard(&text) {
				copied_announcement(count)
			} else {
				// TRANSLATORS: Announced when the copy ran but the text could not be put on the clipboard, so nothing was copied.
				t("Could not copy to the clipboard.")
			}
		}
	};
	announce_for_command(live_region_label, from_keyboard, message);
}

/// Leaves the mark in place so the copy and the jump can be used in either order, repeatedly.
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
		let (four, five, nine) = (DocPos::from_raw(4), DocPos::from_raw(5), DocPos::from_raw(9));
		assert_eq!(ordered_ends(five, nine), (five, nine));
		assert_eq!(ordered_ends(nine, five), (five, nine));
		assert_eq!(ordered_ends(four, four), (four, four));
	}

	#[test]
	fn copied_announcement_names_the_character_count() {
		assert_eq!(copied_announcement(42), "42 characters copied.");
		assert_eq!(copied_announcement(0), "0 characters copied.");
	}

	#[test]
	fn copied_announcement_singular_for_one_character() {
		assert_eq!(copied_announcement(1), "1 character copied.");
	}
}
