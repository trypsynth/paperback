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

const PDFIUM_ANDROID_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-android-arm64.tgz";
const PDFIUM_ANDROID_ARM_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-android-arm.tgz";

pub fn android() -> Result<(), Box<dyn Error>> {
	let args = env::args().skip(2);
	let mut gradle_tasks = Vec::new();

	for arg in args {
		match arg.as_str() {
			"--release" => gradle_tasks.push("assembleRelease"),
			"--debug" => gradle_tasks.push("assembleDebug"),
			"--installrelease" | "--install-release" => gradle_tasks.push("installRelease"),
			"--installdebug" | "--install-debug" => gradle_tasks.push("installDebug"),
			_ => {
				crate::print_help();
				return Err(format!("Unknown argument for android: {}", arg).into());
			}
		}
	}

	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let jni_libs = crate::project_root().join("android/app/src/main/jniLibs");
	download_pdfium_so(PDFIUM_ANDROID_ARM64_URL, &jni_libs.join("arm64-v8a/libpdfium.so"))?;
	download_pdfium_so(PDFIUM_ANDROID_ARM_URL, &jni_libs.join("armeabi-v7a/libpdfium.so"))?;

	println!("Generating Kotlin bindings via uniffi-bindgen...");
	let status = Command::new(&cargo)
		.current_dir(crate::project_root())
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
		.current_dir(crate::project_root())
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
		let android_dir = crate::project_root().join("android");
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
