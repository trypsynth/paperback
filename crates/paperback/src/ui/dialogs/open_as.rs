use std::path::Path;

use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;
use crate::accessibility;

pub fn show_open_as_dialog(parent: &Frame, path: &Path) -> Option<String> {
	let title = t("Open As");
	let dialog = Dialog::builder(parent, &title).build();
	let message_template = t("No suitable parser was found for {}.\nHow would you like to open this file?");
	let message = message_template.replace("{}", &path.display().to_string());
	let label = StaticText::builder(&dialog).with_label(&message).build();
	let format_label_text = t("Open &as:");
	let format_label = StaticText::builder(&dialog).with_label(&format_label_text).build();
	let format_combo = Choice::builder(&dialog).build();
	format_combo.append(&t("Plain Text"));
	format_combo.append(&t("HTML"));
	format_combo.append(&t("Markdown"));
	format_combo.set_selection(0);
	accessibility::set_label(&format_combo, format_label_text.replace('&', "").trim_end_matches(':').trim());
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
	let cancel_label = t("Cancel");
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&cancel_label).build();
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&label, 0, SizerFlag::All, DIALOG_PADDING / 2);
	let format_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	format_sizer.add(&format_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	format_sizer.add(&format_combo, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&format_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	format_combo.set_focus();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let selection = format_combo.get_selection();
	let format = match selection {
		Some(1) => "html",
		Some(2) => "md",
		_ => "txt",
	};
	Some(format.to_string())
}
