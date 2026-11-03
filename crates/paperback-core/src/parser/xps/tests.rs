use super::{XpsParser, read_document};
use crate::{
	document::{DocLen, DocPos, Document, Marker, MarkerType, ParserContext, is_heading_marker},
	parser::{Parser, ParserRegistry, xps::package::Package},
};

const ROOT_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/fixedrepresentation" Target="/Sequence.fdseq"/>
</Relationships>"#;
const SEQUENCE: &str = r#"<FixedDocumentSequence><DocumentReference Source="Doc.fdoc"/></FixedDocumentSequence>"#;

fn context() -> ParserContext {
	ParserContext::new("sample.xps".to_string())
}

fn read_parts(parts: &[(&str, &str)], context: &ParserContext) -> anyhow::Result<Document> {
	read_document(Package::from_parts(parts), context)
}

/// A package whose one document is `document`, with `rest` beside it.
fn read_with(document: &str, rest: &[(&str, &str)], context: &ParserContext) -> anyhow::Result<Document> {
	let mut parts = vec![("_rels/.rels", ROOT_RELS), ("Sequence.fdseq", SEQUENCE), ("Doc.fdoc", document)];
	parts.extend_from_slice(rest);
	read_parts(&parts, context)
}

fn read(document: &str, rest: &[(&str, &str)]) -> anyhow::Result<Document> {
	read_with(document, rest, &context())
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
	assert!(read("<FixedDocument/>", &[]).is_err());
}

#[test]
fn each_page_gets_a_marker_and_an_id() {
	let document = r#"<FixedDocument>
		<PageContent Source="1.fpage"/>
		<PageContent Source="2.fpage"/>
	</FixedDocument>"#;
	let doc = read(document, &[("1.fpage", "<FixedPage/>"), ("2.fpage", "<FixedPage/>")]).expect("read");
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
	let doc = read(document, &[("1.fpage", text), ("3.fpage", text)]).expect("read");
	assert_eq!(doc.buffer.content, "Words\nThis page has no text.\nWords\n");
	assert_eq!(doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::PageBreak).count(), 3);
}

const FRAGMENT_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/storyfragments" Target="1.frag"/>
</Relationships>"#;
const ONE_PAGE: &str = r#"<FixedDocument><PageContent Source="1.fpage"/></FixedDocument>"#;

fn glyphs(name: &str, text: &str, x: f64, size: f64) -> String {
	format!(r#"<Glyphs Name="{name}" OriginX="{x}" OriginY="20" FontRenderingEmSize="{size}" UnicodeString="{text}"/>"#)
}

fn run(name: &str, text: &str) -> String {
	glyphs(name, text, 10.0, 12.0)
}

fn structured(page: &str, fragments: &str) -> anyhow::Result<Document> {
	structured_with(page, fragments, &context())
}

fn structured_with(page: &str, fragments: &str, context: &ParserContext) -> anyhow::Result<Document> {
	let page = format!("<FixedPage>{page}</FixedPage>");
	let fragments = format!("<StoryFragments>{fragments}</StoryFragments>");
	read_with(ONE_PAGE, &[("1.fpage", &page), ("_rels/1.fpage.rels", FRAGMENT_RELS), ("1.frag", &fragments)], context)
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
	let page = format!("<FixedPage>{}</FixedPage>", run("a", "Still here"));
	let rest = [("1.fpage", page.as_str()), ("_rels/1.fpage.rels", FRAGMENT_RELS), ("1.frag", "<StoryFragments")];
	let doc = read(ONE_PAGE, &rest).expect("read");
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
	let mut context = context();
	context.render_tables_inline = false;
	let doc = structured_with(&page, &two_by_two(), &context).expect("read");
	assert!(!doc.buffer.content.contains("Three"));
}

fn with_running_text() -> (String, String) {
	let page = format!("{}{}{}", run("f", "Footer"), run("h", "Header"), run("b", "Body"));
	let fragments = format!(
		r#"<StoryFragment FragmentType="Content">{}</StoryFragment>
		<StoryFragment FragmentType="Footer">{}</StoryFragment>
		<StoryFragment FragmentType="Header">{}</StoryFragment>"#,
		paragraph("b"),
		paragraph("f"),
		paragraph("h")
	);
	(page, fragments)
}

#[test]
fn declared_headers_and_footers_go_with_the_running_text_setting() {
	let (page, fragments) = with_running_text();
	assert_eq!(structured(&page, &fragments).expect("read").buffer.content, "Body\n");
}

#[test]
fn kept_headers_and_footers_read_around_the_page_they_belong_to() {
	let (page, fragments) = with_running_text();
	let context = context().with_strip_running_text(false);
	assert_eq!(structured_with(&page, &fragments, &context).expect("read").buffer.content, "Header\nBody\nFooter\n");
}

const STRUCTURE_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/documentstructure" Target="Doc.struct"/>
</Relationships>"#;
const LINKED_PAGES: &str = r#"<FixedDocument>
	<PageContent Source="1.fpage"><PageContent.LinkTargets><LinkTarget Name="one"/></PageContent.LinkTargets></PageContent>
	<PageContent Source="2.fpage"><PageContent.LinkTargets><LinkTarget Name="two"/><LinkTarget Name="three"/></PageContent.LinkTargets></PageContent>
</FixedDocument>"#;
const SECOND_FRAGMENT_RELS: &str = r#"<Relationships>
	<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/storyfragments" Target="2.frag"/>
</Relationships>"#;

fn sized(name: &str, text: &str, size: f64) -> String {
	glyphs(name, text, 10.0, size)
}

fn outlined(outline: &str) -> Document {
	let first = format!("<FixedPage>{}{}</FixedPage>", sized("a", "Chapter", 20.0), sized("b", BODY, 12.0));
	let second = format!(
		"<FixedPage>{}{}{}</FixedPage>",
		sized("c", "Second chapter", 20.0),
		sized("d", "A section", 16.0),
		sized("e", BODY, 12.0)
	);
	let frag = |names: &[&str]| {
		let body: String = names.iter().map(|name| paragraph(name)).collect();
		format!("<StoryFragments>{}</StoryFragments>", in_fragment(&body))
	};
	read(LINKED_PAGES, &[
		("_rels/Doc.fdoc.rels", STRUCTURE_RELS),
		("Doc.struct", &format!("<DocumentStructure><DocumentStructure.Outline><DocumentOutline>{outline}</DocumentOutline></DocumentStructure.Outline></DocumentStructure>")),
		("1.fpage", &first),
		("_rels/1.fpage.rels", FRAGMENT_RELS),
		("1.frag", &frag(&["a", "b"])),
		("2.fpage", &second),
		("_rels/2.fpage.rels", SECOND_FRAGMENT_RELS),
		("2.frag", &frag(&["c", "d", "e"])),
	])
	.expect("read")
}

const BODY: &str =
	"A paragraph of body text long enough to outweigh every heading in the document put together, by some way.";

#[test]
fn the_outline_becomes_the_table_of_contents() {
	let doc = outlined(
		r#"<OutlineEntry OutlineLevel="1" Description="Chapter" OutlineTarget="Doc.fdoc#one"/>
		<OutlineEntry OutlineLevel="1" Description="Second chapter" OutlineTarget="Doc.fdoc#two"/>
		<OutlineEntry OutlineLevel="2" Description="A section" OutlineTarget="Doc.fdoc#three"/>"#,
	);
	assert_eq!(doc.toc_items.len(), 2);
	assert_eq!(doc.toc_items[0].name, "Chapter");
	assert_eq!(doc.toc_items[0].offset, DocPos::ZERO);
	assert_eq!(doc.toc_items[1].children[0].name, "A section");
	let section = doc.buffer.content.find("A section").expect("section");
	assert_eq!(doc.toc_items[1].children[0].offset, doc.buffer.pos_at_byte(section.into()));
	let levels: Vec<i32> =
		doc.buffer.markers.iter().filter(|marker| is_heading_marker(marker.mtype)).map(|marker| marker.level).collect();
	assert_eq!(levels, [1, 1, 2]);
}

#[test]
fn an_outline_entry_whose_text_is_not_on_its_page_leads_to_the_page() {
	let doc = outlined(r#"<OutlineEntry OutlineLevel="1" Description="Renamed" OutlineTarget="Doc.fdoc#two"/>"#);
	assert_eq!(doc.toc_items[0].offset, doc.id_positions["page_1"]);
}

#[test]
fn an_outline_entry_leading_nowhere_is_left_out() {
	let doc = outlined(r#"<OutlineEntry OutlineLevel="1" Description="Gone" OutlineTarget="Doc.fdoc#missing"/>"#);
	assert!(doc.toc_items.is_empty() || doc.toc_items.iter().all(|item| item.name != "Gone"));
}

#[test]
fn a_file_with_no_outline_finds_headings_by_size_against_its_body_text() {
	let doc = outlined("");
	let names: Vec<&str> = doc.toc_items.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(names, ["Chapter", "Second chapter", "A section"]);
	assert_eq!(markers(&doc, MarkerType::Heading1).len(), 3);
}

#[test]
fn a_declared_outline_wins_over_sizes() {
	let doc = outlined(r#"<OutlineEntry OutlineLevel="1" Description="Chapter" OutlineTarget="Doc.fdoc#one"/>"#);
	assert_eq!(doc.toc_items.len(), 1);
}

#[test]
fn list_items_and_table_cells_are_never_headings() {
	let page = format!("{}{}", sized("a", "Big list item", 30.0), sized("b", BODY, 12.0));
	let body = format!("<ListStructure>{}</ListStructure>{}", item(&paragraph("a")), paragraph("b"));
	assert!(structured(&page, &in_fragment(&body)).expect("read").toc_items.is_empty());
}

fn at(text: &str, y: f64) -> String {
	format!(r#"<Glyphs OriginX="72" OriginY="{y}" FontRenderingEmSize="16" UnicodeString="{text}"/>"#)
}

/// One sentence over three full lines, then a line of its own set well apart.
fn unstructured_page() -> String {
	format!(
		"<FixedPage Height=\"1000\">{}{}{}{}</FixedPage>",
		at("The first line of this one sentence runs the full width ", 100.0),
		at("of the page and carries straight on to the next line, ", 118.0),
		at("where it comes to an end short of the margin. ", 136.0),
		at("A paragraph of its own.", 200.0)
	)
}

fn unstructured_with(pages: &[String], context: &ParserContext) -> Document {
	let contents =
		(1..=pages.len()).map(|index| format!(r#"<PageContent Source="{index}.fpage"/>"#)).collect::<Vec<_>>().concat();
	let document = format!("<FixedDocument>{contents}</FixedDocument>");
	let names: Vec<String> = (1..=pages.len()).map(|index| format!("{index}.fpage")).collect();
	let parts: Vec<(&str, &str)> = names.iter().map(String::as_str).zip(pages.iter().map(String::as_str)).collect();
	read_with(&document, &parts, context).expect("read")
}

fn unstructured(pages: &[String]) -> Document {
	unstructured_with(pages, &context())
}

#[test]
fn wrapped_lines_join_into_paragraphs_on_a_page_with_no_structure() {
	assert_eq!(
		unstructured(&[unstructured_page()]).buffer.content,
		"The first line of this one sentence runs the full width of the page and carries straight on to the next line, where it comes to an end short of the margin.\nA paragraph of its own.\n"
	);
}

#[test]
fn lines_stay_apart_when_joining_is_off() {
	let mut context = context();
	context.join_pdf_paragraphs = false;
	assert_eq!(unstructured_with(&[unstructured_page()], &context).buffer.content.lines().count(), 4);
}

#[test]
fn a_line_repeated_at_the_top_of_every_page_is_running_text() {
	let page = |word: &str| {
		format!(
			"<FixedPage Height=\"1000\">{}{}{}{}</FixedPage>",
			at("Running header", 40.0),
			at(&format!("Words about {word}, enough of them to make up the body of it."), 100.0),
			at(&format!("More words about {word}, a second line for the body of it."), 118.0),
			at(&format!("A third line of words about {word}, also part of its body."), 136.0)
		)
	};
	let pages = ["apples", "pears", "plums", "cherries", "quinces"].map(page);
	assert!(!unstructured(&pages).buffer.content.contains("Running header"));
	let kept = context().with_strip_running_text(false);
	assert!(unstructured_with(&pages, &kept).buffer.content.contains("Running header"));
}

#[test]
fn large_lines_on_a_page_with_no_structure_are_headings() {
	let heading = r#"<Glyphs OriginX="72" OriginY="60" FontRenderingEmSize="27" UnicodeString="A heading"/>"#;
	let body: String = [200.0, 240.0, 280.0].map(|y| at("A paragraph of its own.", y)).concat();
	let page = format!("<FixedPage Height=\"1000\">{heading}{body}</FixedPage>");
	let doc = unstructured(&[page]);
	assert_eq!(doc.toc_items.iter().map(|item| item.name.as_str()).collect::<Vec<_>>(), ["A heading"]);
	assert_eq!(markers(&doc, MarkerType::Heading1).len(), 1);
}

const LINK_AREA: &str =
	r#"<Path FixedPage.NavigateUri="https://example.com/" Data="M 100,10 L 200,10 L 200,30 L 100,30 Z"/>"#;

fn at_x(name: &str, text: &str, x: f64) -> String {
	glyphs(name, text, x, 12.0)
}

#[test]
fn a_link_rectangle_reaches_the_text_it_covers_on_a_declared_page() {
	let page =
		format!("{}{}{}{LINK_AREA}", at_x("a", "Read ", 72.0), at_x("b", "this site", 100.0), at_x("c", " now", 201.0));
	let fragments = r#"<StoryFragment><ParagraphStructure>
		<NamedElement NameReference="a"/><NamedElement NameReference="b"/><NamedElement NameReference="c"/>
	</ParagraphStructure></StoryFragment>"#;
	let doc = structured(&page, fragments).expect("read");
	assert_eq!(doc.buffer.content, "Read this site now\n");
	let links = markers(&doc, MarkerType::Link);
	assert_eq!(links.len(), 1);
	assert_eq!((links[0].text.as_str(), links[0].reference.as_str()), ("this site", "https://example.com/"));
	assert_eq!(links[0].position, DocPos::ZERO + DocLen::of("Read "));
	assert_eq!(links[0].length, DocLen::of("this site"));
}

#[test]
fn a_link_rectangle_reaches_the_text_it_covers_on_a_page_with_no_structure() {
	let page = format!(
		"<FixedPage Height=\"1000\">{}{}{LINK_AREA}</FixedPage>",
		at_x("a", "Read ", 72.0),
		at_x("b", "this site", 100.0)
	);
	let doc = unstructured(&[page]);
	assert_eq!(markers(&doc, MarkerType::Link)[0].text, "this site");
}

#[test]
fn links_go_in_reading_order_whatever_order_the_page_lists_them() {
	let second =
		r#"<Path FixedPage.NavigateUri="https://example.com/second" Data="M 300,10 L 400,10 L 400,30 L 300,30 Z"/>"#;
	let first =
		r#"<Path FixedPage.NavigateUri="https://example.com/first" Data="M 100,10 L 200,10 L 200,30 L 100,30 Z"/>"#;
	let page = format!(
		"<FixedPage Height=\"1000\">{}{}{}{second}{first}</FixedPage>",
		at_x("a", "same", 100.0),
		at_x("b", " and ", 200.5),
		at_x("c", "same", 300.0)
	);
	let doc = unstructured(&[page]);
	let references: Vec<&str> = markers(&doc, MarkerType::Link).iter().map(|link| link.reference.as_str()).collect();
	assert_eq!(references, ["https://example.com/first", "https://example.com/second"]);
}

fn with_internal_link(uri: &str) -> Document {
	let first = format!(
		r#"<FixedPage Height="1000"><Glyphs FixedPage.NavigateUri="{uri}" OriginX="10" OriginY="20" FontRenderingEmSize="12" UnicodeString="Go on"/></FixedPage>"#
	);
	read(LINKED_PAGES, &[("1.fpage", &first), ("2.fpage", "<FixedPage/>")]).expect("read")
}

#[test]
fn a_link_to_a_name_in_the_file_leads_to_its_page() {
	for uri in ["#three", "Doc.fdoc#three", "../Doc.fdoc#three"] {
		assert_eq!(markers(&with_internal_link(uri), MarkerType::Link)[0].reference, "#page_1");
	}
}

#[test]
fn a_link_to_a_name_the_file_lacks_is_left_out() {
	assert!(markers(&with_internal_link("#gone"), MarkerType::Link).is_empty());
}

#[test]
fn a_web_address_with_a_fragment_stays_a_web_address() {
	assert_eq!(
		markers(&with_internal_link("https://example.com/#top"), MarkerType::Link)[0].reference,
		"https://example.com/#top"
	);
}

#[test]
fn fragments_that_name_nothing_leave_a_page_to_its_lines() {
	let page = format!("{}{}", at_x("a", "whethe", 72.0), at_x("b", "r it joins", 108.0));
	assert_eq!(structured(&page, "<StoryFragment/>").expect("read").buffer.content, "whether it joins\n");
}

#[test]
fn a_page_with_no_text_says_so() {
	let doc = unstructured(&["<FixedPage Height=\"1000\"/>".to_string()]);
	assert_eq!(doc.buffer.content, "This page has no text.\n");
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
	let doc = read_parts(
		&[
			("_rels/.rels", rels),
			("Sequence.fdseq", SEQUENCE),
			("Doc.fdoc", ONE_PAGE),
			("1.fpage", "<FixedPage/>"),
			("Metadata/props.xml", core),
		],
		&context(),
	)
	.expect("read");
	assert_eq!((doc.title.as_str(), doc.author.as_str()), ("The title", "The author"));
}

#[test]
fn the_title_falls_back_to_the_file_name() {
	let doc = read(ONE_PAGE, &[("1.fpage", "<FixedPage/>")]).expect("read");
	assert_eq!(doc.title, "sample");
}
