use anyhow::Result;

use super::FeedParser;
use crate::{
	document::{Document, DocumentHandle, Marker, MarkerType, ParserContext},
	parser::Parser,
	reader_core::resolve_link,
	util::test_support::TempDir,
};

fn parse_feed(name: &str, contents: impl AsRef<[u8]>) -> Result<Document> {
	let dir = TempDir::new("feed-parser");
	let path = dir.write_str(name, contents);
	FeedParser.parse(&ParserContext::new(path))
}

fn parse_ok(contents: &str) -> Document {
	parse_feed("feed.xml", contents).expect("parse feed")
}

fn rss(channel: &str) -> String {
	format!(
		r#"<?xml version="1.0" encoding="UTF-8"?><rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:dc="http://purl.org/dc/elements/1.1/"><channel>{channel}</channel></rss>"#
	)
}

fn markers(doc: &Document, mtype: MarkerType) -> Vec<&Marker> {
	doc.buffer.markers.iter().filter(|marker| marker.mtype == mtype).collect()
}

fn heading_texts(doc: &Document) -> Vec<&str> {
	markers(doc, MarkerType::Heading1).iter().map(|marker| marker.text.as_str()).collect()
}

const TWO_POSTS: &str = concat!(
	"<title>Example Blog</title><link>https://blog.example/</link><description>Notes and essays</description>",
	"<managingEditor>ed@blog.example (Ed)</managingEditor>",
	"<item><title>First post</title><link>https://blog.example/first</link>",
	"<dc:creator>Jane Doe</dc:creator><pubDate>Tue, 06 Oct 2026 09:00:00 GMT</pubDate>",
	"<description>&lt;p&gt;Hello &lt;b&gt;world&lt;/b&gt;&lt;/p&gt;</description></item>",
	"<item><title>Second post</title><description>Second body</description></item>",
);

#[test]
fn each_item_is_a_heading_a_section_and_a_contents_entry() {
	let doc = parse_ok(&rss(TWO_POSTS));
	assert_eq!(doc.title, "Example Blog");
	assert_eq!(doc.author, "Ed");
	assert_eq!(heading_texts(&doc), ["First post", "Second post"]);
	let heading_positions: Vec<usize> =
		markers(&doc, MarkerType::Heading1).iter().map(|marker| marker.position).collect();
	let toc: Vec<(&str, usize)> = doc.toc_items.iter().map(|item| (item.name.as_str(), item.offset)).collect();
	assert_eq!(toc, [("First post", heading_positions[0]), ("Second post", heading_positions[1])]);
	let breaks: Vec<usize> = markers(&doc, MarkerType::SectionBreak).iter().map(|marker| marker.position).collect();
	assert_eq!(breaks, [0, heading_positions[0], heading_positions[1]], "the intro, then one section per item");
	assert!(doc.buffer.content.contains("Hello world"), "got {:?}", doc.buffer.content);
	assert!(doc.buffer.content.contains("Second body"), "got {:?}", doc.buffer.content);
}

#[test]
fn the_intro_holds_the_description_and_a_link_to_the_website() {
	let doc = parse_ok(&rss(TWO_POSTS));
	assert!(doc.buffer.content.starts_with("Notes and essays"), "got {:?}", doc.buffer.content);
	let website = markers(&doc, MarkerType::Link)
		.into_iter()
		.find(|marker| marker.text == "Website")
		.expect("a link to the feed's website");
	assert_eq!(website.reference, "https://blog.example/");
	assert!(website.position < markers(&doc, MarkerType::Heading1)[0].position, "it belongs to the intro");
}

#[test]
fn an_item_names_its_date_and_author_and_links_to_the_original() {
	let doc = parse_ok(&rss(TWO_POSTS));
	assert!(doc.buffer.content.contains("Published 6 October 2026 by Jane Doe"), "got {:?}", doc.buffer.content);
	let originals: Vec<&str> = markers(&doc, MarkerType::Link)
		.into_iter()
		.filter(|marker| marker.text == "Original article")
		.map(|marker| marker.reference.as_str())
		.collect();
	assert_eq!(originals, ["https://blog.example/first"], "only the item with a link gets one");
}

#[test]
fn the_byline_is_written_with_whatever_the_item_has() {
	let doc = parse_ok(&rss(concat!(
		"<item><title>Dated</title><pubDate>Tue, 06 Oct 2026 09:00:00 GMT</pubDate></item>",
		"<item><title>Signed</title><dc:creator>Jane Doe</dc:creator></item>",
		"<item><title>Neither</title><description>Plain</description></item>",
	)));
	assert!(doc.buffer.content.contains("Published 6 October 2026\n"), "got {:?}", doc.buffer.content);
	assert!(doc.buffer.content.contains("By Jane Doe"), "got {:?}", doc.buffer.content);
	assert!(!doc.buffer.content.contains("Published \n"), "got {:?}", doc.buffer.content);
}

#[test]
fn a_relative_link_in_a_body_resolves_against_the_item() {
	let doc = parse_ok(&rss(
		r#"<item><title>T</title><link>https://blog.example/posts/first</link><description>&lt;a href="../about"&gt;About&lt;/a&gt;</description></item>"#,
	));
	let about = markers(&doc, MarkerType::Link).into_iter().find(|marker| marker.text == "About").expect("the link");
	assert_eq!(about.reference, "https://blog.example/about");
}

#[test]
fn a_footnote_link_stays_inside_its_own_item() {
	let item = |n: &str| {
		format!(
			r##"<item><title>Post {n}</title><description>&lt;p&gt;Claim&lt;a href="#fn1"&gt;1&lt;/a&gt;&lt;/p&gt;&lt;p id="fn1"&gt;Note {n}&lt;/p&gt;</description></item>"##
		)
	};
	let doc = parse_ok(&rss(&format!("<description>About</description>{}{}", item("one"), item("two"))));
	let second_link = markers(&doc, MarkerType::Link)
		.into_iter()
		.filter(|marker| marker.reference == "#fn1")
		.nth(1)
		.expect("the second post's footnote link")
		.position;
	let handle = DocumentHandle::new(doc);
	let target = resolve_link(&handle, "#fn1", i64::try_from(second_link).unwrap());
	assert!(target.found && !target.is_external, "{target:?}");
	let content = &handle.document().buffer.content;
	assert!(content[target.offset..].starts_with("Note two"), "landed on {:?}", &content[target.offset..]);
}

#[test]
fn an_xml_file_that_is_not_a_feed_is_an_error() {
	let err = parse_feed("page.xml", "<html><body><p>Hi</p></body></html>").expect_err("not a feed");
	assert!(err.to_string().contains("not an RSS or Atom feed"), "got {err}");
}

#[test]
fn a_feed_with_an_undeclared_prefix_and_html_entities_still_opens() {
	let doc = parse_ok(
		"<rss version=\"2.0\"><channel><title>Broken</title><item><title>One</title><content:encoded>Caf&eacute; open</content:encoded></item></channel></rss>",
	);
	assert_eq!(doc.title, "Broken");
	assert!(doc.buffer.content.contains("Café open"), "got {:?}", doc.buffer.content);
}

#[test]
fn a_feed_in_windows_1252_is_decoded() {
	let mut bytes = br#"<?xml version="1.0" encoding="windows-1252"?><rss version="2.0"><channel><title>Caf"#.to_vec();
	bytes.push(0xE9);
	bytes.extend_from_slice(b"</title><item><title>One</title></item></channel></rss>");
	let doc = parse_feed("cafe.rss", bytes).expect("parse feed");
	assert_eq!(doc.title, "Café");
}

#[test]
fn an_empty_feed_says_so() {
	let doc = parse_ok(&rss("<title>Quiet</title>"));
	assert_eq!(doc.buffer.content.trim(), "This feed has no items.");
}

#[test]
fn an_untitled_item_is_named_by_the_start_of_its_text() {
	let doc = parse_ok(&rss(concat!(
		"<item><description>Just a short thought.</description></item>",
		"<item><description>Paperback now reads feeds, so a saved RSS file opens with one heading for every post in it today</description></item>",
		"<item><pubDate>Tue, 06 Oct 2026 09:00:00 GMT</pubDate></item>",
		"<item></item>",
	)));
	assert_eq!(
		heading_texts(&doc),
		[
			"Just a short thought.",
			"Paperback now reads feeds, so a saved RSS file opens with one heading for every…",
			"6 October 2026",
			"Untitled item",
		]
	);
}

#[test]
fn plain_text_ends_its_line_before_the_link_after_it() {
	let doc = parse_ok(&rss(concat!(
		"<description>Plain intro</description><link>https://blog.example/</link>",
		"<item><title>T</title><link>https://blog.example/t</link><description>Plain body</description></item>",
	)));
	assert!(doc.buffer.content.contains("Plain intro\n"), "got {:?}", doc.buffer.content);
	assert!(doc.buffer.content.contains("Plain body\n"), "got {:?}", doc.buffer.content);
}

#[test]
fn a_name_cut_after_trailing_dots_ends_in_one_ellipsis() {
	let text = format!("{} Advocates... and then more words", "x".repeat(66));
	assert_eq!(super::start_of(&text), format!("{} Advocates…", "x".repeat(66)));
}

#[test]
fn a_feed_without_a_title_is_named_after_its_file() {
	let doc = parse_feed("news.xml", rss("<item><title>One</title></item>")).expect("parse feed");
	assert_eq!(doc.title, "news");
}
