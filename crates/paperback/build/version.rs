//! The commit hash and dev/release flag baked into the binary for the about dialog.

use std::process::Command;

pub fn get_commit_info() -> (String, bool) {
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

pub fn embed_commit_hash() {
	let (hash, is_dev) = get_commit_info();
	let short_hash = if hash == "unknown" { "unknown".to_string() } else { hash[..hash.len().min(7)].to_string() };
	println!("cargo:rustc-env=PAPERBACK_COMMIT_HASH={hash}");
	println!("cargo:rustc-env=PAPERBACK_SHORT_HASH={short_hash}");
	println!("cargo:rustc-env=PAPERBACK_IS_DEV={}", if is_dev { "1" } else { "0" });
}
