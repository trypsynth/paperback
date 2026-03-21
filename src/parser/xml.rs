use roxmltree::{Node, NodeType};

pub fn collect_element_text(node: Node) -> String {
	let mut text = String::new();
	collect_text_recursive(node, &mut text);
	text.trim().to_string()
}

fn collect_text_recursive(node: Node, text: &mut String) {
	if node.node_type() == NodeType::Text {
		if let Some(t) = node.text() {
			text.push_str(t);
		}
	}
	for child in node.children() {
		collect_text_recursive(child, text);
	}
}

pub fn collect_text_from_tagged_elements(node: Node, tag_name: &str) -> String {
	let mut text = String::new();
	collect_tagged_text_recursive(node, tag_name, &mut text);
	text
}

fn collect_tagged_text_recursive(node: Node, tag_name: &str, text: &mut String) {
	if node.node_type() == NodeType::Element && node.tag_name().name() == tag_name {
		if let Some(t) = node.text() {
			text.push_str(t);
		}
	}
	for child in node.children() {
		collect_tagged_text_recursive(child, tag_name, text);
	}
}

pub fn find_child_element<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
	node.children().find(|child| child.node_type() == NodeType::Element && child.tag_name().name() == name)
}

#[cfg(test)]
mod tests {
	use roxmltree::Document;

	use super::*;

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
}
