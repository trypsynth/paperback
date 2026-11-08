#[cfg(target_os = "macos")]
use std::path::Path;
use std::{
	env,
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
};

use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

use super::MainWindow;
#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::ipc::commands_from_message;
use crate::{
	config_ext::{config_toml_path, get_update_channel},
	ipc::target_for_argument,
	legacy_config::migrate_if_needed,
	translation_manager::TranslationManager,
};

pub struct PaperbackApp {
	_config: Rc<Mutex<ConfigManager>>,
	_main_window: Rc<MainWindow>,
}

static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

impl PaperbackApp {
	pub fn new(app: App) -> Self {
		#[cfg(not(target_os = "macos"))]
		let _ = app;
		migrate_if_needed();
		let mut config = ConfigManager::new();
		let _ = config.initialize(config_toml_path());
		{
			let mut translations = TranslationManager::instance().lock().unwrap();
			translations.initialize();
			let preferred_language = config.get_app_string("language", "");
			if !preferred_language.is_empty() {
				translations.set_language(&preferred_language);
			}
		}
		let config = Rc::new(Mutex::new(config));
		let main_window = Rc::new(MainWindow::new(Rc::clone(&config)));
		MAIN_WINDOW_PTR.store(Rc::as_ptr(&main_window) as usize, Ordering::SeqCst);
		set_top_window(main_window.frame());
		#[cfg(any(target_os = "linux", target_os = "windows"))]
		listen_for_other_copies();
		main_window.show();
		#[cfg(target_os = "linux")]
		crate::linux_integration::maybe_run_first_run_setup(main_window.frame(), &config);
		#[cfg(target_os = "macos")]
		app.on_should_terminate(|| {
			tracing::debug!("allowing native macOS termination request");
			true
		});
		#[cfg(target_os = "macos")]
		app.on_reopen_app(|| {
			if let Some(window) = main_window_from_ptr() {
				window.show_from_dock();
			}
		});
		// Finder delivers documents via the odoc Apple Event, not argv, so "Open with" and double-click never reach `open_from_command_line`.
		#[cfg(target_os = "macos")]
		app.on_open_files(|files| {
			if let Some(window) = main_window_from_ptr() {
				window.show_from_dock();
				for file in &files {
					tracing::info!(path = %file, "opening file from macOS open-files event");
					window.open_file(Path::new(file), false);
				}
			}
		});
		open_from_command_line(&main_window);
		// Queued so the main window is up and focused before the question takes the focus.
		wx_utils::post_to_ui(|| {
			if let Some(window) = main_window_from_ptr() {
				crate::problem_report::offer_after_crash(window.frame());
			}
		});
		let (check_updates, channel) = {
			let cfg = config.lock().unwrap();
			(cfg.get_app_bool("check_for_updates_on_startup", true), get_update_channel(&cfg))
		};
		if check_updates {
			main_window.check_for_updates(true, channel);
		}
		Self { _config: config, _main_window: main_window }
	}
}

fn open_from_command_line(main_window: &MainWindow) {
	if let Some(argument) = env::args().nth(1) {
		let target = target_for_argument(&argument);
		tracing::info!(?target, "opening from command line");
		main_window.open_target(target, true);
	}
}

pub fn main_window_from_ptr() -> Option<&'static MainWindow> {
	let ptr = MAIN_WINDOW_PTR.load(Ordering::SeqCst);
	if ptr == 0 {
		return None;
	}
	unsafe { (ptr as *const MainWindow).as_ref() }
}

/// The first copy's door, kept from `main`, where the check for a running copy happens before wxWidgets starts, until the main window can take messages.
#[cfg(any(target_os = "linux", target_os = "windows"))]
static DOOR: Mutex<Option<doorbell::Door>> = Mutex::new(None);

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub fn keep_door(door: doorbell::Door) {
	*DOOR.lock().unwrap() = Some(door);
}

/// Copies started while this one runs hand over their files here. Until it's called, their messages wait.
#[cfg(any(target_os = "linux", target_os = "windows"))]
fn listen_for_other_copies() {
	let Some(door) = DOOR.lock().unwrap().take() else { return };
	door.listen(|message| {
		wx_utils::post_to_ui(move || {
			if let Some(window) = main_window_from_ptr() {
				for command in commands_from_message(&message) {
					window.handle_ipc_command(command);
				}
			}
		});
	});
}

/// Force-closes so normal persistence and audio cleanup run, which the updater's default `process::exit` would skip, without macOS's hide-on-close veto.
pub fn quit_for_update() {
	if let Some(window) = main_window_from_ptr() {
		window.frame().enable(false);
		window.announce_update_restart();
	}
	// macOS posts live-region announcements on the next run-loop iteration, so the window stays alive briefly without blocking the UI thread.
	std::thread::spawn(|| {
		std::thread::sleep(std::time::Duration::from_secs(2));
		wx_utils::post_to_ui(finish_quit_for_update);
	});
}

fn finish_quit_for_update() {
	if let Some(window) = main_window_from_ptr() {
		window.frame().close(true);
	}
	if let Some(app) = wxdragon::app::get_app_instance() {
		app.exit_main_loop();
	}
}
