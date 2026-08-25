use std::{
	env,
	fmt::Write as _,
	fs, io,
	io::{Cursor, Read},
	path::{Path, PathBuf},
	process::Command,
};

use embed_manifest::{
	embed_manifest,
	manifest::{
		ActiveCodePage, DpiAwareness, HeapType, Setting,
		SupportedOS::{Windows7, Windows10},
	},
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
		let iconv_lib = format!("{homebrew_prefix}/opt/libiconv/lib");
		if Path::new(&iconv_lib).exists() {
			println!("cargo:rustc-link-search=native={iconv_lib}");
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
			println!("cargo:warning=Failed to embed manifest: {e}");
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
		.is_ok_and(|o| o.status.success());
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
	// Explorer, the taskbar, Alt+Tab, the Start menu, the uninstall entry and the document
	// types the installer registers (`paperback.exe,0`) all read the icon straight out of the
	// executable, so without this the app and every file associated with it show the generic
	// "no icon" placeholder. `ui::icon` handles the places that need a bitmap at runtime.
	res.set_icon("assets/paperback.ico");
	res.set("ProductName", "Paperback")
		.set("FileDescription", "Paperback")
		.set("LegalCopyright", "Copyright © 2025 Quin Gillespie")
		.set("CompanyName", "Quin Gillespie")
		.set("OriginalFilename", "paperback.exe")
		.set("ProductVersion", &product_version)
		.set("FileVersion", &version);
	if let Err(e) = res.compile() {
		println!("cargo:warning=Failed to embed version info: {e}");
	}
}

fn track_packaging_inputs() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=Cargo.lock");
	println!("cargo:rerun-if-changed=src");
	println!("cargo:rerun-if-changed=app");
	println!("cargo:rerun-if-changed=assets");
	println!("cargo:rerun-if-changed=paperback.iss.in");
}

// Only compiles the already-committed po/*.po files into .mo files for runtime loading.
// Regenerating paperback.pot itself is deliberately NOT done here: it used to run on every
// `cargo build`, fighting with `cargo xtask translate`/the auto-translate CI job (which
// regenerate it carefully, suppressing pure timestamp/wrapping churn) and touching the
// tracked .pot file mid-build. Run `cargo xtask translate` (or `--dry-run` to preview)
// to regenerate it instead.
fn build_translations() {
	patois_build::compile_translations("../../po", "locale");
}

fn copy_pdfium_dylib() {
	println!("cargo:rerun-if-env-changed=PAPERBACK_PDFIUM_DYLIB");
	println!("cargo:rerun-if-env-changed=PAPERBACK_SKIP_PDFIUM_DOWNLOAD");
	println!("cargo:rerun-if-env-changed=PAPERBACK_REFRESH_PDFIUM");
	let refresh =
		env::var("PAPERBACK_REFRESH_PDFIUM").is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target output directory for libpdfium.dylib.");
		return;
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
			"cargo:warning=libpdfium.dylib not found. Automatic download failed: {err}. Set PAPERBACK_PDFIUM_DYLIB or place libpdfium.dylib in the project root."
		);
	} else if dest.exists() {
		println!("cargo:rerun-if-changed={}", dest.display());
	}
}

fn ensure_pdfium_dylib(dest: &Path) -> io::Result<()> {
	let skip_download = env::var("PAPERBACK_SKIP_PDFIUM_DOWNLOAD")
		.is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
	if skip_download {
		return Err(io::Error::other("download disabled by PAPERBACK_SKIP_PDFIUM_DOWNLOAD"));
	}
	let refresh =
		env::var("PAPERBACK_REFRESH_PDFIUM").is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
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
	download_and_extract_from_tgz(url, dest_dylib, "libpdfium.dylib")
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

fn copy_pdfium_dll() {
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
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target directory for docs.");
		return;
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
	if pandoc_available {
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
			doc_entries.sort_by_key(std::fs::DirEntry::file_name);
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
	} else {
		println!("cargo:warning=Pandoc not found. Documentation will not be generated.");
	}
	let code = if embedded_langs.is_empty() {
		"pub fn readme_for_lang(_lang: &str) -> Option<&'static [u8]> {\n    None\n}\n".to_string()
	} else {
		let mut code =
			String::from("pub fn readme_for_lang(lang: &str) -> Option<&'static [u8]> {\n    match lang {\n");
		for lang_code in &embedded_langs {
			let filename =
				if lang_code == "en" { "/readme.html".to_string() } else { format!("/readme-{lang_code}.html") };
			let _ = writeln!(
				code,
				"        {lang_code:?} => Some(include_bytes!(concat!(env!(\"OUT_DIR\"), {filename:?}))),",
			);
		}
		code.push_str("        _ => None,\n    }\n}\n");
		code
	};
	let _ = fs::write(out_dir.join("lang_readmes.rs"), code);
}

/// Turns a format's display name into an Inno Setup task identifier, e.g.
/// "Flat `OpenDocument` Presentations" -> "`flat_opendocument_presentations`".
fn slugify(name: &str) -> String {
	let mut slug = String::new();
	let mut last_was_sep = true;
	for ch in name.chars() {
		if ch.is_ascii_alphanumeric() {
			slug.push(ch.to_ascii_lowercase());
			last_was_sep = false;
		} else if !last_was_sep {
			slug.push('_');
			last_was_sep = true;
		}
	}
	if slug.ends_with('_') {
		slug.pop();
	}
	slug
}

/// Builds the `[Tasks]` lines offering to associate each format from `paperback_formats::ALL`.
fn format_tasks_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL {
		let ext_list = format.extensions.iter().map(|ext| format!("*.{ext}")).collect::<Vec<_>>().join(", ");
		let flags = if format.installer.default_checked { String::new() } else { "; Flags: unchecked".to_string() };
		lines.push(format!(
			"\tName: \"assoc_{}\"; Description: \"Associate with {} ({})\"{}",
			slugify(format.name),
			format.name,
			ext_list,
			flags
		));
	}
	lines.join("\n")
}

/// Builds the `[Registry]` lines wiring each format's extensions to the `assoc_*` tasks above.
/// The shared `zip` extension is skipped here; it's handled by its own standalone task since
/// multiple parsers (DAISY, Word-in-zip) register it and it shouldn't be tied to just one checkbox.
fn format_registry_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL {
		let task = format!("assoc_{}", slugify(format.name));
		lines.push(format!("\t; {}.", format.name));
		for ext in format.extensions {
			if *ext == "zip" {
				continue;
			}
			if format.installer.default_handler {
				lines.push(format!(
					"\tRoot: HKCR; Subkey: \".{ext}\"; ValueType: string; ValueName: \"\"; ValueData: \"Paperback.Document\"; Flags: uninsdeletevalue; Tasks: {task}"
				));
			} else {
				lines.push(format!(
					"\tRoot: HKCR; Subkey: \".{ext}\\OpenWithProgids\"; ValueType: string; ValueName: \"Paperback.Document\"; ValueData: \"\"; Flags: uninsdeletevalue; Tasks: {task}"
				));
			}
		}
	}
	lines.join("\n")
}

fn configure_installer() {
	let Some(target_dir) = target_profile_dir() else {
		return;
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
			println!("cargo:warning=Failed to read installer script: {e}");
			return;
		}
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let (arch_allowed, arch_mode) = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
		Ok("aarch64") => ("arm64", "arm64"),
		_ => ("x64compatible", "x64compatible"),
	};
	let new_content = content
		.replace("@PROJECT_VERSION@", &version)
		.replace("@ARCH_ALLOWED@", arch_allowed)
		.replace("@ARCH_MODE@", arch_mode)
		.replace("@FORMAT_TASKS@", &format_tasks_block())
		.replace("@FORMAT_REGISTRY@", &format_registry_block());
	let output_path = target_dir.join("paperback.iss");
	if let Err(e) = fs::write(&output_path, new_content) {
		println!("cargo:warning=Failed to write installer script: {e}");
	}
}

/// Builds the `<string>ext</string>` lines for `CFBundleTypeExtensions`, deduped since a couple
/// of extensions (e.g. `zip`) are shared by more than one entry in `paperback_formats::ALL`.
fn bundle_document_extensions_block() -> String {
	let mut extensions = std::collections::BTreeSet::new();
	for format in paperback_formats::ALL {
		extensions.extend(format.extensions.iter().copied());
	}
	extensions.into_iter().map(|ext| format!("\t\t\t\t<string>{ext}</string>")).collect::<Vec<_>>().join("\n")
}

fn generate_app_bundle() {
	let Some(target_dir) = target_profile_dir() else {
		println!("cargo:warning=Could not determine target directory for macOS app bundle.");
		return;
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let document_extensions = bundle_document_extensions_block();
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
	<key>CFBundleIconFile</key>
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
{document_extensions}
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
		println!("cargo:warning=Failed to write Info.plist: {e}");
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
	// Named to match CFBundleIconFile above; without it the Dock, Finder and the app switcher
	// all fall back to the blank generic-application icon.
	let icns = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("assets/paperback.icns");
	if let Err(e) = fs::copy(&icns, bundle_dir.join("Resources/paperback.icns")) {
		println!("cargo:warning=Failed to copy the app bundle icon: {e}");
	}
}
