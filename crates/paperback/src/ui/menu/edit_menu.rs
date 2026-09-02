use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use crate::ui::menu_ids;

pub fn create_edit_menu(_config: &ConfigManager) -> Menu {
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
	Menu::builder()
		.append_item(menu_ids::UNDO, &undo_label, "")
		.append_item(menu_ids::REDO, &redo_label, "")
		.append_separator()
		.append_item(menu_ids::CUT, &cut_label, "")
		.append_item(menu_ids::COPY, &copy_label, "")
		.append_item(menu_ids::PASTE, &paste_label, "")
		.append_item(menu_ids::DELETE, &delete_label, "")
		.append_separator()
		.append_item(menu_ids::SELECT_ALL, &select_all_label, "")
		.build()
}
