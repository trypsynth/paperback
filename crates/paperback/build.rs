//! Build script for the Paperback desktop app. Everything it actually does lives in the
//! modules under `build/`; this file only decides what runs, and in what order.

#[path = "build/docs.rs"]
mod docs;
#[path = "build/installer.rs"]
mod installer;
#[path = "build/macos.rs"]
mod macos;
#[path = "build/paths.rs"]
mod paths;
#[path = "build/pdfium.rs"]
mod pdfium;
#[path = "build/translations.rs"]
mod translations;
#[path = "build/version.rs"]
mod version;
#[path = "build/windows.rs"]
mod windows;

use std::env;

fn main() {
	paths::track_packaging_inputs();
	translations::build();
	pdfium::copy_dll();
	docs::build();
	installer::configure();
	let target = env::var("TARGET").unwrap_or_default();
	version::embed_commit_hash();
	if target.contains("apple") {
		pdfium::copy_dylib();
		macos::generate_app_bundle();
	}
	if target.contains("windows") {
		windows::embed_app_manifest();
		windows::embed_version_info();
	}
}
