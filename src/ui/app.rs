use std::{env, path::Path, rc::Rc, sync::Mutex};

use wxdragon::prelude::*;

use super::MainWindow;
use crate::{config::ConfigManager, translation_manager::TranslationManager};

pub struct PaperbackApp {
	_config: Rc<Mutex<ConfigManager>>,
	_main_window: MainWindow,
}

impl PaperbackApp {
	pub fn new(_app: App) -> Self {
		let mut config = ConfigManager::new();
		let _ = config.initialize();
		{
			let mut translations = TranslationManager::instance().lock().unwrap();
			translations.initialize();
			let preferred_language = config.get_app_string("language", "");
			if !preferred_language.is_empty() {
				translations.set_language(&preferred_language);
			}
		}
		let config = Rc::new(Mutex::new(config));
		let main_window = MainWindow::new(Rc::clone(&config));
		wxdragon::app::set_top_window(main_window.frame());
		main_window.show();
		open_from_command_line(&main_window);
		if config.lock().unwrap().get_app_bool("check_for_updates_on_startup", true) {
			main_window.check_for_updates(true);
		}
		Self { _config: config, _main_window: main_window }
	}
}

fn open_from_command_line(main_window: &MainWindow) {
	if let Some(path) = env::args().nth(1) {
		main_window.open_file(Path::new(&path));
	}
}
