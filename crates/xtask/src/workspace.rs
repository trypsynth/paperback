use std::{
	env,
	error::Error,
	path::{Path, PathBuf},
	process::Command,
};

pub(crate) fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

/// The whole build graph, not just `crates/`: dependencies like `ship-shape` show strings in Paperback's dialogs, and scanning only `crates/` silently left them untranslated.
pub(crate) fn translatable_crate_src_dirs(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let output = Command::new(&cargo).args(["metadata", "--format-version", "1"]).current_dir(root).output()?;
	if !output.status.success() {
		return Err("cargo metadata failed".into());
	}
	let meta: serde_json::Value = serde_json::from_slice(&output.stdout)?;
	let mut packages: Vec<&serde_json::Value> =
		meta["packages"].as_array().ok_or("cargo metadata: missing packages")?.iter().collect();
	// Not by manifest path: `CARGO_HOME` is spelled differently per machine, which would reorder xgettext's input and churn the pot.
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

pub(crate) use shipfitter::package::crate_version;

/// The core as the mobile apps link it: the FFI, and Open from URL.
pub const MOBILE_CORE_FEATURES: &str = "uniffi,fetch";

pub fn build_host_ffi_library(cargo: &str) -> Result<PathBuf, Box<dyn Error>> {
	let status = Command::new(cargo)
		.current_dir(project_root())
		// Debug, since uniffi reads the interface out of the library's symbols and release strips them.
		.args(["rustc", "-p", "paperback-core", "--features", MOBILE_CORE_FEATURES, "--lib", "--crate-type", "cdylib"])
		.status()?;
	if !status.success() {
		return Err("building paperback-core for the host failed".into());
	}
	let dir = project_root().join("target/debug");
	for name in ["paperback_core.dll", "libpaperback_core.dylib", "libpaperback_core.so"] {
		let candidate = dir.join(name);
		if candidate.exists() {
			return Ok(candidate);
		}
	}
	Err(format!("no host library for paperback-core under {}", dir.display()).into())
}
