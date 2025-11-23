use std::{
	env, fs,
	io::Cursor,
	path::{Path, PathBuf},
};

use bzip2::read::BzDecoder;
use cc::Build;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
	cxx_build::bridge("src/bridge.rs").flag_if_supported("-std=c++20").compile("paperback-bridge");
	println!("cargo:rerun-if-changed=src/bridge.rs");
	build_chmlib();
	setup_pdfium();
}

fn build_chmlib() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let chmlib_dir = out_dir.join("chmlib-0.40");
	let src_dir = chmlib_dir.join("src");
	if !chmlib_dir.exists() {
		download_and_extract_chmlib(&out_dir);
	}
	apply_patches(&src_dir);
	let mut build = Build::new();
	build.file(src_dir.join("chm_lib.c")).file(src_dir.join("lzx.c")).include(&src_dir).warnings(false);
	if cfg!(target_os = "windows") {
		build.define("WIN32", None);
		build.define("_WINDOWS", None);
	} else {
		build.define("CHMLIB_HAVE_STRINGS_H", None);
	}
	build.compile("chm");
	println!("cargo:rustc-link-lib=static=chm");
}

fn download_and_extract_chmlib(out_dir: &Path) {
	let url = "http://www.jedrea.com/chmlib/chmlib-0.40.tar.bz2";
	let response = ureq::get(url).call().expect("Failed to download chmlib");
	let buf = response.into_body().read_to_vec().expect("Failed to read chmlib tarball");
	let decompressor = BzDecoder::new(Cursor::new(buf));
	let mut archive = Archive::new(decompressor);
	archive.unpack(out_dir).expect("Failed to extract chmlib");
}

fn apply_patches(src_dir: &Path) {
	let chm_lib_path = src_dir.join("chm_lib.c");
	let mut contents = fs::read_to_string(&chm_lib_path).expect("Failed to read chm_lib.c");
	contents = contents.replace("/* yielding an error is preferable to yielding incorrect behavior */\n#error \"Please define the sized types for your platform in chm_lib.c\"", "typedef unsigned char           UChar;\ntypedef int16_t                 Int16;\ntypedef uint16_t                UInt16;\ntypedef int32_t                 Int32;\ntypedef uint32_t                UInt32;\ntypedef int64_t                 Int64;\ntypedef uint64_t                UInt64;");
	contents = contents
		.replace("#if __sun || __sgi\n#include <strings.h>", "#ifdef CHMLIB_HAVE_STRINGS_H\n#include <strings.h>");
	fs::write(&chm_lib_path, contents).expect("Failed to write patched chm_lib.c");
}

fn setup_pdfium() {
	let target_dir = get_target_dir();
	let platform = detect_pdfium_platform();
	let pdfium_root = target_dir.join("pdfium").join(&platform);
	let lib_dir = pdfium_root.join("lib");
	let lib_path = find_or_fetch_pdfium(&pdfium_root, &platform);
	println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
	let link_kind = if lib_path.extension().and_then(|e| e.to_str()) == Some("a") { "static" } else { "dylib" };
	println!("cargo:rustc-link-lib={}={}", link_kind, "pdfium");
	println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
	println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
}

fn get_target_dir() -> PathBuf {
	if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
		return PathBuf::from(dir);
	}
	let mut out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
	for _ in 0..4 {
		out_dir.pop();
	}
	out_dir
}

fn detect_pdfium_platform() -> String {
	let os = env::var("CARGO_CFG_TARGET_OS").expect("Missing target OS");
	let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Missing target arch");
	match (os.as_str(), arch.as_str()) {
		("windows", "x86_64") => "win-x64".into(),
		("windows", "x86") => "win-x86".into(),
		("windows", "aarch64") => "win-arm64".into(),
		("linux", "x86_64") => "linux-x64".into(),
		("linux", "x86") => "linux-x86".into(),
		("linux", "aarch64") => "linux-arm64".into(),
		("linux", "arm") => "linux-arm".into(),
		("macos", "x86_64") => "mac-x64".into(),
		("macos", "aarch64") => "mac-arm64".into(),
		_ => panic!("Unsupported target {os}-{arch} for pdfium"),
	}
}

fn find_or_fetch_pdfium(pdfium_root: &Path, platform: &str) -> PathBuf {
	let lib_dir = pdfium_root.join("lib");
	let candidates =
		[("pdfium.lib", "pdfium.dll.lib"), ("libpdfium.a", ""), ("libpdfium.dylib", ""), ("libpdfium.so", "")];
	for (final_name, original_name) in candidates {
		let candidate = lib_dir.join(final_name);
		if candidate.exists() {
			return candidate;
		}
		if !original_name.is_empty() {
			let original = lib_dir.join(original_name);
			if original.exists() {
				fs::rename(&original, &candidate).expect("Failed to rename pdfium import library");
				return candidate;
			}
		}
	}
	download_pdfium(pdfium_root, platform);
	find_or_fetch_pdfium(pdfium_root, platform)
}

fn download_pdfium(pdfium_root: &Path, platform: &str) {
	let url = format!("https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-{platform}.tgz");
	let response = ureq::get(&url).call().expect("Failed to download pdfium");
	let buf = response.into_body().read_to_vec().expect("Failed to read pdfium archive");
	let decompressor = GzDecoder::new(Cursor::new(buf));
	let mut archive = Archive::new(decompressor);
	if pdfium_root.exists() {
		fs::remove_dir_all(pdfium_root).expect("Failed to remove old pdfium directory");
	}
	fs::create_dir_all(pdfium_root).expect("Failed to create pdfium directory");
	archive.unpack(pdfium_root).expect("Failed to extract pdfium");
	if platform.starts_with("win-") {
		let dll_lib = pdfium_root.join("lib").join("pdfium.dll.lib");
		let target = pdfium_root.join("lib").join("pdfium.lib");
		if dll_lib.exists() && !target.exists() {
			fs::rename(&dll_lib, &target).expect("Failed to rename pdfium import library");
		}
	}
}
