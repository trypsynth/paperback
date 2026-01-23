use std::{
	cell::{Cell, RefCell},
	path::Path,
	rc::Rc,
	sync::Mutex,
};

use wxdragon::{prelude::*, timer::Timer, translations::translate as t};

use crate::{
	config::ConfigManager,
	document::{DocumentStats, TocItem},
	ui_types::DocumentListStatus,
};

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
const KEY_SPACE: i32 = 32;
const KEY_ESCAPE: i32 = 27;
const KEY_RETURN: i32 = 13;
const KEY_NUMPAD_ENTER: i32 = 370;

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1));

	let tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();

	let root = tree.add_root(&t("Root"), None, None).unwrap();
	populate_toc_tree(&tree, &root, toc_items);

	if current_offset != -1 {
		find_and_select_item(&tree, &root, current_offset);
	}

	let search_string = Rc::new(RefCell::new(String::new()));
	let search_timer = Rc::new(Timer::new(&dialog));
	let search_string_for_timer = Rc::clone(&search_string);
	search_timer.on_tick(move |_| {
		search_string_for_timer.borrow_mut().clear();
	});

	let tree_for_sel = tree;
	let selected_offset_for_sel = Rc::clone(&selected_offset);
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_sel.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset_for_sel.set(*offset);
				}
			}
		}
	});

	let dialog_for_activate = dialog;
	let tree_for_activate = tree;
	let selected_offset_for_activate = Rc::clone(&selected_offset);
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset_for_activate.set(*offset);
					dialog_for_activate.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});

	let tree_for_search_keydown = tree;
	let search_string_for_search_keydown = Rc::clone(&search_string);
	let search_timer_for_search_keydown = Rc::clone(&search_timer);
	tree.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_SPACE {
				let mut s = search_string_for_search_keydown.borrow_mut();
				if !s.is_empty() {
					let mut new_search = s.clone();
					new_search.push(' ');
					if let Some(root) = tree_for_search_keydown.get_root_item() {
						if find_and_select_item_by_name(&tree_for_search_keydown, &root, &new_search) {
							*s = new_search;
							search_timer_for_search_keydown.start(500, true);
							event.skip(false);
						} else {
							bell();
							event.skip(false);
						}
					} else {
						event.skip(false);
					}
					return;
				}
			}
		}
		event.skip(true);
	});

	let tree_for_search = tree;
	let search_string_for_search = Rc::clone(&search_string);
	let search_timer_for_search = Rc::clone(&search_timer);
	tree.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_unicode_key() {
			if key <= KEY_SPACE || key == KEY_DELETE {
				event.skip(true);
				return;
			}
			let c = std::char::from_u32(key as u32).unwrap_or('\0');
			let mut s = search_string_for_search.borrow_mut();

			if s.is_empty() {
				s.push(c.to_ascii_lowercase());
				search_timer_for_search.start(500, true);
				event.skip(true); // First char, let native handle it too (cycle to first A)
				return;
			}

			if s.chars().last() == Some(c.to_ascii_lowercase()) {
				search_timer_for_search.start(500, true);
				event.skip(true); // Let native handle cycling
				return;
			}

			let mut new_search = s.clone();
			new_search.push(c.to_ascii_lowercase());
			if let Some(root) = tree_for_search.get_root_item() {
				if find_and_select_item_by_name(&tree_for_search, &root, &new_search) {
					*s = new_search;
					search_timer_for_search.start(500, true);
					event.skip(false);
				} else {
					bell();
					event.skip(false);
				}
			} else {
				event.skip(false);
			}
		} else {
			event.skip(true);
		}
	});

	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);

	let dialog_for_ok = dialog;
	let selected_offset_for_ok = Rc::clone(&selected_offset);
	ok_button.on_click(move |_| {
		if selected_offset_for_ok.get() >= 0 {
			dialog_for_ok.end_modal(wxdragon::id::ID_OK);
		} else {
			MessageDialog::builder(
				&dialog_for_ok,
				&t("Please select a section from the table of contents."),
				&t("No Selection"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build()
			.show_modal();
		}
	});

	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);

	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	tree.set_focus();

	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

fn populate_toc_tree(tree: &TreeCtrl, parent: &TreeItemId, items: &[TocItem]) {
	for item in items {
		let display_text = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		if let Some(id) = tree.append_item_with_data(parent, &display_text, item.offset as i32, None, None) {
			if !item.children.is_empty() {
				populate_toc_tree(tree, &id, &item.children);
			}
		}
	}
}

fn find_and_select_item(tree: &TreeCtrl, parent: &TreeItemId, offset: i32) -> bool {
	if let Some((child, mut cookie)) = tree.get_first_child(parent) {
		let mut current_child = Some(child);
		while let Some(item) = current_child {
			if let Some(data) = tree.get_custom_data(&item) {
				if let Some(item_offset) = data.downcast_ref::<i32>() {
					if *item_offset == offset {
						tree.select_item(&item);
						tree.set_focused_item(&item);
						tree.ensure_visible(&item);
						return true;
					}
				}
			}
			if find_and_select_item(tree, &item, offset) {
				return true;
			}
			current_child = tree.get_next_child(parent, &mut cookie);
		}
	}
	false
}

fn find_and_select_item_by_name(tree: &TreeCtrl, parent: &TreeItemId, name: &str) -> bool {
	if let Some((child, mut cookie)) = tree.get_first_child(parent) {
		let mut current_child = Some(child);
		while let Some(item) = current_child {
			if let Some(text) = tree.get_item_text(&item) {
				if text.to_lowercase().starts_with(name) {
					tree.select_item(&item);
					tree.set_focused_item(&item);
					tree.ensure_visible(&item);
					return true;
				}
			}
			if find_and_select_item_by_name(tree, &item, name) {
				return true;
			}
			current_child = tree.get_next_child(parent, &mut cookie);
		}
	}
	false
}

pub fn show_document_info_dialog(parent: &Frame, path: &Path, title: &str, author: &str, stats: &DocumentStats) {
	let dialog_title = t("Document Info");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let info_ctrl = TextCtrl::builder(&dialog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
		.with_size(Size::new(DOC_INFO_WIDTH, DOC_INFO_HEIGHT))
		.build();
	let path_label = t("Path:");
	let title_label = t("Title:");
	let author_label = t("Author:");
	let words_label = t("Words:");
	let lines_label = t("Lines:");
	let characters_label = t("Characters:");
	let characters_no_spaces_label = t("Characters (excluding spaces):");
	let mut info = String::new();
	info.push_str(&format!("{path_label} {}\n\n", path.display()));
	if !title.is_empty() {
		info.push_str(&format!("{title_label} {title}\n"));
	}
	if !author.is_empty() {
		info.push_str(&format!("{author_label} {author}\n"));
	}
	info.push_str(&format!("{words_label} {}\n", stats.word_count));
	info.push_str(&format!("{lines_label} {}\n", stats.line_count));
	info.push_str(&format!("{characters_label} {}\n", stats.char_count));
	info.push_str(&format!("{characters_no_spaces_label} {}\n", stats.char_count_no_whitespace));
	info_ctrl.set_value(&info);
	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&info_ctrl, dialog);
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
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

pub fn show_update_dialog(parent: &dyn WxWidget, new_version: &str, changelog: &str) -> bool {
	let dialog_title = t("Update to %s").replace("%s", new_version);
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let message = StaticText::builder(&dialog)
		.with_label(&t("A new version of Paperback is available. Here's what's new:"))
		.build();
	let changelog_ctrl = TextCtrl::builder(&dialog)
		.with_value(changelog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(Size::new(500, 300))
		.build();
	let yes_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("&Yes")).build();
	let no_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&No")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&changelog_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&yes_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&no_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	changelog_ctrl.set_focus();
	dialog.show_modal() == wxdragon::id::ID_OK
}

pub fn show_all_documents_dialog(
	parent: &Frame,
	config: Rc<Mutex<ConfigManager>>,
	open_paths: Vec<String>,
) -> Option<String> {
	let open_paths = Rc::new(open_paths);
	let dialog_title = t("All Documents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_path = Rc::new(Mutex::new(None));
	let search_label_text = t("&search");
	let search_label = StaticText::builder(&dialog).with_label(&search_label_text).build();
	let search_ctrl = TextCtrl::builder(&dialog).with_size(Size::new(300, -1)).build();
	let doc_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::SingleSel)
		.with_size(Size::new(RECENT_DOCS_LIST_WIDTH, RECENT_DOCS_LIST_HEIGHT))
		.build();
	let file_name_label = t("File Name");
	let status_label = t("Status");
	let path_label = t("Path");
	doc_list.insert_column(0, &file_name_label, ListColumnFormat::Left, RECENT_DOCS_FILENAME_WIDTH);
	doc_list.insert_column(1, &status_label, ListColumnFormat::Left, RECENT_DOCS_STATUS_WIDTH);
	doc_list.insert_column(2, &path_label, ListColumnFormat::Left, RECENT_DOCS_PATH_WIDTH);
	let open_label = t("&Open");
	let remove_label = t("&Remove");
	let clear_all_label = t("&Clear All");
	let open_button = Button::builder(&dialog).with_label(&open_label).build();
	let remove_button = Button::builder(&dialog).with_label(&remove_label).build();
	let clear_all_button = Button::builder(&dialog).with_label(&clear_all_label).build();
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
	let open_action = Rc::new(move || {
		if let Some(path) = get_selected_path(&list_for_open) {
			if Path::new(&path).exists() {
				*selected_for_open.lock().unwrap() = Some(path);
				dialog_for_open.end_modal(wxdragon::id::ID_OK);
			}
		}
	});
	let open_action_for_button = Rc::clone(&open_action);
	open_button.on_click(move |_| {
		open_action_for_button();
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
			&t(
				"Are you sure you want to remove this document from the list? This will also remove its reading position.",
			),
			&t("Confirm"),
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
			&t("Are you sure you want to remove all documents from the list? This will also remove all reading positions and bookmarks."),
			&t("Confirm"),
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
	let open_action_for_keys = Rc::clone(&open_action);
	doc_list.bind_internal(EventType::KEY_DOWN, move |event| {
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
		}
		event.skip(true);
	});
	let open_action_for_char = Rc::clone(&open_action);
	doc_list.bind_internal(EventType::CHAR, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
				open_action_for_char();
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
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

pub fn show_open_as_dialog(parent: &Frame, path: &Path) -> Option<String> {
	let title = t("Open As");
	let dialog = Dialog::builder(parent, &title).build();
	let message_template = t("No suitable parser was found for {}.\nHow would you like to open this file?");
	let message = message_template.replace("{}", &path.display().to_string());
	let label = StaticText::builder(&dialog).with_label(&message).build();
	let format_label_text = t("Open &as:");
	let format_label = StaticText::builder(&dialog).with_label(&format_label_text).build();
	let format_combo = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	format_combo.append(&t("Plain Text"));
	format_combo.append(&t("HTML"));
	format_combo.append(&t("Markdown"));
	format_combo.set_selection(0);
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
	let cancel_label = t("Cancel");
	let cancel_button = Button::builder(&dialog).with_label(&cancel_label).build();
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&label, 0, SizerFlag::All, DIALOG_PADDING / 2);
	let format_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	format_sizer.add(&format_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	format_sizer.add(&format_combo, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&format_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	format_combo.set_focus();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let selection = format_combo.get_selection();
	let format = match selection {
		Some(1) => "html",
		Some(2) => "md",
		_ => "txt",
	};
	Some(format.to_string())
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
	open_button.enable(status != t("Missing"));
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
