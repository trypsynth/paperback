use std::{cell::Cell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

pub fn show_go_to_percent_dialog(parent: &Frame, current_percent: i32, live_region_label: StaticText) -> Option<i32> {
	let current_percent = current_percent.clamp(0, 100);
	// TRANSLATORS: Title of the Go to Percent dialog
	let dialog_title = t("Go to Percent");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	// TRANSLATORS: Label/prompt template for the percentage selection dialog. The %d placeholders represent current_percent and 100 respectively.
	let label_template = t("Go to percent (%d/%d):");
	let label_text = label_template.replacen("%d", &current_percent.to_string(), 1).replacen("%d", "100", 1);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	// TRANSLATORS: Label for the percentage selection slider
	let slider_label = StaticText::builder(&dialog).with_label(&t("&Percent")).build();
	let percent_slider =
		Slider::builder(&dialog).with_value(current_percent).with_min_value(0).with_max_value(100).build();
	// A plain text field rather than a spin control: out-of-range percentages must be
	// rejected, not silently clamped, and (later) relative +n/-n input has no spinner
	// equivalent.
	let input_ctrl = TextCtrl::builder(&dialog)
		.with_value(&current_percent.to_string())
		.with_style(TextCtrlStyle::ProcessEnter)
		.build();
	restrict_to_number_input(input_ctrl);
	let result = Rc::new(Cell::new(None::<i32>));
	// Keep the slider and the field in step. Dragging the slider rewrites the field
	// (`change_value` fires no event, so there is no loop); typing moves the slider to the
	// number that was entered.
	let input_for_slider = input_ctrl;
	percent_slider.on_slider(move |event| {
		input_for_slider.change_value(&event.get_value().to_string());
	});
	let slider_for_input = percent_slider;
	input_ctrl.bind_internal(EventType::TEXT, move |_event| {
		if let Some(resolved) = resolve_percent(&input_ctrl.get_value(), current_percent) {
			slider_for_input.set_value(resolved);
		}
	});
	// Enter in the field, Enter on the slider, and the Go button all submit through the
	// same validation.
	let input_for_enter = input_ctrl;
	let result_for_enter = Rc::clone(&result);
	let dialog_for_enter = dialog;
	input_for_enter.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		submit_go_to_percent(dialog_for_enter, input_for_enter, &result_for_enter, live_region_label, current_percent);
	});
	// TRANSLATORS: Label for the button that jumps to the entered percentage
	let ok_button = Button::builder(&dialog).with_label(&t("Go")).build();
	// TRANSLATORS: Label for the button that closes the dialog without navigating
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	let input_for_ok = input_ctrl;
	let result_for_ok = Rc::clone(&result);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		submit_go_to_percent(dialog_for_ok, input_for_ok, &result_for_ok, live_region_label, current_percent);
	});
	// Keyboard control of the slider: arrows/Page/Home/End move it, Enter submits.
	let input_for_slider_enter = input_ctrl;
	let result_for_slider_enter = Rc::clone(&result);
	let dialog_for_slider_enter = dialog;
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == WXK_RETURN || key == WXK_NUMPAD_ENTER {
			event.skip(false);
			submit_go_to_percent(
				dialog_for_slider_enter,
				input_for_slider_enter,
				&result_for_slider_enter,
				live_region_label,
				current_percent,
			);
			return;
		}
		event.skip(true);
	});
	let percent_slider_for_keys = percent_slider;
	let input_for_char = input_ctrl;
	percent_slider.bind_internal(EventType::CHAR, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		let current = percent_slider_for_keys.value();
		let min_val = percent_slider_for_keys.min();
		let max_val = percent_slider_for_keys.max();
		let new_value = match key {
			WXK_UP | WXK_RIGHT => Some((current + 1).min(max_val)),
			WXK_DOWN | WXK_LEFT => Some((current - 1).max(min_val)),
			WXK_PAGEUP => Some((current + 10).min(max_val)),
			WXK_PAGEDOWN => Some((current - 10).max(min_val)),
			WXK_HOME => Some(min_val),
			WXK_END => Some(max_val),
			_ => None,
		};
		if let Some(val) = new_value {
			percent_slider_for_keys.set_value(val);
			input_for_char.change_value(&val.to_string());
			event.skip(false);
		} else {
			event.skip(true);
		}
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&slider_label, 0, SizerFlag::Left | SizerFlag::Top, 5);
	content_sizer.add(&percent_slider, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	content_sizer.add(&label, 0, SizerFlag::Left, 5);
	content_sizer.add(&input_ctrl, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	// Focus the numeric entry so a screen-reader user can type a percentage directly.
	// Tab order is untouched (it follows widget creation order), so the slider remains the
	// first tab stop, matching the order the controls appear on screen.
	input_ctrl.set_focus();
	input_ctrl.select_all();
	if dialog.show_modal() == ID_OK { result.get() } else { None }
}

/// Resolves the entered `text` to a percentage in `0..=100`, or `None` when the text is
/// not a valid percentage expression or resolves out of range.
///
/// A bare number is the percentage itself; a leading `+`/`-` moves that many percentage
/// points forward or backward from `current_percent`, so "+10" is ten points ahead and
/// "-5" five points back.
fn resolve_percent(text: &str, current_percent: i32) -> Option<i32> {
	let trimmed = text.trim();
	if trimmed.is_empty() {
		return None;
	}
	let (sign, digits) = match trimmed.as_bytes()[0] {
		b'+' | b'-' => (trimmed.as_bytes()[0], &trimmed[1..]),
		_ => (0, trimmed),
	};
	let amount = digits.parse::<i64>().ok()?;
	let current = i64::from(current_percent);
	let resolved = match sign {
		b'+' => current.checked_add(amount)?,
		b'-' => current.checked_sub(amount)?,
		_ => amount,
	};
	if (0..=100).contains(&resolved) { i32::try_from(resolved).ok() } else { None }
}

/// Swallows any keystroke that cannot be part of a percentage expression: digits, `+`/`-`
/// (for relative jumps), and Enter (which submits). Letters and punctuation never reach
/// the field, so an invalid entry can't be typed in the first place.
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

/// Validates the field and either confirms the dialog with the resolved percentage or, when
/// the entry is invalid, announces the rejection and keeps the dialog open so the user can retry.
fn submit_go_to_percent(
	dialog: Dialog,
	input_ctrl: TextCtrl,
	result: &Rc<Cell<Option<i32>>>,
	live_region_label: StaticText,
	current_percent: i32,
) {
	let Some(percent) = resolve_percent(&input_ctrl.get_value(), current_percent) else {
		// TRANSLATORS: Announced when the entered percentage is outside the 0 to 100 range
		live_region::announce(live_region_label, &t("Percent out of range."));
		input_ctrl.set_focus();
		input_ctrl.select_all();
		return;
	};
	result.set(Some(percent));
	dialog.end_modal(ID_OK);
}

#[cfg(test)]
mod tests {
	use super::resolve_percent;

	#[test]
	fn absolute_percentages_resolve_and_reject_out_of_range() {
		assert_eq!(resolve_percent("45", 40), Some(45));
		assert_eq!(resolve_percent(" 0 ", 40), Some(0));
		assert_eq!(resolve_percent("100", 40), Some(100));
		assert_eq!(resolve_percent("101", 40), None);
		assert_eq!(resolve_percent("99999999999", 40), None);
	}

	#[test]
	fn relative_offsets_resolve_from_the_current_percentage() {
		assert_eq!(resolve_percent("+10", 45), Some(55));
		assert_eq!(resolve_percent("-5", 45), Some(40));
		assert_eq!(resolve_percent("+0", 45), Some(45));
	}

	#[test]
	fn relative_offsets_out_of_range_are_rejected() {
		assert_eq!(resolve_percent("+60", 45), None);
		assert_eq!(resolve_percent("-60", 45), None);
	}

	#[test]
	fn garbage_is_rejected() {
		assert_eq!(resolve_percent("", 45), None);
		assert_eq!(resolve_percent("   ", 45), None);
		assert_eq!(resolve_percent("abc", 45), None);
		assert_eq!(resolve_percent("4+5", 45), None);
		assert_eq!(resolve_percent("+", 45), None);
		assert_eq!(resolve_percent("-", 45), None);
	}
}
