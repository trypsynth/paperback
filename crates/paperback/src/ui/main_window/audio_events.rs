use super::MainWindow;
use crate::{
	ui::{audio, navigation::announce_for_command},
	windows_audio::MediaButton,
};

impl MainWindow {
	/// Windows switched its default output, such as when Bluetooth headphones connected or went away.
	pub(crate) fn handle_audio_device_changed(&self) {
		tracing::info!("the default audio output changed");
		// Skipped when busy with something else, since the next switch or the next play finds the new output anyway.
		if let Ok(mut dm) = self.doc_manager.try_lock() {
			dm.follow_default_audio_device();
		}
	}

	/// A play, pause or skip button of headphones or a keyboard.
	pub(crate) fn handle_media_button(&self, button: MediaButton) {
		match button {
			MediaButton::Play | MediaButton::Pause => {
				let Ok(mut dm) = self.doc_manager.try_lock() else { return };
				if let Err(message) = dm.set_media_playing(button == MediaButton::Play) {
					drop(dm);
					announce_for_command(self.live_region_label, false, message);
				}
			}
			MediaButton::Next | MediaButton::Previous => {
				audio::handle_seek_audio(
					&self.doc_manager,
					&self.config,
					self.live_region_label,
					button == MediaButton::Next,
					false,
				);
			}
		}
	}
}
