//! The Windows global show/hide hotkey: registered on its own thread (so its message loop
//! doesn't block the UI thread), torn down and re-registered whenever the user changes it
//! in Options.

use std::{cell::RefCell, rc::Rc};

use windows::Win32::{
	Foundation::{LPARAM, WPARAM},
	System::Threading::GetCurrentThreadId,
	UI::{
		Input::KeyboardAndMouse::{
			HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, RegisterHotKey, UnregisterHotKey, VkKeyScanW,
		},
		WindowsAndMessaging::{GetMessageW, MSG, PostThreadMessageW, WM_HOTKEY, WM_QUIT},
	},
};
use wxdragon::call_after;

use crate::ipc::IpcCommand;

pub(super) struct HotkeyHandle {
	pub(super) thread_id: u32,
	pub(super) join_handle: std::thread::JoinHandle<()>,
}

pub(super) fn start_hotkey_listener(hotkey: &paperback_core::config::HotkeyConfig) -> Option<HotkeyHandle> {
	const HOTKEY_ID: i32 = 1;
	let mut modifiers = HOT_KEY_MODIFIERS(0);
	if hotkey.ctrl {
		modifiers |= MOD_CONTROL;
	}
	if hotkey.alt {
		modifiers |= MOD_ALT;
	}
	if hotkey.shift {
		modifiers |= MOD_SHIFT;
	}
	if hotkey.win {
		modifiers |= MOD_WIN;
	}
	let vk = char_to_vk(hotkey.key)?;
	let (thread_id_tx, thread_id_rx) = std::sync::mpsc::channel();
	let join_handle = std::thread::spawn(move || {
		let thread_id = unsafe { GetCurrentThreadId() };
		let _ = thread_id_tx.send(thread_id);
		let registered = unsafe { RegisterHotKey(None, HOTKEY_ID, modifiers, vk).is_ok() };
		if !registered {
			return;
		}
		let mut msg = MSG::default();
		loop {
			let result = unsafe { GetMessageW(&raw mut msg, None, 0, 0) };
			if result.0 <= 0 {
				break;
			}
			if msg.message == WM_HOTKEY {
				call_after(Box::new(|| {
					if let Some(window) = super::super::app::main_window_from_ptr() {
						window.handle_ipc_command(IpcCommand::ToggleVisibility);
					}
				}));
				wxdragon::wake_up_idle();
			}
		}
		unsafe {
			let _ = UnregisterHotKey(None, HOTKEY_ID);
		}
	});
	let thread_id = thread_id_rx.recv().ok()?;
	Some(HotkeyHandle { thread_id, join_handle })
}

fn char_to_vk(ch: char) -> Option<u32> {
	if ch == '\0' {
		return None;
	}
	let code = u16::try_from(u32::from(ch)).ok()?;
	let result = unsafe { VkKeyScanW(code) };
	let low_byte = u8::try_from(result & 0xFF).ok()?;
	if low_byte == 0xFF { None } else { Some(u32::from(low_byte)) }
}

pub(super) fn re_register_hotkey(
	hotkey_handle: &Rc<RefCell<Option<HotkeyHandle>>>,
	hotkey: &paperback_core::config::HotkeyConfig,
) {
	if let Some(handle) = hotkey_handle.borrow_mut().take() {
		if handle.thread_id != 0 {
			unsafe {
				let _ = PostThreadMessageW(handle.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
			}
		}
		let _ = handle.join_handle.join();
	}
	*hotkey_handle.borrow_mut() = start_hotkey_listener(hotkey);
}
