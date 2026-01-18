#![windows_subsystem = "windows"]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

pub mod config;
pub mod document;
mod html_to_text;
pub mod parser;
pub mod reader_core;
pub mod session;
mod ui;
pub mod ui_types;
pub mod update;
mod utils;
mod xml_to_text;

use ui::MainWindow;

fn main() {
	let _ = wxdragon::main(|_| {
		let main_window = MainWindow::new();
		main_window.show();
	});
}
