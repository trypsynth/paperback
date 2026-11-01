use std::{env, error::Error, process::Command};

use crate::{
	print_help,
	workspace::{MOBILE_CORE_FEATURES, build_host_ffi_library, project_root},
};

pub fn android() -> Result<(), Box<dyn Error>> {
	let args = env::args().skip(2);
	let mut gradle_tasks = Vec::new();
	let mut build_aab = false;
	for arg in args {
		match arg.as_str() {
			"--release" => gradle_tasks.push("assembleRelease"),
			"--debug" => gradle_tasks.push("assembleDebug"),
			"--installrelease" | "--install-release" => gradle_tasks.push("installRelease"),
			"--installdebug" | "--install-debug" => gradle_tasks.push("installDebug"),
			"--build-aab" => build_aab = true,
			_ => {
				print_help();
				return Err(format!("Unknown argument for android: {arg}").into());
			}
		}
	}
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	println!("Generating Kotlin bindings via uniffi-bindgen...");
	let ffi_library = build_host_ffi_library(&cargo)?.to_string_lossy().into_owned();
	let status = Command::new(&cargo)
		.current_dir(project_root())
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
			"kotlin",
			"--out-dir",
			"android/app/src/main/kotlin",
			"--no-format",
		])
		.status()?;
	if !status.success() {
		return Err("uniffi-bindgen generation failed".into());
	}
	println!("Building native libraries for arm64-v8a, armeabi-v7a, and x86_64...");
	let status = Command::new(&cargo)
		.current_dir(project_root())
		.args([
			"ndk",
			"-t",
			"arm64-v8a",
			"-t",
			"armeabi-v7a",
			"-t",
			"x86_64",
			"-o",
			"android/app/src/main/jniLibs",
			"rustc",
			"--release",
			"-p",
			"paperback-core",
			"--features",
			MOBILE_CORE_FEATURES,
			"--lib",
			"--crate-type",
			"cdylib",
		])
		.status()?;
	if !status.success() {
		return Err("cargo ndk build failed".into());
	}
	println!("Android native build complete.");
	let po_dir = project_root().join("po");
	let assets_dir = project_root().join("android/app/src/main/assets");
	if po_dir.is_dir()
		&& let Err(e) = patois_build::gen_android_strings(&po_dir, &assets_dir)
	{
		println!("Warning: could not generate Android translations: {e}");
	}
	if let Err(e) = crate::docs::write_app_readmes(&assets_dir.join("readmes")) {
		println!("Warning: could not generate the Help documents: {e}");
	}
	if gradle_tasks.is_empty() && !build_aab {
		println!("Open android/ in Android Studio to build the APK.");
	} else {
		if !gradle_tasks.is_empty() {
			run_gradle(&gradle_tasks)?;
			println!("Gradle tasks complete.");
		}
		if build_aab {
			// Its own invocation: build.gradle.kts disables per-ABI splits when any task in the same invocation contains "bundle", which would rename the APKs CI expects.
			run_gradle(&["bundleRelease"])?;
			println!("App Bundle built: android/app/build/outputs/bundle/release/app-release.aab");
		}
	}
	Ok(())
}

fn run_gradle(tasks: &[&str]) -> Result<(), Box<dyn Error>> {
	println!("Running gradlew with tasks: {tasks:?}");
	let android_dir = project_root().join("android");
	let mut cmd = if cfg!(windows) {
		let mut c = Command::new("cmd");
		// With NoDefaultCurrentDirectoryInExePath set, cmd /C can't find a bare "gradlew.bat" in the working directory.
		c.arg("/C").arg(".\\gradlew.bat");
		c
	} else {
		Command::new("./gradlew")
	};
	cmd.current_dir(&android_dir).args(tasks);
	let status = cmd.status()?;
	if !status.success() {
		return Err(format!("gradlew failed for tasks: {tasks:?}").into());
	}
	Ok(())
}
