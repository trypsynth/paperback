use std::collections::HashMap;

use roxmltree::Document as XmlDocument;

use super::{build_odt_format_style_map, traverse};
use crate::{
	document::{DocumentBuffer, MarkerType},
	util::text::display_len,
};

/// OFF mode: an ODT table emits a `"[Table]: <first row>"` placeholder. The second cell holds a
/// non-ASCII character (U+1D11E, G Clef, non-BMP) to prove the cell-text extraction no longer
/// mis-slices the display buffer with display-unit offsets as byte indices.
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
	assert_eq!(table_marker.length, display_len("[Table]: Kop \u{1D11E}") + 1, "marker length in display units");
	assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker keeps the first-row caption");
	assert!(table_marker.reference.contains("<table"), "marker reference is the table HTML");
}

/// An `id` attribute on an element nested inside a table cell must be registered in
/// `id_positions` at the table's start position, so internal links to that anchor navigate to
/// the table. Holds in both OFF and ON modes (registration happens before the cells collapse).
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

/// ON mode: the same ODT table emits the full TSV instead of the placeholder.
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
	assert_eq!(table_marker.length, display_len("Kop\t\u{1D11E}") + 1, "marker length spans the TSV");
}

/// Builds the standard test fixture: an `<automatic-styles>` block defining style `"T1"` with the
/// given text-properties attribute (name, value), wrapping a `<span style-name="T1">` around
/// `text`. Uses bare/local tag and attribute names, since this file's roxmltree usage strips
/// namespace prefixes, so test XML omits them too, matching real ODT content.xml parsing.
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
	assert_eq!(marker.length, display_len("bold text"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Italic && m.mtype != MarkerType::Underline));
}

#[test]
fn odt_span_italic_style_adds_italic_marker() {
	let xml = span_fixture("font-style", "italic", "italic text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "italic text\n");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, display_len("italic text"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Bold && m.mtype != MarkerType::Underline));
}

#[test]
fn odt_span_underline_solid_style_adds_underline_marker() {
	let xml = span_fixture("text-underline-style", "solid", "underlined text");
	let buffer = traverse_fixture(&xml);
	assert_eq!(buffer.content, "underlined text\n");
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Underline).expect("Underline marker");
	assert_eq!(marker.position, 0);
	assert_eq!(marker.length, display_len("underlined text"));
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
	assert_eq!(bold.length, display_len("both"));
	assert_eq!(italic.position, 0);
	assert_eq!(italic.length, display_len("both"));
	assert!(buffer.markers.iter().all(|m| m.mtype != MarkerType::Underline));
}

/// A span with no matching/known style-name (or a style resolving to no formatting) falls through
/// to exactly today's behavior: text renders, but no bold/italic/underline marker is added.
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

/// A span with no `style-name` attribute at all also falls through unformatted.
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
