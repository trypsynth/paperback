#![cfg_attr(not(test), windows_subsystem = "windows")]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

patois::embed_domain!();

mod accessibility;
mod config_ext;
mod ipc;
mod legacy_config;
mod logging;
mod translation_manager;
mod ui;

use std::{env, fs, io};

use paperback_core::{set_pdfium_library_path, version};
use ui::PaperbackApp;
use wxdragon::prelude::{Appearance, set_appearance};

fn main() {
	let _log_guard = logging::init(&config_ext::config_dir());
	tracing::info!(version = env!("CARGO_PKG_VERSION"), commit = version::COMMIT_HASH, "starting");
	set_pdfium_path_from_exe();
	cleanup_legacy_files();
	let _ = wxdragon::main(|app| {
		let _ = set_appearance(Appearance::System);
		let app_state = PaperbackApp::new(app);
		let _ = Box::leak(Box::new(app_state));
	});
}

fn set_pdfium_path_from_exe() {
	if let Ok(exe) = env::current_exe()
		&& let Some(dir) = exe.parent()
	{
		set_pdfium_library_path(dir.to_string_lossy().into_owned());
	}
}

fn cleanup_legacy_files() {
	let Ok(exe) = env::current_exe() else { return };
	let Some(dir) = exe.parent() else { return };
	for name in ["nvdaControllerClient64.dll", "SAAPI64.dll"] {
		let path = dir.join(name);
		if let Err(e) = fs::remove_file(&path) {
			if e.kind() != io::ErrorKind::NotFound {
				tracing::warn!(path = %path.display(), error = %e, "failed to remove legacy file");
			}
		}
	}
	if let Err(e) = fs::remove_dir_all(dir.join("langs")) {
		if e.kind() != io::ErrorKind::NotFound {
			tracing::warn!(error = %e, "failed to remove legacy langs directory");
		}
	}
}
