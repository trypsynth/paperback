//! Help, update, and utility functions. maybe clean this up eventually.

use std::{
	env,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	},
	thread,
};

use wxdragon::{prelude::*, translations::translate as t};
use wxdragon_sys as ffi;

use super::{dialogs, document_manager::DocumentManager};
use crate::{
	config::ConfigManager,
	parser,
	update::{self, UpdateCheckOutcome, UpdateError},
	utils::text::markdown_to_text,
};

pub static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

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
	let dm_ref = doc_manager.lock().unwrap();
	let Some(tab) = dm_ref.active_tab() else {
		return;
	};
	let Some(dir) = tab.file_path.parent() else {
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
	let _ = doc_manager.lock().unwrap().open_file(Rc::clone(doc_manager), &path);
}

pub fn handle_donate(frame: &Frame) {
	let url = "https://paypal.me/tygillespie05";
	if !wxdragon::utils::launch_default_browser(url, wxdragon::utils::BrowserLaunchFlags::Default) {
		show_error_message(frame, &t("Failed to open donation page in browser."), &t("Error"));
	}
}

fn present_update_result(outcome: Result<UpdateCheckOutcome, UpdateError>, silent: bool, current_version: &str) {
	let parent_window = main_window_parent();
	match outcome {
		Ok(UpdateCheckOutcome::UpdateAvailable(result)) => {
			let latest_version =
				if result.latest_version.is_empty() { current_version.to_string() } else { result.latest_version };
			let plain_notes = markdown_to_text(&result.release_notes);
			let release_notes =
				if plain_notes.trim().is_empty() { t("No release notes were provided.") } else { plain_notes };
			if let Some(parent) = parent_window.as_ref() {
				if dialogs::show_update_dialog(parent, &latest_version, &release_notes)
					&& !result.download_url.is_empty()
				{
					wxdragon::utils::launch_default_browser(
						&result.download_url,
						wxdragon::utils::BrowserLaunchFlags::Default,
					);
				}
			}
		}
		Ok(UpdateCheckOutcome::UpToDate(latest_version)) => {
			if silent {
				return;
			}
			let message = if latest_version.trim().is_empty() {
				t("No updates available.")
			} else {
				let template = t("No updates available. Latest version: {}");
				template.replace("{}", &latest_version)
			};
			let title = t("Info");
			if let Some(parent) = parent_window.as_ref() {
				let dialog = MessageDialog::builder(parent, &message, &title)
					.with_style(
						MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre,
					)
					.build();
				dialog.show_modal();
			}
		}
		Err(err) => {
			if silent {
				return;
			}
			let (message, title) = match err {
				UpdateError::HttpError(code) if code > 0 => {
					let template = t("Failed to check for updates. HTTP status: %d");
					(template.replacen("%d", &code.to_string(), 1), t("Error"))
				}
				_ => {
					let msg = err.to_string();
					let fallback = t("Error checking for updates.");
					(if msg.is_empty() { fallback } else { msg }, t("Error"))
				}
			};
			if let Some(parent) = parent_window.as_ref() {
				let dialog = MessageDialog::builder(parent, &message, &title)
					.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
					.build();
				dialog.show_modal();
			}
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
