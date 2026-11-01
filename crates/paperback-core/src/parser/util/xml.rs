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

/// Most FictionBooks are windows-1251, so the declared encoding is honoured rather than assuming UTF-8.
pub fn read_xml_to_string(path: &str) -> std::io::Result<String> {
	Ok(decode_xml(&std::fs::read(path)?))
}

#[must_use]
pub fn decode_xml(bytes: &[u8]) -> String {
	let Some(encoding) = crate::util::encoding::xml_declared_encoding(bytes) else {
		// A windows-1251 FictionBook saved without a declaration is still windows-1251, so it is sniffed like plain text.
		return crate::util::encoding::convert_to_utf8(bytes);
	};
	// `decode` follows a byte order mark over the declaration, as the XML specification asks.
	let (text, _, _) = encoding.decode(bytes);
	replace_encoding_declaration(&text)
}

/// roxmltree is handed UTF-8 whatever the file held, but refuses a document whose declaration claims otherwise.
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
	/// Most FictionBooks are windows-1251, which `read_to_string` refuses.
	#[test]
	fn decodes_the_encoding_the_declaration_names() {
		let dir = TempDir::new("xml-encoding");
		let mut bytes = br#"<?xml version="1.0" encoding="windows-1251"?><p>"#.to_vec();
		bytes.extend_from_slice(&[0xC0, 0xED, 0xED, 0xE0]); // "Анна"
		bytes.extend_from_slice(b"</p>");
		let path = dir.write_str("cp1251.xml", bytes);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(text.contains("Анна"), "got {text:?}");
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

	#[test]
	fn an_unknown_encoding_falls_back_to_utf8() {
		let dir = TempDir::new("xml-unknown");
		let path = dir.write_str("odd.xml", r#"<?xml version="1.0" encoding="x-madeup"?><p>plain</p>"#);
		let text = read_xml_to_string(&path).expect("read the file");
		assert!(text.contains("<p>plain</p>"), "got {text:?}");
	}
}
