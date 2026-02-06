use std::{collections::HashMap, fs::File, io::BufReader};

use roxmltree::{Node, NodeType};
use zip::ZipArchive;

use crate::zip::read_zip_entry_by_name;

pub fn read_ooxml_relationships(archive: &mut ZipArchive<BufReader<File>>, rels_path: &str) -> HashMap<String, String> {
	let mut rels = HashMap::new();
	if let Ok(rels_content) = read_zip_entry_by_name(archive, rels_path) {
		if let Ok(rels_doc) = roxmltree::Document::parse(&rels_content) {
			for node in rels_doc.descendants() {
				if node.node_type() == NodeType::Element && node.tag_name().name() == "Relationship" {
					let id = node.attribute("Id").unwrap_or("").to_string();
					let target = node.attribute("Target").unwrap_or("").to_string();
					let rel_type = node.attribute("Type").unwrap_or("");
					if rel_type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
						&& !id.is_empty() && !target.is_empty()
					{
						rels.insert(id, target);
					}
				}
			}
		}
	}
	rels
}

pub fn collect_ooxml_run_text(run_element: Node) -> String {
	let mut text = String::new();
	for child in run_element.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		match child.tag_name().name() {
			"t" => {
				if let Some(t) = child.text() {
					text.push_str(t);
				}
			}
			"tab" => text.push('\t'),
			"br" => text.push('\n'),
			_ => {}
		}
	}
	text
}
