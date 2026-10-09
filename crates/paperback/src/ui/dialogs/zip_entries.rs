use std::{cell::Cell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, build_ok_cancel_buttons};

pub enum ZipChoice {
	Entry(usize),
	Cancelled,
	/// Kept apart from `Cancelled` because closing the window is how a user leaves when the picker is all that opened at startup.
	Closed,
}

pub fn show_zip_entries_dialog(parent: &Frame, names: &[String]) -> ZipChoice {
	// TRANSLATORS: Title of the dialog listing the documents inside a zip archive
	let title = t("Open from Archive");
	let dialog = Dialog::builder(parent, &title).build();
	// TRANSLATORS: Label for the list of documents inside a zip archive
	let list_label_text = t("&Documents in this archive:");
	let list_label = StaticText::builder(&dialog).with_label(&list_label_text).build();
	let list = ListBox::builder(&dialog).build();
	#[cfg(target_os = "macos")]
	list.set_accessibility_label(list_label_text.replace('&', "").trim_end_matches(':').trim());
	for name in names {
		list.append(name);
	}
	list.set_selection(0, true);
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("&Open"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&list_label, 0, SizerFlag::All, DIALOG_PADDING / 2);
	content_sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	list.on_item_double_clicked(move |_| dialog.end_modal(ID_OK));
	let closed = Rc::new(Cell::new(false));
	let closed_in_handler = Rc::clone(&closed);
	dialog.on_close(move |event| {
		closed_in_handler.set(true);
		dialog.end_modal(ID_CANCEL);
		event.skip(false);
	});
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	list.set_focus();
	if dialog.show_modal() != ID_OK {
		return if closed.get() { ZipChoice::Closed } else { ZipChoice::Cancelled };
	}
	list.get_selection().map_or(ZipChoice::Cancelled, |index| ZipChoice::Entry(index as usize))
}
