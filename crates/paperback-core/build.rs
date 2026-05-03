use std::process::Command;

fn main() {
	let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
	let hash = match output {
		Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
		_ => "unknown".to_string(),
	};
	println!("cargo:rustc-env=PAPERBACK_COMMIT_HASH={hash}");
	println!("cargo:rerun-if-changed=.git/HEAD");
}
