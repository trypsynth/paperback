use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, types::DocumentListStatus};
use patois::t;
use wxdragon::prelude::*;

const DIALOG_PADDING: i32 = 10;
const RECENT_DOCS_LIST_WIDTH: i32 = 800;
const RECENT_DOCS_LIST_HEIGHT: i32 = 600;
const RECENT_DOCS_FILENAME_WIDTH: i32 = 250;
const RECENT_DOCS_STATUS_WIDTH: i32 = 100;
const RECENT_DOCS_PATH_WIDTH: i32 = 450;
const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;
const KEY_RETURN: i32 = 13;
const KEY_NUMPAD_ENTER: i32 = 370;

pub struct AllDocumentsResult {
	pub open: Option<String>,
	pub paths_to_close: Vec<String>,
}

pub fn show_all_documents_dialog(
	parent: &Frame,
	config: &Rc<Mutex<ConfigManager>>,
	open_paths: Vec<String>,
) -> AllDocumentsResult {
	let open_paths = Rc::new(open_paths);
	let dialog_title = t("All Documents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_path = Rc::new(Mutex::new(None));
	let paths_to_close: Rc<Mutex<Vec<String>>> = Rc::new(Mutex::new(Vec::new()));
	let search_label = StaticText::builder(&dialog).with_label(&t("&search")).build();
	let search_ctrl = TextCtrl::builder(&dialog).with_size(Size::new(300, -1)).build();
	let doc_list = build_all_documents_list(dialog);
	let (open_button, remove_button, clear_all_button, ok_button) = build_all_documents_buttons(dialog);
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	populate_document_list(&DocumentListParams {
		list: doc_list,
		open_button,
		remove_button,
		clear_all_button,
		config,
		open_paths: open_paths.as_ref(),
		filter: "",
		selection: None,
	});
	bind_all_documents_selection(doc_list, open_button);
	let open_action = make_all_documents_open_action(dialog, doc_list, Rc::clone(&selected_path));
	bind_all_documents_open(doc_list, open_button, &open_action);
	let remove_action = make_all_documents_remove_action(
		dialog,
		doc_list,
		search_ctrl,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
	);
	remove_button.on_click({
		let remove_action = Rc::clone(&remove_action);
		move |_| remove_action()
	});
	bind_all_documents_clear(
		dialog,
		doc_list,
		search_ctrl,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
		Rc::clone(&paths_to_close),
	);
	bind_all_documents_search(
		search_ctrl,
		doc_list,
		open_button,
		remove_button,
		clear_all_button,
		Rc::clone(config),
		Rc::clone(&open_paths),
	);
	bind_all_documents_keys(doc_list, &open_action, &remove_action);
	bind_all_documents_layout(
		dialog,
		AllDocumentsLayout {
			search_label,
			search_ctrl,
			doc_list,
			open_button,
			remove_button,
			clear_all_button,
			ok_button,
		},
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
	let yes_button = Button::builder(&panel).with_id(wxdragon::id::ID_OK).with_label(&t("&Yes")).build();
	let no_button = Button::builder(&panel).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&No")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
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
	dialog.show_modal() == wxdragon::id::ID_OK
}

fn build_all_documents_list(dialog: Dialog) -> ListCtrl {
	let doc_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	doc_list.insert_column(0, &t("File Name"), ListColumnFormat::Left, RECENT_DOCS_FILENAME_WIDTH);
	doc_list.insert_column(1, &t("Status"), ListColumnFormat::Left, RECENT_DOCS_STATUS_WIDTH);
	doc_list.insert_column(2, &t("Path"), ListColumnFormat::Left, RECENT_DOCS_PATH_WIDTH);
	doc_list
}

fn build_all_documents_buttons(dialog: Dialog) -> (Button, Button, Button, Button) {
	let open_button = Button::builder(&dialog).with_label(&t("&Open")).build();
	let remove_button = Button::builder(&dialog).with_label(&t("&Remove")).build();
	let clear_all_button = Button::builder(&dialog).with_label(&t("&Clear All")).build();
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Close")).build();
	(open_button, remove_button, clear_all_button, ok_button)
}

fn bind_all_documents_selection(list: ListCtrl, open_button: Button) {
	let list_for_select = list;
	let open_button_for_select = open_button;
	list.on_item_selected(move |event| {
		let index = event.get_item_index();
		update_open_button_for_index(list_for_select, open_button_for_select, index);
	});
	let list_for_focus = list;
	let open_button_for_focus = open_button;
	list.on_item_focused(move |event| {
		let index = event.get_item_index();
		if index >= 0 {
			update_open_button_for_index(list_for_focus, open_button_for_focus, index);
		}
	});
}

fn make_all_documents_open_action(
	dialog: Dialog,
	list: ListCtrl,
	selected_path: Rc<Mutex<Option<String>>>,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		if let Some(path) = get_selected_path(list)
			&& Path::new(&path).exists()
		{
			*selected_path.lock().unwrap() = Some(path);
			dialog.end_modal(wxdragon::id::ID_OK);
		}
	})
}

fn bind_all_documents_open(list: ListCtrl, open_button: Button, open_action: &Rc<dyn Fn()>) {
	let open_action_for_button = Rc::clone(open_action);
	open_button.on_click(move |_| {
		open_action_for_button();
	});
	let open_action_for_activate = Rc::clone(open_action);
	list.on_item_activated(move |event| {
		if event.get_item_index() >= 0 {
			open_action_for_activate();
		}
	});
}

fn make_all_documents_remove_action(
	dialog: Dialog,
	list: ListCtrl,
	search_ctrl: TextCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
) -> Rc<dyn Fn()> {
	Rc::new(move || {
		let indices = get_selected_indices(list);
		if indices.is_empty() {
			return;
		}
		let confirm_message = if indices.len() == 1 {
			t(
				"Are you sure you want to remove this document from the list? This will also remove its reading position.",
			)
		} else {
			let template = t(
				"Are you sure you want to remove these {} documents from the list? This will also remove their reading positions.",
			);
			template.replace("{}", &indices.len().to_string())
		};
		if !show_yes_no_dialog(&dialog, &confirm_message, &t("Confirm")) {
			return;
		}
		let paths_to_remove: Vec<String> = indices.iter().filter_map(|&i| get_path_for_index(list, i)).collect();
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
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			selection: new_selection,
		});
	})
}

fn bind_all_documents_clear(
	dialog: Dialog,
	list: ListCtrl,
	search_ctrl: TextCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
	paths_to_close: Rc<Mutex<Vec<String>>>,
) {
	clear_button.on_click(move |_| {
		if list.get_item_count() == 0 {
			return;
		}
		if !show_yes_no_dialog(
			&dialog,
			&t("Are you sure you want to remove all documents from the list? This will also remove all reading positions and bookmarks."),
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
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: "",
			selection: None,
		});
	});
}

fn bind_all_documents_search(
	search_ctrl: TextCtrl,
	list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_button: Button,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Rc<Vec<String>>,
) {
	search_ctrl.on_text_updated(move |_event| {
		let filter = search_ctrl.get_value();
		populate_document_list(&DocumentListParams {
			list,
			open_button,
			remove_button,
			clear_all_button: clear_button,
			config: &config,
			open_paths: open_paths.as_ref(),
			filter: &filter,
			selection: None,
		});
	});
}

fn bind_all_documents_keys(list: ListCtrl, open_action: &Rc<dyn Fn()>, remove_action: &Rc<dyn Fn()>) {
	let remove_action_for_keys = Rc::clone(remove_action);
	let open_action_for_keys = Rc::clone(open_action);
	let list_for_keys = list;
	list.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
				remove_action_for_keys();
				event.skip(false);
				return;
			}
			if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
				open_action_for_keys();
				event.skip(false);
				return;
			}
			if key == i32::from(b'A') && event.control_down() {
				list_for_keys.set_item_state(-1, ListItemState::Selected, ListItemState::Selected);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let open_action_for_char = Rc::clone(open_action);
	list.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code()
			&& (key == KEY_RETURN || key == KEY_NUMPAD_ENTER)
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
	doc_list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_all_button: Button,
	ok_button: Button,
}

fn bind_all_documents_layout(dialog: Dialog, layout: AllDocumentsLayout) {
	let AllDocumentsLayout {
		search_label,
		search_ctrl,
		doc_list,
		open_button,
		remove_button,
		clear_all_button,
		ok_button,
	} = layout;
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let search_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	search_sizer.add(&search_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	search_sizer.add(&search_ctrl, 1, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING / 2);
	content_sizer.add_sizer(&search_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(&doc_list, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	doc_list.set_focus();
	let action_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	action_sizer.add(&open_button, 0, SizerFlag::Right, DIALOG_PADDING);
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
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

struct DocumentListParams<'a> {
	list: ListCtrl,
	open_button: Button,
	remove_button: Button,
	clear_all_button: Button,
	config: &'a Rc<Mutex<ConfigManager>>,
	open_paths: &'a [String],
	filter: &'a str,
	selection: Option<i32>,
}

fn populate_document_list(params: &DocumentListParams<'_>) {
	let DocumentListParams {
		list,
		open_button,
		remove_button,
		clear_all_button,
		config,
		open_paths,
		filter,
		selection,
	} = *params;
	list.cleanup_all_custom_data();
	list.delete_all_items();
	let items = {
		let cfg = config.lock().unwrap();
		paperback_core::config::get_sorted_document_list(&cfg, open_paths, filter)
	};
	for item in items {
		let index = i64::from(list.get_item_count());
		list.insert_item(index, &item.filename, None);
		if let Ok(index_u64) = u64::try_from(index) {
			list.set_custom_data(index_u64, item.path.clone());
		}
		let status = match item.status {
			DocumentListStatus::Open => t("Open"),
			DocumentListStatus::Closed => t("Closed"),
			DocumentListStatus::Missing => t("Missing"),
		};
		list.set_item_text_by_column(index, 1, &status);
		list.set_item_text_by_column(index, 2, &item.path);
	}
	if list.get_item_count() > 0 {
		let mut select_index = selection.unwrap_or(0);
		if select_index >= list.get_item_count() {
			select_index = list.get_item_count() - 1;
		}
		list.set_item_state(
			i64::from(select_index),
			ListItemState::Selected | ListItemState::Focused,
			ListItemState::Selected | ListItemState::Focused,
		);
		list.ensure_visible(i64::from(select_index));
		update_open_button_for_index(list, open_button, select_index);
		remove_button.enable(true);
		clear_all_button.enable(true);
	} else {
		open_button.enable(false);
		remove_button.enable(false);
		clear_all_button.enable(false);
	}
}

fn update_open_button_for_index(list: ListCtrl, open_button: Button, index: i32) {
	if index < 0 {
		open_button.enable(false);
		return;
	}
	let status = list.get_item_text(i64::from(index), 1);
	open_button.enable(status != t("Missing"));
}

fn get_selected_index(list: ListCtrl) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 {
		return selected;
	}
	list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused)
}

fn get_selected_indices(list: ListCtrl) -> Vec<i32> {
	let mut indices = Vec::new();
	let mut next = list.get_first_selected_item();
	while next >= 0 {
		indices.push(next);
		next = list.get_next_item(i64::from(next), ListNextItemFlag::All, ListItemState::Selected);
	}
	indices
}

fn get_path_for_index(list: ListCtrl, index: i32) -> Option<String> {
	if index < 0 {
		return None;
	}
	if let Ok(index_u64) = u64::try_from(index)
		&& let Some(data) = list.get_custom_data(index_u64)
		&& let Some(path) = data.as_ref().downcast_ref::<String>()
	{
		return Some(path.clone());
	}
	let path = list.get_item_text(i64::from(index), 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: ListCtrl) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}
