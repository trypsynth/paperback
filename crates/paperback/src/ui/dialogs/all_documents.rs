use std::{cell::Cell, path::Path, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, parser::build_file_filter_string, types::DocumentListStatus};
use patois::t;
use wxdragon::{ffi, prelude::*, timer::Timer, window::FromWindowWithClassName};

use super::DIALOG_PADDING;

const RECENT_DOCS_LIST_WIDTH: i32 = 800;
const RECENT_DOCS_LIST_HEIGHT: i32 = 600;
const RECENT_DOCS_FILENAME_WIDTH: i32 = 250;
const RECENT_DOCS_STATUS_WIDTH: i32 = 100;
const RECENT_DOCS_PATH_WIDTH: i32 = 450;
const STATUS_BAR_DEBOUNCE_MS: i32 = 10;

// wxListCtrl is not exposed to VoiceOver as an accessible table on macOS.
// wxDataViewListCtrl uses the native data-view implementation there, while the
// existing ListCtrl remains appropriate on Windows and Linux.
#[cfg(target_os = "macos")]
type DocumentList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
type DocumentList = ListCtrl;

pub struct AllDocumentsResult {
	pub open: Option<String>,
	pub paths_to_close: Vec<String>,
}

#[derive(Copy, Clone)]
struct AllDocumentsWidgets {
	list: DocumentList,
	open_button: Button,
	locate_button: Button,
	remove_button: Button,
	clear_all_button: Button,
	status_choice: Choice,
	status_bar: StatusBar,
}

#[derive(Clone)]
struct StatusBarDebounce {
	timer: Rc<Timer<Dialog>>,
	suppress: Rc<Cell<bool>>,
}

impl StatusBarDebounce {
	fn request_update(&self) {
		if !self.suppress.get() {
			self.timer.start(STATUS_BAR_DEBOUNCE_MS, true);
		}
	}
}

pub fn show_all_documents_dialog(
	parent: &Frame,
	config: &Rc<Mutex<ConfigManager>>,
	open_paths: Vec<String>,
) -> AllDocumentsResult {
	let open_paths = Rc::new(open_paths);
	// TRANSLATORS: Title of the All Documents dialog
	let dialog_title = t("All Documents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_path = Rc::new(Mutex::new(None));
	let paths_to_close: Rc<Mutex<Vec<String>>> = Rc::new(Mutex::new(Vec::new()));
	// TRANSLATORS: Label for the search input field in the All Documents dialog
	let search_label = StaticText::builder(&dialog).with_label(&t("&search")).build();
	let search_ctrl = TextCtrl::builder(&dialog).with_size(Size::new(300, -1)).build();
	let (status_label, status_choice) = build_all_documents_status_choice(dialog);
	let list = build_all_documents_list(dialog);
	let status_bar = build_all_documents_status_bar(dialog);
	let (open_button, locate_button, remove_button, clear_all_button, ok_button) = build_all_documents_buttons(dialog);
	let widgets = AllDocumentsWidgets {
		list,
		open_button,
		locate_button,
		remove_button,
		clear_all_button,
		status_choice,
		status_bar,
	};
	dialog.set_escape_id(ID_CANCEL);
	let status_debounce =
		StatusBarDebounce { timer: Rc::new(Timer::new(&dialog)), suppress: Rc::new(Cell::new(false)) };
	status_debounce.timer.on_tick(move |_event| {
		update_document_status_bar(widgets.list, widgets.status_bar);
	});
	populate_document_list(&DocumentListParams {
		widgets,
		config,
		open_paths: open_paths.as_ref(),
		filter: "",
		status_filter: None,
		selection: None,
		status_debounce: &status_debounce,
	});
	bind_all_documents_selection(widgets, status_debounce.clone());
	let open_action = make_all_documents_open_action(dialog, widgets.list, Rc::clone(&selected_path));
	bind_all_documents_open(widgets.list, widgets.open_button, &open_action);
	let remove_action = make_all_documents_remove_action(
		dialog,
		widgets,
		search_ctrl,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
		status_debounce.clone(),
	);
	widgets.remove_button.on_click({
		let remove_action = Rc::clone(&remove_action);
		move |_| remove_action()
	});
	bind_all_documents_locate(
		dialog,
		widgets,
		search_ctrl,
		Rc::clone(config),
		Rc::clone(&open_paths),
		status_debounce.clone(),
	);
	bind_all_documents_clear(
		dialog,
		widgets,
		search_ctrl,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
		status_debounce.clone(),
	);
	bind_all_documents_search(search_ctrl, widgets, Rc::clone(config), Rc::clone(&open_paths), status_debounce.clone());
	bind_all_documents_status_choice(
		widgets,
		search_ctrl,
		Rc::clone(config),
		Rc::clone(&open_paths),
		status_debounce.clone(),
	);
	bind_all_documents_keys(widgets, &open_action, &remove_action, status_debounce);
	bind_all_documents_layout(
		dialog,
		AllDocumentsLayout { search_label, search_ctrl, status_label, widgets, ok_button },
	);
	dialog.show_modal();
	AllDocumentsResult {
		open: selected_path.lock().unwrap().clone(),
		paths_to_close: paths_to_close.lock().unwrap().clone(),
	}
}

fn show_yes_no_dialog(parent: &dyn WxWidget, message: &str, title: &str) -> bool {
	let dialog = Dialog::builder(parent, title).build();
	let panel = Panel::builder(&dialog).build();
	let message_label = StaticText::builder(&panel).with_label(message).build();
	// TRANSLATORS: Label for the confirmation dialog "Yes" button
	let yes_button = Button::builder(&panel).with_id(ID_OK).with_label(&t("&Yes")).build();
	// TRANSLATORS: Label for the confirmation dialog "No" button
	let no_button = Button::builder(&panel).with_id(ID_CANCEL).with_label(&t("&No")).build();
	dialog.set_escape_id(ID_CANCEL);
	dialog.set_affirmative_id(ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message_label, 0, SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&yes_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&no_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 0);
	panel.set_sizer(content_sizer, true);
	let dialog_sizer = BoxSizer::builder(Orientation::Vertical).build();
	dialog_sizer.add(&panel, 1, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(dialog_sizer, true);
	dialog.centre();
	dialog.show_modal() == ID_OK
}

#[cfg(not(target_os = "macos"))]
fn build_all_documents_list(dialog: Dialog) -> DocumentList {
	let doc_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	// TRANSLATORS: Column header for the document filename in the All Documents list
	doc_list.insert_column(0, &t("File Name"), ListColumnFormat::Left, RECENT_DOCS_FILENAME_WIDTH);
	// TRANSLATORS: Column header for the document status (e.g. Open, Closed, Missing) in the All Documents list
	doc_list.insert_column(1, &t("Status"), ListColumnFormat::Left, RECENT_DOCS_STATUS_WIDTH);
	// TRANSLATORS: Column header for the file path in the All Documents list
	doc_list.insert_column(2, &t("Path"), ListColumnFormat::Left, RECENT_DOCS_PATH_WIDTH);
	doc_list
}

#[cfg(target_os = "macos")]
fn build_all_documents_list(dialog: Dialog) -> DocumentList {
	let doc_list = DataViewListCtrl::builder(&dialog)
		.with_style(DataViewStyle::Multiple | DataViewStyle::RowLines)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	let column_flags = DataViewColumnFlags::Resizable;
	// TRANSLATORS: Column header for the document filename in the All Documents list
	doc_list.append_text_column(&t("File Name"), 0, DataViewAlign::Left, RECENT_DOCS_FILENAME_WIDTH, column_flags);
	// TRANSLATORS: Column header for the document status (e.g. Open, Closed, Missing) in the All Documents list
	doc_list.append_text_column(&t("Status"), 1, DataViewAlign::Left, RECENT_DOCS_STATUS_WIDTH, column_flags);
	// TRANSLATORS: Column header for the file path in the All Documents list
	doc_list.append_text_column(&t("Path"), 2, DataViewAlign::Left, RECENT_DOCS_PATH_WIDTH, column_flags);
	doc_list
}

fn build_all_documents_status_choice(dialog: Dialog) -> (StaticText, Choice) {
	// TRANSLATORS: Label for the status filter dropdown in the All Documents dialog
	let status_label_text = t("&Status:");
	let status_label = StaticText::builder(&dialog).with_label(&status_label_text).build();
	let status_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: Option in the All Documents status filter to show documents of every status
	status_choice.append(&t("All"));
	// TRANSLATORS: Status of a document that is currently open in a tab
	status_choice.append(&t("Open"));
	// TRANSLATORS: Status of a document that was previously opened but is currently closed
	status_choice.append(&t("Closed"));
	// TRANSLATORS: Status of a document whose file could not be found on disk
	status_choice.append(&t("Missing"));
	status_choice.set_selection(0);
	#[cfg(target_os = "macos")]
	status_choice.set_accessibility_label(status_label_text.replace('&', "").trim_end_matches(':').trim());
	(status_label, status_choice)
}

fn build_all_documents_status_bar(dialog: Dialog) -> StatusBar {
	let raw = unsafe { ffi::wxd_StatusBar_Create(dialog.handle_ptr(), ffi::wxd_Id::try_from(ID_ANY).unwrap_or(-1), 0) };
	unsafe { StatusBar::from_ptr(raw.cast::<ffi::wxd_Window_t>()) }
}

fn build_all_documents_buttons(dialog: Dialog) -> (Button, Button, Button, Button, Button) {
	// TRANSLATORS: Button label to open the selected document
	let open_button = Button::builder(&dialog).with_label(&t("&Open")).build();
	// TRANSLATORS: Button label to locate a missing file on disk
	let locate_button = Button::builder(&dialog).with_label(&t("&Locate…")).build();
	// TRANSLATORS: Button label to remove selected documents from the list
	let remove_button = Button::builder(&dialog).with_label(&t("&Remove")).build();
	// TRANSLATORS: Button label to clear all documents from the list
	let clear_all_button = Button::builder(&dialog).with_label(&t("&Clear All")).build();
	// TRANSLATORS: Label for a button that closes a dialog
	let ok_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Close")).build();
	locate_button.enable(false);
	(open_button, locate_button, remove_button, clear_all_button, ok_button)
}

#[cfg(not(target_os = "macos"))]
fn bind_all_documents_selection(widgets: AllDocumentsWidgets, status_debounce: StatusBarDebounce) {
	let AllDocumentsWidgets { list, open_button, locate_button, .. } = widgets;
	let list_for_select = list;
	let open_button_for_select = open_button;
	let status_debounce_for_select = status_debounce.clone();
	list.on_item_selected(move |event| {
		let index = event.get_item_index();
		update_open_button_for_index(list_for_select, open_button_for_select, index);
		update_locate_button(list_for_select, locate_button);
		status_debounce_for_select.request_update();
	});
	let list_for_focus = list;
	let open_button_for_focus = open_button;
	let status_debounce_for_focus = status_debounce.clone();
	list.on_item_focused(move |event| {
		let index = event.get_item_index();
		if index >= 0 {
			update_open_button_for_index(list_for_focus, open_button_for_focus, index);
			update_locate_button(list_for_focus, locate_button);
			status_debounce_for_focus.request_update();
		}
	});
	list.on_item_deselected(move |_| {
		update_locate_button(list, locate_button);
		status_debounce.request_update();
	});
}

#[cfg(target_os = "macos")]
fn bind_all_documents_selection(widgets: AllDocumentsWidgets, status_debounce: StatusBarDebounce) {
	let AllDocumentsWidgets { list, open_button, locate_button, .. } = widgets;
	list.on_selection_changed(move |event| {
		if let Some(index) = event.get_item().and_then(|item| list.item_to_row(&item)) {
			update_open_button_for_index(list, open_button, i32::try_from(index).unwrap_or(i32::MAX));
		} else {
			open_button.enable(false);
		}
		update_locate_button(list, locate_button);
		status_debounce.request_update();
	});
}

fn make_all_documents_open_action(
	dialog: Dialog,
	list: DocumentList,
	selected_path: Rc<Mutex<Option<String>>>,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		if let Some(path) = get_selected_path(list)
			&& Path::new(&path).exists()
		{
			*selected_path.lock().unwrap() = Some(path);
			dialog.end_modal(ID_OK);
		}
	})
}

fn bind_all_documents_open(list: DocumentList, open_button: Button, open_action: &Rc<dyn Fn()>) {
	let open_action_for_button = Rc::clone(open_action);
	open_button.on_click(move |_| {
		open_action_for_button();
	});
	let open_action_for_activate = Rc::clone(open_action);
	#[cfg(not(target_os = "macos"))]
	list.on_item_activated(move |event| {
		if event.get_item_index() >= 0 {
			open_action_for_activate();
		}
	});
	#[cfg(target_os = "macos")]
	list.on_item_activated(move |event| {
		if event.get_item().is_some() {
			open_action_for_activate();
		}
	});
}

fn make_all_documents_remove_action(
	dialog: Dialog,
	widgets: AllDocumentsWidgets,
	search_ctrl: TextCtrl,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
	status_debounce: StatusBarDebounce,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		let indices = get_selected_indices(widgets.list);
		if indices.is_empty() {
			return;
		}
		let confirm_message = if indices.len() == 1 {
			// TRANSLATORS: Confirmation prompt when removing a single document.
			t(
				"Are you sure you want to remove the selected document? This will also remove its reading position and bookmarks.",
			)
		} else {
			// TRANSLATORS: Confirmation prompt when removing multiple documents. The {} placeholder is replaced with the number of documents.
			let template = t(
				"Are you sure you want to remove the {} selected documents? This will also remove their reading positions and bookmarks.",
			);
			template.replace("{}", &indices.len().to_string())
		};
		// TRANSLATORS: Title of the confirmation dialog
		if !show_yes_no_dialog(&dialog, &confirm_message, &t("Confirm")) {
			return;
		}
		let paths_to_remove: Vec<String> =
			indices.iter().filter_map(|&i| get_path_for_index(widgets.list, i)).collect();
		{
			let cfg = config.lock().unwrap();
			for path in &paths_to_remove {
				cfg.remove_document_history(path);
			}
			cfg.flush();
		}
		{
			let mut to_close = paths_to_close.lock().unwrap();
			for path in &paths_to_remove {
				if open_paths.contains(path) && !to_close.contains(path) {
					to_close.push(path.clone());
				}
			}
		}
		let new_selection = indices.iter().copied().max();
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			widgets,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			status_filter: status_filter_from_choice(widgets.status_choice),
			selection: new_selection,
			status_debounce: &status_debounce,
		});
	})
}

fn bind_all_documents_clear(
	dialog: Dialog,
	widgets: AllDocumentsWidgets,
	search_ctrl: TextCtrl,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
	status_debounce: StatusBarDebounce,
) {
	widgets.clear_all_button.on_click(move |_| {
		if document_list_item_count(widgets.list) == 0 {
			return;
		}
		if !show_yes_no_dialog(
			&dialog,
			// TRANSLATORS: Confirmation prompt when clearing all documents from the list.
			&t("Are you sure you want to remove all documents from the list? This will also remove all reading positions and bookmarks."),
			// TRANSLATORS: Title of the confirmation dialog
			&t("Confirm"),
		) {
			return;
		}
		{
			let cfg = config.lock().unwrap();
			let all_docs = cfg.get_all_documents();
			{
				let mut to_close = paths_to_close.lock().unwrap();
				for path in &all_docs {
					if open_paths.contains(path) && !to_close.contains(path) {
						to_close.push(path.clone());
					}
				}
			}
			for path in &all_docs {
				cfg.remove_document_history(path);
			}
			cfg.flush();
		}
		search_ctrl.set_value("");
		populate_document_list(&DocumentListParams {
			widgets,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: "",
			status_filter: status_filter_from_choice(widgets.status_choice),
			selection: None,
			status_debounce: &status_debounce,
		});
	});
}

fn bind_all_documents_search(
	search_ctrl: TextCtrl,
	widgets: AllDocumentsWidgets,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	status_debounce: StatusBarDebounce,
) {
	search_ctrl.on_text_updated(move |_event| {
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			widgets,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			status_filter: status_filter_from_choice(widgets.status_choice),
			selection: None,
			status_debounce: &status_debounce,
		});
	});
}

fn bind_all_documents_status_choice(
	widgets: AllDocumentsWidgets,
	search_ctrl: TextCtrl,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	status_debounce: StatusBarDebounce,
) {
	widgets.status_choice.on_selection_changed(move |_event| {
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			widgets,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			status_filter: status_filter_from_choice(widgets.status_choice),
			selection: None,
			status_debounce: &status_debounce,
		});
	});
}

fn bind_all_documents_keys(
	widgets: AllDocumentsWidgets,
	open_action: &Rc<dyn Fn()>,
	remove_action: &Rc<dyn Fn()>,
	status_debounce: StatusBarDebounce,
) {
	let remove_action_for_keys = Rc::clone(remove_action);
	let open_action_for_keys = Rc::clone(open_action);
	let list_for_keys = widgets.list;
	widgets.list.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == WXK_DELETE || key == WXK_NUMPAD_DELETE {
				remove_action_for_keys();
				event.skip(false);
				return;
			}
			if key == WXK_RETURN || key == WXK_NUMPAD_ENTER {
				open_action_for_keys();
				event.skip(false);
				return;
			}
			if key == i32::from(b'A') && event.control_down() {
				set_all_document_list_items_selected(list_for_keys, !event.shift_down());
				status_debounce.request_update();
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let open_action_for_char = Rc::clone(open_action);
	widgets.list.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code()
			&& (key == WXK_RETURN || key == WXK_NUMPAD_ENTER)
		{
			open_action_for_char();
			event.skip(false);
			return;
		}
		event.skip(true);
	});
}

#[derive(Copy, Clone)]
struct AllDocumentsLayout {
	search_label: StaticText,
	search_ctrl: TextCtrl,
	status_label: StaticText,
	widgets: AllDocumentsWidgets,
	ok_button: Button,
}

fn bind_all_documents_layout(dialog: Dialog, layout: AllDocumentsLayout) {
	let AllDocumentsLayout { search_label, search_ctrl, status_label, widgets, ok_button } = layout;
	let AllDocumentsWidgets {
		list,
		open_button,
		locate_button,
		remove_button,
		clear_all_button,
		status_choice,
		status_bar,
	} = widgets;
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let search_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	search_sizer.add(&search_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	search_sizer.add(&search_ctrl, 1, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING / 2);
	search_sizer.add(&status_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	search_sizer.add(&status_choice, 0, SizerFlag::AlignCenterVertical, 0);
	content_sizer.add_sizer(&search_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	list.set_focus();
	let action_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	action_sizer.add(&open_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&locate_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&remove_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&clear_all_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(
		&action_sizer,
		0,
		SizerFlag::AlignLeft | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let ok_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	ok_sizer.add_stretch_spacer(1);
	ok_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&ok_sizer, 0, SizerFlag::Expand, 0);
	content_sizer.add(
		&status_bar,
		0,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

struct DocumentListParams<'a> {
	widgets: AllDocumentsWidgets,
	config: &'a Rc<Mutex<ConfigManager>>,
	open_paths: &'a [String],
	filter: &'a str,
	status_filter: Option<DocumentListStatus>,
	selection: Option<i32>,
	status_debounce: &'a StatusBarDebounce,
}

fn populate_document_list(params: &DocumentListParams<'_>) {
	let DocumentListParams { widgets, config, open_paths, filter, status_filter, selection, status_debounce } = *params;
	let AllDocumentsWidgets { list, open_button, locate_button, remove_button, clear_all_button, status_bar, .. } =
		widgets;
	status_debounce.suppress.set(true);
	clear_document_list(list);
	let items = {
		let cfg = config.lock().unwrap();
		paperback_core::config::get_sorted_document_list(&cfg, open_paths, filter, status_filter)
	};
	for item in items {
		let status = match item.status {
			// TRANSLATORS: Status of a document that is currently open in a tab
			DocumentListStatus::Open => t("Open"),
			// TRANSLATORS: Status of a document that was previously opened but is currently closed
			DocumentListStatus::Closed => t("Closed"),
			// TRANSLATORS: Status of a document whose file could not be found on disk
			DocumentListStatus::Missing => t("Missing"),
		};
		append_document_list_item(list, &item.filename, &status, &item.path);
	}
	let item_count = document_list_item_count(list);
	if item_count > 0 {
		let mut select_index = selection.unwrap_or(0);
		if select_index >= item_count {
			select_index = item_count - 1;
		}
		select_document_list_item(list, select_index);
		update_open_button_for_index(list, open_button, select_index);
		update_locate_button(list, locate_button);
		remove_button.enable(true);
		clear_all_button.enable(true);
	} else {
		open_button.enable(false);
		locate_button.enable(false);
		remove_button.enable(false);
		clear_all_button.enable(false);
	}
	status_debounce.suppress.set(false);
	update_document_status_bar(list, status_bar);
}

fn status_filter_from_choice(status_choice: Choice) -> Option<DocumentListStatus> {
	match status_choice.get_selection().unwrap_or(0) {
		1 => Some(DocumentListStatus::Open),
		2 => Some(DocumentListStatus::Closed),
		3 => Some(DocumentListStatus::Missing),
		_ => None,
	}
}

fn update_document_status_bar(list: DocumentList, status_bar: StatusBar) {
	let total = document_list_item_count(list);
	let selected = get_selected_indices(list).len();
	let formatted = format_document_status_text(total, selected);
	status_bar.set_status_text(&formatted, 0);
}

fn format_document_status_text(total: i32, selected: usize) -> String {
	if selected == 0 {
		if total == 0 {
			// TRANSLATORS: Status bar text in the All Documents dialog when the list is empty
			t("No documents.")
		} else if total == 1 {
			// TRANSLATORS: Status bar text in the All Documents dialog when the list contains exactly one document and none are selected
			t("1 document.")
		} else {
			// TRANSLATORS: Status bar text in the All Documents dialog showing the total number of documents in the list. The %d placeholder is replaced with the count.
			t("%d documents.").replacen("%d", &total.to_string(), 1)
		}
	} else if total == 1 {
		// TRANSLATORS: Status bar text in the All Documents dialog when the single document in the list is selected
		t("1 of 1 document selected.")
	} else if usize::try_from(total).is_ok_and(|total| total == selected) {
		// TRANSLATORS: Status bar text in the All Documents dialog when every document in the list is selected. The %d placeholder is replaced with the total count.
		t("All %d documents selected.").replacen("%d", &total.to_string(), 1)
	} else {
		// TRANSLATORS: Status bar text in the All Documents dialog showing how many of the total documents are selected. The first %d is the selected count, the second %d is the total count.
		t("%d of %d documents selected.").replacen("%d", &selected.to_string(), 1).replacen("%d", &total.to_string(), 1)
	}
}

fn update_open_button_for_index(list: DocumentList, open_button: Button, index: i32) {
	if index < 0 {
		open_button.enable(false);
		return;
	}
	let status = get_document_list_text(list, index, 1);
	open_button.enable(status != t("Missing"));
}

fn update_locate_button(list: DocumentList, locate_button: Button) {
	let indices = get_selected_indices(list);
	let enabled = if indices.len() == 1 { get_document_list_text(list, indices[0], 1) == t("Missing") } else { false };
	locate_button.enable(enabled);
}

fn bind_all_documents_locate(
	dialog: Dialog,
	widgets: AllDocumentsWidgets,
	search_ctrl: TextCtrl,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	status_debounce: StatusBarDebounce,
) {
	widgets.locate_button.on_click(move |_| {
		let Some(old_path) = get_selected_path(widgets.list) else { return };
		let filename = Path::new(&old_path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
		let wildcard = build_file_filter_string();
		let file_dialog = FileDialog::builder(&dialog)
			// TRANSLATORS: Message/prompt shown in the file picker dialog to locate a missing book
			.with_message(&t("Locate Book"))
			.with_default_file(&filename)
			.with_wildcard(&wildcard)
			.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
			.build();
		if file_dialog.show_modal() != ID_OK {
			return;
		}
		let Some(new_path) = file_dialog.get_path() else { return };
		{
			let cfg = config.lock().unwrap();
			cfg.rename_document_path(&old_path, &new_path);
			cfg.flush();
		}
		let filter = search_ctrl.get_value();
		let selected_index = get_selected_index(widgets.list);
		populate_document_list(&DocumentListParams {
			widgets,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			status_filter: status_filter_from_choice(widgets.status_choice),
			selection: if selected_index >= 0 { Some(selected_index) } else { None },
			status_debounce: &status_debounce,
		});
	});
}

#[cfg(not(target_os = "macos"))]
fn get_selected_index(list: DocumentList) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 {
		return selected;
	}
	list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused)
}

#[cfg(target_os = "macos")]
fn get_selected_index(list: DocumentList) -> i32 {
	list.get_selected_row().and_then(|index| i32::try_from(index).ok()).unwrap_or(-1)
}

#[cfg(not(target_os = "macos"))]
fn get_selected_indices(list: DocumentList) -> Vec<i32> {
	let mut indices = Vec::new();
	let mut next = list.get_first_selected_item();
	while next >= 0 {
		indices.push(next);
		next = list.get_next_item(i64::from(next), ListNextItemFlag::All, ListItemState::Selected);
	}
	indices
}

#[cfg(target_os = "macos")]
fn get_selected_indices(list: DocumentList) -> Vec<i32> {
	(0..list.get_item_count())
		.filter(|&index| list.is_row_selected(index))
		.filter_map(|index| i32::try_from(index).ok())
		.collect()
}

fn get_path_for_index(list: DocumentList, index: i32) -> Option<String> {
	if index < 0 {
		return None;
	}
	#[cfg(not(target_os = "macos"))]
	if let Ok(index_u64) = u64::try_from(index)
		&& let Some(data) = list.get_custom_data(index_u64)
		&& let Some(path) = data.as_ref().downcast_ref::<String>()
	{
		return Some(path.clone());
	}
	let path = get_document_list_text(list, index, 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: DocumentList) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}

#[cfg(not(target_os = "macos"))]
fn document_list_item_count(list: DocumentList) -> i32 {
	list.get_item_count()
}

#[cfg(target_os = "macos")]
fn document_list_item_count(list: DocumentList) -> i32 {
	i32::try_from(list.get_item_count()).unwrap_or(i32::MAX)
}

#[cfg(not(target_os = "macos"))]
fn clear_document_list(list: DocumentList) {
	list.cleanup_all_custom_data();
	list.delete_all_items();
}

#[cfg(target_os = "macos")]
fn clear_document_list(list: DocumentList) {
	list.delete_all_items();
}

#[cfg(not(target_os = "macos"))]
fn append_document_list_item(list: DocumentList, filename: &str, status: &str, path: &str) {
	let index = i64::from(list.get_item_count());
	list.insert_item(index, filename, None);
	if let Ok(index_u64) = u64::try_from(index) {
		list.set_custom_data(index_u64, path.to_owned());
	}
	list.set_item_text_by_column(index, 1, status);
	list.set_item_text_by_column(index, 2, path);
}

#[cfg(target_os = "macos")]
fn append_document_list_item(list: DocumentList, filename: &str, status: &str, path: &str) {
	list.append_item(&[Variant::from(filename), Variant::from(status), Variant::from(path)]);
}

#[cfg(not(target_os = "macos"))]
fn select_document_list_item(list: DocumentList, index: i32) {
	list.set_item_state(
		i64::from(index),
		ListItemState::Selected | ListItemState::Focused,
		ListItemState::Selected | ListItemState::Focused,
	);
	list.ensure_visible(i64::from(index));
}

#[cfg(target_os = "macos")]
fn select_document_list_item(list: DocumentList, index: i32) {
	let Ok(index) = usize::try_from(index) else { return };
	list.select_row(index);
	if let Some(item) = list.row_to_item(index) {
		list.set_current_item(&item);
		list.ensure_visible(&item);
	}
}

#[cfg(not(target_os = "macos"))]
fn set_all_document_list_items_selected(list: DocumentList, selected: bool) {
	let state = if selected { ListItemState::Selected } else { ListItemState::default() };
	list.set_item_state(-1, state, ListItemState::Selected);
}

#[cfg(target_os = "macos")]
fn set_all_document_list_items_selected(list: DocumentList, selected: bool) {
	if selected {
		list.select_all();
	} else {
		list.unselect_all();
	}
}

#[cfg(not(target_os = "macos"))]
fn get_document_list_text(list: DocumentList, index: i32, column: usize) -> String {
	list.get_item_text(i64::from(index), column as i32)
}

#[cfg(target_os = "macos")]
fn get_document_list_text(list: DocumentList, index: i32, column: usize) -> String {
	usize::try_from(index).map_or_else(|_| String::new(), |index| list.get_text_value(index, column))
}
