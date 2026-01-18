#![windows_subsystem = "windows"]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod ui;

use ui::MainWindow;

fn main() {
	let _ = wxdragon::main(|_| {
		let main_window = MainWindow::new();
		main_window.show();
	});
}
