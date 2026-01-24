use std::{
	env, fs,
	io::{self, Write},
	path::{Path, PathBuf},
	process::Command,
};

use embed_manifest::{
	embed_manifest,
	manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*},
	new_manifest,
};
use flate2::{write::GzEncoder, Compression};
use tar::Builder as TarBuilder;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

fn main() {
	build_translations();
	build_docs();
	configure_installer();
	maybe_build_release_artifacts();
	if env::var("UPDATE_POT").is_ok() {
		generate_pot();
	}
	let target = env::var("TARGET").unwrap_or_default();
	if target.contains("windows") {
		let manifest = new_manifest("Fedra")
			.supported_os(Windows7..=Windows10)
			.active_code_page(ActiveCodePage::Utf8)
			.heap_type(HeapType::SegmentHeap)
			.dpi_awareness(DpiAwareness::PerMonitorV2)
			.long_path_aware(Setting::Enabled);
		if let Err(e) = embed_manifest(manifest) {
			println!("cargo:warning=Failed to embed manifest: {}", e);
			println!("cargo:warning=The application will still work but may lack optimal Windows theming");
		}
		println!("cargo:rerun-if-changed=build.rs");
	}
}

fn maybe_build_release_artifacts() {
	if env::var("PROFILE").ok().as_deref() != Some("release") {
		return;
	}
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target directory for packaging.");
			return;
		}
	};
	let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
	let exe_name = if target_os == "windows" {
		"paperback.exe"
	} else {
		"paperback"
	};
	let exe_path = target_dir.join(exe_name);
	if !exe_path.exists() {
		println!(
			"cargo:warning=Release packaging skipped because {} is missing.",
			exe_path.display()
		);
		return;
	}
	let readme_path = target_dir.join("readme.html");
	let langs_dir = target_dir.join("langs");
	if target_os == "windows" {
		if let Err(err) = build_zip_package(&target_dir, &exe_path, &readme_path, &langs_dir) {
			println!("cargo:warning=Failed to build zip package: {}", err);
		}
		if let Err(err) = build_windows_installer(&target_dir) {
			println!("cargo:warning=Failed to build installer: {}", err);
		}
	} else if target_os == "macos" {
		if let Err(err) = build_zip_package(&target_dir, &exe_path, &readme_path, &langs_dir) {
			println!("cargo:warning=Failed to build zip package: {}", err);
		}
	} else if let Err(err) = build_tar_package(&target_dir, &exe_path, &readme_path, &langs_dir) {
		println!("cargo:warning=Failed to build tar package: {}", err);
	}
}

fn build_zip_package(
	target_dir: &Path,
	exe_path: &Path,
	readme_path: &Path,
	langs_dir: &Path,
) -> io::Result<()> {
	let package_name = if env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("macos") {
		"paperback_mac.zip"
	} else {
		"paperback.zip"
	};
	let package_path = target_dir.join(package_name);
	let file = fs::File::create(&package_path)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
	let exe_rel = exe_path.strip_prefix(target_dir).unwrap_or(exe_path);
	add_file_to_zip(&mut zip, exe_path, exe_rel, options)?;
	if readme_path.exists() {
		let readme_rel = readme_path.strip_prefix(target_dir).unwrap_or(readme_path);
		add_file_to_zip(&mut zip, readme_path, readme_rel, options)?;
	} else {
		println!(
			"cargo:warning=readme.html missing; skipping for {}.",
			package_name
		);
	}
	if langs_dir.exists() {
		add_dir_to_zip(&mut zip, target_dir, langs_dir, options)?;
	} else {
		println!("cargo:warning=langs directory missing; skipping translations.");
	}
	zip.finish()?;
	Ok(())
}

fn build_tar_package(
	target_dir: &Path,
	exe_path: &Path,
	readme_path: &Path,
	langs_dir: &Path,
) -> io::Result<()> {
	let package_path = target_dir.join("paperback.tar.gz");
	let file = fs::File::create(&package_path)?;
	let encoder = GzEncoder::new(file, Compression::default());
	let mut tar = TarBuilder::new(encoder);
	append_file_to_tar(&mut tar, target_dir, exe_path)?;
	if readme_path.exists() {
		append_file_to_tar(&mut tar, target_dir, readme_path)?;
	} else {
		println!(
			"cargo:warning=readme.html missing; skipping for {}.",
			package_path.display()
		);
	}
	if langs_dir.exists() {
		let rel = langs_dir.strip_prefix(target_dir).unwrap_or(langs_dir);
		tar.append_dir_all(rel, langs_dir)?;
	} else {
		println!("cargo:warning=langs directory missing; skipping translations.");
	}
	tar.finish()?;
	Ok(())
}

fn add_file_to_zip<W: Write + io::Seek>(
	zip: &mut ZipWriter<W>,
	path: &Path,
	name: &Path,
	options: SimpleFileOptions,
) -> io::Result<()> {
	let mut file = fs::File::open(path)?;
	let name = name.to_string_lossy().replace('\\', "/");
	zip.start_file(name, options)?;
	io::copy(&mut file, zip)?;
	Ok(())
}

fn add_dir_to_zip<W: Write + io::Seek>(
	zip: &mut ZipWriter<W>,
	base: &Path,
	dir: &Path,
	options: SimpleFileOptions,
) -> io::Result<()> {
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			add_dir_to_zip(zip, base, &path, options)?;
		} else {
			let rel = path.strip_prefix(base).unwrap_or(&path);
			add_file_to_zip(zip, &path, rel, options)?;
		}
	}
	Ok(())
}

fn append_file_to_tar<W: Write>(
	tar: &mut TarBuilder<W>,
	base: &Path,
	path: &Path,
) -> io::Result<()> {
	let rel = path.strip_prefix(base).unwrap_or(path);
	tar.append_path_with_name(path, rel)?;
	Ok(())
}

fn build_windows_installer(target_dir: &Path) -> io::Result<()> {
	if env::var("CARGO_CFG_TARGET_OS").ok().as_deref() != Some("windows") {
		return Ok(());
	}
	let script_path = target_dir.join("paperback.iss");
	if !script_path.exists() {
		println!("cargo:warning=Installer script not found; skipping.");
		return Ok(());
	}
	let iscc = match find_inno_setup() {
		Some(path) => path,
		None => {
			println!("cargo:warning=Inno Setup not found; skipping installer.");
			return Ok(());
		}
	};
	let status = Command::new(iscc)
		.arg(&script_path)
		.current_dir(target_dir)
		.status();
	match status {
		Ok(s) if s.success() => Ok(()),
		Ok(s) => {
			println!("cargo:warning=Inno Setup exited with status {}", s);
			Ok(())
		}
		Err(err) => {
			println!("cargo:warning=Failed to run Inno Setup: {}", err);
			Ok(())
		}
	}
}

fn find_inno_setup() -> Option<PathBuf> {
	let candidates = [
		("ProgramFiles(x86)", "Inno Setup 6\\ISCC.exe"),
		("LOCALAPPDATA", "Programs\\Inno Setup 6\\ISCC.exe"),
		("ProgramFiles", "Inno Setup 6\\ISCC.exe"),
	];
	for (env_var, suffix) in candidates {
		if let Ok(root) = env::var(env_var) {
			let path = PathBuf::from(root).join(suffix);
			if path.exists() {
				return Some(path);
			}
		}
	}
	if Command::new("ISCC.exe").arg("/?").output().is_ok() {
		return Some(PathBuf::from("ISCC.exe"));
	}
	None
}

fn build_translations() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let po_dir = manifest_dir.join("po");
	println!("cargo:rerun-if-changed={}", po_dir.display());
	if !po_dir.exists() {
		return;
	}
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target output directory for translations.");
			return;
		}
	};
	let langs_dir = target_dir.join("langs");
	if let Err(err) = fs::create_dir_all(&langs_dir) {
		println!("cargo:warning=Failed to create langs directory: {}", err);
		return;
	}
	let po_files = match fs::read_dir(&po_dir) {
		Ok(entries) => entries,
		Err(err) => {
			println!("cargo:warning=Failed to read po directory: {}", err);
			return;
		}
	};
	for entry in po_files {
		let entry = match entry {
			Ok(entry) => entry,
			Err(err) => {
				println!("cargo:warning=Failed to read po file: {}", err);
				continue;
			}
		};
		let path = entry.path();
		if path.extension().and_then(|ext| ext.to_str()) != Some("po") {
			continue;
		}
		if path.file_stem().and_then(|stem| stem.to_str()) == Some("paperback") {
			continue;
		}
		let lang = match path.file_stem().and_then(|stem| stem.to_str()) {
			Some(lang) => lang,
			None => continue,
		};
		println!("cargo:rerun-if-changed={}", path.display());
		let output_dir = langs_dir.join(lang).join("LC_MESSAGES");
		if let Err(err) = fs::create_dir_all(&output_dir) {
			println!("cargo:warning=Failed to create translation output directory: {}", err);
			continue;
		}
		let output_path = output_dir.join("paperback.mo");
		if !run_msgfmt(&path, &output_path) {
			println!("cargo:warning=Failed to compile {}", path.display());
		}
	}
}

fn run_msgfmt(input: &Path, output: &Path) -> bool {
	let status = Command::new("msgfmt").arg(input).arg("-o").arg(output).status();
	match status {
		Ok(status) if status.success() => true,
		Ok(status) => {
			println!("cargo:warning=msgfmt exited with status {}", status);
			false
		}
		Err(err) => {
			println!("cargo:warning=Failed to run msgfmt: {}", err);
			false
		}
	}
}

fn target_profile_dir() -> Option<PathBuf> {
	let profile = env::var("PROFILE").ok()?;
	if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
		let mut dir = PathBuf::from(target_dir);
		dir.push(profile);
		return Some(dir);
	}
	let out_dir = PathBuf::from(env::var("OUT_DIR").ok()?);
	out_dir.ancestors().nth(3).map(Path::to_path_buf)
}

fn build_docs() {
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target directory for docs.");
			return;
		}
	};
	let doc_dir = PathBuf::from("doc");
	let readme = doc_dir.join("readme.md");
	let config = doc_dir.join("pandoc.yaml");
	println!("cargo:rerun-if-changed={}", readme.display());
	println!("cargo:rerun-if-changed={}", config.display());
	let pandoc_check = Command::new("pandoc").arg("--version").output();
	if pandoc_check.is_err() {
		println!("cargo:warning=Pandoc not found. Documentation will not be generated.");
		return;
	}
	let output = target_dir.join("readme.html");
	let status = Command::new("pandoc")
		.arg(format!("--defaults={}", config.display()))
		.arg(&readme)
		.arg("-o")
		.arg(&output)
		.status();
	match status {
		Ok(s) if s.success() => {}
		_ => println!("cargo:warning=Failed to generate documentation."),
	}
}

fn configure_installer() {
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => return,
	};
	let input_path = PathBuf::from("paperback.iss.in");
	println!("cargo:rerun-if-changed={}", input_path.display());
	if !input_path.exists() {
		return;
	}
	let content = match fs::read_to_string(&input_path) {
		Ok(c) => c,
		Err(e) => {
			println!("cargo:warning=Failed to read installer script: {}", e);
			return;
		}
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let new_content = content.replace("@PROJECT_VERSION@", &version);
	let output_path = target_dir.join("paperback.iss");
	if let Err(e) = fs::write(&output_path, new_content) {
		println!("cargo:warning=Failed to write installer script: {}", e);
	}
}

fn generate_pot() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let po_dir = manifest_dir.join("po");
	if !po_dir.exists() {
		let _ = fs::create_dir(&po_dir);
	}
	let xgettext_check = Command::new("xgettext").arg("--version").output();
	if xgettext_check.is_err() {
		println!("cargo:warning=xgettext not found. Translation template (.pot) generation will not be available.");
		return;
	}
	let app_dir = manifest_dir.join("app");
	let mut files = Vec::new();
	let _ = collect_translatable_files(&app_dir, &mut files);
	if files.is_empty() {
		return;
	}
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let output_file = po_dir.join("paperback.pot");
	let mut cmd = Command::new("xgettext");
	cmd.arg("--keyword=_")
		.arg("--keyword=wxPLURAL:1,2")
		.arg("--keyword=wxTRANSLATE")
		.arg("--language=C++")
		.arg("--from-code=UTF-8")
		.arg("--add-comments=TRANSLATORS")
		.arg("--add-location=file")
		.arg("--package-name=paperback")
		.arg(format!("--package-version={}", version))
		.arg("--msgid-bugs-address=https://github.com/trypsynth/paperback/issues")
		.arg("--copyright-holder=Quin Gillespie")
		.arg(format!("--output={}", output_file.display()));
	for file in files {
		cmd.arg(file);
	}
	let status = cmd.status();
	match status {
		Ok(s) if s.success() => {}
		_ => println!("cargo:warning=Failed to generate POT file."),
	}
}

fn collect_translatable_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				collect_translatable_files(&path, files)?;
			} else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
				if matches!(ext, "cpp" | "hpp" | "h") {
					files.push(path);
				}
			}
		}
	}
	Ok(())
}
