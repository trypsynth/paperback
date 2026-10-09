#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{cell::Cell, path::Path, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::{prelude::*, timer::Timer};

#[cfg(target_os = "windows")]
use super::tray;
use super::{
	background, commands, dialogs,
	document_manager::{DocumentManager, DocumentTab, display_title, tab_index_for_key},
	find::{self, FindDialogState},
	help, icon, menu, menu_ids, navigation,
	readability::build_font_from_readability,
	sleep_timer, status, window_geometry,
};
use crate::{
	config_ext::{UpdateChannel, get_update_channel},
	updater,
};

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod ipc;
#[cfg(not(target_os = "macos"))]
mod menu_edit;
mod menu_events;
mod menu_file;
mod menu_go;
mod menu_tools;
mod menu_view;
mod parser_ready;
mod restore;
mod window_events;
pub(crate) use parser_ready::{ensure_parser_ready_for_path, resolve_zip_path};

#[cfg(target_os = "windows")]
mod hotkey;
#[cfg(target_os = "windows")]
use hotkey::{HotkeyHandle, re_register_hotkey, start_hotkey_listener};

#[cfg(target_os = "windows")]
mod foreground;
#[cfg(target_os = "windows")]
pub(crate) use foreground::{frame_is_disabled, own_dialog_is_up, remember_frame_hwnd};

use crate::ui::navigation::announce;

pub struct MainWindow {
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	config: Rc<Mutex<ConfigManager>>,
	#[cfg(target_os = "windows")]
	tray_state: Rc<Mutex<Option<tray::TrayState>>>,
	#[cfg_attr(
		not(target_os = "macos"),
		allow(dead_code, reason = "keeps the announcement label with the main window")
	)]
	live_region_label: StaticText,
	_find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	#[cfg(target_os = "windows")]
	_hotkey_handle: Rc<RefCell<Option<HotkeyHandle>>>,
	/// Recurring timers, held for the window's lifetime. `Timer`'s `Drop` destroys the
	/// underlying `wxTimer`, which stops it, so a timer that is only started and then dropped
	/// at the end of the function that set it up never fires again.
	_timers: Vec<Rc<Timer<Frame>>>,
}

impl MainWindow {
	/// Accessor for background callbacks (e.g. the OCR worker thread) that can't hold the app's
	/// own `Rc<Mutex<DocumentManager>>` (an `Rc` is not `Send`); they reach the window via
	/// [`crate::ui::app::main_window_from_ptr`] and then this method.
	pub(crate) fn document_manager(&self) -> &Rc<Mutex<DocumentManager>> {
		&self.doc_manager
	}

	pub fn new(config: Rc<Mutex<ConfigManager>>) -> Self {
		// TRANSLATORS: Main window title when no document is open
		let app_title = t("Paperback");
		let frame = Frame::builder().with_title(&app_title).build();
		window_geometry::apply_defaults(&frame);
		#[cfg(target_os = "windows")]
		remember_frame_hwnd(&frame);
		// The title bar and Alt+Tab entry. On Windows the executable's own icon resource
		// (embedded by build.rs) already covers the taskbar and the shell; this is what the
		// window itself carries, and is the only icon at all on the other platforms.
		if let Some(bitmap) = icon::frame_bitmap() {
			frame.set_icon(&bitmap);
		}
		frame.create_status_bar(1, 0, -1, "statusbar");
		// TRANSLATORS: Default status bar text when no document is open
		frame.set_status_text(&t("Ready"), 0);
		let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);
		menu::update_menu_item_states(&frame, false);
		menu::update_reopen_state(&frame, false);
		let panel = Panel::builder(&frame).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		let live_region_label = StaticText::builder(&panel).with_label("").with_size(Size::new(0, 0)).build();
		live_region_label.show(false);
		let _ = live_region::set_live_region(&live_region_label);
		let notebook = Notebook::builder(&panel).with_style(NotebookStyle::Top).build();
		#[cfg(windows)]
		notebook.msw_disable_composited();
		sizer.add(&notebook, 1, SizerFlag::Expand | SizerFlag::All, 0);
		panel.set_sizer(sizer, true);
		// Shared by the frame's char hook and the menu dispatcher so a command can tell a
		// shortcut from a menu click. See `menu_events::bind_key_source`.
		let from_keyboard = Rc::new(Cell::new(false));
		let doc_manager = Rc::new(Mutex::new(DocumentManager::new(
			frame,
			notebook,
			Rc::clone(&config),
			live_region_label,
			Rc::clone(&from_keyboard),
		)));
		let find_dialog = Rc::new(Mutex::new(None));
		#[cfg(target_os = "windows")]
		let hotkey_handle = Rc::new(RefCell::new(start_hotkey_listener(&config.lock().unwrap().get_hotkey())));
		let timers = Self::bind_menu_events(
			&frame,
			&doc_manager,
			&config,
			&find_dialog,
			live_region_label,
			&from_keyboard,
			#[cfg(target_os = "windows")]
			&hotkey_handle,
		);
		menu_events::bind_key_source(&frame, &config, Rc::clone(&from_keyboard));
		#[cfg(not(target_os = "windows"))]
		window_events::bind_tab_cycling(&frame, &doc_manager);
		window_events::bind_document_events(frame, &doc_manager, live_region_label);
		#[cfg(target_os = "windows")]
		let tray_state = Rc::new(Mutex::new(None));
		#[cfg(target_os = "windows")]
		tray::bind_tray_events(frame, &doc_manager, &config, &tray_state);
		window_events::bind_teardown(
			frame,
			&doc_manager,
			&config,
			&timers,
			#[cfg(target_os = "windows")]
			&tray_state,
			#[cfg(target_os = "windows")]
			&hotkey_handle,
		);
		restore::schedule_restore_documents(frame, Rc::clone(&doc_manager), Rc::clone(&config));
		Self {
			frame,
			doc_manager,
			config,
			#[cfg(target_os = "windows")]
			tray_state,
			live_region_label,
			_find_dialog: find_dialog,
			#[cfg(target_os = "windows")]
			_hotkey_handle: hotkey_handle,
			_timers: timers,
		}
	}

	pub fn show(&self) {
		window_geometry::restore(&self.frame, &self.config.lock().unwrap());
		self.frame.show(true);
	}

	#[cfg(target_os = "macos")]
	pub fn show_from_dock(&self) {
		self.frame.show(true);
		self.frame.raise();
		self.doc_manager.lock().unwrap().restore_focus();
	}

	pub fn check_for_updates(&self, silent: bool, channel: UpdateChannel) {
		updater::run_update_check(&self.frame, silent, channel);
	}

	pub fn open_file(&self, path: &Path, quit_if_picker_closed: bool) -> bool {
		let Some(path) = resolve_zip_path(&self.frame, path, &self.config, quit_if_picker_closed) else {
			return false;
		};
		if !self.ensure_parser_ready(&path) {
			return false;
		}
		let result = self.doc_manager.lock().unwrap().open_file(&self.doc_manager, &path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().focus_document_text();
		}
		result
	}

	fn update_title(&self) {
		if let Ok(dm) = self.doc_manager.try_lock() {
			update_title_from_manager(&self.frame, &dm);
		}
	}

	/// Get the frame
	pub const fn frame(&self) -> &Frame {
		&self.frame
	}

	#[cfg(target_os = "macos")]
	pub(super) fn announce_update_restart(&self) {
		live_region::announce(self.live_region_label, &t("Installing the update. Paperback will restart."));
	}

	fn ensure_parser_ready(&self, path: &Path) -> bool {
		ensure_parser_ready_for_path(&self.frame, path, &self.config)
	}

	fn update_recent_documents_menu(&self) {
		rebuild_menu_bar(&self.frame, &self.doc_manager, &self.config);
	}
}

/// Close the active document, announcing the newly focused document for screen readers.
///
/// The `set_selection` inside `close_document` fires `on_page_changing` while the
/// caller holds the manager lock, so the generic switch announcement is suppressed
/// and this function announces the new focus itself instead, before the focus change
/// actually happens.
pub(crate) fn close_active_document_announced(dm: &mut DocumentManager, live_region_label: StaticText) {
	let Some(index) = dm.active_tab_index() else {
		return;
	};
	let next = dm.active_index_after_closing(index).and_then(|i| dm.get_tab(i)).map(display_title);
	if let Some(next) = &next {
		announce(live_region_label, next);
	}
	dm.close_document(index, true);
}

/// Replaces `frame`'s menu bar with one built from `config`, then re-applies the item states
/// that depend on the open documents and the reopen stack.
pub(crate) fn rebuild_menu_bar(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
	frame.set_menu_bar(menu_bar);
	let dm_ref = doc_manager.lock().unwrap();
	let has_docs = dm_ref.tab_count() > 0;
	let has_reopen = dm_ref.has_recently_closed();
	drop(dm_ref);
	menu::update_menu_item_states(frame, has_docs);
	menu::update_reopen_state(frame, has_reopen);
}

pub(crate) fn update_title_from_manager(frame: &Frame, dm: &DocumentManager) {
	let sleep_start = sleep_timer::start_ms();
	let sleep_duration = sleep_timer::duration_minutes();
	if dm.tab_count() == 0 {
		// TRANSLATORS: Main window title when no document is open
		frame.set_title(&t("Paperback"));
		#[cfg(target_os = "macos")]
		frame.set_represented_filename("");
		// TRANSLATORS: Default status bar text when no document is open
		let mut status_text = t("Ready");
		if sleep_start > 0 {
			let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = status::format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		// The document leads and the app name trails, which is the Windows convention and not
		// just a cosmetic one: the taskbar and Alt+Tab truncate the end of a title, so an
		// app-first title makes every open book look identical in both.
		// TRANSLATORS: Window title when a document is open; {} is the document title
		let template = t("{} - Paperback");
		frame.set_title(&template.replace("{}", &display_title(tab)));
		#[cfg(target_os = "macos")]
		frame.set_represented_filename(&tab.file_path.to_string_lossy());
		let position = navigation::doc_caret(tab);
		let status_info = tab.session.get_status_info(position);
		let mut status_text = status::format_status_text(&status_info);
		if sleep_start > 0 {
			let remaining = status::calculate_sleep_timer_remaining(sleep_start, sleep_duration);
			if remaining > 0 {
				status_text = status::format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
	}
}
