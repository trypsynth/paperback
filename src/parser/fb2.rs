use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{
		Parser,
		toc::heading_level_to_marker_type,
		xml::{collect_element_text, find_child_element},
	},
	xml_to_text::XmlToText,
};

type Metadata = (String, String);

pub struct Fb2Parser;

impl Parser for Fb2Parser {
	fn name(&self) -> &'static str {
		"FictionBook Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["fb2"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_SECTIONS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		const CLOSING_TAG: &str = "</FictionBook>";
		let mut xml_content = fs::read_to_string(&context.file_path)
			.with_context(|| format!("Failed to read FB2 file '{}'", context.file_path))?;
		if xml_content.is_empty() {
			anyhow::bail!("FB2 file is empty");
		}
		if let Some(pos) = xml_content.rfind(CLOSING_TAG) {
			xml_content.truncate(pos + CLOSING_TAG.len());
		}
		let (xml_content, (title, author)) = clean_fb2(&xml_content).unwrap_or_else(|| {
			let (title, author) = extract_metadata(&xml_content);
			(xml_content, (title, author))
		});
		let mut converter = XmlToText::new();
		if !converter.convert(&xml_content) {
			anyhow::bail!("Failed to convert FB2 XML to text");
		}
		let mut buffer = DocumentBuffer::new();
		buffer.append(&converter.get_text());
		for heading in converter.get_headings() {
			let marker_type = heading_level_to_marker_type(heading.level);
			buffer.add_marker(
				Marker::new(marker_type, heading.offset).with_text(heading.text.clone()).with_level(heading.level),
			);
		}
		for offset in converter.get_section_offsets() {
			buffer.add_marker(Marker::new(MarkerType::SectionBreak, *offset));
		}
		for table in converter.get_tables() {
			buffer.add_marker(
				Marker::new(MarkerType::Table, table.offset)
					.with_text(table.text.clone())
					.with_reference(table.html_content.clone())
					.with_length(table.length),
			);
		}
		for link in converter.get_links() {
			buffer.add_marker(
				Marker::new(MarkerType::Link, link.offset)
					.with_text(link.text.clone())
					.with_reference(link.reference.clone()),
			);
		}
		let id_positions: HashMap<String, usize> = converter.get_id_positions().clone();
		let mut document = Document::new().with_title(title).with_author(author);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		Ok(document)
	}
}

fn clean_fb2(xml_content: &str) -> Option<(String, Metadata)> {
	let doc = XmlDocument::parse(xml_content).ok()?;
	let mut result = String::new();
	serialize_without_binary(doc.root(), &mut result);
	let meta = extract_metadata_from_doc(&doc);
	Some((result, meta))
}

fn serialize_without_binary(node: Node, output: &mut String) {
	match node.node_type() {
		NodeType::Root => {
			for child in node.children() {
				serialize_without_binary(child, output);
			}
		}
		NodeType::Element => {
			let tag_name = node.tag_name().name();
			if tag_name == "binary" {
				return;
			}
			output.push('<');
			output.push_str(tag_name);
			for attr in node.attributes() {
				output.push(' ');
				output.push_str(attr.name());
				output.push_str("=\"");
				output.push_str(&escape_xml(attr.value()));
				output.push('"');
			}
			if node.children().count() == 0 {
				output.push_str("/>");
			} else {
				output.push('>');
				for child in node.children() {
					serialize_without_binary(child, output);
				}
				output.push_str("</");
				output.push_str(tag_name);
				output.push('>');
			}
		}
		NodeType::Text => {
			if let Some(text) = node.text() {
				output.push_str(&escape_xml(text));
			}
		}
		NodeType::Comment => {
			if let Some(text) = node.text() {
				output.push_str("<!--");
				output.push_str(text);
				output.push_str("-->");
			}
		}
		NodeType::PI => {
			if let Some(text) = node.text() {
				output.push_str("<?");
				output.push_str(text);
				output.push_str("?>");
			}
		}
	}
}

fn escape_xml(s: &str) -> String {
	if !s.chars().any(|c| matches!(c, '&' | '<' | '>' | '"' | '\'')) {
		return s.to_string();
	}
	let mut result = String::with_capacity(s.len());
	for c in s.chars() {
		match c {
			'&' => result.push_str("&amp;"),
			'<' => result.push_str("&lt;"),
			'>' => result.push_str("&gt;"),
			'"' => result.push_str("&quot;"),
			'\'' => result.push_str("&apos;"),
			_ => result.push(c),
		}
	}
	result
}

fn extract_metadata(xml_content: &str) -> Metadata {
	XmlDocument::parse(xml_content)
		.map_or_else(|_| (String::new(), String::new()), |doc| extract_metadata_from_doc(&doc))
}

fn extract_metadata_from_doc(doc: &XmlDocument<'_>) -> Metadata {
	let mut title = String::new();
	let mut author = String::new();
	if let Some(title_node) =
		find_element_by_path(doc.root(), &["FictionBook", "description", "title-info", "book-title"])
	{
		title = collect_element_text(title_node);
	}
	if let Some(author_node) = find_element_by_path(doc.root(), &["FictionBook", "description", "title-info", "author"])
	{
		let first_name = find_child_element(author_node, "first-name").map(collect_element_text).unwrap_or_default();
		let last_name = find_child_element(author_node, "last-name").map(collect_element_text).unwrap_or_default();
		if !first_name.is_empty() {
			author.push_str(&first_name);
		}
		if !last_name.is_empty() {
			if !author.is_empty() {
				author.push(' ');
			}
			author.push_str(&last_name);
		}
		author = author.trim().to_string();
	}
	(title, author)
}

fn find_element_by_path<'a, 'input>(node: Node<'a, 'input>, path: &[&str]) -> Option<Node<'a, 'input>> {
	if path.is_empty() {
		return Some(node);
	}
	let target = path[0];
	let remaining = &path[1..];
	for child in node.children() {
		if child.node_type() == NodeType::Element {
			let tag_name = child.tag_name().name();
			if tag_name == target {
				if remaining.is_empty() {
					return Some(child);
				}
				return find_element_by_path(child, remaining);
			}
		}
	}
	None
}
