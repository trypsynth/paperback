use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

pub fn show_sleep_timer_dialog(parent: &Frame, initial_duration: i32) -> Option<i32> {
	// TRANSLATORS: Title of the Sleep Timer dialog
	let dialog = Dialog::builder(parent, &t("Sleep Timer")).build();
	// TRANSLATORS: Label for the input field where users enter the number of minutes for the sleep timer.
	let label = StaticText::builder(&dialog).with_label(&t("&Minutes:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, 999)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(initial_duration.clamp(1, 999));
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(ID_OK);
	});
	let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	input_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	input_sizer.add(&input_ctrl, 1, SizerFlag::Expand, 0);
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&dialog).with_id(ID_OK).with_label(&t("OK")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	input_ctrl.set_focus();
	if dialog.show_modal() == ID_OK { Some(input_ctrl.value()) } else { None }
}
