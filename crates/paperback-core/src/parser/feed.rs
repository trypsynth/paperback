//! RSS and Atom feeds: each item becomes a section with its own heading, read through the HTML converter.

mod date;
mod href;
mod read;

use std::collections::HashMap;

use anyhow::{Context, Result};
use rayon::prelude::*;
use roxmltree::Document as XmlDocument;

use self::{
	date::feed_date_text,
	href::resolve_feed_href,
	read::{FEED_PREFIXES, Feed, FeedItem, read_feed},
};
use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser, add_converter_markers_excluding_links,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		util::{
			path::extract_title_from_path,
			xml::{escape_xml, read_xml_to_string, repair_xml},
		},
	},
	t,
};

/// The most characters an untitled item takes from the start of its text for its name.
const UNTITLED_NAME_LEN: usize = 80;

pub struct FeedParser;

impl Parser for FeedParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing feed");
		let xml = read_xml_to_string(&context.file_path)
			.with_context(|| format!("Failed to read feed '{}'", context.file_path))?;
		let feed = parse_feed_xml(&xml)?;
		let document = build_document(&feed, context);
		tracing::debug!(path = %context.file_path, items = feed.items.len(), "parsed feed");
		Ok(document)
	}
}

/// Reads the feed in `xml`, repairing undeclared prefixes and HTML entities when it is not well-formed as it stands.
fn parse_feed_xml(xml: &str) -> Result<Feed> {
	let error = match XmlDocument::parse(xml) {
		Ok(doc) => return read_feed(&doc),
		Err(error) => error,
	};
	if let Some(repaired) = repair_xml(xml, FEED_PREFIXES)
		&& let Ok(doc) = XmlDocument::parse(&repaired)
	{
		return read_feed(&doc);
	}
	// TRANSLATORS: Error shown when a feed file is not well-formed XML; {} is the XML parser's description of the problem
	Err(anyhow::anyhow!(t("Couldn't read the feed: {}").replace("{}", &error.to_string())))
}

/// One stretch of the document: the feed's introduction, or one item.
struct Part {
	/// The section's key in `spine_items`, `manifest_items` and the section-scoped ids.
	key: String,
	/// The item's name, for its table of contents entry; `None` for the introduction.
	name: Option<String>,
	html: String,
	/// The web address the part's relative links resolve against.
	base: Option<String>,
}

fn build_document(feed: &Feed, context: &ParserContext) -> Document {
	let parts = document_parts(feed);
	let converted: Vec<HtmlToText> = parts
		.par_iter()
		.map(|part| {
			let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
			converter.convert(&part.html, HtmlSourceMode::NativeHtml);
			converter
		})
		.collect();
	let (mut buffer, spans) = DocumentBuffer::from_parts(converted.iter().map(HtmlToText::get_text).collect());
	let mut id_positions = HashMap::new();
	let mut toc_items = Vec::new();
	let mut manifest_items = HashMap::new();
	for ((part, converter), span) in parts.iter().zip(&converted).zip(&spans) {
		let start = span.start;
		let label = part.name.clone().unwrap_or_else(|| feed.title.clone());
		buffer
			.add_marker(Marker::new(MarkerType::SectionBreak, start).with_text(label).with_reference(part.key.clone()));
		add_converter_markers_excluding_links(&mut buffer, converter, start);
		for link in converter.get_links() {
			buffer.add_marker(
				Marker::new(MarkerType::Link, start + link.offset)
					.with_text(link.text.clone())
					.with_reference(resolve_feed_href(part.base.as_deref(), &link.reference)),
			);
		}
		for (id, relative) in converter.get_id_positions() {
			let absolute = start + relative;
			id_positions.entry(id.clone()).or_insert(absolute);
			id_positions.insert(format!("{}#{id}", part.key), absolute);
		}
		if let Some(name) = &part.name {
			toc_items.push(TocItem::new(name.clone(), part.key.clone(), start));
		}
		manifest_items.insert(part.key.clone(), part.key.clone());
	}
	let title = if feed.title.is_empty() { extract_title_from_path(&context.file_path) } else { feed.title.clone() };
	let mut document = Document::new().with_title(title).with_author(feed.author.clone());
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	document.spine_items = parts.into_iter().map(|part| part.key).collect();
	document.manifest_items = manifest_items;
	document
}

/// The introduction, when the feed has a description or a website, then one part per item; or a single line saying the feed is empty when it has none of those.
fn document_parts(feed: &Feed) -> Vec<Part> {
	let mut parts = Vec::with_capacity(feed.items.len() + 1);
	if let Some(html) = intro_html(feed) {
		parts.push(Part { key: "intro".to_string(), name: None, html, base: feed.link.clone() });
	}
	for (index, item) in feed.items.iter().enumerate() {
		let name = item_name(item);
		parts.push(Part {
			key: format!("item-{}", index + 1),
			html: item_html(item, &name),
			name: Some(name),
			base: item.link.clone().or_else(|| feed.link.clone()),
		});
	}
	if parts.is_empty() {
		// TRANSLATORS: The whole text of a feed that has no items and no description
		let html = format!("<p>{}</p>", escape_xml(&t("This feed has no items.")));
		parts.push(Part { key: "intro".to_string(), name: None, html, base: None });
	}
	parts
}

fn intro_html(feed: &Feed) -> Option<String> {
	if feed.description_html.trim().is_empty() && feed.link.is_none() {
		return None;
	}
	let mut html = block(&feed.description_html);
	if let Some(link) = &feed.link {
		// TRANSLATORS: Link at the start of a feed to the website the feed comes from
		html.push_str(&link_paragraph(link, &t("Website")));
	}
	Some(html)
}

/// `html` wrapped in a `div`, which ends the line any bare text in it is on.
fn block(html: &str) -> String {
	format!("<div>{html}</div>")
}

fn item_html(item: &FeedItem, name: &str) -> String {
	let mut html = format!("<h1>{}</h1>", escape_xml(name));
	if let Some(byline) = byline(item) {
		html.push_str("<p>");
		html.push_str(&escape_xml(&byline));
		html.push_str("</p>");
	}
	html.push_str(&block(&item.body_html));
	if let Some(link) = &item.link {
		// TRANSLATORS: Link at the end of a feed item to the web page the item comes from
		html.push_str(&link_paragraph(link, &t("Original article")));
	}
	html
}

fn link_paragraph(href: &str, text: &str) -> String {
	format!("<p><a href=\"{}\">{}</a></p>", escape_xml(href), escape_xml(text))
}

fn byline(item: &FeedItem) -> Option<String> {
	let published = (!item.date.trim().is_empty()).then(|| feed_date_text(&item.date));
	let writer = &item.author;
	let line = match (published, writer.is_empty()) {
		(Some(published), false) => {
			// TRANSLATORS: Line under a feed item's heading; {date} is when it was published, such as "6 October 2026", and {author} who wrote it
			t("Published {date} by {author}").replace("{date}", &published).replace("{author}", writer)
		}
		(Some(published), true) => {
			// TRANSLATORS: Line under a feed item's heading; {date} is when it was published, such as "6 October 2026"
			t("Published {date}").replace("{date}", &published)
		}
		(None, false) => {
			// TRANSLATORS: Line under a feed item's heading; {author} is who wrote it
			t("By {author}").replace("{author}", writer)
		}
		(None, true) => return None,
	};
	Some(line)
}

/// The item's title; for an item without one, the start of its text, then its date.
fn item_name(item: &FeedItem) -> String {
	if !item.title.is_empty() {
		return item.title.clone();
	}
	let text = plain_text(&item.body_html);
	if !text.is_empty() {
		return start_of(&text);
	}
	if !item.date.trim().is_empty() {
		return feed_date_text(&item.date);
	}
	// TRANSLATORS: Heading of a feed item that has no title, no text and no date
	t("Untitled item")
}

fn plain_text(html: &str) -> String {
	let mut converter = HtmlToText::new();
	converter.convert(html, HtmlSourceMode::NativeHtml);
	converter.get_text().split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `text` cut after the last whole word that fits in [`UNTITLED_NAME_LEN`] characters, with any dots it ends in replaced by one ellipsis, or all of it when it fits.
fn start_of(text: &str) -> String {
	if text.chars().count() <= UNTITLED_NAME_LEN {
		return text.to_string();
	}
	let mut name = String::new();
	for word in text.split(' ') {
		let separator = usize::from(!name.is_empty());
		if name.chars().count() + separator + word.chars().count() > UNTITLED_NAME_LEN {
			break;
		}
		if separator == 1 {
			name.push(' ');
		}
		name.push_str(word);
	}
	if name.is_empty() {
		name = text.chars().take(UNTITLED_NAME_LEN).collect();
	}
	name.truncate(name.trim_end_matches(['.', '…']).len());
	name.push('…');
	name
}

#[cfg(test)]
mod tests;
