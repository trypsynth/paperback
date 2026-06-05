use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

pub fn show_view_note_dialog(parent: &dyn WxWidget, note_text: &str) {
	let dialog = Dialog::builder(parent, &t("View Note")).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(note_text)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(Size::new(400, 200))
		.build();
	let close_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("Close")).build();
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let dialog_for_close = dialog;
	close_button.on_click(move |_| {
		dialog_for_close.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&note_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	dialog.show_modal();
}
