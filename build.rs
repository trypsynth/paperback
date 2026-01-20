use std::{
	env, fs,
	path::{Path, PathBuf},
	process::Command,
};

use embed_manifest::{
	embed_manifest,
	manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*},
	new_manifest,
};

fn main() {
	build_translations();
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
