#![windows_subsystem = "windows"]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

pub mod config;
pub mod document;
mod html_to_text;
pub mod parser;
pub mod reader_core;
pub mod session;
pub mod translation_manager;
mod ui;
pub mod ui_types;
pub mod update;
mod utils;
pub mod version;
mod xml_to_text;

use ui::PaperbackApp;

fn main() {
	let _ = wxdragon::main(|app| {
		let app_state = PaperbackApp::new(app);
		let _ = Box::leak(Box::new(app_state));
	});
}
