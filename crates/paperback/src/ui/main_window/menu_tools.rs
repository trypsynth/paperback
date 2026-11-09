#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, document::DocumentStats};
use patois::t;
use wxdragon::prelude::*;

use super::{DocumentManager, build_font_from_readability, dialogs, menu, update_title_from_manager};
#[cfg(target_os = "windows")]
use super::{HotkeyHandle, re_register_hotkey};
use crate::{translation_manager::TranslationManager, ui::navigation::announce_for_command};

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

/// One item both starts and stops a batch, so nothing needs enabling or disabling as a job starts and ends.
pub(super) fn handle_batch_ocr(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
	let (max_page, running) = {
		let dm_ref = dm.lock().unwrap();
		let Some(tab) = dm_ref.active_tab() else {
			// TRANSLATORS: Announced when Batch OCR is chosen with no document open
			announce_for_command(live_region_label, from_keyboard, t("No document open."));
			return;
		};
		(i32::try_from(tab.session.page_count()).unwrap_or(i32::MAX), dm_ref.batch_ocr_running())
	};
	if running {
		if wx_utils::confirm(
			frame,
			// TRANSLATORS: Confirmation asked when Batch OCR is chosen while a batch is already running
			t("Batch OCR is running. Stop it?"),
			// TRANSLATORS: Title of the Batch OCR dialog
			&t("Batch OCR"),
		) {
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
	let old = dialogs::OptionsDialogResult::load(&config.lock().unwrap());
	let cfg = config.lock().unwrap();
	options.save(&cfg);
	logkeeper::set_level(options.advanced.log_level);
	cfg.flush();
	tracing::info!("settings saved");
	#[cfg(target_os = "windows")]
	{
		re_register_hotkey(hotkey_handle, options.general.hotkey.as_ref());
	}
	drop(cfg);
	let (was, look) = (&old.appearance, &options.appearance);
	let font_changed = was.readability_font != look.readability_font;
	let bg_color_changed = was.bg_color != look.bg_color;
	let needs_rebuild = was.word_wrap != look.word_wrap
		|| (font_changed && build_font_from_readability(&look.readability_font).is_none())
		|| (bg_color_changed && look.bg_color < 0)
		|| (font_changed && look.readability_font.color < 0);
	if needs_rebuild {
		let dm_for_wrap = Rc::clone(dm);
		let mut dm_ref = dm.lock().unwrap();
		dm_ref.apply_word_wrap(&dm_for_wrap, look.word_wrap);
		dm_ref.restore_focus();
	} else {
		let dm_ref = dm.lock().unwrap();
		if font_changed {
			if let Some(font) = build_font_from_readability(&look.readability_font) {
				dm_ref.apply_font(&font);
			}
			dm_ref.apply_color(look.readability_font.color);
		}
		if bg_color_changed {
			dm_ref.apply_bg_color(look.bg_color);
		}
		if was.line_spacing != look.line_spacing {
			dm_ref.apply_line_spacing(look.line_spacing);
		}
		if was.text_alignment != look.text_alignment {
			dm_ref.apply_text_alignment(look.text_alignment);
		}
		if was.letter_spacing != look.letter_spacing {
			dm_ref.apply_letter_spacing(look.letter_spacing);
		}
		if was.paragraph_spacing != look.paragraph_spacing {
			dm_ref.apply_paragraph_spacing(look.paragraph_spacing);
		}
	}
	if old.documents != options.documents {
		dm.lock().unwrap().apply_parse_settings(options.documents.parse_settings());
	}
	let language_changed = current_language != options.general.language;
	if language_changed || old.reading.compact_go_menu != options.reading.compact_go_menu {
		if language_changed {
			let _ = TranslationManager::instance().lock().unwrap().set_language(&options.general.language);
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
	#[cfg(target_os = "windows")] hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>,
) {
	let initial = {
		let cfg = config.lock().unwrap();
		dialogs::Keymap { shortcuts: cfg.get_shortcuts(), window_hotkey: cfg.get_hotkey() }
	};
	if let Some(updated) = dialogs::prompt_for_shortcuts(frame, &initial) {
		{
			let cfg = config.lock().unwrap();
			cfg.set_shortcuts(&updated.shortcuts);
			cfg.set_hotkey(updated.window_hotkey.as_ref());
			cfg.flush();
		}
		#[cfg(target_os = "windows")]
		if updated.window_hotkey != initial.window_hotkey {
			re_register_hotkey(hotkey_handle, updated.window_hotkey.as_ref());
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
