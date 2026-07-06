use std::{
	env, fs, io,
	io::{Cursor, Read},
	path::{Path, PathBuf},
	process::Command,
};

use embed_manifest::{
	embed_manifest,
	manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*},
	new_manifest,
};
use flate2::read::GzDecoder;
use tar::Archive;
use winres::WindowsResource;

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

fn main() {
	track_packaging_inputs();
	build_translations();
	embed_wx_translations();
	copy_sounds();
	copy_pdfium_dll();
	build_docs();
	configure_installer();
	let target = env::var("TARGET").unwrap_or_default();
	embed_commit_hash();
	if target.contains("apple") {
		copy_pdfium_dylib();
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
		println!("cargo:rerun-if-changed=build.rs");
	}
}

fn get_commit_info() -> (String, bool) {
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
	let hash = match output {
		Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
		_ => "unknown".to_string(),
	};
	let is_dev = !Command::new("git")
		.args(["describe", "--tags", "--exact-match", "HEAD"])
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false);
	(hash, is_dev)
}

fn embed_commit_hash() {
	let (hash, is_dev) = get_commit_info();
	let short_hash = if hash == "unknown" { "unknown".to_string() } else { hash[..hash.len().min(7)].to_string() };
	println!("cargo:rustc-env=PAPERBACK_COMMIT_HASH={hash}");
	println!("cargo:rustc-env=PAPERBACK_SHORT_HASH={short_hash}");
	println!("cargo:rustc-env=PAPERBACK_IS_DEV={}", if is_dev { "1" } else { "0" });
}

fn embed_version_info() {
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let (hash, is_dev) = get_commit_info();
	let product_version = if is_dev {
		let short_hash = &hash[..hash.len().min(7)];
		format!("{version} ({short_hash})")
	} else {
		version.clone()
	};
	let mut res = WindowsResource::new();
	res.set("ProductName", "Paperback")
		.set("FileDescription", "Paperback")
		.set("LegalCopyright", "Copyright © 2025 Quin Gillespie")
		.set("CompanyName", "Quin Gillespie")
		.set("OriginalFilename", "paperback.exe")
		.set("ProductVersion", &product_version)
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
	let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();
	let po_dir = workspace_dir.join("po");
	let pot_file = po_dir.join("paperback.pot");
	println!("cargo:rerun-if-changed={}", workspace_dir.join("ios/Paperback").display());
	println!(
		"cargo:rerun-if-changed={}",
		workspace_dir.join("android/app/src/main/kotlin/dev/paperback/mobile").display()
	);
	if let Err(e) = patois_build::gen_pot(&workspace_dir, &po_dir, "paperback") {
		println!("cargo:warning=Failed to regenerate paperback.pot from Rust sources: {e}");
	}
	let ios_src = workspace_dir.join("ios/Paperback");
	if let Err(e) = patois_build::extend_pot_from_source_dirs(&[&ios_src], "swift", &pot_file) {
		println!("cargo:warning=Failed to extend paperback.pot from Swift sources: {e}");
	}
	let kt_src = workspace_dir.join("android/app/src/main/kotlin/dev/paperback/mobile");
	if let Err(e) = patois_build::extend_pot_from_source_dirs(&[&kt_src], "kt", &pot_file) {
		println!("cargo:warning=Failed to extend paperback.pot from Kotlin sources: {e}");
	}
	patois_build::compile_translations("../../po", "locale");
}

/// Embed wxWidgets' own standard message catalogs (`wxstd`) into the binary.
///
/// wxWidgets' build (driven by `wxdragon-sys`) emits its translation catalogs to
/// `<target>/<profile>/wxdragon_sys_cmake_build/share/locale/<lang>/LC_MESSAGES/wxstd-*.mo`.
/// We scan that tree and generate `wx_translations.rs` in `OUT_DIR` exposing
/// `wx_catalog(lang)` and `wx_available_languages()`, which `wx_translation_loader.rs`
/// includes. This mirrors how `patois::embed_domain!` bundles Paperback's own `.mo`
/// files. Because `paperback` depends on `wxdragon-sys` (via `wxdragon`), Cargo runs
/// that crate's build script first, so the catalogs exist by the time we look.
///
/// Only the languages Paperback itself ships (the `po/*.po` files) are embedded, so
/// the two translation sets stay in lockstep and we don't bloat the binary with wx
/// catalogs for languages the app isn't translated into.
fn embed_wx_translations() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
	let out_file = out_dir.join("wx_translations.rs");
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();
	let paperback_langs = paperback_po_languages(&workspace_dir.join("po"));
	let locale_dir = match target_profile_dir() {
		Some(dir) => dir.join("wxdragon_sys_cmake_build").join("share").join("locale"),
		None => {
			println!("cargo:warning=Could not determine target directory for wx translations.");
			write_empty_wx_translations(&out_file);
			return;
		}
	};
	println!("cargo:rerun-if-changed={}", locale_dir.display());
	// wxWidgets only produces this tree when gettext (msgfmt) is available at build
	// time. If it's missing (e.g. CI without gettext), degrade gracefully: emit an
	// empty catalog set so the crate still builds; wx UI strings just stay untranslated.
	let entries = match fs::read_dir(&locale_dir) {
		Ok(entries) => entries,
		Err(_) => {
			println!(
				"cargo:warning=wxWidgets locale directory not found at {}; wx UI strings will not be translated.",
				locale_dir.display()
			);
			write_empty_wx_translations(&out_file);
			return;
		}
	};
	// Collect (lang, mo_path), sorted by language code for deterministic output.
	let mut catalogs: Vec<(String, PathBuf)> = Vec::new();
	let mut lang_entries: Vec<_> = entries.flatten().collect();
	lang_entries.sort_by_key(|e| e.file_name());
	for entry in lang_entries {
		let lang = entry.file_name().to_string_lossy().to_string();
		// Restrict to languages Paperback ships. Match case-insensitively so wx's
		// `pt_BR`/`zh_CN` line up with Paperback's `pt_br`/`zh_CN` po stems.
		if !paperback_langs.contains(&lang.to_lowercase()) {
			continue;
		}
		let lc_messages = entry.path().join("LC_MESSAGES");
		let Ok(files) = fs::read_dir(&lc_messages) else { continue };
		// wxWidgets names the file `wxstd-<major>.<minor>.mo`; match any `wxstd*.mo`.
		let mut mo_files: Vec<PathBuf> = files
			.flatten()
			.map(|f| f.path())
			.filter(|p| {
				p.extension().and_then(|e| e.to_str()) == Some("mo")
					&& p.file_stem().and_then(|s| s.to_str()).is_some_and(|s| s.starts_with("wxstd"))
			})
			.collect();
		mo_files.sort();
		if let Some(mo_path) = mo_files.into_iter().next() {
			catalogs.push((lang, mo_path));
		}
	}
	if catalogs.is_empty() {
		println!("cargo:warning=No wxstd catalogs found under {}.", locale_dir.display());
	}
	let mut code = String::new();
	code.push_str("fn wx_catalog(lang: &str) -> Option<&'static [u8]> {\n    match lang {\n");
	for (lang, mo_path) in &catalogs {
		code.push_str(&format!("        {:?} => Some(include_bytes!({:?})),\n", lang, mo_path.display().to_string()));
	}
	code.push_str("        _ => None,\n    }\n}\n\n");
	code.push_str("fn wx_available_languages() -> &'static [&'static str] {\n    &[");
	for (lang, _) in &catalogs {
		code.push_str(&format!("{lang:?}, "));
	}
	code.push_str("]\n}\n");
	if let Err(e) = fs::write(&out_file, code) {
		println!("cargo:warning=Failed to write wx_translations.rs: {e}");
	}
}

fn write_empty_wx_translations(out_file: &Path) {
	let code = "fn wx_catalog(_lang: &str) -> Option<&'static [u8]> { None }\n\
		fn wx_available_languages() -> &'static [&'static str] { &[] }\n";
	if let Err(e) = fs::write(out_file, code) {
		println!("cargo:warning=Failed to write wx_translations.rs: {e}");
	}
}

/// Return the set of languages Paperback ships, as lowercased `po` file stems
/// (e.g. `de`, `pt_br`, `zh_cn`). Used to restrict which wx catalogs get embedded.
fn paperback_po_languages(po_dir: &Path) -> std::collections::HashSet<String> {
	println!("cargo:rerun-if-changed={}", po_dir.display());
	let mut langs = std::collections::HashSet::new();
	let Ok(entries) = fs::read_dir(po_dir) else {
		panic!("po directory not found at {}", po_dir.display());
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) != Some("po") {
			continue;
		}
		if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
			langs.insert(stem.to_lowercase());
		}
	}
	langs
}

fn copy_sounds() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();
	let sounds_src = workspace_dir.join("sounds");
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

fn copy_pdfium_dylib() {
	println!("cargo:rerun-if-env-changed=PAPERBACK_PDFIUM_DYLIB");
	println!("cargo:rerun-if-env-changed=PAPERBACK_SKIP_PDFIUM_DOWNLOAD");
	println!("cargo:rerun-if-env-changed=PAPERBACK_REFRESH_PDFIUM");
	let refresh = env::var("PAPERBACK_REFRESH_PDFIUM")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target output directory for libpdfium.dylib.");
			return;
		}
	};
	let dest = target_dir.join("libpdfium.dylib");
	if let Ok(path) = env::var("PAPERBACK_PDFIUM_DYLIB") {
		let src = PathBuf::from(path);
		if src.is_file() {
			println!("cargo:rerun-if-changed={}", src.display());
			if src != dest
				&& let Err(err) = fs::copy(&src, &dest)
			{
				println!("cargo:warning=Failed to copy libpdfium.dylib from {}: {}", src.display(), err);
			}
			return;
		}
	}
	if dest.exists() && !refresh {
		return;
	}
	if let Err(err) = ensure_pdfium_dylib(&dest) {
		println!(
			"cargo:warning=libpdfium.dylib not found. Automatic download failed: {}. Set PAPERBACK_PDFIUM_DYLIB or place libpdfium.dylib in the project root.",
			err
		);
	} else if dest.exists() {
		println!("cargo:rerun-if-changed={}", dest.display());
	}
}

fn ensure_pdfium_dylib(dest: &Path) -> io::Result<()> {
	let skip_download = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
	if skip_download {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	let refresh = env::var("PAPERBACK_REFRESH_PDFIUM")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
	if dest.exists() && !refresh {
		return Ok(());
	}
	let Some(url) = pdfium_dylib_download_url_for_target() else {
		return Err(io::Error::other("no PDFium URL configured for this macOS target architecture"));
	};
	download_pdfium_dylib(url, dest)
}

fn pdfium_dylib_download_url_for_target() -> Option<&'static str> {
	let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
	match arch.as_str() {
		"x86_64" => Some(PDFIUM_MAC_X64_URL),
		"aarch64" => Some(PDFIUM_MAC_ARM64_URL),
		_ => None,
	}
}

fn download_pdfium_dylib(url: &str, dest_dylib: &Path) -> io::Result<()> {
	if let Some(parent) = dest_dylib.parent() {
		fs::create_dir_all(parent)?;
	}
	println!("cargo:warning=Downloading libpdfium.dylib from {}", url);
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
		if path.file_name().and_then(|name| name.to_str()) == Some("libpdfium.dylib") {
			let temp_path = dest_dylib.with_extension("dylib.tmp");
			entry.unpack(&temp_path)?;
			if dest_dylib.exists() {
				fs::remove_file(dest_dylib)?;
			}
			fs::rename(temp_path, dest_dylib)?;
			return Ok(());
		}
	}
	Err(io::Error::other("libpdfium.dylib not found inside downloaded archive"))
}

fn copy_pdfium_dll() {
	let target = env::var("TARGET").unwrap_or_default();
	if !target.contains("windows") {
		return;
	}
	println!("cargo:rerun-if-env-changed=PDFIUM_DLL_PATH");
	println!("cargo:rerun-if-env-changed=PAPERBACK_PDFIUM_DLL");
	println!("cargo:rerun-if-env-changed=PAPERBACK_SKIP_PDFIUM_DOWNLOAD");
	println!("cargo:rerun-if-env-changed=PAPERBACK_REFRESH_PDFIUM");
	let refresh = env::var("PAPERBACK_REFRESH_PDFIUM")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => {
			println!("cargo:warning=Could not determine target output directory for pdfium.dll.");
			return;
		}
	};
	let dest = target_dir.join("pdfium.dll");
	let mut candidates = Vec::new();
	if let Ok(path) = env::var("PAPERBACK_PDFIUM_DLL") {
		push_dll_candidates_from_path(&mut candidates, PathBuf::from(path));
	}
	if let Ok(path) = env::var("PDFIUM_DLL_PATH") {
		push_dll_candidates_from_path(&mut candidates, PathBuf::from(path));
	}
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
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
			"cargo:warning=pdfium.dll not found. Automatic download failed: {}. Set PDFIUM_DLL_PATH (or PAPERBACK_PDFIUM_DLL), install pdfium.dll on PATH, or place it in the project root.",
			err
		);
	} else if dest.exists() {
		println!("cargo:rerun-if-changed={}", dest.display());
	}
}

fn ensure_pdfium_dll(dest_dll: &Path) -> io::Result<()> {
	let skip_download = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
	if skip_download {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	let refresh = env::var("PAPERBACK_REFRESH_PDFIUM")
		.map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
		.unwrap_or(false);
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
	if let Some(parent) = dest_dll.parent() {
		fs::create_dir_all(parent)?;
	}
	println!("cargo:warning=Downloading pdfium.dll from {}", url);
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
		if path.file_name().and_then(|name| name.to_str()) == Some("pdfium.dll") {
			let temp_path = dest_dll.with_extension("dll.tmp");
			entry.unpack(&temp_path)?;
			if dest_dll.exists() {
				fs::remove_file(dest_dll)?;
			}
			fs::rename(temp_path, dest_dll)?;
			return Ok(());
		}
	}
	Err(io::Error::other("pdfium.dll not found inside downloaded archive"))
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
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();
	let doc_dir = workspace_dir.join("doc");
	let readme = doc_dir.join("readme.md");
	let config = doc_dir.join("pandoc.yaml");
	println!("cargo:rerun-if-changed={}", readme.display());
	println!("cargo:rerun-if-changed={}", config.display());
	let mut embedded_langs: Vec<String> = Vec::new();
	let pandoc_available = Command::new("pandoc").arg("--version").output().is_ok();
	if !pandoc_available {
		println!("cargo:warning=Pandoc not found. Documentation will not be generated.");
	} else {
		// English readme: build to both target_dir (for macOS bundle) and OUT_DIR (for embedding)
		let target_output = target_dir.join("readme.html");
		let out_output = out_dir.join("readme.html");
		let status = Command::new("pandoc")
			.arg(format!("--defaults={}", config.display()))
			.arg(&readme)
			.arg("-o")
			.arg(&target_output)
			.status();
		match status {
			Ok(s) if s.success() => {
				let _ = fs::copy(&target_output, &out_output);
				embedded_langs.push("en".to_string());
			}
			_ => println!("cargo:warning=Failed to generate documentation."),
		}
		if let Ok(entries) = fs::read_dir(&doc_dir) {
			let mut doc_entries: Vec<_> = entries.flatten().collect();
			doc_entries.sort_by_key(|e| e.file_name());
			for entry in doc_entries {
				let path = entry.path();
				if path.extension().and_then(|e| e.to_str()) != Some("md") {
					continue;
				}
				let stem = match path.file_stem().and_then(|s| s.to_str()) {
					Some(s) => s.to_string(),
					None => continue,
				};
				let lang_code = match stem.strip_prefix("readme-") {
					Some(code) if !code.is_empty() => code.to_string(),
					_ => continue,
				};
				println!("cargo:rerun-if-changed={}", path.display());
				let lang_output = out_dir.join(format!("readme-{lang_code}.html"));
				let status = Command::new("pandoc")
					.arg(format!("--defaults={}", config.display()))
					.arg(&path)
					.arg("-o")
					.arg(&lang_output)
					.status();
				match status {
					Ok(s) if s.success() => embedded_langs.push(lang_code),
					_ => println!("cargo:warning=Failed to generate documentation for language: {lang_code}"),
				}
			}
		}
	}
	let mut code = String::from("pub fn readme_for_lang(lang: &str) -> Option<&'static [u8]> {\n    match lang {\n");
	for lang_code in &embedded_langs {
		let filename = if lang_code == "en" { "/readme.html".to_string() } else { format!("/readme-{lang_code}.html") };
		code.push_str(&format!(
			"        {:?} => Some(include_bytes!(concat!(env!(\"OUT_DIR\"), {:?}))),\n",
			lang_code, filename,
		));
	}
	code.push_str("        _ => None,\n    }\n}\n");
	let _ = fs::write(out_dir.join("lang_readmes.rs"), code);
}

fn configure_installer() {
	let target_dir = match target_profile_dir() {
		Some(dir) => dir,
		None => return,
	};
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
	let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();
	let input_path = workspace_dir.join("paperback.iss.in");
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
				<string>docm</string>
				<string>doc</string>
				<string>odt</string>
				<string>fodt</string>
				<string>pptx</string>
				<string>pptm</string>
				<string>ppt</string>
				<string>odp</string>
				<string>fodp</string>
				<string>chm</string>
				<string>fb2</string>
				<string>html</string>
				<string>htm</string>
				<string>xhtml</string>
				<string>md</string>
				<string>markdown</string>
				<string>mdx</string>
				<string>mdown</string>
				<string>mdwn</string>
				<string>mkd</string>
				<string>mkdn</string>
				<string>mkdown</string>
				<string>ronn</string>
				<string>mobi</string>
				<string>txt</string>
				<string>log</string>
				<string>rtf</string>
				<string>opf</string>
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
	let dylib_path = target_dir.join("libpdfium.dylib");
	if dylib_path.exists() {
		let _ = fs::copy(&dylib_path, macos_dir.join("libpdfium.dylib"));
	}
	let readme = target_dir.join("readme.html");
	if readme.exists() {
		let _ = fs::copy(&readme, bundle_dir.join("Resources/readme.html"));
	}
}
