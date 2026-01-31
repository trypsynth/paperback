use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use roxmltree::{Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{DocumentBuffer, MarkerType, TocItem},
	html_to_text::HeadingInfo,
	utils::zip::read_zip_entry_by_name,
};

fn children_at_mut<'a>(toc: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = toc;
	for &idx in path {
		current = &mut current[idx].children;
	}
	current
}

pub fn build_toc_from_buffer(buffer: &DocumentBuffer) -> Vec<TocItem> {
	let headings: Vec<HeadingInfo> = buffer
		.markers
		.iter()
		.filter_map(|marker| {
			let level = marker_type_to_heading_level(marker.mtype);
			if level > 0 {
				Some(HeadingInfo { offset: marker.position, level, text: marker.text.clone() })
			} else {
				None
			}
		})
		.collect();
	build_toc_from_headings(&headings)
}

pub fn build_toc_from_headings(headings: &[HeadingInfo]) -> Vec<TocItem> {
	if headings.is_empty() {
		return Vec::new();
	}
	let mut toc = Vec::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut levels: Vec<i32> = Vec::new();
	for heading in headings {
		if heading.level <= 0 {
			continue;
		}
		while let Some(&last_level) = levels.last() {
			if last_level < heading.level {
				break;
			}
			stack.pop();
			levels.pop();
		}
		let item = TocItem::new(heading.text.clone(), String::new(), heading.offset);
		let siblings = children_at_mut(&mut toc, &stack);
		siblings.push(item);
		stack.push(siblings.len() - 1);
		levels.push(heading.level);
	}
	toc
}

pub const fn heading_level_to_marker_type(level: i32) -> MarkerType {
	match level {
		1 => MarkerType::Heading1,
		2 => MarkerType::Heading2,
		3 => MarkerType::Heading3,
		4 => MarkerType::Heading4,
		5 => MarkerType::Heading5,
		_ => MarkerType::Heading6,
	}
}

pub const fn marker_type_to_heading_level(marker_type: MarkerType) -> i32 {
	match marker_type {
		MarkerType::Heading1 => 1,
		MarkerType::Heading2 => 2,
		MarkerType::Heading3 => 3,
		MarkerType::Heading4 => 4,
		MarkerType::Heading5 => 5,
		MarkerType::Heading6 => 6,
		_ => 0,
	}
}

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

pub fn extract_title_from_path(path: &str) -> String {
	Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn extracts_title_from_path() {
		assert_eq!(extract_title_from_path("foo.txt"), "foo");
		assert_eq!(extract_title_from_path("/home/quin/books/worm.epub"), "worm");
		assert_eq!(extract_title_from_path("C:\\Users\\Quin\\Desktop\\file.log"), "file");
		assert_eq!(extract_title_from_path("/path/with/trailing/slash/"), "Untitled");
		assert_eq!(extract_title_from_path(""), "Untitled");
	}
}
