use std::{
	env, fs,
	path::{Path, PathBuf},
	rc::Rc,
	sync::{
		Arc, Mutex,
		atomic::{AtomicUsize, Ordering},
	},
};

mod lang_readmes {
	include!(concat!(env!("OUT_DIR"), "/lang_readmes.rs"));
}

use paperback_core::{config::ConfigManager, parser, version};
use patois::t;
use ship_shape::{UpdateChannel as ShipChannel, UpdaterConfig};
use wx_utils::show_error;
use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager};
use crate::{config_ext::UpdateChannel, translation_manager::TranslationManager};

pub static MAIN_WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);

const PAPERBACK_GITHUB_REPO: &str = "trypsynth/paperback";
const PAPERBACK_MINISIGN_KEY: &str = "RWQasnbWXwK2dhno9ThUm8HONEIo85iiDBZvw3jlNs574QJHEkoRiGX7";

pub fn run_update_check(silent: bool, channel: UpdateChannel) {
	tracing::info!(channel = %channel, silent, "checking for updates");
	let config = Arc::new(UpdaterConfig::new(
		PAPERBACK_GITHUB_REPO,
		"paperback",
		"Paperback",
		PAPERBACK_MINISIGN_KEY,
		version::user_agent(),
	));
	let ship_channel = match channel {
		UpdateChannel::Stable => ShipChannel::Stable,
		UpdateChannel::Dev => ShipChannel::Dev,
	};
	ship_shape::ui::run_update_check(
		config,
		MAIN_WINDOW_PTR.load(Ordering::SeqCst),
		env!("CARGO_PKG_VERSION"),
		version::COMMIT_HASH,
		is_installer_distribution(),
		ship_channel,
		silent,
	);
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
	let lang = TranslationManager::instance().lock().unwrap().current_language();
	if let Some(bytes) = lang_readmes::readme_for_lang(&lang) {
		let tmp = env::temp_dir().join(format!("paperback-readme-{lang}.html"));
		match fs::write(&tmp, bytes) {
			Ok(()) => return Some(tmp),
			Err(e) => tracing::warn!(path = %tmp.display(), error = %e, "failed to write readme temp file"),
		}
	}
	// Fallback for builds without pandoc: look for readme.html next to the exe
	let exe = env::current_exe().ok()?;
	let dir = exe.parent()?;
	Some(dir.join("readme.html"))
}

pub fn handle_open_containing_folder(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dir = doc_manager.lock().unwrap().active_tab().and_then(|tab| tab.file_path.parent()).map(Path::to_path_buf);
	let Some(dir) = dir else {
		show_error(frame, t("Failed to open containing folder."), &t("Error"));
		return;
	};
	let url = format!("file://{}", dir.to_string_lossy());
	if !wxdragon::utils::launch_default_browser(&url, wxdragon::utils::BrowserLaunchFlags::Default) {
		show_error(frame, t("Failed to open containing folder."), &t("Error"));
	}
}

pub fn handle_view_help_browser(frame: &Frame) {
	let Some(path) = readme_path() else {
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return;
	};
	if !path.exists() {
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return;
	}
	let url = format!("file://{}", path.to_string_lossy());
	if !wxdragon::utils::launch_default_browser(&url, wxdragon::utils::BrowserLaunchFlags::Default) {
		tracing::warn!(url = %url, "failed to launch default browser for help");
		show_error(frame, t("Failed to launch default browser."), &t("Error"));
	}
}

pub fn handle_view_help_paperback(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
) -> bool {
	let Some(path) = readme_path() else {
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return false;
	};
	if !path.exists() {
		show_error(frame, t("readme.html not found. Please ensure the application was built properly."), &t("Error"));
		return false;
	}
	if !ensure_parser_ready_for_path(frame, &path, config) {
		return false;
	}
	doc_manager.lock().unwrap().open_help_file(doc_manager, &path)
}

pub fn handle_donate(frame: &Frame) {
	let url = "https://paypal.me/tygillespie05";
	if !wxdragon::utils::launch_default_browser(url, wxdragon::utils::BrowserLaunchFlags::Default) {
		tracing::warn!("failed to launch default browser for donation page");
		show_error(frame, t("Failed to open donation page in browser."), &t("Error"));
	}
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
