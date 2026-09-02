use patois::t;
use wx_utils::dpi;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_single_button_footer};

pub fn show_view_note_dialog(parent: &dyn WxWidget, note_text: &str) {
	// TRANSLATORS: Title of the View Note dialog
	let dialog = Dialog::builder(parent, &t("View Note")).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(note_text)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(dpi::scale_size(&dialog, Size::new(400, 200)))
		.build();
	// TRANSLATORS: Label for a button that closes the View Note dialog
	let close_button = Button::builder(&dialog).with_id(ID_OK).with_label(&t("Close")).build();
	dialog.set_affirmative_id(ID_OK);
	let dialog_for_close = dialog;
	close_button.on_click(move |_| {
		dialog_for_close.end_modal(ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&note_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_single_button_footer(content_sizer, close_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	dialog.show_modal();
}
