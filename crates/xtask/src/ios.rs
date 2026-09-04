use std::{
	env,
	error::Error,
	fs,
	io::{Cursor, Read},
	path::Path,
	process::Command,
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::workspace::project_root;

// Pinned instead of `releases/latest`: whatever `latest` currently resolves to
// crashes on real iOS devices (not the Simulator) with
// `[FATAL:partition_address_space.cc(81)] Check failed: false.` inside
// libpdfium.dylib the moment PDFium is initialized, even though the release
// sets `pdf_use_partition_alloc = false` in its build args. chromium/7920 is
// confirmed (by direct on-device test) not to hit this. Re-verify on a real
// device before bumping this pin.
const PDFIUM_IOS_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F7920/pdfium-ios-device-arm64.tgz";

pub fn ios() -> Result<(), Box<dyn Error>> {
	let release = env::args().any(|a| a == "--release");
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let root = project_root();
	let generated_dir = root.join("ios/Paperback/Generated");
	fs::create_dir_all(&generated_dir)?;
	let pdfium_dest = root.join("ios/libpdfium.dylib");
	download_pdfium_dylib(PDFIUM_IOS_ARM64_URL, &pdfium_dest)?;
	wrap_pdfium_framework(&root, &pdfium_dest)?;
	println!("Generating Swift bindings via uniffi-bindgen...");
	let status = Command::new(&cargo)
		.current_dir(&root)
		.args([
			"run",
			"-p",
			"paperback-core",
			"--features",
			"uniffi",
			"--bin",
			"uniffi-bindgen",
			"--",
			"generate",
			"crates/paperback-core/src/paperback.udl",
			"--language",
			"swift",
			"--out-dir",
			"ios/Paperback/Generated",
			"--no-format",
		])
		.status()?;
	if !status.success() {
		return Err("uniffi-bindgen Swift generation failed".into());
	}
	let profile = if release { "release" } else { "debug" };
	let mut build_args = vec!["build", "-p", "paperback-core", "--features", "uniffi"];
	if release {
		build_args.push("--release");
	}
	// Both slices: without the simulator one the app cannot be built for, let alone run in
	// the Simulator. xcodebuild fails with "no library for this platform was found".
	for (target, label) in [("aarch64-apple-ios", "device"), ("aarch64-apple-ios-sim", "simulator")] {
		println!("Building for {target} ({label})...");
		let status = Command::new(&cargo).current_dir(&root).args(&build_args).args(["--target", target]).status()?;
		if !status.success() {
			return Err(format!("cargo build for {target} failed").into());
		}
	}
	let headers_dir = root.join("ios/Paperback/Generated");
	let device_lib = root.join(format!("target/aarch64-apple-ios/{profile}/libpaperback_core.a"));
	let simulator_lib = root.join(format!("target/aarch64-apple-ios-sim/{profile}/libpaperback_core.a"));
	let xcframework_out = root.join("ios/paperbackFFI.xcframework");
	if xcframework_out.exists() {
		fs::remove_dir_all(&xcframework_out)?;
	}
	println!("Creating paperbackFFI.xcframework...");
	let status = Command::new("xcodebuild")
		.args(["-create-xcframework"])
		.arg("-library")
		.arg(&device_lib)
		.arg("-headers")
		.arg(&headers_dir)
		.arg("-library")
		.arg(&simulator_lib)
		.arg("-headers")
		.arg(&headers_dir)
		.arg("-output")
		.arg(&xcframework_out)
		.status()?;
	if !status.success() {
		return Err("xcodebuild -create-xcframework failed".into());
	}
	// Generate Localizable.strings for each translated language
	let po_dir = root.join("po");
	let ios_dir = root.join("ios/Paperback");
	if po_dir.is_dir() {
		if let Err(e) = patois_build::gen_ios_strings(&po_dir, &ios_dir) {
			println!("Warning: could not generate Localizable.strings: {e}");
		}
		// gen_ios_strings names .lproj folders after the po file stem (Android-style,
		// e.g. pt_br, zh_CN). iOS expects canonical BCP-47 identifiers to match a
		// device's language automatically, so rename the two that differ. Keep this
		// in sync with the language list checked into project.pbxproj's knownRegions
		// and the Localizable.strings variant group.
		for (from, to) in [("pt_br", "pt-BR"), ("zh_CN", "zh-Hans")] {
			let from_dir = ios_dir.join(format!("{from}.lproj"));
			let to_dir = ios_dir.join(format!("{to}.lproj"));
			if from_dir.is_dir() {
				let _ = fs::remove_dir_all(&to_dir);
				fs::rename(&from_dir, &to_dir)?;
			}
		}
	}
	generate_readmes(&root, &ios_dir.join("Readmes"))?;
	println!("iOS build complete.");
	println!("  XCFramework: ios/paperbackFFI.xcframework");
	println!("  PDFium framework: ios/libpdfium.framework");
	println!("  Swift bindings: ios/Paperback/Generated/paperback.swift");
	println!("  Localizable.strings: ios/Paperback/<lang>.lproj/Localizable.strings");
	println!("  Add both XCFramework and Swift bindings to the Xcode project to use the Rust core.");
	Ok(())
}

pub fn ios_release() -> Result<(), Box<dyn Error>> {
	let upload = env::args().any(|a| a == "--upload");
	let root = project_root();
	let ios_dir = root.join("ios");
	let archive_path = root.join("target/Paperback.xcarchive");
	let export_path = root.join("target/PaperbackExport");
	let export_options = ios_dir.join("ExportOptions.plist");
	if !export_options.exists() {
		return Err("ios/ExportOptions.plist not found".into());
	}
	println!("Archiving Paperback.xcodeproj...");
	let status = Command::new("xcodebuild")
		.args([
			"archive",
			"-project",
			&ios_dir.join("Paperback.xcodeproj").to_string_lossy(),
			"-scheme",
			"Paperback",
			"-destination",
			"generic/platform=iOS",
			"-archivePath",
			&archive_path.to_string_lossy(),
		])
		.status()?;
	if !status.success() {
		return Err("xcodebuild archive failed".into());
	}
	println!("Exporting IPA...");
	let _ = fs::remove_dir_all(&export_path);
	let status = Command::new("xcodebuild")
		.args([
			"-exportArchive",
			"-archivePath",
			&archive_path.to_string_lossy(),
			"-exportPath",
			&export_path.to_string_lossy(),
			"-exportOptionsPlist",
			&export_options.to_string_lossy(),
		])
		.status()?;
	if !status.success() {
		return Err("xcodebuild -exportArchive failed".into());
	}
	let ipa = export_path.join("Paperback.ipa");
	if !ipa.exists() {
		return Err(format!("IPA not found at {}", ipa.display()).into());
	}
	println!("IPA ready: {}", ipa.display());
	if upload {
		println!("Uploading to App Store Connect...");
		let status = Command::new("xcrun")
			.args([
				"altool",
				"--upload-app",
				"--type",
				"ios",
				"--file",
				&ipa.to_string_lossy(),
				"--authentication-key-path",
				"",
			])
			.status();
		match status {
			Ok(s) if s.success() => println!("Upload complete."),
			_ => println!(
				"altool upload failed or not configured. Upload {} manually via Transporter or Xcode Organizer.",
				ipa.display()
			),
		}
	} else {
		println!("To upload, run:  cargo xtask ios-release --upload");
		println!("Or drag {} into Transporter or Xcode Organizer.", ipa.display());
	}
	Ok(())
}

fn download_pdfium_dylib(url: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
	let skip = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
	if dest.exists() && !skip {
		return Ok(());
	}
	if skip {
		return Ok(());
	}
	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}
	println!("Downloading {url} ...");
	let response = ureq::get(url).call().map_err(|e| format!("download failed: {e}"))?;
	let mut archive_bytes = Vec::new();
	response.into_body().as_reader().read_to_end(&mut archive_bytes)?;
	let mut archive = Archive::new(GzDecoder::new(Cursor::new(archive_bytes)));
	for entry in archive.entries()? {
		let mut entry = entry?;
		if entry.path()?.file_name().and_then(|n| n.to_str()) == Some("libpdfium.dylib") {
			let tmp = dest.with_extension("dylib.tmp");
			entry.unpack(&tmp)?;
			if dest.exists() {
				fs::remove_file(dest)?;
			}
			fs::rename(&tmp, dest)?;
			println!("Saved {}", dest.display());
			return Ok(());
		}
	}
	Err(format!("libpdfium.dylib not found in archive from {url}").into())
}

// Wraps libpdfium.dylib in a proper .framework bundle. iOS doesn't support standalone
// third-party dylibs at all, only .framework bundles, regardless of whether the dylib is
// wrapped in an XCFramework container; App Store Connect's upload validator rejects a
// bare-dylib XCFramework with ITMS-90426 ("SwiftSupport folder is missing"), because it still
// routes through the legacy validation path that a raw dylib triggers. A real .framework
// bundle takes the modern, SwiftSupport-free path instead.
//
// This is embedded directly (not further wrapped in an XCFramework like paperbackFFI): we
// only ever ship one platform slice, so XCFramework's multi-platform packaging buys nothing
// here, and `xcodebuild -create-xcframework -framework` insists the binary inside be named
// after the bundle itself (e.g. `libpdfium.framework/libpdfium`, no `.dylib`), which would
// conflict with the fixed filename below.
//
// The framework's binary keeps the "libpdfium.dylib" filename (not the usual extension-less
// framework-binary convention) because the `pdfium` crate's dlopen call constructs exactly
// that filename via `libloading::library_filename("pdfium")`. AppViewModel.swift points
// setPdfiumLibraryPath at .../Frameworks/libpdfium.framework to match.
fn wrap_pdfium_framework(root: &Path, dylib: &Path) -> Result<(), Box<dyn Error>> {
	let framework_dir = root.join("ios/libpdfium.framework");
	if framework_dir.exists() {
		fs::remove_dir_all(&framework_dir)?;
	}
	fs::create_dir_all(&framework_dir)?;
	let framework_binary = framework_dir.join("libpdfium.dylib");
	fs::copy(dylib, &framework_binary)?;
	let status = Command::new("install_name_tool")
		.args(["-id", "@rpath/libpdfium.framework/libpdfium.dylib"])
		.arg(&framework_binary)
		.status()?;
	if !status.success() {
		return Err("install_name_tool -id failed on libpdfium.framework/libpdfium.dylib".into());
	}
	// The prebuilt binary's LC_BUILD_VERSION load command declares a minos far above our own
	// deployment target (observed: iOS 26.0, just whatever SDK the upstream release happened to
	// be built with). Nothing in PDFium actually requires that OS version, but App Store
	// Connect's validator takes the declaration at face value and rejects the bundle as
	// unsupported on our (lower) deployment target. Rewrite it to match.
	let status = Command::new("xcrun")
		.args(["vtool", "-set-build-version", "ios", IOS_DEPLOYMENT_TARGET, IOS_DEPLOYMENT_TARGET])
		.arg("-replace")
		.arg("-output")
		.arg(&framework_binary)
		.arg(&framework_binary)
		.status()?;
	if !status.success() {
		return Err("vtool -set-build-version failed on libpdfium.framework/libpdfium.dylib".into());
	}
	fs::write(framework_dir.join("Info.plist"), pdfium_framework_info_plist())?;
	Ok(())
}

// Keep in sync with IPHONEOS_DEPLOYMENT_TARGET in Paperback.xcodeproj/project.pbxproj.
const IOS_DEPLOYMENT_TARGET: &str = "16.0";

fn pdfium_framework_info_plist() -> String {
	format!(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>libpdfium.dylib</string>
	<key>CFBundleIdentifier</key>
	<string>org.pdfium.libpdfium</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>libpdfium</string>
	<key>CFBundlePackageType</key>
	<string>FMWK</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>iPhoneOS</string>
	</array>
	<key>MinimumOSVersion</key>
	<string>{IOS_DEPLOYMENT_TARGET}</string>
</dict>
</plist>
"#
	)
}

fn generate_readmes(root: &Path, readmes_dir: &Path) -> Result<(), Box<dyn Error>> {
	let doc_dir = root.join("doc");
	if !doc_dir.is_dir() {
		return Ok(());
	}
	fs::create_dir_all(readmes_dir)?;
	let pandoc_config = doc_dir.join("pandoc.yaml");
	let default_readme = doc_dir.join("readme.md");
	if default_readme.exists() {
		let status = Command::new("pandoc")
			.arg(format!("--defaults={}", pandoc_config.display()))
			.arg(&default_readme)
			.arg("-o")
			.arg(readmes_dir.join("readme.html"))
			.status();
		match status {
			Ok(s) if s.success() => {}
			_ => println!("Warning: Failed to generate default English documentation"),
		}
	}
	for entry in fs::read_dir(&doc_dir)?.flatten() {
		let path = entry.path();
		let Some(name) = path.file_name().and_then(|n| n.to_str()) else { continue };
		if !name.starts_with("readme-") || !name.ends_with(".md") {
			continue;
		}
		let out_name = name.replace(".md", ".html");
		let status = Command::new("pandoc")
			.arg(format!("--defaults={}", pandoc_config.display()))
			.arg(&path)
			.arg("-o")
			.arg(readmes_dir.join(out_name))
			.status();
		match status {
			Ok(s) if s.success() => {}
			_ => println!("Warning: Failed to generate documentation for language: {name}"),
		}
	}
	Ok(())
}
