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
