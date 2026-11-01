use std::{
	env,
	error::Error,
	fs,
	path::{Path, PathBuf},
	process::Command,
};

use crate::workspace::{MOBILE_CORE_FEATURES, build_host_ffi_library, project_root};

pub fn ios() -> Result<(), Box<dyn Error>> {
	let release = env::args().any(|a| a == "--release");
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let root = project_root();
	let generated_dir = root.join("ios/Paperback/Generated");
	fs::create_dir_all(&generated_dir)?;
	println!("Generating Swift bindings via uniffi-bindgen...");
	let ffi_library = build_host_ffi_library(&cargo)?.to_string_lossy().into_owned();
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
			"--library",
			&ffi_library,
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
	let mut build_args =
		vec!["rustc", "-p", "paperback-core", "--features", MOBILE_CORE_FEATURES, "--lib", "--crate-type", "staticlib"];
	if release {
		build_args.push("--release");
	}
	// Without the simulator slice xcodebuild fails with "no library for this platform was found".
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
	let po_dir = root.join("po");
	let ios_dir = root.join("ios/Paperback");
	if po_dir.is_dir() {
		if let Err(e) = patois_build::gen_ios_strings(&po_dir, &ios_dir) {
			println!("Warning: could not generate Localizable.strings: {e}");
		}
		// iOS only matches a device's language to BCP-47 folder names, not po stems; keep in sync with knownRegions in project.pbxproj and the Localizable.strings variant group.
		for (from, to) in [("pt_br", "pt-BR"), ("zh_CN", "zh-Hans")] {
			let from_dir = ios_dir.join(format!("{from}.lproj"));
			let to_dir = ios_dir.join(format!("{to}.lproj"));
			if from_dir.is_dir() {
				let _ = fs::remove_dir_all(&to_dir);
				fs::rename(&from_dir, &to_dir)?;
			}
		}
	}
	crate::docs::write_app_readmes(&ios_dir.join("Readmes"))?;
	println!("iOS build complete.");
	println!("  XCFramework: ios/paperbackFFI.xcframework");
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
		upload_ipa(&ipa)?;
	} else {
		println!("To upload, run:  cargo xtask ios-release --upload");
		println!("Or drag {} into Transporter or Xcode Organizer.", ipa.display());
	}
	Ok(())
}

/// Uses the macOS notarization step's API key and variables, so there's one set of credentials to keep.
/// A missing key is reported here because altool's own message for it is easy to miss.
fn upload_ipa(ipa: &Path) -> Result<(), Box<dyn Error>> {
	let key_id = env::var("APPSTORE_API_KEY_ID").unwrap_or_default().trim().to_string();
	let issuer_id = env::var("APPSTORE_API_ISSUER_ID").unwrap_or_default().trim().to_string();
	if key_id.is_empty() || issuer_id.is_empty() {
		return Err("APPSTORE_API_KEY_ID and APPSTORE_API_ISSUER_ID must both be set to upload. 			They are the same values the macOS notarization step uses."
			.into());
	}
	let key_name = format!("AuthKey_{key_id}.p8");
	let searched = [
		PathBuf::from("private_keys"),
		home_dir().join("private_keys"),
		home_dir().join(".private_keys"),
		home_dir().join(".appstoreconnect/private_keys"),
	];
	if !searched.iter().any(|dir| dir.join(&key_name).exists()) {
		let places = searched.iter().map(|d| d.display().to_string()).collect::<Vec<_>>().join(", ");
		return Err(format!("{key_name} not found. altool looks for it in: {places}").into());
	}
	println!("Uploading to App Store Connect...");
	let status = Command::new("xcrun")
		.args([
			"altool",
			"--upload-app",
			"--type",
			"ios",
			"--file",
			&ipa.to_string_lossy(),
			"--apiKey",
			&key_id,
			"--apiIssuer",
			&issuer_id,
		])
		.status()?;
	if !status.success() {
		return Err(format!(
			"altool upload failed. The IPA is still at {}, so it can go through Transporter instead.",
			ipa.display()
		)
		.into());
	}
	println!("Upload complete. The build appears in TestFlight once Apple finishes processing it.");
	Ok(())
}

fn home_dir() -> PathBuf {
	PathBuf::from(env::var("HOME").unwrap_or_default())
}
