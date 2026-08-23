use std::{collections::HashMap, fs, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser,
		util::{path::extract_title_from_path, xml::collect_element_text},
	},
	t,
	types::LinkInfo,
	util::zip::read_zip_entry_by_name,
};

pub struct OdpParser;

impl Parser for OdpParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing odp file");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open ODP file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read ODP as zip '{}'", context.file_path))?;
		let content_str = read_zip_entry_by_name(&mut archive, "content.xml")
			.context("ODP file does not contain content.xml or it is empty")?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid ODP content.xml")?;
		let mut buffer = DocumentBuffer::new();
		let id_positions = HashMap::new();
		let pages = find_all_pages(xml_doc.root());
		if pages.is_empty() {
			tracing::warn!(path = %context.file_path, "odp file has no pages");
			// TRANSLATORS: Error shown when an ODP presentation file has no pages/slides
			anyhow::bail!(t("ODP file does not contain any pages"));
		}
		for (index, page_node) in pages.iter().enumerate() {
			let slide_start = buffer.current_position();
			let mut links = Vec::new();
			let slide_text = get_page_text(*page_node, &mut links, slide_start);
			if !slide_text.trim().is_empty() {
				buffer.append(&slide_text);
				if !buffer.content.ends_with('\n') {
					buffer.append("\n");
				}
				buffer.add_marker(
					Marker::new(MarkerType::PageBreak, slide_start).with_text(format!("Slide {}", index + 1)),
				);
				for link in links {
					buffer.add_marker(
						Marker::new(MarkerType::Link, link.offset).with_text(link.text).with_reference(link.reference),
					);
				}
			} else {
				tracing::debug!(slide = index + 1, "skipped odp slide with no text");
			}
		}
		let title = extract_title_from_path(&context.file_path);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		tracing::debug!(path = %context.file_path, "parsed odp file successfully");
		Ok(document)
	}
}

pub struct FodpParser;

impl Parser for FodpParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing fodp file");
		let content_str = fs::read_to_string(&context.file_path)
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
		for (index, page_node) in pages.iter().enumerate() {
			let slide_start = buffer.current_position();
			let mut links = Vec::new();
			let slide_text = get_page_text(*page_node, &mut links, slide_start);
			if !slide_text.trim().is_empty() {
				buffer.append(&slide_text);
				if !buffer.content.ends_with('\n') {
					buffer.append("\n");
				}
				buffer.add_marker(
					Marker::new(MarkerType::PageBreak, slide_start).with_text(format!("Slide {}", index + 1)),
				);
				for link in links {
					buffer.add_marker(
						Marker::new(MarkerType::Link, link.offset).with_text(link.text).with_reference(link.reference),
					);
				}
			} else {
				tracing::debug!(slide = index + 1, "skipped fodp slide with no text");
			}
		}
		let title = extract_title_from_path(&context.file_path);
		let mut document = Document::new().with_title(title);
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

fn get_page_text(page_node: Node, links: &mut Vec<LinkInfo>, slide_start: usize) -> String {
	let mut text = String::new();
	traverse_page(page_node, &mut text, links, slide_start);
	text
}

fn traverse_page(node: Node, text: &mut String, links: &mut Vec<LinkInfo>, slide_start: usize) {
	if node.node_type() == NodeType::Element {
		let tag_name = node.tag_name().name();
		if tag_name == "a" {
			if let Some(href) = node.attribute("href") {
				let link_offset = slide_start + text.len();
				let link_text = collect_element_text(node);
				if !link_text.is_empty() {
					text.push_str(&link_text);
					links.push(LinkInfo { offset: link_offset, text: link_text, reference: href.to_string() });
				}
			}
			return;
		}
		if tag_name == "p" || tag_name == "span" {
			traverse_children(node, text, links, slide_start);
			if tag_name == "p" && !text.ends_with('\n') {
				text.push('\n');
			}
			return;
		}
	} else if node.node_type() == NodeType::Text {
		if let Some(t) = node.text() {
			text.push_str(t);
		}
		return;
	}
	traverse_children(node, text, links, slide_start);
}

fn traverse_children(node: Node, text: &mut String, links: &mut Vec<LinkInfo>, slide_start: usize) {
	for child in node.children() {
		traverse_page(child, text, links, slide_start);
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
	xmlns:xlink="http://www.w3.org/1999/xlink">
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
		assert_eq!(&doc.buffer.content[link.position..link.position + link.text.len()], "the site");
	}

	/// A deck can hold empty layout placeholder slides; they'd otherwise each contribute a page
	/// break the reader would stop on with nothing to read out.
	#[test]
	fn skips_slides_with_no_text() {
		let slides = format!("{}{}", page("<text:p>   </text:p>"), page("<text:p>Real content</text:p>"));
		let doc = parse_ok(&fodp_document(&slides));
		let labels: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(labels, vec!["Slide 2"]);
		assert_eq!(doc.buffer.content, "Real content\n");
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
}
