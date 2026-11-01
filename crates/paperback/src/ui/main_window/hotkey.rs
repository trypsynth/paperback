use std::{cell::RefCell, rc::Rc};

use paperback_core::config::KeyChord;
use wx_utils::global_hotkeys::GlobalHotkeys;
use wxdragon::call_after;

use crate::ipc::IpcCommand;

pub(super) type HotkeyHandle = GlobalHotkeys;

/// `None` when there is no hotkey or it couldn't be registered, nearly always because another program holds it.
pub(super) fn start_hotkey_listener(hotkey: Option<&KeyChord>) -> Option<HotkeyHandle> {
	let (handle, unregistered) = GlobalHotkeys::register(vec![(hotkey?.clone(), ())], |()| {
		// The hotkey thread mustn't touch the window, so the toggle is handed to the UI thread.
		call_after(Box::new(|| {
			if let Some(window) = super::super::app::main_window_from_ptr() {
				window.handle_ipc_command(IpcCommand::ToggleVisibility);
			}
		}));
		wxdragon::wake_up_idle();
	});
	for chord in unregistered {
		tracing::warn!(hotkey = %chord.to_shortcut_string(), "the window hotkey could not be registered");
	}
	handle
}

pub(super) fn re_register_hotkey(hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>, hotkey: Option<&KeyChord>) {
	// Released before registering again, or the new registration would find the chord still taken.
	drop(hotkey_handle.borrow_mut().take());
	*hotkey_handle.borrow_mut() = start_hotkey_listener(hotkey);
}
