use wxdragon::{prelude::*, translations::translate as t};

use super::{DIALOG_PADDING, KEY_NUMPAD_ENTER, KEY_RETURN};

const WXK_END: i32 = 312;
const WXK_HOME: i32 = 313;
const WXK_LEFT: i32 = 314;
const WXK_UP: i32 = 315;
const WXK_RIGHT: i32 = 316;
const WXK_DOWN: i32 = 317;
const WXK_PAGEUP: i32 = 366;
const WXK_PAGEDOWN: i32 = 367;

pub fn show_go_to_percent_dialog(parent: &Frame, current_percent: i32) -> Option<i32> {
	let dialog_title = t("Go to Percent");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let current_percent = current_percent.clamp(0, 100);
	let slider_label = StaticText::builder(&dialog).with_label(&t("&Percent")).build();
	let percent_slider =
		Slider::builder(&dialog).with_value(current_percent).with_min_value(0).with_max_value(100).build();
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
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_slider_enter = dialog;
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
			event.skip(false);
			dialog_for_slider_enter.end_modal(wxdragon::id::ID_OK);
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
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	ok_button.set_default();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add_sizer(&content_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	percent_slider.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(input_ctrl.value().clamp(0, 100)) } else { None }
}
