use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser, add_converter_markers,
		util::xml::{collect_element_text, find_child_element},
		xml_to_text::XmlToText,
	},
	t,
};

type Metadata = (String, String);

pub struct Fb2Parser;

impl Parser for Fb2Parser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		const CLOSING_TAG: &str = "</FictionBook>";
		let mut xml_content = fs::read_to_string(&context.file_path)
			.with_context(|| format!("Failed to read FB2 file '{}'", context.file_path))?;
		if let Some(pos) = xml_content.rfind(CLOSING_TAG) {
			xml_content.truncate(pos + CLOSING_TAG.len());
		}
		let (xml_content, (title, author)) = clean_fb2(&xml_content).unwrap_or_else(|| {
			let (title, author) = extract_metadata(&xml_content);
			(xml_content, (title, author))
		});
		let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
		if !converter.convert(&xml_content) {
			// TRANSLATORS: Error shown when an FB2 (FictionBook) file's XML fails to convert to plain text
			anyhow::bail!(t("Failed to convert FB2 XML to text"));
		}
		let mut buffer = DocumentBuffer::new();
		buffer.append(&converter.get_text());
		add_converter_markers(&mut buffer, &converter, 0);
		for offset in converter.get_section_offsets() {
			buffer.add_marker(Marker::new(MarkerType::SectionBreak, *offset));
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	const BODY: &str = r#"<body><section><title><p>Chapter One</p></title><p>Opening line.</p></section></body>"#;

	fn fb2_document(description: &str, body: &str) -> String {
		format!(
			r#"<?xml version="1.0" encoding="UTF-8"?>
<FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0"><description>{description}</description>{body}</FictionBook>"#
		)
	}

	fn parse_fb2(contents: &str) -> Result<Document> {
		let dir = TempDir::new("fb2-parser");
		let path = dir.write_str("book.fb2", contents);
		Fb2Parser.parse(&ParserContext::new(path))
	}

	fn parse_ok(contents: &str) -> Document {
		parse_fb2(contents).expect("parse fb2 document")
	}

	#[test]
	fn reads_title_and_author_from_the_description() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>A Fine Book</book-title><author><first-name>Ada</first-name><last-name>Lovelace</last-name></author></title-info>",
			BODY,
		));
		assert_eq!(doc.title, "A Fine Book");
		assert_eq!(doc.author, "Ada Lovelace");
	}

	#[test]
	fn joins_a_partial_author_name_without_stray_spaces() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>T</book-title><author><last-name>Plato</last-name></author></title-info>",
			BODY,
		));
		assert_eq!(doc.author, "Plato");
	}

	#[test]
	fn extracts_body_text() {
		let doc = parse_ok(&fb2_document("<title-info><book-title>T</book-title></title-info>", BODY));
		assert!(doc.buffer.content.contains("Opening line."), "text: {:?}", doc.buffer.content);
		assert!(!doc.buffer.content.contains('<'), "markup leaked into text: {:?}", doc.buffer.content);
	}

	#[test]
	fn marks_each_section_with_a_section_break() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>T</book-title></title-info>",
			"<body><section><p>One.</p></section><section><p>Two.</p></section></body>",
		));
		let breaks = doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::SectionBreak).count();
		assert_eq!(breaks, 2);
	}

	/// Cover images arrive as base64 `<binary>` blobs. They must not be decoded into the text,
	/// which is what `clean_fb2` strips them for.
	#[test]
	fn drops_base64_binary_payloads() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>T</book-title></title-info>",
			r#"<body><section><p>Visible.</p></section></body><binary id="cover.jpg" content-type="image/jpeg">iVBORw0KGgoAAAANSUhEUg==</binary>"#,
		));
		assert!(doc.buffer.content.contains("Visible."));
		assert!(!doc.buffer.content.contains("iVBORw0KGgo"), "binary payload leaked: {:?}", doc.buffer.content);
	}

	/// Some writers append junk after the closing tag; the parser truncates there rather than
	/// failing, so the document still opens.
	#[test]
	fn ignores_trailing_junk_after_the_closing_tag() {
		let doc = parse_ok(&format!(
			"{}\u{0}garbage",
			fb2_document("<title-info><book-title>Trailing</book-title></title-info>", BODY)
		));
		assert_eq!(doc.title, "Trailing");
	}

	#[test]
	fn reports_the_path_when_the_file_is_missing() {
		let dir = TempDir::new("fb2-parser");
		let missing = dir.join_str("nope.fb2");
		let err = Fb2Parser.parse(&ParserContext::new(missing.clone())).expect_err("missing file must fail");
		assert!(err.to_string().contains(&missing), "error should name the file: {err}");
	}
}
