//! The heavier File menu handlers: dynamic recent-document ids
//! (`menu_ids::RECENT_DOCUMENT_BASE..=RECENT_DOCUMENT_MAX`), the "All Documents" dialog, exporting
//! the document, and importing and exporting its data. `Open`/`Close`/`Close All`/`Reopen Last
//! Closed`/`Exit` are thin enough to stay inlined in `bind_menu_events`'s dispatch match.

use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, export::ExportFormat};
use patois::t;
use wxdragon::prelude::*;

use super::{
	DocumentManager, DocumentTab, dialogs, ensure_parser_ready_for_path, menu, menu_ids, resolve_zip_path,
	update_title_from_manager,
};
use crate::ui::navigation::announce;

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
		announce(live_region_label, t("No recent documents."));
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
	let mut opened_any = false;
	for path in &result.open {
		let Some(path) = resolve_zip_path(frame, Path::new(path), config, false) else {
			continue;
		};
		// One book that cannot be opened, or whose unknown type the reader declines to pick a reader for, does not stop the rest.
		if ensure_parser_ready_for_path(frame, &path, config) && dm.lock().unwrap().open_file(dm, &path) {
			opened_any = true;
		}
	}
	if opened_any {
		let dm_ref = dm.lock().unwrap();
		update_title_from_manager(frame, &dm_ref);
		dm_ref.focus_document_text();
	}
	let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
	frame.set_menu_bar(menu_bar);
	let dm_ref = dm.lock().unwrap();
	let has_docs = dm_ref.tab_count() > 0;
	let has_reopen = dm_ref.has_recently_closed();
	drop(dm_ref);
	menu::update_menu_item_states(frame, has_docs);
	menu::update_reopen_state(frame, has_reopen);
}

pub(super) fn handle_export_to_plain_text(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	export_document_as(
		frame,
		tab,
		ExportFormat::Text,
		"txt",
		// TRANSLATORS: File filter shown in the "Export to plain text" save dialog
		&t("Plain text files (*.txt)|*.txt|All files (*.*)|*.*"),
		// TRANSLATORS: Title of the file save dialog when exporting a document to plain text
		&t("Export document to plain text"),
	);
}

pub(super) fn handle_export_to_html(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	export_document_as(
		frame,
		tab,
		ExportFormat::Html,
		"html",
		// TRANSLATORS: File filter shown in the "Export to HTML" save dialog
		&t("HTML files (*.html)|*.html|All files (*.*)|*.*"),
		// TRANSLATORS: Title of the file save dialog when exporting a document to HTML
		&t("Export document to HTML"),
	);
}

pub(super) fn handle_export_to_markdown(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	export_document_as(
		frame,
		tab,
		ExportFormat::Markdown,
		"md",
		// TRANSLATORS: File filter shown in the "Export to Markdown" save dialog
		&t("Markdown files (*.md)|*.md|All files (*.*)|*.*"),
		// TRANSLATORS: Title of the file save dialog when exporting a document to Markdown
		&t("Export document to Markdown"),
	);
}

pub(super) fn handle_export_document_data(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	let default_name =
		// TRANSLATORS: Fallback file name stem used when the document's path has no file stem
		tab.file_path.file_stem().map_or_else(|| t("document"), |s| s.to_string_lossy().to_string());
	let default_file = format!("{default_name}.paperback");
	// TRANSLATORS: File filter shown in the export/import notes-and-bookmarks (.paperback) dialogs
	let wildcard = t("Paperback files (*.paperback)|*.paperback");
	let dialog = FileDialog::builder(frame)
		// TRANSLATORS: Title of the file save dialog when exporting a document's notes and bookmarks
		.with_message(&t("Export notes and bookmarks"))
		.with_default_file(&default_file)
		.with_wildcard(&wildcard)
		.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
		.build();
	if dialog.show_modal() == ID_OK
		&& let Some(path) = dialog.get_path()
	{
		let path_str = tab.file_path.to_string_lossy();
		config.lock().unwrap().export_document_settings(&path_str, &path);
		tracing::info!(doc = %tab.file_path.display(), export = %path, "document data exported");
		let dialog = MessageDialog::builder(
			frame,
			// TRANSLATORS: Success message shown after exporting a document's notes and bookmarks
			&t("Notes and bookmarks exported successfully."),
			// TRANSLATORS: Title of the export-succeeded dialog
			&t("Export Successful"),
		)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
		.build();
		dialog.show_modal();
	}
}

pub(super) fn handle_import_document_data(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	// TRANSLATORS: File filter shown in the export/import notes-and-bookmarks (.paperback) dialogs
	let wildcard = t("Paperback files (*.paperback)|*.paperback");
	let dialog = FileDialog::builder(frame)
		// TRANSLATORS: Title of the file open dialog when importing a document's notes and bookmarks
		.with_message(&t("Import notes and bookmarks"))
		.with_wildcard(&wildcard)
		.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
		.build();
	if dialog.show_modal() == ID_OK
		&& let Some(path) = dialog.get_path()
	{
		let path_str = tab.file_path.to_string_lossy();
		let pos = {
			let config = config.lock().unwrap();
			config.import_settings_from_file(&path_str, &path);
			let max_pos = tab.text_ctrl.get_last_position();
			config.get_validated_document_position(&path_str, max_pos)
		};
		tracing::info!(doc = %tab.file_path.display(), import = %path, "document data imported");
		if pos >= 0 {
			tab.text_ctrl.set_insertion_point(pos);
			tab.text_ctrl.show_position(pos);
		}
		let dialog = MessageDialog::builder(
			frame,
			// TRANSLATORS: Success message shown after importing a document's notes and bookmarks
			&t("Notes and bookmarks imported successfully."),
			// TRANSLATORS: Title of the import-succeeded dialog
			&t("Import Successful"),
		)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
		.build();
		dialog.show_modal();
	}
}

/// Prompts for a save path and exports `tab`'s document as `format`, showing a
/// generic failure dialog on error. Shared by the `EXPORT_TO_PLAIN_TEXT` /
/// `EXPORT_TO_HTML` / `EXPORT_TO_MARKDOWN` menu handlers, which differ only in
/// `format`, the default file `extension`, the file-picker `wildcard`, and the
/// file-picker `dialog_title`.
fn export_document_as(
	frame: &Frame,
	tab: &DocumentTab,
	format: ExportFormat,
	extension: &str,
	wildcard: &str,
	dialog_title: &str,
) {
	let default_name =
		// TRANSLATORS: Fallback file name stem used when the document's path has no file stem
		tab.file_path.file_stem().map_or_else(|| t("document"), |s| s.to_string_lossy().to_string());
	let default_file = format!("{default_name}.{extension}");
	let dialog = FileDialog::builder(frame)
		.with_message(dialog_title)
		.with_default_file(&default_file)
		.with_wildcard(wildcard)
		.with_style(FileDialogStyle::Save | FileDialogStyle::OverwritePrompt)
		.build();
	if dialog.show_modal() == ID_OK
		&& let Some(path) = dialog.get_path()
		&& let Err(e) = tab.session.export_as(&path, format)
	{
		tracing::error!(path = %path, error = %e, format = ?format, "failed to export document");
		let dialog =
			// TRANSLATORS: Error dialog shown when exporting a document to another format fails
			MessageDialog::builder(frame, &t("Failed to export document."), &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
				.build();
		dialog.show_modal();
	}
}
