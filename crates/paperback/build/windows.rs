use std::env;

use shipfitter::{build::commit_info, windows};

pub fn embed_app_manifest() {
	if let Err(e) = windows::embed_manifest("Paperback") {
		println!("cargo:warning=Failed to embed manifest: {e}");
		println!("cargo:warning=The application will still work but may lack optimal Windows theming");
	}
}

pub fn embed_version_info() {
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let commit = commit_info();
	let product_version = if commit.is_dev { format!("{version} ({})", commit.short_hash) } else { version };
	let info = windows::VersionInfo {
		product_name: "Paperback",
		company: "Quin Gillespie",
		copyright: "Copyright © 2025 Quin Gillespie",
		original_filename: "paperback.exe",
		product_version: Some(&product_version),
		// Explorer, the taskbar, the Start menu and the installer's file associations (`paperback.exe,0`) all read the icon out of the executable.
		icon: Some("assets/paperback.ico"),
	};
	if let Err(e) = info.embed() {
		println!("cargo:warning=Failed to embed version info: {e}");
	}
}
