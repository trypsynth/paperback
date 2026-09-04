use std::path::Path;

/// Derives a fallback document title from the path of the file being opened.
///
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

/// Resolves a `/`-separated `path` against a `/`-separated `base_dir`, collapsing `.` and
/// `..` segments. An absolute `path` (leading `/`) ignores `base_dir` entirely. Used to
/// resolve hrefs against their containing document's location inside an archive (EPUB,
/// CHM), where paths are always archive-internal and `/`-separated regardless of host
/// platform — deliberately implemented with plain string splitting rather than
/// [`std::path::Path`], since `Path` treats `\` as a separator on Windows and would
/// otherwise resolve a literal backslash in an href differently depending on the platform
/// the code is compiled for. Case-folding and URL-decoding are the caller's job, since
/// those differ per format (CHM paths are case-insensitive, EPUB paths aren't).
#[must_use]
pub fn resolve_relative_path(base_dir: &str, path: &str) -> String {
	let mut parts: Vec<&str> =
		if path.starts_with('/') { Vec::new() } else { base_dir.split('/').filter(|s| !s.is_empty()).collect() };
	for part in path.split('/') {
		match part {
			".." => {
				parts.pop();
			}
			"." | "" => {}
			p => parts.push(p),
		}
	}
	parts.join("/")
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

	#[rstest]
	#[case("OEBPS", "chapter1.xhtml", "OEBPS/chapter1.xhtml")]
	#[case("OEBPS/text", "../images/cover.png", "OEBPS/images/cover.png")]
	#[case("OEBPS", "/OEBPS/toc.ncx", "OEBPS/toc.ncx")]
	#[case("OEBPS/text", "./chapter2.xhtml", "OEBPS/text/chapter2.xhtml")]
	#[case("", "chapter1.xhtml", "chapter1.xhtml")]
	#[case("OEBPS/deep/nested", "../../chapter1.xhtml", "OEBPS/chapter1.xhtml")]
	#[case("OEBPS", "../../../escape.xhtml", "escape.xhtml")]
	fn resolves_relative_paths(#[case] base_dir: &str, #[case] path: &str, #[case] expected: &str) {
		assert_eq!(resolve_relative_path(base_dir, path), expected);
	}
}
