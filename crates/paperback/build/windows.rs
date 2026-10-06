//! Windows-only resources: the application manifest, the icon and the version block Explorer
//! shows in the file properties dialog.

use std::env;

use shipfitter::{build::commit_info, windows};

/// Embeds the application manifest that asks Windows for UTF-8, the segment heap, per-monitor
/// DPI awareness and long path support.
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
		// Explorer, the taskbar, Alt+Tab, the Start menu, the uninstall entry and the document
		// types the installer registers (`paperback.exe,0`) all read the icon straight out of the
		// executable, so without this the app and every file associated with it show the generic
		// "no icon" placeholder. `ui::icon` handles the places that need a bitmap at runtime.
		icon: Some("assets/paperback.ico"),
	};
	if let Err(e) = info.embed() {
		println!("cargo:warning=Failed to embed version info: {e}");
	}
}
