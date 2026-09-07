use std::collections::HashMap;

use roxmltree::Document as XmlDocument;
use rstest::rstest;

use super::{display_len, extract_slide_number, extract_slide_text, extract_slide_title, is_title_shape};

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
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, true);
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
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, true);
	// Tab-separated rows, no flat duplication of the cell text.
	assert_eq!(text, "One\tTwo\nThree\tFour\n");
	assert_eq!(tables.len(), 1);
	let table = &tables[0];
	assert_eq!(table.offset, 0);
	assert_eq!(table.caption, "One Two");
	assert!(table.html.contains("<table"));
	assert!(table.html.contains("Four"));
	// Two rows, each contributing its display width + a trailing newline.
	assert_eq!(table.length, display_len("One\tTwo") + 1 + display_len("Three\tFour") + 1);
	assert!(links.is_empty());
}

#[test]
fn extract_slide_text_renders_table_placeholder() {
	let doc = XmlDocument::parse(TABLE_SLIDE_XML).expect("xml parse");
	let mut links = Vec::new();
	let mut tables = Vec::new();
	let rels = HashMap::new();
	let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, false);
	assert_eq!(text, "[Table]: One Two\n");
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].caption, "One Two");
}
