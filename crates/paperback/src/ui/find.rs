use std::{
	cell::{Cell, RefCell},
	rc::Rc,
	sync::Mutex,
};

use bitflags::bitflags;
use paperback_core::{config::ConfigManager, reader_core, session::FindAllLine, util::text::display_len};
use patois::t;
use wxdragon::prelude::*;

use super::{dialogs::DIALOG_PADDING, document_manager::DocumentManager, navigation};

const MAX_FIND_HISTORY_SIZE: usize = 10;
/// How long to leave the empty results list focused before populating it. Long enough for the
/// screen reader to settle on the (empty) list first; populating while NVDA is still deciding how
/// to announce a freshly-shown control is what made it intermittently enumerate all 40k rows.
const RESULT_POPULATE_DELAY_MS: i32 = 150;

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

/// Which of the dialog's two views is showing: the query (find-what, options, find buttons) or
/// the Find All results list. Switching hides one view's windows and shows the other's, then
/// refits the dialog, so Enter always triggers the visible primary button.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FindView {
	Query,
	Results,
}

// The Find All results list. macOS does not expose wxListCtrl to VoiceOver (see the All
// Documents dialog), so it uses a store-backed wxDataViewListCtrl there; everywhere else the
// results are a *virtual* wxListCtrl, which asks the control for text only for the visible rows.
// That means even hundreds of thousands of matches populate in O(1) instead of appending a
// native row per match (the appending was what froze the dialog on large documents).
#[cfg(target_os = "macos")]
type ResultsList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
type ResultsList = ListCtrl;

/// The row label for one result line: "Page N: <text>" when the document is paginated, else the
/// line text itself.
fn result_row_label(row: &FindAllLine) -> String {
	if row.page > 0 { navigation::page_announcement(row.page, &row.text) } else { row.text.clone() }
}

/// The fixed size of the results list. It is both the minimum and the maximum so the dialog's
/// `fit()` never sizes itself to the list's item count (a 40,000-row virtual list has no business
/// making the dialog tens of thousands of pixels tall).
fn results_list_size(dialog: Dialog) -> Size {
	dialog.from_dip(Size::new(600, 500))
}

#[cfg(not(target_os = "macos"))]
fn build_results_list(
	dialog: Dialog,
	_result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	result_labels: Rc<RefCell<Vec<String>>>,
) -> ResultsList {
	let size = results_list_size(dialog);
	let results_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::Virtual | ListCtrlStyle::SingleSel)
		.with_size(size)
		.build();
	results_list.insert_column(0, "", ListColumnFormat::Left, dialog.from_dip_int(600));
	results_list.set_min_size(size);
	results_list.set_max_size(size);
	// TRANSLATORS: Accessible name of the list of Find All results
	results_list.set_accessibility_label(&t("Results"));
	// The virtual list asks for text on demand; hand back a prebuilt label so the request is a
	// cheap clone rather than a per-row translation.
	results_list.set_virtual_text_callback(move |index, _column| {
		let labels = result_labels.borrow();
		usize::try_from(index).ok().and_then(|index| labels.get(index)).cloned().unwrap_or_default()
	});
	results_list.set_item_count(0);
	results_list
}

#[cfg(target_os = "macos")]
fn build_results_list(
	dialog: Dialog,
	_result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	_result_labels: Rc<RefCell<Vec<String>>>,
) -> ResultsList {
	let size = results_list_size(dialog);
	let results_list = DataViewListCtrl::builder(&dialog).with_style(DataViewStyle::RowLines).with_size(size).build();
	results_list.append_text_column(
		"",
		0,
		DataViewAlign::Left,
		dialog.from_dip_int(600),
		DataViewColumnFlags::Resizable,
	);
	results_list.set_min_size(size);
	results_list.set_max_size(size);
	// TRANSLATORS: Accessible name of the list of Find All results
	results_list.set_accessibility_label(&t("Results"));
	results_list
}

#[cfg(not(target_os = "macos"))]
fn populate_results_list(list: ResultsList, rows: &Rc<RefCell<Vec<FindAllLine>>>, labels: &Rc<RefCell<Vec<String>>>) {
	// Build each row's label once, then hand the virtual list its count - it only ever asks for
	// text for the rows it shows, and the callback reads from the prebuilt labels.
	let mut labels = labels.borrow_mut();
	labels.clear();
	{
		let rows = rows.borrow();
		labels.reserve(rows.len());
		for row in rows.iter() {
			labels.push(result_row_label(row));
		}
	}
	list.set_item_count(i64::try_from(labels.len()).unwrap_or(0));
}

#[cfg(target_os = "macos")]
fn populate_results_list(list: ResultsList, rows: &Rc<RefCell<Vec<FindAllLine>>>, _labels: &Rc<RefCell<Vec<String>>>) {
	list.delete_all_items();
	for row in rows.borrow().iter() {
		list.append_item(&[Variant::from(result_row_label(row))]);
	}
}

/// Empties the results list so a fresh Find All starts from the same empty, instantly-focusable
/// state every time (leaving the previous run's rows in place made NVDA enumerate all of them the
/// moment the list was shown and focused again).
#[cfg(not(target_os = "macos"))]
fn clear_results_list(list: ResultsList) {
	list.set_item_count(0);
}

#[cfg(target_os = "macos")]
fn clear_results_list(list: ResultsList) {
	list.delete_all_items();
}

#[cfg(not(target_os = "macos"))]
fn select_results_row(list: ResultsList, index: i32) {
	if index < 0 {
		return;
	}
	list.set_item_state(
		i64::from(index),
		ListItemState::Selected | ListItemState::Focused,
		ListItemState::Selected | ListItemState::Focused,
	);
	list.ensure_visible(i64::from(index));
}

#[cfg(target_os = "macos")]
fn select_results_row(list: ResultsList, index: i32) {
	let Ok(index) = usize::try_from(index) else { return };
	list.select_row(index);
	if let Some(item) = list.row_to_item(index) {
		list.set_current_item(&item);
		list.ensure_visible(&item);
	}
}

#[cfg(not(target_os = "macos"))]
fn results_selected_index(list: ResultsList) -> Option<usize> {
	let index = list.get_first_selected_item();
	if index >= 0 { usize::try_from(index).ok() } else { None }
}

#[cfg(target_os = "macos")]
fn results_selected_index(list: ResultsList) -> Option<usize> {
	list.get_selected_row()
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
	origin: Rc<Cell<i64>>,
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
			origin: Rc::new(Cell::new(0)),
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

	/// Empties the results list and its cached labels, so leaving Find All does not leave tens of
	/// thousands of rows behind (or hand NVDA a stale huge list the next time the dialog opens).
	fn clear_results(&self) {
		clear_results_list(self.results_list);
		self.result_labels.borrow_mut().clear();
	}

	/// Shows or hides every window that belongs to the query view. The Cancel button is shared by
	/// both views, so it is not part of either set.
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

	/// Shows or hides every window that belongs to the Find All results view.
	fn set_results_windows(&self, visible: bool) {
		self.results_list.show(visible);
		self.go_btn.show(visible);
	}

	/// Refits the dialog to whichever view is showing.
	fn refit(&self) {
		self.dialog.layout();
		self.dialog.fit();
		self.dialog.centre();
	}

	/// Shows the query view, hiding the results view if it is showing.
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

	/// Shows the Find All results view, hiding the query view if it is showing.
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

struct FindDialogWidgets {
	find_label: StaticText,
	find_combo: ComboBox,
	options_static_box: Option<StaticBox>,
	match_case: CheckBox,
	whole_word: CheckBox,
	use_regex: CheckBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	find_all_btn: Button,
	cancel_btn: Button,
	results_list: ResultsList,
	go_btn: Button,
}

struct FindDialogActionParams {
	frame: Frame,
	dialog: Dialog,
	find_combo: ComboBox,
	find_prev_btn: Button,
	find_next_btn: Button,
	find_all_btn: Button,
	go_btn: Button,
	results_list: ResultsList,
	cancel_btn: Button,
	config: Rc<Mutex<ConfigManager>>,
	doc_manager: Rc<Mutex<DocumentManager>>,
	find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	live_region_label: StaticText,
}

fn build_find_dialog_ui(
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

fn bind_find_dialog_actions(params: FindDialogActionParams) {
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
	let frame_for_go = frame;
	let find_dialog_for_go = Rc::clone(&find_dialog);
	let doc_manager_for_go = Rc::clone(&doc_manager);
	go_btn.on_click(move |_| {
		if let Some(state) = find_dialog_for_go.lock().unwrap().as_ref() {
			handle_result_go(&frame_for_go, state, &doc_manager_for_go, live_region_label);
		}
	});
	let frame_for_list = frame;
	let find_dialog_for_list = Rc::clone(&find_dialog);
	let doc_manager_for_list = Rc::clone(&doc_manager);
	results_list.on_item_activated(move |_| {
		if let Some(state) = find_dialog_for_list.lock().unwrap().as_ref() {
			handle_result_go(&frame_for_list, state, &doc_manager_for_list, live_region_label);
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

/// Makes sure the combo has a query, pulling the current document selection into it if it is
/// empty. Returns whether a query is now present.
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
	do_find(frame, forward, &state, doc_manager, config, live_region_label);
}

/// Lists every line holding a match for the current query under the dialog's options. With no
/// matches it reports "Not found." exactly like [`do_find`]; with matches it switches the dialog
/// to the results view, focused on the first line whose match follows the caret.
fn do_find_all(
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
		live_region::announce(live_region_label, &t("Not found."));
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
fn handle_result_go(
	frame: &Frame,
	state: &FindDialogState,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
) {
	let (start, end, announce) = {
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
	if !announce.trim().is_empty() {
		navigation::announce_after_delay(frame, live_region_label, announce);
	}
}

fn do_find(
	frame: &Frame,
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
		state.switch_to_query_view();
		state.dialog.show(true);
		state.dialog.raise();
		state.focus_find_text();
		return;
	}
	// Whether the dialog was on screen matters: hiding a visible dialog makes NVDA
	// start the focus-return chain, which the found line must interrupt; a closed
	// dialog produces no chain to cut.
	let dialog_was_shown = state.dialog.is_shown();
	if result.wrapped && !dialog_was_shown {
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
	// Capture the line containing the match while the document lock is held; it is
	// announced after focus has returned to the book.
	let found_line = tab.session.get_line_text(start);
	navigation::select_doc_range(tab, start, end);
	drop(dm);
	state.dialog.show(false);
	if dialog_was_shown {
		// NVDA starts reading the "Paperback, tab control, ..." ancestor chain the
		// moment focus returns to the book. Delay the found-line announcement so it
		// cuts the chain right as it begins: live-region's High priority maps to UIA
		// NotificationProcessing_ImportantMostRecent, which NVDA handles as
		// cancelSpeech() + speak, the same interrupt NVDA itself uses for its own
		// find dialog. (During a say-all NVDA speaks at Spri.NOW instead of
		// cancelling, so continuous reading is not chopped up.)
		// When the search wrapped, the wrap notice was deferred above so it can be
		// folded into this same announcement and cannot be cut off by it.
		let message = if result.wrapped {
			let notice = t("No more results. Wrapping search.");
			if found_line.trim().is_empty() { notice } else { format!("{notice} {}", found_line.trim()) }
		} else {
			found_line
		};
		if !message.trim().is_empty() {
			navigation::announce_after_delay(frame, live_region_label, message);
		}
	} else if result.wrapped {
		// Find-next / Find-previous with the dialog closed. There is no focus chain
		// to cut, and the wrap notice was announced above, so announce at Medium to
		// queue the found line behind it rather than cutting it off.
		if !found_line.trim().is_empty() {
			live_region::announce_with_priority(live_region_label, &found_line, live_region::Priority::Medium);
		}
	} else if !found_line.trim().is_empty() {
		// Find-next / Find-previous with the dialog closed and no wrap: no chain, so
		// announce the found line directly.
		live_region::announce(live_region_label, &found_line);
	}
}
