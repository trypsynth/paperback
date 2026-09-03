//! Getting the prebuilt `PDFium` library next to the executable: honour whatever the environment
//! points at, otherwise download the release build for the target and unpack it.

use std::{
	env, fs,
	io::{self, Cursor, Read},
	path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::paths::{self, target_profile_dir};

const PDFIUM_WIN_X64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x64.tgz";
const PDFIUM_WIN_X86_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x86.tgz";
const PDFIUM_WIN_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-arm64.tgz";
const PDFIUM_MAC_X64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-mac-x64.tgz";
const PDFIUM_MAC_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-mac-arm64.tgz";
const PDFIUM_LINUX_X64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz";
const PDFIUM_LINUX_ARM64_URL: &str =
	"https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-arm64.tgz";

pub fn copy_so() {
	copy_prebuilt_lib("libpdfium.so", "PAPERBACK_PDFIUM_SO", pdfium_so_download_url_for_target);
}

pub fn copy_dylib() {
	copy_prebuilt_lib("libpdfium.dylib", "PAPERBACK_PDFIUM_DYLIB", pdfium_dylib_download_url_for_target);
}

fn copy_prebuilt_lib(filename: &str, env_var: &str, url_for_target: fn() -> Option<&'static str>) {
	println!("cargo:rerun-if-env-changed={env_var}");
	println!("cargo:rerun-if-env-changed=PAPERBACK_SKIP_PDFIUM_DOWNLOAD");
	println!("cargo:rerun-if-env-changed=PAPERBACK_REFRESH_PDFIUM");
	let refresh = pdfium_refresh_requested();
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target output directory for {filename}.");
		return;
	};
	let dest = target_dir.join(filename);
	if let Ok(path) = env::var(env_var) {
		let src = PathBuf::from(path);
		if src.is_file() {
			println!("cargo:rerun-if-changed={}", src.display());
			if src != dest
				&& let Err(err) = fs::copy(&src, &dest)
			{
				println!("cargo:warning=Failed to copy {filename} from {}: {}", src.display(), err);
			}
			return;
		}
	}
	if dest.exists() && !refresh {
		return;
	}
	if let Err(err) = ensure_prebuilt_lib(&dest, filename, url_for_target) {
		println!(
			"cargo:warning={filename} not found. Automatic download failed: {err}. Set {env_var} or place {filename} in the project root."
		);
	} else if dest.exists() {
		println!("cargo:rerun-if-changed={}", dest.display());
	}
}

fn ensure_prebuilt_lib(dest: &Path, filename: &str, url_for_target: fn() -> Option<&'static str>) -> io::Result<()> {
	let skip_download = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD")
		.is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	if skip_download {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	if dest.exists() && !pdfium_refresh_requested() {
		return Ok(());
	}
	let Some(url) = url_for_target() else {
		return Err(io::Error::other("no PDFium URL configured for this target architecture"));
	};
	download_and_extract_from_tgz(url, dest, filename)
}

fn pdfium_refresh_requested() -> bool {
	env::var("PAPERBACK_REFRESH_PDFIUM").is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}

fn pdfium_so_download_url_for_target() -> Option<&'static str> {
	let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
	match arch.as_str() {
		"x86_64" => Some(PDFIUM_LINUX_X64_URL),
		"aarch64" => Some(PDFIUM_LINUX_ARM64_URL),
		_ => None,
	}
}

fn pdfium_dylib_download_url_for_target() -> Option<&'static str> {
	let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
	match arch.as_str() {
		"x86_64" => Some(PDFIUM_MAC_X64_URL),
		"aarch64" => Some(PDFIUM_MAC_ARM64_URL),
		_ => None,
	}
}

/// Downloads a `.tgz` archive from `url` and unpacks the single entry named
/// `wanted_filename` to `dest`, replacing anything already there. Used to fetch the
/// prebuilt pdfium binary for whichever platform/architecture is currently building.
fn download_and_extract_from_tgz(url: &str, dest: &Path, wanted_filename: &str) -> io::Result<()> {
	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}
	println!("cargo:warning=Downloading {wanted_filename} from {url}");
	let response = ureq::get(url).call().map_err(|err| io::Error::other(format!("request failed: {err}")))?;
	let mut body = response.into_body();
	let mut archive_bytes = Vec::new();
	body.as_reader()
		.read_to_end(&mut archive_bytes)
		.map_err(|err| io::Error::other(format!("failed to read response body: {err}")))?;
	let decoder = GzDecoder::new(Cursor::new(archive_bytes));
	let mut archive = Archive::new(decoder);
	for entry in archive.entries()? {
		let mut entry = entry?;
		let path = entry.path()?;
		if path.file_name().and_then(|name| name.to_str()) == Some(wanted_filename) {
			let temp_ext = match dest.extension() {
				Some(ext) => format!("{}.tmp", ext.to_string_lossy()),
				None => "tmp".to_string(),
			};
			let temp_path = dest.with_extension(temp_ext);
			entry.unpack(&temp_path)?;
			if dest.exists() {
				fs::remove_file(dest)?;
			}
			fs::rename(temp_path, dest)?;
			return Ok(());
		}
	}
	Err(io::Error::other(format!("{wanted_filename} not found inside downloaded archive")))
}

pub fn copy_dll() {
	let target = env::var("TARGET").unwrap_or_default();
	if !target.contains("windows") {
		return;
	}
	println!("cargo:rerun-if-env-changed=PDFIUM_DLL_PATH");
	println!("cargo:rerun-if-env-changed=PAPERBACK_PDFIUM_DLL");
	println!("cargo:rerun-if-env-changed=PAPERBACK_SKIP_PDFIUM_DOWNLOAD");
	println!("cargo:rerun-if-env-changed=PAPERBACK_REFRESH_PDFIUM");
	let refresh =
		env::var("PAPERBACK_REFRESH_PDFIUM").is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target output directory for pdfium.dll.");
		return;
	};
	let dest = target_dir.join("pdfium.dll");
	let mut candidates = Vec::new();
	if let Ok(path) = env::var("PAPERBACK_PDFIUM_DLL") {
		push_dll_candidates_from_path(&mut candidates, PathBuf::from(path));
	}
	if let Ok(path) = env::var("PDFIUM_DLL_PATH") {
		push_dll_candidates_from_path(&mut candidates, PathBuf::from(path));
	}
	let manifest_dir = paths::manifest_dir();
	candidates.push(manifest_dir.join("pdfium.dll"));
	candidates.push(manifest_dir.join("bin").join("pdfium.dll"));
	candidates.extend(find_pdfium_dll_in_path());
	if let Some(source) = candidates.into_iter().find(|path| path.is_file()) {
		println!("cargo:rerun-if-changed={}", source.display());
		if source != dest {
			if let Err(err) = fs::copy(&source, &dest) {
				println!("cargo:warning=Failed to copy pdfium.dll from {}: {}", source.display(), err);
			}
			return;
		}
		if !refresh {
			return;
		}
	}
	if let Err(err) = ensure_pdfium_dll(&dest) {
		println!(
			"cargo:warning=pdfium.dll not found. Automatic download failed: {err}. Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL), install pdfium.dll on PATH, or place it in the project root."
		);
	} else if dest.exists() {
		println!("cargo:rerun-if-changed={}", dest.display());
	}
}

fn ensure_pdfium_dll(dest_dll: &Path) -> io::Result<()> {
	let skip_download = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD")
		.is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	if skip_download {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	let refresh =
		env::var("PAPERBACK_REFRESH_PDFIUM").is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	if dest_dll.exists() && !refresh {
		return Ok(());
	}
	let Some(url) = pdfium_download_url_for_target() else {
		return Err(io::Error::other("no PDFium URL configured for this target architecture"));
	};
	download_pdfium_dll(url, dest_dll)
}

fn pdfium_download_url_for_target() -> Option<&'static str> {
	let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
	match arch.as_str() {
		"x86_64" => Some(PDFIUM_WIN_X64_URL),
		"x86" => Some(PDFIUM_WIN_X86_URL),
		"aarch64" => Some(PDFIUM_WIN_ARM64_URL),
		_ => None,
	}
}

fn download_pdfium_dll(url: &str, dest_dll: &Path) -> io::Result<()> {
	download_and_extract_from_tgz(url, dest_dll, "pdfium.dll")
}

fn push_dll_candidates_from_path(candidates: &mut Vec<PathBuf>, path: PathBuf) {
	if path.is_dir() {
		candidates.push(path.join("pdfium.dll"));
	} else {
		candidates.push(path);
	}
}

fn find_pdfium_dll_in_path() -> Vec<PathBuf> {
	let mut candidates = Vec::new();
	let Ok(path_var) = env::var("PATH") else {
		return candidates;
	};
	for dir in env::split_paths(&path_var) {
		if dir.as_os_str().is_empty() {
			continue;
		}
		candidates.push(dir.join("pdfium.dll"));
	}
	candidates
}
