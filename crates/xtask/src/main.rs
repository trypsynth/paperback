use std::{
	env,
	error::Error,
	fs,
	path::{Path, PathBuf},
};

mod android;
mod ios;
mod release;
mod sanitize_rust;

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release::release()?,
		Some("android") => android::android()?,
		Some("ios") => ios::ios()?,
		Some("ios-release") => ios::ios_release()?,
		Some("gen-pot") => gen_pot()?,
		_ => print_help(),
	}
	Ok(())
}

pub(crate) fn print_help() {
	println!("Tasks:");
	println!("	release       Build release binaries and package them");
	println!("	gen-pot       Regenerate po/paperback.pot from all translatable crates");
	println!("	android       Generate Kotlin bindings and build native Android libraries");
	println!("	  --release          Build APK using gradlew assembleRelease");
	println!("	  --debug            Build APK using gradlew assembleDebug");
	println!("	  --install-release  Install release APK using gradlew installRelease");
	println!("	  --install-debug    Install debug APK using gradlew installDebug");
	println!("	ios           Generate Swift bindings and build XCFramework for iOS");
	println!("	  --release          Build in release mode (default is debug)");
	println!("	ios-release   Archive and export a release IPA for App Store Connect");
	println!("	  --upload           Upload directly to App Store Connect via altool");
}

pub(crate) fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");

	// Step 1: generate from Rust crates tagged with translatable = true. `xgettext
	// --language=C` doesn't understand Rust lifetimes (`'a`) or raw strings (`r#"..."#`) and
	// runs on past them as "unterminated" literals, sometimes splicing unrelated strings
	// together. Feed it sanitized copies of the source instead of the real files so those
	// constructs can't confuse it; see sanitize_rust for why comments/strings stay untouched.
	let translatable_dirs = translatable_crate_src_dirs(&root);
	if translatable_dirs.is_empty() {
		return Err("no translatable crates found — check [package.metadata.patois] translatable = true".into());
	}
	let sanitized_root = root.join("target/gen-pot-sanitized");
	let _ = fs::remove_dir_all(&sanitized_root);
	let mut sanitized_dirs = Vec::new();
	for src_dir in &translatable_dirs {
		let crate_name = src_dir.parent().and_then(|p| p.file_name()).unwrap().to_string_lossy().into_owned();
		let dest = sanitized_root.join(crate_name).join("src");
		sanitize_dir_into(src_dir, &dest)?;
		sanitized_dirs.push(dest);
	}
	let version = crate_version(&root.join("crates/paperback/Cargo.toml"));
	let gen_result = patois_build::gen_pot_from_dirs(&sanitized_dirs, &po_dir, "paperback", &version);
	let _ = fs::remove_dir_all(&sanitized_root);
	gen_result?;

	// Step 2: extend with iOS Swift sources (t() calls in Swift files)
	let ios_src = root.join("ios/Paperback");
	if ios_src.is_dir() {
		patois_build::extend_pot_from_source_dirs(&[&ios_src], "swift", &pot_file)?;
	}

	// Step 3: extend with Android Kotlin sources (excluding uniffi-generated bindings)
	let kt_src = root.join("android/app/src/main/kotlin/dev/paperback/mobile");
	if kt_src.is_dir() {
		patois_build::extend_pot_from_source_dirs(&[&kt_src], "kt", &pot_file)?;
	}

	Ok(())
}

/// Find `src/` directories of every crate under `crates/` tagged
/// `[package.metadata.patois] translatable = true`.
fn translatable_crate_src_dirs(root: &Path) -> Vec<PathBuf> {
	let crates_dir = root.join("crates");
	let mut dirs = Vec::new();
	let Ok(entries) = fs::read_dir(&crates_dir) else {
		return dirs;
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if !path.is_dir() {
			continue;
		}
		let manifest = path.join("Cargo.toml");
		let Ok(content) = fs::read_to_string(&manifest) else {
			continue;
		};
		if content.contains("[package.metadata.patois]") && content.contains("translatable = true") {
			dirs.push(path.join("src"));
		}
	}
	dirs.sort();
	dirs
}

/// Read the `version` field from a crate manifest's `[package]` section.
fn crate_version(manifest_path: &Path) -> String {
	let Ok(content) = fs::read_to_string(manifest_path) else {
		return "0.0.0".to_string();
	};
	let mut in_package = false;
	for line in content.lines() {
		let trimmed = line.trim();
		if trimmed.starts_with('[') {
			in_package = trimmed == "[package]";
			continue;
		}
		if in_package
			&& let Some(rest) = trimmed.strip_prefix("version")
			&& let Some(value) = rest.trim_start().strip_prefix('=')
		{
			return value.trim().trim_matches('"').to_string();
		}
	}
	"0.0.0".to_string()
}

/// Copy every `.rs` file under `src` into the same relative layout under `dest`, sanitizing
/// each one for `xgettext --language=C` on the way (see sanitize_rust).
fn sanitize_dir_into(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
	for entry in walkdir::WalkDir::new(src) {
		let entry = entry?;
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) != Some("rs") {
			continue;
		}
		let rel = path.strip_prefix(src)?;
		let out_path = dest.join(rel);
		if let Some(parent) = out_path.parent() {
			fs::create_dir_all(parent)?;
		}
		let content = fs::read_to_string(path)?;
		fs::write(&out_path, sanitize_rust::sanitize_for_xgettext(&content))?;
	}
	Ok(())
}
