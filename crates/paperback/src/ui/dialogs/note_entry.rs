use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_submits, build_ok_cancel_buttons};

pub fn show_note_entry_dialog(
	parent: &dyn WxWidget,
	title: &str,
	message: &str,
	existing_note: &str,
) -> Option<String> {
	let dialog = Dialog::builder(parent, title).build();
	let message_label = StaticText::builder(&dialog).with_label(message).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(existing_note)
		.with_style(TextCtrlStyle::MultiLine)
		.with_size(dialog.from_dip(Size::new(400, 200)))
		.build();
	// TRANSLATORS: OK button that confirms the entered bookmark/note text
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("OK"));
	bind_enter_submits(dialog, note_ctrl);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message_label, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&note_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	if dialog.show_modal() == ID_OK { Some(note_ctrl.get_value()) } else { None }
}
