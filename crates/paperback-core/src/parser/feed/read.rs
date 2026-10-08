//! Reading an RSS or Atom document into a [`Feed`], before any of it is turned into text.

use anyhow::{Result, anyhow};
use roxmltree::{Document as XmlDocument, Node};

use crate::{
	parser::util::xml::{collect_element_text, escape_xml, serialize_xml},
	t,
};

const RSS_090_NS: &str = "http://my.netscape.com/rdf/simple/0.9/";
const RSS_1_NS: &str = "http://purl.org/rss/1.0/";
const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const CONTENT_NS: &str = "http://purl.org/rss/1.0/modules/content/";
const DC_NS: &str = "http://purl.org/dc/elements/1.1/";
const ATOM_NS: &str = "http://www.w3.org/2005/Atom";
const ATOM_03_NS: &str = "http://purl.org/atom/ns#";

/// The prefixes of feed extensions and their namespaces, for `repair_xml` to declare in a feed that uses them undeclared.
pub(super) const FEED_PREFIXES: &[(&str, &str)] = &[("content", CONTENT_NS), ("dc", DC_NS), ("atom", ATOM_NS)];

/// What a feed says about itself and its items. Bodies are HTML; everything else is plain text.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Feed {
	pub title: String,
	pub description_html: String,
	pub link: Option<String>,
	pub author: String,
	pub items: Vec<FeedItem>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct FeedItem {
	pub title: String,
	pub link: Option<String>,
	/// The item's own author, or the feed's when the item names none.
	pub author: String,
	/// The date as the feed wrote it.
	pub date: String,
	pub body_html: String,
}

/// Reads RSS 0.9x to 2.0 (`<rss>`), RSS 1.0 (`<rdf:RDF>`) and Atom (`<feed>`).
///
/// # Errors
///
/// Returns an error if the document's root element is none of those.
pub(super) fn read_feed(doc: &XmlDocument) -> Result<Feed> {
	let root = doc.root_element();
	let feed = if is_rss(root, "rss") {
		child(root, |node| is_rss(node, "channel")).map(|channel| read_rss(channel, channel))
	} else if is_in(root, RDF_NS, "RDF") {
		child(root, |node| is_rss(node, "channel")).map(|channel| read_rss(channel, root))
	} else if is_atom(root, "feed") {
		Some(read_atom(root))
	} else {
		None
	};
	// TRANSLATORS: Error shown when a file opened as an RSS or Atom feed turns out to be some other kind of XML
	let mut feed = feed.ok_or_else(|| anyhow!(t("This file is not an RSS or Atom feed.")))?;
	for item in &mut feed.items {
		if item.author.is_empty() {
			item.author.clone_from(&feed.author);
		}
	}
	Ok(feed)
}

/// An RSS channel, with its items taken from `item_parent`: the channel itself in RSS 0.9x and 2.0, the document root in RSS 1.0.
fn read_rss(channel: Node, item_parent: Node) -> Feed {
	let author = joined_text(channel, |node| is_in(node, DC_NS, "creator"))
		.or_non_empty(|| person_name(&child_text(channel, |node| is_rss(node, "managingEditor"))));
	Feed {
		title: child_text(channel, |node| is_rss(node, "title")),
		description_html: child(channel, |node| is_rss(node, "description")).map(inner_html).unwrap_or_default(),
		link: non_empty(child_text(channel, |node| is_rss(node, "link"))),
		author,
		items: item_parent.children().filter(|node| is_rss(*node, "item")).map(read_rss_item).collect(),
	}
}

fn read_rss_item(item: Node) -> FeedItem {
	let link = non_empty(child_text(item, |node| is_rss(node, "link")))
		.or_else(|| permalink_guid(item))
		.or_else(|| item.attribute((RDF_NS, "about")).filter(|about| is_web_address(about)).map(str::to_string));
	FeedItem {
		title: child_text(item, |node| is_rss(node, "title")),
		link,
		author: joined_text(item, |node| is_in(node, DC_NS, "creator"))
			.or_non_empty(|| person_name(&child_text(item, |node| is_rss(node, "author")))),
		date: child_text(item, |node| is_rss(node, "pubDate"))
			.or_non_empty(|| child_text(item, |node| is_in(node, DC_NS, "date"))),
		body_html: child(item, |node| is_in(node, CONTENT_NS, "encoded"))
			.map(inner_html)
			.unwrap_or_default()
			.or_non_empty(|| child(item, |node| is_rss(node, "description")).map(inner_html).unwrap_or_default()),
	}
}

/// An RSS `guid` that is also the item's web address, which it is unless it says otherwise.
fn permalink_guid(item: Node) -> Option<String> {
	let guid = child(item, |node| is_rss(node, "guid"))?;
	if guid.attribute("isPermaLink").is_some_and(|value| value.trim().eq_ignore_ascii_case("false")) {
		return None;
	}
	Some(collect_element_text(guid)).filter(|text| is_web_address(text))
}

/// The name in an RSS address written as `jane@example.com (Jane Doe)`, or the whole text when it carries no name in brackets.
fn person_name(text: &str) -> String {
	let trimmed = text.trim();
	trimmed
		.strip_suffix(')')
		.and_then(|rest| rest.split_once('('))
		.map(|(_, name)| name.trim())
		.filter(|name| !name.is_empty())
		.unwrap_or(trimmed)
		.to_string()
}

fn read_atom(feed: Node) -> Feed {
	Feed {
		title: child(feed, |node| is_atom(node, "title")).map(atom_plain_text).unwrap_or_default(),
		description_html: child(feed, |node| is_atom(node, "subtitle")).map(atom_html).unwrap_or_default(),
		link: atom_link(feed),
		author: atom_authors(feed),
		items: feed.children().filter(|node| is_atom(*node, "entry")).map(read_atom_entry).collect(),
	}
}

fn read_atom_entry(entry: Node) -> FeedItem {
	let content = child(entry, |node| is_atom(node, "content"))
		.filter(|content| content.attribute("src").is_none())
		.map(atom_html)
		.unwrap_or_default();
	FeedItem {
		title: child(entry, |node| is_atom(node, "title")).map(atom_plain_text).unwrap_or_default(),
		link: atom_link(entry),
		author: atom_authors(entry),
		date: child_text(entry, |node| is_atom(node, "published"))
			.or_non_empty(|| child_text(entry, |node| is_atom(node, "updated"))),
		body_html: content
			.or_non_empty(|| child(entry, |node| is_atom(node, "summary")).map(atom_html).unwrap_or_default()),
	}
}

fn atom_authors(node: Node) -> String {
	node.children()
		.filter(|author| is_atom(*author, "author"))
		.map(|author| child_text(author, |name| is_atom(name, "name")))
		.filter(|name| !name.is_empty())
		.collect::<Vec<_>>()
		.join(", ")
}

/// The `href` of the first `link` that is the web page itself: `rel="alternate"`, or no `rel`.
fn atom_link(node: Node) -> Option<String> {
	node.children()
		.filter(|link| is_atom(*link, "link"))
		.find(|link| link.attribute("rel").is_none_or(|rel| rel.trim() == "alternate"))
		.and_then(|link| link.attribute("href"))
		.and_then(|href| non_empty(href.trim().to_string()))
}

/// An Atom text construct as HTML: `html` as it is, `xhtml` written back out as markup, and plain text escaped into paragraphs.
fn atom_html(node: Node) -> String {
	let kind = node.attribute("type").unwrap_or("text").trim().to_ascii_lowercase();
	if kind.contains("xhtml") {
		xhtml_html(node)
	} else if kind == "html" || kind.ends_with("/html") {
		inner_html(node)
	} else {
		text_to_html(&collect_element_text(node))
	}
}

/// An Atom text construct as plain text.
fn atom_plain_text(node: Node) -> String {
	let kind = node.attribute("type").unwrap_or("text").trim().to_ascii_lowercase();
	if kind == "html" || kind.ends_with("/html") {
		super::plain_text(&inner_html(node))
	} else {
		collect_element_text(node)
	}
}

/// Inline XHTML written back out as markup, without the `div` Atom wraps it in.
fn xhtml_html(node: Node) -> String {
	let elements: Vec<Node> = node.children().filter(Node::is_element).collect();
	let container = match elements.as_slice() {
		[div] if div.tag_name().name() == "div" => *div,
		_ => node,
	};
	let mut out = String::new();
	for child in container.children() {
		serialize_xml(child, &mut out, &|_| false);
	}
	out.trim().to_string()
}

/// Plain text as HTML: each run of lines between blank lines is a paragraph, and each line break inside one is a `<br>`.
fn text_to_html(text: &str) -> String {
	let mut html = String::new();
	let mut lines: Vec<String> = Vec::new();
	for line in text.lines().chain(std::iter::once("")) {
		let line = line.trim();
		if !line.is_empty() {
			lines.push(escape_xml(line));
		} else if !lines.is_empty() {
			html.push_str("<p>");
			html.push_str(&lines.join("<br>"));
			html.push_str("</p>");
			lines.clear();
		}
	}
	html
}

/// The HTML an element holds. One that holds only text holds HTML escaped or wrapped in CDATA, and that text is the markup. One that holds elements holds the markup itself, which is written back out.
fn inner_html(node: Node) -> String {
	let mut out = String::new();
	if node.children().any(|child| child.is_element()) {
		for child in node.children() {
			serialize_xml(child, &mut out, &|_| false);
		}
	} else {
		for text in node.children().filter(Node::is_text).filter_map(|child| child.text()) {
			out.push_str(text);
		}
	}
	out.trim().to_string()
}

fn is_in(node: Node, namespace: &str, name: &str) -> bool {
	node.is_element() && node.tag_name().name() == name && node.tag_name().namespace() == Some(namespace)
}

/// An RSS element, which RSS 0.9x and 2.0 write in no namespace and RSS 0.90 and 1.0 in their own.
fn is_rss(node: Node, name: &str) -> bool {
	node.is_element()
		&& node.tag_name().name() == name
		&& matches!(node.tag_name().namespace(), None | Some(RSS_1_NS | RSS_090_NS))
}

fn is_atom(node: Node, name: &str) -> bool {
	node.is_element()
		&& node.tag_name().name() == name
		&& matches!(node.tag_name().namespace(), Some(ATOM_NS | ATOM_03_NS))
}

fn child<'a, 'input>(node: Node<'a, 'input>, wanted: impl Fn(Node) -> bool) -> Option<Node<'a, 'input>> {
	node.children().find(|child| wanted(*child))
}

fn child_text(node: Node, wanted: impl Fn(Node) -> bool) -> String {
	child(node, wanted).map(collect_element_text).unwrap_or_default()
}

fn joined_text(node: Node, wanted: impl Fn(Node) -> bool) -> String {
	node.children()
		.filter(|child| wanted(*child))
		.map(collect_element_text)
		.filter(|text| !text.is_empty())
		.collect::<Vec<_>>()
		.join(", ")
}

fn non_empty(text: String) -> Option<String> {
	(!text.is_empty()).then_some(text)
}

fn is_web_address(text: &str) -> bool {
	let lower = text.trim().to_ascii_lowercase();
	lower.starts_with("http://") || lower.starts_with("https://")
}

trait OrNonEmpty {
	/// `self`, or what `fallback` gives when `self` is empty.
	fn or_non_empty(self, fallback: impl FnOnce() -> String) -> String;
}

impl OrNonEmpty for String {
	fn or_non_empty(self, fallback: impl FnOnce() -> Self) -> Self {
		if self.is_empty() { fallback() } else { self }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn read(xml: &str) -> Feed {
		read_feed(&XmlDocument::parse(xml).expect("test feed is well-formed XML")).expect("a feed")
	}

	fn rss(channel: &str) -> String {
		format!(
			r#"<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:atom="http://www.w3.org/2005/Atom"><channel>{channel}</channel></rss>"#
		)
	}

	fn atom(body: &str) -> String {
		format!(r#"<feed xmlns="http://www.w3.org/2005/Atom">{body}</feed>"#)
	}

	#[test]
	fn rss_channel_and_items_are_read() {
		let feed = read(&rss(concat!(
			"<title> Example Blog </title><link>https://blog.example/</link>",
			"<description>Notes &amp; essays</description>",
			"<item><title>First post</title><link>https://blog.example/first</link>",
			"<pubDate>Tue, 06 Oct 2026 09:00:00 GMT</pubDate><description>Hello</description></item>",
			"<item><title>Second post</title></item>",
		)));
		assert_eq!(feed.title, "Example Blog");
		assert_eq!(feed.link.as_deref(), Some("https://blog.example/"));
		assert_eq!(feed.description_html, "Notes & essays");
		assert_eq!(feed.items.len(), 2);
		let first = &feed.items[0];
		assert_eq!(first.title, "First post");
		assert_eq!(first.link.as_deref(), Some("https://blog.example/first"));
		assert_eq!(first.date, "Tue, 06 Oct 2026 09:00:00 GMT");
		assert_eq!(first.body_html, "Hello");
		assert_eq!(feed.items[1].title, "Second post");
		assert_eq!(feed.items[1].link, None);
	}

	#[test]
	fn rss_full_content_wins_over_the_description() {
		let feed = read(&rss(
			"<item><description>Summary</description><content:encoded><![CDATA[<p>Full <b>text</b></p>]]></content:encoded></item>",
		));
		assert_eq!(feed.items[0].body_html, "<p>Full <b>text</b></p>");
	}

	#[test]
	fn rss_atom_links_do_not_replace_the_rss_links() {
		let feed = read(&rss(concat!(
			r#"<atom:link href="https://blog.example/feed.xml" rel="self"/><link>https://blog.example/</link>"#,
			r#"<item><atom:link href="https://blog.example/other" rel="related"/><link>https://blog.example/first</link></item>"#,
		)));
		assert_eq!(feed.link.as_deref(), Some("https://blog.example/"));
		assert_eq!(feed.items[0].link.as_deref(), Some("https://blog.example/first"));
	}

	#[test]
	fn rss_item_author_prefers_dc_creator_then_author_name_then_the_channel() {
		let feed = read(&rss(concat!(
			"<managingEditor>ed@blog.example (Ed Itor)</managingEditor>",
			"<item><dc:creator>Jane Doe</dc:creator><author>jd@blog.example (Someone Else)</author></item>",
			"<item><author>sam@blog.example (Sam Smith)</author></item>",
			"<item><author>bare@blog.example</author></item>",
			"<item></item>",
		)));
		let authors: Vec<&str> = feed.items.iter().map(|item| item.author.as_str()).collect();
		assert_eq!(authors, ["Jane Doe", "Sam Smith", "bare@blog.example", "Ed Itor"]);
		assert_eq!(feed.author, "Ed Itor");
	}

	#[test]
	fn rss_falls_back_to_dc_date() {
		let feed = read(&rss("<item><dc:date>2026-10-06T09:00:00Z</dc:date></item>"));
		assert_eq!(feed.items[0].date, "2026-10-06T09:00:00Z");
	}

	#[test]
	fn rss_permalink_guid_stands_in_for_a_missing_link() {
		let feed = read(&rss(concat!(
			"<item><guid>https://blog.example/permalink</guid></item>",
			r#"<item><guid isPermaLink="false">tag:blog.example,2026:1</guid></item>"#,
		)));
		assert_eq!(feed.items[0].link.as_deref(), Some("https://blog.example/permalink"));
		assert_eq!(feed.items[1].link, None);
	}

	#[test]
	fn rss_body_split_into_text_and_cdata_is_joined() {
		let feed = read(&rss("<item><description>Intro <![CDATA[<b>bold</b>]]> end</description></item>"));
		assert_eq!(feed.items[0].body_html, "Intro <b>bold</b> end");
	}

	#[test]
	fn rss_body_written_as_escaped_html_is_unescaped_once() {
		let feed = read(&rss("<item><description>&lt;p&gt;Fish &amp;amp; chips&lt;/p&gt;</description></item>"));
		assert_eq!(feed.items[0].body_html, "<p>Fish &amp; chips</p>");
	}

	#[test]
	fn rss_body_written_as_raw_markup_is_kept_as_markup() {
		let feed = read(&rss("<item><description><p>Hi <i>there</i> &amp; you</p></description></item>"));
		assert_eq!(feed.items[0].body_html, "<p>Hi <i>there</i> &amp; you</p>");
	}

	#[test]
	fn rss_091_is_read() {
		let feed = read(
			r#"<rss version="0.91"><channel><title>Old</title><item><title>One</title><link>https://old.example/1</link></item></channel></rss>"#,
		);
		assert_eq!(feed.title, "Old");
		assert_eq!(feed.items[0].link.as_deref(), Some("https://old.example/1"));
	}

	#[test]
	fn rss_1_items_beside_the_channel_are_read() {
		let feed = read(concat!(
			r#"<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns="http://purl.org/rss/1.0/" xmlns:dc="http://purl.org/dc/elements/1.1/">"#,
			r#"<channel rdf:about="https://rdf.example/"><title>RDF Site</title><link>https://rdf.example/</link><description>About</description></channel>"#,
			r#"<item rdf:about="https://rdf.example/1"><title>One</title><link>https://rdf.example/1</link><description>Body</description><dc:creator>Ann</dc:creator><dc:date>2026-10-06</dc:date></item>"#,
			r#"<item rdf:about="https://rdf.example/2"><title>Two</title></item>"#,
			"</rdf:RDF>",
		));
		assert_eq!(feed.title, "RDF Site");
		assert_eq!(feed.description_html, "About");
		assert_eq!(feed.items.len(), 2);
		assert_eq!(
			feed.items[0],
			FeedItem {
				title: "One".to_string(),
				link: Some("https://rdf.example/1".to_string()),
				author: "Ann".to_string(),
				date: "2026-10-06".to_string(),
				body_html: "Body".to_string(),
			}
		);
		assert_eq!(feed.items[1].link.as_deref(), Some("https://rdf.example/2"));
	}

	#[test]
	fn atom_feed_and_entry_are_read() {
		let feed = read(&atom(concat!(
			r#"<title>Releases</title><subtitle>What changed</subtitle>"#,
			r#"<link rel="self" href="https://code.example/releases.atom"/><link rel="alternate" type="text/html" href="https://code.example/releases"/>"#,
			"<author><name>Ann</name></author><author><name>Bob</name></author>",
			r#"<entry><title>v1.0</title><link rel="enclosure" href="https://code.example/v1.zip"/><link href="https://code.example/v1"/>"#,
			"<updated>2026-10-07T00:00:00Z</updated><published>2026-10-06T00:00:00Z</published>",
			r#"<content type="html">&lt;p&gt;Notes&lt;/p&gt;</content></entry>"#,
		)));
		assert_eq!(feed.title, "Releases");
		assert_eq!(feed.description_html, "<p>What changed</p>");
		assert_eq!(feed.link.as_deref(), Some("https://code.example/releases"));
		assert_eq!(feed.author, "Ann, Bob");
		let entry = &feed.items[0];
		assert_eq!(entry.title, "v1.0");
		assert_eq!(entry.link.as_deref(), Some("https://code.example/v1"));
		assert_eq!(entry.date, "2026-10-06T00:00:00Z");
		assert_eq!(entry.author, "Ann, Bob");
		assert_eq!(entry.body_html, "<p>Notes</p>");
	}

	#[test]
	fn atom_entry_falls_back_to_updated_and_its_own_author() {
		let feed = read(&atom(
			"<author><name>Feed Author</name></author><entry><author><name>Cy</name></author><updated>2026-10-07T00:00:00Z</updated></entry>",
		));
		assert_eq!(feed.items[0].date, "2026-10-07T00:00:00Z");
		assert_eq!(feed.items[0].author, "Cy");
	}

	#[test]
	fn atom_text_content_is_escaped_into_paragraphs() {
		let feed = read(&atom("<entry><content>Line one\nLine two\n\nFish &amp; &lt;chips&gt;</content></entry>"));
		assert_eq!(feed.items[0].body_html, "<p>Line one<br>Line two</p><p>Fish &amp; &lt;chips&gt;</p>");
	}

	#[test]
	fn atom_xhtml_content_is_written_back_out_without_its_wrapper() {
		let feed = read(&atom(
			r#"<entry><content type="xhtml"><div xmlns="http://www.w3.org/1999/xhtml"><p>Hi <em>there</em> &amp; you</p></div></content></entry>"#,
		));
		assert_eq!(feed.items[0].body_html, "<p>Hi <em>there</em> &amp; you</p>");
	}

	#[test]
	fn atom_xhtml_empty_elements_get_a_closing_tag() {
		let feed = read(&atom(
			r#"<entry><content type="xhtml"><div xmlns="http://www.w3.org/1999/xhtml"><p><a id="fn1"/>Body<br/></p></div></content></entry>"#,
		));
		assert_eq!(feed.items[0].body_html, r#"<p><a id="fn1"></a>Body<br/></p>"#);
	}

	#[test]
	fn atom_content_kept_elsewhere_falls_back_to_the_summary() {
		let feed = read(&atom(
			r#"<entry><summary type="html">&lt;b&gt;Short&lt;/b&gt;</summary><content type="text/html" src="https://code.example/full"/></entry>"#,
		));
		assert_eq!(feed.items[0].body_html, "<b>Short</b>");
	}

	#[test]
	fn atom_html_title_is_reduced_to_text() {
		let feed = read(&atom(r#"<entry><title type="html">&lt;b&gt;Bold&lt;/b&gt; move</title></entry>"#));
		assert_eq!(feed.items[0].title, "Bold move");
	}

	#[test]
	fn a_document_that_is_not_a_feed_is_refused() {
		let doc = XmlDocument::parse("<html><body><p>Not a feed</p></body></html>").unwrap();
		let err = read_feed(&doc).expect_err("an HTML page is not a feed");
		assert!(err.to_string().contains("not an RSS or Atom feed"), "got {err}");
	}
}
