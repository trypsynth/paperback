use std::{
	cell::{Cell, RefCell},
	ffi::CString,
	path::Path,
	rc::Rc,
	sync::Mutex,
};

use wxdragon::{
	event::WebViewEvents,
	ffi,
	prelude::*,
	timer::Timer,
	translations::translate as t,
	uiactionsimulator::{MouseButton, UIActionSimulator},
	widgets::WebView,
};

use crate::{
	config::ConfigManager,
	document::{DocumentStats, TocItem},
	reader_core,
	session::DocumentSession,
	translation_manager::TranslationManager,
	ui_types::{BookmarkDisplayEntry, BookmarkFilterType, DocumentListStatus},
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
const WXK_END: i32 = 312;
const WXK_HOME: i32 = 313;
const WXK_LEFT: i32 = 314;
const WXK_UP: i32 = 315;
const WXK_RIGHT: i32 = 316;
const WXK_DOWN: i32 = 317;
const WXK_PAGEUP: i32 = 366;
const WXK_PAGEDOWN: i32 = 367;

#[derive(Clone, Debug)]
pub struct OptionsDialogResult {
	pub restore_previous_documents: bool,
	pub word_wrap: bool,
	pub minimize_to_tray: bool,
	pub start_maximized: bool,
	pub compact_go_menu: bool,
	pub navigation_wrap: bool,
	pub check_for_updates_on_startup: bool,
	pub recent_documents_to_show: i32,
	pub language: String,
}

pub struct BookmarkDialogResult {
	pub start: i64,
}

pub fn show_options_dialog(parent: &Frame, config: &ConfigManager) -> Option<OptionsDialogResult> {
	let dialog = Dialog::builder(parent, &t("Options")).build();
	let option_padding = 5;
	let max_recent_docs = 100;
	let general_box = StaticBoxSizerBuilder::new_with_label(Orientation::Vertical, &dialog, &t("General")).build();
	let restore_docs_check =
		CheckBox::builder(&dialog).with_label(&t("&Restore previously opened documents on startup")).build();
	let word_wrap_check = CheckBox::builder(&dialog).with_label(&t("&Word wrap")).build();
	let minimize_to_tray_check = CheckBox::builder(&dialog).with_label(&t("&Minimize to system tray")).build();
	let start_maximized_check = CheckBox::builder(&dialog).with_label(&t("&Start maximized")).build();
	let compact_go_menu_check = CheckBox::builder(&dialog).with_label(&t("Show compact &go menu")).build();
	let navigation_wrap_check = CheckBox::builder(&dialog).with_label(&t("&Wrap navigation")).build();
	let check_for_updates_check = CheckBox::builder(&dialog).with_label(&t("Check for &updates on startup")).build();
	general_box.add(&restore_docs_check, 0, SizerFlag::All, option_padding);
	general_box.add(&word_wrap_check, 0, SizerFlag::All, option_padding);
	general_box.add(&minimize_to_tray_check, 0, SizerFlag::All, option_padding);
	general_box.add(&start_maximized_check, 0, SizerFlag::All, option_padding);
	general_box.add(&compact_go_menu_check, 0, SizerFlag::All, option_padding);
	general_box.add(&navigation_wrap_check, 0, SizerFlag::All, option_padding);
	general_box.add(&check_for_updates_check, 0, SizerFlag::All, option_padding);
	let recent_docs_label = StaticText::builder(&dialog).with_label(&t("Number of &recent documents to show:")).build();
	let recent_docs_ctrl = SpinCtrl::builder(&dialog).with_range(0, max_recent_docs).build();
	let recent_docs_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	recent_docs_sizer.add(&recent_docs_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	recent_docs_sizer.add(&recent_docs_ctrl, 0, SizerFlag::AlignCenterVertical, 0);
	general_box.add_sizer(&recent_docs_sizer, 0, SizerFlag::All, option_padding);
	let language_label = StaticText::builder(&dialog).with_label(&t("&Language:")).build();
	let language_combo = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	let languages = TranslationManager::instance().lock().unwrap().available_languages();
	let mut language_codes = Vec::new();
	for lang in &languages {
		language_combo.append(&lang.native_name);
		language_codes.push(lang.code.clone());
	}
	let language_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	language_sizer.add(&language_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	language_sizer.add(&language_combo, 0, SizerFlag::AlignCenterVertical, 0);
	general_box.add_sizer(&language_sizer, 0, SizerFlag::All, option_padding);
	restore_docs_check.set_value(config.get_app_bool("restore_previous_documents", true));
	word_wrap_check.set_value(config.get_app_bool("word_wrap", false));
	minimize_to_tray_check.set_value(config.get_app_bool("minimize_to_tray", false));
	start_maximized_check.set_value(config.get_app_bool("start_maximized", false));
	compact_go_menu_check.set_value(config.get_app_bool("compact_go_menu", true));
	navigation_wrap_check.set_value(config.get_app_bool("navigation_wrap", false));
	check_for_updates_check.set_value(config.get_app_bool("check_for_updates_on_startup", true));
	recent_docs_ctrl.set_value(config.get_app_int("recent_documents_to_show", 25).clamp(0, max_recent_docs));
	let stored_language = config.get_app_string("language", "");
	let current_language = if stored_language.is_empty() {
		TranslationManager::instance().lock().unwrap().current_language()
	} else {
		stored_language
	};
	if let Some(index) = language_codes.iter().position(|code| code == &current_language) {
		language_combo.set_selection(index as u32);
	}
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	ok_button.set_default();
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&general_box, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let language = language_combo
		.get_selection()
		.and_then(|index| language_codes.get(index as usize).cloned())
		.unwrap_or_else(|| current_language.clone());
	Some(OptionsDialogResult {
		restore_previous_documents: restore_docs_check.is_checked(),
		word_wrap: word_wrap_check.is_checked(),
		minimize_to_tray: minimize_to_tray_check.is_checked(),
		start_maximized: start_maximized_check.is_checked(),
		compact_go_menu: compact_go_menu_check.is_checked(),
		navigation_wrap: navigation_wrap_check.is_checked(),
		check_for_updates_on_startup: check_for_updates_check.is_checked(),
		recent_documents_to_show: recent_docs_ctrl.value(),
		language,
	})
}

pub fn show_bookmark_dialog(
	parent: &Frame,
	session: &DocumentSession,
	config: Rc<Mutex<ConfigManager>>,
	current_pos: i64,
	initial_filter: BookmarkFilterType,
) -> Option<BookmarkDialogResult> {
	let file_path = session.file_path().to_string();
	let content = Rc::new(session.content());
	let dialog = Dialog::builder(parent, &t("Jump to Bookmark")).build();
	let filter_label = StaticText::builder(&dialog).with_label(&t("&Filter:")).build();
	let filter_choice = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	filter_choice.append(&t("All"));
	filter_choice.append(&t("Bookmarks"));
	filter_choice.append(&t("Notes"));
	let initial_index = match initial_filter {
		BookmarkFilterType::BookmarksOnly => 1,
		BookmarkFilterType::NotesOnly => 2,
		BookmarkFilterType::All => 0,
	};
	filter_choice.set_selection(initial_index);
	let filter_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	filter_sizer.add(&filter_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 6);
	filter_sizer.add(&filter_choice, 1, SizerFlag::Expand, 0);
	let bookmark_list = ListBox::builder(&dialog).build();
	let edit_button = Button::builder(&dialog).with_label(&t("&Edit Note")).build();
	let delete_button = Button::builder(&dialog).with_label(&t("&Delete")).build();
	let jump_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("&Jump")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("&Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	jump_button.set_default();
	let entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>> = Rc::new(RefCell::new(Vec::new()));
	let selected_start = Rc::new(Cell::new(-1i64));
	let selected_end = Rc::new(Cell::new(-1i64));
	let jump_button_for_state = jump_button;
	let delete_button_for_state = delete_button;
	let edit_button_for_state = edit_button;
	let set_buttons_enabled = move |enabled: bool| {
		jump_button_for_state.enable(enabled);
		delete_button_for_state.enable(enabled);
		edit_button_for_state.enable(enabled);
	};
	set_buttons_enabled(false);
	let list_for_repopulate = bookmark_list;
	let config_for_repopulate = Rc::clone(&config);
	let file_path_for_repopulate = file_path.clone();
	let content_for_repopulate = Rc::clone(&content);
	let entries_for_repopulate = Rc::clone(&entries);
	let selected_start_for_repopulate = Rc::clone(&selected_start);
	let selected_end_for_repopulate = Rc::clone(&selected_end);
	let filter_choice_for_repopulate = filter_choice;
	let set_buttons_enabled = Rc::new(set_buttons_enabled);
	let set_buttons_for_repopulate = Rc::clone(&set_buttons_enabled);
	let repopulate = Rc::new(move |pos: i64| {
		let filter_index = filter_choice_for_repopulate.get_selection().unwrap_or(0);
		let filter = match filter_index {
			1 => BookmarkFilterType::BookmarksOnly,
			2 => BookmarkFilterType::NotesOnly,
			_ => BookmarkFilterType::All,
		};
		let content_for_snippet = Rc::clone(&content_for_repopulate);
		let get_text_range = move |start: i64, end: i64| -> String {
			let content = content_for_snippet.as_str();
			let total_chars = content.chars().count();
			let start_pos = usize::try_from(start.max(0)).unwrap_or(0).min(total_chars);
			let end_pos = usize::try_from(end.max(0)).unwrap_or(0).min(total_chars);
			if start_pos >= end_pos {
				return String::new();
			}
			content.chars().skip(start_pos).take(end_pos - start_pos).collect()
		};
		let content_for_line = Rc::clone(&content_for_repopulate);
		let get_line_text = move |position: i64| -> String {
			let content = content_for_line.as_str();
			let total_chars = content.chars().count();
			let pos = usize::try_from(position.max(0)).unwrap_or(0).min(total_chars);
			let line_start =
				content.chars().take(pos).collect::<Vec<_>>().iter().rposition(|&c| c == '\n').map_or(0, |idx| idx + 1);
			let chars_after_start: String = content.chars().skip(line_start).collect();
			let line_end = chars_after_start.find('\n').map_or(chars_after_start.len(), |idx| idx);
			chars_after_start.chars().take(line_end).collect()
		};
		let previous_selected = selected_start_for_repopulate.get();
		list_for_repopulate.clear();
		entries_for_repopulate.borrow_mut().clear();
		let filtered = {
			let cfg = config_for_repopulate.lock().unwrap();
			reader_core::get_filtered_bookmarks(&cfg, &file_path_for_repopulate, pos, filter)
		};
		for item in filtered.items {
			let snippet =
				if item.is_whole_line { get_line_text(item.start) } else { get_text_range(item.start, item.end) };
			let mut snippet = snippet.trim().to_string();
			if snippet.is_empty() {
				snippet = t("blank");
			}
			let display = if item.note.is_empty() { snippet.clone() } else { format!("{} - {}", item.note, snippet) };
			entries_for_repopulate.borrow_mut().push(BookmarkDisplayEntry { start: item.start, end: item.end });
			list_for_repopulate.append(&display);
		}
		selected_start_for_repopulate.set(-1);
		selected_end_for_repopulate.set(-1);
		set_buttons_for_repopulate(false);
		let entries_ref = entries_for_repopulate.borrow();
		if previous_selected >= 0 {
			if let Some((idx, entry)) =
				entries_ref.iter().enumerate().find(|(_, entry)| entry.start == previous_selected)
			{
				list_for_repopulate.set_selection(idx as u32, true);
				selected_start_for_repopulate.set(entry.start);
				selected_end_for_repopulate.set(entry.end);
				set_buttons_for_repopulate(true);
				return;
			}
		}
		if filtered.closest_index >= 0 {
			let idx = filtered.closest_index as usize;
			if let Some(entry) = entries_ref.get(idx) {
				list_for_repopulate.set_selection(idx as u32, true);
				selected_start_for_repopulate.set(entry.start);
				selected_end_for_repopulate.set(entry.end);
				set_buttons_for_repopulate(true);
			}
		}
	});
	repopulate(current_pos);
	let entries_for_selection = Rc::clone(&entries);
	let selected_start_for_selection = Rc::clone(&selected_start);
	let selected_end_for_selection = Rc::clone(&selected_end);
	let set_buttons_for_selection = Rc::clone(&set_buttons_enabled);
	bookmark_list.on_selection_changed(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			let entries_ref = entries_for_selection.borrow();
			if let Some(entry) = entries_ref.get(selection as usize) {
				selected_start_for_selection.set(entry.start);
				selected_end_for_selection.set(entry.end);
				set_buttons_for_selection(true);
				return;
			}
		}
		selected_start_for_selection.set(-1);
		selected_end_for_selection.set(-1);
		set_buttons_for_selection(false);
	});
	let dialog_for_jump = dialog;
	let selected_start_for_jump = Rc::clone(&selected_start);
	jump_button.on_click(move |_| {
		if selected_start_for_jump.get() >= 0 {
			dialog_for_jump.end_modal(wxdragon::id::ID_OK);
		} else {
			MessageDialog::builder(&dialog_for_jump, &t("Please select a bookmark to jump to."), &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
				.build()
				.show_modal();
		}
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let repopulate_for_filter = Rc::clone(&repopulate);
	filter_choice.on_selection_changed(move |_event| {
		repopulate_for_filter(current_pos);
	});
	let repopulate_for_delete = Rc::clone(&repopulate);
	let selected_start_for_delete = Rc::clone(&selected_start);
	let selected_end_for_delete = Rc::clone(&selected_end);
	let config_for_delete = Rc::clone(&config);
	let file_path_for_delete = file_path.clone();
	delete_button.on_click(move |_| {
		let start = selected_start_for_delete.get();
		let end = selected_end_for_delete.get();
		if start < 0 {
			return;
		}
		{
			let cfg = config_for_delete.lock().unwrap();
			cfg.remove_bookmark(&file_path_for_delete, start, end);
			cfg.flush();
		}
		repopulate_for_delete(current_pos);
	});
	let repopulate_for_edit = Rc::clone(&repopulate);
	let selected_start_for_edit = Rc::clone(&selected_start);
	let selected_end_for_edit = Rc::clone(&selected_end);
	let config_for_edit = Rc::clone(&config);
	let file_path_for_edit = file_path.clone();
	edit_button.on_click(move |_| {
		let start = selected_start_for_edit.get();
		let end = selected_end_for_edit.get();
		if start < 0 {
			return;
		}
		let existing_note = {
			let cfg = config_for_edit.lock().unwrap();
			cfg.get_bookmarks(&file_path_for_edit)
				.into_iter()
				.find(|bm| bm.start == start && bm.end == end)
				.map(|bm| bm.note)
				.unwrap_or_default()
		};
		let Some(note) =
			show_note_entry_dialog(&dialog, &t("Bookmark Note"), &t("Edit bookmark note:"), &existing_note)
		else {
			return;
		};
		{
			let cfg = config_for_edit.lock().unwrap();
			cfg.update_bookmark_note(&file_path_for_edit, start, end, &note);
			cfg.flush();
		}
		repopulate_for_edit(current_pos);
	});
	let repopulate_for_key = Rc::clone(&repopulate);
	let selected_start_for_key = Rc::clone(&selected_start);
	let selected_end_for_key = Rc::clone(&selected_end);
	let config_for_key = Rc::clone(&config);
	let file_path_for_key = file_path;
	bookmark_list.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
			let start = selected_start_for_key.get();
			let end = selected_end_for_key.get();
			if start >= 0 {
				{
					let cfg = config_for_key.lock().unwrap();
					cfg.remove_bookmark(&file_path_for_key, start, end);
					cfg.flush();
				}
				repopulate_for_key(current_pos);
			}
			event.skip(false);
			return;
		}
		event.skip(true);
	});
	let selected_start_for_double = Rc::clone(&selected_start);
	let dialog_for_double = dialog;
	bookmark_list.on_item_double_clicked(move |_| {
		if selected_start_for_double.get() >= 0 {
			dialog_for_double.end_modal(wxdragon::id::ID_OK);
		}
	});
	let action_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	action_sizer.add(&edit_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&delete_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&jump_button, 0, SizerFlag::Right, DIALOG_PADDING);
	action_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&filter_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&bookmark_list,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	content_sizer.add_sizer(&action_sizer, 0, SizerFlag::AlignRight | SizerFlag::All, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	bookmark_list.set_focus();
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let start = selected_start.get();
	if start >= 0 { Some(BookmarkDialogResult { start }) } else { None }
}

pub fn show_note_entry_dialog(
	parent: &dyn WxWidget,
	title: &str,
	message: &str,
	existing_note: &str,
) -> Option<String> {
	let dialog = Dialog::builder(parent, title).build();
	let message_label = StaticText::builder(&dialog).with_label(message).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(existing_note)
		.with_style(TextCtrlStyle::MultiLine)
		.with_size(Size::new(400, 200))
		.build();
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_cancel = dialog;
	cancel_button.on_click(move |_| {
		dialog_for_cancel.end_modal(wxdragon::id::ID_CANCEL);
	});
	let dialog_for_key = dialog;
	note_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_RETURN {
				if event.shift_down() {
					event.skip(true);
				} else {
					dialog_for_key.end_modal(wxdragon::id::ID_OK);
					event.skip(false);
				}
				return;
			}
		}
		event.skip(true);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&message_label, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&note_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(note_ctrl.get_value()) } else { None }
}

pub fn show_view_note_dialog(parent: &dyn WxWidget, note_text: &str) {
	let dialog = Dialog::builder(parent, &t("View Note")).build();
	let note_ctrl = TextCtrl::builder(&dialog)
		.with_value(note_text)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly | TextCtrlStyle::Rich2)
		.with_size(Size::new(400, 200))
		.build();
	let close_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("Close")).build();
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let dialog_for_close = dialog;
	close_button.on_click(move |_| {
		dialog_for_close.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&note_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	note_ctrl.set_focus();
	dialog.show_modal();
}

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
			if s.ends_with(c.to_ascii_lowercase()) {
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
	info.push_str(&format!("{path_label} {}\n", path.display()));
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

pub fn show_go_to_line_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog_title = t("Go to Line");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let label_text = t("&Line number:");
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let status = session.get_status_info(current_pos);
	let total_lines = session.line_count().max(1);
	let max_lines = total_lines.min(i64::from(i32::MAX)) as i32;
	let current_line = status.line_number.clamp(1, total_lines).min(i64::from(i32::MAX)) as i32;
	let line_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_lines)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	line_ctrl.set_value(current_line);
	let dialog_for_enter = dialog;
	line_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let line_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	line_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	line_sizer.add(&line_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&line_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	line_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(i64::from(line_ctrl.value())) } else { None }
}

pub fn show_go_to_page_dialog(parent: &Frame, session: &DocumentSession, current_page: i32) -> Option<i64> {
	let max_page = session.page_count().max(1) as i32;
	let dialog_title = t("Go to page");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let label_template = t("Go to page (%d/%d):");
	let label_text = label_template.replacen("%d", &current_page.clamp(1, max_page).to_string(), 1).replacen(
		"%d",
		&max_page.to_string(),
		1,
	);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let current = current_page.clamp(1, max_page);
	let page_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_page)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	page_ctrl.set_value(current);
	let dialog_for_enter = dialog;
	page_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let label_for_update = label;
	let label_template_for_update = label_template;
	page_ctrl.on_value_changed(move |event| {
		let text = label_template_for_update.replacen("%d", &event.get_value().to_string(), 1).replacen(
			"%d",
			&max_page.to_string(),
			1,
		);
		label_for_update.set_label(&text);
	});
	let page_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	page_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	page_sizer.add(&page_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&page_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	page_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let page = page_ctrl.value().clamp(1, max_page);
		Some(session.page_offset(page))
	} else {
		None
	}
}

pub fn show_go_to_percent_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog_title = t("Go to Percent");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let status = session.get_status_info(current_pos);
	let current_percent = status.percentage.clamp(0, 100);
	let slider_label = StaticText::builder(&dialog).with_label(&t("&Percent")).build();
	let percent_slider =
		Slider::builder(&dialog).with_value(current_percent).with_min_value(0).with_max_value(100).build();
	let input_label = StaticText::builder(&dialog).with_label(&t("P&ercent:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(0, 100)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(current_percent);
	let input_ctrl_for_slider = input_ctrl;
	percent_slider.on_slider(move |event| {
		input_ctrl_for_slider.set_value(event.get_value());
	});
	let percent_slider_for_spin = percent_slider;
	input_ctrl.on_value_changed(move |event| {
		percent_slider_for_spin.set_value(event.get_value());
	});
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let dialog_for_slider_enter = dialog;
	percent_slider.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
			event.skip(false);
			dialog_for_slider_enter.end_modal(wxdragon::id::ID_OK);
			return;
		}
		event.skip(true);
	});
	let percent_slider_for_keys = percent_slider;
	let input_ctrl_for_keys = input_ctrl;
	percent_slider.bind_internal(EventType::CHAR, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		let current = percent_slider_for_keys.value();
		let min_val = percent_slider_for_keys.min();
		let max_val = percent_slider_for_keys.max();
		let new_value = match key {
			WXK_UP | WXK_RIGHT => Some((current + 1).min(max_val)),
			WXK_DOWN | WXK_LEFT => Some((current - 1).max(min_val)),
			WXK_PAGEUP => Some((current + 10).min(max_val)),
			WXK_PAGEDOWN => Some((current - 10).max(min_val)),
			WXK_HOME => Some(min_val),
			WXK_END => Some(max_val),
			_ => None,
		};
		if let Some(val) = new_value {
			percent_slider_for_keys.set_value(val);
			input_ctrl_for_keys.set_value(val);
			event.skip(false);
		} else {
			event.skip(true);
		}
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&slider_label, 0, SizerFlag::Left | SizerFlag::Top, 5);
	content_sizer.add(&percent_slider, 0, SizerFlag::Expand | SizerFlag::Bottom, 5);
	content_sizer.add(&input_label, 0, SizerFlag::Left, 5);
	content_sizer.add(&input_ctrl, 0, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let main_sizer = BoxSizer::builder(Orientation::Vertical).build();
	main_sizer.add_sizer(&content_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(main_sizer, true);
	dialog.centre();
	percent_slider.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let percent = input_ctrl.value().clamp(0, 100);
		Some(session.position_from_percent(percent))
	} else {
		None
	}
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
				i64::from(index),
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
			let path = list_for_activate.get_item_text(i64::from(index), 2);
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
			let cfg = config_for_remove.lock().unwrap();
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
			let cfg = config_for_clear.lock().unwrap();
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
		let index = i64::from(list.get_item_count());
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

fn update_open_button_for_index(list: &ListCtrl, open_button: &Button, index: i32) {
	if index < 0 {
		open_button.enable(false);
		return;
	}
	let status = list.get_item_text(i64::from(index), 1);
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
	let path = list.get_item_text(i64::from(index), 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: &ListCtrl) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}

pub fn show_sleep_timer_dialog(parent: &Frame, initial_duration: i32) -> Option<i32> {
	let dialog = Dialog::builder(parent, &t("Sleep Timer")).build();
	let label = StaticText::builder(&dialog).with_label(&t("&Minutes:")).build();
	let input_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, 999)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	input_ctrl.set_value(initial_duration.clamp(1, 999));
	let dialog_for_enter = dialog;
	input_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	input_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	input_sizer.add(&input_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	input_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(input_ctrl.value()) } else { None }
}

pub fn show_web_view_dialog(
	parent: &Frame,
	title: &str,
	url_or_content: &str,
	is_url: bool,
	navigation_handler: Option<Box<dyn Fn(&str) -> bool>>,
) {
	let dialog = Dialog::builder(parent, title).build();
	let web_view = WebView::builder(&dialog).build();
	web_view.add_script_message_handler("wx");
	let dialog_for_close = dialog;
	web_view.on_script_message_received(move |event| {
		if event.get_string() == Some("close_dialog".to_string()) {
			dialog_for_close.end_modal(wxdragon::id::ID_CANCEL);
		}
	});
	if let Some(handler) = navigation_handler {
		web_view.on_navigating(move |event| {
			if let Some(url) = event.get_string() {
				let url_str: String = url;
				if !handler(&url_str) {
					event.event.event.veto();
				}
			}
		});
	}
	if is_url {
		web_view.load_url(url_or_content);
	} else {
		let full_html = if url_or_content.to_lowercase().contains("<html") {
			url_or_content.to_string()
		} else {
			format!("<html><head><title>{title}</title></head><body>{url_or_content}</body></html>")
		};
		web_view.set_page(&full_html, "");
	}
	let web_view_for_load = web_view;
	let timer = Rc::new(Timer::new(&dialog));
	let timer_copy = Rc::clone(&timer);
	web_view.on_loaded(move |_| {
		let web_view_for_timer = web_view_for_load;
		timer_copy.on_tick(move |_| {
			let pos = web_view_for_timer.client_to_screen(Point::new(0, 0));
			let size = web_view_for_timer.get_size();
			let x = pos.x + size.width / 2;
			let y = pos.y + size.height / 2;
			let sim = UIActionSimulator::new();
			sim.mouse_move(x, y);
			sim.mouse_click(MouseButton::Left);
		});
		timer_copy.start(100, true);
		web_view_for_load.run_script(
			"document.addEventListener('keydown', function(event) { \
             if (event.key === 'Escape' || event.keyCode === 27) { \
             window.wx.postMessage('close_dialog'); \
             } \
             });",
		);
	});
	let close_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Close")).build();
	let dialog_for_ok = dialog;
	close_button.on_click(move |_| {
		dialog_for_ok.end_modal(wxdragon::id::ID_OK);
	});
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	let sizer = BoxSizer::builder(Orientation::Vertical).build();
	sizer.add(&web_view, 1, SizerFlag::Expand | SizerFlag::All, 5);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&close_button, 0, SizerFlag::All, 5);
	sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(sizer, true);
	dialog.centre();
	dialog.show_modal();
}

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	let choice_label = StaticText::builder(&dialog).with_label(&t("&View:")).build();
	let view_choice = ComboBox::builder(&dialog).with_style(ComboBoxStyle::ReadOnly).build();
	view_choice.append(&t("Headings"));
	view_choice.append(&t("Links"));
	view_choice.set_selection(0);
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let headings_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let headings_tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	headings_sizer.add(&headings_tree, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&headings_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	let links_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let links_list = ListBox::builder(&dialog).build();
	links_sizer.add(&links_list, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&links_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	links_list.show(false);
	let selected_offset = Rc::new(Cell::new(-1i64));
	let root = headings_tree.add_root(&t("Root"), None, None).unwrap();
	let tree_data = session.heading_tree(current_pos);
	let mut item_ids: Vec<wxdragon::widgets::treectrl::TreeItemId> = Vec::new();
	if !tree_data.items.is_empty() {
		item_ids.reserve(tree_data.items.len());
	}
	for item in &tree_data.items {
		let parent_id = if item.parent_index >= 0 {
			if let Some(id) = item_ids.get(item.parent_index as usize) { id.clone() } else { root.clone() }
		} else {
			root.clone()
		};
		let display_text = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		if let Some(id) = headings_tree.append_item_with_data(&parent_id, &display_text, item.offset as i64, None, None)
		{
			item_ids.push(id);
		} else if let Some(root_child) =
			headings_tree.append_item_with_data(&root, &display_text, item.offset as i64, None, None)
		{
			item_ids.push(root_child);
		}
	}
	headings_tree.expand_all();
	if tree_data.closest_index >= 0 {
		if let Some(item) = item_ids.get(tree_data.closest_index as usize) {
			headings_tree.select_item(item);
			headings_tree.ensure_visible(item);
		}
	} else if let Some((first_child, _)) = headings_tree.get_first_child(&root) {
		headings_tree.select_item(&first_child);
		headings_tree.ensure_visible(&first_child);
	}
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in link_data.items {
		links_list.append(&item.text);
		link_offsets.push(item.offset as i64);
	}
	if !link_offsets.is_empty() {
		let idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		links_list.set_selection(idx as u32, true);
	}
	let link_offsets = Rc::new(link_offsets);
	let headings_tree_for_choice = headings_tree;
	let links_list_for_choice = links_list;
	let dialog_for_layout = dialog;
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			headings_tree_for_choice.show(true);
			links_list_for_choice.show(false);
			headings_tree_for_choice.set_focus();
		} else {
			headings_tree_for_choice.show(false);
			links_list_for_choice.show(true);
			links_list_for_choice.set_focus();
		}
		dialog_for_layout.layout();
	});
	let selected_offset_for_tree = Rc::clone(&selected_offset);
	let tree_for_activate = headings_tree;
	let dialog_for_tree = dialog;
	headings_tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i64>() {
					selected_offset_for_tree.set(*offset);
					dialog_for_tree.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
	let selected_offset_for_list = Rc::clone(&selected_offset);
	let offsets_for_list = Rc::clone(&link_offsets);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			if let Some(offset) = offsets_for_list.get(selection as usize) {
				selected_offset_for_list.set(*offset);
				dialog_for_list.end_modal(wxdragon::id::ID_OK);
			}
		}
	});
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	let selected_offset_for_ok = Rc::clone(&selected_offset);
	let view_choice_for_ok = view_choice;
	let headings_tree_for_ok = headings_tree;
	let links_list_for_ok = links_list;
	let offsets_for_ok = Rc::clone(&link_offsets);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		let selection = view_choice_for_ok.get_selection().unwrap_or(0);
		if selection == 0 {
			if let Some(item) = headings_tree_for_ok.get_selection() {
				if let Some(data) = headings_tree_for_ok.get_custom_data(&item) {
					if let Some(offset) = data.downcast_ref::<i64>() {
						selected_offset_for_ok.set(*offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(idx) = links_list_for_ok.get_selection() {
			if let Some(offset) = offsets_for_ok.get(idx as usize) {
				selected_offset_for_ok.set(*offset);
				dialog_for_ok.end_modal(wxdragon::id::ID_OK);
			}
		}
	});
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	if view_choice.get_selection().unwrap_or(0) == 0 {
		headings_tree.set_focus();
	} else {
		links_list.set_focus();
	}
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

pub fn show_about_dialog(parent: &Frame) {
	let name = CString::new("Paperback").unwrap_or_else(|_| CString::new("").unwrap());
	let version = CString::new(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| CString::new("").unwrap());
	let description = CString::new(t("An accessible, lightweight, fast ebook and document reader"))
		.unwrap_or_else(|_| CString::new("").unwrap());
	let copyright = CString::new("Copyright (C) 2025-2026 Quin Gillespie. All rights reserved.")
		.unwrap_or_else(|_| CString::new("").unwrap());
	let website = CString::new("https://paperback.dev").unwrap_or_else(|_| CString::new("").unwrap());
	unsafe {
		let info = ffi::wxd_AboutDialogInfo_Create();
		if info.is_null() {
			return;
		}
		ffi::wxd_AboutDialogInfo_SetName(info, name.as_ptr());
		ffi::wxd_AboutDialogInfo_SetVersion(info, version.as_ptr());
		ffi::wxd_AboutDialogInfo_SetDescription(info, description.as_ptr());
		ffi::wxd_AboutDialogInfo_SetCopyright(info, copyright.as_ptr());
		ffi::wxd_AboutDialogInfo_SetWebSite(info, website.as_ptr());
		ffi::wxd_AboutBox(info, parent.handle_ptr());
		ffi::wxd_AboutDialogInfo_Destroy(info);
	}
}
