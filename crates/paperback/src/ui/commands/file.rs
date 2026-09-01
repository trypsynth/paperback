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
	main_window::{close_active_document_announced, ensure_parser_ready_for_path, update_title_from_manager},
	menu,
};

pub fn open(ctx: &Ctx) {
	let wildcard = build_file_filter_string();
	// TRANSLATORS: Title of the file picker dialog shown when opening a document
	let dialog_title = t("Open Document");
	let dialog = FileDialog::builder(ctx.frame)
		.with_message(&dialog_title)
		.with_wildcard(&wildcard)
		.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
		.build();
	if dialog.show_modal() == ID_OK
		&& let Some(path) = dialog.get_path()
	{
		let path = Path::new(&path);
		if !ensure_parser_ready_for_path(ctx.frame, path, ctx.config) {
			return;
		}
		if ctx.dm.lock().unwrap().open_file(ctx.dm, path) {
			let Ok(dm_ref) = ctx.dm.try_lock() else {
				return;
			};
			update_title_from_manager(ctx.frame, &dm_ref);
			dm_ref.focus_document_text();
			drop(dm_ref);
			let menu_bar = menu::create_menu_bar(&ctx.config.lock().unwrap());
			ctx.frame.set_menu_bar(menu_bar);
			menu::update_menu_item_states(ctx.frame, true);
		}
	}
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

pub fn exit(ctx: &Ctx) {
	ctx.dm.lock().unwrap().save_all_positions();
	process::exit(0);
}
