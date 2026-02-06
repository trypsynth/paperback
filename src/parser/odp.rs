use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{Parser, path::extract_title_from_path, xml::collect_element_text},
	types::LinkInfo,
	zip::read_zip_entry_by_name,
};

pub struct OdpParser;

impl Parser for OdpParser {
	fn name(&self) -> &'static str {
		"OpenDocument Presentations"
	}

	fn extensions(&self) -> &[&str] {
		&["odp"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::empty()
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
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
			anyhow::bail!("ODP file does not contain any pages");
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
			}
		}
		let title = extract_title_from_path(&context.file_path);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
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
