//! Edit > Copy and Edit > Select All on Windows and Linux.

use std::{rc::Rc, sync::Mutex};

use wxdragon::clipboard::Clipboard;

use super::DocumentManager;

/// Puts the selection, or the whole document after a Select All, on the clipboard. The document
/// manager is unlocked before the clipboard write.
pub fn handle_copy(dm: &Rc<Mutex<DocumentManager>>) {
	let text = dm.lock().unwrap().text_to_copy();
	if let Some(text) = text {
		Clipboard::get().set_text(&text);
	}
}

pub fn handle_select_all(dm: &Rc<Mutex<DocumentManager>>) {
	if let Some(tab) = dm.lock().unwrap().active_tab() {
		tab.text_ctrl.select_all();
	}
}
