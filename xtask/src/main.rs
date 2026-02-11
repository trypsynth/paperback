use std::{
	env,
	error::Error,
	fs::File,
	io,
	path::{Path, PathBuf},
	process::Command,
};

use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release()?,
		_ => print_help(),
	}
	Ok(())
}

fn print_help() {
	println!("Tasks:");
	println!("	release	Build release binaries and package them");
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
	if !exe_path.exists() {
		return Err("Executable not found".into());
	}
	println!("Packaging binaries, docs, and translations...");
	build_zip_package(&target_dir, &exe_path, &readme_path, &langs_path)?;
	if cfg!(windows) {
		build_windows_installer(&target_dir)?;
	}
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
) -> Result<(), Box<dyn Error>> {
	let package_name = if cfg!(target_os = "windows") {
		"paperback_windows.zip"
	} else if cfg!(target_os = "macos") {
		"paperback_mac.zip"
	} else if cfg!(target_os = "linux") {
		"paperback_linux.zip"
	} else {
		"paperback.zip"
	};
	let package_path = target_dir.join(package_name);
	let file = File::create(&package_path)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	let exe_filename = exe_path.file_name().unwrap();
	zip.start_file(exe_filename.to_string_lossy(), options)?;
	let mut f = File::open(exe_path)?;
	io::copy(&mut f, &mut zip)?;

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
