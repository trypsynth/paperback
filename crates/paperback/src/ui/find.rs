use std::{
	cell::{Cell, RefCell},
	rc::Rc,
	sync::Mutex,
};

use paperback_core::{
	config::ConfigManager,
	document::DocPos,
	reader_core::{self, SearchOptions},
	session::FindAllLine,
};
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::{DocumentManager, DocumentTab},
	navigation,
};

mod dialog;
mod find_all;
mod results_list;

use dialog::{FindDialogActionParams, FindDialogWidgets, bind_find_dialog_actions, build_find_dialog_ui};
use find_all::{do_find_all, handle_result_go};
use results_list::{ResultsList, clear_results_list};

use crate::ui::navigation::announce;

const MAX_FIND_HISTORY_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FindView {
	Query,
	Results,
}

#[derive(Clone)]
pub struct FindDialogState {
	pub dialog: Dialog,
	find_label: StaticText,
	find_combo: ComboBox,
	options_static_box: Option<StaticBox>,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	find_all_btn: Button,
	results_list: ResultsList,
	go_btn: Button,
	in_progress: Rc<Cell<bool>>,
	view: Rc<Cell<FindView>>,
	origin: Rc<Cell<DocPos>>,
	/// Set only when found text is left unselected, since then the widget holds nothing for `search_origin` to read.
	last_match: Rc<Cell<Option<(DocPos, DocPos)>>>,
	result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	result_labels: Rc<RefCell<Vec<String>>>,
}

impl FindDialogState {
	pub fn new(
		frame: &Frame,
		config: &Rc<Mutex<ConfigManager>>,
		doc_manager: &Rc<Mutex<DocumentManager>>,
		find_dialog: &Rc<Mutex<Option<Self>>>,
		live_region_label: StaticText,
	) -> Self {
		// TRANSLATORS: Title of the Find dialog
		let dialog = Dialog::builder(frame, &t("Find")).build();
		let result_rows = Rc::new(RefCell::new(Vec::new()));
		let result_labels = Rc::new(RefCell::new(Vec::new()));
		let FindDialogWidgets {
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
		} = build_find_dialog_ui(dialog, Rc::clone(&result_rows), Rc::clone(&result_labels));
		bind_find_dialog_actions(FindDialogActionParams {
			frame: *frame,
			dialog,
			find_combo,
			find_prev_btn,
			find_next_btn,
			find_all_btn,
			go_btn,
			results_list,
			cancel_btn,
			config: Rc::clone(config),
			doc_manager: Rc::clone(doc_manager),
			find_dialog: Rc::clone(find_dialog),
			live_region_label,
		});
		let state = Self {
			dialog,
			find_label,
			find_combo,
			options_static_box,
			match_case,
			whole_word,
			use_regex,
			find_prev_btn,
			find_next_btn,
			find_all_btn,
			results_list,
			go_btn,
			in_progress: Rc::new(Cell::new(false)),
			view: Rc::new(Cell::new(FindView::Query)),
			origin: Rc::new(Cell::new(DocPos::ZERO)),
			last_match: Rc::new(Cell::new(None)),
			result_rows,
			result_labels,
		};
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

	/// Searching from the caret on an unselected match would land back on it and Find Next would never move, so the last match's span is used while the caret sits at one of its ends.
	fn search_origin(&self, tab: &DocumentTab) -> (DocPos, DocPos) {
		let (start, end) = navigation::doc_selected_range(tab);
		if start != end {
			return (start, end);
		}
		match self.last_match.get() {
			Some((match_start, match_end)) if start == match_start || start == match_end => (match_start, match_end),
			_ => (start, end),
		}
	}

	fn reveal_match(&self, tab: &mut DocumentTab, start: DocPos, end: DocPos, highlight: bool) {
		navigation::reveal_doc_range(tab, start, end, highlight);
		if !highlight {
			self.last_match.set(Some((start, end)));
		}
	}

	/// So leaving Find All does not keep tens of thousands of rows around or hand NVDA a stale huge list next time.
	fn clear_results(&self) {
		clear_results_list(self.results_list);
		self.result_labels.borrow_mut().clear();
	}

	/// Cancel is shared by both views, so it belongs to neither set.
	fn set_query_windows(&self, visible: bool) {
		self.find_label.show(visible);
		self.find_combo.show(visible);
		if let Some(static_box) = &self.options_static_box {
			static_box.show(visible);
		}
		self.match_case.show(visible);
		self.whole_word.show(visible);
		self.use_regex.show(visible);
		self.find_prev_btn.show(visible);
		self.find_next_btn.show(visible);
		self.find_all_btn.show(visible);
	}

	fn set_results_windows(&self, visible: bool) {
		self.results_list.show(visible);
		self.go_btn.show(visible);
	}

	fn refit(&self) {
		self.dialog.layout();
		self.dialog.fit();
		self.dialog.centre();
	}

	fn switch_to_query_view(&self) {
		self.clear_results();
		if self.view.get() == FindView::Query {
			return;
		}
		self.set_query_windows(true);
		self.set_results_windows(false);
		self.view.set(FindView::Query);
		self.find_next_btn.set_default();
		self.refit();
	}

	fn switch_to_results_view(&self) {
		if self.view.get() == FindView::Results {
			return;
		}
		self.set_query_windows(false);
		self.set_results_windows(true);
		self.view.set(FindView::Results);
		self.go_btn.set_default();
		self.refit();
	}
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
	state.switch_to_query_view();
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

fn ensure_query(state: &FindDialogState, doc_manager: &Rc<Mutex<DocumentManager>>) -> bool {
	if !state.find_text().trim().is_empty() {
		return true;
	}
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
	!state.find_text().trim().is_empty()
}

pub fn handle_find_action(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
	forward: bool,
	from_keyboard: bool,
) {
	ensure_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
	let state = {
		let dialog_state = find_dialog.lock().unwrap();
		dialog_state.as_ref().cloned()
	};
	let Some(state) = state else {
		return;
	};
	state.switch_to_query_view();
	if !ensure_query(&state, doc_manager) {
		show_find_dialog(frame, doc_manager, config, find_dialog, live_region_label);
		return;
	}
	do_find(forward, &state, doc_manager, config, live_region_label, from_keyboard);
}

fn do_find(
	forward: bool,
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
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
	// Read before taking the document lock, so the two locks are never taken in the opposite order to the rest of the app.
	let highlight_found_text = config.lock().unwrap().get_app_bool("highlight_found_text", true);
	let mut options = SearchOptions::empty();
	options.set(SearchOptions::FORWARD, forward);
	options.set(SearchOptions::MATCH_CASE, state.match_case.is_checked());
	options.set(SearchOptions::WHOLE_WORD, state.whole_word.is_checked());
	options.set(SearchOptions::REGEX, state.use_regex.is_checked());
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	if !tab.text_ctrl.is_valid() {
		return;
	}
	// Searches the whole document rather than the loaded window, so a match outside the window can still be reached.
	let (origin_start, origin_end) = state.search_origin(tab);
	let start_pos = if forward { origin_end } else { origin_start };
	let result =
		reader_core::reader_search_with_wrap(&tab.session.handle().document().buffer, &query, start_pos, options);
	tracing::debug!(query = %query, forward, found = result.found, wrapped = result.wrapped, "find search");
	if !result.found {
		drop(dm);
		// TRANSLATORS: Announced when a search finds no matches in the document
		announce(live_region_label, t("Not found."));
		state.switch_to_query_view();
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	// Hiding the dialog and dismissing a menu both start a focus chain NVDA reads over the message, the dialog's earlier than the menu's, so the two wait different lengths of time.
	let dialog_was_shown = state.dialog.is_shown();
	let doc_len = tab.session.document_len();
	if doc_len.is_zero() {
		return;
	}
	let start = result.position.min(doc_len.as_pos());
	let end = result.end.min(doc_len.as_pos());
	let found_line = tab.session.get_line_text(start);
	state.reveal_match(tab, start, end, highlight_found_text);
	drop(dm);
	state.dialog.show(false);
	let message = if result.wrapped {
		// TRANSLATORS: Announced when a search reaches the end of the document and wraps back to the start
		let notice = t("No more results. Wrapping search.");
		if found_line.trim().is_empty() { notice } else { format!("{notice} {}", found_line.trim()) }
	} else {
		found_line
	};
	if !message.trim().is_empty() {
		// Delayed to cut NVDA's ancestor chain as it starts: High priority maps to UIA ImportantMostRecent, which NVDA handles as cancelSpeech + speak (except during Say All).
		if dialog_was_shown {
			announce(live_region_label, message);
		} else {
			navigation::announce_for_command(live_region_label, from_keyboard, message);
		}
	}
}
