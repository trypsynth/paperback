const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COMMIT_HASH: &str = env!("PAPERBACK_COMMIT_HASH");
const SHORT_HASH: &str = env!("PAPERBACK_SHORT_HASH");
const IS_DEV: &str = env!("PAPERBACK_IS_DEV");

#[must_use]
pub fn user_agent() -> String {
	format!("paperback/{VERSION}")
}

#[must_use]
pub fn display_version() -> String {
	if IS_DEV == "1" { format!("{VERSION} ({SHORT_HASH})") } else { VERSION.to_string() }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn user_agent_has_expected_prefix() {
		assert!(user_agent().starts_with("paperback/"));
	}

	#[test]
	fn user_agent_contains_package_version() {
		assert_eq!(user_agent(), format!("paperback/{}", env!("CARGO_PKG_VERSION")));
	}

	#[test]
	fn user_agent_has_no_surrounding_whitespace() {
		let ua = user_agent();
		assert_eq!(ua.trim(), ua);
	}
}
