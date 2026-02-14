use std::path::Path;

pub fn extract_title_from_path(path: &str) -> String {
	let trimmed = path.trim();
	if trimmed.is_empty() {
		return "Untitled".to_string();
	}
	if trimmed.ends_with('/') || trimmed.ends_with('\\') {
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
	#[case("C:\\Users\\Quin\\Desktop\\file.log", "file")]
	#[case("/path/with/trailing/slash/", "Untitled")]
	#[case("C:\\path\\with\\trailing\\slash\\", "Untitled")]
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
}
