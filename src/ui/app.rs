use std::{env, path::Path, rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::MainWindow;
use crate::config::ConfigManager;

pub struct PaperbackApp {
	_config: Rc<Mutex<ConfigManager>>,
	_main_window: MainWindow,
}

impl PaperbackApp {
	pub fn new(_app: App) -> Self {
		let mut config = ConfigManager::new();
		config.initialize();
		let config = Rc::new(Mutex::new(config));
		let main_window = MainWindow::new(Rc::clone(&config));
		wxdragon::app::set_top_window(main_window.frame());
		main_window.show();
		open_from_command_line(&main_window);
		Self { _config: config, _main_window: main_window }
	}
}

fn open_from_command_line(main_window: &MainWindow) {
	if let Some(path) = env::args().nth(1) {
		main_window.open_file(Path::new(&path));
	}
}
