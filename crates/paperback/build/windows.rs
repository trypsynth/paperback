//! Windows-only resources: the application manifest, the icon and the version block Explorer
//! shows in the file properties dialog.

use std::env;

use embed_manifest::{
	embed_manifest,
	manifest::{
		ActiveCodePage, DpiAwareness, HeapType, Setting,
		SupportedOS::{Windows7, Windows10},
	},
	new_manifest,
};
use winres::WindowsResource;

use crate::version::get_commit_info;

/// Embeds the application manifest that asks Windows for UTF-8, the segment heap, per-monitor
/// DPI awareness and long path support.
pub fn embed_app_manifest() {
	let manifest = new_manifest("Paperback")
		.supported_os(Windows7..=Windows10)
		.active_code_page(ActiveCodePage::Utf8)
		.heap_type(HeapType::SegmentHeap)
		.dpi_awareness(DpiAwareness::PerMonitorV2)
		.long_path_aware(Setting::Enabled);
	if let Err(e) = embed_manifest(manifest) {
		println!("cargo:warning=Failed to embed manifest: {e}");
		println!("cargo:warning=The application will still work but may lack optimal Windows theming");
	}
}

pub fn embed_version_info() {
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let (hash, is_dev) = get_commit_info();
	let product_version = if is_dev {
		let short_hash = &hash[..hash.len().min(7)];
		format!("{version} ({short_hash})")
	} else {
		version.clone()
	};
	let mut res = WindowsResource::new();
	// Explorer, the taskbar, Alt+Tab, the Start menu, the uninstall entry and the document
	// types the installer registers (`paperback.exe,0`) all read the icon straight out of the
	// executable, so without this the app and every file associated with it show the generic
	// "no icon" placeholder. `ui::icon` handles the places that need a bitmap at runtime.
	res.set_icon("assets/paperback.ico");
	res.set("ProductName", "Paperback")
		.set("FileDescription", "Paperback")
		.set("LegalCopyright", "Copyright © 2025 Quin Gillespie")
		.set("CompanyName", "Quin Gillespie")
		.set("OriginalFilename", "paperback.exe")
		.set("ProductVersion", &product_version)
		.set("FileVersion", &version);
	if let Err(e) = res.compile() {
		println!("cargo:warning=Failed to embed version info: {e}");
	}
}
