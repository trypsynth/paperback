use std::{cell::RefCell, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, session::FindAllLine};
use patois::t;
use wxdragon::prelude::*;

use super::{
	FindDialogState, do_find_all, handle_find_action, handle_result_go,
	results_list::{ResultsList, build_results_list},
};
use crate::ui::{dialogs::DIALOG_PADDING, document_manager::DocumentManager};

pub(super) struct FindDialogWidgets {
	pub(super) find_label: StaticText,
	pub(super) find_combo: ComboBox,
	pub(super) options_static_box: Option<StaticBox>,
	pub(super) match_case: CheckBox,
	pub(super) whole_word: CheckBox,
	pub(super) use_regex: CheckBox,
	pub(super) find_prev_btn: Button,
	pub(super) find_next_btn: Button,
	pub(super) find_all_btn: Button,
	pub(super) cancel_btn: Button,
	pub(super) results_list: ResultsList,
	pub(super) go_btn: Button,
}

pub(super) struct FindDialogActionParams {
	pub(super) frame: Frame,
	pub(super) dialog: Dialog,
	pub(super) find_combo: ComboBox,
	pub(super) find_prev_btn: Button,
	pub(super) find_next_btn: Button,
	pub(super) find_all_btn: Button,
	pub(super) go_btn: Button,
	pub(super) results_list: ResultsList,
	pub(super) cancel_btn: Button,
	pub(super) config: Rc<Mutex<ConfigManager>>,
	pub(super) doc_manager: Rc<Mutex<DocumentManager>>,
	pub(super) find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	pub(super) live_region_label: StaticText,
}

pub(super) fn build_find_dialog_ui(
	dialog: Dialog,
	result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	result_labels: Rc<RefCell<Vec<String>>>,
) -> FindDialogWidgets {
	let combo_width = 250;
	let option_padding = 2;
	let button_spacing = 5;
	// TRANSLATORS: Label for the text field where the user types what to search for
	let find_label = StaticText::builder(&dialog).with_label(&t("Find &what:")).build();
	let find_combo = ComboBox::builder(&dialog)
		.with_style(ComboBoxStyle::ProcessEnter)
		.with_size(dialog.from_dip(Size::new(combo_width, -1)))
		.build();
	// TRANSLATORS: Group box heading for the search options in the Find dialog
	let options_box = StaticBoxSizerBuilder::new_with_label(Orientation::Vertical, &dialog, &t("Options")).build();
	let options_static_box = options_box.get_static_box();
	// TRANSLATORS: Checkbox to make the search case-sensitive
	let match_case = CheckBox::builder(&dialog).with_label(&t("&Match case")).build();
	// TRANSLATORS: Checkbox to only match whole words, not substrings
	let whole_word = CheckBox::builder(&dialog).with_label(&t("Match &whole word")).build();
	// TRANSLATORS: Checkbox to treat the search text as a regular expression
	let use_regex = CheckBox::builder(&dialog).with_label(&t("Use &regular expressions")).build();
	options_box.add(&match_case, 0, SizerFlag::All, option_padding);
	options_box.add(&whole_word, 0, SizerFlag::All, option_padding);
	options_box.add(&use_regex, 0, SizerFlag::All, option_padding);
	// TRANSLATORS: Button to search backward for the previous match
	let find_prev_btn = Button::builder(&dialog).with_label(&t("Find &Previous")).build();
	// TRANSLATORS: Button to search forward for the next match
	let find_next_btn = Button::builder(&dialog).with_id(ID_OK).with_label(&t("Find &Next")).build();
	// TRANSLATORS: Button that finds and lists every occurrence of the search text
	let find_all_btn = Button::builder(&dialog).with_label(&t("Find &All")).build();
	// TRANSLATORS: Cancel button that closes the Find dialog
	let cancel_btn = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	dialog.set_affirmative_id(ID_OK);
	let find_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	find_sizer.add(&find_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	find_sizer.add(&find_combo, 1, SizerFlag::Expand, 0);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add(&find_prev_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add(&find_next_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add(&find_all_btn, 0, SizerFlag::Right, button_spacing);
	// The Find All results view, hidden until a search with matches switches to it.
	let results_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let results_list = build_results_list(dialog, result_rows, result_labels);
	results_sizer.add(&results_list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let results_button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	// TRANSLATORS: Button that jumps to the selected find result
	let go_btn = Button::builder(&dialog).with_label(&t("&Go")).build();
	results_button_sizer.add(&go_btn, 0, SizerFlag::Right, button_spacing);
	results_sizer.add_sizer(
		&results_button_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	// A single Cancel is shared by both views and always visible, so Escape always maps to one
	// available button.
	let cancel_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	cancel_sizer.add_stretch_spacer(1);
	cancel_sizer.add(&cancel_btn, 0, SizerFlag::All, 0);
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add_sizer(&find_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	main_sizer.add_sizer(
		&options_box,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	main_sizer.add_sizer(
		&button_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	main_sizer.add_sizer(
		&results_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	main_sizer.add_sizer(
		&cancel_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	// Open on the query view: hide the results windows before sizing so the dialog starts small.
	results_list.show(false);
	go_btn.show(false);
	find_next_btn.set_default();
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	FindDialogWidgets {
		find_label,
		find_combo,
		options_static_box,
		match_case,
		whole_word,
		use_regex,
		find_prev_btn,
		find_next_btn,
		find_all_btn,
		cancel_btn,
		results_list,
		go_btn,
	}
}

/// Binds a Cancel button to hide the dialog and save its settings.
fn bind_cancel_button(
	button: Button,
	dialog: Dialog,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let dialog_for_cancel = dialog;
	let find_dialog_for_cancel = Rc::clone(find_dialog);
	let config_for_cancel = Rc::clone(config);
	button.on_click(move |_| {
		{
			let guard = find_dialog_for_cancel.lock().unwrap();
			if let Some(state) = guard.as_ref() {
				state.save_settings(&config_for_cancel);
			}
		}
		dialog_for_cancel.show(false);
		if let Some(state) = find_dialog_for_cancel.lock().unwrap().as_ref() {
			state.clear_results();
		}
	});
}

pub(super) fn bind_find_dialog_actions(params: FindDialogActionParams) {
	let FindDialogActionParams {
		frame,
		dialog,
		find_combo,
		find_prev_btn,
		find_next_btn,
		find_all_btn,
		go_btn,
		results_list,
		cancel_btn,
		config,
		doc_manager,
		find_dialog,
		live_region_label,
	} = params;
	let frame_for_next = frame;
	let find_dialog_for_next = Rc::clone(&find_dialog);
	let doc_manager_for_next = Rc::clone(&doc_manager);
	let config_for_next = Rc::clone(&config);
	find_next_btn.on_click(move |_| {
		handle_find_action(
			&frame_for_next,
			&doc_manager_for_next,
			&config_for_next,
			&find_dialog_for_next,
			live_region_label,
			true,
			false,
		);
	});
	let frame_for_prev = frame;
	let find_dialog_for_prev = Rc::clone(&find_dialog);
	let doc_manager_for_prev = Rc::clone(&doc_manager);
	let config_for_prev = Rc::clone(&config);
	find_prev_btn.on_click(move |_| {
		handle_find_action(
			&frame_for_prev,
			&doc_manager_for_prev,
			&config_for_prev,
			&find_dialog_for_prev,
			live_region_label,
			false,
			false,
		);
	});
	let frame_for_all = frame;
	let find_dialog_for_all = Rc::clone(&find_dialog);
	let doc_manager_for_all = Rc::clone(&doc_manager);
	let config_for_all = Rc::clone(&config);
	find_all_btn.on_click(move |_| {
		if let Some(state) = find_dialog_for_all.lock().unwrap().as_ref() {
			do_find_all(&frame_for_all, state, &doc_manager_for_all, &config_for_all, live_region_label);
		}
	});
	bind_cancel_button(cancel_btn, dialog, &find_dialog, &config);
	let frame_for_enter = frame;
	let find_dialog_for_enter = Rc::clone(&find_dialog);
	let doc_manager_for_enter = Rc::clone(&doc_manager);
	let config_for_enter = Rc::clone(&config);
	find_combo.bind_internal(EventType::TEXT_ENTER, move |event| {
		handle_find_action(
			&frame_for_enter,
			&doc_manager_for_enter,
			&config_for_enter,
			&find_dialog_for_enter,
			live_region_label,
			true,
			false,
		);
		event.skip(false);
	});
	let dialog_for_close = dialog;
	let find_dialog_for_close = Rc::clone(&find_dialog);
	let config_for_close = Rc::clone(&config);
	dialog.on_close(move |event| {
		{
			let guard = find_dialog_for_close.lock().unwrap();
			if let Some(state) = guard.as_ref() {
				state.save_settings(&config_for_close);
			}
		}
		dialog_for_close.show(false);
		if let Some(state) = find_dialog_for_close.lock().unwrap().as_ref() {
			state.clear_results();
		}
		event.skip(false);
	});
	let find_dialog_for_go = Rc::clone(&find_dialog);
	let doc_manager_for_go = Rc::clone(&doc_manager);
	go_btn.on_click(move |_| {
		if let Some(state) = find_dialog_for_go.lock().unwrap().as_ref() {
			handle_result_go(state, &doc_manager_for_go, live_region_label);
		}
	});
	let find_dialog_for_list = Rc::clone(&find_dialog);
	let doc_manager_for_list = Rc::clone(&doc_manager);
	results_list.on_item_activated(move |_| {
		if let Some(state) = find_dialog_for_list.lock().unwrap().as_ref() {
			handle_result_go(state, &doc_manager_for_list, live_region_label);
		}
	});
}
