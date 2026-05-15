#![cfg_attr(not(test), windows_subsystem = "windows")]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod config_ext;
mod ipc;
mod legacy_config;
mod translation_manager;
mod ui;

use ui::PaperbackApp;
use wxdragon::prelude::{Appearance, set_appearance};

fn main() {
	let _ = wxdragon::main(|app| {
		let _ = set_appearance(Appearance::System);
		let app_state = PaperbackApp::new(app);
		let _ = Box::leak(Box::new(app_state));
	});
}
