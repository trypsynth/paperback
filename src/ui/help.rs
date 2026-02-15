//! Help, update, and utility functions. maybe clean this up eventually.

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use std::process::{self, Command};
use std::{
	cell::RefCell,
	env,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
	},
	thread,
	time::Duration,
};

use wxdragon::{ffi, prelude::*, translations::translate as t};

use super::{dialogs, document_manager::DocumentManager};
use crate::{
	config::ConfigManager,
	parser,
	text::markdown_to_text,
	update::{self, UpdateCheckOutcome, UpdateError},
};

pub static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

thread_local! {
	static ACTIVE_PROGRESS: RefCell<Option<ProgressDialog>> = const { RefCell::new(None) };
}

pub fn run_update_check(silent: bool) {
	let current_version = env!("CARGO_PKG_VERSION").to_string();
	let is_installer = is_installer_distribution();
	thread::spawn(move || {
		let outcome = update::check_for_updates(&current_version, is_installer);
		wxdragon::call_after(Box::new(move || {
			present_update_result(outcome, silent, &current_version);
		}));
	});
}

pub fn is_installer_distribution() -> bool {
	let Ok(exe_path) = env::current_exe() else {
		return false;
	};
	let Some(exe_dir) = exe_path.parent() else {
		return false;
	};
	exe_dir.join("unins000.exe").exists()
}

pub fn readme_path() -> Option<PathBuf> {
	let exe = env::current_exe().ok()?;
	let dir = exe.parent()?;
	Some(dir.join("readme.html"))
}

pub fn show_error_message(frame: &Frame, message: &str, title: &str) {
	let dialog = MessageDialog::builder(frame, message, title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
	dialog.show_modal();
}

pub fn handle_open_containing_folder(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dir = doc_manager.lock().unwrap().active_tab().and_then(|tab| tab.file_path.parent()).map(Path::to_path_buf);
	let Some(dir) = dir else {
		show_error_message(frame, &t("Failed to open containing folder."), &t("Error"));
		return;
	};
	let url = format!("file://{}", dir.to_string_lossy());
	if !wxdragon::utils::launch_default_browser(&url, wxdragon::utils::BrowserLaunchFlags::Default) {
		show_error_message(frame, &t("Failed to open containing folder."), &t("Error"));
	}
}

pub fn handle_view_help_browser(frame: &Frame) {
	let Some(path) = readme_path() else {
		show_error_message(
			frame,
			&t("readme.html not found. Please ensure the application was built properly."),
			&t("Error"),
		);
		return;
	};
	if !path.exists() {
		show_error_message(
			frame,
			&t("readme.html not found. Please ensure the application was built properly."),
			&t("Error"),
		);
		return;
	}
	let url = format!("file://{}", path.to_string_lossy());
	if !wxdragon::utils::launch_default_browser(&url, wxdragon::utils::BrowserLaunchFlags::Default) {
		show_error_message(frame, &t("Failed to launch default browser."), &t("Error"));
	}
}

pub fn handle_view_help_paperback(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) {
	let Some(path) = readme_path() else {
		show_error_message(
			frame,
			&t("readme.html not found. Please ensure the application was built properly."),
			&t("Error"),
		);
		return;
	};
	if !path.exists() {
		show_error_message(
			frame,
			&t("readme.html not found. Please ensure the application was built properly."),
			&t("Error"),
		);
		return;
	}
	if !ensure_parser_ready_for_path(frame, &path, config) {
		return;
	}
	let _ = doc_manager.lock().unwrap().open_file(doc_manager, &path);
}

pub fn handle_donate(frame: &Frame) {
	let url = "https://paypal.me/tygillespie05";
	if !wxdragon::utils::launch_default_browser(url, wxdragon::utils::BrowserLaunchFlags::Default) {
		show_error_message(frame, &t("Failed to open donation page in browser."), &t("Error"));
	}
}

fn present_update_result(outcome: Result<UpdateCheckOutcome, UpdateError>, silent: bool, current_version: &str) {
	let parent_window = main_window_parent();
	let parent = parent_window.as_ref();
	match outcome {
		Ok(UpdateCheckOutcome::UpdateAvailable(result)) => {
			handle_update_available(parent, result, current_version);
		}
		Ok(UpdateCheckOutcome::UpToDate(latest_version)) => {
			handle_update_up_to_date(parent, silent, &latest_version);
		}
		Err(err) => {
			handle_update_error(parent, silent, &err);
		}
	}
}

fn handle_update_available(
	parent: Option<&ParentWindow>,
	result: update::UpdateAvailableResult,
	current_version: &str,
) {
	let latest_version =
		if result.latest_version.is_empty() { current_version.to_string() } else { result.latest_version };
	let plain_notes = markdown_to_text(&result.release_notes);
	let release_notes = if plain_notes.trim().is_empty() { t("No release notes were provided.") } else { plain_notes };
	let Some(parent) = parent else {
		return;
	};
	if !dialogs::show_update_dialog(parent, &latest_version, &release_notes) || result.download_url.is_empty() {
		return;
	}
	let download_url = result.download_url;
	let progress = ProgressDialog::builder(parent, &t("Paperback Update"), &t("Downloading update..."), 100)
		.with_style(ProgressDialogStyle::AutoHide | ProgressDialogStyle::AppModal | ProgressDialogStyle::RemainingTime)
		.build();
	ACTIVE_PROGRESS.with(|p| {
		*p.borrow_mut() = Some(progress);
	});
	let downloaded = Arc::new(AtomicU64::new(0));
	let total = Arc::new(AtomicU64::new(0));
	let is_running = Arc::new(AtomicBool::new(true));
	// Heartbeat thread to keep UI alive
	let hb_downloaded = downloaded.clone();
	let hb_total = total.clone();
	let hb_is_running = is_running.clone();
	thread::spawn(move || {
		while hb_is_running.load(Ordering::Relaxed) {
			let d = hb_downloaded.load(Ordering::Relaxed);
			let t = hb_total.load(Ordering::Relaxed);
			wxdragon::call_after(Box::new(move || {
				ACTIVE_PROGRESS.with(|p| {
					if let Some(dialog) = p.borrow().as_ref() {
						if t > 0 {
							let percent = i32::try_from(d.saturating_mul(100) / t).unwrap_or(i32::MAX);
							dialog.update(percent, None);
						} else {
							dialog.pulse(None);
						}
					}
				});
			}));
			thread::sleep(Duration::from_millis(200));
		}
	});
	// Download thread
	let d_downloaded = downloaded;
	let d_total = total;
	let d_is_running = is_running;
	thread::spawn(move || {
		let res = update::download_update_file(&download_url, |d, t| {
			d_downloaded.store(d, Ordering::Relaxed);
			d_total.store(t, Ordering::Relaxed);
		});
		d_is_running.store(false, Ordering::Relaxed);
		wxdragon::call_after(Box::new(move || {
			ACTIVE_PROGRESS.with(|p| {
				*p.borrow_mut() = None;
			});
			#[cfg(target_os = "windows")]
			execute_update(res);
		}));
	});
}

fn handle_update_up_to_date(parent: Option<&ParentWindow>, silent: bool, latest_version: &str) {
	if silent {
		return;
	}
	let message = if latest_version.trim().is_empty() {
		t("No updates available.")
	} else {
		let template = t("No updates available. Latest version: {}");
		template.replace("{}", latest_version)
	};
	let title = t("Info");
	let Some(parent) = parent else {
		return;
	};
	let dialog = MessageDialog::builder(parent, &message, &title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
		.build();
	dialog.show_modal();
}

fn handle_update_error(parent: Option<&ParentWindow>, silent: bool, err: &UpdateError) {
	if silent {
		return;
	}
	let (message, title) = match err {
		UpdateError::HttpError(code) if *code > 0 => {
			let template = t("Failed to check for updates. HTTP status: %d");
			(template.replacen("%d", &code.to_string(), 1), t("Error"))
		}
		_ => {
			let msg = err.to_string();
			let fallback = t("Error checking for updates.");
			(if msg.is_empty() { fallback } else { msg }, t("Error"))
		}
	};
	let Some(parent) = parent else {
		return;
	};
	let dialog = MessageDialog::builder(parent, &message, &title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build();
	dialog.show_modal();
}

#[cfg(target_os = "windows")]
fn execute_update(result: Result<PathBuf, UpdateError>) {
	let parent_window = main_window_parent();
	let Some(parent) = parent_window.as_ref() else {
		return;
	};
	match result {
		Ok(path) => {
			let is_exe = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
			let is_zip = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
			if is_exe {
				if let Err(e) = Command::new(&path).spawn() {
					let dlg = MessageDialog::builder(
						parent,
						&format!("{}: {e}", t("Failed to launch installer")),
						&t("Error"),
					)
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError)
					.build();
					dlg.show_modal();
					return;
				}
				process::exit(0);
			} else if is_zip {
				let current_exe = match env::current_exe() {
					Ok(p) => p,
					Err(e) => {
						let dlg = MessageDialog::builder(
							parent,
							&format!("{}: {e}", t("Failed to get current exe path")),
							&t("Error"),
						)
						.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError)
						.build();
						dlg.show_modal();
						return;
					}
				};
				let exe_dir = current_exe.parent().unwrap_or(&current_exe);
				let pid = process::id();
				let script = format!(
					"Start-Sleep -Seconds 1; Wait-Process -Id {}; Expand-Archive -Path '{}' -DestinationPath '{}' -Force; Remove-Item -Path '{}' -Force; Start-Process '{}'",
					pid,
					path.display(),
					exe_dir.display(),
					path.display(),
					current_exe.display()
				);
				if let Err(e) = Command::new("powershell.exe")
					.arg("-NoProfile")
					.arg("-ExecutionPolicy")
					.arg("Bypass")
					.arg("-Command")
					.arg(&script)
					.creation_flags(0x0800_0000) // CREATE_NO_WINDOW
					.spawn()
				{
					let dlg = MessageDialog::builder(
						parent,
						&format!("{}: {e}", t("Failed to launch update script")),
						&t("Error"),
					)
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError)
					.build();
					dlg.show_modal();
					return;
				}
				process::exit(0);
			} else {
				let dlg = MessageDialog::builder(parent, &t("Unknown update file format."), &t("Error"))
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError)
					.build();
				dlg.show_modal();
			}
		}
		Err(e) => {
			let dlg = MessageDialog::builder(parent, &format!("{}: {e}", t("Update failed")), &t("Error"))
				.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError)
				.build();
			dlg.show_modal();
		}
	}
}

struct ParentWindow {
	handle: *mut ffi::wxd_Window_t,
}

impl wxdragon::window::WxWidget for ParentWindow {
	fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
		self.handle
	}
}

fn main_window_parent() -> Option<ParentWindow> {
	let ptr = MAIN_WINDOW_PTR.load(Ordering::SeqCst);
	if ptr == 0 {
		return None;
	}
	let handle = ptr as *mut ffi::wxd_Window_t;
	if handle.is_null() {
		return None;
	}
	Some(ParentWindow { handle })
}

fn ensure_parser_ready_for_path(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
	if extension.is_empty() || parser::parser_supports_extension(extension) {
		return true;
	}
	let cfg = config.lock().unwrap();
	ensure_parser_for_unknown_file(frame, path, &cfg)
}

fn ensure_parser_for_unknown_file(parent: &Frame, path: &Path, config: &ConfigManager) -> bool {
	let path_str = path.to_string_lossy();
	let saved_format = config.get_document_format(&path_str);
	if !saved_format.is_empty() && parser::parser_supports_extension(&saved_format) {
		return true;
	}
	let Some(format) = dialogs::show_open_as_dialog(parent, path) else {
		return false;
	};
	if !parser::parser_supports_extension(&format) {
		let message = t("Unsupported format selected.");
		let title = t("Error");
		let dialog = MessageDialog::builder(parent, &message, &title)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return false;
	}
	config.set_document_format(&path_str, &format);
	true
}
