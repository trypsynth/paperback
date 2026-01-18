use std::env;

use embed_manifest::{
	embed_manifest,
	manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*},
	new_manifest,
};

fn main() {
	let target = env::var("TARGET").unwrap_or_default();
	if target.contains("windows") {
		let manifest = new_manifest("Fedra")
			.supported_os(Windows7..=Windows10)
			.active_code_page(ActiveCodePage::Utf8)
			.heap_type(HeapType::SegmentHeap)
			.dpi_awareness(DpiAwareness::PerMonitorV2)
			.long_path_aware(Setting::Enabled);
		if let Err(e) = embed_manifest(manifest) {
			println!("cargo:warning=Failed to embed manifest: {}", e);
			println!("cargo:warning=The application will still work but may lack optimal Windows theming");
		}
		println!("cargo:rerun-if-changed=build.rs");
	}
}
