use objc::{msg_send, runtime::Object, sel, sel_impl};
use wxdragon::prelude::*;

/// Cocoa data views activate rows for Return, but can swallow keypad Enter before the default button.
/// wxWidgets can report keypad Enter as Return depending on the keyboard layout.
pub(super) fn bind_enter_button(dialog: Dialog, button: Button) {
	dialog.bind_internal(EventType::CHAR_HOOK, move |event| {
		if !matches!(event.get_key_code(), Some(WXK_RETURN | WXK_NUMPAD_ENTER))
			|| event.control_down()
			|| event.alt_down()
			|| !button.is_enabled()
		{
			event.skip(true);
			return;
		}
		event.skip(false);
		// wxWidgets owns this live NSButton; the callback runs on the UI thread. A native click
		// reaches the dialog's OK handler, which resolves the current selection before closing.
		unsafe {
			let native_button = button.get_handle().cast::<Object>();
			let _: () = msg_send![native_button, performClick: std::ptr::null_mut::<Object>()];
		}
	});
}
