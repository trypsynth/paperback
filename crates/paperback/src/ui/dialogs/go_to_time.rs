use std::{cell::Cell, rc::Rc};

use paperback_core::audio::clock::{self, TimeEntryError};
use patois::t;
use wxdragon::prelude::*;

use super::go_to::add_go_cancel_footer;
use crate::ui::navigation::announce;

/// A plain text field, since a time needs the colons and decimal point that the other Go To dialogs' number fields refuse.
pub fn show_go_to_time_dialog(
	parent: &Frame,
	current_ms: u64,
	total_ms: u64,
	live_region_label: StaticText,
) -> Option<u64> {
	// TRANSLATORS: Title of the Go to Time dialog, which jumps to a time typed in, in an audiobook
	let title = t("Go to Time");
	let dialog = Dialog::builder(parent, &title).build();
	let _destroy = super::DestroyOnDrop(dialog);
	let padding = super::dialog_padding(&dialog);
	let current = clock::format_ms(current_ms);
	// TRANSLATORS: Label of the field in the Go to Time dialog. The first {} is the time played so far and the second is the recording's length, both written like 1:02:03
	let label_text =
		t("Go to time ({}/{}):").replacen("{}", &current, 1).replacen("{}", &clock::format_ms(total_ms), 1);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let entry = TextCtrl::builder(&dialog).with_value(&current).with_style(TextCtrlStyle::ProcessEnter).build();
	let entry_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	entry_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	entry_sizer.add(&entry, 1, SizerFlag::Expand, 0);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&entry_sizer, 0, SizerFlag::Expand | SizerFlag::All, padding);
	let result = Rc::new(Cell::new(None));
	let result_for_submit = Rc::clone(&result);
	add_go_cancel_footer(dialog, content_sizer, entry, move || {
		match clock::resolve_entry_ms(&entry.get_value(), current_ms, total_ms) {
			Ok(target_ms) => {
				result_for_submit.set(Some(target_ms));
				dialog.end_modal(ID_OK);
			}
			Err(error) => {
				let message = match error {
					// TRANSLATORS: Announced when what was typed in the Go to Time dialog is not a time
					TimeEntryError::NotATime => t("Type a time like 1:23:45, 23:45 or 45."),
					// TRANSLATORS: Announced when the time typed in the Go to Time dialog is past the end of the recording, or before its start
					TimeEntryError::OutOfRange => t("Time out of range."),
				};
				announce(live_region_label, message);
				entry.set_focus();
				entry.select_all();
			}
		}
	});
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	entry.set_focus();
	entry.select_all();
	if dialog.show_modal() == ID_OK { result.get() } else { None }
}
