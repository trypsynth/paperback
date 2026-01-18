use std::{cell::RefCell, env, path::Path, rc::Rc};

use wxdragon::prelude::*;

use super::MainWindow;
use crate::config::ConfigManager;

pub struct PaperbackApp {
	_config: Rc<RefCell<ConfigManager>>,
	_main_window: MainWindow,
}

impl PaperbackApp {
	pub fn new(_app: App) -> Self {
		let mut config = ConfigManager::new();
		config.initialize();
		let config = Rc::new(RefCell::new(config));

		let main_window = MainWindow::new(Rc::clone(&config));
		wxdragon::app::set_top_window(main_window.frame());
		main_window.show();
		restore_previous_documents(&main_window, &config);
		open_from_command_line(&main_window);

		Self { _config: config, _main_window: main_window }
	}
}

fn restore_previous_documents(main_window: &MainWindow, config: &Rc<RefCell<ConfigManager>>) {
	let restore = config.borrow().get_app_bool("restore_previous_documents", true);
	if !restore {
		return;
	}
	for path in config.borrow().get_opened_documents_existing() {
		main_window.open_file(Path::new(&path));
	}
}

fn open_from_command_line(main_window: &MainWindow) {
	if let Some(path) = env::args().nth(1) {
		main_window.open_file(Path::new(&path));
	}
}
