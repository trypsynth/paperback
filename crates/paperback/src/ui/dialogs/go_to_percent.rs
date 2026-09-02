use std::rc::Rc;

use patois::t;
use wxdragon::prelude::*;

use super::go_to::{NumberEntry, add_go_cancel_footer};

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
	let entry = Rc::new(NumberEntry::new(dialog, current_percent, 0, 100));
	// Keep the slider and the field in step. Dragging the slider rewrites the field
	// (`change_value` fires no event, so there is no loop); typing moves the slider to the
	// number that was entered.
	let input_for_slider = entry.ctrl;
	percent_slider.on_slider(move |event| {
		input_for_slider.change_value(&event.get_value().to_string());
	});
	let entry_for_text = Rc::clone(&entry);
	let slider_for_input = percent_slider;
	entry.ctrl.bind_internal(EventType::TEXT, move |_event| {
		if let Some(resolved) = entry_for_text.resolve() {
			slider_for_input.set_value(resolved);
		}
	});
	// Enter in the field, Enter on the slider, and the Go button all submit through the
	// same validation.
	// TRANSLATORS: Announced when the entered percentage is outside the 0 to 100 range
	let out_of_range = t("Percent out of range.");
	let entry_for_submit = Rc::clone(&entry);
	let submit = Rc::new(move || entry_for_submit.submit(dialog, live_region_label, &out_of_range));
	// Keyboard control of the slider: arrows/Page/Home/End move it, Enter submits.
	let submit_for_slider = Rc::clone(&submit);
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == WXK_RETURN || key == WXK_NUMPAD_ENTER {
			event.skip(false);
			submit_for_slider();
			return;
		}
		event.skip(true);
	});
	let percent_slider_for_keys = percent_slider;
	let input_for_char = entry.ctrl;
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
	content_sizer.add(&entry.ctrl, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	add_go_cancel_footer(dialog, content_sizer, entry.ctrl, move || submit());
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	// Focus the numeric entry so a screen-reader user can type a percentage directly.
	// Tab order is untouched (it follows widget creation order), so the slider remains the
	// first tab stop, matching the order the controls appear on screen.
	entry.focus();
	if dialog.show_modal() == ID_OK { entry.result() } else { None }
}
