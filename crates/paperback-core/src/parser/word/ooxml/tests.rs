use std::collections::HashMap;

use roxmltree::Document as XmlDocument;

use super::{super::numbering::Numbering, Annotations, PageSource, Walk, number_pages, traverse};
use crate::{
	document::{DocumentBuffer, MarkerType},
	parser::util::ooxml::Relationships,
	util::text::display_len,
};

/// Walks a parsed `document.xml` the way `parse_ooxml_from_archive` does, with the given
/// numbering definitions, and returns what it wrote.
fn walk(xml_doc: &XmlDocument, render_tables_inline: bool, numbering: Numbering) -> DocumentBuffer {
	let mut buffer = DocumentBuffer::new();
	let mut headings = Vec::new();
	let mut id_positions = HashMap::new();
	let rels = Relationships::default();
	let style_heading_map = HashMap::new();
	let mut walk = Walk {
		buffer: &mut buffer,
		headings: &mut headings,
		id_positions: &mut id_positions,
		rels: &rels,
		style_heading_map: &style_heading_map,
		render_tables_inline,
		numbering,
		list: None,
		page_source: PageSource::for_document(xml_doc.input_text()),
		annotations: Annotations::default(),
	};
	traverse(xml_doc.root(), &mut walk);
	walk.end_list();
	number_pages(&mut buffer, 0, 0);
	buffer
}

/// Parse a Word table. The second cell contains U+1D11E (MUSICAL SYMBOL G CLEF, non-BMP,
/// UTF-16 width 2) to lock the display-unit arithmetic. OFF mode emits the placeholder; ON mode
/// emits the full TSV. In both cases the Table marker keeps the caption as text and its length
/// equals the emitted display extent.
#[test]
fn word_table_emits_placeholder_or_tsv_by_flag() {
	// Minimal OOXML XML: one table with one row, two cells.
	let xml = r"<document><body>
		<tbl>
			<tr>
				<tc><p><r><t>Kop</t></r></p></tc>
				<tc><p><r><t>&#x1D11E;</t></r></p></tc>
			</tr>
		</tbl>
	</body></document>";
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	// OFF: placeholder "[Table]: Kop 𝄞".
	let buffer = walk(&xml_doc, false, Numbering::default());
	assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker caption is the first-row text, no prefix");
	assert_eq!(table_marker.length, display_len("[Table]: Kop \u{1D11E}") + 1, "marker length in display units");
	assert!(table_marker.reference.contains("<td>Kop</td>"), "marker reference is the table HTML");
	// ON: full TSV "Kop\t𝄞".
	let buffer = walk(&xml_doc, true, Numbering::default());
	assert_eq!(buffer.content, "Kop\t\u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.length, display_len("Kop\t\u{1D11E}") + 1, "marker length spans the TSV");
}

/// Parse a single paragraph and return the buffer, so run-property (`<w:rPr>`) format markers
/// can be inspected. Test XML uses unnamespaced tags/attributes to match `attribute("val")`
/// (roxmltree matches on the local name here, mirroring the existing table test fixtures).
fn parse_run_props(xml: &str) -> DocumentBuffer {
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	walk(&xml_doc, false, Numbering::default())
}

/// Word's automatic numbering is generated, not stored in the text: a numbered procedure used to
/// come out as unnumbered paragraphs. Each item now carries its label and a list-item marker,
/// and the run of items is one list that ends at the next ordinary paragraph.
#[test]
fn numbered_paragraphs_get_their_labels_and_list_markers() {
	let numbering = Numbering::load(
		Some(
			r#"<numbering><abstractNum abstractNumId="7"><lvl ilvl="0"><start val="1"/><numFmt val="decimal"/><lvlText val="%1."/></lvl></abstractNum><num numId="4"><abstractNumId val="7"/></num></numbering>"#,
		),
		None,
	);
	let xml = r#"<document><body>
		<p><r><t>Before.</t></r></p>
		<p><pPr><numPr><ilvl val="0"/><numId val="4"/></numPr></pPr><r><t>Press Power.</t></r></p>
		<p><pPr><numPr><ilvl val="0"/><numId val="4"/></numPr></pPr><r><t>Wait for the beep.</t></r></p>
		<p><r><t>After.</t></r></p>
	</body></document>"#;
	let buffer = walk(&XmlDocument::parse(xml).expect("valid xml"), false, numbering);
	assert_eq!(buffer.content, "Before.\n1. Press Power.\n2. Wait for the beep.\nAfter.\n");
	let items: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::ListItem).collect();
	assert_eq!(items.len(), 2);
	assert_eq!((items[0].position, items[0].text.as_str(), items[0].level), (8, "1. Press Power.", 1));
	let list = buffer.markers.iter().find(|m| m.mtype == MarkerType::List).expect("List marker");
	assert_eq!((list.position, list.level), (8, 2));
	assert_eq!(list.length, display_len("1. Press Power.\n2. Wait for the beep.\n"));
}

#[test]
fn run_bold_property_emits_bold_marker() {
	let buffer = parse_run_props(r"<document><body><p><r><rPr><b/></rPr><t>bold</t></r></p></body></document>");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, display_len("bold"));
}

#[test]
fn run_italic_property_emits_italic_marker() {
	let buffer = parse_run_props(r"<document><body><p><r><rPr><i/></rPr><t>italic</t></r></p></body></document>");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, display_len("italic"));
}

#[test]
fn run_underline_property_emits_underline_marker() {
	let buffer =
		parse_run_props(r#"<document><body><p><r><rPr><u val="single"/></rPr><t>under</t></r></p></body></document>"#);
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Underline).expect("Underline marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, display_len("under"));
}

#[test]
fn run_bold_and_italic_together_emit_both_spanning_same_range() {
	let buffer = parse_run_props(r"<document><body><p><r><rPr><b/><i/></rPr><t>both</t></r></p></body></document>");
	let bold = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	let italic = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
	assert_eq!(bold.position, italic.position);
	assert_eq!(bold.length, italic.length);
	assert_eq!(bold.position, 0);
	assert_eq!(bold.length, display_len("both"));
}

#[test]
fn run_underline_none_is_not_underlined() {
	let buffer =
		parse_run_props(r#"<document><body><p><r><rPr><u val="none"/></rPr><t>plain</t></r></p></body></document>"#);
	assert!(
		!buffer.markers.iter().any(|m| m.mtype == MarkerType::Underline),
		"u val=none must not produce an Underline marker"
	);
}

#[test]
fn run_bold_false_cancels_bold() {
	let buffer =
		parse_run_props(r#"<document><body><p><r><rPr><b val="false"/></rPr><t>plain</t></r></p></body></document>"#);
	assert!(!buffer.markers.iter().any(|m| m.mtype == MarkerType::Bold), "b val=false must not produce a Bold marker");
}

#[test]
fn run_bold_zero_cancels_bold() {
	let buffer =
		parse_run_props(r#"<document><body><p><r><rPr><b val="0"/></rPr><t>plain</t></r></p></body></document>"#);
	assert!(!buffer.markers.iter().any(|m| m.mtype == MarkerType::Bold), "b val=0 must not produce a Bold marker");
}

/// The offset of a format marker must be computed in DISPLAY units, not byte length. A paragraph
/// beginning with a multi-byte (but display-stable) character before the bold run would place the
/// Bold marker at the wrong position if `String::len()` (bytes) were used instead of `display_len`.
#[test]
fn run_format_offset_uses_display_units_not_bytes() {
	// "é" is 2 bytes in UTF-8 but 1 display unit (single UTF-16 code unit / one char).
	let buffer =
		parse_run_props(r"<document><body><p><r><t>é</t></r><r><rPr><b/></rPr><t>bold</t></r></p></body></document>");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	assert_eq!(marker.position, display_len("é"), "offset must be display-unit, not byte length");
	assert_ne!(marker.position, "é".len(), "byte length (2) would be the bug");
	assert_eq!(marker.length, display_len("bold"));
}

/// A paragraph starting with a whitespace-only unformatted run before a bold run must not
/// desync the Bold marker's offset. `process_paragraph` only appends the TRIMMED paragraph
/// text to the buffer, so the leading spaces never make it into the final content - the
/// bold run's offset must be shifted left by the same amount that gets trimmed, or the
/// marker ends up pointing past the start of "bold" into the wrong text.
#[test]
fn run_format_offset_accounts_for_leading_whitespace_trim() {
	let buffer = parse_run_props(
		r#"<document><body><p><r><t xml:space="preserve">  </t></r><r><rPr><b/></rPr><t>bold</t></r></p></body></document>"#,
	);
	assert_eq!(buffer.content, "bold\n", "leading whitespace run must be trimmed from the final content");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	assert_eq!(marker.position, 0, "Bold marker must point at the start of the trimmed content");
	assert_eq!(marker.length, display_len("bold"));
}

fn pages(buffer: &DocumentBuffer) -> Vec<(usize, String)> {
	let mut pages: Vec<(usize, String)> = buffer
		.markers
		.iter()
		.filter(|m| m.mtype == MarkerType::PageBreak)
		.map(|m| (m.position, m.text.clone()))
		.collect();
	pages.sort();
	pages
}

/// #1121: a .docx has no pages of its own, so they come from where Word last broke them, and the first page, which Word records no break for, starts the document.
#[test]
fn pages_start_where_word_last_broke_them() {
	let xml = r"<document><body><p><r><t>Cover</t></r></p><p><r><lastRenderedPageBreak/><t>Contents</t></r></p><p><r><t>More</t></r></p><p><r><lastRenderedPageBreak/><t>Body</t></r></p></body></document>";
	let buffer = parse_run_props(xml);
	assert_eq!(
		buffer.content,
		"Cover
Contents
More
Body
"
	);
	assert_eq!(pages(&buffer), [(0, "Page 1".to_string()), (6, "Page 2".to_string()), (20, "Page 3".to_string())]);
}

/// A document Word never laid out records no breaks, and claiming it is one page would be a guess.
#[test]
fn a_document_with_no_recorded_breaks_has_no_pages() {
	let buffer = parse_run_props(r"<document><body><p><r><t>One</t></r></p><p><r><t>Two</t></r></p></body></document>");
	assert_eq!(pages(&buffer), []);
}

/// A page that breaks partway through a paragraph starts at the words after the break.
#[test]
fn a_page_can_start_inside_a_paragraph() {
	let xml = r"<document><body><p><r><t>Start</t></r></p><p><r><t>end of one </t></r><r><lastRenderedPageBreak/><t>top of two</t></r></p></body></document>";
	let buffer = parse_run_props(xml);
	assert_eq!(
		pages(&buffer)[1],
		(
			display_len(
				"Start
end of one "
			),
			"Page 2".to_string()
		)
	);
}

/// A page inside a table starts with the table, so the pages after it keep their numbers.
#[test]
fn a_page_that_starts_inside_a_table_starts_with_it() {
	let xml = r"<document><body><p><r><t>Intro</t></r></p><tbl><tr><tc><p><r><t>A</t></r></p></tc></tr><tr><tc><p><r><lastRenderedPageBreak/><t>B</t></r></p></tc></tr></tbl><p><r><lastRenderedPageBreak/><t>After</t></r></p></body></document>";
	let buffer = parse_run_props(xml);
	let after = buffer.content.find("After").unwrap();
	assert_eq!(pages(&buffer), [(0, "Page 1".to_string()), (6, "Page 2".to_string()), (after, "Page 3".to_string())]);
}

/// Links and bookmarks are display positions, which differ from byte offsets once a character outside the Basic Multilingual Plane, such as an emoji, comes before them. The leading spaces are trimmed away, so they also check the positions move back with the trim.
#[test]
fn links_and_bookmarks_land_on_their_text_after_an_emoji() {
	let xml = "<document><body><p><r><t xml:space=\"preserve\">  \u{1F600} see </t></r><bookmarkStart name=\"here\"/><hyperlink anchor=\"here\"><r><t>this</t></r></hyperlink><r><t xml:space=\"preserve\"> and </t></r><r><fldChar fldCharType=\"begin\"/></r><r><instrText> HYPERLINK \"https://example.com\" </instrText></r><r><fldChar fldCharType=\"separate\"/></r><r><t>that</t></r><r><fldChar fldCharType=\"end\"/></r></p></body></document>";
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let mut buffer = DocumentBuffer::new();
	let mut headings = Vec::new();
	let mut id_positions = HashMap::new();
	let rels = Relationships::default();
	let style_heading_map = HashMap::new();
	let mut walk = Walk {
		buffer: &mut buffer,
		headings: &mut headings,
		id_positions: &mut id_positions,
		rels: &rels,
		style_heading_map: &style_heading_map,
		render_tables_inline: false,
		numbering: Numbering::default(),
		list: None,
		page_source: PageSource::for_document(xml),
		annotations: Annotations::default(),
	};
	traverse(xml_doc.root(), &mut walk);
	let this = display_len("\u{1F600} see ");
	assert_eq!(id_positions["here"], this, "the bookmark is where its text starts");
	let links: Vec<(usize, &str)> =
		buffer.markers.iter().filter(|m| m.mtype == MarkerType::Link).map(|m| (m.position, m.text.as_str())).collect();
	assert_eq!(links[0], (this, "this"));
	assert_eq!(links[1].0, display_len("\u{1F600} see this and "));
	assert_eq!(buffer.content, "\u{1F600} see this and that\n");
}

/// A file Word never laid out, such as one from Google Docs or pandoc, has only the breaks its author wrote, and they are its pages.
#[test]
fn a_document_word_never_laid_out_takes_its_pages_from_its_explicit_breaks() {
	let xml = r#"<document><body><p><r><t>Cover</t></r></p><p><r><br type="page"/></r></p><p><r><t>Contents</t></r></p><p><pPr><pageBreakBefore/></pPr><r><t>Body</t></r></p></body></document>"#;
	let buffer = parse_run_props(xml);
	let body = buffer.content.find("Body").unwrap();
	let contents = buffer.content.find("Contents").unwrap();
	assert_eq!(
		pages(&buffer),
		[(0, "Page 1".to_string()), (contents, "Page 2".to_string()), (body, "Page 3".to_string())]
	);
}

/// A break partway through a paragraph starts the page at the words after it, and one closing the document starts no page at all.
#[test]
fn an_explicit_break_inside_a_paragraph_starts_the_page_at_the_words_after_it() {
	let xml = r#"<document><body><p><r><t>one</t><br type="page"/><t>two</t></r></p><p><r><t>end</t><br type="page"/></r></p></body></document>"#;
	let buffer = parse_run_props(xml);
	assert_eq!(
		pages(&buffer),
		[
			(0, "Page 1".to_string()),
			(
				display_len(
					"one
"
				),
				"Page 2".to_string()
			)
		]
	);
}

/// A section that starts a new page does so after the paragraph that ends it, unless it is continuous.
#[test]
fn a_new_page_section_starts_a_page_and_a_continuous_one_does_not() {
	let xml = r#"<document><body><p><pPr><sectPr><type val="continuous"/></sectPr></pPr><r><t>A</t></r></p><p><pPr><sectPr/></pPr><r><t>B</t></r></p><p><r><t>C</t></r></p></body></document>"#;
	let buffer = parse_run_props(xml);
	let c = buffer.content.find('C').unwrap();
	assert_eq!(pages(&buffer), [(0, "Page 1".to_string()), (c, "Page 2".to_string())]);
}

/// Word records a rendered break after each explicit one, so counting both would give every explicit break two pages.
#[test]
fn explicit_breaks_are_not_counted_on_top_of_rendered_ones() {
	let xml = r#"<document><body><p><r><t>One</t><br type="page"/></r></p><p><r><lastRenderedPageBreak/><t>Two</t></r></p></body></document>"#;
	let buffer = parse_run_props(xml);
	assert_eq!(pages(&buffer), [(0, "Page 1".to_string()), (4, "Page 2".to_string())]);
}

fn docx(entries: &[(&str, &str)]) -> zip::ZipArchive<std::io::Cursor<Vec<u8>>> {
	use std::io::Write;
	let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
	for (name, contents) in entries {
		writer.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
		writer.write_all(contents.as_bytes()).unwrap();
	}
	zip::ZipArchive::new(std::io::Cursor::new(writer.finish().unwrap().into_inner())).unwrap()
}

/// #1123: Word keeps its notes in parts of their own, and reading only the document lost both the numbers and the notes. A citation now links to its note, written after the text and numbered as Word shows it.
#[test]
fn footnotes_and_endnotes_are_cited_in_the_text_and_written_after_it() {
	let document = r#"<document><body><p><r><t>Water boils</t></r><r><footnoteReference id="2"/></r><r><t> at sea level</t></r><r><endnoteReference id="1"/></r><r><t>.</t></r></p><p><r><t>Marked</t></r><r><footnoteReference customMarkFollows="1" id="3"/><t>*</t></r></p></body></document>"#;
	let footnotes = r#"<footnotes><footnote type="separator" id="-1"><p><r><separator/></r></p></footnote><footnote id="2"><p><r><footnoteRef/></r><r><t xml:space="preserve"> At 100 degrees.</t></r></p></footnote><footnote id="3"><p><r><t>A starred note.</t></r></p></footnote></footnotes>"#;
	let endnotes =
		r#"<endnotes><endnote id="1"><p><r><endnoteRef/></r><r><t>See the appendix.</t></r></p></endnote></endnotes>"#;
	let mut archive =
		docx(&[("word/document.xml", document), ("word/footnotes.xml", footnotes), ("word/endnotes.xml", endnotes)]);
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut headings = Vec::new();
	super::parse_ooxml_from_archive(&mut archive, &mut buffer, &mut id_positions, &mut headings, false).unwrap();
	assert_eq!(
		buffer.content,
		"Water boils[1] at sea level[i].\nMarked[*]\nNotes\n1. At 100 degrees.\ni. See the appendix.\n* A starred note.\n"
	);
	let links: Vec<(usize, &str)> = buffer
		.markers
		.iter()
		.filter(|m| m.mtype == MarkerType::Link)
		.map(|m| (m.position, m.reference.as_str()))
		.collect();
	assert_eq!(links.len(), 3);
	let (first, reference) = links[0];
	assert_eq!(first, display_len("Water boils"));
	let target = id_positions[reference.trim_start_matches('#')];
	assert_eq!(target, buffer.content.find("1. At").unwrap());
	assert!(headings.iter().any(|heading| heading.text == "Notes"));
}

/// #1123: Word comments used to be dropped. They now follow the text, naming their author and the words they are on, and link back to those words.
#[test]
fn comments_follow_the_text_with_their_authors_and_link_back() {
	let document = r#"<document><body><p><r><t xml:space="preserve">Sales </t></r><commentRangeStart id="0"/><r><t>grew fast</t></r><commentRangeEnd id="0"/><r><commentReference id="0"/></r><r><t xml:space="preserve"> this year.</t></r></p><p><r><t>Done.</t></r><r><commentReference id="1"/></r></p></body></document>"#;
	let comments = r#"<comments><comment id="0" author="Ann" date="2026-10-23T00:00:00Z"><p><r><annotationRef/></r><r><t>Check this figure.</t></r></p></comment><comment id="1"><p><r><t>Nice.</t></r></p></comment></comments>"#;
	let mut archive = docx(&[("word/document.xml", document), ("word/comments.xml", comments)]);
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut headings = Vec::new();
	super::parse_ooxml_from_archive(&mut archive, &mut buffer, &mut id_positions, &mut headings, false).unwrap();
	assert_eq!(
		buffer.content,
		"Sales grew fast this year.\nDone.\nComments\nAnn on \u{201c}grew fast\u{201d}: Check this figure.\nComment: Nice.\n"
	);
	let link = buffer.markers.iter().find(|m| m.mtype == MarkerType::Link).expect("comment link");
	assert_eq!(id_positions[link.reference.trim_start_matches('#')], display_len("Sales "));
}

/// #1123: tracked insertions were dropped along with deletions, so the text matched neither version: "Kept text  and  end". It now reads as it does with every change accepted.
#[test]
fn tracked_changes_read_as_accepted() {
	let buffer = parse_run_props(
		r#"<document><body><p><r><t xml:space="preserve">Kept text </t></r><del><r><delText xml:space="preserve">old </delText></r></del><ins><r><t xml:space="preserve">new </t></r></ins><r><t>and end.</t></r></p></body></document>"#,
	);
	assert_eq!(buffer.content, "Kept text new and end.\n");
}

/// Forms and templates put their text in content controls, and smart tags and simple fields wrap runs too; all of it was lost.
#[test]
fn text_inside_content_controls_smart_tags_and_fields_is_read() {
	let buffer = parse_run_props(
		r#"<document><body><p><r><t xml:space="preserve">Name: </t></r><sdt><sdtPr><alias val="Name"/></sdtPr><sdtContent><r><t>Ann Smith</t></r></sdtContent></sdt><r><t xml:space="preserve">, in </t></r><smartTag><r><t>Paris</t></r></smartTag><r><t xml:space="preserve">, page </t></r><fldSimple instr="PAGE"><r><t>3</t></r></fldSimple></p></body></document>"#,
	);
	assert_eq!(buffer.content, "Name: Ann Smith, in Paris, page 3\n");
}

/// A table cell read only the runs directly in its paragraphs, losing links, content controls and nested tables.
#[test]
fn a_table_cell_reads_links_content_controls_and_nested_tables() {
	let buffer = parse_run_props(
		r#"<document><body><tbl><tr><tc><p><hyperlink anchor="x"><r><t>Linked</t></r></hyperlink></p></tc><tc><p><sdt><sdtContent><r><t>Filled</t></r></sdtContent></sdt></p></tc><tc><tbl><tr><tc><p><r><t>Inner</t></r></p></tc></tr></tbl></tc></tr></tbl></body></document>"#,
	);
	assert!(buffer.content.starts_with("[Table]: Linked Filled Inner"), "{:?}", buffer.content);
}
