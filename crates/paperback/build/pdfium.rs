//! Getting the prebuilt `PDFium` library next to the executable: honour whatever the environment
//! points at, otherwise fetch the pinned release build for the target from `pdfium-fetch`.

use std::{
	env, fs, io,
	path::{Path, PathBuf},
};

use crate::paths::{self, target_profile_dir};

pub fn copy_dll() {
	let target = env::var("TARGET").unwrap_or_default();
	if !target.contains("windows") {
		return;
	}
	let mut candidates = Vec::new();
	for var in ["PAPERBACK_PDFIUM_DLL", "PDFIUM_DLL_PATH"] {
		if let Ok(path) = env::var(var) {
			push_dll_candidates_from_path(&mut candidates, PathBuf::from(path));
		}
	}
	let manifest_dir = paths::manifest_dir();
	candidates.push(manifest_dir.join("pdfium.dll"));
	candidates.push(manifest_dir.join("bin").join("pdfium.dll"));
	candidates.extend(find_pdfium_dll_in_path());
	place(
		"pdfium.dll",
		&["PAPERBACK_PDFIUM_DLL", "PDFIUM_DLL_PATH"],
		candidates,
		"Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL), install pdfium.dll on PATH, or place it in the project root.",
	);
}

pub fn copy_dylib() {
	let candidates = env::var("PAPERBACK_PDFIUM_DYLIB").map(PathBuf::from).into_iter().collect();
	place("libpdfium.dylib", &["PAPERBACK_PDFIUM_DYLIB"], candidates, "Set PAPERBACK_PDFIUM_DYLIB.");
}

pub fn copy_so() {
	let candidates = env::var("PAPERBACK_PDFIUM_SO").map(PathBuf::from).into_iter().collect();
	place("libpdfium.so", &["PAPERBACK_PDFIUM_SO"], candidates, "Set PAPERBACK_PDFIUM_SO.");
}

/// Puts `filename` in the target directory: the first of `candidates` that exists, else the copy already there, else a fresh download. `PAPERBACK_REFRESH_PDFIUM` skips the copy already there, which is how a local build picks up a newer pin.
fn place(filename: &str, env_vars: &[&str], candidates: Vec<PathBuf>, hint: &str) {
	for var in env_vars.iter().chain(&["PAPERBACK_SKIP_PDFIUM_DOWNLOAD", "PAPERBACK_REFRESH_PDFIUM"]) {
		println!("cargo:rerun-if-env-changed={var}");
	}
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target output directory for {filename}.");
		return;
	};
	let dest = target_dir.join(filename);
	if let Some(source) = candidates.into_iter().find(|path| path.is_file()) {
		println!("cargo:rerun-if-changed={}", source.display());
		if source != dest {
			if let Err(err) = fs::copy(&source, &dest) {
				println!("cargo:warning=Failed to copy {filename} from {}: {}", source.display(), err);
			}
			return;
		}
	}
	if dest.exists() && !env_flag("PAPERBACK_REFRESH_PDFIUM") {
		return;
	}
	match fetch(filename, &dest) {
		Ok(()) => println!("cargo:rerun-if-changed={}", dest.display()),
		Err(err) => println!("cargo:warning={filename} not found. Automatic download failed: {err}. {hint}"),
	}
}

fn fetch(filename: &str, dest: &Path) -> io::Result<()> {
	if env_flag("PAPERBACK_SKIP_PDFIUM_DOWNLOAD") {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
	let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
	let archive = pdfium_fetch::desktop(&os, &arch)
		.ok_or_else(|| io::Error::other(format!("no PDFium archive pinned for {os} {arch}")))?;
	println!("cargo:warning=Downloading {filename} from {}", archive.url());
	archive.fetch(filename, dest)
}

fn env_flag(name: &str) -> bool {
	env::var(name).is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}

fn push_dll_candidates_from_path(candidates: &mut Vec<PathBuf>, path: PathBuf) {
	if path.is_dir() {
		candidates.push(path.join("pdfium.dll"));
	} else {
		candidates.push(path);
	}
}

fn find_pdfium_dll_in_path() -> Vec<PathBuf> {
	let Ok(path_var) = env::var("PATH") else {
		return Vec::new();
	};
	env::split_paths(&path_var).filter(|dir| !dir.as_os_str().is_empty()).map(|dir| dir.join("pdfium.dll")).collect()
}
