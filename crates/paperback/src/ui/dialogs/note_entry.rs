use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, build_ok_cancel_buttons};

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
		.with_size(Size::new(400, 200))
		.build();
	let (ok_button, cancel_button) = build_ok_cancel_buttons(dialog, &t("OK"));
	let dialog_for_key = dialog;
	note_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code()
			&& key == WXK_RETURN
		{
			if event.shift_down() {
				event.skip(true);
			} else {
				dialog_for_key.end_modal(ID_OK);
				event.skip(false);
			}
			return;
		}
		event.skip(true);
	});
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
