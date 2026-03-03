use std::{
	env,
	error::Error,
	fs::{self, File},
	io::{self, Cursor, Read},
	path::{Path, PathBuf},
	process::Command,
};

use flate2::read::GzDecoder;
use tar::Archive;
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

const PDFIUM_WIN_X64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x64.tgz";

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release()?,
		Some("pdfium") => sync_pdfium_win_x64()?,
		_ => print_help(),
	}
	Ok(())
}

fn print_help() {
	println!("Tasks:");
	println!("	release	Build release binaries and package them");
	println!("	pdfium	Download latest pdfium.dll (Windows x64) into vendor/");
}

fn release() -> Result<(), Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let status = Command::new(cargo).current_dir(project_root()).args(&["build", "--release"]).status()?;
	if !status.success() {
		return Err("Cargo build failed".into());
	}
	let target_dir = project_root().join("target/release");
	let exe_name = if cfg!(windows) { "paperback.exe" } else { "paperback" };
	let exe_path = target_dir.join(exe_name);
	let readme_path = target_dir.join("readme.html");
	let langs_path = target_dir.join("langs");
	let sounds_path = target_dir.join("sounds");
	let pdfium_dll_path = target_dir.join("pdfium.dll");
	if !exe_path.exists() {
		return Err("Executable not found".into());
	}
	println!("Packaging binaries, docs, and translations...");
	build_zip_package(&target_dir, &exe_path, &readme_path, &langs_path, &sounds_path, &pdfium_dll_path)?;
	if cfg!(windows) {
		build_windows_installer(&target_dir)?;
	}
	Ok(())
}

fn sync_pdfium_win_x64() -> Result<(), Box<dyn Error>> {
	let vendor_dir = project_root().join("vendor").join("pdfium").join("win-x64");
	fs::create_dir_all(&vendor_dir)?;
	let dest_dll = vendor_dir.join("pdfium.dll");
	println!("Downloading {}", PDFIUM_WIN_X64_URL);
	let mut response = ureq::get(PDFIUM_WIN_X64_URL).call()?.into_body();
	let mut archive_bytes = Vec::new();
	response.as_reader().read_to_end(&mut archive_bytes)?;
	let decoder = GzDecoder::new(Cursor::new(archive_bytes));
	let mut archive = Archive::new(decoder);
	let mut found = false;
	for entry in archive.entries()? {
		let mut entry = entry?;
		let path = entry.path()?;
		let file_name = path.file_name().and_then(|name| name.to_str());
		if file_name == Some("pdfium.dll") {
			let tmp_path = dest_dll.with_extension("dll.tmp");
			entry.unpack(&tmp_path)?;
			if dest_dll.exists() {
				fs::remove_file(&dest_dll)?;
			}
			fs::rename(tmp_path, &dest_dll)?;
			found = true;
			break;
		}
	}
	if !found {
		return Err("pdfium.dll was not found inside the downloaded archive".into());
	}
	println!("Saved {}", dest_dll.display());
	Ok(())
}

fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

fn build_zip_package(
	target_dir: &Path,
	exe_path: &Path,
	readme_path: &Path,
	langs_dir: &Path,
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
	if langs_dir.exists() {
		for entry in WalkDir::new(langs_dir) {
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
		println!("Warning: langs directory not found, skipping translations.");
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
