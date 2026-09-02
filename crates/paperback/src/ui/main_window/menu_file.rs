//! The "open a document by path" side of the File menu: dynamic recent-document ids
//! (`menu_ids::RECENT_DOCUMENT_BASE..=RECENT_DOCUMENT_MAX`) and the "All Documents" dialog.
//! `Open`/`Close`/`Close All`/`Reopen Last Closed`/`Exit` are thin enough to stay inlined in
//! `bind_menu_events`'s dispatch match.

use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, dialogs, ensure_parser_ready_for_path, menu, menu_ids, update_title_from_manager};

/// Handles every menu id not covered by `bind_menu_events`'s own dispatch match: dynamic
/// recent-document entries and "Show All Documents". Does nothing if `id` matches neither.
pub(super) fn handle_fallback(
	id: i32,
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	if (menu_ids::RECENT_DOCUMENT_BASE..=menu_ids::RECENT_DOCUMENT_MAX).contains(&id) {
		let doc_index = id - menu_ids::RECENT_DOCUMENT_BASE;
		let recent_docs = {
			let config_guard = config.lock().unwrap();
			menu::recent_documents_for_menu(&config_guard)
		};
		if let Ok(doc_index) = usize::try_from(doc_index)
			&& let Some(path) = recent_docs.get(doc_index)
		{
			let path = Path::new(path);
			if !ensure_parser_ready_for_path(frame, path, config) {
				return;
			}
			if dm.lock().unwrap().open_file(dm, path) {
				{
					let dm_ref = dm.lock().unwrap();
					update_title_from_manager(frame, &dm_ref);
					dm_ref.focus_document_text();
				}
				let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
				frame.set_menu_bar(menu_bar);
				menu::update_menu_item_states(frame, true);
				let has_reopen = dm.lock().unwrap().has_recently_closed();
				menu::update_reopen_state(frame, has_reopen);
			}
		}
	} else if id == menu_ids::SHOW_ALL_DOCUMENTS {
		handle_show_all_documents(frame, dm, config, live_region_label);
	}
}

fn handle_show_all_documents(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let has_documents = {
		let config_guard = config.lock().unwrap();
		!config_guard.get_all_documents().is_empty()
	};
	if !has_documents {
		// TRANSLATORS: Announced when opening "All Documents" while the recent-documents list is empty
		live_region::announce(live_region_label, &t("No recent documents."));
		return;
	}
	let open_paths = dm.lock().unwrap().open_paths();
	let config_for_dialog = Rc::clone(config);
	let result = dialogs::show_all_documents_dialog(frame, &config_for_dialog, open_paths);
	{
		let mut dm_ref = dm.lock().unwrap();
		for path_str in &result.paths_to_close {
			let path = Path::new(path_str);
			if let Some(index) = dm_ref.find_tab_by_path(path) {
				dm_ref.close_document(index, false);
			}
		}
		if !result.paths_to_close.is_empty() {
			update_title_from_manager(frame, &dm_ref);
			dm_ref.restore_focus();
		}
	}
	if let Some(path) = result.open {
		let path_buf = Path::new(&path).to_path_buf();
		let path = path_buf.as_path();
		if !ensure_parser_ready_for_path(frame, path, config) {
			return;
		}
		if dm.lock().unwrap().open_file(dm, path) {
			{
				let dm_ref = dm.lock().unwrap();
				update_title_from_manager(frame, &dm_ref);
				dm_ref.focus_document_text();
			}
			let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
			frame.set_menu_bar(menu_bar);
			menu::update_menu_item_states(frame, true);
			let has_reopen = dm.lock().unwrap().has_recently_closed();
			menu::update_reopen_state(frame, has_reopen);
		} else {
			let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
			frame.set_menu_bar(menu_bar);
			let dm_ref = dm.lock().unwrap();
			let has_docs = dm_ref.tab_count() > 0;
			let has_reopen = dm_ref.has_recently_closed();
			drop(dm_ref);
			menu::update_menu_item_states(frame, has_docs);
			menu::update_reopen_state(frame, has_reopen);
		}
	} else {
		let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);
		let dm_ref = dm.lock().unwrap();
		let has_docs = dm_ref.tab_count() > 0;
		let has_reopen = dm_ref.has_recently_closed();
		drop(dm_ref);
		menu::update_menu_item_states(frame, has_docs);
		menu::update_reopen_state(frame, has_reopen);
	}
}
