#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::os::unix::fs::symlink;
use std::{env, error::Error, path::Path, process::Command};
#[cfg(not(target_os = "macos"))]
use std::{
	fs::{self, File},
	io,
};

#[cfg(not(target_os = "macos"))]
use flate2::{Compression, write::GzEncoder};
#[cfg(target_os = "macos")]
use walkdir::WalkDir;
#[cfg(not(target_os = "macos"))]
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::workspace::project_root;

pub fn release() -> Result<(), Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	// Built as two separate invocations rather than `-p paperback -p pb` in one: Cargo leaks
	// wxdragon-sys's build-script native-library search paths (wxWidgets' own libs) into
	// every binary linked in the same invocation when multiple root packages are requested
	// together, even into pb, which doesn't depend on wxdragon at all. That pulled GUI DLL
	// imports (comctl32's GetWindowSubclass and friends) into pb.exe, which then failed to
	// launch on end-user machines with "entry point not found" since it never gets the
	// comctl32-v6 manifest paperback.exe's build.rs embeds. Building one package per
	// invocation keeps each link step's inputs isolated.
	for package in ["paperback", "pb"] {
		let status =
			Command::new(&cargo).current_dir(project_root()).args(["build", "--release", "-p", package]).status()?;
		if !status.success() {
			return Err(format!("Cargo build failed for {package}").into());
		}
	}
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
			let pdfium_dll_path = target_dir.join("pdfium.dll");
			build_zip_package(&target_dir, &exe_path, &pb_exe_path, &pdfium_dll_path)?;
			build_windows_installer(&target_dir)?;
		} else {
			let pdfium_so_path = target_dir.join("libpdfium.so");
			build_targz_package(&target_dir, &exe_path, &pb_exe_path, &pdfium_so_path)?;
			build_appimage(&target_dir, &exe_path, &pb_exe_path, &pdfium_so_path)?;
		}
		Ok(())
	}
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
	println!("Built app: {}", bundle_dir.display());
	sign_mac_bundle(&bundle_dir, &macos_dir)?;
	// Build a DMG: staging folder contains the .app plus an /Applications symlink
	// so users get the standard drag-to-install experience.
	let staging = target_dir.join("dmg-staging");
	let _ = fs::remove_dir_all(&staging);
	fs::create_dir_all(&staging)?;
	copy_dir_all(&bundle_dir, &staging.join("Paperback.app"))?;
	symlink("/Applications", staging.join("Applications"))?;
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

/// Signs the bundle with the Developer ID Application identity named by the
/// MACOS_SIGN_IDENTITY env var, so the shipped DMG can be notarized and doesn't trip
/// Gatekeeper's "app is damaged" check. A no-op when that var isn't set (i.e. local dev
/// builds from contributors without a signing certificate). Everything signs deepest
/// first: the third-party dylib, then the executable, then the bundle as a whole, which
/// is what Apple's docs recommend over a single `--deep` sign.
#[cfg(target_os = "macos")]
fn sign_mac_bundle(bundle_dir: &Path, macos_dir: &Path) -> Result<(), Box<dyn Error>> {
	let Ok(identity) = env::var("MACOS_SIGN_IDENTITY") else {
		println!("MACOS_SIGN_IDENTITY not set; skipping code signing.");
		return Ok(());
	};
	let dylib = macos_dir.join("libpdfium.dylib");
	if dylib.exists() {
		codesign(&dylib, &identity)?;
	}
	codesign(&macos_dir.join("paperback"), &identity)?;
	codesign(bundle_dir, &identity)?;
	println!("Signed {}", bundle_dir.display());
	Ok(())
}

#[cfg(target_os = "macos")]
fn codesign(path: &Path, identity: &str) -> Result<(), Box<dyn Error>> {
	let status = Command::new("codesign")
		.args(["--force", "--timestamp", "--options", "runtime", "--sign", identity])
		.arg(path)
		.status()?;
	if !status.success() {
		return Err(format!("codesign failed for {}", path.display()).into());
	}
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
fn add_file_to_zip(
	zip: &mut ZipWriter<File>,
	options: SimpleFileOptions,
	path: &Path,
	name: &str,
) -> Result<(), Box<dyn Error>> {
	zip.start_file(name, options)?;
	let mut f = File::open(path)?;
	io::copy(&mut f, zip)?;
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_zip_package(
	target_dir: &Path,
	exe_path: &Path,
	pb_exe_path: &Path,
	pdfium_dll_path: &Path,
) -> Result<(), Box<dyn Error>> {
	let package_path = target_dir.join("paperback.zip");
	let file = File::create(&package_path)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	let exe_filename = exe_path.file_name().unwrap().to_string_lossy().into_owned();
	add_file_to_zip(&mut zip, options, exe_path, &exe_filename)?;
	if pb_exe_path.exists() {
		let pb_filename = pb_exe_path.file_name().unwrap().to_string_lossy().into_owned();
		add_file_to_zip(&mut zip, options, pb_exe_path, &pb_filename)?;
	} else {
		println!("Warning: pb binary not found, skipping.");
	}
	if cfg!(windows) {
		if !pdfium_dll_path.exists() {
			return Err(
				"pdfium.dll not found in target directory. Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL) before building."
					.into(),
			);
		}
		add_file_to_zip(&mut zip, options, pdfium_dll_path, "pdfium.dll")?;
	}
	println!("Created zip: {}", package_path.display());
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_targz_package(
	target_dir: &Path,
	exe_path: &Path,
	pb_exe_path: &Path,
	pdfium_so_path: &Path,
) -> Result<(), Box<dyn Error>> {
	let package_path = target_dir.join("paperback.tar.gz");
	let file = File::create(&package_path)?;
	let encoder = GzEncoder::new(file, Compression::default());
	let mut tar = tar::Builder::new(encoder);
	let mut exe_file = File::open(exe_path)?;
	tar.append_file(exe_path.file_name().unwrap(), &mut exe_file)?;
	if pb_exe_path.exists() {
		let mut pb_file = File::open(pb_exe_path)?;
		tar.append_file(pb_exe_path.file_name().unwrap(), &mut pb_file)?;
	} else {
		println!("Warning: pb binary not found, skipping.");
	}
	if pdfium_so_path.exists() {
		let mut pdfium_file = File::open(pdfium_so_path)?;
		tar.append_file(pdfium_so_path.file_name().unwrap(), &mut pdfium_file)?;
	} else {
		println!("Warning: libpdfium.so not found in target directory; PDF support will be unavailable.");
	}
	tar.into_inner()?.finish()?;
	println!("Created tar.gz: {}", package_path.display());
	Ok(())
}

/// Assembles an `AppDir` and hands it to `appimagetool`. Unlike [`build_targz_package`], a
/// missing or failing `appimagetool` doesn't fail the release: the `AppImage` is a convenience
/// on top of the always-built portable tarball, the same way a missing ISCC.exe just skips the
/// Windows installer in [`build_windows_installer`] rather than failing the whole build.
#[cfg(not(target_os = "macos"))]
fn build_appimage(
	target_dir: &Path,
	exe_path: &Path,
	pb_exe_path: &Path,
	pdfium_so_path: &Path,
) -> Result<(), Box<dyn Error>> {
	let app_dir = target_dir.join("Paperback.AppDir");
	let _ = fs::remove_dir_all(&app_dir);
	let bin_dir = app_dir.join("usr/bin");
	fs::create_dir_all(&bin_dir)?;
	fs::copy(exe_path, bin_dir.join(exe_path.file_name().unwrap()))?;
	make_executable(&bin_dir.join(exe_path.file_name().unwrap()))?;
	if pb_exe_path.exists() {
		fs::copy(pb_exe_path, bin_dir.join(pb_exe_path.file_name().unwrap()))?;
		make_executable(&bin_dir.join(pb_exe_path.file_name().unwrap()))?;
	}
	if pdfium_so_path.exists() {
		fs::copy(pdfium_so_path, bin_dir.join("libpdfium.so"))?;
	}
	// The desktop file and icon double as the AppImage's required metadata and as what
	// `linux_integration.rs` copies into the user's own applications/icons directories once
	// they pick file associations, so both need to be right at the AppDir root.
	fs::copy(project_root().join("paperback.desktop"), app_dir.join("paperback.desktop"))?;
	let icon_path = project_root().join("crates/paperback/assets/paperback.png");
	fs::copy(&icon_path, app_dir.join("paperback.png"))?;
	fs::copy(&icon_path, app_dir.join(".DirIcon"))?;
	let apprun_path = app_dir.join("AppRun");
	fs::write(
		&apprun_path,
		// The AppImage runtime sets ARGV0 to the name this AppImage was invoked as when that
		// differs from its own filename (e.g. through a symlink), so the ~/.local/bin/pb
		// symlink linux_integration.rs's setup dialog can write dispatches into the pb CLI
		// instead of the paperback GUI.
		r#"#!/bin/sh
HERE="$(dirname "$(readlink -f "${0}")")"
case "$(basename "${ARGV0:-$0}")" in
	pb) exec "${HERE}/usr/bin/pb" "$@" ;;
	*) exec "${HERE}/usr/bin/paperback" "$@" ;;
esac
"#,
	)?;
	make_executable(&apprun_path)?;
	let appimagetool = env::var("APPIMAGETOOL").unwrap_or_else(|_| "appimagetool".to_string());
	let output_path = target_dir.join("paperback.AppImage");
	// `--appimage-extract-and-run` avoids needing FUSE, which CI runners don't have set up.
	match Command::new(&appimagetool).args(["--appimage-extract-and-run"]).arg(&app_dir).arg(&output_path).status() {
		Ok(status) if status.success() => println!("Created AppImage: {}", output_path.display()),
		Ok(status) => println!("Warning: appimagetool exited with {status}; skipping AppImage."),
		Err(err) => println!("Warning: failed to run appimagetool ({err}). Is it in your PATH? Skipping AppImage."),
	}
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn make_executable(path: &Path) -> io::Result<()> {
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let mut permissions = fs::metadata(path)?.permissions();
		permissions.set_mode(0o755);
		fs::set_permissions(path, permissions)?;
	}
	#[cfg(not(unix))]
	{
		let _ = path;
	}
	Ok(())
}

#[cfg(not(target_os = "macos"))]
fn build_windows_installer(target_dir: &Path) -> io::Result<()> {
	let iss_path = target_dir.join("paperback.iss");
	if !iss_path.exists() {
		println!("Skipping installer: paperback.iss not found.");
		return Ok(());
	}
	let status = Command::new("ISCC.exe").arg("/Q").arg(&iss_path).status();
	match status {
		Ok(s) if s.success() => println!("Installer created successfully."),
		_ => println!("Failed to run Inno Setup (ISCC.exe). Is it in your PATH?"),
	}
	Ok(())
}
