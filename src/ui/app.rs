use wxdragon::prelude::*;

use super::MainWindow;

pub struct PaperbackApp {
	_main_window: MainWindow,
}

impl PaperbackApp {
	pub fn new(_app: App) -> Self {
		let main_window = MainWindow::new();
		wxdragon::app::set_top_window(main_window.frame());
		main_window.show();
		Self { _main_window: main_window }
	}
}
