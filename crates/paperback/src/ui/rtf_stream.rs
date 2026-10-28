use std::ptr::{addr_of_mut, copy_nonoverlapping};

use wxdragon::prelude::*;

struct RtfStreamCursor<'a> {
	data: &'a [u8],
	pos: usize,
}

// RichEdit calls this synchronously inside `SendMessageW`, so the caller's `RtfStreamCursor` borrow stays valid for every call.
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

/// Uses raw `EM_STREAMIN` because `wxTextCtrl::SetValue` never reaches the native `{\rtf` detection in `WM_SETTEXT` and stores the markup as literal text.
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

/// The selection is saved and restored around the collapse to the end: a caret left there made the window pump think the reader had reached the loaded end and append on every tick until the whole document loaded.
pub fn append_rtf_into_ctrl(text_ctrl: TextCtrl, rtf: &str) -> bool {
	use windows::Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{CHARRANGE, EDITSTREAM, EM_EXGETSEL, EM_EXSETSEL, EM_STREAMIN, SF_RTF, SFF_SELECTION},
			WindowsAndMessaging::SendMessageW,
		},
	};
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return false;
	}
	let hwnd = HWND(hwnd_ptr);
	let mut previous = CHARRANGE { cpMin: 0, cpMax: 0 };
	unsafe {
		SendMessageW(hwnd, EM_EXGETSEL, Some(WPARAM(0)), Some(LPARAM(&raw mut previous as isize)));
	}
	// -1/-1 is RichEdit's own idiom for the end of the text, so this does not need the length.
	let end = CHARRANGE { cpMin: -1, cpMax: -1 };
	unsafe {
		SendMessageW(hwnd, EM_EXSETSEL, Some(WPARAM(0)), Some(LPARAM(&raw const end as isize)));
	}
	let mut cursor = RtfStreamCursor { data: rtf.as_bytes(), pos: 0 };
	let mut stream =
		EDITSTREAM { dwCookie: addr_of_mut!(cursor) as usize, dwError: 0, pfnCallback: Some(rtf_stream_read_callback) };
	unsafe {
		SendMessageW(
			hwnd,
			EM_STREAMIN,
			Some(WPARAM((SF_RTF | SFF_SELECTION) as usize)),
			Some(LPARAM(addr_of_mut!(stream) as isize)),
		);
	}
	// Restored even when the stream failed: the caller rebuilds from the caret, so a caret left at the end would decide where the rebuild centres.
	unsafe {
		SendMessageW(hwnd, EM_EXSETSEL, Some(WPARAM(0)), Some(LPARAM(&raw const previous as isize)));
	}
	stream.dwError == 0 && cursor.pos == cursor.data.len()
}
