use wxdragon::{event::WxEvtHandler, prelude::*};

pub(crate) fn bind_activation<C>(text_ctrl: C, activate: impl Fn(bool) + 'static)
where
	C: WxEvtHandler + Copy,
{
	text_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
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
