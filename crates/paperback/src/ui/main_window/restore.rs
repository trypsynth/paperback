//! Reopening the documents that were open when the app last exited, and forgetting the ones
//! that can no longer be reopened.

/// Runs `open` on every stored path in order and returns the ones it could not reopen.
pub(super) fn restore_each(paths: Vec<String>, mut open: impl FnMut(&str) -> bool) -> Vec<String> {
	paths.into_iter().filter(|path| !open(path)).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn restore_each_returns_the_paths_that_failed() {
		let mut seen = Vec::new();
		let failed = restore_each(vec!["ok".to_string(), "bad".to_string(), "ok2".to_string()], |path| {
			seen.push(path.to_string());
			path != "bad"
		});
		assert_eq!(failed, vec!["bad"]);
		assert_eq!(seen, vec!["ok", "bad", "ok2"]);
	}

	#[test]
	fn restore_each_returns_empty_when_all_succeed() {
		let failed = restore_each(vec!["a".to_string(), "b".to_string()], |_| true);
		assert!(failed.is_empty());
	}
}
