use std::collections::HashMap;

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};

use crate::{
	document::{DocLen, DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser,
		odf_crypto::read_odf_content,
		util::{odf, path::extract_title_from_path, slides::no_text_placeholder, xml::read_xml_to_string},
	},
	t,
	types::LinkInfo,
};

pub struct OdpParser;

impl Parser for OdpParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing odp file");
		// No `.context()` here: it would wrap the `[password_required]` sentinel an encrypted file returns, and the layers above match on that sentinel at the top of the error.
		let content_str = read_odf_content(&context.file_path, "content.xml", context.password.as_deref())?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid ODP content.xml")?;
		let mut buffer = DocumentBuffer::new();
		let id_positions = HashMap::new();
		let pages = find_all_pages(xml_doc.root());
		if pages.is_empty() {
			tracing::warn!(path = %context.file_path, "odp file has no pages");
			// TRANSLATORS: Error shown when an ODP presentation file has no pages/slides
			anyhow::bail!(t("ODP file does not contain any pages"));
		}
		build_slides(&pages, &mut buffer);
		let (title, author) = odf::packaged_metadata(&context.file_path, context.password.as_deref());
		let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		tracing::debug!(path = %context.file_path, "parsed odp file successfully");
		Ok(document)
	}
}

/// One slide per page, each opening with its page break, and a slide with no text still there to be reached.
fn build_slides(pages: &[Node], buffer: &mut DocumentBuffer) {
	for (index, page_node) in pages.iter().enumerate() {
		let slide_start = buffer.end();
		let slide = get_page_text(*page_node, slide_start);
		if slide.text.trim().is_empty() {
			buffer.append(&no_text_placeholder());
		} else {
			buffer.append(&slide.text);
		}
		if !buffer.content.ends_with('\n') {
			buffer.append("\n");
		}
		if !slide.notes.is_empty() {
			// TRANSLATORS: Written after a presentation slide's text, before the presenter's notes for that slide; {} is the notes
			buffer.append(&t("Speaker notes: {}").replace("{}", &slide.notes.join("\n")));
			buffer.append("\n");
		}
		for (author, text) in slide.comments {
			let comment = if author.is_empty() {
				// TRANSLATORS: Written after a presentation slide, for a comment a reviewer left on it without a name; {} is the comment
				t("Comment: {}").replace("{}", &text)
			} else {
				// TRANSLATORS: Written after a presentation slide, for a comment a reviewer left on it: the first {} is the reviewer, the second the comment
				t("Comment from {}: {}").replacen("{}", &author, 1).replacen("{}", &text, 1)
			};
			buffer.append(&comment);
			buffer.append("\n");
		}
		buffer.add_marker(Marker::new(MarkerType::PageBreak, slide_start).with_text(format!("Slide {}", index + 1)));
		for (offset, description, length) in slide.images {
			buffer.add_marker(Marker::new(MarkerType::Image, offset).with_text(description).with_length(length));
		}
		for link in slide.links {
			buffer.add_marker(
				Marker::new(MarkerType::Link, link.offset).with_text(link.text).with_reference(link.reference),
			);
		}
	}
}

pub struct FodpParser;

impl Parser for FodpParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing fodp file");
		let content_str = read_xml_to_string(&context.file_path)
			.with_context(|| format!("Failed to open FODP file '{}'", context.file_path))?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid FODP document")?;
		let mut buffer = DocumentBuffer::new();
		let id_positions = HashMap::new();
		let pages = find_all_pages(xml_doc.root());
		if pages.is_empty() {
			tracing::warn!(path = %context.file_path, "fodp file has no pages");
			// TRANSLATORS: Error shown when a flat-XML ODP presentation file has no pages/slides
			anyhow::bail!(t("FODP file does not contain any pages"));
		}
		build_slides(&pages, &mut buffer);
		let (title, author) = odf::metadata(&xml_doc);
		let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		tracing::debug!(path = %context.file_path, "parsed fodp file successfully");
		Ok(document)
	}
}

fn find_all_pages<'a, 'input>(node: Node<'a, 'input>) -> Vec<Node<'a, 'input>> {
	let mut pages = Vec::new();
	collect_pages(node, &mut pages);
	pages
}

fn collect_pages<'a, 'input>(node: Node<'a, 'input>, pages: &mut Vec<Node<'a, 'input>>) {
	if node.node_type() == NodeType::Element && node.tag_name().name() == "page" {
		pages.push(node);
	}
	for child in node.children() {
		collect_pages(child, pages);
	}
}

/// What one slide holds: its text with the links and pictures in it, and the presenter's notes and reviewers' comments, kept apart from the text they used to run into.
#[derive(Default)]
struct Slide {
	text: String,
	links: Vec<LinkInfo>,
	/// Each picture's position, description and length.
	images: Vec<(DocPos, String, DocLen)>,
	notes: Vec<String>,
	/// Each comment's author, which may be empty, and text.
	comments: Vec<(String, String)>,
}

fn get_page_text(page_node: Node, slide_start: DocPos) -> Slide {
	let mut slide = Slide::default();
	traverse_page(page_node, &mut slide, slide_start);
	slide
}

fn traverse_page(node: Node, slide: &mut Slide, slide_start: DocPos) {
	if node.is_text() {
		slide.text.push_str(node.text().unwrap_or_default());
		return;
	}
	if node.is_element() {
		if let Some(spaces) = odf::spacing(node) {
			slide.text.push_str(&spaces);
			return;
		}
		let tag_name = node.tag_name().name();
		if tag_name == "annotation" {
			let child_text = |name: &str| {
				node.children()
					.filter(|child| child.is_element() && child.tag_name().name() == name)
					.map(|child| odf::plain_text(child).trim().to_string())
					.filter(|text| !text.is_empty())
					.collect::<Vec<_>>()
					.join(" ")
			};
			let text = child_text("p");
			if !text.is_empty() {
				slide.comments.push((child_text("creator"), text));
			}
			return;
		}
		if odf::is_hidden(node) {
			return;
		}
		if tag_name == "notes" {
			slide.notes.extend(
				node.descendants()
					.filter(|descendant| descendant.is_element() && descendant.tag_name().name() == "p")
					.map(|paragraph| odf::plain_text(paragraph).trim().to_string())
					.filter(|paragraph| !paragraph.is_empty()),
			);
			return;
		}
		if let Some((placeholder, description)) = odf::picture(node) {
			slide.images.push((slide_start + DocLen::of(&slide.text), description, DocLen::of(&placeholder)));
			slide.text.push_str(&placeholder);
			return;
		}
		if tag_name == "a" {
			if let Some(href) = node.attribute("href") {
				let link_offset = slide_start + DocLen::of(&slide.text);
				let link_text = odf::plain_text(node);
				if !link_text.is_empty() {
					slide.text.push_str(&link_text);
					slide.links.push(LinkInfo { offset: link_offset, text: link_text, reference: href.to_string() });
				}
			}
			return;
		}
		if tag_name == "p" {
			traverse_children(node, slide, slide_start);
			if !slide.text.ends_with('\n') {
				slide.text.push('\n');
			}
			return;
		}
	}
	traverse_children(node, slide, slide_start);
}

fn traverse_children(node: Node, slide: &mut Slide, slide_start: DocPos) {
	for child in node.children() {
		traverse_page(child, slide, slide_start);
	}
}

#[cfg(test)]
mod tests {
	use std::io::{Cursor, Write};

	use zip::{ZipWriter, write::FileOptions};

	use super::*;
	use crate::util::test_support::TempDir;

	/// Flat ODP: the same content tree an `.odp` carries in `content.xml`, stored as one XML file.
	fn fodp_document(pages: &str) -> String {
		format!(
			r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
	xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
	xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
	xmlns:xlink="http://www.w3.org/1999/xlink"
	xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0"
	xmlns:presentation="urn:oasis:names:tc:opendocument:xmlns:presentation:1.0"
	xmlns:dc="http://purl.org/dc/elements/1.1/">
<office:body><office:presentation>{pages}</office:presentation></office:body></office:document>"#
		)
	}

	fn page(paragraphs: &str) -> String {
		format!(r"<draw:page><draw:frame><draw:text-box>{paragraphs}</draw:text-box></draw:frame></draw:page>")
	}

	fn parse_fodp(contents: &str) -> Result<Document> {
		let dir = TempDir::new("fodp-parser");
		let path = dir.write_str("deck.fodp", contents);
		FodpParser.parse(&ParserContext::new(path))
	}

	fn parse_ok(contents: &str) -> Document {
		parse_fodp(contents).expect("parse fodp document")
	}

	#[test]
	fn extracts_slide_text_one_paragraph_per_line() {
		let doc = parse_ok(&fodp_document(&page("<text:p>Title slide</text:p><text:p>Subtitle</text:p>")));
		assert_eq!(doc.buffer.content, "Title slide\nSubtitle\n");
	}

	#[test]
	fn marks_each_slide_with_a_numbered_page_break() {
		let slides = format!("{}{}", page("<text:p>One</text:p>"), page("<text:p>Two</text:p>"));
		let doc = parse_ok(&fodp_document(&slides));
		let labels: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(labels, vec!["Slide 1", "Slide 2"]);
	}

	#[test]
	fn records_links_at_the_offset_of_their_text() {
		let doc = parse_ok(&fodp_document(&page(
			r#"<text:p>See <text:a xlink:href="https://example.com">the site</text:a></text:p>"#,
		)));
		let link = doc.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Link).expect("link marker");
		assert_eq!(link.text, "the site");
		assert_eq!(link.reference, "https://example.com");
		assert_eq!(&doc.buffer.content[link.position.get()..link.position.get() + link.text.len()], "the site");
	}

	/// A link's position is counted in chars, which an emoji before it makes differ from bytes.
	#[test]
	fn a_link_after_an_emoji_lands_on_its_text() {
		let doc = parse_ok(&fodp_document(&page(
			"<text:p>\u{1F600} <text:a xlink:href=\"https://example.com\">the site</text:a></text:p>",
		)));
		let link = doc.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Link).expect("link marker");
		assert_eq!(link.position, DocLen::of("\u{1F600} ").as_pos());
	}

	/// A slide with no text, usually one of pictures alone, is still a slide of the deck: it says so rather than vanishing, which would also leave the reader jumping from one slide number to the one after next.
	#[test]
	fn a_slide_with_no_text_says_so() {
		let slides = format!("{}{}", page("<text:p>   </text:p>"), page("<text:p>Real content</text:p>"));
		let doc = parse_ok(&fodp_document(&slides));
		let labels: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(labels, vec!["Slide 1", "Slide 2"]);
		assert_eq!(doc.buffer.content, "This slide has no text.\nReal content\n");
	}

	#[test]
	fn rejects_a_presentation_with_no_pages() {
		let err = parse_fodp(&fodp_document("")).expect_err("a deck with no slides must fail");
		assert!(err.to_string().contains("does not contain any pages"), "unexpected error: {err}");
	}

	/// The zipped and flat parsers share their page walking but differ in how they get at the
	/// content, so the `.odp` path needs its own coverage.
	#[test]
	fn reads_the_same_content_from_a_zipped_odp() {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			writer.start_file("content.xml", FileOptions::<()>::default()).expect("start entry");
			writer
				.write_all(fodp_document(&page("<text:p>Zipped slide</text:p>")).as_bytes())
				.expect("write content.xml");
			writer.finish().expect("finish zip");
		}
		let dir = TempDir::new("odp-parser");
		let path = dir.write_str("deck.odp", cursor.into_inner());
		let doc = OdpParser.parse(&ParserContext::new(path)).expect("parse odp document");
		assert_eq!(doc.buffer.content, "Zipped slide\n");
	}

	#[test]
	fn rejects_an_odp_without_content_xml() {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			writer.start_file("mimetype", FileOptions::<()>::default()).expect("start entry");
			writer.write_all(b"application/vnd.oasis.opendocument.presentation").expect("write mimetype");
			writer.finish().expect("finish zip");
		}
		let dir = TempDir::new("odp-parser");
		let path = dir.write_str("deck.odp", cursor.into_inner());
		let err = OdpParser.parse(&ParserContext::new(path)).expect_err("an odp with no content.xml must fail");
		assert!(err.to_string().contains("content.xml"), "unexpected error: {err}");
	}

	#[test]
	fn rejects_malformed_xml() {
		let err = parse_fodp("<office:document><unclosed>").expect_err("malformed xml must fail");
		assert!(err.to_string().contains("Invalid FODP document"), "unexpected error: {err}");
	}

	/// Alt text, comments and the presenter's notes all used to run into the slide's words unlabelled.
	#[test]
	fn a_slide_keeps_its_own_words_and_labels_its_notes() {
		let slide = r"<draw:page><draw:frame><draw:text-box><text:p>Sales<text:s/>grew<office:annotation><dc:creator>Ann</dc:creator><text:p>Check this</text:p></office:annotation> fast</text:p></draw:text-box></draw:frame><draw:frame><draw:image/><svg:title>Bar chart</svg:title></draw:frame><presentation:notes><draw:frame><draw:text-box><text:p>Mention Q3.</text:p><text:p>Pause here.</text:p></draw:text-box></draw:frame></presentation:notes></draw:page>";
		let doc = parse_ok(&fodp_document(slide));
		assert_eq!(
			doc.buffer.content,
			"Sales grew fast
[Image: Bar chart]
Speaker notes: Mention Q3.
Pause here.
Comment from Ann: Check this
"
		);
		let image = doc.buffer.markers.iter().find(|m| m.mtype == MarkerType::Image).expect("image marker");
		assert_eq!(
			(image.position, image.text.as_str()),
			(
				DocLen::of(
					"Sales grew fast
"
				)
				.as_pos(),
				"Bar chart"
			)
		);
	}

	/// A slide of one picture reads as that picture, not as a slide with no text.
	#[test]
	fn a_picture_slide_reads_as_its_picture() {
		let doc = parse_ok(&fodp_document("<draw:page><draw:frame><draw:image/></draw:frame></draw:page>"));
		assert_eq!(
			doc.buffer.content,
			"[Image]
"
		);
	}
}
