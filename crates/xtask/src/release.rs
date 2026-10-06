use std::error::Error;

use shipfitter::package::cargo_build_release;

use crate::workspace::project_root;

pub fn release() -> Result<(), Box<dyn Error>> {
	crate::wxwidgets::ensure()?;
	// Built as two separate invocations rather than `-p paperback -p pb` in one: Cargo leaks
	// wxdragon-sys's build-script native-library search paths (wxWidgets' own libs) into
	// every binary linked in the same invocation when multiple root packages are requested
	// together, even into pb, which doesn't depend on wxdragon at all. That pulled GUI DLL
	// imports (comctl32's GetWindowSubclass and friends) into pb.exe, which then failed to
	// launch on end-user machines with "entry point not found" since it never gets the
	// comctl32-v6 manifest paperback.exe's build.rs embeds. cargo_build_release builds one
	// package per invocation, which keeps each link step's inputs isolated.
	cargo_build_release(&project_root(), &["paperback", "pb"])?;
	let target_dir = project_root().join("target/release");
	#[cfg(target_os = "macos")]
	return build_mac_dmg(&target_dir);
	#[cfg(not(target_os = "macos"))]
	{
		let exe_name = if cfg!(windows) { "paperback.exe" } else { "paperback" };
		let pb_exe_name = if cfg!(windows) { "pb.exe" } else { "pb" };
		let exe_path = target_dir.join(exe_name);
		let pb_exe_path = target_dir.join(pb_exe_name);
		if !exe_path.exists() {
			return Err("Executable not found".into());
		}
		println!("Packaging binary...");
		if cfg!(windows) {
			build_zip_package(&target_dir, &exe_path, &pb_exe_path)?;
			shipfitter::package::inno_setup(&target_dir.join("paperback.iss"))?;
		} else {
			build_linux_packages(&target_dir, &exe_path, &pb_exe_path)?;
		}
		Ok(())
	}
}

#[cfg(target_os = "macos")]
fn build_mac_dmg(target_dir: &std::path::Path) -> Result<(), Box<dyn Error>> {
	use std::fs;

	use shipfitter::macos::{dmg, sign};

	// build.rs lays out the bundle and its Info.plist, but runs before the binaries are linked, so
	// the freshly linked GUI and CLI binaries are copied in now.
	let bundle = target_dir.join("Paperback.app");
	let macos_dir = bundle.join("Contents/MacOS");
	let exe = target_dir.join("paperback");
	if !exe.exists() {
		return Err("paperback binary not found after build".into());
	}
	let pb_exe = target_dir.join("pb");
	if !pb_exe.exists() {
		return Err("pb binary not found after build".into());
	}
	fs::copy(&exe, macos_dir.join("paperback"))?;
	fs::copy(&pb_exe, macos_dir.join("pb"))?;
	let dylib = target_dir.join("libpdfium.dylib");
	if dylib.exists() {
		fs::copy(&dylib, macos_dir.join("libpdfium.dylib"))?;
	} else {
		println!("Warning: libpdfium.dylib not found in target directory; PDF support will be unavailable.");
	}
	println!("Built app: {}", bundle.display());
	sign(&bundle, &[])?;
	let dmg_path = target_dir.join("paperback.dmg");
	dmg(&bundle, &dmg_path)?;
	println!("Created DMG: {}", dmg_path.display());
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_zip_package(
	target_dir: &std::path::Path,
	exe_path: &std::path::Path,
	pb_exe_path: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
	let package_path = target_dir.join("paperback.zip");
	let mut zip = shipfitter::package::Zip::create(&package_path)?;
	zip.file(exe_path, "paperback.exe")?;
	if pb_exe_path.exists() {
		zip.file(pb_exe_path, "pb.exe")?;
	} else {
		println!("Warning: pb binary not found, skipping.");
	}
	let pdfium_dll_path = target_dir.join("pdfium.dll");
	if !pdfium_dll_path.exists() {
		return Err(
			"pdfium.dll not found in target directory. Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL) before building."
				.into(),
		);
	}
	zip.file(&pdfium_dll_path, "pdfium.dll")?;
	zip.finish()?;
	println!("Created zip: {}", package_path.display());
	Ok(())
}

/// The portable tarball, then an `AppImage` on top of it.
#[cfg(not(target_os = "macos"))]
fn build_linux_packages(
	target_dir: &std::path::Path,
	exe_path: &std::path::Path,
	pb_exe_path: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
	use shipfitter::package::{AppImage, TarGz};

	let pdfium_so_path = target_dir.join("libpdfium.so");
	let mut binaries = vec![exe_path];
	if pb_exe_path.exists() {
		binaries.push(pb_exe_path);
	} else {
		println!("Warning: pb binary not found, skipping.");
	}
	if pdfium_so_path.exists() {
		binaries.push(&pdfium_so_path);
	} else {
		println!("Warning: libpdfium.so not found in target directory; PDF support will be unavailable.");
	}
	let package_path = target_dir.join("paperback.tar.gz");
	let mut tarball = TarGz::create(&package_path)?;
	for binary in &binaries {
		tarball.file(binary, &binary.file_name().unwrap().to_string_lossy())?;
	}
	tarball.finish()?;
	println!("Created tar.gz: {}", package_path.display());
	// The desktop file and icon double as the AppImage's required metadata and as what
	// `linux_integration.rs` copies into the user's own applications/icons directories once
	// they pick file associations, so both need to be right at the AppDir root.
	let root = project_root();
	let appimage = AppImage {
		binaries: &binaries,
		desktop_file: &root.join("paperback.desktop"),
		icon: &root.join("crates/paperback/assets/paperback.png"),
		// The AppImage runtime sets ARGV0 to the name this AppImage was invoked as when that
		// differs from its own filename (e.g. through a symlink), so the ~/.local/bin/pb
		// symlink linux_integration.rs's setup dialog can write dispatches into the pb CLI
		// instead of the paperback GUI.
		app_run: Some(
			r#"#!/bin/sh
HERE="$(dirname "$(readlink -f "${0}")")"
case "$(basename "${ARGV0:-$0}")" in
	pb) exec "${HERE}/usr/bin/pb" "$@" ;;
	*) exec "${HERE}/usr/bin/paperback" "$@" ;;
esac
"#,
		),
	};
	let output_path = target_dir.join("paperback.AppImage");
	if appimage.build(&output_path)? {
		println!("Created AppImage: {}", output_path.display());
	}
	Ok(())
}
