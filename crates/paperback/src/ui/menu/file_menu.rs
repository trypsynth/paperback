//! Assembles the File menu. Labels, shortcuts, help text and ids all come from the command
//! table; this file only decides the order things appear in, and owns the Recent Documents
//! submenu, whose items are generated from the recent list rather than being fixed commands.

use std::path::Path;

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;

use super::builder::{MenuEntry, format_menu_label, item, item_with_help, submenu};
use crate::ui::{commands, menu_ids};

/// The File menu's fixed items, in order, above the Recent Documents submenu.
const ITEMS: &[ActionId] =
	&[ActionId::Open, ActionId::Close, ActionId::CloseAll, ActionId::ReopenLastClosed, ActionId::Reload];

/// Shown only on Windows and Linux; macOS puts Quit in the application menu.
const EXIT_ITEM: ActionId = ActionId::Exit;

/// Last item of the Recent Documents submenu, below the generated entries and Show All.
const CLEAR_RECENT_ITEM: ActionId = ActionId::ClearRecentDocuments;

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let mut entries = commands::menu_entries(ITEMS, config);
	// TRANSLATORS: Label for the Recent Documents submenu in the File menu.
	let recent_label = t("&Recent Documents");
	// TRANSLATORS: Status-bar help text for the File > Recent Documents submenu.
	let recent_help = t("Open a recent document");
	entries.push(submenu(recent_label, recent_help, recent_document_entries(config)));
	if !cfg!(target_os = "macos") {
		entries.push(MenuEntry::Separator);
		entries.push(commands::menu_entry(EXIT_ITEM, config));
	}
	entries
}

fn recent_document_entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let recent_docs = recent_documents_for_menu(config);
	let mut entries = Vec::new();
	if recent_docs.is_empty() {
		// TRANSLATORS: Placeholder menu item shown in the Recent Documents submenu when there are no recent documents.
		entries.push(MenuEntry::Disabled(t("(No recent documents)")));
	} else {
		for (index, path) in recent_docs.iter().enumerate() {
			let filename =
				Path::new(path).file_name().map_or_else(|| path.clone(), |s| s.to_string_lossy().to_string());
			let label = format!("&{} {}", index + 1, filename);
			if let Ok(offset) = i32::try_from(index) {
				entries.push(item_with_help(menu_ids::RECENT_DOCUMENT_BASE + offset, label, path.clone()));
			}
		}
	}
	entries.push(MenuEntry::Separator);
	// TRANSLATORS: Menu item at the bottom of the Recent Documents submenu to open the full list of documents.
	let show_all_label = format_menu_label(&t("Show All..."), ActionId::ShowAllRecentDocuments, config);
	entries.push(item(menu_ids::SHOW_ALL_DOCUMENTS, show_all_label));
	entries.push(commands::menu_entry(CLEAR_RECENT_ITEM, config));
	entries
}

pub fn recent_documents_for_menu(config: &ConfigManager) -> Vec<String> {
	let mut docs = config.get_recent_documents();
	docs.truncate(config.recent_documents_limit());
	docs
}
