use wxdragon::prelude::*;

/// Brings the frame to the front when an update has just relaunched the app.
///
/// Windows only lets a window take the foreground from the foreground process or one it handed the
/// right to, and the relaunched instance is neither: a windowless PowerShell starts it after the
/// old instance has exited and the installer's own window has come and gone. Sharing input state
/// with the thread that owns the foreground window, for the length of the call, is what lets
/// `SetForegroundWindow` through. Only ever done after an update the reader asked for, never at an
/// ordinary start.
pub(super) fn claim_foreground(frame: &Frame) {
	use windows::Win32::{
		Foundation::HWND,
		System::Threading::{AttachThreadInput, GetCurrentThreadId},
		UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow},
	};
	let ours = unsafe { GetCurrentThreadId() };
	let foreground = unsafe { GetForegroundWindow() };
	let theirs = if foreground.is_invalid() { 0 } else { unsafe { GetWindowThreadProcessId(foreground, None) } };
	let attached = theirs != 0 && theirs != ours && unsafe { AttachThreadInput(ours, theirs, true) }.as_bool();
	let raised = unsafe { SetForegroundWindow(HWND(frame.get_handle().cast())) }.as_bool();
	if attached {
		let _ = unsafe { AttachThreadInput(ours, theirs, false) };
	}
	tracing::info!(attached, raised, "claimed the foreground after an update relaunch");
}
