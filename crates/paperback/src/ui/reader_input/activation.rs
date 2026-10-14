use wxdragon::prelude::*;

/// Cocoa's read-only text view can consume Return/Space without sending a character event.
/// The hook runs before native text handling; other platforms retain their character handler.
pub(super) fn bind_activation(text_ctrl: TextCtrl, activate: impl Fn(bool) + 'static) {
	#[cfg(target_os = "macos")]
	let event_type = EventType::CHAR_HOOK;
	#[cfg(not(target_os = "macos"))]
	let event_type = EventType::CHAR;
	text_ctrl.bind_internal(event_type, move |event| {
		// Command and Option chords, including VoiceOver's Control+Option, belong to the system.
		#[cfg(target_os = "macos")]
		if event.control_down() || event.alt_down() {
			event.skip(true);
			return;
		}
		match event.get_key_code() {
			Some(WXK_RETURN | WXK_NUMPAD_ENTER) => {
				event.skip(false);
				activate(true);
			}
			Some(WXK_SPACE) => {
				event.skip(false);
				activate(false);
			}
			_ => event.skip(true),
		}
	});
}
