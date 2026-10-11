//! The Tools menu's heavier handlers: word/document info, batch OCR, the Options dialog (and
//! applying whatever changed), and shortcut customization.

#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{rc::Rc, sync::Mutex};

use paperback_core::{
	config::ConfigManager,
	document::{DocumentStats, ParseSettings},
};
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, build_font_from_readability, dialogs, menu, update_title_from_manager};
#[cfg(target_os = "windows")]
use super::{HotkeyHandle, re_register_hotkey};
use crate::{
	config_ext::{set_log_level, set_update_channel},
	translation_manager::TranslationManager,
	ui::navigation::announce,
};

pub(super) fn handle_word_count(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
	let Ok(dm_ref) = dm.try_lock() else {
		return;
	};
	if let Some(tab) = dm_ref.active_tab() {
		let selection = tab.text_ctrl.get_string_selection();
		let is_selection = !selection.trim().is_empty();
		let word_count =
			if is_selection { DocumentStats::from_text(&selection).word_count } else { tab.session.stats().word_count };
		let audio_only = (!is_selection
			&& tab.session.handle().document().audio_only
			&& tab.session.audio().is_some_and(|timeline| !timeline.is_empty()))
		.then(|| {
			let stats = tab.session.stats();
			dialogs::AudioOnlySummary {
				file_count: stats.audio_file_count,
				total_duration_ms: stats.audio_total_duration_ms,
			}
		});
		let wpm = config.lock().unwrap().get_app_int("reading_speed_wpm", 150);
		dialogs::show_word_count_dialog(frame, word_count, wpm, is_selection, audio_only);
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

/// Opens the Batch OCR range dialog, or offers to stop the batch that is already running. One
/// menu item covers both so there is nothing to enable and disable as a job starts and ends, and
/// the answer to "how do I stop this" is the same item that started it.
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(super) fn handle_batch_ocr(frame: &Frame, dm: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText) {
	let (max_page, running) = {
		let dm_ref = dm.lock().unwrap();
		let Some(tab) = dm_ref.active_tab() else {
			// TRANSLATORS: Announced when Batch OCR is chosen with no document open
			announce(live_region_label, t("No document open."));
			return;
		};
		(i32::try_from(tab.session.page_count()).unwrap_or(i32::MAX), dm_ref.batch_ocr_running())
	};
	if running {
		let confirm = MessageDialog::builder(
			frame,
			// TRANSLATORS: Confirmation asked when Batch OCR is chosen while a batch is already running
			&t("Batch OCR is running. Stop it?"),
			// TRANSLATORS: Title of the Batch OCR dialog
			&t("Batch OCR"),
		)
		.with_style(MessageDialogStyle::YesNo | MessageDialogStyle::IconQuestion)
		.build();
		if confirm.show_modal() == ID_YES {
			dm.lock().unwrap().cancel_ocr(false);
		}
		return;
	}
	if let Some(range) = dialogs::show_batch_ocr_dialog(frame, max_page) {
		dm.lock().unwrap().start_batch_ocr(range.start, range.end, range.include_text_pages);
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
		old_join_pdf_paragraphs,
		old_strip_running_text,
		old_markdown_dollar_math,
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
			cfg.get_app_bool("join_pdf_paragraphs", true),
			cfg.get_app_bool("strip_running_text", true),
			cfg.get_app_bool("markdown_dollar_math", true),
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
	cfg.set_app_bool("join_pdf_paragraphs", options.join_pdf_paragraphs);
	cfg.set_app_bool("strip_running_text", options.strip_running_text);
	cfg.set_app_bool("markdown_dollar_math", options.markdown_dollar_math);
	cfg.set_app_bool("minimize_to_tray", options.minimize_to_tray);
	cfg.set_app_bool("start_maximized", options.start_maximized);
	cfg.set_app_bool("compact_go_menu", options.compact_go_menu);
	cfg.set_app_bool("navigation_wrap", options.navigation_wrap);
	cfg.set_app_bool("line_start_navigation", options.line_start_navigation);
	// Read at each Find rather than applied here: nothing on screen changes until the next
	// search, and a match already selected is left as it is rather than cleared out from under
	// the reader as the dialog closes.
	cfg.set_app_bool("highlight_found_text", options.highlight_found_text);
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
	set_log_level(&cfg, options.log_level);
	crate::logging::set_level(options.log_level);
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
	let parse_settings_changed = old_render_tables_inline != options_render_tables_inline
		|| old_join_pdf_paragraphs != options.join_pdf_paragraphs
		|| old_strip_running_text != options.strip_running_text
		|| old_markdown_dollar_math != options.markdown_dollar_math;
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
	if parse_settings_changed {
		let settings = ParseSettings {
			render_tables_inline: options_render_tables_inline,
			join_pdf_paragraphs: options.join_pdf_paragraphs,
			strip_running_text: options.strip_running_text,
			markdown_dollar_math: options.markdown_dollar_math,
		};
		let mut dm_ref = dm.lock().unwrap();
		dm_ref.apply_parse_settings(settings);
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
