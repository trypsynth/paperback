//! Opening and closing documents: the behaviour behind the File menu's own commands.
//!
//! The dynamic recent-document ids and the All Documents dialog are still in
//! `main_window/menu_file.rs`, since neither is a fixed command with an [`ActionId`]:
//! they are a range of ids generated from the recent list.
//!
//! [`ActionId`]: paperback_core::config::ActionId

use std::{path::Path, process};

use paperback_core::parser::build_file_filter_string;
use patois::t;
use wxdragon::prelude::*;

use super::Ctx;
use crate::ui::{
	main_window::{
		close_active_document_announced, ensure_parser_ready_for_path, rebuild_menu_bar, update_title_from_manager,
	},
	menu,
	navigation::announce,
};

pub fn open(ctx: &Ctx) {
	let wildcard = build_file_filter_string();
	// TRANSLATORS: Title of the file picker dialog shown when opening a document
	let dialog_title = t("Open Document");
	let dialog = FileDialog::builder(ctx.frame)
		.with_message(&dialog_title)
		.with_wildcard(&wildcard)
		.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist | FileDialogStyle::Multiple)
		.build();
	if dialog.show_modal() != ID_OK {
		return;
	}
	// Windows hands back a multiple selection in no useful order, often the last file clicked first, so the tabs follow the folder instead.
	let mut paths = dialog.get_paths();
	paths.sort();
	let mut opened_any = false;
	for path in &paths {
		let path = Path::new(path);
		// One book that cannot be opened, or whose unknown type the reader declines to pick a reader for, does not stop the rest.
		if ensure_parser_ready_for_path(ctx.frame, path, ctx.config) && ctx.dm.lock().unwrap().open_file(ctx.dm, path) {
			opened_any = true;
		}
	}
	if !opened_any {
		return;
	}
	let Ok(dm_ref) = ctx.dm.try_lock() else {
		return;
	};
	update_title_from_manager(ctx.frame, &dm_ref);
	dm_ref.focus_document_text();
	drop(dm_ref);
	let menu_bar = menu::create_menu_bar(&ctx.config.lock().unwrap());
	ctx.frame.set_menu_bar(menu_bar);
	menu::update_menu_item_states(ctx.frame, true);
	// The rebuilt menu bar comes back with every item enabled, so the reopen item has
	// to be told again whether there is anything to reopen. Every other place that
	// rebuilds the bar does this; this one did not, which left Reopen Last Closed
	// enabled and doing nothing after opening a document from a fresh start.
	let has_reopen = ctx.dm.lock().unwrap().has_recently_closed();
	menu::update_reopen_state(ctx.frame, has_reopen);
}

pub fn close(ctx: &Ctx) {
	let mut dm = ctx.dm.lock().unwrap();
	close_active_document_announced(&mut dm, ctx.live_region_label);
	update_title_from_manager(ctx.frame, &dm);
	let has_docs = dm.tab_count() > 0;
	if has_docs {
		dm.restore_focus();
	} else {
		dm.notebook().set_focus();
	}
	drop(dm);
	menu::update_menu_item_states(ctx.frame, has_docs);
	menu::update_reopen_state(ctx.frame, true);
}

pub fn close_all(ctx: &Ctx) {
	let mut dm = ctx.dm.lock().unwrap();
	dm.close_all_documents();
	update_title_from_manager(ctx.frame, &dm);
	dm.notebook().set_focus();
	drop(dm);
	menu::update_menu_item_states(ctx.frame, false);
	menu::update_reopen_state(ctx.frame, true);
}

pub fn reopen_last_closed(ctx: &Ctx) {
	let path = ctx.dm.lock().unwrap().pop_recently_closed();
	if let Some(path) = path {
		if !ensure_parser_ready_for_path(ctx.frame, &path, ctx.config) {
			// Put it back: the document was never reopened, so it is still the last closed one.
			ctx.dm.lock().unwrap().push_recently_closed(path);
			return;
		}
		if ctx.dm.lock().unwrap().open_file(ctx.dm, &path) {
			let dm_ref = ctx.dm.lock().unwrap();
			update_title_from_manager(ctx.frame, &dm_ref);
			dm_ref.focus_document_text();
			drop(dm_ref);
			let menu_bar = menu::create_menu_bar(&ctx.config.lock().unwrap());
			ctx.frame.set_menu_bar(menu_bar);
			menu::update_menu_item_states(ctx.frame, true);
		}
		let has_reopen = ctx.dm.lock().unwrap().has_recently_closed();
		menu::update_reopen_state(ctx.frame, has_reopen);
	}
}

/// Re-reads the current document from disk, whether or not it changed and whether or not automatic reloading is on.
///
/// `try_lock`, because `reload_tab` can raise a password prompt whose event loop re-enters the menu handlers. A missing file or a failed parse is silent, as on the automatic path: the document they were reading stays put.
pub fn reload(ctx: &Ctx) {
	let Ok(mut dm) = ctx.dm.try_lock() else {
		return;
	};
	let Some(index) = dm.active_tab_index() else {
		return;
	};
	// A file that is gone has nothing to re-read, so a batch running on it keeps going rather than losing its pages for nothing.
	if !dm.active_tab().is_some_and(|tab| tab.file_path.exists()) {
		return;
	}
	// Stopped before the re-read, or its pages would be put back into the new buffer, and quietly, since a count of pages recognized describes text the re-read throws away.
	dm.cancel_ocr(true);
	if !dm.reload_tab(index, true) {
		return;
	}
	update_title_from_manager(ctx.frame, &dm);
	dm.update_status_bar();
	// High, unlike the automatic reload's Medium: here the reader asked for it.
	// TRANSLATORS: Announced by screen readers after a document was re-read from disk, either because the reader asked for it with F5 or because its file changed on disk
	live_region::announce_with_priority(ctx.live_region_label, &t("Document reloaded."), live_region::Priority::High);
}

pub fn clear_recent_documents(ctx: &Ctx) {
	{
		let cfg = ctx.config.lock().unwrap();
		if !cfg.has_recent_documents() {
			// TRANSLATORS: Announced when clearing the Recent Documents list while it is already empty
			announce(ctx.live_region_label, t("No recent documents."));
			return;
		}
		cfg.clear_recent_documents();
		cfg.flush();
	}
	rebuild_menu_bar(ctx.frame, ctx.dm, ctx.config);
	// TRANSLATORS: Announced after the Recent Documents list has been emptied
	announce(ctx.live_region_label, t("Recent documents cleared."));
}

pub fn exit(ctx: &Ctx) {
	ctx.dm.lock().unwrap().save_all_positions();
	process::exit(0);
}
