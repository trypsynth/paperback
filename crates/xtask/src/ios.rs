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

const PDFIUM_IOS_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-ios-device-arm64.tgz";

pub fn ios() -> Result<(), Box<dyn Error>> {
	let release = env::args().any(|a| a == "--release");
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let root = crate::project_root();
	let generated_dir = root.join("ios/Paperback/Generated");
	fs::create_dir_all(&generated_dir)?;

	let pdfium_dest = root.join("ios/libpdfium.dylib");
	download_pdfium_dylib(PDFIUM_IOS_ARM64_URL, &pdfium_dest)?;

	println!("Generating Swift bindings via uniffi-bindgen...");
	let status = Command::new(&cargo)
		.current_dir(&root)
		.args([
			"run",
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
	let mut build_args = vec!["build", "-p", "paperback-core"];
	if release {
		build_args.push("--release");
	}

	println!("Building for aarch64-apple-ios (device)...");
	let status =
		Command::new(&cargo).current_dir(&root).args(&build_args).args(["--target", "aarch64-apple-ios"]).status()?;
	if !status.success() {
		return Err("cargo build for aarch64-apple-ios failed".into());
	}

	let headers_dir = root.join("ios/Paperback/Generated");
	let device_lib = root.join(format!("target/aarch64-apple-ios/{profile}/libpaperback_core.a"));
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
	}

	println!("iOS build complete.");
	println!("  XCFramework: ios/paperbackFFI.xcframework");
	println!("  Swift bindings: ios/Paperback/Generated/paperback.swift");
	println!("  Localizable.strings: ios/Paperback/<lang>.lproj/Localizable.strings");
	println!("  Add both XCFramework and Swift bindings to the Xcode project to use the Rust core.");
	Ok(())
}

pub fn ios_release() -> Result<(), Box<dyn Error>> {
	let upload = env::args().any(|a| a == "--upload");
	let root = crate::project_root();
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
	let skip =
		env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false);
	if dest.exists() && !skip {
		return Ok(());
	}
	if skip {
		return Ok(());
	}
	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}
	println!("Downloading {} ...", url);
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
