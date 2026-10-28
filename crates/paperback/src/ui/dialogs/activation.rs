use objc::{msg_send, runtime::Object, sel, sel_impl};
use wxdragon::prelude::*;

/// Cocoa data views can swallow keypad Enter before the default button, and wxWidgets may report keypad Enter as Return depending on the keyboard layout.
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
		// wxWidgets owns this live NSButton and we are on the UI thread; a native click lets the OK handler resolve the selection before closing.
		unsafe {
			let native_button = button.get_handle().cast::<Object>();
			let _: () = msg_send![native_button, performClick: std::ptr::null_mut::<Object>()];
		}
	});
}
