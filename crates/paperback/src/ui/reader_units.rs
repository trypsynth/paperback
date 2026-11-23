//! The word and visual line around a position in the reading control, as the control lays them out.

use windows::Win32::{
	Foundation::{HWND, LPARAM, WPARAM},
	UI::{
		Controls::{RichEdit::EM_FINDWORDBREAK, WB_MOVEWORDRIGHT},
		WindowsAndMessaging::SendMessageW,
	},
};
use wxdragon::prelude::*;

use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

/// The start of the next word after `position`, where Ctrl+Right moves.
pub fn word_end(text_ctrl: ReaderCtrl, position: CtrlPos) -> CtrlPos {
	let hwnd = HWND(text_ctrl.widget().get_handle());
	let action = usize::try_from(WB_MOVEWORDRIGHT.0).unwrap_or_default();
	let from = isize::try_from(position.get()).unwrap_or_default();
	let next = unsafe { SendMessageW(hwnd, EM_FINDWORDBREAK, Some(WPARAM(action)), Some(LPARAM(from))) };
	CtrlPos::from_raw(i64::try_from(next.0).unwrap_or_default()).max(position)
}

/// The visual line holding `position`, without its line break.
pub fn visual_line_at(text_ctrl: ReaderCtrl, position: CtrlPos) -> Option<(CtrlPos, CtrlPos)> {
	let (column, line) = text_ctrl.position_to_xy(position)?;
	let start = position + -column;
	Some((start, start + i64::from(text_ctrl.get_line_length(line))))
}
