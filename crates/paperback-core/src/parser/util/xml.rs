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
	let Some(encoding) = declared_encoding(&bytes) else {
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

/// How far into a file to look for the XML declaration. It has to be the first thing in the file,
/// so anything past the first line is not one.
const DECLARATION_SCAN_BYTES: usize = 200;

/// The encoding an XML declaration names, if it names one this build knows.
fn declared_encoding(bytes: &[u8]) -> Option<&'static encoding_rs::Encoding> {
	let head = &bytes[..bytes.len().min(DECLARATION_SCAN_BYTES)];
	let head = String::from_utf8_lossy(head);
	let declaration = head.split_once("?>")?.0;
	let after_key = declaration.split_once("encoding")?.1;
	let quoted = after_key.trim_start().strip_prefix('=')?.trim_start();
	let quote = quoted.chars().next().filter(|c| *c == '"' || *c == '\'')?;
	let label = quoted[1..].split(quote).next()?;
	encoding_rs::Encoding::for_label(label.as_bytes())
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
}
