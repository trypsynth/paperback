use std::{
	cell::Cell,
	path::Path,
	rc::Rc,
	sync::{Mutex, atomic::Ordering},
};
#[cfg(target_os = "windows")]
use std::{cell::RefCell, sync::atomic::AtomicIsize};

use paperback_core::config::ConfigManager;
use patois::{nt, t};
use wxdragon::{prelude::*, timer::Timer};

#[cfg(target_os = "windows")]
use super::tray;
use super::{
	background, commands, dialogs,
	document_manager::{DocumentManager, DocumentTab, display_title},
	find::{self, FindDialogState},
	help, icon, menu, menu_ids, navigation,
	readability::build_font_from_readability,
	sleep_timer, status,
	update::{self, MAIN_WINDOW_PTR},
	window_geometry,
};
use crate::config_ext::{UpdateChannel, get_update_channel};
#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::ipc::IpcCommand;

mod menu_events;
mod menu_file;
mod menu_go;
mod menu_tools;
mod parser_ready;
mod restore;
pub(crate) use parser_ready::ensure_parser_ready_for_path;

#[cfg(target_os = "windows")]
mod hotkey;
#[cfg(target_os = "windows")]
use hotkey::{HotkeyHandle, re_register_hotkey, start_hotkey_listener};

#[cfg(target_os = "windows")]
mod foreground;
#[cfg(target_os = "windows")]
pub(super) use foreground::{frame_is_disabled, own_dialog_is_up, remember_frame_hwnd};

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
		MAIN_WINDOW_PTR.store(frame.handle_ptr() as usize, Ordering::SeqCst);
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
					// Medium rather than the default high: a file changing on disk is not
					// something the user asked for, so it waits for the current utterance to
					// finish instead of cutting them off mid sentence.
					// TRANSLATORS: Announced by screen readers after a document was automatically reloaded because its file changed on disk
					live_region::announce_with_priority(
						live_region_label,
						&t("Document reloaded."),
						live_region::Priority::Medium,
					);
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
		frame.on_activate(move |event| {
			event.skip(true);
			if let WindowEventData::Activate(activate) = &event {
				if activate.is_active() {
					// On Windows the read-only Richedit does not emit its own focus event when the
					// window is re-activated, so screen readers keep announcing the frame ("pane")
					// instead of the book text. Restore focus to whichever control had it before we
					// left (text control or tab strip) and fire the MSAA focus event explicitly so
					// screen readers re-sync.
					#[cfg(target_os = "windows")]
					if let Ok(dm) = dm_for_activate.try_lock() {
						dm.restore_focus();
						if let Some(hwnd) = dm.focus_target_handle() {
							let hwnd = windows::Win32::Foundation::HWND(hwnd);
							// EVENT_OBJECT_FOCUS = 0x8005, OBJID_CLIENT = -4, CHILDID_SELF = 0
							unsafe {
								windows::Win32::UI::Accessibility::NotifyWinEvent(0x8005, hwnd, -4, 0);
							}
						}
					}
				} else {
					// Deactivating: remember where focus was so we restore the same control (text
					// control or tab strip) on the way back, rather than always forcing the text.
					#[cfg(target_os = "windows")]
					if let Ok(dm) = dm_for_activate.try_lock() {
						dm.record_focus_target();
					}
				}
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
					// Medium rather than the default high, for the same reason as the reload
					// announcement on the page change path.
					// TRANSLATORS: Announced by screen readers after a document was automatically reloaded because its file changed on disk
					live_region::announce_with_priority(
						live_region_label,
						&t("Document reloaded."),
						live_region::Priority::Medium,
					);
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
				{
					let cfg = config_for_close.lock().unwrap();
					window_geometry::save(&frame, &cfg);
					if let Some(tab) = dm.active_tab() {
						cfg.set_app_string("active_document", &tab.file_path.to_string_lossy());
					}
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
		restore::schedule_restore_documents(frame, Rc::clone(&doc_manager), Rc::clone(&config));
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
		window_geometry::restore(&self.frame, &self.config.lock().unwrap());
		self.frame.show(true);
	}

	#[cfg(target_os = "macos")]
	pub fn show_from_dock(&self) {
		self.frame.show(true);
		self.frame.raise();
		self.doc_manager.lock().unwrap().restore_focus();
	}

	pub fn check_for_updates(silent: bool, channel: UpdateChannel) {
		update::run_update_check(silent, channel);
	}

	pub fn open_file(&self, path: &Path) -> bool {
		if !self.ensure_parser_ready(path) {
			return false;
		}
		let result = self.doc_manager.lock().unwrap().open_file(&self.doc_manager, path);
		if result {
			self.update_title();
			self.update_recent_documents_menu();
			self.doc_manager.lock().unwrap().focus_document_text();
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
				self.doc_manager.lock().unwrap().focus_document_text();
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
			let template = t("{} - Paperback");
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
		rebuild_menu_bar(&self.frame, &self.doc_manager, &self.config);
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
		live_region::announce(live_region_label, next);
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
