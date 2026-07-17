use std::process::Command;

fn main() {
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
	let hash = match output {
		Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
		_ => "unknown".to_string(),
	};
	let short_hash = if hash == "unknown" { "unknown".to_string() } else { hash[..hash.len().min(7)].to_string() };
	let is_dev = !Command::new("git")
		.args(["describe", "--tags", "--exact-match", "HEAD"])
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false);
	println!("cargo:rustc-env=PAPERBACK_COMMIT_HASH={hash}");
	println!("cargo:rustc-env=PAPERBACK_SHORT_HASH={short_hash}");
	println!("cargo:rustc-env=PAPERBACK_IS_DEV={}", if is_dev { "1" } else { "0" });
	println!("cargo:rerun-if-changed=.git/HEAD");

	#[cfg(feature = "uniffi")]
	uniffi::generate_scaffolding("src/paperback.udl").expect("Building the UDL file failed");
}
