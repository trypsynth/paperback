#[cfg(target_os = "macos")]
use std::fs;
use std::{env, error::Error, path::Path, process::Command};
#[cfg(not(target_os = "macos"))]
use std::{fs::File, io};

use walkdir::WalkDir;
#[cfg(not(target_os = "macos"))]
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

pub fn release() -> Result<(), Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let status = Command::new(&cargo)
		.current_dir(crate::project_root())
		.args(["build", "--release", "-p", "paperback", "-p", "pb"])
		.status()?;
	if !status.success() {
		return Err("Cargo build failed".into());
	}
	let target_dir = crate::project_root().join("target/release");
	#[cfg(target_os = "macos")]
	return build_mac_dmg(&target_dir);
	#[cfg(not(target_os = "macos"))]
	{
		let exe_name = if cfg!(windows) { "paperback.exe" } else { "paperback" };
		let pb_exe_name = if cfg!(windows) { "pb.exe" } else { "pb" };
		let exe_path = target_dir.join(exe_name);
		let pb_exe_path = target_dir.join(pb_exe_name);
		let sounds_path = target_dir.join("sounds");
		let pdfium_dll_path = target_dir.join("pdfium.dll");
		if !exe_path.exists() {
			return Err("Executable not found".into());
		}
		println!("Packaging binary and sounds...");
		build_zip_package(&target_dir, &exe_path, &pb_exe_path, &sounds_path, &pdfium_dll_path)?;
		if cfg!(windows) {
			build_windows_installer(&target_dir)?;
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

	// Copy sounds into the bundle's Resources so the app can find them.
	let sounds_src = target_dir.join("sounds");
	if sounds_src.exists() {
		copy_dir_all(&sounds_src, &resources_dir.join("sounds"))?;
	} else {
		println!("Warning: sounds directory not found, skipping.");
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
	pb_exe_path: &Path,
	sounds_dir: &Path,
	pdfium_dll_path: &Path,
) -> Result<(), Box<dyn Error>> {
	let package_path = target_dir.join("paperback.zip");
	let file = File::create(&package_path)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	let exe_filename = exe_path.file_name().unwrap();
	zip.start_file(exe_filename.to_string_lossy(), options)?;
	let mut f = File::open(exe_path)?;
	io::copy(&mut f, &mut zip)?;
	if pb_exe_path.exists() {
		let pb_filename = pb_exe_path.file_name().unwrap();
		zip.start_file(pb_filename.to_string_lossy(), options)?;
		let mut f = File::open(pb_exe_path)?;
		io::copy(&mut f, &mut zip)?;
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
		zip.start_file("pdfium.dll", options)?;
		let mut f = File::open(pdfium_dll_path)?;
		io::copy(&mut f, &mut zip)?;
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
