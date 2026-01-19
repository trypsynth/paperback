use std::{path::Path, rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use crate::{config::ConfigManager, document::DocumentStats, ui_types::DocumentListStatus};

const DIALOG_PADDING: i32 = 10;
const RECENT_DOCS_LIST_WIDTH: i32 = 800;
const RECENT_DOCS_LIST_HEIGHT: i32 = 600;
const RECENT_DOCS_FILENAME_WIDTH: i32 = 250;
const RECENT_DOCS_STATUS_WIDTH: i32 = 100;
const RECENT_DOCS_PATH_WIDTH: i32 = 450;
const DOC_INFO_WIDTH: i32 = 600;
const DOC_INFO_HEIGHT: i32 = 400;
const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;
const KEY_ESCAPE: i32 = 27;

pub fn show_document_info_dialog(parent: &Frame, path: &Path, title: &str, author: &str, stats: &DocumentStats) {
	let dialog = Dialog::builder(parent, "Document Info").build();

	let info_ctrl = TextCtrl::builder(&dialog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
		.with_size(Size::new(DOC_INFO_WIDTH, DOC_INFO_HEIGHT))
		.build();

	let mut info = String::new();
	info.push_str(&format!("Path: {}\n", path.display()));
	if !title.is_empty() {
		info.push_str(&format!("Title: {title}\n"));
	}
	if !author.is_empty() {
		info.push_str(&format!("Author: {author}\n"));
	}
	info.push_str(&format!("Words: {}\n", stats.word_count));
	info.push_str(&format!("Lines: {}\n", stats.line_count));
	info.push_str(&format!("Characters: {}\n", stats.char_count));
	info.push_str(&format!("Characters (excluding spaces): {}\n", stats.char_count_no_whitespace));
	info_ctrl.set_value(&info);

	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&info_ctrl, dialog);

	let ok_button = Button::builder(&dialog).with_label("OK").build();
	bind_escape_to_close(&ok_button, dialog);
	let dialog_copy = dialog;
	ok_button.on_click(move |_| {
		dialog_copy.end_modal(wxdragon::id::ID_OK);
	});

	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&info_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);

	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);

	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	dialog.show_modal();
}

pub fn show_all_documents_dialog(
	parent: &Frame,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Vec<String>,
) -> Option<String> {
	let open_paths = Rc::new(open_paths);
	let dialog = Dialog::builder(parent, "All Documents").build();
	let selected_path = Rc::new(Mutex::new(None));

	let search_label = StaticText::builder(&dialog).with_label("&search").build();
	let search_ctrl = TextCtrl::builder(&dialog).with_size(Size::new(300, -1)).build();

	let doc_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::SingleSel)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	doc_list.insert_column(0, "File Name", ListColumnFormat::Left, RECENT_DOCS_FILENAME_WIDTH);
	doc_list.insert_column(1, "Status", ListColumnFormat::Left, RECENT_DOCS_STATUS_WIDTH);
	doc_list.insert_column(2, "Path", ListColumnFormat::Left, RECENT_DOCS_PATH_WIDTH);

	let open_button = Button::builder(&dialog).with_label("&Open").build();
	let remove_button = Button::builder(&dialog).with_label("&Remove").build();
	let clear_all_button = Button::builder(&dialog).with_label("&Clear All").build();
	bind_escape_to_close(&open_button, dialog);
	bind_escape_to_close(&remove_button, dialog);
	bind_escape_to_close(&clear_all_button, dialog);

	populate_document_list(
		&doc_list,
		&open_button,
		&remove_button,
		&clear_all_button,
		&config,
		open_paths.as_ref(),
		"",
		None,
	);

	let list_for_select = doc_list;
	let open_button_for_select = open_button;
	doc_list.on_item_selected(move |event| {
		let index = event.get_item_index();
		update_open_button_for_index(&list_for_select, &open_button_for_select, index);
	});

	let list_for_focus = doc_list;
	let open_button_for_focus = open_button;
	doc_list.on_item_focused(move |event| {
		let index = event.get_item_index();
		if index >= 0 {
			list_for_focus.set_item_state(
				index as i64,
				ListItemState::Selected | ListItemState::Focused,
				ListItemState::Selected | ListItemState::Focused,
			);
			update_open_button_for_index(&list_for_focus, &open_button_for_focus, index);
		}
	});

	let dialog_for_activate = dialog;
	let list_for_activate = doc_list;
	let selected_for_activate = Rc::clone(&selected_path);
	doc_list.on_item_activated(move |event| {
		let index = event.get_item_index();
		if index >= 0 {
			let path = list_for_activate.get_item_text(index as i64, 2);
			if Path::new(&path).exists() {
				*selected_for_activate.lock().unwrap() = Some(path);
				dialog_for_activate.end_modal(wxdragon::id::ID_OK);
			}
		}
	});

	let dialog_for_open = dialog;
	let list_for_open = doc_list;
	let selected_for_open = Rc::clone(&selected_path);
	open_button.on_click(move |_| {
		if let Some(path) = get_selected_path(&list_for_open) {
			if Path::new(&path).exists() {
				*selected_for_open.lock().unwrap() = Some(path);
				dialog_for_open.end_modal(wxdragon::id::ID_OK);
			}
		}
	});

	let config_for_remove = Rc::clone(&config);
	let list_for_remove = doc_list;
	let open_button_for_remove = open_button;
	let remove_button_for_remove = remove_button;
	let clear_button_for_remove = clear_all_button;
	let open_paths_for_remove = Rc::clone(&open_paths);
	let remove_action = Rc::new(move || {
		let index = get_selected_index(&list_for_remove);
		if index < 0 {
			return;
		}
		let confirm = MessageDialog::builder(
			&dialog,
			"Are you sure you want to remove this document from the list? This will also remove its reading position.",
			"Confirm",
		)
		.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
		.build();
		if confirm.show_modal() != wxdragon::id::ID_YES {
			return;
		}
		let Some(path_to_remove) = get_path_for_index(&list_for_remove, index) else {
			return;
		};
		{
			let mut cfg = config_for_remove.lock().unwrap();
			cfg.remove_document_history(&path_to_remove);
			cfg.flush();
		}
		populate_document_list(
			&list_for_remove,
			&open_button_for_remove,
			&remove_button_for_remove,
			&clear_button_for_remove,
			&config_for_remove,
			open_paths_for_remove.as_ref(),
			"",
			Some(index),
		);
	});
	let remove_action_for_button = Rc::clone(&remove_action);
	remove_button.on_click(move |_| {
		remove_action_for_button();
	});

	let config_for_clear = Rc::clone(&config);
	let list_for_clear = doc_list;
	let open_button_for_clear = open_button;
	let remove_button_for_clear = remove_button;
	let clear_button_for_clear = clear_all_button;
	let open_paths_for_clear = Rc::clone(&open_paths);
	clear_all_button.on_click(move |_| {
		if list_for_clear.get_item_count() == 0 {
			return;
		}
		let confirm = MessageDialog::builder(
			&dialog,
			"Are you sure you want to remove all documents from the list? This will also remove all reading positions and bookmarks.",
			"Confirm",
		)
		.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconWarning | MessageDialogStyle::Centre)
		.build();
		if confirm.show_modal() != wxdragon::id::ID_YES {
			return;
		}
		{
			let mut cfg = config_for_clear.lock().unwrap();
			for path in cfg.get_all_documents() {
				cfg.remove_document_history(&path);
			}
			cfg.flush();
		}
		populate_document_list(
			&list_for_clear,
			&open_button_for_clear,
			&remove_button_for_clear,
			&clear_button_for_clear,
			&config_for_clear,
			open_paths_for_clear.as_ref(),
			"",
			None,
		);
	});

	let list_for_search = doc_list;
	let open_button_for_search = open_button;
	let remove_button_for_search = remove_button;
	let clear_button_for_search = clear_all_button;
	let config_for_search = Rc::clone(&config);
	let open_paths_for_search = Rc::clone(&open_paths);
	search_ctrl.on_text_updated(move |_event| {
		let filter = search_ctrl.get_value();
		populate_document_list(
			&list_for_search,
			&open_button_for_search,
			&remove_button_for_search,
			&clear_button_for_search,
			&config_for_search,
			open_paths_for_search.as_ref(),
			&filter,
			None,
		);
	});

	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&search_ctrl, dialog);
	bind_escape_to_close(&doc_list, dialog);

	let remove_action_for_keys = Rc::clone(&remove_action);
	doc_list.bind_internal(EventType::LIST_KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
				remove_action_for_keys();
				event.skip(false);
				return;
			}
			if key == KEY_ESCAPE {
				dialog.end_modal(wxdragon::id::ID_CANCEL);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});

	let ok_button = Button::builder(&dialog).with_label("OK").build();
	bind_escape_to_close(&ok_button, dialog);
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

	let result = dialog.show_modal();
	if result == wxdragon::id::ID_OK { selected_path.lock().unwrap().clone() } else { None }
}

fn populate_document_list(
	list: &ListCtrl,
	open_button: &Button,
	remove_button: &Button,
	clear_all_button: &Button,
	config: &Rc<Mutex<ConfigManager>>,
	open_paths: &[String],
	filter: &str,
	selection: Option<i32>,
) {
	list.cleanup_all_custom_data();
	list.delete_all_items();

	let items = {
		let cfg = config.lock().unwrap();
		crate::config::get_sorted_document_list(&cfg, open_paths, filter)
	};

	for item in items {
		let index = list.get_item_count() as i64;
		list.insert_item(index, &item.filename, None);
		list.set_custom_data(index as u64, item.path.clone());
		let status = match item.status {
			DocumentListStatus::Open => "Open",
			DocumentListStatus::Closed => "Closed",
			DocumentListStatus::Missing => "Missing",
		};
		list.set_item_text_by_column(index, 1, status);
		list.set_item_text_by_column(index, 2, &item.path);
	}

	if list.get_item_count() > 0 {
		let mut select_index = selection.unwrap_or(0);
		if select_index >= list.get_item_count() {
			select_index = list.get_item_count() - 1;
		}
		list.set_item_state(
			select_index as i64,
			ListItemState::Selected | ListItemState::Focused,
			ListItemState::Selected | ListItemState::Focused,
		);
		list.ensure_visible(select_index as i64);
		update_open_button_for_index(list, open_button, select_index);
		remove_button.enable(true);
		clear_all_button.enable(true);
	} else {
		open_button.enable(false);
		remove_button.enable(false);
		clear_all_button.enable(false);
	}
}

fn update_open_button_for_index(list: &ListCtrl, open_button: &Button, index: i32) {
	if index < 0 {
		open_button.enable(false);
		return;
	}
	let status = list.get_item_text(index as i64, 1);
	open_button.enable(status != "Missing");
}

fn bind_escape_to_close(handler: &impl WxEvtHandler, dialog: Dialog) {
	let dialog_for_escape = dialog;
	handler.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_ESCAPE {
				dialog_for_escape.end_modal(wxdragon::id::ID_CANCEL);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let dialog_for_escape = dialog;
	handler.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_ESCAPE {
				dialog_for_escape.end_modal(wxdragon::id::ID_CANCEL);
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

fn get_selected_index(list: &ListCtrl) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 {
		return selected;
	}
	list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused)
}

fn get_path_for_index(list: &ListCtrl, index: i32) -> Option<String> {
	if index < 0 {
		return None;
	}
	if let Some(data) = list.get_custom_data(index as u64) {
		if let Some(path) = data.as_ref().downcast_ref::<String>() {
			return Some(path.clone());
		}
	}
	let path = list.get_item_text(index as i64, 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: &ListCtrl) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}
