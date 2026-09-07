use rstest::rstest;

use super::*;

#[test]
fn parses_manifest_spine_and_metadata() {
	let opf = r#"<?xml version="1.0"?>
	<package>
		<metadata>
			<title>My Book</title>
			<creator>Some Author</creator>
		</metadata>
		<manifest>
			<item id="dtbook" href="book.xml" media-type="application/x-dtbook+xml"/>
			<item id="smil1" href="smil/section1.smil" media-type="application/smil"/>
		</manifest>
		<spine>
			<itemref idref="smil1"/>
		</spine>
	</package>"#;
	let package = parse_opf_package(opf, "OEBPS").expect("valid opf");
	assert_eq!(package.title, Some("My Book".to_string()));
	assert_eq!(package.author, Some("Some Author".to_string()));
	assert_eq!(package.spine, vec!["smil1".to_string()]);
	assert_eq!(package.item("dtbook").unwrap().href, "OEBPS/book.xml");
	assert_eq!(package.item("smil1").unwrap().media_type, "application/smil");
	assert!(package.item("missing").is_none());
}

#[rstest]
#[case("Title", "Creator")]
#[case("title", "creator")]
fn accepts_both_capitalized_and_lowercase_metadata_tags(#[case] title_tag: &str, #[case] creator_tag: &str) {
	let opf = format!(
		"<package><metadata><{title_tag}>T</{title_tag}><{creator_tag}>A</{creator_tag}></metadata><manifest/><spine/></package>"
	);
	let package = parse_opf_package(&opf, "").expect("valid opf");
	assert_eq!(package.title, Some("T".to_string()));
	assert_eq!(package.author, Some("A".to_string()));
}

#[test]
fn falls_back_to_nested_metadata_elements_when_no_direct_child_matches() {
	// Some DAISY 2 books wrap title/creator inside an x-metadata element rather than as a
	// direct child of <metadata>; parse_opf_package should still find them via descendants().
	let opf = r"<package><metadata><x-metadata><Title>Nested Title</Title></x-metadata></metadata><manifest/><spine/></package>";
	let package = parse_opf_package(opf, "").expect("valid opf");
	assert_eq!(package.title, Some("Nested Title".to_string()));
}

#[test]
fn direct_child_metadata_takes_priority_over_nested() {
	let opf = r"<package><metadata><Title>Direct</Title><x-metadata><Title>Nested</Title></x-metadata></metadata><manifest/><spine/></package>";
	let package = parse_opf_package(opf, "").expect("valid opf");
	assert_eq!(package.title, Some("Direct".to_string()));
}

#[test]
fn manifest_items_without_href_are_skipped() {
	let opf = r#"<package><manifest><item id="bad" media-type="text/xml"/></manifest><spine/></package>"#;
	let package = parse_opf_package(opf, "").expect("valid opf");
	assert!(package.item("bad").is_none());
}

#[test]
fn missing_package_element_yields_an_empty_package() {
	let package = parse_opf_package("<not-a-package/>", "").expect("still parses as valid xml");
	assert!(package.items.is_empty());
	assert!(package.spine.is_empty());
	assert!(package.title.is_none());
}

#[test]
fn invalid_xml_is_an_error() {
	assert!(parse_opf_package("<package>", "").is_err());
}

#[rstest]
#[case("application/x-dtbook+xml", "book.xml", true)]
#[case("text/xml", "book.xml", false)]
#[case("application/smil", "section.smil", false)]
fn identifies_dtbook_items(#[case] media_type: &str, #[case] href: &str, #[case] expected: bool) {
	let item = ManifestItem { href: href.to_string(), media_type: media_type.to_string() };
	assert_eq!(is_dtbook_item(&item), expected);
}

#[rstest]
#[case("application/x-dtbook+xml", "book.xml", true)]
#[case("text/xml", "book.xml", true)]
#[case("text/xml", "book.smil", false)]
#[case("application/smil", "section.smil", false)]
fn identifies_dtbook_like_items(#[case] media_type: &str, #[case] href: &str, #[case] expected: bool) {
	let item = ManifestItem { href: href.to_string(), media_type: media_type.to_string() };
	assert_eq!(is_dtbook_like_item(&item), expected);
}

#[test]
fn finds_the_declared_dtbook_item_over_the_untyped_xml_fallback() {
	let package = OpfPackage {
		items: vec![
			("misc".to_string(), ManifestItem { href: "misc.xml".to_string(), media_type: "text/xml".to_string() }),
			(
				"dtbook".to_string(),
				ManifestItem { href: "book.xml".to_string(), media_type: "application/x-dtbook+xml".to_string() },
			),
		],
		spine: Vec::new(),
		title: None,
		author: None,
	};
	assert_eq!(find_single_dtbook_href(&package), Some("book.xml".to_string()));
}

#[test]
fn falls_back_to_the_first_untyped_xml_item_when_no_item_is_declared_as_dtbook() {
	let package = OpfPackage {
		items: vec![
			("a".to_string(), ManifestItem { href: "a.smil".to_string(), media_type: "application/smil".to_string() }),
			("b".to_string(), ManifestItem { href: "b.xml".to_string(), media_type: "text/xml".to_string() }),
		],
		spine: Vec::new(),
		title: None,
		author: None,
	};
	assert_eq!(find_single_dtbook_href(&package), Some("b.xml".to_string()));
}

#[test]
fn returns_none_when_nothing_looks_like_a_dtbook() {
	let package = OpfPackage {
		items: vec![(
			"a".to_string(),
			ManifestItem { href: "a.smil".to_string(), media_type: "application/smil".to_string() },
		)],
		spine: Vec::new(),
		title: None,
		author: None,
	};
	assert_eq!(find_single_dtbook_href(&package), None);
}

#[rstest]
#[case("OEBPS/book.opf", "OEBPS")]
#[case("book.opf", "")]
#[case("a/b/c.opf", "a/b")]
fn extracts_the_directory_of_a_resolved_path(#[case] path: &str, #[case] expected: &str) {
	assert_eq!(dir_of(path), expected);
}
