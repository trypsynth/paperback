use std::{
	cell::{Cell, RefCell},
	path::Path,
	rc::Rc,
	sync::Mutex,
};

use wxdragon::{prelude::*, translations::translate as t};

#[cfg(target_os = "linux")]
mod accessible_tree;
#[cfg(target_os = "linux")]
mod elements_gtk;
#[cfg(target_os = "linux")]
mod toc_gtk;

use paperback_core::{
	config::ConfigManager,
	document::TocItem,
	reader_core,
	session::DocumentSession,
	types::{BookmarkDisplayEntry, BookmarkFilterType, DocumentListStatus},
};

mod about;
pub use about::show_about_dialog;
mod document_info;
pub use document_info::show_document_info_dialog;
mod go_to_line;
pub use go_to_line::show_go_to_line_dialog;
mod go_to_page;
pub use go_to_page::show_go_to_page_dialog;
mod go_to_percent;
pub use go_to_percent::show_go_to_percent_dialog;
mod note_entry;
pub use note_entry::show_note_entry_dialog;
mod open_as;
pub use open_as::show_open_as_dialog;
mod options;
pub use options::show_options_dialog;
mod sleep_timer;
pub use sleep_timer::show_sleep_timer_dialog;
mod update;
pub use update::show_update_dialog;
mod view_note;
pub use view_note::show_view_note_dialog;
mod web_view;
pub use web_view::show_web_view_dialog;
mod word_count;
pub use word_count::show_word_count_dialog;

const DIALOG_PADDING: i32 = 10;
const RECENT_DOCS_LIST_WIDTH: i32 = 800;
const RECENT_DOCS_LIST_HEIGHT: i32 = 600;
const RECENT_DOCS_FILENAME_WIDTH: i32 = 250;
const RECENT_DOCS_STATUS_WIDTH: i32 = 100;
const RECENT_DOCS_PATH_WIDTH: i32 = 450;
const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;
#[cfg(not(target_os = "linux"))]
const KEY_SPACE: i32 = 32;
const KEY_RETURN: i32 = 13;
const KEY_NUMPAD_ENTER: i32 = 370;

pub struct BookmarkDialogResult {
	pub start: i64,
}

pub fn show_bookmark_dialog(
	parent: &Frame,
	session: &DocumentSession,
	config: &Rc<Mutex<ConfigManager>>,
	current_pos: i64,
	initial_filter: BookmarkFilterType,
) -> Option<BookmarkDialogResult> {
	let file_path = session.file_path().to_string();
	let content = Rc::new(session.content());
	let dialog = Dialog::builder(parent, &t("Jump to Bookmark")).build();
	let BookmarkDialogUi {
		filter_choice,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	} = build_bookmark_dialog_ui(dialog, initial_filter);
	let state = build_bookmark_dialog_state(jump_button, delete_button, edit_button);
	let repopulate = build_bookmark_repopulate(BookmarkRepopulateParams {
		list: bookmark_list,
		config: Rc::clone(config),
		file_path: file_path.clone(),
		content: Rc::clone(&content),
		entries: Rc::clone(&state.entries),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		filter_choice,
		set_buttons_enabled: Rc::clone(&state.set_buttons_enabled),
	});
	repopulate(current_pos);
	bind_bookmark_selection(BookmarkSelectionParams {
		list: bookmark_list,
		entries: Rc::clone(&state.entries),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		set_buttons_enabled: Rc::clone(&state.set_buttons_enabled),
	});
	bind_bookmark_jump(dialog, jump_button, &state.selected_start);
	bind_bookmark_actions(BookmarkDialogActions {
		dialog,
		filter_choice,
		bookmark_list,
		edit_button,
		delete_button,
		cancel_button,
		repopulate: Rc::clone(&repopulate),
		selected_start: Rc::clone(&state.selected_start),
		selected_end: Rc::clone(&state.selected_end),
		config: Rc::clone(config),
		file_path,
		current_pos,
	});
	finalize_bookmark_dialog_layout(
		dialog,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	);
	if dialog.show_modal() != wxdragon::id::ID_OK {
		return None;
	}
	let start = state.selected_start.get();
	if start >= 0 { Some(BookmarkDialogResult { start }) } else { None }
}

struct BookmarkDialogUi {
	filter_choice: ComboBox,
	filter_sizer: BoxSizer,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	jump_button: Button,
	cancel_button: Button,
}

struct BookmarkDialogState {
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkRepopulateParams {
	list: ListBox,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	content: Rc<String>,
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	filter_choice: ComboBox,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkSelectionParams {
	list: ListBox,
	entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	set_buttons_enabled: Rc<dyn Fn(bool)>,
}

struct BookmarkDialogActions {
	dialog: Dialog,
	filter_choice: ComboBox,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	cancel_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
}

fn build_bookmark_dialog_ui(dialog: Dialog, initial_filter: BookmarkFilterType) -> BookmarkDialogUi {
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
	BookmarkDialogUi {
		filter_choice,
		filter_sizer,
		bookmark_list,
		edit_button,
		delete_button,
		jump_button,
		cancel_button,
	}
}

fn build_bookmark_dialog_state(jump_button: Button, delete_button: Button, edit_button: Button) -> BookmarkDialogState {
	let entries: Rc<RefCell<Vec<BookmarkDisplayEntry>>> = Rc::new(RefCell::new(Vec::new()));
	let selected_start = Rc::new(Cell::new(-1i64));
	let selected_end = Rc::new(Cell::new(-1i64));
	let jump_button_for_state = jump_button;
	let delete_button_for_state = delete_button;
	let edit_button_for_state = edit_button;
	let set_buttons_enabled = Rc::new(move |enabled: bool| {
		jump_button_for_state.enable(enabled);
		delete_button_for_state.enable(enabled);
		edit_button_for_state.enable(enabled);
	});
	set_buttons_enabled(false);
	BookmarkDialogState { entries, selected_start, selected_end, set_buttons_enabled }
}

fn build_bookmark_repopulate(params: BookmarkRepopulateParams) -> Rc<dyn Fn(i64)> {
	let BookmarkRepopulateParams {
		list,
		config,
		file_path,
		content,
		entries,
		selected_start,
		selected_end,
		filter_choice,
		set_buttons_enabled,
	} = params;
	Rc::new(move |pos: i64| {
		let filter_index = filter_choice.get_selection().unwrap_or(0);
		let filter = match filter_index {
			1 => BookmarkFilterType::BookmarksOnly,
			2 => BookmarkFilterType::NotesOnly,
			_ => BookmarkFilterType::All,
		};
		let content_for_snippet = Rc::clone(&content);
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
		let content_for_line = Rc::clone(&content);
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
		let previous_selected = selected_start.get();
		list.clear();
		entries.borrow_mut().clear();
		let filtered = {
			let cfg = config.lock().unwrap();
			reader_core::get_filtered_bookmarks(&cfg, &file_path, pos, filter)
		};
		for item in filtered.items {
			let snippet =
				if item.is_whole_line { get_line_text(item.start) } else { get_text_range(item.start, item.end) };
			let mut snippet = snippet.trim().to_string();
			if snippet.is_empty() {
				snippet = t("blank");
			}
			let display = if item.note.is_empty() { snippet.clone() } else { format!("{} - {}", item.note, snippet) };
			entries.borrow_mut().push(BookmarkDisplayEntry { start: item.start, end: item.end });
			list.append(&display);
		}
		selected_start.set(-1);
		selected_end.set(-1);
		set_buttons_enabled(false);
		let entries_ref = entries.borrow();
		if previous_selected >= 0 {
			if let Some((idx, entry)) =
				entries_ref.iter().enumerate().find(|(_, entry)| entry.start == previous_selected)
			{
				if let Ok(idx_u32) = u32::try_from(idx) {
					list.set_selection(idx_u32, true);
				}
				selected_start.set(entry.start);
				selected_end.set(entry.end);
				set_buttons_enabled(true);
				return;
			}
		}
		if filtered.closest_index >= 0 {
			if let Ok(idx) = usize::try_from(filtered.closest_index) {
				if let Some(entry) = entries_ref.get(idx) {
					if let Ok(idx_u32) = u32::try_from(idx) {
						list.set_selection(idx_u32, true);
					}
					selected_start.set(entry.start);
					selected_end.set(entry.end);
					set_buttons_enabled(true);
				}
			}
		}
	})
}

fn bind_bookmark_selection(params: BookmarkSelectionParams) {
	let BookmarkSelectionParams { list, entries, selected_start, selected_end, set_buttons_enabled } = params;
	list.on_selection_changed(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			let entries_ref = entries.borrow();
			if let Ok(index) = usize::try_from(selection) {
				if let Some(entry) = entries_ref.get(index) {
					selected_start.set(entry.start);
					selected_end.set(entry.end);
					set_buttons_enabled(true);
					return;
				}
			}
		}
		selected_start.set(-1);
		selected_end.set(-1);
		set_buttons_enabled(false);
	});
}

fn bind_bookmark_jump(dialog: Dialog, jump_button: Button, selected_start: &Rc<Cell<i64>>) {
	let dialog_for_jump = dialog;
	let selected_start_for_jump = Rc::clone(selected_start);
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
}

fn bind_bookmark_actions(actions: BookmarkDialogActions) {
	let BookmarkDialogActions {
		dialog,
		filter_choice,
		bookmark_list,
		edit_button,
		delete_button,
		cancel_button,
		repopulate,
		selected_start,
		selected_end,
		config,
		file_path,
		current_pos,
	} = actions;
	bind_bookmark_cancel(dialog, cancel_button);
	bind_bookmark_filter(filter_choice, Rc::clone(&repopulate), current_pos);
	bind_bookmark_delete(
		delete_button,
		Rc::clone(&repopulate),
		Rc::clone(&selected_start),
		Rc::clone(&selected_end),
		Rc::clone(&config),
		file_path.clone(),
		current_pos,
	);
	bind_bookmark_edit(BookmarkEditParams {
		dialog,
		edit_button,
		repopulate: Rc::clone(&repopulate),
		selected_start: Rc::clone(&selected_start),
		selected_end: Rc::clone(&selected_end),
		config: Rc::clone(&config),
		file_path: file_path.clone(),
		current_pos,
	});
	bind_bookmark_key_actions(
		bookmark_list,
		Rc::clone(&repopulate),
		Rc::clone(&selected_start),
		Rc::clone(&selected_end),
		Rc::clone(&config),
		file_path,
		current_pos,
	);
	bind_bookmark_double_click(bookmark_list, dialog, selected_start);
}

fn bind_bookmark_cancel(dialog: Dialog, cancel_button: Button) {
	cancel_button.on_click(move |_| {
		dialog.end_modal(wxdragon::id::ID_CANCEL);
	});
}

fn bind_bookmark_filter(filter_choice: ComboBox, repopulate: Rc<dyn Fn(i64)>, current_pos: i64) {
	filter_choice.on_selection_changed(move |_event| {
		repopulate(current_pos);
	});
}

fn bind_bookmark_delete(
	delete_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
) {
	delete_button.on_click(move |_| {
		let start = selected_start.get();
		let end = selected_end.get();
		if start < 0 {
			return;
		}
		{
			let cfg = config.lock().unwrap();
			cfg.remove_bookmark(&file_path, start, end);
			cfg.flush();
		}
		repopulate(current_pos);
	});
}

struct BookmarkEditParams {
	dialog: Dialog,
	edit_button: Button,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
}

fn bind_bookmark_edit(params: BookmarkEditParams) {
	let BookmarkEditParams {
		dialog,
		edit_button,
		repopulate,
		selected_start,
		selected_end,
		config,
		file_path,
		current_pos,
	} = params;
	edit_button.on_click(move |_| {
		let start = selected_start.get();
		let end = selected_end.get();
		if start < 0 {
			return;
		}
		let existing_note = {
			let cfg = config.lock().unwrap();
			cfg.get_bookmarks(&file_path)
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
			let cfg = config.lock().unwrap();
			cfg.update_bookmark_note(&file_path, start, end, &note);
			cfg.flush();
		}
		repopulate(current_pos);
	});
}

fn bind_bookmark_key_actions(
	bookmark_list: ListBox,
	repopulate: Rc<dyn Fn(i64)>,
	selected_start: Rc<Cell<i64>>,
	selected_end: Rc<Cell<i64>>,
	config: Rc<Mutex<ConfigManager>>,
	file_path: String,
	current_pos: i64,
) {
	bookmark_list.bind_internal(EventType::KEY_DOWN, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		if key == KEY_DELETE || key == KEY_NUMPAD_DELETE {
			let start = selected_start.get();
			let end = selected_end.get();
			if start >= 0 {
				{
					let cfg = config.lock().unwrap();
					cfg.remove_bookmark(&file_path, start, end);
					cfg.flush();
				}
				repopulate(current_pos);
			}
			event.skip(false);
			return;
		}
		event.skip(true);
	});
}

fn bind_bookmark_double_click(bookmark_list: ListBox, dialog: Dialog, selected_start: Rc<Cell<i64>>) {
	bookmark_list.on_item_double_clicked(move |_| {
		if selected_start.get() >= 0 {
			dialog.end_modal(wxdragon::id::ID_OK);
		}
	});
}

fn finalize_bookmark_dialog_layout(
	dialog: Dialog,
	filter_sizer: BoxSizer,
	bookmark_list: ListBox,
	edit_button: Button,
	delete_button: Button,
	jump_button: Button,
	cancel_button: Button,
) {
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
}

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	#[cfg(target_os = "linux")]
	return toc_gtk::show_toc_dialog(parent, toc_items, current_offset);
	#[cfg(not(target_os = "linux"))]
	return show_toc_dialog_wx(parent, toc_items, current_offset);
}

#[cfg(not(target_os = "linux"))]
fn show_toc_dialog_wx(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1));
	let (tree, _root) = build_toc_tree(dialog, toc_items, current_offset);
	bind_toc_selection(tree, Rc::clone(&selected_offset));
	bind_toc_activation(dialog, tree, Rc::clone(&selected_offset));
	bind_toc_search(tree);
	let (ok_button, cancel_button) = build_toc_buttons(dialog);
	bind_toc_ok(dialog, ok_button, Rc::clone(&selected_offset));
	bind_toc_layout(dialog, tree, ok_button, cancel_button);
	tree.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "linux"))]
fn build_toc_tree(dialog: Dialog, toc_items: &[TocItem], current_offset: i32) -> (TreeCtrl, TreeItemId) {
	let tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	let root = tree.add_root(&t("Root"), None, None).unwrap();
	populate_toc_tree(tree, &root, toc_items);
	if current_offset != -1 {
		find_and_select_item(tree, &root, current_offset);
	}
	(tree, root)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_selection(tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let tree_for_sel = tree;
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_sel.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_activation(dialog: Dialog, tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let dialog_for_activate = dialog;
	let tree_for_activate = tree;
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
					dialog_for_activate.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_search(tree: TreeCtrl) {
	// Native Win32 first-letter navigation is used as-is. The only tweak needed is
	// preventing space from activating the OK button (space fires item_activated on
	// the tree, which our activation handler maps to OK).
	tree.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_SPACE {
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

#[cfg(not(target_os = "linux"))]
fn build_toc_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_ok(dialog: Dialog, ok_button: Button, selected_offset: Rc<Cell<i32>>) {
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		if selected_offset.get() >= 0 {
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
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_layout(dialog: Dialog, tree: TreeCtrl, ok_button: Button, cancel_button: Button) {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

#[cfg(not(target_os = "linux"))]
fn populate_toc_tree(tree: TreeCtrl, parent: &TreeItemId, items: &[TocItem]) {
	for item in items {
		let display_text = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		let offset = i32::try_from(item.offset).unwrap_or(i32::MAX);
		if let Some(id) = tree.append_item_with_data(parent, &display_text, offset, None, None) {
			if !item.children.is_empty() {
				populate_toc_tree(tree, &id, &item.children);
			}
		}
	}
}

#[cfg(not(target_os = "linux"))]
fn find_and_select_item(tree: TreeCtrl, parent: &TreeItemId, offset: i32) -> bool {
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
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
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
		if let Some(path) = get_selected_path(list) {
			if Path::new(&path).exists() {
				*selected_path.lock().unwrap() = Some(path);
				dialog.end_modal(wxdragon::id::ID_OK);
			}
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
		if let Some(key) = event.get_key_code() {
			if key == KEY_RETURN || key == KEY_NUMPAD_ENTER {
				open_action_for_char();
				event.skip(false);
				return;
			}
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
	if let Ok(index_u64) = u64::try_from(index) {
		if let Some(data) = list.get_custom_data(index_u64) {
			if let Some(path) = data.as_ref().downcast_ref::<String>() {
				return Some(path.clone());
			}
		}
	}
	let path = list.get_item_text(i64::from(index), 2);
	if path.is_empty() { None } else { Some(path) }
}

fn get_selected_path(list: ListCtrl) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	#[cfg(target_os = "linux")]
	return elements_gtk::show_elements_dialog(parent, session, current_pos);
	#[cfg(not(target_os = "linux"))]
	return show_elements_dialog_wx(parent, session, current_pos);
}

#[cfg(not(target_os = "linux"))]
fn show_elements_dialog_wx(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list } = build_elements_dialog_ui(dialog);
	let (selected_offset, link_offsets) = populate_elements_dialog(session, current_pos, headings_tree, links_list);
	bind_elements_view_toggle(view_choice, headings_tree, links_list, dialog);
	bind_elements_activation(dialog, headings_tree, links_list, &selected_offset, &link_offsets);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action(dialog, view_choice, headings_tree, links_list, &link_offsets, &selected_offset, ok_button);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
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

#[cfg(not(target_os = "linux"))]
struct ElementsDialogUi {
	content_sizer: BoxSizer,
	view_choice: ComboBox,
	headings_tree: TreeCtrl,
	links_list: ListBox,
}

#[cfg(not(target_os = "linux"))]
fn build_elements_dialog_ui(dialog: Dialog) -> ElementsDialogUi {
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
	ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list }
}

#[cfg(not(target_os = "linux"))]
fn populate_elements_dialog(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: TreeCtrl,
	links_list: ListBox,
) -> (Rc<Cell<i64>>, Rc<Vec<i64>>) {
	let selected_offset = Rc::new(Cell::new(-1i64));
	let root = headings_tree.add_root(&t("Root"), None, None).unwrap();
	let tree_data = session.heading_tree(current_pos);
	let mut item_ids: Vec<wxdragon::widgets::treectrl::TreeItemId> = Vec::new();
	if !tree_data.items.is_empty() {
		item_ids.reserve(tree_data.items.len());
	}
	for item in &tree_data.items {
		let parent_id = if item.parent_index >= 0 {
			usize::try_from(item.parent_index)
				.ok()
				.and_then(|idx| item_ids.get(idx).cloned())
				.unwrap_or_else(|| root.clone())
		} else {
			root.clone()
		};
		let display_text = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		if let Some(id) = headings_tree.append_item_with_data(&parent_id, &display_text, offset, None, None) {
			item_ids.push(id);
		} else if let Some(root_child) = headings_tree.append_item_with_data(&root, &display_text, offset, None, None) {
			item_ids.push(root_child);
		}
	}
	headings_tree.expand_all();
	if tree_data.closest_index >= 0 {
		if let Ok(index) = usize::try_from(tree_data.closest_index) {
			if let Some(item) = item_ids.get(index) {
				headings_tree.select_item(item);
				headings_tree.ensure_visible(item);
			}
		}
	} else if let Some((first_child, _)) = headings_tree.get_first_child(&root) {
		headings_tree.select_item(&first_child);
		headings_tree.ensure_visible(&first_child);
	}
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in link_data.items {
		links_list.append(&item.text);
		link_offsets.push(i64::try_from(item.offset).unwrap_or(i64::MAX));
	}
	if !link_offsets.is_empty() {
		let idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		if let Ok(idx_u32) = u32::try_from(idx) {
			links_list.set_selection(idx_u32, true);
		}
	}
	(selected_offset, Rc::new(link_offsets))
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_view_toggle(view_choice: ComboBox, headings_tree: TreeCtrl, links_list: ListBox, dialog: Dialog) {
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
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_activation(
	dialog: Dialog,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	selected_offset: &Rc<Cell<i64>>,
	link_offsets: &Rc<Vec<i64>>,
) {
	let selected_offset_for_tree = Rc::clone(selected_offset);
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
	let selected_offset_for_list = Rc::clone(selected_offset);
	let offsets_for_list = Rc::clone(link_offsets);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			if let Ok(index) = usize::try_from(selection) {
				if let Some(offset) = offsets_for_list.get(index) {
					selected_offset_for_list.set(*offset);
					dialog_for_list.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn build_elements_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
fn bind_elements_ok_action(
	dialog: Dialog,
	view_choice: ComboBox,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	link_offsets: &Rc<Vec<i64>>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(link_offsets);
	let selected_offset_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			if let Some(item) = headings_tree.get_selection() {
				if let Some(data) = headings_tree.get_custom_data(&item) {
					if let Some(offset) = data.downcast_ref::<i64>() {
						selected_offset_for_ok.set(*offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(idx) = links_list.get_selection() {
			if let Ok(index) = usize::try_from(idx) {
				if let Some(offset) = offsets_for_ok.get(index) {
					selected_offset_for_ok.set(*offset);
					dialog_for_ok.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn finalize_elements_layout(dialog: Dialog, content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}
