use std::collections::HashMap;

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser, add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::xml::{collect_element_text, find_child_element, read_xml_to_string, repair_xml, serialize_xml},
	},
	t,
};

type Metadata = (String, String);

const XLINK_NS: &str = "http://www.w3.org/1999/xlink";

/// `l` and `xlink` are both how an FB2 book writes a link.
const FB2_PREFIXES: &[(&str, &str)] = &[("l", XLINK_NS), ("xlink", XLINK_NS)];

pub struct Fb2Parser;

impl Parser for Fb2Parser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing fb2 file");
		const CLOSING_TAG: &str = "</FictionBook>";
		let mut xml_content = read_xml_to_string(&context.file_path)
			.with_context(|| format!("Failed to read FB2 file '{}'", context.file_path))?;
		if let Some(pos) = xml_content.rfind(CLOSING_TAG) {
			xml_content.truncate(pos + CLOSING_TAG.len());
		}
		let (xml_content, (title, author)) = clean_fb2(&xml_content)
			.or_else(|| clean_fb2(&repair_xml(&xml_content, FB2_PREFIXES)?))
			.unwrap_or_else(|| {
				tracing::warn!(
					path = %context.file_path,
					"roxmltree failed to parse fb2 xml, falling back to unstripped xml which may include base64 binary blobs"
				);
				let (title, author) = extract_metadata(&xml_content);
				(xml_content, (title, author))
			});
		let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
		if !converter.convert(&xml_content) {
			tracing::warn!(path = %context.file_path, "failed to convert fb2 xml to text");
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
		tracing::debug!(path = %context.file_path, "parsed fb2 file successfully");
		Ok(document)
	}
}

fn clean_fb2(xml_content: &str) -> Option<(String, Metadata)> {
	let doc = XmlDocument::parse(xml_content).ok()?;
	let mut result = String::new();
	serialize_xml(doc.root(), &mut result, &|node| node.tag_name().name() == "binary");
	let meta = extract_metadata_from_doc(&doc);
	Some((result, meta))
}

fn extract_metadata(xml_content: &str) -> Metadata {
	XmlDocument::parse(xml_content).map_or_else(
		|e| {
			tracing::warn!(error = %e, "failed to parse fb2 xml for metadata, title and author will be empty");
			(String::new(), String::new())
		},
		|doc| extract_metadata_from_doc(&doc),
	)
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

	const BODY: &str = r"<body><section><title><p>Chapter One</p></title><p>Opening line.</p></section></body>";

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

	/// FictionBook came out of the Russian ebook world and most of its books are written in
	/// windows-1251, named in the XML declaration. Reading one as UTF-8 loses the whole book.
	#[test]
	fn reads_a_book_written_in_windows_1251() {
		let xml = concat!(
			r#"<?xml version="1.0" encoding="windows-1251"?>"#,
			r"<FictionBook><description><title-info><book-title>Anna</book-title></title-info></description>",
			r"<body><section><p>@@</p></section></body></FictionBook>"
		);
		// The Cyrillic for "Anna Karenina", as windows-1251 bytes rather than UTF-8 ones.
		let cyrillic: [u8; 13] = [0xC0, 0xED, 0xED, 0xE0, 0x20, 0xCA, 0xE0, 0xF0, 0xE5, 0xED, 0xE8, 0xED, 0xE0];
		let bytes = xml.as_bytes();
		let at = bytes.windows(2).position(|w| w == b"@@").expect("the placeholder");
		let mut encoded = bytes[..at].to_vec();
		encoded.extend_from_slice(&cyrillic);
		encoded.extend_from_slice(&bytes[at + 2..]);
		let dir = TempDir::new("fb2-cp1251");
		let path = dir.write_str("book.fb2", encoded);
		let doc = Fb2Parser.parse(&ParserContext::new(path)).expect("a windows-1251 book still opens");
		assert!(doc.buffer.content.contains("Анна Каренина"), "got {:?}", doc.buffer.content);
	}

	/// A FictionBook written by hand often opens with a bare `<FictionBook>` and then uses `l:href`
	/// for a footnote without declaring the prefix. Every reader of the format takes it.
	#[test]
	fn reads_a_book_that_never_declared_its_namespace_prefix() {
		let doc = parse_ok(
			r##"<?xml version="1.0" encoding="utf-8"?>
<FictionBook><description><title-info><book-title>Bare</book-title></title-info></description>
<body><section><p>See <a l:href="#n1">the note</a>.</p></section></body></FictionBook>"##,
		);
		assert!(doc.buffer.content.contains("See the note."), "got {:?}", doc.buffer.content);
		assert_eq!(doc.title, "Bare");
	}

	/// A converter that has just read HTML writes what it read, and XML declares five entities.
	#[test]
	fn resolves_the_html_entities_xml_has_never_heard_of() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>Entities</book-title></title-info>",
			"<body><section><p>Hello&nbsp;world&mdash;and&hellip;</p></section></body>",
		));
		// The no-break space resolves and then collapses with the rest of the whitespace, which is
		// what the converter does to every run of spaces in a book.
		assert!(doc.buffer.content.contains("Hello world—and…"), "got {:?}", doc.buffer.content);
	}

	/// An entity nothing knows is dropped rather than taken as a reason to lose the book. FBReader's
	/// own help files are written with an undeclared `&FBReaderVersion;` in them.
	#[test]
	fn an_entity_nothing_knows_costs_only_itself() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>Unknown</book-title></title-info>",
			"<body><section><p>Version &FBReaderVersion; of it.</p></section></body>",
		));
		assert!(doc.buffer.content.contains("Version of it."), "got {:?}", doc.buffer.content);
	}

	/// Anything at all can be handed to a parser, and a file of random bytes has to come back as an
	/// error rather than take the program down with it. This one found a panic in the namespace
	/// repair: the byte after the character that ended a name is not a place a string can be cut
	/// when that character is more than one byte long.
	#[test]
	fn random_bytes_are_an_error_and_not_a_panic() {
		let dir = TempDir::new("fb2-random");
		// A colon straight after a multi-byte character, which is the shape that panicked.
		let mut bytes = b"<FictionBook><p>".to_vec();
		bytes.extend_from_slice("\u{fffd}".as_bytes());
		bytes.extend_from_slice(b":name=\"x\"");
		let path = dir.write_str("random.fb2", bytes);
		assert!(Fb2Parser.parse(&ParserContext::new(path)).is_err(), "a file that is not a book is an error");
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
