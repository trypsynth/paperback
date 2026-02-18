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
use winres::WindowsResource;

fn main() {
	track_packaging_inputs();
	build_translations();
	copy_sounds();
	build_docs();
	configure_installer();
	generate_pot();
	let target = env::var("TARGET").unwrap_or_default();
	if target.contains("apple") {
		// Homebrew's libiconv is keg-only and not on the default search path.
		// wxWidgets links against it, so we need to tell the linker where to find it.
		let homebrew_prefix = if target.contains("aarch64") { "/opt/homebrew" } else { "/usr/local" };
		let iconv_lib = format!("{}/opt/libiconv/lib", homebrew_prefix);
		if Path::new(&iconv_lib).exists() {
			println!("cargo:rustc-link-search=native={}", iconv_lib);
		}
		generate_app_bundle();
	}
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
		embed_version_info();
		embed_commit_hash();
		println!("cargo:rerun-if-changed=build.rs");
	}
}

fn embed_commit_hash() {
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
	let hash = match output {
		Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
		_ => "unknown".to_string(),
	};
	println!("cargo:rustc-env=PAPERBACK_COMMIT_HASH={hash}");
}

fn embed_version_info() {
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let description = env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default();
	let mut res = WindowsResource::new();
	res.set("ProductName", "Paperback")
		.set("FileDescription", &description)
		.set("LegalCopyright", "Copyright © 2025 Quin Gillespie")
		.set("CompanyName", "Quin Gillespie")
		.set("OriginalFilename", "paperback.exe")
		.set("ProductVersion", &version)
		.set("FileVersion", &version);
	if let Err(e) = res.compile() {
		println!("cargo:warning=Failed to embed version info: {}", e);
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

fn copy_sounds() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let sounds_src = manifest_dir.join("sounds");
	println!("cargo:rerun-if-changed={}", sounds_src.display());
	if !sounds_src.exists() {
		return;
	}
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target output directory for sounds.");
			return;
		}
	};
	let sounds_dst = target_dir.join("sounds");
	if let Err(err) = fs::create_dir_all(&sounds_dst) {
		println!("cargo:warning=Failed to create sounds directory: {}", err);
		return;
	}
	let entries = match fs::read_dir(&sounds_src) {
		Ok(entries) => entries,
		Err(err) => {
			println!("cargo:warning=Failed to read sounds directory: {}", err);
			return;
		}
	};
	for entry in entries {
		let Ok(entry) = entry else { continue };
		let path = entry.path();
		if path.is_file() {
			let dest = sounds_dst.join(entry.file_name());
			if let Err(err) = fs::copy(&path, &dest) {
				println!("cargo:warning=Failed to copy sound file: {}", err);
			}
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
		.arg("--no-location")
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
		content.lines().filter(|line| !line.starts_with("\"POT-Creation-Date:")).collect::<Vec<_>>().join("\n")
	};
	let old_content = fs::read_to_string(old_path).unwrap_or_default();
	let new_content = match fs::read_to_string(new_path) {
		Ok(c) => c,
		Err(_) => return true,
	};
	strip_date(&old_content) != strip_date(&new_content)
}

fn generate_app_bundle() {
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target directory for macOS app bundle.");
			return;
		}
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let bundle_dir = target_dir.join("Paperback.app/Contents");
	let macos_dir = bundle_dir.join("MacOS");
	let _ = fs::create_dir_all(&macos_dir);
	let _ = fs::create_dir_all(bundle_dir.join("Resources"));
	let plist = format!(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key>
	<string>Paperback</string>
	<key>CFBundleDisplayName</key>
	<string>Paperback</string>
	<key>CFBundleIdentifier</key>
	<string>com.trypsynth.paperback</string>
	<key>CFBundleVersion</key>
	<string>{version}</string>
	<key>CFBundleShortVersionString</key>
	<string>{version}</string>
	<key>CFBundleExecutable</key>
	<string>paperback</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>CFBundleDocumentTypes</key>
	<array>
		<dict>
			<key>CFBundleTypeRole</key>
			<string>Viewer</string>
			<key>CFBundleTypeExtensions</key>
			<array>
				<string>epub</string>
				<string>pdf</string>
				<string>docx</string>
				<string>odt</string>
				<string>pptx</string>
				<string>odp</string>
				<string>chm</string>
				<string>fb2</string>
				<string>html</string>
				<string>htm</string>
				<string>md</string>
				<string>txt</string>
				<string>rtf</string>
			</array>
			<key>CFBundleTypeName</key>
			<string>Document</string>
		</dict>
	</array>
</dict>
</plist>"#
	);
	let plist_path = bundle_dir.join("Info.plist");
	if let Err(e) = fs::write(&plist_path, plist) {
		println!("cargo:warning=Failed to write Info.plist: {}", e);
		return;
	}
	// Copy the binary into the bundle if it exists (from a previous build)
	let exe_path = target_dir.join("paperback");
	let bundle_exe = macos_dir.join("paperback");
	if exe_path.exists() {
		let _ = fs::copy(&exe_path, &bundle_exe);
	}
	// Copy readme.html and langs into Resources if they exist
	let readme = target_dir.join("readme.html");
	if readme.exists() {
		let _ = fs::copy(&readme, bundle_dir.join("Resources/readme.html"));
	}
	let langs = target_dir.join("langs");
	if langs.exists() {
		let _ = copy_dir_recursive(&langs, &bundle_dir.join("Resources/langs"));
	}
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
	fs::create_dir_all(dst)?;
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let src_path = entry.path();
		let dst_path = dst.join(entry.file_name());
		if src_path.is_dir() {
			copy_dir_recursive(&src_path, &dst_path)?;
		} else {
			fs::copy(&src_path, &dst_path)?;
		}
	}
	Ok(())
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
