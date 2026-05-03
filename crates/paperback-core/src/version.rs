const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COMMIT_HASH: &str = env!("PAPERBACK_COMMIT_HASH");

pub fn user_agent() -> String {
	format!("paperback/{VERSION}")
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
