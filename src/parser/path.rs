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
	use super::*;

	#[test]
	fn extracts_title_from_path() {
		assert_eq!(extract_title_from_path("foo.txt"), "foo");
		assert_eq!(extract_title_from_path("/home/quin/books/worm.epub"), "worm");
		assert_eq!(extract_title_from_path("C:\\Users\\Quin\\Desktop\\file.log"), "file");
		assert_eq!(extract_title_from_path("/path/with/trailing/slash/"), "Untitled");
		assert_eq!(extract_title_from_path("C:\\path\\with\\trailing\\slash\\"), "Untitled");
		assert_eq!(extract_title_from_path("  spaced.txt  "), "spaced");
		assert_eq!(extract_title_from_path(""), "Untitled");
	}

	#[test]
	fn extracts_title_from_paths_without_extension() {
		assert_eq!(extract_title_from_path("README"), "README");
		assert_eq!(extract_title_from_path("/var/log/system"), "system");
	}

	#[test]
	fn returns_untitled_for_whitespace_only_or_trailing_separator() {
		assert_eq!(extract_title_from_path("   "), "Untitled");
		assert_eq!(extract_title_from_path(" /tmp/dir/ "), "Untitled");
	}

	#[test]
	fn extracts_title_from_multi_dot_filename() {
		assert_eq!(extract_title_from_path("archive.tar.gz"), "archive.tar");
	}
}
