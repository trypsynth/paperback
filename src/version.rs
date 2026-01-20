const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version_string() -> &'static str {
	VERSION
}

pub fn version_parts() -> (u64, u64, u64) {
	let mut parts = VERSION.split('.');
	let major = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
	let minor = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
	let patch = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
	(major, minor, patch)
}

pub fn user_agent() -> String {
	format!("paperback/{}", version_string())
}
