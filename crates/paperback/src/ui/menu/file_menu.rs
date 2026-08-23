use std::path::Path;

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::builder::format_menu_label;
use crate::ui::menu_ids;

pub fn create_file_menu(config: &ConfigManager) -> Menu {
	let open_label = format_menu_label(&t("&Open..."), ActionId::Open, config);
	let open_help = t("Open a document");
	let close_label = format_menu_label(&t("&Close"), ActionId::Close, config);
	let close_help = t("Close the current document");
	let close_all_label = format_menu_label(&t("Close &All"), ActionId::CloseAll, config);
	let close_all_help = t("Close all documents");
	let reopen_label = format_menu_label(&t("Reopen &Last Closed"), ActionId::ReopenLastClosed, config);
	let reopen_help = t("Reopen the last closed document");
	let file_menu = Menu::builder()
		.append_item(menu_ids::OPEN, &open_label, &open_help)
		.append_item(menu_ids::CLOSE, &close_label, &close_help)
		.append_item(menu_ids::CLOSE_ALL, &close_all_label, &close_all_help)
		.append_item(menu_ids::REOPEN_LAST_CLOSED, &reopen_label, &reopen_help)
		.build();
	let recent_menu = Menu::builder().build();
	populate_recent_documents_menu(&recent_menu, config);
	let recent_label = t("&Recent Documents");
	let recent_help = t("Open a recent document");
	let _ = file_menu.append_submenu(recent_menu, &recent_label, &recent_help);
	if !cfg!(target_os = "macos") {
		file_menu.append_separator();
		let exit_label = format_menu_label(&t("E&xit"), ActionId::Exit, config);
		let exit_help = t("Exit the application");
		let _ = file_menu.append(menu_ids::EXIT, &exit_label, &exit_help, ItemKind::Normal);
	}
	file_menu
}

fn populate_recent_documents_menu(menu: &Menu, config: &ConfigManager) {
	let recent_docs = recent_documents_for_menu(config);
	if recent_docs.is_empty() {
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
