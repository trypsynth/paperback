#![windows_subsystem = "windows"]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod config;
mod document;
mod html_to_text;
mod live_region;
mod parser;
mod reader_core;
mod session;
mod translation_manager;
mod ui;
mod ui_types;
mod update;
mod utils;
mod version;
mod xml_to_text;

use ui::PaperbackApp;
use wxdragon::prelude::{Appearance, SystemOptions, set_appearance};

fn main() {
	SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
	let _ = wxdragon::main(|app| {
		// Allow the app to follow the OS dark/light mode setting.
		let _ = set_appearance(Appearance::System);
		let app_state = PaperbackApp::new(app);
		let _ = Box::leak(Box::new(app_state));
	});
}
