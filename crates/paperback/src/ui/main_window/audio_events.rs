use super::MainWindow;
use crate::{
	ui::{audio, navigation::announce_for_command},
	windows_audio::MediaButton,
};

impl MainWindow {
	/// A play, pause or skip button of headphones or a keyboard. Does nothing while a dialog such as Options holds the config, since the handlers lock it and would freeze the window.
	pub(crate) fn handle_media_button(&self, button: MediaButton) {
		let free = self.config.try_lock().is_ok() && self.doc_manager.try_lock().is_ok();
		if !free {
			return;
		}
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
