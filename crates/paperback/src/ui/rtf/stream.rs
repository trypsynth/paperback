//! Feeds a `write::build_rtf` blob into the native `RichEdit` control behind a
//! `wxTE_RICH2` `TextCtrl`, via the Win32 `EM_STREAMIN` message. See
//! `document_manager::fill_text_ctrl_with_formatting` for the round-trip check
//! that decides whether callers trust this path or fall back to plain text.

use std::ptr::{addr_of_mut, copy_nonoverlapping};

use wxdragon::prelude::*;

struct RtfStreamCursor<'a> {
	data: &'a [u8],
	pos: usize,
}

/// `EDITSTREAMCALLBACK` for `EM_STREAMIN`: `RichEdit` calls this repeatedly,
/// asking for up to `cb` bytes each time, until we report 0 bytes written
/// (end of stream) or return a nonzero error code. Called synchronously
/// within `SendMessageW` on the same thread, so the `RtfStreamCursor` borrow
/// in `stream_rtf_into_ctrl` stays valid for every call.
unsafe extern "system" fn rtf_stream_read_callback(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32 {
	if pbbuff.is_null() || pcb.is_null() || dwcookie == 0 {
		return 1;
	}
	let cursor = unsafe { &mut *(dwcookie as *mut RtfStreamCursor<'_>) };
	let remaining = cursor.data.len() - cursor.pos;
	let to_copy = remaining.min(usize::try_from(cb.max(0)).unwrap_or(0));
	if to_copy > 0 {
		unsafe { copy_nonoverlapping(cursor.data[cursor.pos..].as_ptr(), pbbuff, to_copy) };
		cursor.pos += to_copy;
	}
	unsafe { *pcb = i32::try_from(to_copy).unwrap_or(i32::MAX) };
	0
}

/// Feeds `rtf` into the native `RichEdit` control behind `text_ctrl` via the
/// Win32 `EM_STREAMIN` message. `wxTextCtrl::SetValue` cannot be used for this:
/// it does not forward to the native `WM_SETTEXT` handler that auto-detects a
/// `{\rtf` prefix, so it just stores the markup as literal text (confirmed by
/// a round-trip mismatch where `GetValue()` returned the raw RTF source
/// unchanged). `EM_STREAMIN` is the documented, explicit way to load RTF into
/// a `RichEdit` control, and is why this needs a raw `SendMessageW` call rather
/// than a wx-level API — the same pattern already used for letter-spacing
/// (`EM_SETCHARFORMAT`) in `document_manager::apply_readability_format_to_ctrl`.
///
/// Returns `false` if the control has no native handle yet or the stream
/// didn't fully complete, in which case callers should fall back to the
/// plain-text + segment-loop path rather than trust partial content.
pub fn stream_rtf_into_ctrl(text_ctrl: TextCtrl, rtf: &str) -> bool {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{EDITSTREAM, EM_STREAMIN, SF_RTF},
			WindowsAndMessaging::SendMessageW,
		},
	};

	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return false;
	}
	let hwnd = HWND(hwnd_ptr);
	let mut cursor = RtfStreamCursor { data: rtf.as_bytes(), pos: 0 };
	let mut stream =
		EDITSTREAM { dwCookie: addr_of_mut!(cursor) as usize, dwError: 0, pfnCallback: Some(rtf_stream_read_callback) };
	unsafe {
		SendMessageW(hwnd, EM_STREAMIN, Some(WPARAM(SF_RTF as usize)), Some(LPARAM(addr_of_mut!(stream) as isize)));
	}
	stream.dwError == 0 && cursor.pos == cursor.data.len()
}
