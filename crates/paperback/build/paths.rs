//! Locating the directories the other build steps read from and write to, plus the
//! `rerun-if-changed` list that decides when Cargo runs this script again.

use std::{
	env,
	path::{Path, PathBuf},
};

pub fn track_packaging_inputs() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=build");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=Cargo.lock");
	println!("cargo:rerun-if-changed=src");
	println!("cargo:rerun-if-changed=app");
	println!("cargo:rerun-if-changed=assets");
	println!("cargo:rerun-if-changed=paperback.iss.in");
}

/// The `crates/paperback` directory this build script lives in.
pub fn manifest_dir() -> PathBuf {
	PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
}

/// The workspace root, two levels above the crate.
pub fn workspace_dir() -> PathBuf {
	let manifest_dir = manifest_dir();
	manifest_dir.parent().and_then(Path::parent).expect("crate lives two levels below the workspace root").to_path_buf()
}

pub fn target_profile_dir() -> Option<PathBuf> {
	let profile = env::var("PROFILE").ok()?;
	if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
		let mut dir = PathBuf::from(target_dir);
		dir.push(profile);
		return Some(dir);
	}
	let out_dir = PathBuf::from(env::var("OUT_DIR").ok()?);
	out_dir.ancestors().nth(3).map(Path::to_path_buf)
}
