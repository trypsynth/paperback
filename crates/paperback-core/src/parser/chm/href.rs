use crate::parser::{is_external_url, util::path::resolve_relative_path};

pub(super) fn normalize_path(path: &str) -> String {
	let mut result = path.replace('\\', "/").to_lowercase();
	if !result.starts_with('/') {
		result.insert(0, '/');
	}
	result
}

pub(super) fn resolve_chm_href(current_file: &str, href: &str) -> String {
	if is_external_url(href) {
		return href.to_string();
	}
	let (path_part, fragment) = href.split_once('#').map_or((href, None), |(p, f)| (p, Some(f)));
	let resolved_path = if path_part.is_empty() {
		normalize_path(current_file)
	} else {
		let current_normalized = normalize_path(current_file);
		let current_dir = current_normalized.rfind('/').map_or("", |i| &current_normalized[..i]);
		let path_normalized = path_part.replace('\\', "/");
		format!("/{}", resolve_relative_path(current_dir, &path_normalized)).to_lowercase()
	};
	match fragment {
		Some(frag) if !frag.is_empty() => format!("{resolved_path}#{frag}"),
		_ => resolved_path,
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case("Folder\\Page.htm", "/folder/page.htm")]
	#[case("/Folder/Page.htm", "/folder/page.htm")]
	#[case("page.htm", "/page.htm")]
	#[case("PAGE.HTM", "/page.htm")]
	fn normalizes_paths(#[case] input: &str, #[case] expected: &str) {
		assert_eq!(normalize_path(input), expected);
	}

	#[rstest]
	#[case("/folder/page.htm", "other.htm", "/folder/other.htm")]
	#[case("/folder/page.htm", "../other.htm", "/other.htm")]
	#[case("/folder/page.htm", "#section1", "/folder/page.htm#section1")]
	#[case("/folder/page.htm", "other.htm#section1", "/folder/other.htm#section1")]
	#[case("/folder/page.htm", "http://example.com/page", "http://example.com/page")]
	#[case("/folder/page.htm", "https://example.com/page", "https://example.com/page")]
	#[case("/folder/sub\\page.htm", "other.htm", "/folder/sub/other.htm")]
	#[case("/folder/PAGE.HTM", "Other.HTM", "/folder/other.htm")]
	fn resolves_chm_hrefs(#[case] current_file: &str, #[case] href: &str, #[case] expected: &str) {
		assert_eq!(resolve_chm_href(current_file, href), expected);
	}
}
