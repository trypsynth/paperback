use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_confirms, build_ok_cancel_buttons};

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
	bind_enter_confirms(&dialog, input_ctrl);
	let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	input_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	input_sizer.add(&input_ctrl, 1, SizerFlag::Expand, 0);
	// TRANSLATORS: OK button that confirms the entered sleep timer duration
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("OK"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	input_ctrl.set_focus();
	if dialog.show_modal() == ID_OK { Some(input_ctrl.value()) } else { None }
}
