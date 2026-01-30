use std::{
	env, fs, io,
	path::{Path, PathBuf},
	process::Command,
};

use embed_manifest::{
	embed_manifest,
	manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*},
	new_manifest,
};

fn main() {
	track_packaging_inputs();
	build_translations();
	build_docs();
	configure_installer();
	generate_pot();
	let target = env::var("TARGET").unwrap_or_default();
	if target.contains("windows") {
		let manifest = new_manifest("Paperback")
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

fn track_packaging_inputs() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=Cargo.lock");
	println!("cargo:rerun-if-changed=src");
	println!("cargo:rerun-if-changed=app");
	println!("cargo:rerun-if-changed=paperback.iss.in");
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
	let src_dir = manifest_dir.join("src");
	let mut files = Vec::new();
	let _ = collect_translatable_rust_files(&src_dir, &mut files);
	if files.is_empty() {
		println!("cargo:warning=No Rust source files found for POT generation.");
		return;
	}
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let output_file = po_dir.join("paperback.pot");
	let temp_file = po_dir.join("paperback.pot.new");
	let mut cmd = Command::new("xgettext");
	cmd.arg("--keyword=t")
		.arg("--language=C")
		.arg("--from-code=UTF-8")
		.arg("--add-comments=TRANSLATORS")
		.arg("--add-location=file")
		.arg("--package-name=paperback")
		.arg(format!("--package-version={}", version))
		.arg("--msgid-bugs-address=https://github.com/trypsynth/paperback/issues")
		.arg("--copyright-holder=Quin Gillespie")
		.arg(format!("--output={}", temp_file.display()));
	for file in files {
		cmd.arg(file);
	}
	let status = cmd.status();
	match status {
		Ok(s) if s.success() => {
			// Only update the pot file if the content (excluding dates) has changed
			if pot_content_changed(&output_file, &temp_file) {
				let _ = fs::rename(&temp_file, &output_file);
			} else {
				let _ = fs::remove_file(&temp_file);
			}
		}
		_ => {
			println!("cargo:warning=Failed to generate POT file.");
			let _ = fs::remove_file(&temp_file);
		}
	}
}

/// Compare two POT files, ignoring the POT-Creation-Date header.
/// Returns true if the files differ in meaningful content.
fn pot_content_changed(old_path: &Path, new_path: &Path) -> bool {
	let strip_date = |content: &str| -> String {
		content
			.lines()
			.filter(|line| !line.starts_with("\"POT-Creation-Date:"))
			.collect::<Vec<_>>()
			.join("\n")
	};
	let old_content = fs::read_to_string(old_path).unwrap_or_default();
	let new_content = match fs::read_to_string(new_path) {
		Ok(c) => c,
		Err(_) => return true,
	};
	strip_date(&old_content) != strip_date(&new_content)
}

fn collect_translatable_rust_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				collect_translatable_rust_files(&path, files)?;
			} else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
				if ext == "rs" {
					files.push(path);
				}
			}
		}
	}
	Ok(())
}
