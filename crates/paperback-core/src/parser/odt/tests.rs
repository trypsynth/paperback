use std::collections::HashMap;

use roxmltree::Document as XmlDocument;

use super::{build_odt_format_style_map, traverse};
use crate::document::{DocLen, DocPos, DocumentBuffer, MarkerType};

/// The second cell holds a non-BMP character to catch cell text being mis-sliced with char offsets as byte indices.
#[test]
fn odt_table_emits_placeholder_when_off() {
	let xml = "<document><table><table-row><table-cell>Kop</table-cell><table-cell>\u{1D11E}</table-cell></table-row></table></document>";
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let format_style_map = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut id_positions, false, &format_style_map);
	assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.position, 0, "marker starts at buffer start");
	assert_eq!(table_marker.length, DocLen::of("[Table]: Kop \u{1D11E}") + 1.into(), "marker length in positions");
	assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker keeps the first-row caption");
	assert!(table_marker.reference.contains("<table"), "marker reference is the table HTML");
}

#[test]
fn odt_table_cell_id_registered_at_table_start() {
	let xml = "<document><p>before</p><table><table-row><table-cell><span id=\"anchor1\">Kop</span></table-cell></table-row></table></document>";
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let format_style_map = HashMap::new();
	for inline in [false, true] {
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, inline, &format_style_map);
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(
			id_positions.get("anchor1"),
			Some(&table_marker.position),
			"in-cell anchor id maps to the table start (inline={inline})"
		);
	}
}

#[test]
fn odt_table_emits_tsv_when_inline() {
	let xml = "<document><table><table-row><table-cell>Kop</table-cell><table-cell>\u{1D11E}</table-cell></table-row></table></document>";
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let format_style_map = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut id_positions, true, &format_style_map);
	assert_eq!(buffer.content, "Kop\t\u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.length, DocLen::of("Kop\t\u{1D11E}") + 1.into(), "marker length spans the TSV");
}

/// Names are bare because the parser matches local names only, ignoring namespace prefixes.
fn span_fixture(prop_name: &str, prop_value: &str, text: &str) -> String {
	format!(
		"<document><automatic-styles><style family=\"text\" name=\"T1\"><text-properties {prop_name}=\"{prop_value}\"/></style></automatic-styles><p><span style-name=\"T1\">{text}</span></p></document>"
	)
}

fn traverse_fixture(xml: &str) -> DocumentBuffer {
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let format_style_map = build_odt_format_style_map(xml_doc.root());
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut id_positions, false, &format_style_map);
	buffer
}

#[test]
fn odt_span_bold_style_adds_bold_marker() {
	let xml = span_fixture("font-weight", "bold", "bold text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "bold text\n");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, DocLen::of("bold text"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Italic && m.mtype != MarkerType::Underline));
}

#[test]
fn odt_span_italic_style_adds_italic_marker() {
	let xml = span_fixture("font-style", "italic", "italic text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "italic text\n");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, DocLen::of("italic text"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Bold && m.mtype != MarkerType::Underline));
}

#[test]
fn odt_span_underline_solid_style_adds_underline_marker() {
	let xml = span_fixture("text-underline-style", "solid", "underlined text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "underlined text\n");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Underline).expect("Underline marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, DocLen::of("underlined text"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Bold && m.mtype != MarkerType::Italic));
}

#[test]
fn odt_span_underline_none_style_adds_no_underline_marker() {
	let xml = span_fixture("text-underline-style", "none", "plain text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "plain text\n");
	assert!(
		buffer.markers.iter().all(|m| m.mtype != MarkerType::Underline),
		"text-underline-style=none must not be treated as underlined"
	);
}

#[test]
fn odt_span_combined_bold_and_italic_style_adds_both_markers() {
	let xml = "<document><automatic-styles><style family=\"text\" name=\"T1\"><text-properties font-weight=\"bold\" font-style=\"italic\"/></style></automatic-styles><p><span style-name=\"T1\">both</span></p></document>";
	let buffer = traverse_fixture(xml);
	assert_eq!(buffer.content, "both\n");
	let bold = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
	let italic = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
	assert_eq!(bold.position, 0);
	assert_eq!(bold.length, DocLen::of("both"));
	assert_eq!(italic.position, 0);
	assert_eq!(italic.length, DocLen::of("both"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Underline));
}

#[test]
fn odt_span_with_unknown_style_falls_through_unformatted() {
	let xml = "<document><automatic-styles><style family=\"text\" name=\"T1\"><text-properties font-weight=\"bold\"/></style></automatic-styles><p><span style-name=\"Unknown\">plain</span></p></document>";
	let buffer = traverse_fixture(xml);
	assert_eq!(buffer.content, "plain\n");
	assert!(
		buffer
			.markers
			.iter()
			.all(|m| m.mtype != MarkerType::Bold && m.mtype != MarkerType::Italic && m.mtype != MarkerType::Underline),
		"unknown style-name must not add any formatting marker"
	);
}

#[test]
fn odt_span_without_style_name_falls_through_unformatted() {
	let xml = "<document><p><span>plain</span></p></document>";
	let buffer = traverse_fixture(xml);
	assert_eq!(buffer.content, "plain\n");
	assert!(
		buffer
			.markers
			.iter()
			.all(|m| m.mtype != MarkerType::Bold && m.mtype != MarkerType::Italic && m.mtype != MarkerType::Underline),
		"span without style-name must not add any formatting marker"
	);
}

const NAMESPACES: &str = r#"xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0" xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0" xmlns:dc="http://purl.org/dc/elements/1.1/""#;

fn walk_text(body: &str) -> (DocumentBuffer, HashMap<String, DocPos>) {
	let xml = format!(
		"<office:document-content {NAMESPACES}><office:body><office:text>{body}</office:text></office:body></office:document-content>"
	);
	let xml_doc = XmlDocument::parse(&xml).expect("valid xml");
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut id_positions, false, &HashMap::new());
	(buffer, id_positions)
}

/// A footnote read where it sits breaks its sentence in two.
#[test]
fn a_footnote_is_cited_in_its_sentence_and_written_after_the_text() {
	let (buffer, ids) = walk_text(
		"<text:p>Water boils<text:note text:id=\"ftn1\" text:note-class=\"footnote\"><text:note-citation>1</text:note-citation><text:note-body><text:p>At sea level.</text:p></text:note-body></text:note> at 100 degrees.</text:p><text:p>More text.</text:p>",
	);
	assert_eq!(buffer.content, "Water boils[1] at 100 degrees.\nMore text.\nNotes\n1. At sea level.\n");
	let link = buffer.markers.iter().find(|m| m.mtype == MarkerType::Link).expect("citation link");
	assert_eq!(
		(link.position, link.text.as_str(), link.reference.as_str()),
		(DocLen::of("Water boils").as_pos(), "[1]", "#ftn1")
	);
	assert_eq!(ids["ftn1"], buffer.content.find("1. At sea").unwrap());
	assert!(buffer.markers.iter().any(|m| m.mtype == MarkerType::Heading1 && m.text == "Notes"));
}

/// A comment read where it sits breaks the sentence, and dropped it loses what the reviewer said.
#[test]
fn a_comment_goes_after_the_text_and_links_back_to_its_words() {
	let (buffer, ids) = walk_text(
		"<text:p>Sales <office:annotation office:name=\"c1\"><dc:creator>Ann</dc:creator><dc:date>2026-10-23</dc:date><text:p>Check this figure.</text:p></office:annotation>grew fast<office:annotation-end office:name=\"c1\"/> this year.</text:p>",
	);
	assert_eq!(
		buffer.content,
		"Sales grew fast this year.\nComments\nAnn on \u{201c}grew fast\u{201d}: Check this figure.\n"
	);
	let link = buffer.markers.iter().find(|m| m.mtype == MarkerType::Link).expect("comment link");
	assert_eq!(link.reference, "#comment-1");
	assert_eq!(ids["comment-1"], DocLen::of("Sales ").as_pos());
}

#[test]
fn a_comment_on_a_point_names_only_its_author() {
	let (buffer, _) = walk_text("<text:p>Done.<office:annotation><text:p>Nice.</text:p></office:annotation></text:p>");
	assert_eq!(buffer.content, "Done.\nComments\nComment: Nice.\n");
}

/// ODF writes repeated spaces, tabs and line breaks as elements, and without them words run together.
#[test]
fn spacing_elements_keep_words_apart() {
	let (buffer, _) = walk_text("<text:p>Name:<text:tab/>Ann<text:line-break/>Age:<text:s text:c=\"2\"/>40</text:p>");
	assert_eq!(buffer.content, "Name:\tAnn\nAge:  40\n");
}

#[test]
fn a_picture_reads_as_its_placeholder_and_can_be_found() {
	let (buffer, _) =
		walk_text("<text:p>See <draw:frame><draw:image/><svg:title>A map</svg:title></draw:frame> below.</text:p>");
	assert_eq!(buffer.content, "See [Image: A map] below.\n");
	let image = buffer.markers.iter().find(|m| m.mtype == MarkerType::Image).expect("image marker");
	assert_eq!((image.position, image.text.as_str()), (DocLen::of("See ").as_pos(), "A map"));
}
