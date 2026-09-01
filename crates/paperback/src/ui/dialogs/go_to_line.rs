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
	restrict_to_number_input(line_ctrl);
	let result = Rc::new(Cell::new(None::<i32>));
	// Enter in the field and the Go button both submit through the same validation.
	let line_ctrl_for_enter = line_ctrl;
	let result_for_enter = Rc::clone(&result);
	let dialog_for_enter = dialog;
	line_ctrl_for_enter.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		submit_go_to_line(
			dialog_for_enter,
			line_ctrl_for_enter,
			&result_for_enter,
			live_region_label,
			current_line,
			max_lines,
		);
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
		submit_go_to_line(dialog_for_ok, line_ctrl_for_ok, &result_for_ok, live_region_label, current_line, max_lines);
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
/// is not a valid line expression or resolves out of range.
///
/// A bare number is the line itself; a leading `+`/`-` moves that many lines forward or
/// backward from `current_line`, so "+5" is five lines ahead and "-3" three lines back.
fn resolve_line(text: &str, current_line: i32, max_lines: i32) -> Option<i32> {
	let trimmed = text.trim();
	if trimmed.is_empty() {
		return None;
	}
	let (sign, digits) = match trimmed.as_bytes()[0] {
		b'+' | b'-' => (trimmed.as_bytes()[0], &trimmed[1..]),
		_ => (0, trimmed),
	};
	let amount = digits.parse::<i64>().ok()?;
	let current = i64::from(current_line);
	let resolved = match sign {
		b'+' => current.checked_add(amount)?,
		b'-' => current.checked_sub(amount)?,
		_ => amount,
	};
	if (1..=i64::from(max_lines)).contains(&resolved) { i32::try_from(resolved).ok() } else { None }
}

/// Swallows any keystroke that cannot be part of a line expression: digits, `+`/`-` (for
/// relative jumps), and Enter (which submits). Letters and punctuation never reach the
/// field, so an invalid entry can't be typed in the first place.
///
/// Only `CHAR` events are filtered, which fire for character insertion. Everything that
/// is not a printable character passes through untouched, so every navigation, editing
/// and shortcut key keeps working: control codes below space (Tab, Backspace, Enter,
/// Escape), Delete, the special keys at [`WXK_START`] and above (arrows, Home/End, Page
/// keys, Insert, F-keys, numpad Enter), and all Ctrl/Cmd shortcuts.
fn restrict_to_number_input(ctrl: TextCtrl) {
	ctrl.bind_internal(EventType::CHAR, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		let allowed = event.control_down()
			|| event.cmd_down()
			|| key < i32::from(b' ')
			|| key == WXK_DELETE
			|| key >= WXK_START
			|| (i32::from(b'0')..=i32::from(b'9')).contains(&key)
			|| key == i32::from(b'+')
			|| key == i32::from(b'-');
		event.skip(allowed);
	});
}

/// Validates the field and either confirms the dialog with the resolved line or, when the
/// entry is invalid, announces the rejection and keeps the dialog open so the user can retry.
fn submit_go_to_line(
	dialog: Dialog,
	line_ctrl: TextCtrl,
	result: &Rc<Cell<Option<i32>>>,
	live_region_label: StaticText,
	current_line: i32,
	max_lines: i32,
) {
	let Some(line) = resolve_line(&line_ctrl.get_value(), current_line, max_lines) else {
		// TRANSLATORS: Announced when the entered line number is outside the document's range
		live_region::announce(live_region_label, &t("Line out of range."));
		line_ctrl.set_focus();
		line_ctrl.select_all();
		return;
	};
	result.set(Some(line));
	dialog.end_modal(ID_OK);
}

#[cfg(test)]
mod tests {
	use super::resolve_line;

	#[test]
	fn absolute_lines_resolve_and_reject_out_of_range() {
		assert_eq!(resolve_line("5", 10, 100), Some(5));
		assert_eq!(resolve_line(" 42 ", 10, 100), Some(42));
		assert_eq!(resolve_line("0", 10, 100), None);
		assert_eq!(resolve_line("101", 10, 100), None);
		assert_eq!(resolve_line("99999999999", 10, 100), None);
	}

	#[test]
	fn relative_offsets_resolve_from_the_current_line() {
		assert_eq!(resolve_line("+5", 10, 100), Some(15));
		assert_eq!(resolve_line("-3", 10, 100), Some(7));
		assert_eq!(resolve_line("+0", 10, 100), Some(10));
		assert_eq!(resolve_line("-0", 10, 100), Some(10));
	}

	#[test]
	fn relative_offsets_out_of_range_are_rejected() {
		assert_eq!(resolve_line("-20", 10, 100), None);
		assert_eq!(resolve_line("+100", 10, 100), None);
	}

	#[test]
	fn garbage_is_rejected() {
		assert_eq!(resolve_line("", 10, 100), None);
		assert_eq!(resolve_line("   ", 10, 100), None);
		assert_eq!(resolve_line("abc", 10, 100), None);
		assert_eq!(resolve_line("5+3", 10, 100), None);
		assert_eq!(resolve_line("+", 10, 100), None);
		assert_eq!(resolve_line("-", 10, 100), None);
	}
}
