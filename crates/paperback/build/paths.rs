use std::{
	env,
	path::{Path, PathBuf},
};

pub fn track_packaging_inputs() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=build");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=assets");
	println!("cargo:rerun-if-changed={}", workspace_dir().join("paperback.iss.in").display());
}

pub fn manifest_dir() -> PathBuf {
	PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
}

pub fn workspace_dir() -> PathBuf {
	let manifest_dir = manifest_dir();
	manifest_dir.parent().and_then(Path::parent).expect("crate lives two levels below the workspace root").to_path_buf()
}

pub use shipfitter::build::target_profile_dir;
