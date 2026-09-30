//! The global window hotkey picker, shown from the Options dialog only on Windows, the only
//! platform with a global hotkey listener (see start_hotkey_listener in main_window.rs).

use paperback_core::config::HotkeyConfig;
use patois::t;
use wxdragon::prelude::*;

pub(super) fn prompt_for_hotkey(parent: &dyn WxWidget, initial: &HotkeyConfig) -> Option<HotkeyConfig> {
	// TRANSLATORS: Title of the hotkey customization dialog
	let dialog = Dialog::builder(parent, &t("Window Hotkey"))
		.with_size(parent.from_dip_int(300), parent.from_dip_int(230))
		.build();
	let panel = Panel::builder(&dialog).build();
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	// TRANSLATORS: Checkbox label for Control modifier key
	let ctrl_cb = CheckBox::builder(&panel).with_label(&t("&Ctrl")).build();
	ctrl_cb.set_value(initial.ctrl);
	// TRANSLATORS: Checkbox label for Alt modifier key
	let alt_cb = CheckBox::builder(&panel).with_label(&t("&Alt")).build();
	alt_cb.set_value(initial.alt);
	// TRANSLATORS: Checkbox label for Shift modifier key
	let shift_cb = CheckBox::builder(&panel).with_label(&t("&Shift")).build();
	shift_cb.set_value(initial.shift);
	// TRANSLATORS: Checkbox label for Windows modifier key
	let win_cb = CheckBox::builder(&panel).with_label(&t("&Win")).build();
	win_cb.set_value(initial.win);
	main_sizer.add(&ctrl_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top, 10);
	main_sizer.add(&alt_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);
	main_sizer.add(&shift_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);
	main_sizer.add(&win_cb, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 10);
	// TRANSLATORS: Label for the hotkey key selection input field
	let key_label = StaticText::builder(&panel).with_label(&t("&Key:")).build();
	let key_text = TextCtrl::builder(&panel).build();
	key_text.set_value(&hotkey_key_display_name(initial.key));
	let key_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	key_sizer.add(&key_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 8);
	key_sizer.add(&key_text, 1, SizerFlag::Expand, 0);
	main_sizer.add_sizer(&key_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	// TRANSLATORS: Button label to clear the current hotkey selection
	let clear_button = Button::builder(&panel).with_label(&t("Clear")).build();
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&panel).with_id(ID_OK).with_label(&t("OK")).build();
	ok_button.set_default();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&panel).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	let key_text_clone = key_text;
	let ctrl_cb_clone = ctrl_cb;
	let alt_cb_clone = alt_cb;
	let shift_cb_clone = shift_cb;
	let win_cb_clone = win_cb;
	clear_button.on_click(move |_| {
		key_text_clone.set_value("");
		ctrl_cb_clone.set_value(false);
		alt_cb_clone.set_value(false);
		shift_cb_clone.set_value(false);
		win_cb_clone.set_value(false);
	});
	button_sizer.add(&clear_button, 0, SizerFlag::Right, 8);
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, 8);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, 8);
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 10);
	panel.set_sizer(main_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer(dialog_sizer, true);
	dialog.set_affirmative_id(ID_OK);
	dialog.set_escape_id(ID_CANCEL);
	dialog.centre();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	let key_value = key_text.get_value();
	let key_char = if key_value.trim().is_empty() { '\0' } else { parse_hotkey_key(&key_value).unwrap_or(initial.key) };
	Some(HotkeyConfig {
		ctrl: ctrl_cb.is_checked(),
		alt: alt_cb.is_checked(),
		shift: shift_cb.is_checked(),
		win: win_cb.is_checked(),
		key: key_char,
	})
}

fn hotkey_key_display_name(key: char) -> String {
	match key {
		'\0' => String::new(),
		// TRANSLATORS: Representation of the Spacebar key
		' ' => t("Space"),
		c if c.is_ascii_alphanumeric() => c.to_ascii_uppercase().to_string(),
		c => c.to_string(),
	}
}

fn parse_hotkey_key(input: &str) -> Option<char> {
	let trimmed = input.trim();
	if trimmed.eq_ignore_ascii_case("space") {
		return Some(' ');
	}
	let ch = if trimmed.is_empty() { return None } else { trimmed.chars().last()? };
	if ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() || ch == ' ' {
		Some(ch.to_ascii_uppercase())
	} else {
		None
	}
}
