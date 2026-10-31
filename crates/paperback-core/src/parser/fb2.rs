use std::{collections::HashMap, fmt::Write as _, fs, io::Cursor};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser, add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::{
			toc::build_toc_from_headings,
			xml::{collect_element_text, decode_xml, find_child_element},
		},
	},
	t,
	util::{text::collapse_whitespace, zip::read_zip_entry_bytes},
};

type Metadata = (String, String);

/// FictionBook is usually passed around zipped (`book.fb2.zip`), one book to an archive.
fn read_fb2_bytes(path: &str) -> Result<Vec<u8>> {
	let bytes = fs::read(path).with_context(|| format!("Failed to read FB2 file '{path}'"))?;
	if !bytes.starts_with(b"PK\x03\x04") {
		return Ok(bytes);
	}
	let mut archive =
		ZipArchive::new(Cursor::new(bytes)).with_context(|| format!("Failed to read ZIP archive '{path}'"))?;
	let Some(name) =
		archive.file_names().flatten().find(|name| name.to_ascii_lowercase().ends_with(".fb2")).map(String::from)
	else {
		// TRANSLATORS: Error shown when a ZIP file contains no readable Word document content
		anyhow::bail!(t("No readable content found in the ZIP archive"));
	};
	read_zip_entry_bytes(&mut archive, &name)
}

pub struct Fb2Parser;

impl Parser for Fb2Parser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing fb2 file");
		const CLOSING_TAG: &str = "</FictionBook>";
		let mut xml_content = decode_xml(&read_fb2_bytes(&context.file_path)?);
		if let Some(pos) = xml_content.rfind(CLOSING_TAG) {
			xml_content.truncate(pos + CLOSING_TAG.len());
		}
		let (xml_content, (title, author)) =
			clean_fb2(&xml_content).or_else(|| clean_fb2(&repair_fb2_xml(&xml_content)?)).unwrap_or_else(|| {
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
		buffer.append(converter.get_text());
		add_converter_markers(&mut buffer, &converter, DocPos::ZERO);
		for offset in converter.get_section_offsets() {
			buffer.add_marker(Marker::new(MarkerType::SectionBreak, *offset));
		}
		let id_positions: HashMap<String, DocPos> = converter.get_id_positions().clone();
		let mut document = Document::new().with_title(title).with_author(author);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = build_toc_from_headings(converter.get_headings());
		tracing::debug!(path = %context.file_path, "parsed fb2 file successfully");
		Ok(document)
	}
}

/// # Errors
///
/// Returns an error when the file cannot be read or parsed, even after repair, or holds no binary with that id.
pub fn binary_data(file_path: &str, href: &str) -> Result<String> {
	const CLOSING_TAG: &str = "</FictionBook>";
	let mut xml_content = decode_xml(&read_fb2_bytes(file_path)?);
	if let Some(pos) = xml_content.rfind(CLOSING_TAG) {
		xml_content.truncate(pos + CLOSING_TAG.len());
	}
	let repaired;
	let doc = match XmlDocument::parse(&xml_content) {
		Ok(doc) => doc,
		Err(error) => {
			repaired = repair_fb2_xml(&xml_content).ok_or(error)?;
			XmlDocument::parse(&repaired)?
		}
	};
	let id = href.strip_prefix('#').unwrap_or(href);
	let binary = doc
		.descendants()
		.find(|node| node.tag_name().name() == "binary" && node.attribute("id") == Some(id))
		.with_context(|| format!("no binary with id '{id}'"))?;
	Ok(binary.descendants().filter(Node::is_text).filter_map(|node| node.text()).collect())
}

fn clean_fb2(xml_content: &str) -> Option<(String, Metadata)> {
	let doc = XmlDocument::parse(xml_content).ok()?;
	let mut result = String::new();
	serialize_without_binary(doc.root(), &mut result, Place::default());
	let meta = extract_metadata_from_doc(&doc);
	Some((result, meta))
}

#[derive(Clone, Copy, Default)]
struct Place {
	sections: usize,
	in_body: bool,
	/// A named `<body>` such as the notes, whose numbered section titles would fill the contents with numbers as headings.
	in_notes: bool,
	/// Poem, stanza, epigraph and quotation titles are part of the text rather than the book's structure.
	in_verse: bool,
}

fn serialize_without_binary(node: Node, output: &mut String, place: Place) {
	match node.node_type() {
		NodeType::Root => {
			for child in node.children() {
				serialize_without_binary(child, output, place);
			}
		}
		NodeType::Element => {
			let tag_name = node.tag_name().name();
			if tag_name == "binary" {
				return;
			}
			let mut place = place;
			let tag_name = match tag_name {
				"body" => {
					place.in_body = true;
					place.in_notes = node.attribute("name").is_some();
					tag_name
				}
				"section" => {
					place.sections += 1;
					tag_name
				}
				"title" if place.in_body && !place.in_verse => {
					let text = escape_xml(&title_text(node));
					if place.in_notes && place.sections > 0 {
						let _ = write!(output, "<p>{text}</p>");
					} else {
						let level = (place.sections + 1).min(6);
						let _ = write!(output, "<h{level}>{text}</h{level}>");
					}
					return;
				}
				"poem" | "stanza" | "epigraph" | "cite" => {
					place.in_verse = true;
					"div"
				}
				"title" | "annotation" => "div",
				"v" | "subtitle" | "text-author" => "p",
				"empty-line" => "br",
				_ => tag_name,
			};
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
					serialize_without_binary(child, output, place);
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

/// FB2 often splits "Chapter 1" and its name into two paragraphs, which run together into "Chapter 1The Beginning" if simply joined.
fn title_text(title: Node) -> String {
	let lines: Vec<String> = title
		.children()
		.filter(|child| child.tag_name().name() == "p")
		.map(|line| collapse_whitespace(&collect_element_text(line)).trim().to_string())
		.filter(|line| !line.is_empty())
		.collect();
	if lines.is_empty() {
		return collapse_whitespace(&collect_element_text(title)).trim().to_string();
	}
	let mut text = String::new();
	for line in lines {
		if !text.is_empty() {
			let ends_a_sentence = text.ends_with(['.', ':', '!', '?', ';', '\u{2014}', '-']);
			text.push_str(if ends_a_sentence { " " } else { ": " });
		}
		text.push_str(&line);
	}
	text
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

/// FictionBooks often use a prefix such as `l:href` with no `xmlns:l`, which roxmltree refuses.
fn declare_missing_namespaces(xml: &str) -> Option<String> {
	let mut missing: Vec<&str> = Vec::new();
	for prefix in used_prefixes(xml) {
		let declaration = format!("xmlns:{prefix}");
		if !xml.contains(&declaration) && !missing.contains(&prefix) {
			missing.push(prefix);
		}
	}
	if missing.is_empty() {
		return None;
	}
	let insert_at = root_element_attribute_position(xml)?;
	let mut out = String::with_capacity(xml.len() + missing.len() * 48);
	out.push_str(&xml[..insert_at]);
	for prefix in &missing {
		// `l` and `xlink` are both how FictionBook writes a link; anything else gets its own URI so two prefixes never collide.
		let uri = if matches!(*prefix, "l" | "xlink") {
			"http://www.w3.org/1999/xlink".to_string()
		} else {
			format!("urn:paperback:undeclared:{prefix}")
		};
		let _ = write!(out, " xmlns:{prefix}=\"{uri}\"");
	}
	out.push_str(&xml[insert_at..]);
	Some(out)
}

fn used_prefixes(xml: &str) -> Vec<&str> {
	let mut prefixes = Vec::new();
	let is_name_char = |c: char| c.is_alphanumeric() || matches!(c, '_' | '-' | '.');
	for (index, _) in xml.match_indices(':') {
		let before = &xml[..index];
		// Step over the whole character that ended the name: random bytes reach here, and one byte into a multi-byte character panics.
		let start = before.char_indices().rev().find(|(_, c)| !is_name_char(*c)).map_or(0, |(at, c)| at + c.len_utf8());
		let prefix = &before[start..];
		let opener = before[..start].chars().next_back();
		let follows_name_start = opener.is_some_and(|c| c == '<' || c.is_whitespace());
		let next_is_name = xml[index + 1..].chars().next().is_some_and(|c| c.is_alphabetic() || c == '_');
		if prefix.is_empty() || prefix == "xmlns" || !follows_name_start || !next_is_name {
			continue;
		}
		// Only inside a tag: a colon in the body of the book says nothing about namespaces.
		if xml[..index].rfind('<').is_none_or(|tag| xml[tag..index].contains('>')) {
			continue;
		}
		if !prefixes.contains(&prefix) {
			prefixes.push(prefix);
		}
	}
	prefixes
}

fn root_element_attribute_position(xml: &str) -> Option<usize> {
	let mut at = 0;
	loop {
		let open = xml[at..].find('<')? + at;
		let after = xml[open + 1..].chars().next()?;
		if after == '?' || after == '!' {
			at = xml[open..].find('>')? + open + 1;
			continue;
		}
		let mut quote = None;
		for (offset, ch) in xml[open..].char_indices() {
			match (quote, ch) {
				(None, c) if c == '"' || c == '\'' => quote = Some(ch),
				(Some(open_quote), ch) if ch == open_quote => quote = None,
				(None, '>') => {
					let end = open + offset;
					// In a self-closing tag the `/` belongs after what is being added.
					let insert = if xml[..end].ends_with('/') { end - 1 } else { end };
					return Some(insert);
				}
				_ => {}
			}
		}
		return None;
	}
}

/// Mends FictionBooks that other readers open but a strict XML parser refuses.
fn repair_fb2_xml(xml: &str) -> Option<String> {
	let namespaced = declare_missing_namespaces(xml);
	let entities = resolve_html_entities(namespaced.as_deref().unwrap_or(xml));
	entities.or(namespaced)
}

/// Any HTML entity XML doesn't define, such as `&nbsp;`, would otherwise lose the whole book, so unknown ones are dropped.
fn resolve_html_entities(xml: &str) -> Option<String> {
	/// Longest HTML entity name, `CounterClockwiseContourIntegral;`, plus room.
	const MAX_ENTITY_LEN: usize = 34;
	let mut out = String::with_capacity(xml.len());
	let mut rest = xml;
	let mut changed = false;
	while let Some(start) = rest.find('&') {
		out.push_str(&rest[..start]);
		let after = &rest[start + 1..];
		let name_end = after.char_indices().take(MAX_ENTITY_LEN).find(|(_, c)| *c == ';').map(|(at, _)| at);
		let Some(name_end) = name_end else {
			out.push('&');
			rest = after;
			continue;
		};
		let name = &after[..name_end];
		rest = &after[name_end + 1..];
		// The five XML declares, and the numeric references every parser resolves on its own.
		if matches!(name, "amp" | "lt" | "gt" | "quot" | "apos") || name.starts_with('#') {
			out.push('&');
			out.push_str(name);
			out.push(';');
			continue;
		}
		changed = true;
		if let Some((first, second)) = web_atoms::NAMED_ENTITIES.get(&format!("{name};")) {
			out.extend(char::from_u32(*first));
			// The table gives a second code point of zero for single-character entities.
			out.extend(char::from_u32(*second).filter(|c| *c != '\0'));
		}
	}
	if !changed {
		return None;
	}
	out.push_str(rest);
	Some(out)
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

	#[test]
	fn a_zipped_book_opens_through_the_zip_extension() {
		let mut zipped = std::io::Cursor::new(Vec::new());
		{
			let mut writer = zip::ZipWriter::new(&mut zipped);
			writer.start_file("book.fb2", zip::write::SimpleFileOptions::default()).unwrap();
			std::io::Write::write_all(&mut writer, fb2_document("", BODY).as_bytes()).unwrap();
			writer.finish().unwrap();
		}
		let dir = TempDir::new("fb2-zip");
		let path = dir.write_str("book.fb2.zip", zipped.into_inner());
		let doc = crate::parser::parse_document(&ParserContext::new(path)).expect("a zipped FB2 book opens");
		assert!(doc.buffer.content.contains("Opening line."), "{}", doc.buffer.content);
	}

	fn parse_ok(contents: &str) -> Document {
		parse_fb2(contents).expect("parse fb2 document")
	}

	/// Most FictionBooks are windows-1251, named in the XML declaration.
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

	/// Hand-written FictionBooks often use `l:href` without declaring the prefix.
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
		// The no-break space resolves and then collapses like every run of whitespace.
		assert!(doc.buffer.content.contains("Hello world—and…"), "got {:?}", doc.buffer.content);
	}

	/// FBReader's own help files use an undeclared `&FBReaderVersion;`.
	#[test]
	fn an_entity_nothing_knows_costs_only_itself() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>Unknown</book-title></title-info>",
			"<body><section><p>Version &FBReaderVersion; of it.</p></section></body>",
		));
		assert!(doc.buffer.content.contains("Version of it."), "got {:?}", doc.buffer.content);
	}

	/// This found a panic in the namespace repair, cutting a string one byte into a multi-byte character.
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

	#[test]
	fn drops_base64_binary_payloads() {
		let doc = parse_ok(&fb2_document(
			"<title-info><book-title>T</book-title></title-info>",
			r#"<body><section><p>Visible.</p></section></body><binary id="cover.jpg" content-type="image/jpeg">iVBORw0KGgoAAAANSUhEUg==</binary>"#,
		));
		assert!(doc.buffer.content.contains("Visible."));
		assert!(!doc.buffer.content.contains("iVBORw0KGgo"), "binary payload leaked: {:?}", doc.buffer.content);
	}

	/// Some writers append junk after the closing tag.
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

	fn headings(document: &Document) -> Vec<(i32, String)> {
		document
			.buffer
			.markers
			.iter()
			.filter(|m| crate::document::is_heading_marker(m.mtype))
			.map(|m| (crate::parser::util::toc::marker_type_to_heading_level(m.mtype), m.text.clone()))
			.collect()
	}

	/// FB2 marks a chapter with a section and its title, not h1 to h6, so without this a book has no headings or contents.
	#[test]
	fn section_titles_are_headings_by_depth_and_make_the_contents() {
		let body = r#"<body><title><p>The Book</p></title><section><title><p>Part One</p></title><section><title><p>Chapter 1</p><p>The Beginning</p></title><p>Text.</p></section></section></body><body name="notes"><title><p>Notes</p></title><section id="n1"><title><p>1</p></title><p>A note.</p></section></body>"#;
		let document = parse_ok(&fb2_document("", body));
		assert_eq!(
			headings(&document),
			[
				(1, "The Book".to_string()),
				(2, "Part One".to_string()),
				(3, "Chapter 1: The Beginning".to_string()),
				(1, "Notes".to_string())
			]
		);
		let top: Vec<&str> = document.toc_items.iter().map(|item| item.name.as_str()).collect();
		assert_eq!(top, ["The Book", "Notes"]);
		assert!(
			document.buffer.content.contains(
				"
1
A note."
			),
			"a note keeps its number as text: {:?}",
			document.buffer.content
		);
	}

	#[test]
	fn verse_keeps_its_lines_and_its_title_is_not_a_heading() {
		let body = r"<body><section><title><p>Chapter</p></title><poem><title><p>Ode</p></title><stanza><v>Line one</v><v>Line two</v></stanza><text-author>Someone</text-author></poem></section></body>";
		let document = parse_ok(&fb2_document("", body));
		assert!(
			document.buffer.content.contains(
				"Ode
Line one
Line two
Someone"
			),
			"{:?}",
			document.buffer.content
		);
		assert_eq!(headings(&document), [(2, "Chapter".to_string())]);
	}
}
