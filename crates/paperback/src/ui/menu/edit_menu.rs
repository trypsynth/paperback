use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

#[cfg(target_os = "macos")]
use super::builder::item;
use super::builder::{MenuEntry, format_menu_label, item_with_help};
use crate::ui::{commands, menu_ids};

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let mut entries = Vec::new();
	#[cfg(target_os = "macos")]
	{
		// TRANSLATORS: Menu item in the Edit menu to undo the last action.
		let undo_label = t("&Undo\tCtrl+Z");
		// TRANSLATORS: Menu item in the Edit menu to redo the last undone action.
		let redo_label = t("&Redo\tCtrl+Shift+Z");
		// TRANSLATORS: Menu item in the Edit menu to cut the current selection.
		let cut_label = t("Cu&t\tCtrl+X");
		// TRANSLATORS: Menu item in the Edit menu to copy the current selection.
		let copy_label = t("&Copy\tCtrl+C");
		// TRANSLATORS: Menu item in the Edit menu to paste from the clipboard.
		let paste_label = t("&Paste\tCtrl+V");
		// TRANSLATORS: Menu item in the Edit menu to delete the current selection.
		let delete_label = t("&Delete");
		// TRANSLATORS: Menu item in the Edit menu to select all text.
		let select_all_label = t("Select &All\tCtrl+A");
		entries.extend([
			item(menu_ids::UNDO, undo_label),
			item(menu_ids::REDO, redo_label),
			MenuEntry::Separator,
			item(menu_ids::CUT, cut_label),
			item(menu_ids::COPY, copy_label),
			item(menu_ids::PASTE, paste_label),
			item(menu_ids::DELETE, delete_label),
			MenuEntry::Separator,
			item(menu_ids::SELECT_ALL, select_all_label),
			MenuEntry::Separator,
		]);
	}
	// TRANSLATORS: Menu item in the Edit menu to open the find dialog.
	let find_label = format_menu_label(&t("&Find..."), ActionId::Find, config);
	// TRANSLATORS: Status-bar help text for the Edit > Find menu item.
	let find_help = t("Find text in the document");
	// TRANSLATORS: Menu item in the Edit menu to find the next occurrence of the current search term.
	let find_next_label = format_menu_label(&t("Find &Next"), ActionId::FindNext, config);
	// TRANSLATORS: Status-bar help text for the Edit > Find Next menu item.
	let find_next_help = t("Find next occurrence");
	// TRANSLATORS: Menu item in the Edit menu to find the previous occurrence of the current search term.
	let find_prev_label = format_menu_label(&t("Find &Previous"), ActionId::FindPrevious, config);
	// TRANSLATORS: Status-bar help text for the Edit > Find Previous menu item.
	let find_prev_help = t("Find previous occurrence");
	entries.extend([
		item_with_help(menu_ids::FIND, find_label, find_help),
		item_with_help(menu_ids::FIND_NEXT, find_next_label, find_next_help),
		item_with_help(menu_ids::FIND_PREVIOUS, find_prev_label, find_prev_help),
		MenuEntry::Separator,
	]);
	entries.extend(commands::menu_entries(
		&[ActionId::SetSelectionStart, ActionId::CopyFromSelectionStart, ActionId::JumpToSelectionStart],
		config,
	));
	entries
}
