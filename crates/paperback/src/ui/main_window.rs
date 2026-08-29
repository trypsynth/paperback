#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{
	cell::Cell,
	path::Path,
	process,
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicI32, AtomicI64, AtomicIsize, Ordering},
	},
	time::{SystemTime, UNIX_EPOCH},
};

use paperback_core::{config::ConfigManager, parser::build_file_filter_string, types::BookmarkFilterType};
use patois::{nt, t};
use wxdragon::{prelude::*, timer::Timer};

#[cfg(target_os = "windows")]
use super::tray;
use super::{
	dialogs,
	document_manager::{DocumentManager, DocumentTab, build_font_from_readability, display_title},
	dpi,
	find::{self, FindDialogState},
	help::{self, MAIN_WINDOW_PTR},
	icon, menu, menu_ids,
	navigation::{self, MarkerNavTarget},
	status,
};
use crate::config_ext::{UpdateChannel, get_update_channel};
#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::ipc::IpcCommand;

mod menu_file;
mod menu_go;
mod menu_tools;
mod parser_ready;
use parser_ready::ensure_parser_ready_for_path;

#[cfg(target_os = "windows")]
mod hotkey;
#[cfg(target_os = "windows")]
use hotkey::{HotkeyHandle, re_register_hotkey, start_hotkey_listener};

/// The main window's starting size, in device-independent pixels (see `ui::dpi`).
const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
/// Delay before restoring screen-reader focus after window activation (see `focus_restore_timer`).
const FOCUS_RESTORE_DELAY_MS: i32 = 100;

pub static SLEEP_TIMER_START_MS: AtomicI64 = AtomicI64::new(0);
pub static SLEEP_TIMER_DURATION_MINUTES: AtomicI32 = AtomicI32::new(0);

#[derive(Default)]
struct RestoreState {
	restored: bool,
	closing: bool,
}

pub struct MainWindow {
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	config: Rc<Mutex<ConfigManager>>,
	#[cfg(target_os = "windows")]
	tray_state: Rc<Mutex<Option<tray::TrayState>>>,
	_live_region_label: StaticText,
	_find_dialog: Rc<Mutex<Option<FindDialogState>>>,
	#[cfg(target_os = "windows")]
	_hotkey_handle: Rc<RefCell<Option<HotkeyHandle>>>,
	/// Recurring timers, held for the window's lifetime. `Timer`'s `Drop` destroys the
	/// underlying `wxTimer`, which stops it, so a timer that is only started and then dropped
	/// at the end of the function that set it up never fires again.
	_timers: Vec<Rc<Timer<Frame>>>,
}

#[cfg(target_os = "windows")]
static HIDDEN_POPUP: AtomicIsize = AtomicIsize::new(0);

impl MainWindow {
	pub fn new(config: Rc<Mutex<ConfigManager>>) -> Self {
		// TRANSLATORS: Main window title when no document is open
		let app_title = t("Paperback");
		let frame = Frame::builder().with_title(&app_title).build();
		// Sized after building rather than through the builder: the size has to be scaled for
		// the display the window actually lands on, and there is nothing to ask about that
		// until the frame exists. It isn't shown until later, so there's no visible resize.
		frame.set_size(dpi::scale_size(&frame, Size::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)));
		MAIN_WINDOW_PTR.store(frame.handle_ptr() as usize, Ordering::SeqCst);
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
		let doc_manager =
			Rc::new(Mutex::new(DocumentManager::new(frame, notebook, Rc::clone(&config), live_region_label)));
		let find_dialog = Rc::new(Mutex::new(None));
		#[cfg(target_os = "windows")]
		let hotkey_handle = Rc::new(RefCell::new(start_hotkey_listener(&config.lock().unwrap().get_hotkey())));
		let timers = Self::bind_menu_events(
			&frame,
			&doc_manager,
			&config,
			&find_dialog,
			live_region_label,
			#[cfg(target_os = "windows")]
			&hotkey_handle,
		);
		let frame_copy = frame;
		let notebook = *doc_manager.lock().unwrap().notebook();
		// One-shot timer that restores screen-reader focus after window activation. On Windows the
		// read-only Richedit does not emit its own focus event when the window is re-activated, so
		// screen readers keep announcing the frame ("pane") instead of the book text. Defer past
		// Windows' own activation/focus processing, restore focus to the text control, then fire the
		// MSAA focus event explicitly so screen readers re-sync. Owned by the notebook so its events
		// don't cross-fire with the frame-owned sleep/status timers.
		let focus_restore_timer = Rc::new(Timer::new(&notebook));
		{
			let dm_for_focus_timer = Rc::clone(&doc_manager);
			focus_restore_timer.on_tick(move |_| {
				let dm = dm_for_focus_timer.lock().unwrap();
				dm.restore_focus();
				#[cfg(target_os = "windows")]
				if let Some(tab) = dm.active_tab() {
					let hwnd = windows::Win32::Foundation::HWND(tab.text_ctrl.get_handle());
					// EVENT_OBJECT_FOCUS = 0x8005, OBJID_CLIENT = -4, CHILDID_SELF = 0
					unsafe {
						windows::Win32::UI::Accessibility::NotifyWinEvent(0x8005, hwnd, -4, 0);
					}
				}
			});
		}
		let dm = Rc::clone(&doc_manager);
		notebook.on_page_changing(move |event| {
			let Ok(dm_ref) = dm.try_lock() else {
				return;
			};
			if !dm_ref.notebook().has_focus()
				&& let Some(new_index) = event.get_selection()
				&& let Ok(new_index) = usize::try_from(new_index)
				&& let Some(tab) = dm_ref.get_tab(new_index)
			{
				live_region::announce(live_region_label, &display_title(tab));
			}
		});
		let reload_guard = Rc::new(Cell::new(false));
		let dm = Rc::clone(&doc_manager);
		let page_reload_guard = Rc::clone(&reload_guard);
		notebook.on_page_changed(move |_event| {
			let Ok(mut dm_ref) = dm.try_lock() else {
				return;
			};
			if !page_reload_guard.get() {
				page_reload_guard.set(true);
				if let Some(index) = dm_ref.active_tab_index()
					&& dm_ref.reload_tab_if_changed(index)
				{
					// TRANSLATORS: Announced by screen readers after a document was automatically reloaded because its file changed on disk
					live_region::announce(live_region_label, &t("Document reloaded."));
				}
				page_reload_guard.set(false);
			}
			update_title_from_manager(&frame_copy, &dm_ref);
			dm_ref.reset_sound_line();
			dm_ref.pause_inactive_audio();
		});
		let dm_for_activate = Rc::clone(&doc_manager);
		let activate_reload_guard = Rc::clone(&reload_guard);
		let frame_for_activate = frame;
		let focus_timer = Rc::clone(&focus_restore_timer);
		frame.on_activate(move |event| {
			event.skip(true);
			if let WindowEventData::Activate(activate) = &event
				&& activate.is_active()
			{
				// Defer focus restoration past Windows' own activation/focus processing so screen
				// readers land back on the book text (see focus_restore_timer above).
				focus_timer.start(FOCUS_RESTORE_DELAY_MS, true);
			}
			if let WindowEventData::Activate(activate) = &event
				&& activate.is_active()
				&& !activate_reload_guard.get()
				&& let Ok(mut dm_ref) = dm_for_activate.try_lock()
				&& let Some(index) = dm_ref.active_tab_index()
			{
				activate_reload_guard.set(true);
				if dm_ref.reload_tab_if_changed(index) {
					update_title_from_manager(&frame_for_activate, &dm_ref);
					dm_ref.update_status_bar();
					// TRANSLATORS: Announced by screen readers after a document was automatically reloaded because its file changed on disk
					live_region::announce(live_region_label, &t("Document reloaded."));
				}
				activate_reload_guard.set(false);
			}
		});
		let dm = Rc::clone(&doc_manager);
		let frame_copy = frame;
		notebook.on_key_down(move |event| {
			if let WindowEventData::Keyboard(key_event) = &event
				&& let Some(key) = key_event.get_key_code()
				&& (key == WXK_DELETE || key == WXK_NUMPAD_DELETE)
			{
				let mut dm = dm.lock().unwrap();
				close_active_document_announced(&mut dm, live_region_label);
				update_title_from_manager(&frame_copy, &dm);
				let has_docs = dm.tab_count() > 0;
				let has_reopen = dm.has_recently_closed();
				if has_docs {
					dm.restore_focus();
				} else {
					dm.notebook().set_focus();
				}
				drop(dm);
				menu::update_menu_item_states(&frame_copy, has_docs);
				menu::update_reopen_state(&frame_copy, has_reopen);
				event.skip(false);
				return;
			}
			event.skip(true);
		});
		#[cfg(target_os = "windows")]
		let tray_state = Rc::new(Mutex::new(None));
		#[cfg(target_os = "windows")]
		tray::bind_tray_events(frame, &doc_manager, &config, &tray_state);
		{
			let dm_for_close = Rc::clone(&doc_manager);
			let config_for_close = Rc::clone(&config);
			#[cfg(target_os = "windows")]
			let tray_for_close = Rc::clone(&tray_state);
			#[cfg(target_os = "windows")]
			let hotkey_for_close = Rc::clone(&hotkey_handle);
			frame.on_close(move |event| {
				let mut dm = dm_for_close.lock().unwrap();
				if let Some(tab) = dm.active_tab() {
					let path = tab.file_path.to_string_lossy();
					let cfg = config_for_close.lock().unwrap();
					cfg.set_app_string("active_document", &path);
					cfg.flush();
				}
				dm.save_all_positions();
				// Stop audio and move focus off the per-tab child controls (the hidden audio
				// control included) before the frame tears its children down.
				dm.stop_all_audio();
				frame.set_focus();
				#[cfg(target_os = "macos")]
				if let WindowEventData::General(ref ev) = event
					&& ev.can_veto()
				{
					drop(dm);
					ev.veto();
					frame.show(false);
					return;
				}
				#[cfg(target_os = "windows")]
				if let Some(state) = tray_for_close.lock().unwrap().as_ref() {
					state.icon.remove_icon();
				}
				#[cfg(target_os = "windows")]
				if let Some(handle) = hotkey_for_close.borrow_mut().take() {
					use windows::Win32::{
						Foundation::{LPARAM, WPARAM},
						UI::WindowsAndMessaging::{PostThreadMessageW, WM_QUIT},
					};
					if handle.thread_id != 0 {
						unsafe {
							let _ = PostThreadMessageW(handle.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
						}
					}
				}
				event.skip(true);
			});
		}
		#[cfg(target_os = "windows")]
		{
			let tray_for_destroy = Rc::clone(&tray_state);
			frame.on_destroy(move |_event| {
				if let Some(state) = tray_for_destroy.lock().unwrap().take() {
					state.icon.destroy();
				}
			});
		}
		Self::schedule_restore_documents(frame, Rc::clone(&doc_manager), Rc::clone(&config));
		Self {
			frame,
			doc_manager,
			config,
			#[cfg(target_os = "windows")]
			tray_state,
			_live_region_label: live_region_label,
			_find_dialog: find_dialog,
			#[cfg(target_os = "windows")]
			_hotkey_handle: hotkey_handle,
			_timers: timers,
		}
	}

	pub fn show(&self) {
		if self.config.lock().unwrap().get_app_bool("start_maximized", false) {
			self.frame.maximize(true);
		}
		self.frame.show(true);
		self.frame.centre();
	}

	#[cfg(target_os = "macos")]
	pub fn show_from_dock(&self) {
		self.frame.show(true);
		self.frame.raise();
		self.doc_manager.lock().unwrap().restore_focus();
	}

	pub fn check_for_updates(silent: bool, channel: UpdateChannel) {
		help::run_update_check(silent, channel);
	}

	pub fn open_file(&self, path: &Path) -> bool {
		if !self.ensure_parser_ready(path) {
			return false;
		}
		let result = self.doc_manager.lock().unwrap().open_file(&self.doc_manager, path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().restore_focus();
		}
		result
	}

	#[cfg(any(target_os = "linux", target_os = "windows"))]
	pub fn handle_ipc_command(&self, command: IpcCommand) {
		tracing::info!(command = ?command, "received IPC command");
		let mut web_view_dialog = None;
		dialogs::ACTIVE_WEB_VIEW.with(|v| {
			web_view_dialog = v.get();
		});
		if let Some(parent_dialog) = web_view_dialog {
			let dialog = MessageDialog::builder(
				&parent_dialog,
				// TRANSLATORS: Message shown when the user tries to perform an action while a help/documentation Web View window is open
				&t("Paperback cannot perform any actions while Web View is open."),
				// TRANSLATORS: Title of a warning dialog
				&t("Warning"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning | MessageDialogStyle::Centre)
			.build();
			dialog.show_modal();
			return;
		}
		match command {
			IpcCommand::Activate => {
				self.activate_from_ipc();
			}
			IpcCommand::ToggleVisibility => {
				self.toggle_visibility();
			}
			IpcCommand::OpenFile(path) => {
				self.activate_from_ipc();
				self.open_file(&path);
				self.frame.raise();
				self.doc_manager.lock().unwrap().restore_focus();
			}
		}
	}

	#[cfg(any(target_os = "linux", target_os = "windows"))]
	fn toggle_visibility(&self) {
		let is_shown = self.frame.is_shown();
		if is_shown && self.is_window_active() {
			let mut has_popup = false;
			#[cfg(target_os = "windows")]
			{
				use windows::Win32::{
					Foundation::HWND,
					UI::WindowsAndMessaging::{GetLastActivePopup, SW_HIDE, ShowWindow},
				};
				let handle = self.frame.get_handle();
				if !handle.is_null() {
					let frame_hwnd = HWND(handle);
					let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
					if active_popup != frame_hwnd {
						has_popup = true;
						HIDDEN_POPUP.store(active_popup.0 as isize, Ordering::SeqCst);
						let _ = unsafe { ShowWindow(active_popup, SW_HIDE) };
					}
				}
			}
			if has_popup {
				self.frame.show(false);
			} else {
				self.frame.iconize(true);
			}
		} else {
			self.activate_from_ipc();
		}
	}

	#[cfg(any(target_os = "linux", target_os = "windows"))]
	fn activate_from_ipc(&self) {
		self.frame.show(true);
		self.frame.iconize(false);
		self.frame.request_user_attention(UserAttentionFlag::Info);
		self.frame.raise();
		#[allow(unused_mut)]
		let mut has_popup = false;
		#[cfg(target_os = "windows")]
		{
			use windows::Win32::{
				Foundation::HWND,
				UI::WindowsAndMessaging::{GetLastActivePopup, SW_SHOW, SetForegroundWindow, ShowWindow},
			};
			let handle = self.frame.get_handle();
			if !handle.is_null() {
				let frame_hwnd = HWND(handle);
				let hidden = HIDDEN_POPUP.swap(0, Ordering::SeqCst);
				if hidden != 0 {
					let active_popup = HWND(hidden as _);
					let _ = unsafe { ShowWindow(active_popup, SW_SHOW) };
					let _ = unsafe { SetForegroundWindow(active_popup) };
					has_popup = true;
				} else {
					let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
					has_popup = active_popup != frame_hwnd;
					let _ = unsafe { SetForegroundWindow(active_popup) };
				}
			}
		}
		if !has_popup {
			self.doc_manager.lock().unwrap().restore_focus();
		}
		#[cfg(not(target_os = "linux"))]
		if let Some(state) = self.tray_state.lock().unwrap().as_mut() {
			tray::set_tray_icon(&state.icon);
		}
	}

	#[cfg(any(target_os = "linux", target_os = "windows"))]
	fn is_window_active(&self) -> bool {
		#[cfg(target_os = "windows")]
		{
			use windows::Win32::{
				Foundation::HWND,
				UI::WindowsAndMessaging::{GetForegroundWindow, GetLastActivePopup},
			};
			let handle = self.frame.get_handle();
			if handle.is_null() {
				return self.frame.has_focus();
			}
			let frame_hwnd = HWND(handle);
			let foreground = unsafe { GetForegroundWindow() };
			let active_popup = unsafe { GetLastActivePopup(frame_hwnd) };
			foreground == frame_hwnd || foreground == active_popup
		}
		#[cfg(not(target_os = "windows"))]
		{
			self.frame.has_focus()
		}
	}

	fn update_title(&self) {
		let Ok(dm) = self.doc_manager.try_lock() else {
			return;
		};
		if dm.tab_count() == 0 {
			// TRANSLATORS: Main window title when no document is open
			self.frame.set_title(&t("Paperback"));
			#[cfg(target_os = "macos")]
			self.frame.set_represented_filename("");
			// TRANSLATORS: Default status bar text when no document is open
			self.frame.set_status_text(&t("Ready"), 0);
			return;
		}
		if let Some(tab) = dm.active_tab() {
			// TRANSLATORS: Window title when a document is open; {} is the document title
			let template = t("Paperback - {}");
			self.frame.set_title(&template.replace("{}", &display_title(tab)));
			#[cfg(target_os = "macos")]
			self.frame.set_represented_filename(&tab.file_path.to_string_lossy());
			// TRANSLATORS: Status bar character count. The %d placeholder is replaced with the number of characters.
			let char_count = tab.session.content().len();
			let chars_label = nt("%d char", "%d chars", char_count as u64).replacen("%d", &char_count.to_string(), 1);
			self.frame.set_status_text(&chars_label, 0);
		}
	}

	/// Get the frame
	pub const fn frame(&self) -> &Frame {
		&self.frame
	}

	fn ensure_parser_ready(&self, path: &Path) -> bool {
		ensure_parser_ready_for_path(&self.frame, path, &self.config)
	}

	fn update_recent_documents_menu(&self) {
		let menu_bar = menu::create_menu_bar(&self.config.lock().unwrap());
		self.frame.set_menu_bar(menu_bar);
		let dm_ref = self.doc_manager.lock().unwrap();
		let has_docs = dm_ref.tab_count() > 0;
		let has_reopen = dm_ref.has_recently_closed();
		drop(dm_ref);
		menu::update_menu_item_states(&self.frame, has_docs);
		menu::update_reopen_state(&self.frame, has_reopen);
	}

	fn schedule_restore_documents(
		frame: Frame,
		doc_manager: Rc<Mutex<DocumentManager>>,
		config: Rc<Mutex<ConfigManager>>,
	) {
		let restore = config.lock().unwrap().get_app_bool("restore_previous_documents", true);
		if !restore {
			return;
		}
		let state = Rc::new(Mutex::new(RestoreState::default()));
		let state_for_close = Rc::clone(&state);
		frame.on_close(move |_event| {
			state_for_close.lock().unwrap().closing = true;
		});
		let state_for_destroy = Rc::clone(&state);
		frame.on_destroy(move |_event| {
			state_for_destroy.lock().unwrap().closing = true;
		});
		let state_for_idle = Rc::clone(&state);
		frame.on_idle(move |_event| {
			let mut state = state_for_idle.lock().unwrap();
			if state.restored || state.closing {
				return;
			}
			state.restored = true;
			drop(state);
			let pre_restore_active = doc_manager.lock().unwrap().active_tab_index();
			let active_path = config.lock().unwrap().get_app_string("active_document", "");
			let paths = config.lock().unwrap().get_opened_documents_existing();
			tracing::info!(count = paths.len(), "restoring previously open documents");
			for path in paths {
				let path = Path::new(&path);
				if !ensure_parser_ready_for_path(&frame, path, &config) {
					continue;
				}
				let _ = doc_manager.lock().unwrap().open_file_restore(&doc_manager, path);
			}
			let mut target_idx = pre_restore_active;
			if target_idx.is_none() && !active_path.is_empty() {
				target_idx = doc_manager.lock().unwrap().find_tab_by_path(Path::new(&active_path));
			}
			if let Some(idx) = target_idx {
				doc_manager.lock().unwrap().notebook().set_selection(idx);
			}
			let dm_ref = doc_manager.lock().unwrap();
			update_title_from_manager(&frame, &dm_ref);
			let has_docs = dm_ref.tab_count() > 0;
			let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
			frame.set_menu_bar(menu_bar);
			menu::update_menu_item_states(&frame, has_docs);
			menu::update_reopen_state(&frame, false);
			dm_ref.restore_focus();
		});
	}

	/// Prompts for a save path and exports `tab`'s document as `format`, showing a
	/// generic failure dialog on error. Shared by the `EXPORT_TO_PLAIN_TEXT` /
	/// `EXPORT_TO_HTML` / `EXPORT_TO_MARKDOWN` menu handlers, which differ only in
	/// `format`, the default file `extension`, the file-picker `wildcard`, and the
	/// file-picker `dialog_title`.
	fn export_document_as(
		frame: &Frame,
		tab: &DocumentTab,
		format: paperback_core::export::ExportFormat,
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

	fn handle_open(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) {
		let wildcard = build_file_filter_string();
		// TRANSLATORS: Title of the file picker dialog shown when opening a document
		let dialog_title = t("Open Document");
		let dialog = FileDialog::builder(frame)
			.with_message(&dialog_title)
			.with_wildcard(&wildcard)
			.with_style(FileDialogStyle::Open | FileDialogStyle::FileMustExist)
			.build();
		if dialog.show_modal() == ID_OK
			&& let Some(path) = dialog.get_path()
		{
			let path = Path::new(&path);
			if !ensure_parser_ready_for_path(frame, path, config) {
				return;
			}
			if doc_manager.lock().unwrap().open_file(doc_manager, path) {
				let Ok(dm_ref) = doc_manager.try_lock() else {
					return;
				};
				update_title_from_manager(frame, &dm_ref);
				dm_ref.restore_focus();
				drop(dm_ref);
				let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
				frame.set_menu_bar(menu_bar);
				menu::update_menu_item_states(frame, true);
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	fn bind_menu_events(
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
		let sleep_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running = Rc::new(Cell::new(false));
		let sleep_timer_start_time = Rc::new(Cell::new(0i64));
		let sleep_timer_duration_minutes = Rc::new(Cell::new(0i32));
		let sleep_timer_for_tick = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_tick = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_tick = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_tick = Rc::clone(&sleep_timer_duration_minutes);
		let frame_for_timer = *frame;
		let dm_for_timer = Rc::clone(doc_manager);
		let config_for_timer = Rc::clone(&config);
		sleep_timer.on_tick(move |_| {
			// `Timer::on_tick` binds `EventType::TIMER` on the *owner*, not on the timer, and
			// wxdragon gives its timers no distinguishing id, so every timer parented to this
			// frame delivers its ticks to every handler bound here. This one shuts the app down,
			// so it has to confirm the deadline really passed rather than trust that being
			// called means its own timer fired.
			if !sleep_timer_running_for_tick.get() {
				return;
			}
			let now_ms = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.ok()
				.and_then(|d| i64::try_from(d.as_millis()).ok())
				.unwrap_or(0);
			let deadline_ms = sleep_timer_start_for_tick
				.get()
				.saturating_add(i64::from(sleep_timer_duration_for_tick.get()) * 60_000);
			if now_ms < deadline_ms {
				return;
			}
			tracing::info!("sleep timer fired, closing application");
			sleep_timer_running_for_tick.set(false);
			sleep_timer_for_tick.stop();
			SLEEP_TIMER_START_MS.store(0, Ordering::SeqCst);
			SLEEP_TIMER_DURATION_MINUTES.store(0, Ordering::SeqCst);
			{
				let dm = dm_for_timer.lock().unwrap();
				let cfg = config_for_timer.lock().unwrap();
				for i in 0..dm.tab_count() {
					if let Some(tab) = dm.get_tab(i) {
						let current_pos = navigation::doc_caret(tab);
						let path_str = tab.file_path.to_string_lossy();
						cfg.set_document_position(&path_str, current_pos);
					}
				}
				cfg.flush();
			}
			frame_for_timer.close(true);
		});
		let status_update_timer = Rc::new(Timer::new(frame));
		let sleep_timer_running_for_status = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_status = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_status = Rc::clone(&sleep_timer_duration_minutes);
		let dm_for_status = Rc::clone(doc_manager);
		let frame_for_status = *frame;
		status_update_timer.on_tick(move |_| {
			if !sleep_timer_running_for_status.get() {
				return;
			}
			let Ok(dm) = dm_for_status.try_lock() else {
				return;
			};
			status::update_status_bar_with_sleep_timer(
				&frame_for_status,
				&dm,
				sleep_timer_start_for_status.get(),
				sleep_timer_duration_for_status.get(),
			);
		});
		status_update_timer.start(1000, false);
		let audio_sync_timer = Rc::new(Timer::new(frame));
		let dm_for_audio_sync = Rc::clone(doc_manager);
		audio_sync_timer.on_tick(move |_| {
			if let Ok(mut dm) = dm_for_audio_sync.try_lock() {
				dm.pump_audio();
			}
		});
		audio_sync_timer.start(250, false);
		let window_reload_timer = Rc::new(Timer::new(frame));
		let dm_for_window_reload = Rc::clone(doc_manager);
		window_reload_timer.on_tick(move |_| {
			if let Ok(mut dm) = dm_for_window_reload.try_lock() {
				dm.pump_window_reload();
			}
		});
		window_reload_timer.start(250, false);
		let sleep_timer_for_menu = Rc::clone(&sleep_timer);
		let sleep_timer_running_for_menu = Rc::clone(&sleep_timer_running);
		let sleep_timer_start_for_menu = Rc::clone(&sleep_timer_start_time);
		let sleep_timer_duration_for_menu = Rc::clone(&sleep_timer_duration_minutes);
		frame.on_menu(move |event| {
			let id = event.get_id();
			match id {
				menu_ids::OPEN => {
					Self::handle_open(&frame_copy, &dm, &config);
				}
				menu_ids::CLOSE => {
					let mut dm = dm.lock().unwrap();
					close_active_document_announced(&mut dm, live_region_label);
					update_title_from_manager(&frame_copy, &dm);
					let has_docs = dm.tab_count() > 0;
					if has_docs {
						dm.restore_focus();
					} else {
						dm.notebook().set_focus();
					}
					drop(dm);
					menu::update_menu_item_states(&frame_copy, has_docs);
					menu::update_reopen_state(&frame_copy, true);
				}
				menu_ids::CLOSE_ALL => {
					let mut dm = dm.lock().unwrap();
					dm.close_all_documents();
					update_title_from_manager(&frame_copy, &dm);
					dm.notebook().set_focus();
					drop(dm);
					menu::update_menu_item_states(&frame_copy, false);
					menu::update_reopen_state(&frame_copy, true);
				}
				menu_ids::REOPEN_LAST_CLOSED => {
					let path = dm.lock().unwrap().pop_recently_closed();
					if let Some(path) = path {
						if !ensure_parser_ready_for_path(&frame_copy, &path, &config) {
							dm.lock().unwrap().push_recently_closed(path);
							return;
						}
						if dm.lock().unwrap().open_file(&dm, &path) {
							let dm_ref = dm.lock().unwrap();
							update_title_from_manager(&frame_copy, &dm_ref);
							dm_ref.restore_focus();
							drop(dm_ref);
							let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
							frame_copy.set_menu_bar(menu_bar);
							menu::update_menu_item_states(&frame_copy, true);
						}
						let has_reopen = dm.lock().unwrap().has_recently_closed();
						menu::update_reopen_state(&frame_copy, has_reopen);
					}
				}
				menu_ids::EXIT => {
					dm.lock().unwrap().save_all_positions();
					process::exit(0);
				}
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
					menu_go::handle_go_to_line(&frame_copy, &dm, &config);
				}
				menu_ids::GO_TO_PAGE => {
					menu_go::handle_go_to_page(&frame_copy, &dm, &config, live_region_label);
				}
				menu_ids::GO_TO_PERCENT => {
					menu_go::handle_go_to_percent(&frame_copy, &dm, &config);
				}
				menu_ids::GO_BACK => {
					navigation::handle_history_navigation(&dm, &config, live_region_label, false);
				}
				menu_ids::GO_FORWARD => {
					navigation::handle_history_navigation(&dm, &config, live_region_label, true);
				}
				menu_ids::PREVIOUS_SECTION => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Section,
						false,
					);
				}
				menu_ids::NEXT_SECTION => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Section,
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(0),
						false,
					);
				}
				menu_ids::NEXT_HEADING => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(0),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_1 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(1),
						false,
					);
				}
				menu_ids::NEXT_HEADING_1 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(1),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_2 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(2),
						false,
					);
				}
				menu_ids::NEXT_HEADING_2 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(2),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_3 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(3),
						false,
					);
				}
				menu_ids::NEXT_HEADING_3 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(3),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_4 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(4),
						false,
					);
				}
				menu_ids::NEXT_HEADING_4 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(4),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_5 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(5),
						false,
					);
				}
				menu_ids::NEXT_HEADING_5 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(5),
						true,
					);
				}
				menu_ids::PREVIOUS_HEADING_6 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(6),
						false,
					);
				}
				menu_ids::NEXT_HEADING_6 => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Heading(6),
						true,
					);
				}
				menu_ids::PREVIOUS_PAGE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, false);
				}
				menu_ids::NEXT_PAGE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Page, true);
				}
				menu_ids::PREVIOUS_BOOKMARK => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, false, false);
				}
				menu_ids::NEXT_BOOKMARK => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, true, false);
				}
				menu_ids::PREVIOUS_NOTE => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, false, true);
				}
				menu_ids::NEXT_NOTE => {
					navigation::handle_bookmark_navigation(&dm, &config, live_region_label, true, true);
				}
				menu_ids::JUMP_TO_ALL_BOOKMARKS => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::All,
					);
				}
				menu_ids::JUMP_TO_BOOKMARKS_ONLY => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::BookmarksOnly,
					);
				}
				menu_ids::JUMP_TO_NOTES_ONLY => {
					navigation::handle_bookmark_dialog(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						BookmarkFilterType::NotesOnly,
					);
				}
				menu_ids::TOGGLE_BOOKMARK => {
					navigation::handle_toggle_bookmark(&dm, &config, live_region_label);
				}
				menu_ids::BOOKMARK_WITH_NOTE => {
					navigation::handle_bookmark_with_note(&frame_copy, &dm, &config, live_region_label);
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
				menu_ids::PLAY_PAUSE_AUDIO => {
					navigation::handle_toggle_play_pause_audio(&dm, live_region_label);
				}
				menu_ids::SEEK_AUDIO_FORWARD => {
					navigation::handle_seek_audio(&dm, &config, live_region_label, true);
				}
				menu_ids::SEEK_AUDIO_BACKWARD => {
					navigation::handle_seek_audio(&dm, &config, live_region_label, false);
				}
				menu_ids::INCREASE_AUDIO_SEEK_AMOUNT => {
					navigation::handle_change_seek_amount(&config, live_region_label, true);
				}
				menu_ids::DECREASE_AUDIO_SEEK_AMOUNT => {
					navigation::handle_change_seek_amount(&config, live_region_label, false);
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
				menu_ids::VIEW_NOTE_TEXT => {
					navigation::handle_view_note_text(&frame_copy, &dm, &config);
				}
				menu_ids::PREVIOUS_LINK => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, false);
				}
				menu_ids::NEXT_LINK => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Link, true);
				}
				menu_ids::PREVIOUS_IMAGE => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Image,
						false,
					);
				}
				menu_ids::NEXT_IMAGE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Image, true);
				}
				menu_ids::PREVIOUS_FIGURE => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Figure,
						false,
					);
				}
				menu_ids::NEXT_FIGURE => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Figure,
						true,
					);
				}
				menu_ids::PREVIOUS_TABLE => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Table,
						false,
					);
				}
				menu_ids::NEXT_TABLE => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::Table, true);
				}
				menu_ids::PREVIOUS_SEPARATOR => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Separator,
						false,
					);
				}
				menu_ids::NEXT_SEPARATOR => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::Separator,
						true,
					);
				}
				menu_ids::PREVIOUS_LIST => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, false);
				}
				menu_ids::NEXT_LIST => {
					navigation::handle_marker_navigation(&dm, &config, live_region_label, MarkerNavTarget::List, true);
				}
				menu_ids::PREVIOUS_LIST_ITEM => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::ListItem,
						false,
					);
				}
				menu_ids::NEXT_LIST_ITEM => {
					navigation::handle_marker_navigation(
						&dm,
						&config,
						live_region_label,
						MarkerNavTarget::ListItem,
						true,
					);
				}
				menu_ids::CONTAINER_START => {
					navigation::handle_container_navigation(&dm, &config, live_region_label, false);
				}
				menu_ids::CONTAINER_END => {
					navigation::handle_container_navigation(&dm, &config, live_region_label, true);
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
					menu_tools::handle_elements_list(&frame_copy, &dm, &config);
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
					menu_tools::handle_sleep_timer(
						&frame_copy,
						&dm,
						&config,
						live_region_label,
						&sleep_timer_for_menu,
						&sleep_timer_running_for_menu,
						&sleep_timer_start_for_menu,
						&sleep_timer_duration_for_menu,
					);
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
					help::run_update_check(false, channel);
				}
				menu_ids::DONATE => {
					help::handle_donate(&frame_copy);
				}
				_ => {
					menu_file::handle_fallback(id, &frame_copy, &dm, &config, live_region_label);
				}
			}
		});
		vec![sleep_timer, status_update_timer, audio_sync_timer, window_reload_timer]
	}
}

/// Close the active document, announcing the newly focused document for screen readers.
///
/// The `set_selection` inside `close_document` fires `on_page_changing` while the
/// caller holds the manager lock, so the generic switch announcement is suppressed
/// and this function announces the new focus itself instead, before the focus change
/// actually happens.
fn close_active_document_announced(dm: &mut DocumentManager, live_region_label: StaticText) {
	let Some(index) = dm.active_tab_index() else {
		return;
	};
	let next = dm.active_index_after_closing(index).and_then(|i| dm.get_tab(i)).map(display_title);
	if let Some(next) = &next {
		live_region::announce(live_region_label, next);
	}
	dm.close_document(index, true);
}

fn update_title_from_manager(frame: &Frame, dm: &DocumentManager) {
	let sleep_start = SLEEP_TIMER_START_MS.load(Ordering::SeqCst);
	let sleep_duration = SLEEP_TIMER_DURATION_MINUTES.load(Ordering::SeqCst);
	if dm.tab_count() == 0 {
		frame.set_title(&t("Paperback"));
		#[cfg(target_os = "macos")]
		frame.set_represented_filename("");
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
		// TRANSLATORS: Window title when a document is open; {} is the document title
		let template = t("Paperback - {}");
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
