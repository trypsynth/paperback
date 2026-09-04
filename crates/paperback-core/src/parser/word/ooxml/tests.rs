use std::collections::HashMap;

use roxmltree::Document as XmlDocument;

use super::traverse;
use crate::{
	document::{DocumentBuffer, MarkerType},
	util::text::display_len,
};

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
	let mut buffer = DocumentBuffer::new();
	let mut headings = Vec::new();
	let mut id_positions = HashMap::new();
	let rels = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), false);
	assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker caption is the first-row text, no prefix");
	assert_eq!(table_marker.length, display_len("[Table]: Kop \u{1D11E}") + 1, "marker length in display units");
	assert!(table_marker.reference.contains("<td>Kop</td>"), "marker reference is the table HTML");
	// ON: full TSV "Kop\t𝄞".
	let mut buffer = DocumentBuffer::new();
	let mut headings = Vec::new();
	let mut id_positions = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), true);
	assert_eq!(buffer.content, "Kop\t\u{1D11E}\n");
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.length, display_len("Kop\t\u{1D11E}") + 1, "marker length spans the TSV");
}

/// Parse a single paragraph and return the buffer, so run-property (`<w:rPr>`) format markers
/// can be inspected. Test XML uses unnamespaced tags/attributes to match `attribute("val")`
/// (roxmltree matches on the local name here, mirroring the existing table test fixtures).
fn parse_run_props(xml: &str) -> DocumentBuffer {
	let xml_doc = XmlDocument::parse(xml).expect("valid xml");
	let mut buffer = DocumentBuffer::new();
	let mut headings = Vec::new();
	let mut id_positions = HashMap::new();
	let rels = HashMap::new();
	traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), false);
	buffer
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
