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
		let check_disk = !page_reload_guard.replace(true);
		dm_ref.finish_page_change(check_disk);
		if check_disk {
			page_reload_guard.set(false);
		}
	});
	let dm_for_activate = Rc::clone(doc_manager);
	let activate_reload_guard = Rc::clone(&reload_guard);
	let frame_for_activate = frame;
	frame.on_activate(move |event| {
		event.skip(true);
		if let WindowEventData::Activate(activate) = &event {
			if activate.is_active() {
				// The read-only RichEdit emits no focus event on reactivation, so screen readers would say "pane"; fire the MSAA focus event ourselves.
				#[cfg(target_os = "windows")]
				if let Ok(dm) = dm_for_activate.try_lock() {
					dm.restore_focus();
					dm.announce_focus();
				}
			} else {
				// So reactivation returns to the tab strip if that had focus, rather than always forcing the text.
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
				// Medium rather than the default high, for the same reason as the reload announcement on the page change path.
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
			// `switch_to_tab` announces itself: the page-changing handler stays quiet when the notebook has focus, and this call holds its lock.
			if let Some(index) =
				tab_index_for_key(key, key_event.control_down(), key_event.alt_down(), key_event.shift_down())
				&& let Ok(mut dm) = dm.try_lock()
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
			// Geometry has to be read while the window is still on screen.
			window_geometry::save(&frame, &cfg);
			if let Some(tab) = dm.active_tab() {
				cfg.set_app_string("active_document", &tab.file_path.to_string_lossy());
			}
			// Hidden before the bookkeeping below so the window doesn't feel slow to close.
			frame.show(false);
			cfg.flush();
		}
		dm.save_all_positions();
		// Audio stops and focus leaves the per-tab controls (hidden audio control included) before the frame tears its children down.
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
		// Only once really quitting: on macOS the window is only hidden above, and its documents stay open.
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
		// A tick during teardown reaches a destroyed handler, an access violation; stopped only past the macOS veto, or they'd stay stopped when the window came back.
		for timer in &timers_for_close {
			timer.stop();
		}
		event.skip(true);
	});
	#[cfg(target_os = "windows")]
	{
		let tray_for_destroy = Rc::clone(tray_state);
		frame.on_destroy(move |_event| {
			let state = tray_for_destroy.lock().unwrap().take();
			if let Some(state) = state {
				state.icon.destroy();
			}
		});
	}
}

/// The Windows tab control handles Ctrl+Tab itself; macOS and GTK don't. On the char hook so it works from the book and tab strip before Tab moves focus.
#[cfg(not(target_os = "windows"))]
pub(super) fn bind_tab_cycling(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dm = Rc::clone(doc_manager);
	frame.bind_internal(EventType::CHAR_HOOK, move |event| {
		// wxWidgets reports Command as Control on macOS, so the physical Control key is read directly.
		#[cfg(target_os = "macos")]
		let control = wxdragon::utils::get_key_state(WXK_RAW_CONTROL) && !event.control_down();
		#[cfg(not(target_os = "macos"))]
		let control = event.control_down();
		if event.get_key_code() != Some(WXK_TAB) || !control || event.alt_down() {
			return;
		}
		let Ok(mut dm) = dm.try_lock() else { return };
		event.skip(false);
		dm.cycle_tab(!event.shift_down());
	});
}
