use std::{cell::Cell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

pub fn show_go_to_line_dialog(
	parent: &Frame,
	current_line: i32,
	max_lines: i32,
	live_region_label: StaticText,
) -> Option<i32> {
	let max_lines = max_lines.max(1);
	// TRANSLATORS: Title of the Go to Line dialog
	let dialog_title = t("Go to Line");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	// TRANSLATORS: Label/prompt template for the line selection dialog. The %d placeholders represent current_line and max_lines respectively.
	let label_template = t("Go to line (%d/%d):");
	let label_text = label_template.replacen("%d", &current_line.clamp(1, max_lines).to_string(), 1).replacen(
		"%d",
		&max_lines.to_string(),
		1,
	);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let current = current_line.clamp(1, max_lines);
	// A plain text field rather than a spin control: out-of-range lines must be rejected,
	// not silently clamped, and (later) relative +n/-n input has no spinner equivalent.
	let line_ctrl =
		TextCtrl::builder(&dialog).with_value(&current.to_string()).with_style(TextCtrlStyle::ProcessEnter).build();
	let result = Rc::new(Cell::new(None::<i32>));
	// Enter in the field and the Go button both submit through the same validation.
	let line_ctrl_for_enter = line_ctrl;
	let result_for_enter = Rc::clone(&result);
	let dialog_for_enter = dialog;
	line_ctrl_for_enter.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		submit_go_to_line(dialog_for_enter, line_ctrl_for_enter, &result_for_enter, live_region_label, max_lines);
	});
	// TRANSLATORS: Label for the button that jumps to the entered line
	let ok_button = Button::builder(&dialog).with_label(&t("Go")).build();
	// TRANSLATORS: Label for the button that closes the dialog without navigating
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	let line_ctrl_for_ok = line_ctrl;
	let result_for_ok = Rc::clone(&result);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		submit_go_to_line(dialog_for_ok, line_ctrl_for_ok, &result_for_ok, live_region_label, max_lines);
	});
	let line_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	line_sizer.add(&line_ctrl, 1, SizerFlag::Expand, 0);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&line_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	line_ctrl.set_focus();
	line_ctrl.select_all();
	if dialog.show_modal() == ID_OK { result.get() } else { None }
}

/// Resolves the entered `text` to a line number in `1..=max_lines`, or `None` when the text
/// is not a bare line number or the line is out of range.
fn resolve_line(text: &str, max_lines: i32) -> Option<i32> {
	let line = text.trim().parse::<i64>().ok()?;
	if (1..=i64::from(max_lines)).contains(&line) { i32::try_from(line).ok() } else { None }
}

/// Validates the field and either confirms the dialog with the resolved line or, when the
/// entry is invalid, announces the rejection and keeps the dialog open so the user can retry.
fn submit_go_to_line(
	dialog: Dialog,
	line_ctrl: TextCtrl,
	result: &Rc<Cell<Option<i32>>>,
	live_region_label: StaticText,
	max_lines: i32,
) {
	let Some(line) = resolve_line(&line_ctrl.get_value(), max_lines) else {
		// TRANSLATORS: Announced when the entered line number is outside the document's range
		live_region::announce(live_region_label, &t("Line out of range."));
		line_ctrl.set_focus();
		line_ctrl.select_all();
		return;
	};
	result.set(Some(line));
	dialog.end_modal(ID_OK);
}
