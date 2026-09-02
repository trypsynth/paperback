use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_confirms, build_ok_cancel_buttons};

pub fn show_go_to_line_dialog(parent: &Frame, current_line: i32, max_lines: i32) -> Option<i32> {
	// TRANSLATORS: Title of the Go to Line dialog
	let dialog_title = t("Go to Line");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	// TRANSLATORS: Label for the input field where users enter the target line number.
	let label_text = t("&Line number:");
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let max_lines = max_lines.max(1);
	let current_line = current_line.clamp(1, max_lines);
	let line_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_lines)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	line_ctrl.set_value(current_line);
	bind_enter_confirms(&dialog, line_ctrl);
	let line_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	line_sizer.add(&line_ctrl, 1, SizerFlag::Expand, 0);
	// TRANSLATORS: Label for the button that jumps to the entered position (a line, page, or percentage, depending on the dialog)
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("Go"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&line_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	line_ctrl.set_focus();
	if dialog.show_modal() == ID_OK { Some(line_ctrl.value().clamp(1, max_lines)) } else { None }
}
