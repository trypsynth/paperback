use roxmltree::Document;

use super::*;

fn parse(xml: &str) -> PackageParts {
	let doc = Document::parse(xml).expect("valid test xml");
	let package = doc.root_element();
	parse_package(package, "OEBPS")
}

#[test]
fn collects_manifest_items_and_reading_order() {
	let xml = r#"<package>
		<manifest>
			<item id="c1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
			<item id="c2" href="sub/chapter2.xhtml" media-type="application/xhtml+xml"/>
		</manifest>
		<spine>
			<itemref idref="c1"/>
			<itemref idref="c2"/>
		</spine>
	</package>"#;
	let (manifest, spine, nav_path, ncx_path, _metadata) = parse(xml);
	assert_eq!(manifest.len(), 2);
	assert_eq!(manifest["c1"].path, "OEBPS/chapter1.xhtml");
	assert_eq!(manifest["c2"].path, "OEBPS/sub/chapter2.xhtml");
	assert_eq!(spine, vec!["c1".to_string(), "c2".to_string()]);
	assert!(nav_path.is_none());
	assert!(ncx_path.is_none());
}

#[test]
fn skips_manifest_items_missing_id_or_href() {
	let xml = r#"<package>
		<manifest>
			<item href="no-id.xhtml" media-type="application/xhtml+xml"/>
			<item id="no-href" media-type="application/xhtml+xml"/>
			<item id="ok" href="ok.xhtml" media-type="application/xhtml+xml"/>
		</manifest>
		<spine/>
	</package>"#;
	let (manifest, ..) = parse(xml);
	assert_eq!(manifest.len(), 1);
	assert!(manifest.contains_key("ok"));
}

#[test]
fn detects_epub3_nav_document_via_properties() {
	let xml = r#"<package>
		<manifest>
			<item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>
			<item id="other" href="other.xhtml" media-type="application/xhtml+xml" properties="scripted nav"/>
		</manifest>
		<spine/>
	</package>"#;
	let (_manifest, _spine, nav_path, _ncx_path, _metadata) = parse(xml);
	// The last manifest item with the "nav" property wins; both carry it here, so either would be
	// acceptable, but exactly one nav_path must be set.
	assert_eq!(nav_path, Some("OEBPS/other.xhtml".to_string()));
}

#[test]
fn detects_ncx_via_media_type() {
	let xml = r#"<package>
		<manifest>
			<item id="ncx" href="toc.ncx" media-type="application/x-dtbncx+xml"/>
		</manifest>
		<spine/>
	</package>"#;
	let (_manifest, _spine, _nav_path, ncx_path, _metadata) = parse(xml);
	assert_eq!(ncx_path, Some("OEBPS/toc.ncx".to_string()));
}

#[test]
fn falls_back_to_spine_toc_attribute_for_ncx() {
	let xml = r#"<package>
		<manifest>
			<item id="ncx" href="toc.ncx" media-type="application/octet-stream"/>
		</manifest>
		<spine toc="ncx"/>
	</package>"#;
	let (_manifest, _spine, _nav_path, ncx_path, _metadata) = parse(xml);
	assert_eq!(ncx_path, Some("OEBPS/toc.ncx".to_string()));
}

#[test]
fn media_type_detected_ncx_takes_priority_over_spine_toc_attribute() {
	let xml = r#"<package>
		<manifest>
			<item id="real-ncx" href="toc.ncx" media-type="application/x-dtbncx+xml"/>
			<item id="decoy" href="decoy.ncx" media-type="application/octet-stream"/>
		</manifest>
		<spine toc="decoy"/>
	</package>"#;
	let (_manifest, _spine, _nav_path, ncx_path, _metadata) = parse(xml);
	assert_eq!(ncx_path, Some("OEBPS/toc.ncx".to_string()));
}

#[test]
fn extracts_title_and_author_from_metadata() {
	let xml = r#"<package xmlns:dc="http://purl.org/dc/elements/1.1/">
		<metadata>
			<dc:title>My Book</dc:title>
			<dc:creator>Some Author</dc:creator>
		</metadata>
		<manifest/>
		<spine/>
	</package>"#;
	let (_manifest, _spine, _nav_path, _ncx_path, metadata) = parse(xml);
	assert_eq!(metadata.title, Some("My Book".to_string()));
	assert_eq!(metadata.author, Some("Some Author".to_string()));
}

#[test]
fn missing_metadata_yields_none_title_and_author() {
	let xml = r"<package><manifest/><spine/></package>";
	let (_manifest, _spine, _nav_path, _ncx_path, metadata) = parse(xml);
	assert!(metadata.title.is_none());
	assert!(metadata.author.is_none());
}

#[test]
fn href_percent_encoding_is_decoded_before_resolving() {
	let xml = r#"<package>
		<manifest>
			<item id="c1" href="chapter%201.xhtml" media-type="application/xhtml+xml"/>
		</manifest>
		<spine/>
	</package>"#;
	let (manifest, ..) = parse(xml);
	assert_eq!(manifest["c1"].path, "OEBPS/chapter 1.xhtml");
}
