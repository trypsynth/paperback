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
