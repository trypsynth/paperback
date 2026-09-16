use super::*;

#[test]
fn rewrites_font_size_seven_to_h1() {
	let html = r#"<p>Before</p><font size="7">Title</font><p>After</p>"#;
	assert_eq!(rewrite_font_size_headings(html), "<p>Before</p><h1>Title</h1><p>After</p>");
}

#[test]
fn rewrites_each_recognized_size_to_its_own_level() {
	let html = concat!(
		r#"<font size="7">one</font>"#,
		r#"<font size="6">two</font>"#,
		r#"<font size="5">three</font>"#,
		r#"<font size="4">four</font>"#,
	);
	assert_eq!(rewrite_font_size_headings(html), "<h1>one</h1><h2>two</h2><h3>three</h3><h4>four</h4>");
}

#[test]
fn leaves_unrecognized_font_sizes_untouched() {
	let html = r#"<font size="2">small print</font>"#;
	assert_eq!(rewrite_font_size_headings(html), html);
}

#[test]
fn does_nothing_when_semantic_headings_already_exist() {
	let html = r#"<h2>Existing Heading</h2><font size="7">Not a heading</font>"#;
	assert_eq!(rewrite_font_size_headings(html), html);
}

#[test]
fn matches_size_attribute_regardless_of_quote_style_or_case() {
	let html = "<FONT SIZE=7>Title</FONT>";
	assert_eq!(rewrite_font_size_headings(html), "<h1>Title</h1>");
}

#[test]
fn leaves_content_with_no_font_tags_untouched() {
	let html = "<p>Just a paragraph.</p>";
	assert_eq!(rewrite_font_size_headings(html), html);
}

// Old Mobipocket HTML leaves font tags unclosed, so a rewrite has to pick one of two broken
// readings. Taking them in the order they open keeps the text that follows an unclosed tag
// inside the heading it opened, rather than stranding it in a block of its own.
#[test]
fn an_unclosed_font_tag_is_taken_in_the_order_it_opens() {
	let html = r#"<font size="4">outer<font size="7">inner</font>"#;
	assert_eq!(rewrite_font_size_headings(html), r#"<h4>outer<font size="7">inner</h4>"#);
}
