//! The window's own event handlers: switching and closing documents from the tab strip, coming
//! back to the window, and closing it.

#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{cell::Cell, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::{prelude::*, timer::Timer};

use super::{
	DocumentManager, close_active_document_announced, display_title, menu, tab_index_for_key,
	update_title_from_manager, window_geometry,
};
#[cfg(target_os = "windows")]
use super::{HotkeyHandle, tray};
use crate::ui::navigation::announce;

pub(super) fn bind_document_events(
	frame: Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
) {
	let frame_copy = frame;
	let notebook = *doc_manager.lock().unwrap().notebook();
	let dm = Rc::clone(doc_manager);
	notebook.on_page_changing(move |event| {
		let Ok(dm_ref) = dm.try_lock() else {
			return;
		};
		if !dm_ref.notebook().has_focus()
			&& let Some(new_index) = event.get_selection()
			&& let Ok(new_index) = usize::try_from(new_index)
			&& let Some(tab) = dm_ref.get_tab(new_index)
		{
			announce(live_region_label, display_title(tab));
		}
	});
	let reload_guard = Rc::new(Cell::new(false));
	let dm = Rc::clone(doc_manager);
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
				// TRANSLATORS: Announced by screen readers after a document was re-read from disk, either because the reader asked for it with F5 or because its file changed on disk
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
	let dm_for_activate = Rc::clone(doc_manager);
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
					dm.announce_focus();
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
				// TRANSLATORS: Announced by screen readers after a document was re-read from disk, either because the reader asked for it with F5 or because its file changed on disk
				live_region::announce_with_priority(
					live_region_label,
					&t("Document reloaded."),
					live_region::Priority::Medium,
				);
			}
			activate_reload_guard.set(false);
		}
	});
	let dm = Rc::clone(doc_manager);
	let frame_copy = frame;
	notebook.on_key_down(move |event| {
		if let WindowEventData::Keyboard(key_event) = &event
			&& let Some(key) = key_event.get_key_code()
		{
			// The same Ctrl+digit chords the reading control answers, so arrowing across the tab
			// strip and then reaching for a number still works. The native tab control announces
			// its own selection from here, and the page-changing handler deliberately stays
			// quiet when the notebook has focus - but `switch_to_tab` announces either way,
			// because it cannot rely on that handler, whose lock this call already holds.
			if let Some(index) =
				tab_index_for_key(key, key_event.control_down(), key_event.alt_down(), key_event.shift_down())
				&& let Ok(dm) = dm.try_lock()
			{
				key_event.event.skip(false);
				dm.switch_to_tab(index);
				return;
			}
			if key == WXK_DELETE || key == WXK_NUMPAD_DELETE {
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
		}
		event.skip(true);
	});
}

pub(super) fn bind_teardown(
	frame: Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	timers: &[Rc<Timer<Frame>>],
	#[cfg(target_os = "windows")] tray_state: &Rc<Mutex<Option<tray::TrayState>>>,
	#[cfg(target_os = "windows")] hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>,
) {
	let dm_for_close = Rc::clone(doc_manager);
	let config_for_close = Rc::clone(config);
	let timers_for_close = timers.to_vec();
	#[cfg(target_os = "windows")]
	let tray_for_close = Rc::clone(tray_state);
	#[cfg(target_os = "windows")]
	let hotkey_for_close = Rc::clone(hotkey_handle);
	frame.on_close(move |event| {
		let mut dm = dm_for_close.lock().unwrap();
		{
			let cfg = config_for_close.lock().unwrap();
			// Geometry has to be read while the window is still on screen, so this
			// comes before it goes.
			window_geometry::save(&frame, &cfg);
			if let Some(tab) = dm.active_tab() {
				cfg.set_app_string("active_document", &tab.file_path.to_string_lossy());
			}
			// Off the screen now. Everything below is bookkeeping: writing the config,
			// saving each document's position, winding audio down. None of it is slow,
			// but all of it happens after the key press, and a window that is still
			// there while it runs is a window that feels slow to close.
			frame.show(false);
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
		// Only once the app is really quitting: on macOS the window is only hidden above, and its
		// documents stay open.
		dm.remove_working_copies();
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
		// Last, and only once the close is really going ahead: macOS hides the window
		// and vetoes instead of exiting, and a timer stopped on that path would stay
		// stopped when the window came back. These tick every 250ms against the frame,
		// so one landing while it tears its children down is delivered to an event
		// handler that no longer exists, which is an access violation rather than a
		// panic. Stopping them here makes that impossible rather than unlikely.
		for timer in &timers_for_close {
			timer.stop();
		}
		event.skip(true);
	});
	#[cfg(target_os = "windows")]
	{
		let tray_for_destroy = Rc::clone(tray_state);
		frame.on_destroy(move |_event| {
			if let Some(state) = tray_for_destroy.lock().unwrap().take() {
				state.icon.destroy();
			}
		});
	}
}

/// Ctrl+Tab and Ctrl+Shift+Tab move between documents, as they do in a browser. The Windows tab control does this on its own; the macOS and GTK ones do not. Bound on the frame's char hook so it works from the book and the tab strip alike, before Tab can move focus.
#[cfg(not(target_os = "windows"))]
pub(super) fn bind_tab_cycling(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dm = Rc::clone(doc_manager);
	frame.bind_internal(EventType::CHAR_HOOK, move |event| {
		// wxWidgets reports Command as Control on macOS, so the physical Control key has to be read directly. Command+Tab is the system's application switcher and never arrives here anyway.
		#[cfg(target_os = "macos")]
		let control = wxdragon::utils::get_key_state(WXK_RAW_CONTROL) && !event.control_down();
		#[cfg(not(target_os = "macos"))]
		let control = event.control_down();
		if event.get_key_code() != Some(WXK_TAB) || !control || event.alt_down() {
			return;
		}
		let Ok(dm) = dm.try_lock() else { return };
		event.skip(false);
		dm.cycle_tab(!event.shift_down());
	});
}
