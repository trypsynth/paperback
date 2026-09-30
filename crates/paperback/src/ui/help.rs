#[cfg(any(target_os = "macos", test))]
use std::path::Path;
#[cfg(any(target_os = "macos", target_os = "windows", test))]
use std::process::Command;
use std::{env, fs, path::PathBuf, rc::Rc, sync::Mutex};

mod lang_readmes {
	include!(concat!(env!("OUT_DIR"), "/lang_readmes.rs"));
}

use paperback_core::config::ConfigManager;
use patois::t;
use wx_utils::show_error;
use wxdragon::prelude::*;

use super::document_manager::DocumentManager;
use crate::translation_manager::TranslationManager;

/// Materializes the readme for the current UI language and returns its path. Every readme is
/// embedded at build time, so this reads nothing from the install directory; languages without a
/// translated readme fall back to English rather than to no help at all.
pub fn readme_path() -> Option<PathBuf> {
	let lang = TranslationManager::instance().lock().unwrap().current_language();
	let (lang, bytes) = match lang_readmes::readme_for_lang(&lang) {
		Some(bytes) => (lang, bytes),
		None => ("en".to_string(), lang_readmes::readme_for_lang("en")?),
	};
	let tmp = env::temp_dir().join(format!("paperback-readme-{lang}.html"));
	match fs::write(&tmp, bytes) {
		Ok(()) => Some(tmp),
		Err(e) => {
			tracing::warn!(path = %tmp.display(), error = %e, "failed to write readme temp file");
			None
		}
	}
}

pub fn handle_reveal_file_in_folder(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let file_path = doc_manager.lock().unwrap().active_tab().map(|tab| tab.file_path.clone());
	let Some(file_path) = file_path else {
		// TRANSLATORS: Error shown when the active document has no known file path to reveal
		show_error(frame, t("Failed to reveal file in folder."), &t("Error"));
		return;
	};
	#[cfg(target_os = "windows")]
	{
		use std::os::windows::process::CommandExt;
		let path_to_reveal =
			dunce::canonicalize(&file_path).unwrap_or_else(|_| dunce::simplified(&file_path).to_path_buf());
		let path_str = path_to_reveal.to_string_lossy();
		if Command::new("explorer").raw_arg(format!("/select,\"{path_str}\"")).spawn().is_err() {
			// TRANSLATORS: Error shown when the file manager could not be launched to reveal the active document's file
			show_error(frame, t("Failed to reveal file in folder."), &t("Error"));
		}
	}
	#[cfg(target_os = "macos")]
	if finder_reveal_command(&file_path).spawn().is_err() {
		// TRANSLATORS: Error shown when the file manager could not be launched to reveal the active document's file
		show_error(frame, t("Failed to reveal file in folder."), &t("Error"));
	}
	#[cfg(not(any(target_os = "macos", target_os = "windows")))]
	{
		if let Some(dir) = file_path.parent() {
			let url = format!("file://{}", dir.to_string_lossy());
			if !wxdragon::utils::launch_default_browser(&url, wxdragon::utils::BrowserLaunchFlags::Default) {
				// TRANSLATORS: Error shown when the file manager could not be launched to reveal the active document's file
				show_error(frame, t("Failed to reveal file in folder."), &t("Error"));
			}
		}
	}
}

#[cfg(any(target_os = "macos", test))]
fn finder_reveal_command(file_path: &Path) -> Command {
	let mut command = Command::new("/usr/bin/open");
	command.arg("-R").arg(file_path);
	command
}

pub fn handle_view_help_browser(frame: &Frame) {
	let Some(path) = readme_path() else {
		// TRANSLATORS: Error shown when the bundled help/readme file could not be located on disk
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return;
	};
	if !path.exists() {
		// TRANSLATORS: Error shown when the bundled help/readme file could not be located on disk
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return;
	}
	let url = format!("file://{}", path.to_string_lossy());
	if !launch_default_browser(&url, BrowserLaunchFlags::Default) {
		tracing::warn!(url = %url, "failed to launch default browser for help");
		// TRANSLATORS: Error shown when the OS default web browser could not be launched to display help
		show_error(frame, t("Failed to launch default browser."), &t("Error"));
	}
}

pub fn handle_view_help_paperback(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) -> bool {
	let Some(path) = readme_path() else {
		// TRANSLATORS: Error shown when the bundled help/readme file could not be located on disk
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return false;
	};
	if !path.exists() {
		// TRANSLATORS: Error shown when the bundled help/readme file could not be located on disk
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return false;
	}
	if !super::main_window::ensure_parser_ready_for_path(frame, &path, config) {
		return false;
	}
	doc_manager.lock().unwrap().open_help_file(doc_manager, &path)
}

pub fn handle_donate(frame: &Frame) {
	let url = "https://paypal.me/tygillespie05";
	if !launch_default_browser(url, BrowserLaunchFlags::Default) {
		tracing::warn!("failed to launch default browser for donation page");
		// TRANSLATORS: Error shown when the OS default web browser could not be launched to open the donation page
		show_error(frame, t("Failed to open donation page in browser."), &t("Error"));
	}
}

#[cfg(test)]
mod tests {
	use std::{ffi::OsStr, path::Path};

	use super::finder_reveal_command;

	#[test]
	fn finder_reveal_command_preserves_paths_with_spaces() {
		let file_path = Path::new("/Users/reader/Fics in progress/story.epub");
		let command = finder_reveal_command(file_path);
		assert_eq!(command.get_program(), OsStr::new("/usr/bin/open"));
		assert_eq!(command.get_args().collect::<Vec<_>>(), [OsStr::new("-R"), file_path.as_os_str()]);
	}
}
