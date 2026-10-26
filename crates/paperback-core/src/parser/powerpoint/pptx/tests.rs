use std::collections::HashMap;

use roxmltree::Document as XmlDocument;
use rstest::rstest;

use super::{extract_slide_number, extract_slide_text, extract_slide_title, is_title_shape};
use crate::document::{DocLen, DocPos};

#[rstest]
#[case("ppt/slides/slide1.xml", 1)]
#[case("ppt/slides/slide12.xml", 12)]
#[case("slide007.xml", 7)]
#[case("ppt/slides/custom.xml", 0)]
fn extract_slide_number_parses_digits(#[case] name: &str, #[case] expected: usize) {
	assert_eq!(extract_slide_number(name), expected);
}

#[test]
fn is_title_shape_true_for_title_and_center_title() {
	let xml = r#"
		<root>
			<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr></sp>
			<sp><nvSpPr><nvPr><ph type="ctrTitle" /></nvPr></nvSpPr></sp>
		</root>
	"#;
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let shapes: Vec<_> = doc.descendants().filter(|n| n.tag_name().name() == "sp").collect();
	assert!(is_title_shape(shapes[0]));
	assert!(is_title_shape(shapes[1]));
}

#[test]
fn is_title_shape_false_without_title_placeholder() {
	let xml = r#"<root><sp><nvSpPr><nvPr><ph type="body" /></nvPr></nvSpPr></sp></root>"#;
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let shape = doc.descendants().find(|n| n.tag_name().name() == "sp").expect("shape");
	assert!(!is_title_shape(shape));
}

#[test]
fn extract_slide_title_uses_first_non_empty_title_shape() {
	let xml = r#"
		<root>
			<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr><txBody><p><r><t>  </t></r></p></txBody></sp>
			<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr><txBody><p><r><t>Agenda</t></r></p></txBody></sp>
		</root>
	"#;
	let doc = XmlDocument::parse(xml).expect("xml parse");
	assert_eq!(extract_slide_title(doc.root()), "Agenda");
}

#[test]
fn extract_slide_title_returns_empty_when_missing() {
	let xml = r"<root><sp><txBody><p><r><t>Body text</t></r></p></txBody></sp></root>";
	let doc = XmlDocument::parse(xml).expect("xml parse");
	assert!(extract_slide_title(doc.root()).is_empty());
}

#[test]
fn extract_slide_text_collects_paragraphs_and_breaks() {
	let xml = r"
		<root>
			<p><r><t>Hello</t></r><br/><r><t>World</t></r></p>
			<p><r><t>Next</t></r></p>
		</root>
	";
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::new();
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, true);
	assert_eq!(text, "Hello\nWorld\nNext\n");
	assert!(links.is_empty());
	assert!(tables.is_empty());
}

const TABLE_SLIDE_XML: &str = r"
	<root>
		<graphicFrame><graphic><graphicData>
			<tbl>
				<tblPr/>
				<tblGrid><gridCol/><gridCol/></tblGrid>
				<tr><tc><txBody><p><r><t>One</t></r></p></txBody></tc><tc><txBody><p><r><t>Two</t></r></p></txBody></tc></tr>
				<tr><tc><txBody><p><r><t>Three</t></r></p></txBody></tc><tc><txBody><p><r><t>Four</t></r></p></txBody></tc></tr>
			</tbl>
		</graphicData></graphic></graphicFrame>
	</root>
";

#[test]
fn extract_slide_text_renders_table_inline_with_marker_data() {
	let doc = XmlDocument::parse(TABLE_SLIDE_XML).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::new();
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, true);
	// Tab-separated rows, no flat duplication of the cell text.
	assert_eq!(text, "One\tTwo\nThree\tFour\n");
	assert_eq!(tables.len(), 1);
	let table = &tables[0];
	assert_eq!(table.offset, 0);
	assert_eq!(table.caption, "One Two");
	assert!(table.html.contains("<table"));
	assert!(table.html.contains("Four"));
	// Two rows, each contributing its length + a trailing newline.
	assert_eq!(table.length, DocLen::of("One\tTwo") + 1.into() + DocLen::of("Three\tFour") + 1.into());
	assert!(links.is_empty());
}

#[test]
fn extract_slide_text_renders_table_placeholder() {
	let doc = XmlDocument::parse(TABLE_SLIDE_XML).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::new();
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, false);
	assert_eq!(text, "[Table]: One Two\n");
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].caption, "One Two");
}

/// A link's position is counted in chars, which an emoji before it makes differ from bytes.
#[test]
fn extract_slide_text_places_a_link_after_an_emoji_in_positions() {
	let xml = "<root><p><r><t>\u{1F600} see </t></r><r><hlinkClick id=\"rId1\"/><t>this</t></r></p></root>";
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::from([("rId1".to_string(), "https://example.com".to_string())]);
	extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, true);
	assert_eq!(links[0].offset, DocLen::of("\u{1F600} see ").as_pos());
}

/// Real decks put the link in the run's properties, beside its text, and split a link into several runs where its formatting changes.
#[test]
fn a_link_in_a_runs_properties_covers_its_text_once() {
	let xml = r#"<root><p><r><t>Visit </t></r><r><rPr><hlinkClick id="rId1"/></rPr><t>our </t></r><r><rPr><b/><hlinkClick id="rId1"/></rPr><t>site</t></r><r><t> today</t></r></p></root>"#;
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::from([("rId1".to_string(), "https://example.com".to_string())]);
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, true);
	assert_eq!(
		text,
		"Visit our site today
"
	);
	assert_eq!(links.len(), 1);
	assert_eq!(
		(links[0].offset, links[0].text.as_str(), links[0].reference.as_str()),
		(6.into(), "our site", "https://example.com")
	);
}

/// A link written directly in the run, as some tools do, still covers the run's text without writing it twice.
#[test]
fn a_link_directly_in_a_run_does_not_repeat_its_text() {
	let xml = r#"<root><p><r><hlinkClick id="rId1"/><t>this</t></r></p></root>"#;
	let doc = XmlDocument::parse(xml).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::from([("rId1".to_string(), "https://example.com".to_string())]);
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0.into(), &rels, true);
	assert_eq!(
		text,
		"this
"
	);
	assert_eq!(links[0].text, "this");
}

/// A slide of pictures alone has no text, and still has to be there to reach, under its own number.
#[test]
fn a_slide_with_no_text_is_kept_and_says_so() {
	use std::{fs::File, io::Write};
	let dir = crate::util::test_support::TempDir::new("pptx-empty-slide");
	let path = dir.join_str("deck.pptx");
	let mut zip = zip::ZipWriter::new(File::create(&path).unwrap());
	for (name, body) in [
		("ppt/slides/slide1.xml", "<sld><pic/></sld>"),
		("ppt/slides/slide2.xml", "<sld><sp><txBody><p><r><t>Words</t></r></p></txBody></sp></sld>"),
	] {
		zip.start_file(name, zip::write::SimpleFileOptions::default()).unwrap();
		zip.write_all(body.as_bytes()).unwrap();
	}
	zip.finish().unwrap();
	let document = super::parse_pptx(&crate::document::ParserContext::new(path)).unwrap();
	assert_eq!(document.buffer.content, "This slide has no text.\n\nWords\n");
	let slides: Vec<(DocPos, &str)> = document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == crate::document::MarkerType::PageBreak)
		.map(|marker| (marker.position, marker.text.as_str()))
		.collect();
	assert_eq!(slides, [(DocPos::ZERO, "Slide 1"), (DocLen::of("This slide has no text.\n\n").as_pos(), "Slide 2")]);
}
