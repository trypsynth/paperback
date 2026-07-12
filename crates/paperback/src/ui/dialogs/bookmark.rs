use std::{
	cell::{Cell, RefCell},
	rc::Rc,
	sync::Mutex,
};

use paperback_core::{
	config::ConfigManager,
	reader_core,
	session::DocumentSession,
	types::{BookmarkDisplayEntry, BookmarkFilterType},
};
use patois::t;
use wxdragon::prelude::*;

use super::show_note_entry_dialog;

const DIALOG_PADDING: i32 = 10;
const KEY_DELETE: i32 = 127;
const KEY_NUMPAD_DELETE: i32 = 330;

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
	// TRANSLATORS: Title of the Jump to Bookmark dialog
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
	filter_choice: Choice,
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
	filter_choice: Choice,
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
	filter_choice: Choice,
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
	// TRANSLATORS: Label for the bookmark filter dropdown
	let filter_label_text = t("&Filter:");
	let filter_label = StaticText::builder(&dialog).with_label(&filter_label_text).build();
	let filter_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: Option in the filter dropdown to show all entries
	filter_choice.append(&t("All"));
	// TRANSLATORS: Option in the filter dropdown to show bookmarks only
	filter_choice.append(&t("Bookmarks"));
	// TRANSLATORS: Option in the filter dropdown to show notes only
	filter_choice.append(&t("Notes"));
	let initial_index = match initial_filter {
		BookmarkFilterType::BookmarksOnly => 1,
		BookmarkFilterType::NotesOnly => 2,
		BookmarkFilterType::All => 0,
	};
	filter_choice.set_selection(initial_index);
	#[cfg(target_os = "macos")]
	filter_choice.set_accessibility_label(filter_label_text.replace('&', "").trim_end_matches(':').trim());
	let filter_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	filter_sizer.add(&filter_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 6);
	filter_sizer.add(&filter_choice, 1, SizerFlag::Expand, 0);
	let bookmark_list = ListBox::builder(&dialog).build();
	// TRANSLATORS: Label for the bookmark/note list (used only as an accessibility label on macOS)
	let list_label_text = t("&Bookmarks:");
	let _list_label = StaticText::builder(&dialog).with_label(&list_label_text).build();
	#[cfg(target_os = "macos")]
	bookmark_list.set_accessibility_label(list_label_text.replace('&', "").trim_end_matches(':').trim());
	// TRANSLATORS: Label for the button to edit a note
	let edit_button = Button::builder(&dialog).with_label(&t("&Edit Note")).build();
	// TRANSLATORS: Label for the button to delete a bookmark
	let delete_button = Button::builder(&dialog).with_label(&t("&Delete")).build();
	// TRANSLATORS: Label for the button to jump to the selected bookmark
	let jump_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("&Jump")).build();
	// TRANSLATORS: Label for the button to cancel the action
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
			if snippet.trim().is_empty() {
				// TRANSLATORS: Placeholder text shown in the bookmarks list when the bookmark text range is empty or blank
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
		if previous_selected >= 0
			&& let Some((idx, entry)) =
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
		if filtered.closest_index >= 0
			&& let Ok(idx) = usize::try_from(filtered.closest_index)
			&& let Some(entry) = entries_ref.get(idx)
		{
			if let Ok(idx_u32) = u32::try_from(idx) {
				list.set_selection(idx_u32, true);
			}
			selected_start.set(entry.start);
			selected_end.set(entry.end);
			set_buttons_enabled(true);
		}
	})
}

fn bind_bookmark_selection(params: BookmarkSelectionParams) {
	let BookmarkSelectionParams { list, entries, selected_start, selected_end, set_buttons_enabled } = params;
	list.on_selection_changed(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			let entries_ref = entries.borrow();
			if let Ok(index) = usize::try_from(selection)
				&& let Some(entry) = entries_ref.get(index)
			{
				selected_start.set(entry.start);
				selected_end.set(entry.end);
				set_buttons_enabled(true);
				return;
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
			MessageDialog::builder(
				&dialog_for_jump,
				// TRANSLATORS: Error message shown when the user attempts to jump without selecting any bookmark
				&t("Please select a bookmark to jump to."),
				// TRANSLATORS: Title of the error dialog
				&t("Error"),
			)
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

fn bind_bookmark_filter(filter_choice: Choice, repopulate: Rc<dyn Fn(i64)>, current_pos: i64) {
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
		let Some(note) = show_note_entry_dialog(
			&dialog,
			// TRANSLATORS: Title of the Bookmark Note editor dialog
			&t("Bookmark Note"),
			// TRANSLATORS: Label/prompt in the Note editor dialog
			&t("Edit bookmark note:"),
			&existing_note,
		) else {
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
