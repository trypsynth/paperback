use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};

use paperback_core::config::{ActionId, KeyChord, ShortcutCategory, ShortcutsConfig};
use patois::t;
use wxdragon::prelude::*;

use super::{add_ok_cancel_footer, build_ok_cancel_buttons};
use crate::ui::dpi;

type RefreshCallbacks = Rc<RefCell<Vec<Box<dyn Fn()>>>>;

pub fn prompt_for_shortcuts(parent: &dyn WxWidget, initial: &ShortcutsConfig) -> Option<ShortcutsConfig> {
	let config_state = Rc::new(RefCell::new(initial.clone()));
	let refresh_all_callbacks: RefreshCallbacks = Rc::new(RefCell::new(Vec::new()));

	let dialog = Dialog::builder(parent, &t("Customize Keyboard Shortcuts"))
		.with_size(dpi::scale(parent, 600), dpi::scale(parent, 560))
		.build();
	let notebook = Notebook::builder(&dialog).build();

	for &category in ShortcutCategory::all() {
		let tab_panel =
			build_category_tab(&notebook, config_state.clone(), category, &dialog, refresh_all_callbacks.clone());
		notebook.add_page(&tab_panel, &t(category.display_name()), category == ShortcutCategory::File, None);
	}

	// Uses the shared `wxStdDialogButtonSizer`-backed helper so OK/Cancel follow platform HIG
	// order (Cancel/OK on macOS, OK/Cancel on Windows) instead of a hardcoded LTR order.
	let (ok_button, cancel_button) = build_ok_cancel_buttons(dialog, &t("OK"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 8);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer(content_sizer, true);
	dialog.centre();

	if dialog.show_modal() == ID_OK {
		let result = config_state.borrow().clone();
		Some(result)
	} else {
		None
	}
}

fn build_category_tab(
	notebook: &Notebook,
	config_state: Rc<RefCell<ShortcutsConfig>>,
	category: ShortcutCategory,
	parent_dialog: &Dialog,
	refresh_all_callbacks: RefreshCallbacks,
) -> Panel {
	let panel = Panel::builder(notebook).with_style(PanelStyle::TabTraversal).build();
	let sizer = BoxSizer::builder(Orientation::Vertical).build();

	let list_label = StaticText::builder(&panel).with_label(&t("&Shortcuts:")).build();
	sizer.add(&list_label, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top, 8);

	let list_box = ListBox::builder(&panel).build();
	let actions = category.actions();
	for &action in &actions {
		let item_text = format_list_item(&config_state.borrow(), action);
		list_box.append(&item_text);
	}
	if !actions.is_empty() {
		list_box.set_selection(0, true);
	}
	sizer.add(&list_box, 1, SizerFlag::Expand | SizerFlag::All, 8);

	let buttons_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	let set_button = Button::builder(&panel).with_label(&t("&Set Shortcut...")).build();
	let clear_button = Button::builder(&panel).with_label(&t("&Clear Shortcut")).build();
	let reset_button = Button::builder(&panel).with_label(&t("&Reset to Default")).build();
	let reset_all_button = Button::builder(&panel).with_label(&t("Reset &All to Defaults")).build();

	buttons_sizer.add(&set_button, 0, SizerFlag::Right, 8);
	buttons_sizer.add(&clear_button, 0, SizerFlag::Right, 8);
	buttons_sizer.add(&reset_button, 0, SizerFlag::Right, 8);
	buttons_sizer.add(&reset_all_button, 0, SizerFlag::Right, 8);
	sizer.add_sizer(&buttons_sizer, 0, SizerFlag::Expand | SizerFlag::All, 8);

	panel.set_sizer(sizer, true);

	let refresh_this_tab = {
		let config_state = config_state.clone();
		let actions = actions.clone();
		move || {
			for (idx, &action) in actions.iter().enumerate() {
				let item_text = format_list_item(&config_state.borrow(), action);
				list_box.set_string(u32::try_from(idx).unwrap_or(u32::MAX), &item_text);
			}
		}
	};
	refresh_all_callbacks.borrow_mut().push(Box::new(refresh_this_tab));

	let refresh_all = {
		let refresh_all_callbacks = refresh_all_callbacks;
		move || {
			for cb in refresh_all_callbacks.borrow().iter() {
				cb();
			}
		}
	};

	let trigger_set_shortcut = {
		let config_state = config_state.clone();
		let actions = actions.clone();
		let parent = *parent_dialog;
		let refresh_all = refresh_all.clone();
		move || {
			let Some(sel) = list_box.get_selection() else { return };
			let Ok(idx) = usize::try_from(sel) else { return };
			let Some(&action) = actions.get(idx) else { return };

			let current_chord = config_state.borrow().get_chord(action);
			if let Some(result) = prompt_for_key_chord(&parent, action, current_chord.as_ref()) {
				if let Some(new_chord) = &result {
					let conflict = find_conflict(&config_state.borrow(), action, new_chord);
					if let Some(other_action) = conflict {
						let msg = format!(
							"'{}' is already assigned to '{}'. Reassign it to '{}'?",
							new_chord.to_shortcut_string(),
							other_action.display_name(),
							action.display_name()
						);
						let warn = MessageDialog::builder(&parent, &msg, &t("Shortcut Conflict"))
							.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconWarning)
							.build();
						if warn.show_modal() != ID_YES {
							return;
						}
						config_state.borrow_mut().set_chord(other_action, None);
					}
				}
				config_state.borrow_mut().set_chord(action, result);
				refresh_all();
			}
		}
	};

	let set_on_click = trigger_set_shortcut.clone();
	set_button.on_click(move |_| {
		set_on_click();
	});

	let set_on_dclick = trigger_set_shortcut;
	list_box.on_item_double_clicked(move |_| {
		set_on_dclick();
	});

	let config_on_clear = config_state.clone();
	let list_on_clear = list_box;
	let actions_on_clear = actions.clone();
	let refresh_on_clear = refresh_all.clone();
	clear_button.on_click(move |_| {
		let Some(sel) = list_on_clear.get_selection() else { return };
		let Ok(idx) = usize::try_from(sel) else { return };
		let Some(&action) = actions_on_clear.get(idx) else { return };
		config_on_clear.borrow_mut().set_chord(action, None);
		refresh_on_clear();
	});

	let config_on_reset = config_state.clone();
	let list_on_reset = list_box;
	let actions_on_reset = actions;
	let refresh_on_reset = refresh_all.clone();
	reset_button.on_click(move |_| {
		let Some(sel) = list_on_reset.get_selection() else { return };
		let Ok(idx) = usize::try_from(sel) else { return };
		let Some(&action) = actions_on_reset.get(idx) else { return };
		config_on_reset.borrow_mut().reset_action(action);
		refresh_on_reset();
	});

	let config_on_reset_all = config_state;
	let refresh_on_reset_all = refresh_all;
	let parent_reset_all = *parent_dialog;
	reset_all_button.on_click(move |_| {
		let warn = MessageDialog::builder(
			&parent_reset_all,
			&t("Reset all shortcuts to their default values?"),
			&t("Reset Shortcuts"),
		)
		.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconQuestion)
		.build();
		if warn.show_modal() == ID_YES {
			config_on_reset_all.borrow_mut().reset_all();
			refresh_on_reset_all();
		}
	});

	panel
}

fn format_list_item(config: &ShortcutsConfig, action: ActionId) -> String {
	let chord_str = config.get_display_str(action);
	format!("{}: {}", action.display_name(), chord_str)
}

fn find_conflict(config: &ShortcutsConfig, target_action: ActionId, target_chord: &KeyChord) -> Option<ActionId> {
	for &action in ActionId::all() {
		if action == target_action {
			continue;
		}
		if let Some(chord) = config.get_chord(action) {
			let ctrl_matches = (chord.ctrl || chord.raw_ctrl) == (target_chord.ctrl || target_chord.raw_ctrl);
			if ctrl_matches
				&& chord.alt == target_chord.alt
				&& chord.shift == target_chord.shift
				&& chord.key == target_chord.key
			{
				return Some(action);
			}
		}
	}
	None
}

const ID_CLEAR_SHORTCUT: i32 = 10099;

fn prompt_for_key_chord(
	parent: &dyn WxWidget,
	action: ActionId,
	initial: Option<&KeyChord>,
) -> Option<Option<KeyChord>> {
	let title = format!("Set Shortcut for {}", action.display_name());
	let dialog = Dialog::builder(parent, &title).with_size(dpi::scale(parent, 400), dpi::scale(parent, 260)).build();
	let panel = Panel::builder(&dialog).build();
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

	let live_region_label = StaticText::builder(&panel).with_label("").with_size(Size::new(0, 0)).build();
	live_region_label.show(false);
	let _ = live_region::set_live_region(&live_region_label);

	let info_text = format!("Configure shortcut for {}:", action.display_name());
	let info_label = StaticText::builder(&panel).with_label(&info_text).build();
	main_sizer.add(&info_label, 0, SizerFlag::Expand | SizerFlag::All, 8);

	let ctrl_cb = CheckBox::builder(&panel).with_label(&t("&Ctrl")).build();
	let alt_cb = CheckBox::builder(&panel).with_label(&t("&Alt")).build();
	let shift_cb = CheckBox::builder(&panel).with_label(&t("&Shift")).build();

	let raw_ctrl = Rc::new(Cell::new(initial.is_some_and(|chord| chord.raw_ctrl)));

	if let Some(chord) = initial {
		ctrl_cb.set_value(chord.ctrl || chord.raw_ctrl);
		alt_cb.set_value(chord.alt);
		shift_cb.set_value(chord.shift);
	}

	let mod_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	mod_sizer.add(&ctrl_cb, 0, SizerFlag::Right, 12);
	mod_sizer.add(&alt_cb, 0, SizerFlag::Right, 12);
	mod_sizer.add(&shift_cb, 0, SizerFlag::Right, 12);
	main_sizer.add_sizer(&mod_sizer, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom, 8);

	let key_label = StaticText::builder(&panel).with_label(&t("&Key:")).build();
	let key_text_ctrl = TextCtrl::builder(&panel)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::DontWrap)
		.build();
	if let Some(chord) = initial {
		key_text_ctrl.set_value(&chord.key);
	}

	let key_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	key_sizer.add(&key_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 8);
	key_sizer.add(&key_text_ctrl, 1, SizerFlag::Expand, 0);
	main_sizer.add_sizer(&key_sizer, 0, SizerFlag::Expand | SizerFlag::All, 8);

	let hint_label = StaticText::builder(&panel)
		.with_label(&t("Tip: click in the key field and press the key combination you want."))
		.build();
	main_sizer.add(&hint_label, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top, 8);

	let preview_label = StaticText::builder(&panel).with_label("Detected: (none)").build();
	main_sizer.add(&preview_label, 0, SizerFlag::Expand | SizerFlag::All, 8);

	let current_shortcut_text = move || {
		let key = key_text_ctrl.get_value();
		let trimmed = key.trim().to_string();
		if trimmed.is_empty() {
			None
		} else {
			let chord = KeyChord::new(ctrl_cb.get_value(), alt_cb.get_value(), shift_cb.get_value(), &trimmed);
			Some(chord.to_shortcut_string())
		}
	};

	let refresh_preview_label = move || {
		let text = current_shortcut_text().map_or_else(|| "Detected: (none)".to_string(), |s| format!("Detected: {s}"));
		preview_label.set_label(&text);
	};
	refresh_preview_label();

	let update_preview = move || {
		refresh_preview_label();
		let announce_text =
			current_shortcut_text().map_or_else(|| "No key detected".to_string(), |s| format!("Detected: {s}"));
		live_region::announce(live_region_label, &announce_text);
	};

	let update_preview_key = update_preview;
	let raw_ctrl_key = raw_ctrl.clone();
	key_text_ctrl.on_key_down(move |event| {
		if let WindowEventData::Keyboard(ref key_event) = event
			&& let Some(k) = key_event.get_key_code()
			&& k != 9 && k != 27
			&& !matches!(k, 314 | 315 | 316 | 317 | 378 | 380 | 382 | 383)
			&& let Some(parsed) =
				KeyChord::from_key_code(k, key_event.control_down(), key_event.alt_down(), key_event.shift_down())
		{
			raw_ctrl_key.set(false);
			ctrl_cb.set_value(parsed.ctrl);
			alt_cb.set_value(parsed.alt);
			shift_cb.set_value(parsed.shift);
			key_text_ctrl.set_value(&parsed.key);
			update_preview_key();
			event.skip(false);
			return;
		}
		event.skip(true);
	});

	let update_preview_ctrl = update_preview;
	let raw_ctrl_toggle = raw_ctrl.clone();
	ctrl_cb.on_toggled(move |_| {
		raw_ctrl_toggle.set(false);
		update_preview_ctrl();
	});
	let update_preview_alt = update_preview;
	alt_cb.on_toggled(move |_| update_preview_alt());
	let update_preview_shift = update_preview;
	shift_cb.on_toggled(move |_| update_preview_shift());

	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	let ok_button = Button::builder(&panel).with_id(ID_OK).with_label(&t("OK")).build();
	ok_button.set_default();
	let clear_button = Button::builder(&panel).with_id(ID_CLEAR_SHORTCUT).with_label(&t("&Clear")).build();
	let cancel_button = Button::builder(&panel).with_id(ID_CANCEL).with_label(&t("Cancel")).build();

	button_sizer.add(&clear_button, 0, SizerFlag::Right, 8);
	button_sizer.add_stretch_spacer(1);
	// macOS HIG puts the default/affirmative action rightmost (Cancel, then OK); Windows
	// puts it leftmost (OK, then Cancel).
	#[cfg(target_os = "macos")]
	{
		button_sizer.add(&cancel_button, 0, SizerFlag::Right, 8);
		button_sizer.add(&ok_button, 0, SizerFlag::Right, 8);
	}
	#[cfg(not(target_os = "macos"))]
	{
		button_sizer.add(&ok_button, 0, SizerFlag::Right, 8);
		button_sizer.add(&cancel_button, 0, SizerFlag::Right, 8);
	}
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 8);

	panel.set_sizer(main_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer(dialog_sizer, true);
	dialog.set_affirmative_id(ID_OK);
	dialog.set_escape_id(ID_CANCEL);

	let dialog_clear = dialog;
	clear_button.on_click(move |_| {
		dialog_clear.end_modal(ID_CLEAR_SHORTCUT);
	});

	dialog.centre();
	key_text_ctrl.set_focus();

	let res = dialog.show_modal();
	if res == ID_CLEAR_SHORTCUT {
		Some(None)
	} else if res == ID_OK {
		let key_text = key_text_ctrl.get_value();
		let trimmed = key_text.trim();
		if trimmed.is_empty() {
			Some(None)
		} else {
			let chord = if ctrl_cb.get_value() && raw_ctrl.get() {
				KeyChord::new_raw_ctrl(true, alt_cb.get_value(), shift_cb.get_value(), trimmed)
			} else {
				KeyChord::new(ctrl_cb.get_value(), alt_cb.get_value(), shift_cb.get_value(), trimmed)
			};
			Some(Some(chord))
		}
	} else {
		None
	}
}
