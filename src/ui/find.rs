use std::{cell::Cell, rc::Rc, sync::Mutex};

use bitflags::bitflags;
use wxdragon::{prelude::*, translations::translate as t};

use super::{accessibility, document_manager::DocumentManager};
use crate::{config::ConfigManager, reader_core, text::display_len};

const DIALOG_PADDING: i32 = 10;
const MAX_FIND_HISTORY_SIZE: usize = 10;

#[derive(Clone, Debug, Default)]
pub struct SearchResult {
	pub found: bool,
	pub wrapped: bool,
	pub position: i64,
}

bitflags! {
	#[derive(Copy, Clone, Default)]
	pub struct FindOptions: u8 {
		const NONE = 0;
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const MATCH_WHOLE_WORD = 1 << 2;
		const USE_REGEX = 1 << 3;
	}
}

pub fn find_text_with_wrap(haystack: &str, needle: &str, start: i64, options: FindOptions) -> SearchResult {
	if needle.is_empty() {
		return SearchResult::default();
	}
	let mut search_options = reader_core::SearchOptions::empty();
	if options.contains(FindOptions::FORWARD) {
		search_options |= reader_core::SearchOptions::FORWARD;
	}
	if options.contains(FindOptions::MATCH_CASE) {
		search_options |= reader_core::SearchOptions::MATCH_CASE;
	}
	if options.contains(FindOptions::MATCH_WHOLE_WORD) {
		search_options |= reader_core::SearchOptions::WHOLE_WORD;
	}
	if options.contains(FindOptions::USE_REGEX) {
		search_options |= reader_core::SearchOptions::REGEX;
	}
	let result = reader_core::reader_search_with_wrap(haystack, needle, start, search_options);
	SearchResult { found: result.found, wrapped: result.wrapped, position: result.position }
}

#[derive(Clone)]
pub struct FindDialogState {
	pub dialog: Dialog,
	find_combo: ComboBox,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	in_progress: Rc<Cell<bool>>,
}

impl FindDialogState {
	pub fn new(
		frame: &Frame,
		config: &Rc<Mutex<ConfigManager>>,
		doc_manager: &Rc<Mutex<DocumentManager>>,
		find_dialog: &Rc<Mutex<Option<Self>>>,
		live_region_label: StaticText,
	) -> Self {
		let dialog = Dialog::builder(frame, &t("Find")).build();
		let FindDialogWidgets {
			find_combo,
			match_case,
			whole_word,
			use_regex,
			find_prev_btn,
			find_next_btn,
			cancel_btn,
		} = build_find_dialog_ui(dialog);
		bind_find_dialog_actions(FindDialogActionParams {
			frame: *frame,
			dialog,
			find_combo,
			find_prev_btn,
			find_next_btn,
			cancel_btn,
			config: Rc::clone(config),
			doc_manager: Rc::clone(doc_manager),
			find_dialog: Rc::clone(find_dialog),
			live_region_label,
		});
		let state =
			Self { dialog, find_combo, match_case, whole_word, use_regex, in_progress: Rc::new(Cell::new(false)) };
		state.reload_history(config);
		state.save_settings(config);
		state
	}

	pub fn reload_history(&self, config: &Rc<Mutex<ConfigManager>>) {
		self.find_combo.clear();
		let settings = {
			let cfg = config.lock().unwrap();
			for entry in cfg.get_find_history() {
				self.find_combo.append(&entry);
			}
			cfg.get_find_settings()
		};
		self.match_case.set_value(settings.match_case);
		self.whole_word.set_value(settings.whole_word);
		self.use_regex.set_value(settings.use_regex);
	}

	pub fn save_settings(&self, config: &Rc<Mutex<ConfigManager>>) {
		let settings = crate::config::FindSettings {
			match_case: self.match_case.is_checked(),
			whole_word: self.whole_word.is_checked(),
			use_regex: self.use_regex.is_checked(),
		};
		config.lock().unwrap().set_find_settings(settings);
	}

	pub fn add_to_history(&self, config: &Rc<Mutex<ConfigManager>>, text: &str) {
		config.lock().unwrap().add_find_history(text, MAX_FIND_HISTORY_SIZE);
		self.reload_history(config);
		self.find_combo.set_value(text);
	}

	pub fn find_text(&self) -> String {
		self.find_combo.get_value()
	}

	pub fn set_find_text(&self, text: &str) {
		self.find_combo.set_value(text);
		let len = self.find_combo.get_last_position();
		self.find_combo.set_text_selection(0, len);
	}

	pub fn focus_find_text(&self) {
		self.find_combo.set_focus();
		let len = self.find_combo.get_last_position();
		self.find_combo.set_text_selection(0, len);
	}

	pub fn try_begin_find(&self) -> Option<FindInProgressGuard> {
		if self.in_progress.replace(true) {
			return None;
		}
		Some(FindInProgressGuard { flag: Rc::clone(&self.in_progress) })
	}
}

struct FindDialogWidgets {
	find_combo: ComboBox,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	cancel_btn: Button,
}

struct FindDialogActionParams {
	frame: Frame,
	dialog: Dialog,
	find_combo: ComboBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	cancel_btn: Button,
	config: Rc<Mutex<ConfigManager>>,
	doc_manager: Rc<Mutex<DocumentManager>>,
	find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
}

fn build_find_dialog_ui(dialog: Dialog) -> FindDialogWidgets {
	let combo_width = 250;
	let option_padding = 2;
	let button_spacing = 5;
	let find_label = StaticText::builder(&dialog).with_label(&t("Find &what:")).build();
	let find_combo = ComboBox::builder(&dialog)
		.with_style(ComboBoxStyle::ProcessEnter)
		.with_size(Size::new(combo_width, -1))
		.build();
	let options_box = StaticBoxSizerBuilder::new_with_label(Orientation::Vertical, &dialog, &t("Options")).build();
	let match_case = CheckBox::builder(&dialog).with_label(&t("&Match case")).build();
	let whole_word = CheckBox::builder(&dialog).with_label(&t("Match &whole word")).build();
	let use_regex = CheckBox::builder(&dialog).with_label(&t("Use &regular expressions")).build();
	options_box.add(&match_case, 0, SizerFlag::All, option_padding);
	options_box.add(&whole_word, 0, SizerFlag::All, option_padding);
	options_box.add(&use_regex, 0, SizerFlag::All, option_padding);
	let find_prev_btn = Button::builder(&dialog).with_label(&t("Find &Previous")).build();
	let find_next_btn = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("Find &Next")).build();
	let cancel_btn = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let find_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	find_sizer.add(&find_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	find_sizer.add(&find_combo, 1, SizerFlag::Expand, 0);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add(&find_prev_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add(&find_next_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&cancel_btn, 0, SizerFlag::All, 0);
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
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	FindDialogWidgets { find_combo, match_case, whole_word, use_regex, find_prev_btn, find_next_btn, cancel_btn }
}

fn bind_find_dialog_actions(params: FindDialogActionParams) {
	let FindDialogActionParams {
		frame,
		dialog,
		find_combo,
		find_prev_btn,
		find_next_btn,
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
		);
	});
	let dialog_for_cancel = dialog;
	let find_dialog_for_cancel = Rc::clone(&find_dialog);
	let config_for_cancel = Rc::clone(&config);
	cancel_btn.on_click(move |_| {
		if let Some(state) = find_dialog_for_cancel.lock().unwrap().as_ref() {
			state.save_settings(&config_for_cancel);
			dialog_for_cancel.show(false);
		}
	});
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
		);
		event.skip(false);
	});
	let dialog_for_close = dialog;
	let find_dialog_for_close = Rc::clone(&find_dialog);
	let config_for_close = Rc::clone(&config);
	dialog.on_close(move |event| {
		if let Some(state) = find_dialog_for_close.lock().unwrap().as_ref() {
			state.save_settings(&config_for_close);
		}
		dialog_for_close.show(false);
		event.skip(false);
	});
}

pub struct FindInProgressGuard {
	flag: Rc<Cell<bool>>,
}

impl Drop for FindInProgressGuard {
	fn drop(&mut self) {
		self.flag.set(false);
	}
}

pub fn ensure_find_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
) {
	let mut dialog_guard = find_dialog.lock().unwrap();
	if dialog_guard.is_some() {
		return;
	}
	let state = FindDialogState::new(frame, config, doc_manager, find_dialog, live_region_label);
	*dialog_guard = Some(state);
}

pub fn show_find_dialog(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
) {
	ensure_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
	let state = {
		let dialog_state = find_dialog.lock().unwrap();
		dialog_state.as_ref().cloned()
	};
	let Some(state) = state else {
		return;
	};
	let text_ctrl = {
		let dm = doc_manager.lock().unwrap();
		dm.active_tab().map(|tab| tab.text_ctrl)
	};
	if let Some(text_ctrl) = text_ctrl {
		let (start, end) = text_ctrl.get_selection();
		if start != end {
			let selection = text_ctrl.get_string_selection();
			state.set_find_text(&selection);
		}
	}
	state.dialog.show(true);
	state.dialog.raise();
	state.focus_find_text();
}

pub fn handle_find_action(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
	forward: bool,
) {
	ensure_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
	let state = {
		let dialog_state = find_dialog.lock().unwrap();
		dialog_state.as_ref().cloned()
	};
	let Some(state) = state else {
		return;
	};
	if state.find_text().trim().is_empty() {
		let text_ctrl = {
			let dm = doc_manager.lock().unwrap();
			dm.active_tab().map(|tab| tab.text_ctrl)
		};
		if let Some(text_ctrl) = text_ctrl {
			let (start, end) = text_ctrl.get_selection();
			if start != end {
				let selection = text_ctrl.get_string_selection();
				state.set_find_text(&selection);
			}
		}
	}
	if state.find_text().trim().is_empty() {
		show_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
		return;
	}
	do_find(forward, &state, doc_manager, config, live_region_label);
}

fn do_find(
	forward: bool,
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let (text_ctrl, text) = {
		let dm = doc_manager.lock().unwrap();
		match dm.active_tab() {
			Some(tab) => (tab.text_ctrl, tab.session.content()),
			None => return,
		}
	};
	if !text_ctrl.is_valid() {
		return;
	}
	let query = state.find_text();
	if query.trim().is_empty() {
		return;
	}
	let Some(_find_guard) = state.try_begin_find() else {
		return;
	};
	state.save_settings(config);
	state.add_to_history(config, &query);
	let mut options = FindOptions::default();
	if forward {
		options |= FindOptions::FORWARD;
	}
	if state.match_case.is_checked() {
		options |= FindOptions::MATCH_CASE;
	}
	if state.whole_word.is_checked() {
		options |= FindOptions::MATCH_WHOLE_WORD;
	}
	if state.use_regex.is_checked() {
		options |= FindOptions::USE_REGEX;
	}
	let (sel_start, sel_end) = text_ctrl.get_selection();
	let start_pos = if forward { sel_end } else { sel_start };
	let result = find_text_with_wrap(&text, &query, start_pos, options);
	if !result.found {
		accessibility::announce(live_region_label, &t("Not found."));
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	if result.wrapped {
		accessibility::announce(live_region_label, &t("No more results. Wrapping search."));
	}
	if result.position < 0 {
		return;
	}
	let len = i64::try_from(display_len(&query)).unwrap_or(i64::MAX);
	let last_pos = text_ctrl.get_last_position();
	if last_pos <= 0 {
		return;
	}
	let start = result.position.clamp(0, last_pos);
	let end = (start + len).min(last_pos);
	text_ctrl.set_focus();
	text_ctrl.set_selection(start, end);
	text_ctrl.show_position(start);
	state.dialog.show(false);
}
