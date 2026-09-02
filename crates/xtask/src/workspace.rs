//! Finding things in the workspace: its root, the crates whose strings are translatable, and a
//! package's version.

use std::{
	env,
	error::Error,
	path::{Path, PathBuf},
	process::Command,
};

pub(crate) fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

/// Find the `src/` directory of every package in the build graph tagged
/// `[package.metadata.patois] translatable = true`: Paperback's own crates plus
/// dependencies like ship-shape and wx-utils, whose strings show up in Paperback's own
/// dialogs and therefore have to live in Paperback's catalog. Scanning only `crates/`
/// silently left every dependency string out of the pot, so anything a dependency added
/// after it was vendored in stayed untranslated no matter what the po files said.
pub(crate) fn translatable_crate_src_dirs(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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
pub(crate) fn crate_version(root: &Path, package_name: &str) -> Result<String, Box<dyn Error>> {
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
