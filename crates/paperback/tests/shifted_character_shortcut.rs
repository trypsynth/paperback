#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

use paperback_core::config::{ActionId, KeyChord};

/// On a US layout `:` is Shift+;, as `;` is Shift+, on a Finnish one, so this is #1134 on the layout CI has: an accelerator for Ctrl+: would land on Ctrl+;.
#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn a_chord_on_a_shifted_character_does_not_take_the_unshifted_key() {
	let app = common::launch_with("shifted-character-chord", |config| {
		config.app.shortcuts.set_chord(ActionId::Options, Some(KeyChord::new(true, false, false, ":")));
		config.app.shortcuts.set_chord(ActionId::DecreaseAudioVolume, None);
	});
	common::wait_for_reader_focus(app.pid);
	common::press_ctrl_punctuation(';');
	assert!(
		!common::window_appears(app.pid, "Settings", Duration::from_secs(3)),
		"Ctrl+; opened Settings, bound to Ctrl+:"
	);
}
