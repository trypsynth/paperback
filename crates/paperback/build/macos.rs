//! The macOS side of packaging: the linker search path for Homebrew's libiconv and the
//! `Paperback.app` bundle laid out around the freshly built binary.

use std::{collections::BTreeSet, env, fs, path::Path};

use crate::paths::{self, target_profile_dir};

/// Homebrew's libiconv is keg-only and not on the default search path. wxWidgets links against
/// it, so we need to tell the linker where to find it.
pub fn link_libiconv(target: &str) {
	let homebrew_prefix = if target.contains("aarch64") { "/opt/homebrew" } else { "/usr/local" };
	let iconv_lib = format!("{homebrew_prefix}/opt/libiconv/lib");
	if Path::new(&iconv_lib).exists() {
		println!("cargo:rustc-link-search=native={iconv_lib}");
	}
}

/// Builds the `<string>ext</string>` lines for `CFBundleTypeExtensions`, deduped since a couple
/// of extensions (e.g. `zip`) are shared by more than one entry in `paperback_formats::ALL`.
fn bundle_document_extensions_block() -> String {
	let mut extensions = BTreeSet::new();
	for format in paperback_formats::ALL {
		extensions.extend(format.extensions.iter().copied());
	}
	extensions.into_iter().map(|ext| format!("\t\t\t\t<string>{ext}</string>")).collect::<Vec<_>>().join("\n")
}

pub fn generate_app_bundle() {
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target directory for macOS app bundle.");
		return;
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let document_extensions = bundle_document_extensions_block();
	let bundle_dir = target_dir.join("Paperback.app/Contents");
	let macos_dir = bundle_dir.join("MacOS");
	let _ = fs::create_dir_all(&macos_dir);
	let _ = fs::create_dir_all(bundle_dir.join("Resources"));
	let plist = format!(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key>
	<string>Paperback</string>
	<key>CFBundleDisplayName</key>
	<string>Paperback</string>
	<key>CFBundleIdentifier</key>
	<string>com.trypsynth.paperback</string>
	<key>CFBundleVersion</key>
	<string>{version}</string>
	<key>CFBundleShortVersionString</key>
	<string>{version}</string>
	<key>CFBundleExecutable</key>
	<string>paperback</string>
	<key>CFBundleIconFile</key>
	<string>paperback</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>CFBundleDocumentTypes</key>
	<array>
		<dict>
			<key>CFBundleTypeRole</key>
			<string>Viewer</string>
			<key>CFBundleTypeExtensions</key>
			<array>
{document_extensions}
			</array>
			<key>CFBundleTypeName</key>
			<string>Document</string>
		</dict>
	</array>
</dict>
</plist>"#
	);
	let plist_path = bundle_dir.join("Info.plist");
	if let Err(e) = fs::write(&plist_path, plist) {
		println!("cargo:warning=Failed to write Info.plist: {e}");
		return;
	}
	// Copy the binary into the bundle if it exists (from a previous build)
	let exe_path = target_dir.join("paperback");
	let bundle_exe = macos_dir.join("paperback");
	if exe_path.exists() {
		let _ = fs::copy(&exe_path, &bundle_exe);
	}
	let dylib_path = target_dir.join("libpdfium.dylib");
	if dylib_path.exists() {
		let _ = fs::copy(&dylib_path, macos_dir.join("libpdfium.dylib"));
	}
	// Named to match CFBundleIconFile above; without it the Dock, Finder and the app switcher
	// all fall back to the blank generic-application icon.
	let icns = paths::manifest_dir().join("assets/paperback.icns");
	if let Err(e) = fs::copy(&icns, bundle_dir.join("Resources/paperback.icns")) {
		println!("cargo:warning=Failed to copy the app bundle icon: {e}");
	}
}
