//! Assembles the File menu. Labels, shortcuts, help text and ids all come from the command
//! table; this file only decides the order things appear in, and owns the Recent Documents
//! submenu, whose items are generated from the recent list rather than being fixed commands.

use std::path::Path;

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::{commands, menu_ids};

/// The File menu's fixed items, in order, above the Recent Documents submenu.
const ITEMS: &[ActionId] = &[ActionId::Open, ActionId::Close, ActionId::CloseAll, ActionId::ReopenLastClosed];

/// Shown only on Windows and Linux; macOS puts Quit in the application menu.
const EXIT_ITEM: ActionId = ActionId::Exit;

pub fn create_file_menu(config: &ConfigManager) -> Menu {
	let file_menu = Menu::builder().build();
	for &action in ITEMS {
		commands::append_item(&file_menu, action, config);
	}
	let recent_menu = Menu::builder().build();
	populate_recent_documents_menu(&recent_menu, config);
	// TRANSLATORS: Label for the Recent Documents submenu in the File menu.
	let recent_label = t("&Recent Documents");
	// TRANSLATORS: Status-bar help text for the File > Recent Documents submenu.
	let recent_help = t("Open a recent document");
	let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);
	if !cfg!(target_os = "macos") {
		file_menu.append_separator();
		commands::append_item(&file_menu, EXIT_ITEM, config);
	}
	file_menu
}

fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
	let recent_docs = recent_documents_for_menu(config);
	if recent_docs.is_empty() {
		// TRANSLATORS: Placeholder menu item shown in the Recent Documents submenu when there are no recent documents.
		let empty_label = t("(No recent documents)");
		if let Some(item) = menu.append(ID_ANY.try_into().unwrap(), &empty_label, "", ItemKind::Normal) {
			item.enable(false);
		}
	} else {
		for (index, path) in recent_docs.iter().enumerate() {
			let filename =
				Path::new(path).file_name().map_or_else(|| path.clone(), |s| s.to_string_lossy().to_string());
			let label = format!("&{} {}", index + 1, filename);
			if let Ok(offset) = i32::try_from(index) {
				let id = menu_ids::RECENT_DOCUMENT_BASE + offset;
				let _ = menu.append(id, &label, path, ItemKind::Normal);
			}
		}
	}
	menu.append_separator();
	// TRANSLATORS: Menu item at the bottom of the Recent Documents submenu to open the full list of documents.
	let show_all_label = format_menu_label(&t("Show All..."), ActionId::ShowAllRecentDocuments, config);
	let _ = menu.append(menu_ids::SHOW_ALL_DOCUMENTS, &show_all_label, "", ItemKind::Normal);
}

pub fn recent_documents_for_menu(config: &ConfigManager) -> Vec<String> {
	let limit = usize::try_from(config.get_app_int("recent_documents_to_show", 25).max(0)).unwrap_or(0);
	let mut docs = config.get_recent_documents();
	if docs.len() > limit {
		docs.truncate(limit);
	}
	docs
}

#[cfg(test)]
mod tests {
	use super::*;

	/// `commands::append_item` panics on an action that is not in the table, so an unported
	/// entry here would crash the app while building its menu bar at startup.
	#[test]
	fn every_file_menu_item_is_a_known_command() {
		for &action in ITEMS.iter().chain(std::iter::once(&EXIT_ITEM)) {
			assert!(commands::for_action(action).is_some(), "{action:?} is in the File menu but not in COMMANDS");
		}
	}
}
