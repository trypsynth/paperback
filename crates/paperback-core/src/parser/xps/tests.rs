use super::{XpsParser, read_document};
use crate::{
	document::{Document, MarkerType, ParserContext},
	parser::{Parser, ParserRegistry, xps::package::Package},
};

pub(super) const ROOT_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/fixedrepresentation" Target="/Sequence.fdseq"/>
</Relationships>"#;
pub(super) const SEQUENCE: &str =
	r#"<FixedDocumentSequence><DocumentReference Source="Doc.fdoc"/></FixedDocumentSequence>"#;

pub(super) fn read(parts: &[(&str, &str)]) -> anyhow::Result<Document> {
	read_with(parts, &ParserContext::new("sample.xps".to_string()))
}

pub(super) fn read_with(parts: &[(&str, &str)], context: &ParserContext) -> anyhow::Result<Document> {
	read_document(Package::from_parts(parts), context)
}

#[test]
fn both_extensions_reach_the_parser() {
	for extension in ["xps", "oxps", "OXPS"] {
		let parsers = ParserRegistry::global().get_parsers_for_extension(extension);
		assert_eq!(parsers.iter().map(|parser| parser.name()).collect::<Vec<_>>(), ["XPS Documents"]);
	}
}

#[test]
fn a_file_that_is_not_a_zip_is_refused() {
	let dir = crate::util::test_support::TempDir::new("xps");
	let path = dir.path().join("broken.xps");
	std::fs::write(&path, "not a zip").expect("write");
	let context = ParserContext::new(path.to_string_lossy().into_owned());
	assert!(XpsParser.parse(&context).is_err());
}

#[test]
fn a_package_with_no_pages_is_refused() {
	assert!(
		read(&[("_rels/.rels", ROOT_RELS), ("Sequence.fdseq", SEQUENCE), ("Doc.fdoc", "<FixedDocument/>")]).is_err()
	);
}

#[test]
fn each_page_gets_a_marker_and_an_id() {
	let document = r#"<FixedDocument>
		<PageContent Source="1.fpage"/>
		<PageContent Source="2.fpage"/>
	</FixedDocument>"#;
	let doc = read(&[
		("_rels/.rels", ROOT_RELS),
		("Sequence.fdseq", SEQUENCE),
		("Doc.fdoc", document),
		("1.fpage", "<FixedPage/>"),
		("2.fpage", "<FixedPage/>"),
	])
	.expect("read");
	let pages: Vec<_> = doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::PageBreak).collect();
	assert_eq!(pages.iter().map(|marker| marker.text.as_str()).collect::<Vec<_>>(), ["Page 1", "Page 2"]);
	assert_eq!(doc.id_positions["page_0"], pages[0].position);
	assert_eq!(doc.id_positions["page_1"], pages[1].position);
}

#[test]
fn the_title_falls_back_to_the_file_name() {
	let doc = read(&[
		("_rels/.rels", ROOT_RELS),
		("Sequence.fdseq", SEQUENCE),
		("Doc.fdoc", r#"<FixedDocument><PageContent Source="1.fpage"/></FixedDocument>"#),
		("1.fpage", "<FixedPage/>"),
	])
	.expect("read");
	assert_eq!(doc.title, "sample");
}
