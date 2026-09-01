use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, build_ok_cancel_buttons};

/// One line item in the Linux file-association dialog: either a real document format from
/// `paperback_formats::ALL`, or the standalone ZIP entry shared by DAISY and Word-in-zip. See
/// `FormatMeta::mime_types`'s doc comment for why zip isn't part of either format's own entry.
pub struct AssociationChoice {
	pub label: String,
	pub mime_types: &'static [&'static str],
	pub default_checked: bool,
}

/// Shows the one-time "set up file associations" dialog offered the first time Paperback runs
/// from an AppImage, mirroring the Windows installer's per-format Tasks page. Returns `None` if
/// the user dismissed the dialog without applying anything (Cancel/Escape), or `Some` with one
/// bool per `choices` entry, in the same order, if they confirmed.
pub fn show_linux_setup_dialog(parent: &Frame, choices: &[AssociationChoice]) -> Option<Vec<bool>> {
	// TRANSLATORS: Title of the one-time dialog offering to associate Paperback with file types, shown the first time it's run from an AppImage
	let title = t("Set Up Paperback");
	let dialog = Dialog::builder(parent, &title).build();
	// TRANSLATORS: Explains the checkbox list below in the Linux file-association setup dialog
	let intro_text = t("Choose which file types should open in Paperback:");
	let intro = StaticText::builder(&dialog).with_label(&intro_text).build();
	let list = CheckListBox::builder(&dialog).with_size(Size::new(420, 320)).build();
	for choice in choices {
		list.append(&choice.label);
	}
	for (index, choice) in choices.iter().enumerate() {
		list.check(u32::try_from(index).unwrap_or(u32::MAX), choice.default_checked);
	}
	// TRANSLATORS: Confirms the file association choices in the Linux setup dialog and applies them
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("Set Up"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&intro, 0, SizerFlag::All, DIALOG_PADDING / 2);
	content_sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	list.set_focus();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	Some((0..choices.len()).map(|index| list.is_checked(u32::try_from(index).unwrap_or(u32::MAX))).collect())
}
