use roxmltree::{Node, NodeType};

#[must_use]
pub fn collect_element_text(node: Node) -> String {
	let mut text = String::new();
	collect_text_recursive(node, &mut text);
	text.trim().to_string()
}

fn collect_text_recursive(node: Node, text: &mut String) {
	if node.node_type() == NodeType::Text
		&& let Some(t) = node.text()
	{
		text.push_str(t);
	}
	for child in node.children() {
		collect_text_recursive(child, text);
	}
}

#[must_use]
pub fn collect_text_from_tagged_elements(node: Node, tag_name: &str) -> String {
	let mut text = String::new();
	collect_tagged_text_recursive(node, tag_name, &mut text);
	text
}

fn collect_tagged_text_recursive(node: Node, tag_name: &str, text: &mut String) {
	if node.node_type() == NodeType::Element
		&& node.tag_name().name() == tag_name
		&& let Some(t) = node.text()
	{
		text.push_str(t);
	}
	for child in node.children() {
		collect_tagged_text_recursive(child, tag_name, text);
	}
}

#[must_use]
pub fn find_child_element<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
	node.children().find(|child| child.node_type() == NodeType::Element && child.tag_name().name() == name)
}

/// Reads an XML file as text, decoded with the encoding the file's own declaration names.
///
/// [`std::fs::read_to_string`] demands UTF-8, and a FictionBook usually is not. The format came
/// out of the Russian ebook world and most of its books are written in windows-1251, named in the
/// `<?xml version="1.0" encoding="windows-1251"?>` at the top. Reading one of those as UTF-8 fails
/// outright, so the whole book is lost over a header Paperback could simply have read.
///
/// A file that names an encoding this does not know is read as UTF-8, and one that names none at
/// all is guessed at the way a plain text file is. A byte order mark wins over the declaration,
/// which is what the XML specification asks for.
pub fn read_xml_to_string(path: &str) -> std::io::Result<String> {
	let bytes = std::fs::read(path)?;
	let Some(encoding) = crate::util::encoding::xml_declared_encoding(&bytes) else {
		// A file that declares nothing is guessed at the way a plain text file is, byte order
		// marks and all, rather than simply assumed to be UTF-8: a FictionBook written in
		// windows-1251 and saved without a declaration is still a windows-1251 book.
		return Ok(crate::util::encoding::convert_to_utf8(&bytes));
	};
	// `decode` sniffs for a byte order mark first and follows it where there is one, so a UTF-16
	// file is read as UTF-16 whatever its declaration says.
	let (text, _, _) = encoding.decode(&bytes);
	Ok(replace_encoding_declaration(&text))
}

/// Rewrites the decoded text's own encoding declaration to UTF-8, which is what it now is.
///
/// roxmltree reads `&str` and so is handed UTF-8 whatever the file held, but it reads the
/// declaration too and refuses a document that claims to be anything else.
fn replace_encoding_declaration(text: &str) -> String {
	let Some((declaration, rest)) = text.split_once("?>") else { return text.to_string() };
	if !declaration.starts_with("<?xml") || !declaration.contains("encoding") {
		return text.to_string();
	}
	let mut out = String::with_capacity(text.len());
	let mut remaining = declaration;
	while let Some((before, after)) = remaining.split_once("encoding") {
		out.push_str(before);
		out.push_str("encoding");
		let trimmed = after.trim_start();
		let Some(value) = trimmed.strip_prefix('=') else {
			remaining = after;
			continue;
		};
		let value = value.trim_start();
		let Some(quote) = value.chars().next().filter(|c| *c == '"' || *c == '\'') else {
			remaining = after;
			continue;
		};
		let Some((_, tail)) = value[1..].split_once(quote) else {
			remaining = after;
			continue;
		};
		out.push_str("=\"UTF-8\"");
		remaining = tail;
	}
	out.push_str(remaining);
	out.push_str("?>");
	out.push_str(rest);
	out
}

/// Writes `node` and everything under it back out as XML, leaving out every element `skip` accepts
/// along with its contents. Elements are written under their local names, without prefixes.
pub fn serialize_xml(node: Node, output: &mut String, skip: &dyn Fn(Node) -> bool) {
	match node.node_type() {
		NodeType::Root => {
			for child in node.children() {
				serialize_xml(child, output, skip);
			}
		}
		NodeType::Element => {
			if skip(node) {
				return;
			}
			let tag_name = node.tag_name().name();
			output.push('<');
			output.push_str(tag_name);
			for attr in node.attributes() {
				output.push(' ');
				output.push_str(attr.name());
				output.push_str("=\"");
				output.push_str(&escape_xml(attr.value()));
				output.push('"');
			}
			if node.children().count() == 0 && is_html_void_element(tag_name) {
				output.push_str("/>");
			} else {
				output.push('>');
				for child in node.children() {
					serialize_xml(child, output, skip);
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

fn is_html_void_element(name: &str) -> bool {
	matches!(
		name,
		"area"
			| "base"
			| "br"
			| "col"
			| "embed"
			| "hr"
			| "img"
			| "input"
			| "link"
			| "meta"
			| "source"
			| "track"
			| "wbr"
	)
}

#[must_use]
pub fn escape_xml(s: &str) -> String {
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

/// Makes a document that is not quite valid XML readable, or returns `None` when there was
/// nothing to mend.
///
/// Declares the namespace prefixes the document uses and never declared, then replaces the HTML
/// character entities XML does not know. A prefix listed in `known_prefixes` is declared with the
/// URI listed beside it.
#[must_use]
pub fn repair_xml(xml: &str, known_prefixes: &[(&str, &str)]) -> Option<String> {
	let namespaced = declare_missing_namespaces(xml, known_prefixes);
	let entities = resolve_html_entities(namespaced.as_deref().unwrap_or(xml));
	entities.or(namespaced)
}

/// Declares the namespace prefixes a document uses and never declared, so a strict XML parser will
/// read it.
///
/// A FictionBook written by hand or by a careless converter often opens with a bare
/// `<FictionBook>` and then writes a footnote as `<a l:href="#n1">`, with no `xmlns:l` anywhere.
/// That is not valid XML and roxmltree refuses the whole file, though every FictionBook reader
/// takes it: Bulgakov's "The White Guard" in FBReader's own test corpus is written exactly that
/// way. Returns `None` when every prefix the document uses is already declared, which is when there
/// is nothing to gain by parsing it again.
fn declare_missing_namespaces(xml: &str, known_prefixes: &[(&str, &str)]) -> Option<String> {
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
		// A prefix missing from `known_prefixes` gets a URI of its own.
		let uri = known_prefixes
			.iter()
			.find(|(known, _)| known == prefix)
			.map_or_else(|| format!("urn:paperback:undeclared:{prefix}"), |(_, uri)| (*uri).to_string());
		out.push_str(&format!(" xmlns:{prefix}=\"{uri}\""));
	}
	out.push_str(&xml[insert_at..]);
	Some(out)
}

/// The prefixes a document uses on an element or an attribute name.
fn used_prefixes(xml: &str) -> Vec<&str> {
	let mut prefixes = Vec::new();
	let is_name_char = |c: char| c.is_alphanumeric() || matches!(c, '_' | '-' | '.');
	for (index, _) in xml.match_indices(':') {
		let before = &xml[..index];
		// Stepping over the character that ended the name, not over one byte of it: a file of
		// random bytes reaches here, and one byte into a multi-byte character is not a place a
		// string can be cut.
		let start = before.char_indices().rev().find(|(_, c)| !is_name_char(*c)).map_or(0, |(at, c)| at + c.len_utf8());
		let prefix = &before[start..];
		// A prefix is a name, it follows either the `<` of a tag or the whitespace before an
		// attribute, and what comes after the colon is a name too.
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

/// Where an attribute can be added to the root element's start tag, which is just before the `>`
/// that closes it.
fn root_element_attribute_position(xml: &str) -> Option<usize> {
	let mut at = 0;
	loop {
		let open = xml[at..].find('<')? + at;
		let after = xml[open + 1..].chars().next()?;
		if after == '?' || after == '!' {
			// A declaration, a comment or a doctype, none of which is the root element.
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
					// A tag that closes itself has nothing under it, so `/` belongs after what is
					// being added rather than before it.
					let insert = if xml[..end].ends_with('/') { end - 1 } else { end };
					return Some(insert);
				}
				_ => {}
			}
		}
		return None;
	}
}

/// Replaces the HTML character entities an XML parser does not know with the characters they
/// stand for.
///
/// XML declares five entities and no more, so `&nbsp;` or `&mdash;` in a FictionBook is an error
/// that costs the whole book. They are common: a converter that has just read HTML writes what it
/// read. An entity that is not in the HTML set either is dropped, because a missing character is
/// a smaller loss than a missing book.
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
			// The table gives a second code point of zero for the entities that stand for one
			// character, which is most of them.
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
	use roxmltree::Document;

	use super::*;
	use crate::util::test_support::TempDir;

	#[test]
	fn collect_element_text_trims_and_collects_nested_text() {
		let xml = "<root>  hello <b>world</b> ! </root>";
		let doc = Document::parse(xml).unwrap();
		let text = collect_element_text(doc.root_element());
		assert_eq!(text, "hello world !");
	}

	#[test]
	fn collect_element_text_ignores_non_text_nodes() {
		let xml = "<root><!-- comment --><a>one</a><b>two</b></root>";
		let doc = Document::parse(xml).unwrap();
		let text = collect_element_text(doc.root_element());
		assert_eq!(text, "onetwo");
	}

	#[test]
	fn collect_text_from_tagged_elements_collects_matching_nodes_only() {
		let xml = "<root><p>one</p><q>two</q><p>three</p></root>";
		let doc = Document::parse(xml).unwrap();
		let text = collect_text_from_tagged_elements(doc.root_element(), "p");
		assert_eq!(text, "onethree");
	}

	#[test]
	fn collect_text_from_tagged_elements_returns_empty_for_missing_tag() {
		let xml = "<root><a>one</a></root>";
		let doc = Document::parse(xml).unwrap();
		let text = collect_text_from_tagged_elements(doc.root_element(), "p");
		assert_eq!(text, "");
	}

	#[test]
	fn find_child_element_returns_first_direct_match() {
		let xml = "<root><x>1</x><target>hit</target><target>miss</target></root>";
		let doc = Document::parse(xml).unwrap();
		let found = find_child_element(doc.root_element(), "target").unwrap();
		assert_eq!(found.text(), Some("hit"));
	}

	#[test]
	fn find_child_element_does_not_match_grandchildren() {
		let xml = "<root><wrapper><target>nested</target></wrapper></root>";
		let doc = Document::parse(xml).unwrap();
		assert!(find_child_element(doc.root_element(), "target").is_none());
	}
	/// The windows-1251 an ordinary FictionBook is written in, which `read_to_string` refuses.
	#[test]
	fn decodes_the_encoding_the_declaration_names() {
		let dir = TempDir::new("xml-encoding");
		let mut bytes = br#"<?xml version="1.0" encoding="windows-1251"?><p>"#.to_vec();
		bytes.extend_from_slice(&[0xC0, 0xED, 0xED, 0xE0]); // "Анна"
		bytes.extend_from_slice(b"</p>");
		let path = dir.write_str("cp1251.xml", bytes);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(text.contains("Анна"), "got {text:?}");
		// roxmltree is handed a `&str` and refuses a document that claims to be anything but the
		// UTF-8 it now is.
		assert!(text.contains(r#"encoding="UTF-8""#), "got {text:?}");
	}

	/// A byte order mark outranks the declaration, which is what the XML specification asks for.
	#[test]
	fn a_byte_order_mark_wins_over_the_declaration() {
		let dir = TempDir::new("xml-bom");
		let mut bytes = vec![0xFF, 0xFE]; // UTF-16 little endian
		for unit in r#"<?xml version="1.0" encoding="windows-1251"?><p>hi</p>"#.encode_utf16() {
			bytes.extend_from_slice(&unit.to_le_bytes());
		}
		let path = dir.write_str("utf16.xml", bytes);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(text.contains("<p>hi</p>"), "got {text:?}");
	}

	/// A file that declares nothing at all is guessed at rather than assumed to be UTF-8.
	#[test]
	fn a_file_with_no_declaration_is_detected() {
		let dir = TempDir::new("xml-nodecl");
		let mut bytes = b"<FictionBook><p>".to_vec();
		bytes.extend_from_slice(&[0xC0, 0xED, 0xED, 0xE0]); // "Анна" in windows-1251
		bytes.extend_from_slice(b"</p></FictionBook>");
		let path = dir.write_str("nodecl.fb2", bytes);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(!text.contains('\u{fffd}'), "no replacement characters: {text:?}");
	}

	/// A declaration naming an encoding this build has never heard of is read as UTF-8 rather than
	/// refused.
	#[test]
	fn an_unknown_encoding_falls_back_to_utf8() {
		let dir = TempDir::new("xml-unknown");
		let path = dir.write_str("odd.xml", r#"<?xml version="1.0" encoding="x-madeup"?><p>plain</p>"#);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(text.contains("<p>plain</p>"), "got {text:?}");
	}

	const CONTENT_NS: &str = "http://purl.org/rss/1.0/modules/content/";

	fn namespace_of(xml: &str, local_name: &str) -> Option<String> {
		let doc = Document::parse(xml).expect("the repaired document parses");
		doc.descendants()
			.find(|node| node.is_element() && node.tag_name().name() == local_name)
			.and_then(|node| node.tag_name().namespace().map(str::to_string))
	}

	#[test]
	fn repair_declares_a_known_prefix_with_its_real_namespace() {
		let xml = "<rss><item><content:encoded>x</content:encoded></item></rss>";
		let repaired = repair_xml(xml, &[("content", CONTENT_NS)]).expect("a prefix was missing");
		assert_eq!(namespace_of(&repaired, "encoded").as_deref(), Some(CONTENT_NS));
	}

	#[test]
	fn repair_declares_an_unknown_prefix_with_a_placeholder_namespace() {
		let xml = "<rss><item><foo:bar>x</foo:bar></item></rss>";
		let repaired = repair_xml(xml, &[("content", CONTENT_NS)]).expect("a prefix was missing");
		assert_eq!(namespace_of(&repaired, "bar").as_deref(), Some("urn:paperback:undeclared:foo"));
	}

	#[test]
	fn repair_replaces_html_entities() {
		let repaired = repair_xml("<p>a&nbsp;b&mdash;c</p>", &[]).expect("entities were replaced");
		let doc = Document::parse(&repaired).expect("the repaired document parses");
		assert_eq!(doc.root_element().text(), Some("a\u{a0}b\u{2014}c"));
	}

	#[test]
	fn repair_leaves_a_well_formed_document_alone() {
		assert_eq!(repair_xml(r#"<p xmlns:c="urn:c"><c:x>a &amp; b</c:x></p>"#, &[]), None);
	}

	#[test]
	fn serialize_skips_what_the_predicate_names_and_escapes_the_rest() {
		let doc = Document::parse(r#"<a><b x="1 &quot; 2">3 &lt; 4</b><binary>AAAA</binary><c/></a>"#).unwrap();
		let mut out = String::new();
		serialize_xml(doc.root(), &mut out, &|node| node.tag_name().name() == "binary");
		assert_eq!(out, r#"<a><b x="1 &quot; 2">3 &lt; 4</b><c></c></a>"#);
	}

	#[test]
	fn serialize_closes_an_empty_element_but_self_closes_an_html_void_one() {
		let doc = Document::parse(r#"<p><a id="fn1"/>text<br/><img src="x.png"/><iframe/></p>"#).unwrap();
		let mut out = String::new();
		serialize_xml(doc.root(), &mut out, &|_| false);
		assert_eq!(out, r#"<p><a id="fn1"></a>text<br/><img src="x.png"/><iframe></iframe></p>"#);
	}
}
