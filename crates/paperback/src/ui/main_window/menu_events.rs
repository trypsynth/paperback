//! Binds the main frame's menu events: the big `on_menu` dispatcher that routes each menu id to
//! its handler, plus the recurring background timers set up alongside it.

#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::{prelude::*, timer::Timer};

#[cfg(target_os = "windows")]
use super::HotkeyHandle;
use super::{
	DocumentManager, FindDialogState, MainWindow, background, commands, dialogs, find, get_update_channel, help, menu,
	menu_file, menu_go, menu_ids, menu_tools, sleep_timer, update, update_title_from_manager,
};

impl MainWindow {
	#[allow(clippy::too_many_lines)]
	pub(super) fn bind_menu_events(
		frame: &Frame,
		doc_manager: &Rc<Mutex<DocumentManager>>,
		config: &Rc<Mutex<ConfigManager>>,
		find_dialog: &Rc<Mutex<Option<FindDialogState>>>,
		live_region_label: StaticText,
		#[cfg(target_os = "windows")] hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>,
	) -> Vec<Rc<Timer<Frame>>> {
		let frame_copy = *frame;
		let dm = Rc::clone(doc_manager);
		let config = Rc::clone(config);
		let find_dialog = Rc::clone(find_dialog);
		#[cfg(target_os = "windows")]
		let hotkey_handle_for_options = Rc::clone(hotkey_handle);
		let sleep_timer = sleep_timer::SleepTimer::new(frame, doc_manager, &config);
		// Taken before the menu closure captures the SleepTimer: the returned vec is what
		// keeps the wx timer alive for the window's lifetime.
		let sleep_timer_handle = Rc::clone(sleep_timer.timer());
		let mut timers = background::start_timers(frame, doc_manager);
		timers.push(sleep_timer_handle);
		background::bind_resize(frame, doc_manager);
		frame.on_menu(move |event| {
			let id = event.get_id();
			// Commands that have moved to the table handle themselves; the match below is the
			// shrinking remainder, still keyed to menu ids by hand.
			if commands::dispatch(
				id,
				&commands::Ctx { frame: &frame_copy, dm: &dm, config: &config, live_region_label },
			) {
				return;
			}
			match id {
				menu_ids::FIND => {
					find::show_find_dialog(&frame_copy, &dm, &config, &find_dialog, live_region_label);
				}
				menu_ids::FIND_NEXT => {
					find::handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, true);
				}
				menu_ids::FIND_PREVIOUS => {
					find::handle_find_action(&frame_copy, &dm, &config, &find_dialog, live_region_label, false);
				}
				menu_ids::ANNOUNCE_PERCENT => {
					if let Ok(dm_ref) = dm.try_lock() {
						dm_ref.announce_current_percent();
					}
				}
				menu_ids::SET_TEMPORARY_BOOKMARK => {
					if let Ok(dm_ref) = dm.try_lock() {
						dm_ref.set_temporary_bookmark();
					}
				}
				menu_ids::JUMP_TO_TEMPORARY_BOOKMARK => {
					if let Ok(mut dm_ref) = dm.try_lock() {
						dm_ref.jump_to_temporary_bookmark();
					}
				}
				menu_ids::GO_TO_LINE => {
					menu_go::handle_go_to_line(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::GO_TO_PAGE => {
					menu_go::handle_go_to_page(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::GO_TO_PERCENT => {
					menu_go::handle_go_to_percent(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::TOGGLE_WORD_WRAP => {
					let new_state = {
						let cfg = config.lock().unwrap();
						let v = !cfg.get_app_bool("word_wrap", false);
						cfg.set_app_bool("word_wrap", v);
						cfg.flush();
						v
					};
					{
						let dm_for_wrap = Rc::clone(&dm);
						let mut dm_ref = dm.lock().unwrap();
						dm_ref.apply_word_wrap(&dm_for_wrap, new_state);
					}
					if let Some(menu_bar) = frame_copy.get_menu_bar() {
						menu_bar.check_item(menu_ids::TOGGLE_WORD_WRAP, new_state);
					}
					// TRANSLATORS: Announced when toggling word wrap; the message reflects the new state
					let msg = if new_state { t("Word wrap on.") } else { t("Word wrap off.") };
					live_region::announce(live_region_label, &msg);
					dm.lock().unwrap().restore_focus();
				}
				menu_ids::TOGGLE_FULL_SCREEN => {
					let new_state = !frame_copy.is_full_screen();
					frame_copy.show_full_screen(new_state);
					if let Some(menu_bar) = frame_copy.get_menu_bar() {
						menu_bar.check_item(menu_ids::TOGGLE_FULL_SCREEN, new_state);
					}
					// TRANSLATORS: Announced when toggling full screen mode; the message reflects the new state
					let msg = if new_state { t("Full screen on.") } else { t("Full screen off.") };
					live_region::announce(live_region_label, &msg);
				}
				menu_ids::EXPORT_TO_PLAIN_TEXT => {
					menu_tools::handle_export_to_plain_text(&frame_copy, &dm);
				}
				menu_ids::EXPORT_TO_HTML => {
					menu_tools::handle_export_to_html(&frame_copy, &dm);
				}
				menu_ids::EXPORT_TO_MARKDOWN => {
					menu_tools::handle_export_to_markdown(&frame_copy, &dm);
				}
				menu_ids::EXPORT_DOCUMENT_DATA => {
					menu_tools::handle_export_document_data(&frame_copy, &dm, &config);
				}
				menu_ids::IMPORT_DOCUMENT_DATA => {
					menu_tools::handle_import_document_data(&frame_copy, &dm, &config);
				}
				menu_ids::WORD_COUNT => {
					menu_tools::handle_word_count(&frame_copy, &dm, &config);
				}
				menu_ids::DOCUMENT_INFO => {
					menu_tools::handle_document_info(&frame_copy, &dm);
				}
				menu_ids::TABLE_OF_CONTENTS => {
					menu_tools::handle_table_of_contents(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::ELEMENTS_LIST => {
					menu_tools::handle_elements_list(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::OPEN_IN_WEB_VIEW => {
					menu_tools::handle_open_in_web_view(&frame_copy, &dm);
				}
				menu_ids::REVEAL_FILE_IN_FOLDER => {
					help::handle_reveal_file_in_folder(&frame_copy, &dm);
				}
				menu_ids::VIEW_SOURCE => {
					menu_tools::handle_view_source(&frame_copy, &dm);
				}
				menu_ids::OPTIONS | menu_ids::PREFERENCES => {
					menu_tools::handle_options(
						&frame_copy,
						&dm,
						&config,
						#[cfg(target_os = "windows")]
						&hotkey_handle_for_options,
					);
				}
				menu_ids::CUSTOMIZE_SHORTCUTS => {
					menu_tools::handle_customize_shortcuts(&frame_copy, &dm, &config);
				}
				menu_ids::SLEEP_TIMER => {
					sleep_timer.toggle(&frame_copy, &dm, &config, live_region_label);
				}
				#[cfg(any(target_os = "windows", target_os = "macos"))]
				menu_ids::BATCH_OCR => {
					menu_tools::handle_batch_ocr(&frame_copy, &dm, live_region_label);
				}
				menu_ids::ABOUT => {
					dialogs::show_about_dialog(&frame_copy);
				}
				menu_ids::VIEW_HELP_BROWSER => {
					help::handle_view_help_browser(&frame_copy);
				}
				menu_ids::VIEW_HELP_PAPERBACK => {
					if help::handle_view_help_paperback(&frame_copy, &dm, &config) {
						{
							let dm_ref = dm.lock().unwrap();
							update_title_from_manager(&frame_copy, &dm_ref);
							dm_ref.restore_focus();
						}
						let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
						frame_copy.set_menu_bar(menu_bar);
						menu::update_menu_item_states(&frame_copy, true);
						let has_reopen = dm.lock().unwrap().has_recently_closed();
						menu::update_reopen_state(&frame_copy, has_reopen);
					}
				}
				menu_ids::CHECK_FOR_UPDATES => {
					let channel = get_update_channel(&config.lock().unwrap());
					update::run_update_check(false, channel);
				}
				menu_ids::DONATE => {
					help::handle_donate(&frame_copy);
				}
				#[cfg(target_os = "macos")]
				menu_ids::COPY => {
					// Only macOS builds an Edit menu, so only macOS sees Copy as a menu event; every
					// other platform intercepts the key in `build_text_ctrl`.
					let widened = dm.lock().unwrap().copy_whole_document_if_all_selected();
					if !widened {
						event.skip(true);
					}
				}
				_ => {
					menu_file::handle_fallback(id, &frame_copy, &dm, &config, live_region_label);
				}
			}
		});
		timers
	}
}
