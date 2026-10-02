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

/// Reloads the current document from disk on request, whether or not the file has changed and
/// whether or not automatic reloading is switched on.
///
/// `try_lock` rather than the `lock().unwrap()` its neighbours here use, because `reload_tab`
/// can raise a modal password prompt whose event loop re-enters the menu handlers; holding this
/// lock across that is the deadlock `reload_tab`'s own `try_lock` on the config already avoids
/// one level down.
///
/// A file that is not there, and a re-parse that fails, are silent here exactly as they are on
/// the automatic path. The old text staying put is visible enough, and a modal error over a key
/// someone pressed once is a worse answer than the document they were reading remaining. A
/// running batch is cancelled either way, before the file is even looked at.
pub fn reload(ctx: &Ctx) {
	let Ok(mut dm) = ctx.dm.try_lock() else {
		return;
	};
	let Some(index) = dm.active_tab_index() else {
		return;
	};
	// Before the re-parse, and not after it: a batch that is still running would go on reading
	// pages of the file into a document that is being replaced underneath it. `apply_ocr_results`
	// re-resolves each page against the current session by number, so its text would land in the
	// freshly read buffer and put back exactly what the re-read was meant to clear. Closing the
	// document does not stop a batch either - the tab is dropped, the worker keeps its own handle
	// on the cancel flag and runs to the end - so this is the only place a reader can call one off
	// short of finding it by hand.
	//
	// Quiet, because the count of pages it managed before stopping describes text this re-read is
	// discarding. "Document reloaded." is the whole of what happened as far as the reader is
	// concerned, and a second line about the OCR would only contradict it.
	dm.cancel_ocr(true);
	if !dm.reload_tab(index, true) {
		return;
	}
	// The same two updates the frame-activation handler makes after a reload: both the title and
	// the status bar are derived from a session that has just been rebuilt.
	update_title_from_manager(ctx.frame, &dm);
	dm.update_status_bar();
	// High rather than the Medium the automatic path uses, because that priority is chosen
	// precisely on the grounds that the reader did not ask for this and it should queue behind
	// whatever is being read out. Here they did ask.
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
