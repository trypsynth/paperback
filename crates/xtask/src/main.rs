use std::{
	env,
	error::Error,
	path::{Path, PathBuf},
	process::Command,
};

mod android;
mod ios;
mod release;
mod translate;

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release::release()?,
		Some("android") => android::android()?,
		Some("ios") => ios::ios()?,
		Some("ios-release") => ios::ios_release()?,
		Some("gen-pot") => gen_pot()?,
		Some("translate") => translate::translate()?,
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
	println!("	  --build-aab        Build a release App Bundle (.aab) for Play Store upload");
	println!("	ios           Generate Swift bindings and build XCFramework for iOS");
	println!("	  --release          Build in release mode (default is debug)");
	println!("	ios-release   Archive and export a release IPA for App Store Connect");
	println!("	  --upload           Upload directly to App Store Connect via altool");
	println!("	translate     Regenerate the pot, sync po/*.po via msgmerge, and fill blank/fuzzy");
	println!("	              entries via the Claude API (needs ANTHROPIC_API_KEY)");
	println!("	  --dry-run          Report what would change; no API calls, no writes");
}

pub(crate) fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");
	// Step 1: generate from Rust crates tagged with translatable = true. patois-build
	// sanitizes the sources for `xgettext --language=C` itself, so the real source dirs go
	// straight in.
	let translatable_dirs = translatable_crate_src_dirs(&root)?;
	if translatable_dirs.is_empty() {
		return Err("no translatable crates found — check [package.metadata.patois] translatable = true".into());
	}
	let version = crate_version(&root, "paperback")?;
	patois_build::gen_pot_from_dirs(&translatable_dirs, &po_dir, "paperback", &version)?;
	// Steps 2 and 3: layer the mobile front-ends' own strings on top. Both directories are
	// required rather than skipped when absent: the Kotlin path spent a package rename
	// (dev.paperback.mobile -> dev.paperback.android) pointing at nothing, and because a
	// missing directory was silently fine, the only symptom was that Kotlin strings stopped
	// being extracted, which stayed invisible for as long as the pot carried the last good
	// copy of them forward.
	extend_pot_from(&pot_file, &root.join("ios/Paperback"), "swift")?;
	// The uniffi-generated bindings live in the sibling `kotlin/uniffi` tree and have no
	// translatable strings, so the scan starts below them.
	extend_pot_from(&pot_file, &root.join("android/app/src/main/kotlin/dev/paperback/android"), "kt")
}

/// Scan one mobile front-end's sources and add whatever they turn up to the pot, failing loudly
/// if the directory isn't where it's expected to be.
fn extend_pot_from(pot_file: &Path, src: &Path, extension: &str) -> Result<(), Box<dyn Error>> {
	if !src.is_dir() {
		return Err(format!("{extension} sources not found at {}", src.display()).into());
	}
	patois_build::extend_pot_from_source_dirs(&[src], extension, pot_file)?;
	Ok(())
}

/// Find the `src/` directory of every package in the build graph tagged
/// `[package.metadata.patois] translatable = true`: Paperback's own crates plus
/// dependencies like ship-shape and wx-utils, whose strings show up in Paperback's own
/// dialogs and therefore have to live in Paperback's catalog. Scanning only `crates/`
/// silently left every dependency string out of the pot, so anything a dependency added
/// after it was vendored in stayed untranslated no matter what the po files said.
fn translatable_crate_src_dirs(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let output = Command::new(&cargo).args(["metadata", "--format-version", "1"]).current_dir(root).output()?;
	if !output.status.success() {
		return Err("cargo metadata failed".into());
	}
	let meta: serde_json::Value = serde_json::from_slice(&output.stdout)?;
	let mut packages: Vec<&serde_json::Value> =
		meta["packages"].as_array().ok_or("cargo metadata: missing packages")?.iter().collect();
	// Order by name and version rather than by manifest path: a dependency's manifest lives
	// under CARGO_HOME, whose absolute spelling differs per machine (`.cargo` vs `scoop`, say)
	// and sorts differently from the workspace paths, which would reorder xgettext's input and
	// churn the pot for whoever regenerates it next.
	packages.sort_by_key(|pkg| {
		(pkg["name"].as_str().unwrap_or_default().to_string(), pkg["version"].as_str().unwrap_or_default().to_string())
	});
	let mut dirs = Vec::new();
	for pkg in packages {
		if pkg["metadata"]["patois"]["translatable"] != true {
			continue;
		}
		let manifest = pkg["manifest_path"].as_str().ok_or("cargo metadata: missing manifest_path")?;
		let src = Path::new(manifest).parent().unwrap().join("src");
		if src.is_dir() {
			dirs.push(src);
		}
	}
	Ok(dirs)
}

/// Resolve `package_name`'s version via `cargo metadata`, which correctly follows
/// `version.workspace = true` inheritance from the workspace root. Hand-parsing the crate's
/// own `Cargo.toml` instead would just find no literal `version = "..."` line and silently
/// produce a wrong default.
fn crate_version(root: &Path, package_name: &str) -> Result<String, Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let output =
		Command::new(&cargo).args(["metadata", "--format-version", "1", "--no-deps"]).current_dir(root).output()?;
	if !output.status.success() {
		return Err("cargo metadata failed".into());
	}
	let meta: serde_json::Value = serde_json::from_slice(&output.stdout)?;
	meta["packages"]
		.as_array()
		.and_then(|packages| packages.iter().find(|p| p["name"] == package_name))
		.and_then(|p| p["version"].as_str())
		.map(str::to_string)
		.ok_or_else(|| format!("cargo metadata: package {package_name} not found").into())
}
