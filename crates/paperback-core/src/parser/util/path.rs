use std::path::Path;

/// Derives a fallback document title from the path of the file being opened.
/// Inputs are always native local paths, so separators are interpreted
/// per-platform: on unix, a backslash is an ordinary filename character.
#[must_use]
pub fn extract_title_from_path(path: &str) -> String {
	let trimmed = path.trim();
	if trimmed.is_empty() {
		return "Untitled".to_string();
	}
	if trimmed.ends_with('/') || (cfg!(windows) && trimmed.ends_with('\\')) {
		return "Untitled".to_string();
	}
	Path::new(trimmed).file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case("foo.txt", "foo")]
	#[case("/home/quin/books/worm.epub", "worm")]
	#[case("/path/with/trailing/slash/", "Untitled")]
	#[case("  spaced.txt  ", "spaced")]
	#[case("", "Untitled")]
	#[case("README", "README")]
	#[case("/var/log/system", "system")]
	#[case("   ", "Untitled")]
	#[case(" /tmp/dir/ ", "Untitled")]
	#[case("archive.tar.gz", "archive.tar")]
	fn extracts_title_from_path(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(extract_title_from_path(input), expected);
	}

	#[cfg(windows)]
	#[rstest]
	#[case("C:\\Users\\Quin\\Desktop\\file.log", "file")]
	#[case("C:\\path\\with\\trailing\\slash\\", "Untitled")]
	fn extracts_title_from_windows_path(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(extract_title_from_path(input), expected);
	}

	/// On unix, a backslash is an ordinary filename character, not a separator.
	#[cfg(not(windows))]
	#[rstest]
	#[case("weird\\name.txt", "weird\\name")]
	#[case("trailing\\", "trailing\\")]
	fn backslash_is_a_filename_character_on_unix(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(extract_title_from_path(input), expected);
	}
}
