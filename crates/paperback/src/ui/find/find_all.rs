use std::{cell::RefCell, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, reader_core};
use patois::t;
use wxdragon::prelude::*;

use super::{
	FindDialogState, FindView, ensure_query,
	results_list::{clear_results_list, populate_results_list, results_selected_index, select_results_row},
};
use crate::ui::{document_manager::DocumentManager, navigation, navigation::announce};

/// How long to leave the empty results list focused before populating it. Long enough for the
/// screen reader to settle on the (empty) list first; populating while NVDA is still deciding how
/// to announce a freshly-shown control is what made it intermittently enumerate all 40k rows.
const RESULT_POPULATE_DELAY_MS: i32 = 150;

/// Lists every line holding a match for the current query under the dialog's options. With no
/// matches it reports "Not found." exactly like [`super::do_find`]; with matches it switches the dialog
/// to the results view, focused on the first line whose match follows the caret.
pub(super) fn do_find_all(
	frame: &Frame,
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let Some(_find_guard) = state.try_begin_find() else {
		return;
	};
	if !ensure_query(state, doc_manager) {
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	let query = state.find_text();
	state.save_settings(config);
	state.add_to_history(config, &query);
	let mut options = reader_core::SearchOptions::empty();
	if state.match_case.is_checked() {
		options |= reader_core::SearchOptions::MATCH_CASE;
	}
	if state.whole_word.is_checked() {
		options |= reader_core::SearchOptions::WHOLE_WORD;
	}
	if state.use_regex.is_checked() {
		options |= reader_core::SearchOptions::REGEX;
	}
	let (rows, origin) = {
		let dm = doc_manager.lock().unwrap();
		let Some(tab) = dm.active_tab() else {
			return;
		};
		if !tab.text_ctrl.is_valid() {
			return;
		}
		let (_, origin) = navigation::doc_selected_range(tab);
		let rows = tab.session.find_all_lines(&query, options);
		drop(dm);
		(rows, origin)
	};
	if rows.is_empty() {
		announce(live_region_label, t("Not found."));
		state.switch_to_query_view();
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	let selected = rows.iter().position(|row| row.matches.iter().any(|m| m.start >= origin)).unwrap_or(0);
	state.origin.set(origin);
	*state.result_rows.borrow_mut() = rows;
	// Show and focus the (still empty) results list first, so the screen reader latches onto the
	// list before it grows to its full size, then populate on the next event-loop tick once the
	// empty list has had a chance to paint and be announced.
	clear_results_list(state.results_list);
	state.switch_to_results_view();
	state.results_list.set_focus();
	schedule_results_population(frame, state, i32::try_from(selected).unwrap_or(0));
}

/// Populates the results list one tick after the empty results view was shown and focused, so the
/// screen reader settles on the list before it grows to tens of thousands of rows. Falls back to
/// populating immediately if the timer cannot be armed.
fn schedule_results_population(frame: &Frame, state: &FindDialogState, selected: i32) {
	let timer_holder: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let state_for_tick = state.clone();
	let timer = Timer::new(frame);
	timer.on_tick(move |_event| {
		populate_find_results(&state_for_tick, selected);
		*holder.borrow_mut() = None;
	});
	if !timer.start(RESULT_POPULATE_DELAY_MS, true) {
		populate_find_results(state, selected);
		return;
	}
	*timer_holder.borrow_mut() = Some(timer);
}

/// Fills the results list from the stored rows and selects/focuses the given row. A no-op once the
/// dialog has left the results view (e.g. it was closed during the deferred populate).
fn populate_find_results(state: &FindDialogState, selected: i32) {
	if state.view.get() != FindView::Results {
		return;
	}
	populate_results_list(state.results_list, &state.result_rows, &state.result_labels);
	select_results_row(state.results_list, selected);
	state.results_list.set_focus();
}

/// Jumps to the selected results row, landing on the match on that line that follows the caret
/// origin (falling back to the line's first match), then behaves like a found Find: selects the
/// range, hides the dialog, and announces the line after the focus chain is cut.
pub(super) fn handle_result_go(
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
) {
	let (start, end, line_text) = {
		let rows = state.result_rows.borrow();
		let selected = results_selected_index(state.results_list).unwrap_or(0);
		let Some(row) = rows.get(selected) else {
			return;
		};
		let origin = state.origin.get();
		let Some(span) = row.matches.iter().find(|m| m.start >= origin).or_else(|| row.matches.first()) else {
			return;
		};
		(span.start, span.end, row.text.clone())
	};
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else {
		return;
	};
	if !tab.text_ctrl.is_valid() {
		return;
	}
	navigation::select_doc_range(tab, start, end);
	drop(dm);
	state.dialog.show(false);
	state.clear_results();
	if !line_text.trim().is_empty() {
		announce(live_region_label, line_text);
	}
}
