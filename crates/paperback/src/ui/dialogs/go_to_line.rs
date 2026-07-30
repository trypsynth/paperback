use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

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
	let dialog_for_enter = dialog;
	line_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(ID_OK);
	});
	let line_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	line_sizer.add(&line_ctrl, 1, SizerFlag::Expand, 0);
	// TRANSLATORS: Label for the button that jumps to the entered position (a line, page, or percentage, depending on the dialog)
	let ok_button = Button::builder(&dialog).with_id(ID_OK).with_label(&t("Go")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&line_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	line_ctrl.set_focus();
	if dialog.show_modal() == ID_OK { Some(line_ctrl.value().clamp(1, max_lines)) } else { None }
}
