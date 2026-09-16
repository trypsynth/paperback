use std::sync::atomic::{AtomicIsize, Ordering};

use wxdragon::prelude::*;

/// Whether this process has one of its own dialogs on screen right now.
///
/// The updater's dialogs - the changelog, the progress bar - are all `wxDialog`s, which are the
/// standard `#32770` class on Windows, while the frame is `wxWindowNR`. So this answers "is an
/// update in progress" from the outside, without `ship-shape` having to report it.
///
/// `FindWindowExW` with a null parent looks like the obvious way to walk top-level windows and is
/// not: `EnumWindows` is the API that actually enumerates them. Swapping one for the other is the
/// kind of change that fails silently - a walk that matches nothing means `help.rs` never hands the
/// foreground over, which looks exactly like the hand-off not working.
pub(crate) fn own_dialog_is_up() -> bool {
	use windows::{
		Win32::{
			Foundation::{HWND, LPARAM},
			UI::WindowsAndMessaging::EnumWindows,
		},
		core::BOOL,
	};

	/// Set as the walk goes, so it can stop at the first match.
	struct Found(bool);

	unsafe extern "system" fn note_dialog(window: HWND, param: LPARAM) -> BOOL {
		let found = unsafe { &mut *(param.0 as *mut Found) };
		if window_pid(window) == std::process::id() && class_name(window) == DIALOG_CLASS {
			found.0 = true;
			// One is enough, and the update flow never has two of its dialogs up at once.
			return BOOL(0);
		}
		BOOL(1)
	}

	let mut found = Found(false);
	// The only way this fails is a bad callback, and there is nothing to do about it but carry on:
	// a missed dialog costs the hand-off, not correctness.
	let _ = unsafe { EnumWindows(Some(note_dialog), LPARAM(std::ptr::from_mut(&mut found) as isize)) };
	found.0
}

/// The frame's own Win32 handle, taken when the window is made and read from the beat thread in
/// [`super::super::help`].
///
/// Kept apart from `update::MAIN_WINDOW_PTR`, which holds the wxWidgets object pointer that
/// `ship-shape` wants for parenting its dialogs. The two are different numbers for the same
/// window, and reading one as the other is what #853 was: every call below answered "disabled",
/// because a pointer that is not a window handle is not an enabled window either.
static MAIN_FRAME_HWND: AtomicIsize = AtomicIsize::new(0);

/// Remember the frame's Win32 handle. Called once, from the main thread, with the frame in hand.
///
/// The handle is logged because a zero here is the one way the check below can go quiet without
/// anything else looking wrong: it would answer "no dialog" for the rest of the session, which
/// costs the hand-off rather than leaking it.
pub(crate) fn remember_frame_hwnd(frame: &Frame) {
	let hwnd = frame.get_handle() as isize;
	MAIN_FRAME_HWND.store(hwnd, Ordering::SeqCst);
	tracing::debug!(hwnd, valid = is_disabled_check_possible(), "remembered the main frame's window handle");
}

/// Whether the handle taken above is one Windows knows, which is all the beat thread needs of it.
fn is_disabled_check_possible() -> bool {
	use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::IsWindow};
	unsafe { IsWindow(Some(HWND(MAIN_FRAME_HWND.load(Ordering::SeqCst) as *mut _))) }.as_bool()
}

/// Whether this app's frame is disabled, which wx does for the lifetime of any modal dialog it
/// shows - every dialog in the update flow among them.
///
/// A second way to see the same thing as [`own_dialog_is_up`], kept because the two fail
/// differently: the class walk depends on the dialogs being ordinary top-level `#32770`s, this
/// depends on wx disabling the parent. A missed dialog costs the whole hand-off, while a spurious
/// one costs a few extra grants, so it is worth asking twice.
pub(crate) fn frame_is_disabled() -> bool {
	is_disabled(windows::Win32::Foundation::HWND(MAIN_FRAME_HWND.load(Ordering::SeqCst) as *mut _))
}

/// The question [`frame_is_disabled`] asks, over a handle passed in so it can be tested.
///
/// Anything that is not a live window answers "not disabled". Windows reports a handle it does not
/// know as a window that is not enabled, which reads as a dialog being up forever, so the handle
/// has to be checked before the answer means anything.
fn is_disabled(frame: windows::Win32::Foundation::HWND) -> bool {
	use windows::Win32::UI::{Input::KeyboardAndMouse::IsWindowEnabled, WindowsAndMessaging::IsWindow};
	unsafe { IsWindow(Some(frame)) }.as_bool() && !unsafe { IsWindowEnabled(frame) }.as_bool()
}

/// The process that owns a window, so the walk can tell this app's own windows from another's.
fn window_pid(hwnd: windows::Win32::Foundation::HWND) -> u32 {
	use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
	let mut pid = 0u32;
	unsafe { GetWindowThreadProcessId(hwnd, Some(&raw mut pid)) };
	pid
}

/// The window class, which is what says whether a handle is a dialog or a frame.
fn class_name(hwnd: windows::Win32::Foundation::HWND) -> String {
	use windows::Win32::UI::WindowsAndMessaging::GetClassNameW;
	let mut class = [0u16; 256];
	let class_len = usize::try_from(unsafe { GetClassNameW(hwnd, &mut class) }).unwrap_or(0);
	String::from_utf16_lossy(&class[..class_len])
}

/// The Win32 class of every dialog on Windows, `wxDialog` and `MessageBox` alike.
const DIALOG_CLASS: &str = "#32770";

#[cfg(test)]
mod tests {
	use windows::Win32::Foundation::HWND;

	use super::is_disabled;

	/// The trap behind #853: `IsWindowEnabled` answers "not enabled" for a handle Windows does not
	/// know, so a pointer that is not a window reads as a frame with a modal dialog over it, for
	/// as long as the app runs. What was being passed in was the wxWidgets object pointer.
	#[test]
	fn a_pointer_that_is_not_a_window_is_not_a_disabled_frame() {
		assert!(!is_disabled(HWND(0x1234_5678 as *mut _)), "a stray pointer must not read as disabled");
		assert!(!is_disabled(HWND(std::ptr::null_mut())), "nor must a null one");
	}
}
