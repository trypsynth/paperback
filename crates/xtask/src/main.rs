use std::{
	env,
	error::Error,
	fs,
	io::{Cursor, Read},
	path::{Path, PathBuf},
	process::Command,
};
#[cfg(not(target_os = "macos"))]
use std::{fs::File, io};

use flate2::read::GzDecoder;
use tar::Archive;
use walkdir::WalkDir;
#[cfg(not(target_os = "macos"))]
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

const PDFIUM_ANDROID_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-android-arm64.tgz";
const PDFIUM_ANDROID_ARM_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-android-arm.tgz";
const PDFIUM_IOS_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-ios-device-arm64.tgz";

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release()?,
		Some("android") => android()?,
		Some("ios") => ios()?,
		Some("gen-pot") => gen_pot()?,
		_ => print_help(),
	}
	Ok(())
}

fn print_help() {
	println!("Tasks:");
	println!("	release	Build release binaries and package them");
	println!("	gen-pot	Regenerate po/paperback.pot from all translatable crates");
	println!("	android	Generate Kotlin bindings and build native Android libraries");
	println!("		--release          Build APK using gradlew assembleRelease");
	println!("		--debug            Build APK using gradlew assembleDebug");
	println!("		--install-release  Install release APK using gradlew installRelease");
	println!("		--install-debug    Install debug APK using gradlew installDebug");
	println!("	ios	Generate Swift bindings and build XCFramework for iOS");
	println!("		--release          Build in release mode (default is debug)");
}

fn release() -> Result<(), Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let status =
		Command::new(cargo).current_dir(project_root()).args(&["build", "--release", "-p", "paperback"]).status()?;
	if !status.success() {
		return Err("Cargo build failed".into());
	}
	let target_dir = project_root().join("target/release");
	#[cfg(target_os = "macos")]
	return build_mac_dmg(&target_dir);
	#[cfg(not(target_os = "macos"))]
	{
		let exe_name = if cfg!(windows) { "paperback.exe" } else { "paperback" };
		let exe_path = target_dir.join(exe_name);
		let readme_path = target_dir.join("readme.html");
		let sounds_path = target_dir.join("sounds");
		let pdfium_dll_path = target_dir.join("pdfium.dll");
		if !exe_path.exists() {
			return Err("Executable not found".into());
		}
		println!("Packaging binary, docs, and sounds...");
		build_zip_package(&target_dir, &exe_path, &readme_path, &sounds_path, &pdfium_dll_path)?;
		if cfg!(windows) {
			build_windows_installer(&target_dir)?;
		}
		Ok(())
	}
}

fn android() -> Result<(), Box<dyn Error>> {
	let args = env::args().skip(2);
	let mut gradle_tasks = Vec::new();

	for arg in args {
		match arg.as_str() {
			"--release" => gradle_tasks.push("assembleRelease"),
			"--debug" => gradle_tasks.push("assembleDebug"),
			"--installrelease" | "--install-release" => gradle_tasks.push("installRelease"),
			"--installdebug" | "--install-debug" => gradle_tasks.push("installDebug"),
			_ => {
				print_help();
				return Err(format!("Unknown argument for android: {}", arg).into());
			}
		}
	}

	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let jni_libs = project_root().join("android/app/src/main/jniLibs");
	download_pdfium_so(PDFIUM_ANDROID_ARM64_URL, &jni_libs.join("arm64-v8a/libpdfium.so"))?;
	download_pdfium_so(PDFIUM_ANDROID_ARM_URL, &jni_libs.join("armeabi-v7a/libpdfium.so"))?;

	println!("Generating Kotlin bindings via uniffi-bindgen...");
	let status = Command::new(&cargo)
		.current_dir(project_root())
		.args(&[
			"run",
			"--bin",
			"uniffi-bindgen",
			"--",
			"generate",
			"crates/paperback-core/src/paperback.udl",
			"--language",
			"kotlin",
			"--out-dir",
			"android/app/src/main/kotlin",
			"--no-format",
		])
		.status()?;
	if !status.success() {
		return Err("uniffi-bindgen generation failed".into());
	}
	println!("Building native libraries for arm64-v8a and armeabi-v7a...");
	let status = Command::new(&cargo)
		.current_dir(project_root())
		.args(&[
			"ndk",
			"-t",
			"arm64-v8a",
			"-t",
			"armeabi-v7a",
			"-o",
			"android/app/src/main/jniLibs",
			"build",
			"--release",
			"-p",
			"paperback-core",
		])
		.status()?;
	if !status.success() {
		return Err("cargo ndk build failed".into());
	}
	println!("Android native build complete.");

	if !gradle_tasks.is_empty() {
		println!("Running gradlew with tasks: {:?}", gradle_tasks);
		let android_dir = project_root().join("android");
		let mut cmd = if cfg!(windows) {
			let mut c = Command::new("cmd");
			c.arg("/C").arg("gradlew.bat");
			c
		} else {
			Command::new("./gradlew")
		};
		cmd.current_dir(&android_dir).args(&gradle_tasks);
		let status = cmd.status()?;
		if !status.success() {
			return Err("gradlew failed".into());
		}
		println!("Gradle tasks complete.");
	} else {
		println!("Open android/ in Android Studio to build the APK.");
	}

	Ok(())
}

fn download_pdfium_so(url: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
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
		if entry.path()?.file_name().and_then(|n| n.to_str()) == Some("libpdfium.so") {
			let tmp = dest.with_extension("so.tmp");
			entry.unpack(&tmp)?;
			if dest.exists() {
				fs::remove_file(dest)?;
			}
			fs::rename(&tmp, dest)?;
			println!("Saved {}", dest.display());
			return Ok(());
		}
	}
	Err(format!("libpdfium.so not found in archive from {url}").into())
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

fn ios() -> Result<(), Box<dyn Error>> {
	let release = env::args().any(|a| a == "--release");
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let root = project_root();
	let generated_dir = root.join("ios/Paperback/Generated");
	fs::create_dir_all(&generated_dir)?;

	let pdfium_dest = root.join("ios/libpdfium.dylib");
	download_pdfium_dylib(PDFIUM_IOS_ARM64_URL, &pdfium_dest)?;

	println!("Generating Swift bindings via uniffi-bindgen...");
	let status = Command::new(&cargo)
		.current_dir(&root)
		.args(&[
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
		Command::new(&cargo).current_dir(&root).args(&build_args).args(&["--target", "aarch64-apple-ios"]).status()?;
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
		.args(&["-create-xcframework"])
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

	println!("iOS build complete.");
	println!("  XCFramework: ios/paperbackFFI.xcframework");
	println!("  Swift bindings: ios/Paperback/Generated/paperback.swift");
	println!("  Add both to the Xcode project to use the Rust core.");
	Ok(())
}

fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	patois_build::gen_pot(&root, root.join("po"), "paperback")
}

#[cfg(target_os = "macos")]
fn build_mac_dmg(target_dir: &Path) -> Result<(), Box<dyn Error>> {
	let bundle_dir = target_dir.join("Paperback.app");
	let macos_dir = bundle_dir.join("Contents/MacOS");
	let resources_dir = bundle_dir.join("Contents/Resources");
	fs::create_dir_all(&macos_dir)?;
	fs::create_dir_all(&resources_dir)?;

	// build.rs creates the bundle skeleton but only copies the binary if one already
	// existed from a prior build.  Copy the freshly-linked binary now.
	let exe = target_dir.join("paperback");
	if !exe.exists() {
		return Err("paperback binary not found after build".into());
	}
	fs::copy(&exe, macos_dir.join("paperback"))?;
	use std::os::unix::fs::PermissionsExt;
	fs::set_permissions(macos_dir.join("paperback"), fs::Permissions::from_mode(0o755))?;

	// Copy libpdfium.dylib into the bundle so it ships alongside the binary.
	let dylib_src = target_dir.join("libpdfium.dylib");
	if dylib_src.exists() {
		fs::copy(&dylib_src, macos_dir.join("libpdfium.dylib"))?;
	} else {
		println!("Warning: libpdfium.dylib not found in target directory; PDF support will be unavailable.");
	}

	// Copy sounds into the bundle's Resources so the app can find them.
	let sounds_src = target_dir.join("sounds");
	if sounds_src.exists() {
		copy_dir_all(&sounds_src, &resources_dir.join("sounds"))?;
	} else {
		println!("Warning: sounds directory not found, skipping.");
	}

	// Copy readme.
	let readme = target_dir.join("readme.html");
	if readme.exists() {
		let _ = fs::copy(&readme, resources_dir.join("readme.html"));
	}

	println!("Built app: {}", bundle_dir.display());

	// Build a DMG: staging folder contains the .app plus an /Applications symlink
	// so users get the standard drag-to-install experience.
	let staging = target_dir.join("dmg-staging");
	let _ = fs::remove_dir_all(&staging);
	fs::create_dir_all(&staging)?;
	copy_dir_all(&bundle_dir, &staging.join("Paperback.app"))?;
	std::os::unix::fs::symlink("/Applications", staging.join("Applications"))?;

	let dmg_path = target_dir.join("paperback.dmg");
	let status = Command::new("hdiutil")
		.args([
			"create",
			"-volname",
			"Paperback",
			"-srcfolder",
			&staging.to_string_lossy(),
			"-ov",
			"-format",
			"UDZO",
			&dmg_path.to_string_lossy(),
		])
		.status()?;
	if !status.success() {
		return Err("hdiutil create failed".into());
	}
	println!("Created DMG: {}", dmg_path.display());
	Ok(())
}

#[cfg(target_os = "macos")]
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
	fs::create_dir_all(dst)?;
	for entry in WalkDir::new(src) {
		let entry = entry?;
		let path = entry.path();
		let rel = path.strip_prefix(src)?;
		let dest = dst.join(rel);
		if path.is_dir() {
			fs::create_dir_all(&dest)?;
		} else {
			fs::copy(path, &dest)?;
		}
	}
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_zip_package(
	target_dir: &Path,
	exe_path: &Path,
	readme_path: &Path,
	sounds_dir: &Path,
	pdfium_dll_path: &Path,
) -> Result<(), Box<dyn Error>> {
	let package_name = if cfg!(target_os = "macos") { "paperback_mac.zip" } else { "paperback.zip" };
	let package_path = target_dir.join(package_name);
	let file = File::create(&package_path)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	let exe_filename = exe_path.file_name().unwrap();
	zip.start_file(exe_filename.to_string_lossy(), options)?;
	let mut f = File::open(exe_path)?;
	io::copy(&mut f, &mut zip)?;
	if cfg!(windows) {
		if !pdfium_dll_path.exists() {
			return Err(
				"pdfium.dll not found in target directory. Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL) before building."
					.into(),
			);
		}
		zip.start_file("pdfium.dll", options)?;
		let mut f = File::open(pdfium_dll_path)?;
		io::copy(&mut f, &mut zip)?;
	}
	if readme_path.exists() {
		zip.start_file("readme.html", options)?;
		let mut f = File::open(readme_path)?;
		io::copy(&mut f, &mut zip)?;
	} else {
		println!("Warning: readme.html not found, skipping.");
	}
	if sounds_dir.exists() {
		for entry in WalkDir::new(sounds_dir) {
			let entry = entry?;
			let path = entry.path();
			if path.is_file() {
				let relative_path = path.strip_prefix(target_dir)?;
				let name = relative_path.to_string_lossy().replace('\\', "/");
				zip.start_file(name, options)?;
				let mut f = File::open(path)?;
				io::copy(&mut f, &mut zip)?;
			}
		}
	} else {
		println!("Warning: sounds directory not found, skipping sounds.");
	}
	println!("Created zip: {}", package_path.display());
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_windows_installer(target_dir: &Path) -> io::Result<()> {
	let iss_path = target_dir.join("paperback.iss");
	if !iss_path.exists() {
		println!("Skipping installer: paperback.iss not found.");
		return Ok(());
	}
	let status = Command::new("ISCC.exe").arg(&iss_path).status();
	match status {
		Ok(s) if s.success() => println!("Installer created successfully."),
		_ => println!("Failed to run Inno Setup (ISCC.exe). Is it in your PATH?"),
	}
	Ok(())
}
