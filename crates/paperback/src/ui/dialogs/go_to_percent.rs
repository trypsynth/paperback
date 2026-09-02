use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_confirms, build_ok_cancel_buttons};

pub fn show_go_to_percent_dialog(parent: &Frame, current_percent: i32) -> Option<i32> {
	// TRANSLATORS: Title of the Go to Percent dialog
	let dialog_title = t("Go to Percent");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let current_percent = current_percent.clamp(0, 100);
	// TRANSLATORS: Label for the percentage selection slider
	let slider_label = StaticText::builder(&dialog).with_label(&t("&Percent")).build();
	let percent_slider =
		Slider::builder(&dialog).with_value(current_percent).with_min_value(0).with_max_value(100).build();
	// TRANSLATORS: Label for the numeric percentage entry field
	let input_label = StaticText::builder(&dialog).with_label(&t("P&ercent:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(0, 100)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(current_percent);
	let input_ctrl_for_slider = input_ctrl;
	percent_slider.on_slider(move |event| {
		input_ctrl_for_slider.set_value(event.get_value());
	});
	let percent_slider_for_spin = percent_slider;
	input_ctrl.on_value_changed(move |event| {
		percent_slider_for_spin.set_value(event.get_value());
	});
	bind_enter_confirms(&dialog, input_ctrl);
	let dialog_for_slider_enter = dialog;
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == WXK_RETURN || key == WXK_NUMPAD_ENTER {
			event.skip(false);
			dialog_for_slider_enter.end_modal(ID_OK);
			return;
		}
		event.skip(true);
	});
	let percent_slider_for_keys = percent_slider;
	let input_ctrl_for_keys = input_ctrl;
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
			input_ctrl_for_keys.set_value(val);
			event.skip(false);
		} else {
			event.skip(true);
		}
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&slider_label, 0, SizerFlag::Left | SizerFlag::Top, 5);
	content_sizer.add(&percent_slider, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	content_sizer.add(&input_label, 0, SizerFlag::Left, 5);
	content_sizer.add(&input_ctrl, 0, SizerFlag::Expand, 0);
	// TRANSLATORS: Label for the button that jumps to the entered position (a line, page, or percentage, depending on the dialog)
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("Go"));
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add_sizer(&content_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_ok_cancel_footer(main_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	percent_slider.set_focus();
	if dialog.show_modal() == ID_OK { Some(input_ctrl.value().clamp(0, 100)) } else { None }
}
