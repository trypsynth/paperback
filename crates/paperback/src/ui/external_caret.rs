//! Reports each caret move in the reading control, and whether another process, such as a screen reader, made it through the Text Object Model.

use std::cell::Cell;

use windows::Win32::{
	Foundation::{HWND, LPARAM, LRESULT, WPARAM},
	System::Com::{CoGetCallContext, IServerSecurity},
	UI::{
		Controls::{
			NMHDR,
			RichEdit::{EM_GETEVENTMASK, EM_SETEVENTMASK, EN_SELCHANGE, ENM_SELCHANGE},
		},
		Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
		WindowsAndMessaging::{GetParent, SendMessageW, WM_NCDESTROY, WM_NOTIFY},
	},
};
use wxdragon::prelude::*;

use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaretMove {
	pub before: CtrlPos,
	pub after: CtrlPos,
	/// Made by another process through the Text Object Model.
	pub external: bool,
}

struct Watch {
	text_ctrl: ReaderCtrl,
	caret: Cell<CtrlPos>,
	on_move: Box<dyn Fn(CaretMove)>,
}

impl Watch {
	fn selection_changed(&self) {
		let after = self.text_ctrl.get_insertion_point();
		let before = self.caret.replace(after);
		if before == after {
			return;
		}
		let external = unsafe { CoGetCallContext::<IServerSecurity>() }.is_ok();
		(self.on_move)(CaretMove { before, after, external });
	}
}

/// Calls `on_move` after each caret move in `text_ctrl`, until the control is destroyed.
pub fn watch_caret(text_ctrl: ReaderCtrl, on_move: impl Fn(CaretMove) + 'static) {
	let hwnd = HWND(text_ctrl.widget().get_handle());
	let Ok(parent) = (unsafe { GetParent(hwnd) }) else {
		return;
	};
	let watch = Watch { text_ctrl, caret: Cell::new(text_ctrl.get_insertion_point()), on_move: Box::new(on_move) };
	let id = hwnd.0.addr();
	let data = Box::into_raw(Box::new(watch)).expose_provenance();
	if !unsafe { SetWindowSubclass(parent, Some(parent_proc), id, data) }.as_bool() {
		drop(unsafe { Box::from_raw(std::ptr::with_exposed_provenance_mut::<Watch>(data)) });
		return;
	}
	let events = unsafe { SendMessageW(hwnd, EM_GETEVENTMASK, None, None) };
	let with_selection = LPARAM(events.0 | isize::try_from(ENM_SELCHANGE).unwrap_or_default());
	unsafe { SendMessageW(hwnd, EM_SETEVENTMASK, None, Some(with_selection)) };
	let handle = text_ctrl.widget().handle_ptr();
	text_ctrl.bind_internal(EventType::DESTROY, move |event| {
		if event.event_object_ptr() == handle {
			unwatch(parent, id, data);
		}
		event.skip(true);
	});
}

/// Frees the watch once its subclass is removed; a second call finds none and does nothing.
fn unwatch(parent: HWND, id: usize, data: usize) {
	if unsafe { RemoveWindowSubclass(parent, Some(parent_proc), id) }.as_bool() {
		drop(unsafe { Box::from_raw(std::ptr::with_exposed_provenance_mut::<Watch>(data)) });
	}
}

/// `id` is the watched control's window and `data` its [`Watch`].
unsafe extern "system" fn parent_proc(
	hwnd: HWND,
	message: u32,
	wparam: WPARAM,
	lparam: LPARAM,
	id: usize,
	data: usize,
) -> LRESULT {
	if message == WM_NOTIFY {
		let header = unsafe { &*std::ptr::with_exposed_provenance::<NMHDR>(lparam.0.cast_unsigned()) };
		if header.code == EN_SELCHANGE && header.hwndFrom.0.addr() == id {
			unsafe { &*std::ptr::with_exposed_provenance::<Watch>(data) }.selection_changed();
		}
	} else if message == WM_NCDESTROY {
		unwatch(hwnd, id, data);
	}
	unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}
