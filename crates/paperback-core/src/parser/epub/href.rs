use crate::{
	parser::{is_external_url, util::path::resolve_relative_path},
	util::text::url_decode,
};

pub(super) fn resolve_href(current_path: &str, target: &str) -> String {
	if is_external_url(target) {
		return target.to_string();
	}
	if target.starts_with('#') {
		return target.to_string();
	}
	let (path_part, fragment) = split_href(target);
	let resolved = if path_part.is_empty() {
		current_path.to_string()
	} else {
		let current_dir = current_path.rfind('/').map_or("", |i| &current_path[..i]);
		resolve_relative_path(current_dir, &path_part)
	};
	if let Some(frag) = fragment {
		if frag.is_empty() { resolved } else { format!("{resolved}#{frag}") }
	} else {
		resolved
	}
}

pub(super) fn split_href(input: &str) -> (String, Option<String>) {
	let decoded = url_decode(input);
	let trimmed = decoded.strip_prefix("epub://").unwrap_or(&decoded);
	if let Some((path, frag)) = trimmed.split_once('#') {
		(path.to_string(), Some(frag.to_string()))
	} else {
		(trimmed.to_string(), None)
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case("OEBPS/chapter1.xhtml", "chapter2.xhtml", "OEBPS/chapter2.xhtml")]
	#[case("OEBPS/chapter1.xhtml", "chapter2.xhtml#section1", "OEBPS/chapter2.xhtml#section1")]
	#[case("OEBPS/chapter1.xhtml", "#section1", "#section1")]
	#[case("OEBPS/chapter1.xhtml", "../images/cover.png", "images/cover.png")]
	#[case("OEBPS/text/chapter1.xhtml", "http://example.com/page", "http://example.com/page")]
	#[case("OEBPS/text/chapter1.xhtml", "https://example.com/page", "https://example.com/page")]
	#[case("OEBPS/chapter1.xhtml", "chapter2.xhtml#", "OEBPS/chapter2.xhtml")]
	#[case("OEBPS/chapter1.xhtml", "chapter%201.xhtml", "OEBPS/chapter 1.xhtml")]
	fn resolves_href_against_current_path(#[case] current_path: &str, #[case] target: &str, #[case] expected: &str) {
		assert_eq!(resolve_href(current_path, target), expected);
	}

	#[rstest]
	#[case("chapter1.xhtml", ("chapter1.xhtml".to_string(), None))]
	#[case("chapter1.xhtml#section1", ("chapter1.xhtml".to_string(), Some("section1".to_string())))]
	#[case("chapter1.xhtml#", ("chapter1.xhtml".to_string(), Some(String::new())))]
	#[case("epub://OEBPS/chapter1.xhtml", ("OEBPS/chapter1.xhtml".to_string(), None))]
	#[case("chapter%201.xhtml", ("chapter 1.xhtml".to_string(), None))]
	#[case("#section1", (String::new(), Some("section1".to_string())))]
	fn splits_href_into_path_and_fragment(#[case] input: &str, #[case] expected: (String, Option<String>)) {
		assert_eq!(split_href(input), expected);
	}
}
