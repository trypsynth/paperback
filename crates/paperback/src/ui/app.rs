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

use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

use super::MainWindow;
use crate::{
	ipc::{IPC_COMMAND_ACTIVATE, IpcCommand, SINGLE_INSTANCE_NAME, normalize_cli_path},
	translation_manager::TranslationManager,
};

pub struct PaperbackApp {
	_config: Rc<Mutex<ConfigManager>>,
	_main_window: Rc<MainWindow>,
	_pipe_server: PipeServer,
	_single_instance_checker: Option<SingleInstanceChecker>,
}

static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

impl PaperbackApp {
	pub fn new(_app: App) -> Self {
		crate::legacy_config::migrate_if_needed();
		let mut config = ConfigManager::new();
		let _ = config.initialize(crate::config_ext::config_toml_path());
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
		if let Some(checker) = single_instance_checker.as_ref()
			&& checker.is_another_running()
		{
			let cmd = ipc_command_from_cli();
			tracing::info!(command = ?cmd, "another instance is running, forwarding command and exiting");
			send_ipc_command(cmd);
			process::exit(0);
		}
		let main_window = Rc::new(MainWindow::new(Rc::clone(&config)));
		MAIN_WINDOW_PTR.store(Rc::as_ptr(&main_window) as usize, Ordering::SeqCst);
		wxdragon::app::set_top_window(main_window.frame());
		let pipe_server = start_pipe_server(&Rc::clone(&main_window));
		main_window.show();
		#[cfg(target_os = "macos")]
		_app.on_reopen_app(|| {
			if let Some(window) = main_window_from_ptr() {
				window.show_from_dock();
			}
		});
		open_from_command_line(&main_window);
		let (check_updates, channel) = {
			let cfg = config.lock().unwrap();
			(cfg.get_app_bool("check_for_updates_on_startup", true), crate::config_ext::get_update_channel(&cfg))
		};
		if check_updates {
			MainWindow::check_for_updates(true, channel);
		}
		Self {
			_config: config,
			_main_window: main_window,
			_pipe_server: pipe_server,
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

pub fn main_window_from_ptr() -> Option<&'static MainWindow> {
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

// Replaces wxWidgets DDE which has no access controls; any process in the
// same desktop session could send arbitrary OpenFile commands.  Named pipes use
// the default security descriptor, which restricts connections to the same user
// + SYSTEM/Administrators.  The pipe name is also scoped by USERNAME so
// different users on the same machine never share a pipe.
#[cfg(windows)]
mod pipe {
	use std::{ffi::OsStr, os::windows::ffi::OsStrExt as _};

	use windows::{
		Win32::{
			Foundation::{CloseHandle, ERROR_PIPE_CONNECTED, HANDLE},
			Storage::FileSystem::{
				CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, OPEN_EXISTING,
				ReadFile, WriteFile,
			},
			System::Pipes::{ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, NAMED_PIPE_MODE, WaitNamedPipeW},
		},
		core::PCWSTR,
	};

	const BUF: usize = 4096;
	const GENERIC_WRITE: u32 = 0x4000_0000;
	// Raw pipe constants (Windows SDK values; windows-rs doesn't export all of these
	// without a larger feature matrix).
	const PIPE_ACCESS_INBOUND: u32 = 0x0000_0001;
	const PIPE_FLAG_FIRST_INSTANCE: u32 = 0x0008_0000; // FILE_FLAG_FIRST_PIPE_INSTANCE
	const PIPE_UNLIMITED_INSTANCES: u32 = 255;

	fn wide_nul(s: &str) -> Vec<u16> {
		OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
	}

	/// Try to create the server-side named pipe instance.
	/// Returns `None` when the pipe already exists (another instance is running).
	pub fn try_create_server(pipe_name: &str) -> Option<HANDLE> {
		let name = wide_nul(pipe_name);
		let handle = unsafe {
			CreateNamedPipeW(
				PCWSTR(name.as_ptr()),
				FILE_FLAGS_AND_ATTRIBUTES(PIPE_ACCESS_INBOUND | PIPE_FLAG_FIRST_INSTANCE),
				NAMED_PIPE_MODE(0), // PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT = 0
				PIPE_UNLIMITED_INSTANCES,
				0,
				BUF as u32,
				0,
				None,
			)
		};
		if handle.is_invalid() { None } else { Some(handle) }
	}

	/// Accept one connection, read the payload, disconnect, repeat.
	/// HANDLE is !Send; convert to raw usize so the closure can cross the thread boundary.
	pub fn serve_loop(handle: HANDLE, on_data: impl Fn(Vec<u8>) + Send + 'static) {
		let raw = handle.0 as usize;
		std::thread::spawn(move || {
			let h = HANDLE(raw as *mut _);
			loop {
				let conn = unsafe { ConnectNamedPipe(h, None) };
				let ready =
					conn.is_ok() || unsafe { windows::Win32::Foundation::GetLastError() } == ERROR_PIPE_CONNECTED;
				if ready {
					let mut buf = vec![0u8; BUF];
					let mut n = 0u32;
					let ok = unsafe { ReadFile(h, Some(&mut buf), Some(&raw mut n), None) };
					if ok.is_ok() && n > 0 {
						on_data(buf[..n as usize].to_vec());
					}
				}
				let _ = unsafe { DisconnectNamedPipe(h) };
			}
		});
	}

	pub fn send(pipe_name: &str, payload: &str) {
		let name = wide_nul(pipe_name);
		// Allow up to 2 s for the server to become ready.
		let _ = unsafe { WaitNamedPipeW(PCWSTR(name.as_ptr()), 2000) };
		let Ok(file) = (unsafe {
			CreateFileW(
				PCWSTR(name.as_ptr()),
				GENERIC_WRITE,
				FILE_SHARE_MODE(0),
				None,
				OPEN_EXISTING,
				FILE_ATTRIBUTE_NORMAL,
				None, // hTemplateFile
			)
		}) else {
			return;
		};
		let _ = unsafe { WriteFile(file, Some(payload.as_bytes()), None, None) };
		let _ = unsafe { CloseHandle(file) };
	}
}

// Uses $XDG_RUNTIME_DIR which is owned by the user with mode 700, so only the
// same user (and root) can connect — no libc / SO_PEERCRED needed.
// The socket name is also suffixed with the username as belt-and-suspenders.
#[cfg(target_os = "linux")]
mod pipe_unix {
	use std::{
		io::{Read, Write},
		os::unix::net::{UnixListener, UnixStream},
		path::PathBuf,
	};

	pub fn socket_path() -> Option<PathBuf> {
		let dir = std::env::var("XDG_RUNTIME_DIR").ok()?;
		let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
		Some(std::path::Path::new(&dir).join(format!("paperback-{user}.sock")))
	}

	/// Create the listening socket, removing any stale file first.
	/// Returns None if XDG_RUNTIME_DIR is not set.
	pub fn try_create_server() -> Option<UnixListener> {
		let path = socket_path()?;
		// Safe to remove: SingleInstanceChecker already confirmed no other instance.
		let _ = std::fs::remove_file(&path);
		UnixListener::bind(&path).ok()
	}

	pub fn serve_loop(listener: UnixListener, on_data: impl Fn(Vec<u8>) + Send + 'static) {
		std::thread::spawn(move || {
			for conn in listener.incoming() {
				if let Ok(mut stream) = conn {
					let mut buf = vec![0u8; 4096];
					if let Ok(n) = stream.read(&mut buf) {
						if n > 0 {
							on_data(buf[..n].to_vec());
						}
					}
				}
			}
		});
	}

	pub fn send(payload: &str) {
		let Some(path) = socket_path() else { return };
		if let Ok(mut stream) = UnixStream::connect(&path) {
			let _ = stream.write_all(payload.as_bytes());
		}
	}
}

/// Opaque guard; the server thread runs for the lifetime of this value.
pub struct PipeServer {
	// Intentionally empty — the thread runs until the process exits.
	// Holding this type in PaperbackApp makes the intent explicit.
}

fn start_pipe_server(main_window: &Rc<MainWindow>) -> PipeServer {
	#[cfg(not(any(target_os = "linux", target_os = "windows")))]
	let _ = main_window;
	#[cfg(windows)]
	{
		use patois::t;

		use crate::ipc::{decode_execute_payload, named_pipe_path};
		let name = named_pipe_path();
		if let Some(handle) = pipe::try_create_server(&name) {
			tracing::info!(pipe = %name, "IPC server started");
			pipe::serve_loop(handle, move |data| {
				if let Some(cmd) = decode_execute_payload(&data) {
					wxdragon::call_after(Box::new(move || {
						if let Some(window) = main_window_from_ptr() {
							window.handle_ipc_command(cmd);
						}
					}));
					wake_up_idle();
				}
			});
		} else {
			tracing::error!(pipe = %name, "failed to create IPC server; named pipe already exists");
			let dialog = MessageDialog::builder(main_window.frame(), &t("Failed to create IPC server"), &t("Warning"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconWarning | MessageDialogStyle::Centre)
				.build();
			dialog.show_modal();
		}
	}
	#[cfg(target_os = "linux")]
	{
		use crate::ipc::decode_execute_payload;
		if let Some(listener) = pipe_unix::try_create_server() {
			if let Some(path) = pipe_unix::socket_path() {
				tracing::info!(socket = %path.display(), "IPC server started");
			}
			pipe_unix::serve_loop(listener, move |data| {
				if let Some(cmd) = decode_execute_payload(&data) {
					wxdragon::call_after(Box::new(move || {
						if let Some(window) = main_window_from_ptr() {
							window.handle_ipc_command(cmd);
						}
					}));
				}
			});
		} else {
			tracing::warn!("XDG_RUNTIME_DIR not set; IPC file forwarding unavailable");
		}
	}
	PipeServer {}
}

fn send_ipc_command(command: IpcCommand) {
	tracing::debug!(command = ?command, "sending IPC command to existing instance");
	let payload = match &command {
		IpcCommand::Activate => IPC_COMMAND_ACTIVATE.to_string(),
		#[cfg(any(target_os = "linux", target_os = "windows"))]
		IpcCommand::ToggleVisibility => crate::ipc::IPC_COMMAND_TOGGLE_VISIBILITY.to_string(),
		IpcCommand::OpenFile(path) => path.to_string_lossy().to_string(),
	};
	#[cfg(windows)]
	{
		use windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow;

		use crate::ipc::named_pipe_path;
		let _ = unsafe { AllowSetForegroundWindow(u32::MAX) };
		pipe::send(&named_pipe_path(), &payload);
	}
	#[cfg(target_os = "linux")]
	pipe_unix::send(&payload);
	#[cfg(not(any(windows, target_os = "linux")))]
	let _ = payload;
}
