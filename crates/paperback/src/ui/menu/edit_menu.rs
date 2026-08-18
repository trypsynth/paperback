use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use crate::ui::menu_ids;

pub fn create_edit_menu(_config: &ConfigManager) -> Menu {
	let undo_label = t("&Undo\tCtrl+Z");
	let redo_label = t("&Redo\tCtrl+Shift+Z");
	let cut_label = t("Cu&t\tCtrl+X");
	let copy_label = t("&Copy\tCtrl+C");
	let paste_label = t("&Paste\tCtrl+V");
	let delete_label = t("&Delete");
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
