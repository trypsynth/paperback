use rstest::rstest;

use super::*;
use crate::{
	document::{DocumentBuffer, MarkerType},
	parser::add_converter_markers,
	util::text::display_len,
};

/// End-to-end: the `HtmlToText` converter emits each table's on-screen text at parse time, and a
/// heading that follows the table is offset by the emitted display extent. Verified in both
/// modes: OFF (placeholder) and ON (full TSV). The fixture has an "Intro" paragraph before the
/// table (so the table offset is non-zero) and an `<h2>` after it.
#[rstest]
#[case(false)]
#[case(true)]
fn html_converter_emits_table_inline_or_placeholder(#[case] inline: bool) {
	let html = concat!(
		"<html><body>",
		"<p>Intro</p>",
		"<table><tr><td>A</td><td>B</td></tr></table>",
		"<h2>After heading</h2>",
		"</body></html>"
	);
	let mut converter = HtmlToText::with_render_tables_inline(inline);
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].offset, 6, "table follows 'Intro\n' (6 display units)");
	let table_line = if inline { "A\tB" } else { "[Table]: A B" };
	let expected_text = format!("Intro\n{table_line}\nAfter heading");
	assert_eq!(converter.get_text(), expected_text, "table emitted as {table_line:?}");
	// display_length equals the emitted display extent (the table line plus its newline).
	let expected_display_length = display_len(table_line) + 1;
	assert_eq!(tables[0].length, expected_display_length);
	// The heading marker that follows the table sits right after the emitted table span.
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(
		headings[0].offset,
		tables[0].offset + expected_display_length,
		"h2 immediately follows the emitted table span"
	);
	// Through the real marker path, the Table marker's length matches the emitted extent.
	let mut buffer = DocumentBuffer::with_content(converter.get_text());
	add_converter_markers(&mut buffer, &converter, 0);
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.length, expected_display_length);
}

#[test]
fn test_title_and_text() {
	let html = "<html><head><title>  Hello   World </title></head><body><p>Hi</p></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert_eq!(converter.get_title(), "Hello World");
	assert_eq!(converter.get_text(), "Hi");
}

#[test]
fn test_link_collection() {
	let html = "<html><body><a href=\"https://example.com\">Hello   world</a></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	assert_eq!(links[0].text, "Hello world");
	assert_eq!(links[0].reference, "https://example.com");
	assert_eq!(converter.get_text(), "Hello world");
}

#[rstest]
#[case("b", MarkerType::Bold)]
#[case("strong", MarkerType::Bold)]
#[case("i", MarkerType::Italic)]
#[case("em", MarkerType::Italic)]
#[case("u", MarkerType::Underline)]
fn test_format_span_collection(#[case] tag: &str, #[case] kind: MarkerType) {
	let html = format!("<html><body><p>Some <{tag}>bold</{tag}> text</p></body></html>");
	let mut converter = HtmlToText::new();
	assert!(converter.convert(&html, HtmlSourceMode::NativeHtml));
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
fn test_format_span_nested_bold_and_italic() {
	let html = "<html><body><p><b>outer <i>inner</i></b></p></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
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
	// Known limitation: text inside an `<a>` is buffered in `current_link_text` and only
	// pushed into `current_line` on `</a>` (see `handle_text_node`/`handle_element_closing`),
	// so `get_current_text_position()` does not advance while inside a link. A `<b>`/`<i>`/`<u>`
	// that opens and closes fully inside an `<a>` therefore sees `start == end` and is recorded
	// as a zero-length span rather than spanning the link's text. This degrades gracefully (no
	// panic, no bad offset; the link itself is still recorded correctly) rather than corrupting
	// data, so it is left as-is (no changes to the `<a>` deferred-buffering behavior).
	let html = "<html><body><a href=\"https://example.com\"><b>bold</b></a></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let bolds = converter.get_bolds();
	assert_eq!(bolds.len(), 1);
	assert_eq!(bolds[0].length, 0);
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	assert_eq!(links[0].text, "bold");
}

#[test]
fn test_no_format_spans_without_formatting_tags() {
	let html = "<html><body><p>Plain paragraph with no formatting.</p></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert!(converter.get_bolds().is_empty());
	assert!(converter.get_italics().is_empty());
	assert!(converter.get_underlines().is_empty());
}

#[test]
fn test_ordered_list_metadata() {
	let html = "<html><body><ol start=\"3\" type=\"a\"><li>First</li><li>Second</li></ol></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let lists = converter.get_lists();
	let items = converter.get_list_items();
	assert_eq!(lists.len(), 1);
	assert_eq!(lists[0].item_count, 2);
	// The recorded length must span the whole list, reaching at least the last item's start.
	assert!(lists[0].length > 0);
	assert!(lists[0].offset + lists[0].length >= items[1].offset);
	// End lands at most one line break past the content (trailing newline at document end).
	assert!(lists[0].offset + lists[0].length <= display_len(&converter.get_text()) + 1);
	assert_eq!(items.len(), 2);
	assert_eq!(items[0].level, 1);
	assert_eq!(items[0].text, "First");
	assert_eq!(items[1].text, "Second");
}

#[test]
fn test_table_caption_fallback() {
	let html = "<html><body><table><tr><td>Header</td></tr></table></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].text, "Header");
}

#[rstest]
#[case("h1", 1)]
#[case("h2", 2)]
#[case("h3", 3)]
#[case("h4", 4)]
#[case("h5", 5)]
#[case("h6", 6)]
fn heading_levels_h1_to_h6(#[case] tag: &str, #[case] expected_level: i32) {
	let html = format!("<html><body><{tag}>Title</{tag}></body></html>");
	let mut converter = HtmlToText::new();
	assert!(converter.convert(&html, HtmlSourceMode::NativeHtml));
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(headings[0].level, expected_level);
	assert_eq!(headings[0].text, "Title");
}

#[test]
fn hr_produces_separator() {
	let html = "<html><body><p>Before</p><hr/><p>After</p></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert_eq!(converter.get_separators().len(), 1);
}

#[test]
fn nested_ul_increments_list_level() {
	let html = "<html><body><ul><li>Outer<ul><li>Inner</li></ul></li></ul></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let items = converter.get_list_items();
	assert!(items.len() >= 2, "expected at least two list items");
	let outer_level = items.iter().find(|i| i.text == "Outer").map_or(0, |i| i.level);
	let inner_level = items.iter().find(|i| i.text == "Inner").map_or(0, |i| i.level);
	assert!(inner_level > outer_level, "nested item should have a higher level");
}

#[test]
fn element_id_is_indexed() {
	let html = "<html><body><p id=\"anchor\">Content</p></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert!(converter.get_id_positions().contains_key("anchor"));
}

#[test]
fn pre_block_preserves_whitespace_characters() {
	let html = "<html><body><pre>  spaced  </pre></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert!(converter.get_text().contains("  spaced  "));
}

/// A `<pre>` block whose source text uses CRLF line endings must not leak a stray `\r` onto
/// the end of each preserved line - the shared `LineBuilder::add_line` strips trailing
/// `\r`/`\n` from a preserved line before storing it, same as `XmlToText` already relied on.
#[test]
fn pre_block_with_crlf_content_does_not_leak_carriage_returns() {
	let html = "<html><body><pre>line one\r\nline two</pre></body></html>";
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	assert!(!converter.get_text().contains('\r'), "got: {:?}", converter.get_text());
}

#[test]
fn clear_resets_converter_state() {
	let html1 = "<html><head><title>First</title></head><body><h1>One</h1></body></html>";
	let html2 = "<html><head><title>Second</title></head><body><p>Two</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html1, HtmlSourceMode::NativeHtml);
	converter.clear();
	assert!(converter.convert(html2, HtmlSourceMode::NativeHtml));
	assert_eq!(converter.get_title(), "Second");
	assert_eq!(converter.get_text(), "Two");
	assert!(converter.get_headings().is_empty());
}

#[test]
fn html_table_display_length_is_display_extent_not_byte_length() {
	let html =
		concat!("<html><body><p>Intro</p>", "<table><tr><td>A</td><td>\u{1D11E}</td></tr></table>", "</body></html>");
	let mut converter = HtmlToText::with_render_tables_inline(true);
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1, "expected exactly one table");
	let table = &tables[0];
	assert_eq!(table.offset, 6, "table starts after 'Intro\n'");
	assert_eq!(table.length, 5, "length must be the display extent (5 display units), not byte length (6)");
}

#[test]
fn html_two_tables_offsets_are_cumulative() {
	let html = concat!(
		"<html><body>",
		"<table><tr><td>X</td></tr></table>",
		"<table><tr><td>Y</td></tr></table>",
		"</body></html>"
	);
	let mut converter = HtmlToText::new();
	assert!(converter.convert(html, HtmlSourceMode::NativeHtml));
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
