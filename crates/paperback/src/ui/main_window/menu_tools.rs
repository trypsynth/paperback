//! The Tools menu's heavier handlers: word/document info, TOC/elements dialogs, web view and
//! source view, the Options dialog (and applying whatever changed), shortcut customization,
//! sleep timer, and document import/export.

#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{
	cell::Cell,
	env,
	path::Path,
	rc::Rc,
	sync::{Mutex, atomic::Ordering},
	time::{SystemTime, UNIX_EPOCH},
};

use paperback_core::{config::ConfigManager, document::DocumentStats, export::ExportFormat, session::SourceView};
use patois::{nt, t};
use wxdragon::{prelude::*, timer::Timer};

use super::{
	DocumentManager, MainWindow, SLEEP_TIMER_DURATION_MINUTES, SLEEP_TIMER_START_MS, build_font_from_readability,
	dialogs, menu, navigation, update_title_from_manager,
};
#[cfg(target_os = "windows")]
use super::{HotkeyHandle, re_register_hotkey};
use crate::{config_ext::set_update_channel, translation_manager::TranslationManager};

pub(super) fn handle_word_count(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	if let Some(tab) = dm_ref.active_tab() {
		let selection = tab.text_ctrl.get_string_selection();
		let (word_count, is_selection) = if selection.trim().is_empty() {
			(tab.session.stats().word_count, false)
		} else {
			(DocumentStats::from_text(&selection).word_count, true)
		};
		let wpm = config.lock().unwrap().get_app_int("reading_speed_wpm", 150);
		dialogs::show_word_count_dialog(frame, word_count, wpm, is_selection);
	}
}

pub(super) fn handle_document_info(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	if let Some(tab) = dm_ref.active_tab() {
		let stats = tab.session.stats();
		let title = tab.session.title();
		let author = tab.session.author();
		dialogs::show_document_info_dialog(frame, &tab.file_path, &title, &author, stats);
	}
}

pub(super) fn handle_table_of_contents(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
) {
	let mut dm_guard = dm.lock().unwrap();
	if let Some(tab) = dm_guard.active_tab_mut() {
		let toc_items = &tab.session.handle().document().toc_items;
		if toc_items.is_empty() {
			// TRANSLATORS: Announced when opening the Table of Contents for a document that has none
			live_region::announce(live_region_label, &t("No table of contents."));
			return;
		}
		let current_pos = navigation::doc_caret(tab);
		let current_pos_usize = usize::try_from(current_pos).unwrap_or(0);
		let current_toc_offset = tab.session.handle().find_closest_toc_offset(current_pos_usize);
		if let Some(offset) =
			dialogs::show_toc_dialog(frame, toc_items, i32::try_from(current_toc_offset).unwrap_or(i32::MAX))
		{
			let update = navigation::move_to_offset_and_record_history(tab, i64::from(offset));
			navigation::persist_navigation_history(config, Some(&update));
		}
	}
}

pub(super) fn handle_elements_list(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
	let mut dm_guard = dm.lock().unwrap();
	if let Some(tab) = dm_guard.active_tab_mut() {
		let current_pos = navigation::doc_caret(tab);
		if let Some(offset) = dialogs::show_elements_dialog(frame, &tab.session, current_pos) {
			let update = navigation::move_to_offset_and_record_history(tab, offset);
			navigation::persist_navigation_history(config, Some(&update));
		}
	}
}

pub(super) fn handle_open_in_web_view(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	let current_pos = navigation::doc_caret(tab);
	let temp_dir = env::temp_dir().to_string_lossy().to_string();
	if let Some(target) = tab.session.webview_target_path(current_pos, &temp_dir) {
		let mut url = format!("file:///{}", target.path.replace('\\', "/"));
		let fragment = target.fragment.or_else(|| tab.session.webview_fragment_for_position(current_pos));
		if let Some(fragment) = fragment {
			url.push('#');
			url.push_str(&fragment);
		}
		drop(dm_ref);
		dialogs::show_web_view_dialog(
			frame,
			// TRANSLATORS: Title of the window that renders a document's content as HTML (e.g. for embedded web pages)
			&t("Web View"),
			&url,
			true,
			Some(Box::new(|url| {
				if url.to_lowercase().starts_with("http://")
					|| url.to_lowercase().starts_with("https://")
					|| url.to_lowercase().starts_with("mailto:")
				{
					launch_default_browser(url, BrowserLaunchFlags::Default);
					false
				} else {
					true
				}
			})),
		);
	} else {
		tracing::warn!(path = %tab.file_path.display(), "could not determine web view content");
		let dialog = MessageDialog::builder(
			frame,
			// TRANSLATORS: Error shown when the document has no content that can be rendered in the Web View
			&t("Could not determine content to display in Web View."),
			// TRANSLATORS: Generic error dialog title
			&t("Error"),
		)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
		dialog.show_modal();
	}
}

pub(super) fn handle_view_source(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	// `None` => format has no text source; `Some(None)` => source could not
	// be loaded; `Some(Some(..))` => source ready. Locks are dropped before
	// any dialog is shown.
	let outcome: Option<Option<(SourceView, String)>> = {
		let Ok(dm_ref) = dm.try_lock() else {
			return;
		};
		let Some(tab) = dm_ref.active_tab() else {
			return;
		};
		if tab.session.source_view_available() {
			let current_pos = navigation::doc_caret(tab);
			let orig_name = tab
				.file_path
				.file_name()
				// TRANSLATORS: Fallback file name stem used when the document's path has no file stem
				.map_or_else(|| t("document"), |name| name.to_string_lossy().to_string());
			let temp_dir = env::temp_dir().to_string_lossy().to_string();
			Some(tab.session.view_source(current_pos, &temp_dir).map(|view| (view, orig_name)))
		} else {
			None
		}
	};
	match outcome {
		Some(Some((view, orig_name))) => {
			// TRANSLATORS: Prefix before the file name in the tab title for a "View Source" tab, e.g. "Source: book.epub"
			let title = format!("{} {orig_name}", t("Source:"));
			let opened = dm.lock().unwrap().open_source_file(dm, Path::new(&view.path), &title);
			if opened {
				let dm_ref = dm.lock().unwrap();
				if let Some(tab) = dm_ref.active_tab() {
					tab.text_ctrl.set_insertion_point(view.caret);
					tab.text_ctrl.show_position(view.caret);
				}
			}
		}
		unavailable => {
			let message = if unavailable.is_none() {
				tracing::debug!("source view not available for this format");
				// TRANSLATORS: Error shown when "View Source" is used on a document format that has no raw source to view
				t("Source view is not available for this document format.")
			} else {
				tracing::warn!("failed to load document source for view source");
				// TRANSLATORS: Error shown when "View Source" fails to load the document's underlying source
				t("Could not load the document source.")
			};
			// TRANSLATORS: Generic error dialog title
			let dialog = MessageDialog::builder(frame, &message, &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
				.build();
			dialog.show_modal();
		}
	}
}

pub(super) fn handle_export_to_plain_text(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	MainWindow::export_document_as(
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
	MainWindow::export_document_as(
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
	MainWindow::export_document_as(
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

pub(super) fn handle_options(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	#[cfg(target_os = "windows")] hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>,
) {
	let current_language = TranslationManager::instance().lock().unwrap().current_language();
	let options = {
		let cfg = config.lock().unwrap();
		dialogs::show_options_dialog(frame, &cfg)
	};
	let Some(options) = options else {
		return;
	};
	let (
		old_word_wrap,
		old_render_tables_inline,
		old_compact_menu,
		old_readability_font,
		old_line_spacing,
		old_bg_color,
		old_text_alignment,
		old_letter_spacing,
		old_paragraph_spacing,
	) = {
		let cfg = config.lock().unwrap();
		(
			cfg.get_app_bool("word_wrap", false),
			cfg.get_app_bool("render_tables_inline", true),
			cfg.get_app_bool("compact_go_menu", true),
			cfg.get_readability_font(),
			cfg.get_line_spacing(),
			cfg.get_bg_color(),
			cfg.get_text_alignment(),
			cfg.get_letter_spacing(),
			cfg.get_paragraph_spacing(),
		)
	};
	let cfg = config.lock().unwrap();
	cfg.set_app_bool("restore_previous_documents", options.restore_previous_documents);
	cfg.set_app_bool("word_wrap", options.word_wrap);
	cfg.set_app_bool("render_tables_inline", options.render_tables_inline);
	cfg.set_app_bool("minimize_to_tray", options.minimize_to_tray);
	cfg.set_app_bool("start_maximized", options.start_maximized);
	cfg.set_app_bool("compact_go_menu", options.compact_go_menu);
	cfg.set_app_bool("navigation_wrap", options.navigation_wrap);
	cfg.set_app_bool("line_start_navigation", options.line_start_navigation);
	cfg.set_app_bool("check_for_updates_on_startup", options.check_for_updates_on_startup);
	cfg.set_app_bool("bookmark_sounds", options.bookmark_sounds);
	cfg.set_app_bool("sync_caret_to_audio", options.sync_caret_to_audio);
	cfg.set_app_int("audio_seek_amount_seconds", options.audio_seek_amount_seconds);
	cfg.set_app_bool("audio_seek_continues_into_next_file", options.audio_seek_continues_into_next_file);
	cfg.set_app_bool("auto_reload_documents", options.auto_reload_documents);
	cfg.set_app_int("recent_documents_to_show", options.recent_documents_to_show);
	cfg.set_app_int("reading_speed_wpm", options.reading_speed_wpm);
	cfg.set_app_string("language", &options.language);
	set_update_channel(&cfg, options.update_channel);
	cfg.set_hotkey(&options.hotkey);
	cfg.set_shortcuts(&options.shortcuts);
	cfg.set_readability_font(&options.readability_font);
	cfg.set_line_spacing(options.line_spacing);
	cfg.set_bg_color(options.bg_color);
	cfg.set_text_alignment(options.text_alignment);
	cfg.set_letter_spacing(options.letter_spacing);
	cfg.set_paragraph_spacing(options.paragraph_spacing);
	cfg.flush();
	tracing::info!("settings saved");
	#[cfg(target_os = "windows")]
	{
		re_register_hotkey(hotkey_handle, &options.hotkey);
	}
	drop(cfg);
	let options_word_wrap = options.word_wrap;
	let options_render_tables_inline = options.render_tables_inline;
	let render_tables_inline_changed = old_render_tables_inline != options_render_tables_inline;
	let font_changed = old_readability_font != options.readability_font;
	let line_spacing_changed = old_line_spacing != options.line_spacing;
	let bg_color_changed = old_bg_color != options.bg_color;
	let text_alignment_changed = old_text_alignment != options.text_alignment;
	let letter_spacing_changed = old_letter_spacing != options.letter_spacing;
	let paragraph_spacing_changed = old_paragraph_spacing != options.paragraph_spacing;
	let needs_rebuild = old_word_wrap != options_word_wrap
		|| (font_changed && build_font_from_readability(&options.readability_font).is_none())
		|| (bg_color_changed && options.bg_color < 0)
		|| (font_changed && options.readability_font.color < 0);
	if needs_rebuild {
		let dm_for_wrap = Rc::clone(dm);
		let mut dm_ref = dm.lock().unwrap();
		dm_ref.apply_word_wrap(&dm_for_wrap, options_word_wrap);
		dm_ref.restore_focus();
	} else {
		let dm_ref = dm.lock().unwrap();
		if font_changed {
			if let Some(font) = build_font_from_readability(&options.readability_font) {
				dm_ref.apply_font(&font);
			}
			dm_ref.apply_color(options.readability_font.color);
		}
		if bg_color_changed {
			dm_ref.apply_bg_color(options.bg_color);
		}
		if line_spacing_changed {
			dm_ref.apply_line_spacing(options.line_spacing);
		}
		if text_alignment_changed {
			dm_ref.apply_text_alignment(options.text_alignment);
		}
		if letter_spacing_changed {
			dm_ref.apply_letter_spacing(options.letter_spacing);
		}
		if paragraph_spacing_changed {
			dm_ref.apply_paragraph_spacing(options.paragraph_spacing);
		}
	}
	if render_tables_inline_changed {
		let mut dm_ref = dm.lock().unwrap();
		dm_ref.apply_render_tables_inline(options_render_tables_inline);
	}
	let options_compact_menu = options.compact_go_menu;
	if current_language != options.language || old_compact_menu != options_compact_menu {
		if current_language != options.language {
			let _ = TranslationManager::instance().lock().unwrap().set_language(&options.language);
		}
		let dm_ref = dm.lock().unwrap();
		update_title_from_manager(frame, &dm_ref);
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

pub(super) fn handle_customize_shortcuts(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let initial_shortcuts = config.lock().unwrap().get_shortcuts();
	if let Some(updated) = dialogs::prompt_for_shortcuts(frame, &initial_shortcuts) {
		{
			let cfg = config.lock().unwrap();
			cfg.set_shortcuts(&updated);
			cfg.flush();
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
}

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_sleep_timer(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	sleep_timer: &Rc<Timer<Frame>>,
	sleep_timer_running: &Rc<Cell<bool>>,
	sleep_timer_start_time: &Rc<Cell<i64>>,
	sleep_timer_duration_minutes: &Rc<Cell<i32>>,
) {
	if sleep_timer_running.get() {
		sleep_timer.stop();
		sleep_timer_running.set(false);
		sleep_timer_start_time.set(0);
		sleep_timer_duration_minutes.set(0);
		SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
		SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
		tracing::info!("sleep timer cancelled");
		let dm_ref = dm.lock().unwrap();
		update_title_from_manager(frame, &dm_ref);
		// TRANSLATORS: Announced when the user cancels a running sleep timer
		live_region::announce(live_region_label, &t("Sleep timer cancelled."));
		return;
	}
	let initial_duration = config.lock().unwrap().get_app_int("sleep_timer_duration", 30);
	if let Some(duration) = dialogs::show_sleep_timer_dialog(frame, initial_duration) {
		{
			let cfg = config.lock().unwrap();
			cfg.set_app_int("sleep_timer_duration", duration);
			cfg.flush();
		}
		let duration_ms = u64::try_from(duration).unwrap_or(0) * 60 * 1000;
		sleep_timer.start(i32::try_from(duration_ms).unwrap_or(i32::MAX), true);
		sleep_timer_running.set(true);
		tracing::info!(duration_minutes = duration, "sleep timer started");
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.ok()
			.and_then(|d| i64::try_from(d.as_millis()).ok())
			.unwrap_or(0);
		sleep_timer_start_time.set(now);
		sleep_timer_duration_minutes.set(duration);
		SLEEP_TIMER_START_MS.store(now, Ordering::SeqCst);
		SLEEP_TIMER_DURATION_MINUTES.store(duration, Ordering::SeqCst);
		// TRANSLATORS: Announcement when the sleep timer is set. The %d placeholder is replaced with the number of minutes.
		let msg = nt(
			"Sleep timer set for %d minute.",
			"Sleep timer set for %d minutes.",
			u64::try_from(duration).unwrap_or(0),
		)
		.replacen("%d", &duration.to_string(), 1);
		live_region::announce(live_region_label, &msg);
	}
}
