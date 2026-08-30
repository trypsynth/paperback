use std::{cell::Cell, rc::Rc, sync::Mutex};

use bitflags::bitflags;
use paperback_core::{config::ConfigManager, reader_core, util::text::display_len};
use patois::t;
use wx_utils::dpi;
use wxdragon::prelude::*;

use super::{dialogs::DIALOG_PADDING, document_manager::DocumentManager, navigation};

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
	/// The in-window find strip. A child of the active tab's page rather than a separate dialog
	/// window, so focus moving between the find controls and the book text stays within the same
	/// top-level window and NVDA doesn't announce the whole "Paperback, tab control" chain on the
	/// way back. Shown/hidden (with a border and a heading) so it still reads as a dialog.
	pub panel: Panel,
	/// The tab page the strip was created under, so it can be rebuilt if the active tab changes.
	host_page: Panel,
	/// Used to return focus to the book text when the strip hides: a hidden panel's combo keeps
	/// keyboard focus, so without this ESC keeps landing on the (invisible) find strip.
	doc_manager: Rc<Mutex<DocumentManager>>,
	find_combo: ComboBox,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	in_progress: Rc<Cell<bool>>,
}

impl FindDialogState {
	pub fn new(
		frame: &Frame,
		host_page: Panel,
		config: &Rc<Mutex<ConfigManager>>,
		doc_manager: &Rc<Mutex<DocumentManager>>,
		find_dialog: &Rc<Mutex<Option<Self>>>,
		live_region_label: StaticText,
		initial_text: &str,
	) -> Self {
		// TRANSLATORS: Heading shown at the top of the Find panel
		let panel = Panel::builder(&host_page).with_style(PanelStyle::TabTraversal | PanelStyle::BorderRaised).build();
		let FindDialogWidgets {
			find_combo,
			match_case,
			whole_word,
			use_regex,
			find_prev_btn,
			find_next_btn,
			cancel_btn,
		} = build_find_dialog_ui(panel);
		bind_find_dialog_actions(FindDialogActionParams {
			frame: *frame,
			panel,
			find_combo,
			find_prev_btn,
			find_next_btn,
			cancel_btn,
			config: Rc::clone(config),
			doc_manager: Rc::clone(doc_manager),
			find_dialog: Rc::clone(find_dialog),
			live_region_label,
		});
		let state = Self {
			panel,
			host_page,
			doc_manager: Rc::clone(doc_manager),
			find_combo,
			match_case,
			whole_word,
			use_regex,
			in_progress: Rc::new(Cell::new(false)),
		};
		state.reload_history(config);
		state.save_settings(config);
		if !initial_text.is_empty() {
			state.set_find_text(initial_text);
		}
		state
	}

	pub fn show(&self) {
		self.panel.show(true);
		self.panel.raise();
	}

	pub fn hide(&self) {
		let panel_rect = Rect::from_point_and_size(self.panel.get_position(), self.panel.get_size());
		self.panel.show(false);
		// ShowWindow(SW_HIDE) leaves the combo holding keyboard focus, so keys (Escape included)
		// keep landing on the invisible strip. Move focus back to the book text, which also forces
		// the page to repaint the area the strip covered (its pixels would otherwise linger).
		if let Ok(dm) = self.doc_manager.try_lock() {
			dm.restore_focus();
		}
		if let Some(parent) = self.panel.get_parent() {
			parent.refresh(true, Some(&panel_rect));
		}
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
		let settings = paperback_core::config::FindSettings {
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
	panel: Panel,
	find_combo: ComboBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	cancel_btn: Button,
	config: Rc<Mutex<ConfigManager>>,
	doc_manager: Rc<Mutex<DocumentManager>>,
	find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
}

fn build_find_dialog_ui(panel: Panel) -> FindDialogWidgets {
	let combo_width = 250;
	let option_padding = 2;
	let button_spacing = 5;
	// TRANSLATORS: Heading shown at the top of the Find panel
	let title = StaticText::builder(&panel).with_label(&t("Find")).build();
	// TRANSLATORS: Label for the text field where the user types what to search for
	let find_label = StaticText::builder(&panel).with_label(&t("Find &what:")).build();
	let find_combo = ComboBox::builder(&panel)
		.with_style(ComboBoxStyle::ProcessEnter)
		.with_size(dpi::scale_size(&panel, Size::new(combo_width, -1)))
		.build();
	let options_box = StaticBoxSizerBuilder::new_with_label(Orientation::Vertical, &panel, &t("Options")).build();
	// TRANSLATORS: Checkbox to make the search case-sensitive
	let match_case = CheckBox::builder(&panel).with_label(&t("&Match case")).build();
	// TRANSLATORS: Checkbox to only match whole words, not substrings
	let whole_word = CheckBox::builder(&panel).with_label(&t("Match &whole word")).build();
	// TRANSLATORS: Checkbox to treat the search text as a regular expression
	let use_regex = CheckBox::builder(&panel).with_label(&t("Use &regular expressions")).build();
	options_box.add(&match_case, 0, SizerFlag::All, option_padding);
	options_box.add(&whole_word, 0, SizerFlag::All, option_padding);
	options_box.add(&use_regex, 0, SizerFlag::All, option_padding);
	// TRANSLATORS: Button to search backward for the previous match
	let find_prev_btn = Button::builder(&panel).with_label(&t("Find &Previous")).build();
	// TRANSLATORS: Button to search forward for the next match
	let find_next_btn = Button::builder(&panel).with_label(&t("Find &Next")).build();
	let cancel_btn = Button::builder(&panel).with_label(&t("Cancel")).build();
	let find_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	find_sizer.add(&find_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	find_sizer.add(&find_combo, 1, SizerFlag::Expand, 0);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add(&find_prev_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add(&find_next_btn, 0, SizerFlag::Right, button_spacing);
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&cancel_btn, 0, SizerFlag::All, 0);
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add(&title, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, DIALOG_PADDING);
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
	panel.set_sizer_and_fit(main_sizer, true);
	FindDialogWidgets { find_combo, match_case, whole_word, use_regex, find_prev_btn, find_next_btn, cancel_btn }
}

fn bind_find_dialog_actions(params: FindDialogActionParams) {
	let FindDialogActionParams {
		frame,
		panel,
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
	let find_dialog_for_cancel = Rc::clone(&find_dialog);
	let config_for_cancel = Rc::clone(&config);
	cancel_btn.on_click(move |_| {
		if let Some(state) = find_dialog_for_cancel.lock().unwrap().as_ref() {
			state.save_settings(&config_for_cancel);
			state.hide();
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
	// Escape hides the strip, standing in for the dialog window's escape-to-cancel behaviour.
	// Bind both the combo and the panel: the combo may consume Escape natively before it would
	// bubble up to the panel, and the panel catches it when focus is on one of the other
	// controls.
	let find_dialog_for_combo_escape = Rc::clone(&find_dialog);
	let config_for_combo_escape = Rc::clone(&config);
	find_combo.on_key_down(move |event| {
		if let WindowEventData::Keyboard(key) = &event
			&& key.get_key_code() == Some(WXK_ESCAPE)
		{
			if let Some(state) = find_dialog_for_combo_escape.lock().unwrap().as_ref() {
				state.save_settings(&config_for_combo_escape);
				state.hide();
			}
			event.skip(false);
		} else {
			event.skip(true);
		}
	});
	let find_dialog_for_escape = Rc::clone(&find_dialog);
	let config_for_escape = Rc::clone(&config);
	panel.on_key_down(move |event| {
		if let WindowEventData::Keyboard(key) = &event
			&& key.get_key_code() == Some(WXK_ESCAPE)
		{
			if let Some(state) = find_dialog_for_escape.lock().unwrap().as_ref() {
				state.save_settings(&config_for_escape);
				state.hide();
			}
			event.skip(false);
		} else {
			event.skip(true);
		}
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
	let host_page = {
		let dm = doc_manager.lock().unwrap();
		dm.active_tab().map(|tab| tab.panel)
	};
	let needs_build = match (dialog_guard.as_ref(), host_page) {
		(None, Some(_)) => true,
		(Some(state), Some(page)) => !state.panel.is_valid() || state.host_page.handle_ptr() != page.handle_ptr(),
		_ => false,
	};
	if needs_build {
		// Keep the previous query across the rebuild.
		let initial_text = dialog_guard.as_ref().map(FindDialogState::find_text).unwrap_or_default();
		let Some(page) = host_page else {
			return;
		};
		if let Some(state) = dialog_guard.take() {
			state.panel.destroy();
		}
		let state =
			FindDialogState::new(frame, page, config, doc_manager, find_dialog, live_region_label, &initial_text);
		*dialog_guard = Some(state);
	}
}

/// Centres the find strip over the tab page so it reads as a dialog floating over the book
/// rather than a fixed toolbar.
fn position_find_panel(panel: Panel, host_page: Panel) {
	let page_size = host_page.get_client_size();
	let panel_size = panel.get_best_size();
	let x = ((page_size.width - panel_size.width) / 2).max(0);
	let y = ((page_size.height - panel_size.height) / 3).max(0);
	panel.move_window(x, y);
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
	let (text_ctrl, page) = {
		let dm = doc_manager.lock().unwrap();
		let tab = dm.active_tab();
		let result = (tab.map(|t| t.text_ctrl), tab.map(|t| t.panel));
		drop(dm);
		result
	};
	let Some(page) = page else {
		return;
	};
	if let Some(text_ctrl) = text_ctrl {
		let (start, end) = text_ctrl.get_selection();
		if start != end {
			let selection = text_ctrl.get_string_selection();
			state.set_find_text(&selection);
		}
	}
	position_find_panel(state.panel, page);
	state.show();
	state.focus_find_text();
}

/// Whether the find strip is currently visible, so app-wide key handling (the notebook's
/// Delete-to-close, for instance) can leave the keys alone while the user types in it.
pub fn is_find_shown(find_dialog: &Rc<Mutex<Option<FindDialogState>>>) -> bool {
	find_dialog.lock().unwrap().as_ref().is_some_and(|state| state.panel.is_shown())
}

/// Hides the find strip if it's showing. Used by app-wide key handling (the notebook's Escape
/// fallback) that can't reach the strip's own controls.
pub fn hide_find_dialog(find_dialog: &Rc<Mutex<Option<FindDialogState>>>) {
	if let Some(state) = find_dialog.lock().unwrap().as_ref() {
		state.hide();
	}
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
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	if !tab.text_ctrl.is_valid() {
		return;
	}
	// Search the whole document (not just whatever window is currently loaded into
	// text_ctrl) in the same document-absolute coordinate space `tab.window` uses, so a
	// found match can be reached even when it falls outside the loaded window.
	let text = tab.session.content();
	let (sel_start, sel_end) = navigation::doc_selected_range(tab);
	let start_pos = if forward { sel_end } else { sel_start };
	let result = find_text_with_wrap(&text, &query, start_pos, options);
	tracing::debug!(query = %query, forward, found = result.found, wrapped = result.wrapped, "find search");
	if !result.found {
		drop(dm);
		// TRANSLATORS: Announced when a search finds no matches in the document
		live_region::announce(live_region_label, &t("Not found."));
		state.show();
		state.focus_find_text();
		return;
	}
	if result.wrapped {
		// TRANSLATORS: Announced when a search reaches the end of the document and wraps back to the start
		live_region::announce(live_region_label, &t("No more results. Wrapping search."));
	}
	if result.position < 0 {
		return;
	}
	let doc_len = tab.session.document_len();
	if doc_len <= 0 {
		return;
	}
	let len = i64::try_from(display_len(&query)).unwrap_or(i64::MAX);
	let start = result.position.clamp(0, doc_len);
	let end = (start + len).min(doc_len);
	let found_line = tab.session.get_line_text(start);
	let announce_line = !found_line.trim().is_empty();
	if announce_line {
		// Speak the found line before focus returns to the book text, so the match text comes
		// first. The strip shares the book's window, so the focus move back is reported briefly
		// (just the text control) rather than as the whole "Paperback, tab control" chain.
		live_region::announce(live_region_label, &found_line);
	}
	navigation::select_doc_range(tab, start, end);
	drop(dm);
	state.hide();
}
