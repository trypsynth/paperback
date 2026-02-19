use std::{
	env,
	path::Path,
	process,
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
};

use wxdragon::{prelude::*, translations::translate as t};

use super::MainWindow;
use crate::{
	config::ConfigManager,
	ipc::{
		IPC_COMMAND_ACTIVATE, IPC_HOST_LOCALHOST, IPC_SERVICE, IPC_TOPIC_OPEN_FILE, IpcCommand, SINGLE_INSTANCE_NAME,
		decode_execute_payload, normalize_cli_path,
	},
	translation_manager::TranslationManager,
};

pub struct PaperbackApp {
	_config: Rc<Mutex<ConfigManager>>,
	_main_window: Rc<MainWindow>,
	_ipc_server: IPCServer,
	_single_instance_checker: Option<SingleInstanceChecker>,
}

static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

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
		let single_instance_checker = SingleInstanceChecker::new(SINGLE_INSTANCE_NAME, None);
		if let Some(checker) = single_instance_checker.as_ref() {
			if checker.is_another_running() {
				send_ipc_command(ipc_command_from_cli());
				process::exit(0);
			}
		}
		let main_window = Rc::new(MainWindow::new(Rc::clone(&config)));
		MAIN_WINDOW_PTR.store(Rc::as_ptr(&main_window) as usize, Ordering::SeqCst);
		wxdragon::app::set_top_window(main_window.frame());
		let ipc_server = start_ipc_server(&Rc::clone(&main_window));
		main_window.show();
		open_from_command_line(&main_window);
		let (check_updates, channel) = {
			let cfg = config.lock().unwrap();
			(cfg.get_app_bool("check_for_updates_on_startup", true), cfg.get_update_channel())
		};
		if check_updates {
			MainWindow::check_for_updates(true, channel);
		}
		Self {
			_config: config,
			_main_window: main_window,
			_ipc_server: ipc_server,
			_single_instance_checker: single_instance_checker,
		}
	}
}

fn open_from_command_line(main_window: &MainWindow) {
	if let Some(path) = env::args().nth(1) {
		let normalized = normalize_cli_path(Path::new(&path));
		main_window.open_file(&normalized);
	}
}

fn main_window_from_ptr() -> Option<&'static MainWindow> {
	let ptr = MAIN_WINDOW_PTR.load(Ordering::SeqCst);
	if ptr == 0 {
		return None;
	}
	unsafe { (ptr as *const MainWindow).as_ref() }
}

fn ipc_command_from_cli() -> IpcCommand {
	if let Some(path) = env::args().nth(1) {
		let normalized = normalize_cli_path(Path::new(&path));
		return IpcCommand::OpenFile(normalized);
	}
	IpcCommand::Activate
}

fn send_ipc_command(command: IpcCommand) {
	let client = IPCClient::new();
	let Some(conn) = client.make_connection(IPC_HOST_LOCALHOST, IPC_SERVICE, IPC_TOPIC_OPEN_FILE) else {
		return;
	};
	let payload = match command {
		IpcCommand::Activate => IPC_COMMAND_ACTIVATE.to_string(),
		IpcCommand::OpenFile(path) => path.to_string_lossy().to_string(),
	};
	let _ = conn.execute_string(&payload);
	let _ = conn.disconnect();
}

fn start_ipc_server(main_window: &Rc<MainWindow>) -> IPCServer {
	let server = IPCServer::new(move |topic| {
		if topic != IPC_TOPIC_OPEN_FILE {
			return None;
		}
		Some(
			IPCConnection::builder()
				.on_execute({
					move |_topic, data, _format| {
						let Some(command) = decode_execute_payload(data) else {
							return false;
						};
						wxdragon::call_after(Box::new(move || {
							if let Some(window) = main_window_from_ptr() {
								window.handle_ipc_command(command);
							}
						}));
						true
					}
				})
				.build(),
		)
	});
	if !server.create(IPC_SERVICE) {
		let dialog = MessageDialog::builder(main_window.frame(), &t("Failed to create IPC server"), &t("Warning"))
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
	}
	server
}
