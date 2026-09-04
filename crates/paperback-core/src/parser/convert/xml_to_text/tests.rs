use rstest::rstest;

use super::*;
use crate::document::MarkerType;

#[test]
fn test_link_collection() {
	let xml = "<root><body><a href=\"https://example.com\">Hello   world</a></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	assert_eq!(links[0].text, "Hello world");
	assert_eq!(links[0].reference, "https://example.com");
	assert_eq!(converter.get_text(), "Hello world");
}

// SMIL audio and NCX targets anchor to an id expecting it to mark the start of what it
// narrates, so a link's own id must resolve to where its text starts, not where it ends.
#[test]
fn link_id_position_is_the_start_of_its_text_not_the_end() {
	let xml = r##"<root><body><p>Before. <a id="lnk1" href="#x">Link text</a> After.</p></body></root>"##;
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	let link_offset = links[0].offset;
	assert_eq!(converter.get_id_positions().get("lnk1").copied(), Some(link_offset));
	assert_eq!(&converter.get_text()[link_offset..link_offset + "Link text".len()], "Link text");
}

#[test]
fn heading_id_position_is_the_start_of_the_heading_not_the_previous_line() {
	let xml = r##"<root><body><p>Before paragraph.</p><h2 id="hdr1">Chapter One</h2></body></root>"##;
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	let heading_offset = headings[0].offset;
	assert_eq!(converter.get_id_positions().get("hdr1").copied(), Some(heading_offset));
	assert_eq!(&converter.get_text()[heading_offset..heading_offset + "Chapter One".len()], "Chapter One");
}

// `<a>` emits its text in one go and skips its children, so ids on elements inside a link
// were never recorded, losing the audio clip of any DAISY SMIL par anchored to one.
#[test]
fn ids_inside_a_link_are_recorded_at_the_links_start() {
	let xml = r##"<root><body><p>Before. <a href="#x"><em id="em1">Title Page</em></a></p></body></root>"##;
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let link_offset = converter.get_links()[0].offset;
	assert_eq!(converter.get_id_positions().get("em1").copied(), Some(link_offset));
	assert_eq!(&converter.get_text()[link_offset..link_offset + "Title Page".len()], "Title Page");
}

// Same skipped-subtree problem for tables, which emit their text through a helper.
#[test]
fn ids_inside_a_table_are_recorded_at_the_tables_start() {
	let xml = r#"<root><body><p>Before.</p><table id="t1"><tr><td id="cell1">Value</td></tr></table></body></root>"#;
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let table_offset = converter.get_id_positions().get("t1").copied().expect("table id recorded");
	assert_eq!(converter.get_id_positions().get("cell1").copied(), Some(table_offset));
	assert_ne!(table_offset, 0, "table should start after the preceding paragraph");
}

// Same off-by-one-line bug as headings/list items/tables/`<hr>`, which
// `handle_list_start_xml` was missing.
#[test]
fn list_id_position_is_the_start_of_the_list_not_the_previous_line() {
	let xml = r##"<root><body><p>Before paragraph.</p><ul id="lst1"><li>One</li><li>Two</li></ul></body></root>"##;
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let lists = converter.get_lists();
	assert_eq!(lists.len(), 1);
	let list_offset = lists[0].offset;
	assert_eq!(converter.get_id_positions().get("lst1").copied(), Some(list_offset));
	assert_ne!(list_offset, 0, "list offset should be after the preceding paragraph, not at document start");
}

#[test]
fn test_heading_normalization() {
	let xml = "<root><body><h2>  Hello \n world </h2></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(headings[0].level, 2);
	assert_eq!(headings[0].text, "Hello world");
}

#[test]
fn test_ordered_list_metadata() {
	let xml = "<root><body><ol start=\"2\"><li>One</li><li>Two</li></ol></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let lists = converter.get_lists();
	let items = converter.get_list_items();
	assert_eq!(lists.len(), 1);
	assert_eq!(lists[0].item_count, 2);
	assert_eq!(items.len(), 2);
	assert_eq!(items[0].level, 1);
	assert_eq!(items[0].text, "One");
	assert_eq!(items[1].text, "Two");
}

#[rstest]
#[case("h1", 1)]
#[case("h2", 2)]
#[case("h3", 3)]
#[case("h4", 4)]
#[case("h5", 5)]
#[case("h6", 6)]
fn heading_levels_h1_to_h6(#[case] tag: &str, #[case] expected_level: i32) {
	let xml = format!("<root><body><{tag}>Title</{tag}></body></root>");
	let mut converter = XmlToText::new();
	assert!(converter.convert(&xml));
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(headings[0].level, expected_level);
	assert_eq!(headings[0].text, "Title");
}

#[test]
fn hr_produces_separator() {
	let xml = "<root><body><p>Before</p><hr/><p>After</p></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	assert_eq!(converter.get_separators().len(), 1);
}

#[test]
fn unordered_list_items_have_level_one() {
	let xml = "<root><body><ul><li>First</li><li>Second</li></ul></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let items = converter.get_list_items();
	assert_eq!(items.len(), 2);
	assert_eq!(items[0].level, 1);
	assert_eq!(items[1].level, 1);
	assert_eq!(items[0].text, "First");
}

#[test]
fn nested_list_increments_level() {
	let xml = "<root><body><ul><li>A</li><ul><li>B</li></ul></ul></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let items = converter.get_list_items();
	assert!(items.len() >= 2, "expected at least two list items");
	let level_a = items.iter().find(|i| i.text == "A").map_or(0, |i| i.level);
	let level_b = items.iter().find(|i| i.text == "B").map_or(0, |i| i.level);
	assert!(level_b > level_a, "nested item should have a higher level");
}

#[test]
fn table_is_detected() {
	let xml = "<root><body><table><tr><td>Cell</td></tr></table></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	assert_eq!(converter.get_tables().len(), 1);
}

#[test]
fn find_anchor_byte_offset_locates_block_containing_position() {
	let xml = "<root><body><p>First paragraph.</p><p>Second paragraph.</p></body></root>";
	// Text output: "First paragraph.\nSecond paragraph." (second paragraph starts at 17).
	let mut converter = XmlToText::new();
	let offset = converter.find_anchor_byte_offset(xml, 20).expect("offset for position in second paragraph");
	assert!(xml[offset..].starts_with("<p>Second"), "got offset {offset}: {}", &xml[offset..]);
	let offset = converter.find_anchor_byte_offset(xml, 5).expect("offset for position in first paragraph");
	assert!(xml[offset..].starts_with("<p>First"), "got offset {offset}: {}", &xml[offset..]);
}

#[test]
fn find_anchor_byte_offset_at_position_zero_uses_first_body_element() {
	let xml = "<root><head><title>T</title></head><body><p>First.</p></body></root>";
	let mut converter = XmlToText::new();
	let offset = converter.find_anchor_byte_offset(xml, 0).expect("offset at start");
	assert!(xml[offset..].starts_with("<p>First."), "got offset {offset}: {}", &xml[offset..]);
}

#[test]
fn find_anchor_byte_offset_picks_nearest_inline_element() {
	let xml = "<root><body><p>Start <em>middle</em> end of line</p></body></root>";
	// Position inside " end of line". Nearest preceding element start is <em>.
	let mut converter = XmlToText::new();
	let offset = converter.find_anchor_byte_offset(xml, 16).expect("offset for position after em");
	assert!(xml[offset..].starts_with("<em>"), "got offset {offset}: {}", &xml[offset..]);
}

#[test]
fn find_anchor_byte_offset_returns_none_for_invalid_xml() {
	let mut converter = XmlToText::new();
	assert_eq!(converter.find_anchor_byte_offset("<p>broken", 0), None);
}

#[test]
fn inject_anchor_at_position_inserts_span_before_block() {
	let xml = "<root><body><p>First paragraph.</p><p>Second paragraph.</p></body></root>";
	let result = inject_anchor_at_position(xml, 20, "reading-pos").expect("injection succeeds");
	assert!(result.contains(r#"</p><span id="reading-pos"></span><p>Second paragraph.</p>"#), "got: {result}");
}

#[test]
fn inject_anchor_at_position_returns_none_for_invalid_xml() {
	assert_eq!(inject_anchor_at_position("<p>broken", 0, "reading-pos"), None);
}

#[test]
fn dl_dt_dd_produce_separate_lines() {
	let xml = "<root><body><dl><dt>Term</dt><dd>Definition</dd></dl></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let text = converter.get_text();
	let lines: Vec<&str> = text.lines().collect();
	assert!(lines.contains(&"Term"), "dt content should be on its own line");
	assert!(lines.contains(&"Definition"), "dd content should be on its own line");
}
/// `TableInfo.length` must equal the emitted display extent (display units), NOT the
/// emitted text's byte length. Prefix text ensures `start_offset` > 0. With inline rendering the
/// emitted row is the TSV "A\t𝄞"; a non-BMP char (U+1D11E, G Clef, width 2) locks the math.
#[test]
fn xml_table_display_length_is_display_extent_not_byte_length() {
	// "Intro\n" → 6 display units. Inline table row: "A\t𝄞" = 4 display units + newline = 5.
	let xml =
		concat!("<root><body><p>Intro</p>", "<table><tr><td>A</td><td>\u{1D11E}</td></tr></table>", "</body></root>");
	let mut converter = XmlToText::with_render_tables_inline(true);
	assert!(converter.convert(xml));
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1, "expected exactly one table");
	let table = &tables[0];
	assert_eq!(table.offset, 6, "table starts after 'Intro\\n'");
	// display_length = 5 (display extent); emitted byte length = 6, they differ.
	assert_eq!(table.length, 5, "length must be the display extent (5), not byte length (6)");
}

/// OFF mode emits the `"[Table]: <first row>"` placeholder; ON mode emits the full TSV.
#[test]
fn xml_table_emits_placeholder_or_tsv_by_flag() {
	let xml = "<root><body><table><tr><td>A</td><td>B</td></tr><tr><td>c</td><td>d</td></tr></table></body></root>";
	let mut off = XmlToText::new();
	assert!(off.convert(xml));
	assert_eq!(off.get_text(), "[Table]: A B");
	let mut on = XmlToText::with_render_tables_inline(true);
	assert!(on.convert(xml));
	assert_eq!(on.get_text(), "A\tB\nc\td");
}

/// Two XML tables: second table's offset equals first offset + first `display_length`.
#[test]
fn xml_two_tables_offsets_are_cumulative() {
	let xml = concat!(
		"<root><body>",
		"<table><tr><td>X</td></tr></table>",
		"<table><tr><td>Y</td></tr></table>",
		"</body></root>"
	);
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 2, "expected two tables");
	let t1_offset = tables[0].offset;
	let t1_display_length = tables[0].length;
	let t2_offset = tables[1].offset;
	assert_eq!(t1_offset, 0, "first table starts at 0");
	assert!(t1_display_length > 0, "first table has non-zero display_length");
	assert_eq!(
		t2_offset,
		t1_offset + t1_display_length,
		"second table offset must equal first offset + first display_length"
	);
}

#[rstest]
#[case("b", MarkerType::Bold)]
#[case("strong", MarkerType::Bold)]
#[case("i", MarkerType::Italic)]
#[case("em", MarkerType::Italic)]
#[case("u", MarkerType::Underline)]
fn test_format_span_collection(#[case] tag: &str, #[case] kind: MarkerType) {
	let xml = format!("<root><body><p>Some <{tag}>bold</{tag}> text</p></body></root>");
	let mut converter = XmlToText::new();
	assert!(converter.convert(&xml));
	let spans = match kind {
		MarkerType::Bold => converter.get_bolds(),
		MarkerType::Italic => converter.get_italics(),
		MarkerType::Underline => converter.get_underlines(),
		_ => unreachable!(),
	};
	assert_eq!(spans.len(), 1);
	let text = converter.get_text();
	assert_eq!(&text[spans[0].offset..spans[0].offset + spans[0].length], "bold");
}

#[test]
fn test_format_span_tag_matching_is_case_insensitive() {
	let xml = "<root><body><p>Some <B>bold</B> text</p></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let spans = converter.get_bolds();
	assert_eq!(spans.len(), 1);
	let text = converter.get_text();
	assert_eq!(&text[spans[0].offset..spans[0].offset + spans[0].length], "bold");
}

#[test]
fn test_format_span_nested_bold_and_italic() {
	let xml = "<root><body><p><b>outer <i>inner</i></b></p></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	let text = converter.get_text();
	let italics = converter.get_italics();
	assert_eq!(italics.len(), 1);
	assert_eq!(&text[italics[0].offset..italics[0].offset + italics[0].length], "inner");
	let bolds = converter.get_bolds();
	assert_eq!(bolds.len(), 1);
	assert_eq!(&text[bolds[0].offset..bolds[0].offset + bolds[0].length], "outer inner");
}

#[test]
fn test_format_span_fully_inside_link_is_dropped_known_limitation() {
	// Known limitation: `<a>` handling (see `handle_element_opening_xml`) eagerly grabs the full
	// flattened text of the element via `collect_element_text`, pushes it into `current_line`, and
	// sets `skip_children = true`. `process_node` then skips recursing into the `<a>`'s children
	// entirely, so the open/close handlers for a `<b>`/`<i>`/`<u>` nested inside the link never
	// fire and no format span is recorded. This degrades gracefully (no panic, no bad offset; the
	// link itself is still recorded correctly) and is left as-is per this task's scope (no changes
	// to the `<a>`/`skip_children` behavior).
	let xml = "<root><body><a href=\"https://example.com\"><b>bold</b></a></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	assert!(converter.get_bolds().is_empty());
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	assert_eq!(links[0].text, "bold");
}

#[test]
fn test_no_format_spans_without_formatting_tags() {
	let xml = "<root><body><p>Plain paragraph with no formatting.</p></body></root>";
	let mut converter = XmlToText::new();
	assert!(converter.convert(xml));
	assert!(converter.get_bolds().is_empty());
	assert!(converter.get_italics().is_empty());
	assert!(converter.get_underlines().is_empty());
}
