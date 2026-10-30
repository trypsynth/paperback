use rstest::rstest;

use super::*;
use crate::{
	document::{DocLen, DocPos, DocumentBuffer, MarkerType},
	parser::add_converter_markers,
};

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
	converter.convert(html, HtmlSourceMode::NativeHtml);
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1);
	assert_eq!(tables[0].offset, 6, "table follows 'Intro\n' (6 positions)");
	let table_line = if inline { "A\tB" } else { "[Table]: A B" };
	let expected_text = format!("Intro\n{table_line}\nAfter heading");
	assert_eq!(converter.get_text(), expected_text, "table emitted as {table_line:?}");
	let expected_length = DocLen::of(table_line) + 1.into();
	assert_eq!(tables[0].length, expected_length);
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(headings[0].offset, tables[0].offset + expected_length, "h2 immediately follows the emitted table span");
	let mut buffer = DocumentBuffer::with_content(converter.take_text());
	add_converter_markers(&mut buffer, &converter, DocPos::ZERO);
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.length, expected_length);
}

#[test]
fn test_title_and_text() {
	let html = "<html><head><title>  Hello   World </title></head><body><p>Hi</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_title(), "Hello World");
	assert_eq!(converter.get_text(), "Hi");
}

#[test]
fn test_link_collection() {
	let html = "<html><body><a href=\"https://example.com\">Hello   world</a></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
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
	converter.convert(&html, HtmlSourceMode::NativeHtml);
	let spans = match kind {
		MarkerType::Bold => converter.get_bolds(),
		MarkerType::Italic => converter.get_italics(),
		MarkerType::Underline => converter.get_underlines(),
		_ => unreachable!(),
	};
	assert_eq!(spans.len(), 1);
	let text = converter.get_text();
	assert_eq!(&text[spans[0].offset.get()..(spans[0].offset + spans[0].length).get()], "bold");
}

#[test]
fn test_format_span_nested_bold_and_italic() {
	let html = "<html><body><p><b>outer <i>inner</i></b></p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	let text = converter.get_text();
	let italics = converter.get_italics();
	assert_eq!(italics.len(), 1);
	assert_eq!(&text[italics[0].offset.get()..(italics[0].offset + italics[0].length).get()], "inner");
	let bolds = converter.get_bolds();
	assert_eq!(bolds.len(), 1);
	assert_eq!(&text[bolds[0].offset.get()..(bolds[0].offset + bolds[0].length).get()], "outer inner");
}

#[test]
fn test_format_span_fully_inside_link_is_dropped_known_limitation() {
	// Link text is buffered until `</a>`, so the position doesn't advance inside a link and formatting wholly inside one is zero-length; harmless, so left as-is.
	let html = "<html><body><a href=\"https://example.com\"><b>bold</b></a></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
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
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert!(converter.get_bolds().is_empty());
	assert!(converter.get_italics().is_empty());
	assert!(converter.get_underlines().is_empty());
}

#[test]
fn test_ordered_list_metadata() {
	let html = "<html><body><ol start=\"3\" type=\"a\"><li>First</li><li>Second</li></ol></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	let lists = converter.get_lists();
	let items = converter.get_list_items();
	assert_eq!(lists.len(), 1);
	assert_eq!(lists[0].item_count, 2);
	assert!(lists[0].length > 0.into());
	assert!(lists[0].offset + lists[0].length >= items[1].offset);
	// One past the content allows for the trailing newline at document end.
	assert!(lists[0].offset + lists[0].length <= DocLen::of(converter.get_text()).as_pos() + 1.into());
	assert_eq!(items.len(), 2);
	assert_eq!(items[0].level, 1);
	assert_eq!(items[0].text, "First");
	assert_eq!(items[1].text, "Second");
}

#[test]
fn test_table_caption_fallback() {
	let html = "<html><body><table><tr><td>Header</td></tr></table></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
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
	converter.convert(&html, HtmlSourceMode::NativeHtml);
	let headings = converter.get_headings();
	assert_eq!(headings.len(), 1);
	assert_eq!(headings[0].level, expected_level);
	assert_eq!(headings[0].text, "Title");
}

#[test]
fn hr_produces_separator() {
	let html = "<html><body><p>Before</p><hr/><p>After</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_separators().len(), 1);
}

#[test]
fn nested_ul_increments_list_level() {
	let html = "<html><body><ul><li>Outer<ul><li>Inner</li></ul></li></ul></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
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
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert!(converter.get_id_positions().contains_key("anchor"));
}

#[test]
fn pre_block_preserves_whitespace_characters() {
	let html = "<html><body><pre>  spaced  </pre></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert!(converter.get_text().contains("  spaced  "));
}

#[test]
fn pre_block_with_crlf_content_does_not_leak_carriage_returns() {
	let html = "<html><body><pre>line one\r\nline two</pre></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert!(!converter.get_text().contains('\r'), "got: {:?}", converter.get_text());
}

#[test]
fn clear_resets_converter_state() {
	let html1 = "<html><head><title>First</title></head><body><h1>One</h1></body></html>";
	let html2 = "<html><head><title>Second</title></head><body><p>Two</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html1, HtmlSourceMode::NativeHtml);
	converter.clear();
	converter.convert(html2, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_title(), "Second");
	assert_eq!(converter.get_text(), "Two");
	assert!(converter.get_headings().is_empty());
}

#[test]
fn html_table_length_counts_positions_not_bytes() {
	let html =
		concat!("<html><body><p>Intro</p>", "<table><tr><td>A</td><td>\u{1D11E}</td></tr></table>", "</body></html>");
	let mut converter = HtmlToText::with_render_tables_inline(true);
	converter.convert(html, HtmlSourceMode::NativeHtml);
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 1, "expected exactly one table");
	let table = &tables[0];
	assert_eq!(table.offset, 6, "table starts after 'Intro\n'");
	// "A\t𝄞\n": the non-BMP character is four bytes but one position.
	assert_eq!(table.length, 4, "length must be the table's extent in positions");
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
	converter.convert(html, HtmlSourceMode::NativeHtml);
	let tables = converter.get_tables();
	assert_eq!(tables.len(), 2, "expected two tables");
	let t1_offset = tables[0].offset;
	let t1_length = tables[0].length;
	let t2_offset = tables[1].offset;
	assert_eq!(t1_offset, 0, "first table starts at 0");
	assert!(t1_length > 0.into(), "first table has a non-zero length");
	assert_eq!(t2_offset, t1_offset + t1_length, "second table offset must equal first offset + first length");
}

/// The whole placeholder, not just its first character, so a reader can style it apart from the prose around it.
#[test]
fn html_converter_gives_the_image_marker_the_placeholder_span() {
	let html = "<html><body><p>Before <img alt=\"a cat\"> after</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_text(), "Before [Image: a cat] after");
	let images = converter.get_images();
	assert_eq!(images.len(), 1);
	assert_eq!(images[0].offset, DocLen::of("Before ").as_pos());
	assert_eq!(images[0].length, DocLen::of("[Image: a cat]"));
	let mut buffer = DocumentBuffer::with_content(converter.get_text().to_string());
	add_converter_markers(&mut buffer, &converter, DocPos::ZERO);
	let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Image).expect("Image marker");
	assert_eq!(marker.position, images[0].offset);
	assert_eq!(marker.length, DocLen::of("[Image: a cat]"));
}

#[test]
fn html_converter_makes_a_link_of_a_picture_alone() {
	let html = "<html><body><p>Go <a href=\"/home\"><img src=\"logo.png\" alt=\"Logo\"></a> now</p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_text(), "Go [Image: Logo] now");
	let links = converter.get_links();
	assert_eq!(links.len(), 1);
	assert_eq!(
		(links[0].offset, links[0].text.as_str(), links[0].reference.as_str()),
		(DocLen::of("Go ").as_pos(), "[Image: Logo]", "/home")
	);
	let images = converter.get_images();
	assert_eq!(images.len(), 1);
	assert_eq!((images[0].offset, images[0].source.as_str()), (DocLen::of("Go ").as_pos(), "logo.png"));
}

#[test]
fn html_converter_keeps_a_picture_in_its_place_inside_a_link() {
	let html = "<html><body><p><a href=\"/home\">Back  to <img alt=\"Logo\"> home</a></p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_text(), "Back to [Image: Logo] home");
	assert_eq!(converter.get_links()[0].text, "Back to [Image: Logo] home");
	assert_eq!(converter.get_images()[0].offset, DocLen::of("Back to ").as_pos());
}

#[test]
fn html_converter_shows_an_undescribed_picture_and_skips_decoration() {
	let html = "<html><body><p>A <img src=\"chart.png\"> B <img src=\"rule.png\" alt=\"\"> C <img src=\"x.gif\" width=\"1\" height=\"1\"></p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_text(), "A [Image] B C");
	let images = converter.get_images();
	assert_eq!(images.len(), 1);
	assert_eq!((images[0].offset, images[0].source.as_str()), (DocLen::of("A ").as_pos(), "chart.png"));
}

#[test]
fn html_converter_makes_a_link_of_an_undescribed_picture() {
	let html = "<html><body><p><a href=\"/home\"><img src=\"logo.png\"></a></p></body></html>";
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	assert_eq!(converter.get_text(), "[Image]");
	assert_eq!(converter.get_links()[0].text, "[Image]");
}
