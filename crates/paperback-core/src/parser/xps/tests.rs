use super::{XpsParser, read_document};
use crate::{
	document::{DocLen, DocPos, Document, Marker, MarkerType, ParserContext},
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
fn a_page_that_cannot_be_read_keeps_its_place() {
	let document = r#"<FixedDocument>
		<PageContent Source="1.fpage"/>
		<PageContent Source="missing.fpage"/>
		<PageContent Source="3.fpage"/>
	</FixedDocument>"#;
	let text =
		r#"<FixedPage><Glyphs OriginX="1" OriginY="1" FontRenderingEmSize="12" UnicodeString="Words"/></FixedPage>"#;
	let doc = read(&[
		("_rels/.rels", ROOT_RELS),
		("Sequence.fdseq", SEQUENCE),
		("Doc.fdoc", document),
		("1.fpage", text),
		("3.fpage", text),
	])
	.expect("read");
	assert_eq!(doc.buffer.content, "Words\nWords\n");
	assert_eq!(doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::PageBreak).count(), 3);
}

pub(super) const FRAGMENT_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/storyfragments" Target="1.frag"/>
</Relationships>"#;
pub(super) const ONE_PAGE: &str = r#"<FixedDocument><PageContent Source="1.fpage"/></FixedDocument>"#;

pub(super) fn run(name: &str, text: &str) -> String {
	format!(r#"<Glyphs Name="{name}" OriginX="10" OriginY="20" FontRenderingEmSize="12" UnicodeString="{text}"/>"#)
}

pub(super) fn structured(page: &str, fragments: &str) -> anyhow::Result<Document> {
	structured_with(page, fragments, &ParserContext::new("sample.xps".to_string()))
}

pub(super) fn structured_with(page: &str, fragments: &str, context: &ParserContext) -> anyhow::Result<Document> {
	read_with(
		&[
			("_rels/.rels", ROOT_RELS),
			("Sequence.fdseq", SEQUENCE),
			("Doc.fdoc", ONE_PAGE),
			("1.fpage", &format!("<FixedPage>{page}</FixedPage>")),
			("_rels/1.fpage.rels", FRAGMENT_RELS),
			("1.frag", &format!("<StoryFragments>{fragments}</StoryFragments>")),
		],
		context,
	)
}

#[test]
fn a_paragraph_joins_its_runs_exactly_as_listed() {
	let page = format!("{}{}{}", run("c", "second."), run("a", "The first "), run("b", "and the "));
	let fragments = r#"<StoryFragment><ParagraphStructure>
		<NamedElement NameReference="a"/><NamedElement NameReference="b"/><NamedElement NameReference="c"/>
	</ParagraphStructure></StoryFragment>"#;
	assert_eq!(structured(&page, fragments).expect("read").buffer.content, "The first and the second.\n");
}

#[test]
fn a_named_canvas_supplies_every_run_under_it() {
	let page = format!(r#"<Canvas Name="c">{}{}</Canvas>"#, run("x", "One "), run("y", "two"));
	let fragments =
		r#"<StoryFragment><ParagraphStructure><NamedElement NameReference="c"/></ParagraphStructure></StoryFragment>"#;
	assert_eq!(structured(&page, fragments).expect("read").buffer.content, "One two\n");
}

#[test]
fn a_paragraph_is_cleaned_of_the_spaces_its_runs_carry() {
	let page = format!("{}{}", run("a", " Spaced  out "), run("b", " "));
	let fragments = r#"<StoryFragment><ParagraphStructure><NamedElement NameReference="a"/><NamedElement NameReference="b"/></ParagraphStructure></StoryFragment>"#;
	assert_eq!(structured(&page, fragments).expect("read").buffer.content, "Spaced out\n");
}

#[test]
fn a_paragraph_with_no_text_leaves_no_blank_line() {
	let page = format!("{}{}", run("a", " "), run("b", "Words"));
	let fragments = r#"<StoryFragment>
		<ParagraphStructure><NamedElement NameReference="a"/></ParagraphStructure>
		<ParagraphStructure><NamedElement NameReference="b"/></ParagraphStructure>
	</StoryFragment>"#;
	assert_eq!(structured(&page, fragments).expect("read").buffer.content, "Words\n");
}

#[test]
fn runs_no_fragment_names_still_reach_the_reader() {
	let page = format!("{}{}", run("a", "Listed"), run("b", "Forgotten"));
	let fragments =
		r#"<StoryFragment><ParagraphStructure><NamedElement NameReference="a"/></ParagraphStructure></StoryFragment>"#;
	assert_eq!(structured(&page, fragments).expect("read").buffer.content, "Listed\nForgotten\n");
}

#[test]
fn a_reference_to_a_name_the_page_lacks_is_ignored() {
	let fragments = r#"<StoryFragment><ParagraphStructure><NamedElement NameReference="gone"/></ParagraphStructure></StoryFragment>"#;
	assert_eq!(structured(&run("a", "Kept"), fragments).expect("read").buffer.content, "Kept\n");
}

#[test]
fn a_broken_fragment_part_falls_back_to_the_page() {
	let doc = read(&[
		("_rels/.rels", ROOT_RELS),
		("Sequence.fdseq", SEQUENCE),
		("Doc.fdoc", ONE_PAGE),
		("1.fpage", &format!("<FixedPage>{}</FixedPage>", run("a", "Still here"))),
		("_rels/1.fpage.rels", FRAGMENT_RELS),
		("1.frag", "<StoryFragments"),
	])
	.expect("read");
	assert_eq!(doc.buffer.content, "Still here\n");
}

fn paragraph(name: &str) -> String {
	format!(r#"<ParagraphStructure><NamedElement NameReference="{name}"/></ParagraphStructure>"#)
}

fn in_fragment(body: &str) -> String {
	format!("<StoryFragment>{body}</StoryFragment>")
}

fn item(body: &str) -> String {
	format!("<ListItemStructure>{body}</ListItemStructure>")
}

fn markers(doc: &Document, kind: MarkerType) -> Vec<&Marker> {
	doc.buffer.markers.iter().filter(|marker| marker.mtype == kind).collect()
}

#[test]
fn a_list_item_drops_its_drawn_bullet() {
	let page = format!("{}{}{}{}", run("b1", "·"), run("t1", "First"), run("b2", "\u{F0B7}"), run("t2", "Second"));
	let list = format!(
		"<ListStructure>{}{}</ListStructure>",
		item(&format!("{}{}", paragraph("b1"), paragraph("t1"))),
		item(&format!("{}{}", paragraph("b2"), paragraph("t2")))
	);
	let doc = structured(&page, &in_fragment(&list)).expect("read");
	assert_eq!(doc.buffer.content, "First\nSecond\n");
	let items = markers(&doc, MarkerType::ListItem);
	assert_eq!(items.iter().map(|marker| marker.text.as_str()).collect::<Vec<_>>(), ["First", "Second"]);
	assert_eq!((items[0].level, items[1].position), (1, DocPos::ZERO + DocLen::of("First\n")));
	let list = markers(&doc, MarkerType::List);
	assert_eq!((list[0].level, list[0].length), (2, DocLen::of("First\nSecond\n")));
}

#[test]
fn a_list_item_keeps_its_number() {
	let page = format!("{}{}", run("n", "1."), run("t", "First"));
	let list = format!("<ListStructure>{}</ListStructure>", item(&format!("{}{}", paragraph("n"), paragraph("t"))));
	assert_eq!(structured(&page, &in_fragment(&list)).expect("read").buffer.content, "1. First\n");
}

#[test]
fn a_nested_list_item_goes_one_level_deeper() {
	let page = format!("{}{}", run("o", "Outer"), run("i", "Inner"));
	let inner = format!("<ListStructure>{}</ListStructure>", item(&paragraph("i")));
	let outer = format!("<ListStructure>{}</ListStructure>", item(&format!("{}{inner}", paragraph("o"))));
	let doc = structured(&page, &in_fragment(&outer)).expect("read");
	assert_eq!(doc.buffer.content, "Outer\nInner\n");
	let levels: Vec<i32> = markers(&doc, MarkerType::ListItem).iter().map(|marker| marker.level).collect();
	assert_eq!(levels, [1, 2]);
	assert_eq!(markers(&doc, MarkerType::List).len(), 2);
}

fn two_by_two() -> String {
	let cell = |name: &str| format!("<TableCellStructure>{}</TableCellStructure>", paragraph(name));
	let row = |left: &str, right: &str| format!("<TableRowStructure>{}{}</TableRowStructure>", cell(left), cell(right));
	in_fragment(&format!(
		"<TableStructure><TableRowGroupStructure>{}{}</TableRowGroupStructure></TableStructure>",
		row("a", "b"),
		row("c", "d")
	))
}

#[test]
fn a_table_gets_its_marker_and_its_html() {
	let page =
		format!("{}{}{}{}", run("a", "Cell one "), run("b", "Cell two"), run("c", "Cell three"), run("d", "Cell four"));
	let doc = structured(&page, &two_by_two()).expect("read");
	let tables = markers(&doc, MarkerType::Table);
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].position, DocPos::ZERO);
	assert!(tables[0].reference.contains("<td>Cell one</td>"), "{}", tables[0].reference);
	assert!(tables[0].reference.contains("<td>Cell four</td>"), "{}", tables[0].reference);
	assert!(doc.buffer.content.contains("Cell two"));
	assert!(doc.buffer.content.ends_with('\n'));
}

#[test]
fn a_table_follows_the_setting_for_showing_tables_inline() {
	let page = format!("{}{}{}{}", run("a", "One"), run("b", "Two"), run("c", "Three"), run("d", "Four"));
	let mut context = ParserContext::new("sample.xps".to_string());
	context.render_tables_inline = false;
	let doc = structured_with(&page, &two_by_two(), &context).expect("read");
	assert!(!doc.buffer.content.contains("Three"));
}

#[test]
fn title_and_author_come_from_the_core_properties_relationship() {
	let rels = r#"<Relationships>
		<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/fixedrepresentation" Target="/Sequence.fdseq"/>
		<Relationship Id="R1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="Metadata/props.xml"/>
	</Relationships>"#;
	let core = r#"<cp:coreProperties xmlns:cp="c" xmlns:dc="http://purl.org/dc/elements/1.1/">
		<dc:title>The title</dc:title><dc:creator>The author</dc:creator>
	</cp:coreProperties>"#;
	let doc = read(&[
		("_rels/.rels", rels),
		("Sequence.fdseq", SEQUENCE),
		("Doc.fdoc", ONE_PAGE),
		("1.fpage", "<FixedPage/>"),
		("Metadata/props.xml", core),
	])
	.expect("read");
	assert_eq!((doc.title.as_str(), doc.author.as_str()), ("The title", "The author"));
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
