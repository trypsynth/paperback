//! Building an EPUB's table of contents from its EPUB3 nav document (preferred) or NCX
//! (fallback), and resolving each entry's href to an absolute offset in the converted text
//! via the spine section it falls in.

use std::{
	collections::HashMap,
	io::{Read, Seek},
	path::Path,
};

use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use super::{
	href::{resolve_href, split_href},
	spine::SectionMeta,
};
use crate::{
	document::TocItem,
	parser::util::xml::collect_element_text,
	util::{
		text::{collapse_whitespace, trim_string},
		zip::read_zip_entry_by_name,
	},
};

pub(super) fn build_epub_toc<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	nav_path: Option<&str>,
	ncx_path: Option<&str>,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Vec<TocItem> {
	let (items, source) = if let Some(nav_path) = nav_path {
		if let Some(items) = build_toc_from_nav_document(archive, nav_path, sections, id_positions) {
			(items, "nav")
		} else if let Some(items) = ncx_path.and_then(|p| build_toc_from_ncx(archive, p, sections, id_positions)) {
			(items, "ncx")
		} else {
			(Vec::new(), "none")
		}
	} else if let Some(ncx) = ncx_path {
		match build_toc_from_ncx(archive, ncx, sections, id_positions) {
			Some(items) => (items, "ncx"),
			None => (Vec::new(), "none"),
		}
	} else {
		(Vec::new(), "none")
	};
	tracing::debug!(source, items = items.len(), "epub toc built");
	items
}

fn build_toc_from_nav_document<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	nav_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let nav_content = read_zip_entry_by_name(archive, nav_path).ok()?;
	let nav_doc =
		XmlDocument::parse_with_options(&nav_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_node = nav_doc.descendants().find(|node| {
		if node.node_type() != NodeType::Element || node.tag_name().name() != "nav" {
			return false;
		}
		node.attributes().any(|attr| {
			let attr_name = attr.name();
			let matches_name = attr_name.eq_ignore_ascii_case("epub:type")
				|| attr_name.eq_ignore_ascii_case("type")
				|| attr_name.eq_ignore_ascii_case("role");
			matches_name
				&& attr
					.value()
					.split_ascii_whitespace()
					.any(|part| part.eq_ignore_ascii_case("toc") || part.eq_ignore_ascii_case("doc-toc"))
		})
	});
	let nav_node = nav_node?;
	let mut items = Vec::new();
	for child in nav_node.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		match child.tag_name().name() {
			"ol" | "ul" => items.extend(parse_nav_list(child, nav_path, sections, id_positions)),
			"li" => {
				if let Some(item) = parse_nav_item(child, nav_path, sections, id_positions) {
					items.push(item);
				}
			}
			_ => {}
		}
	}
	if items.is_empty() {
		items = parse_nav_list(nav_node, nav_path, sections, id_positions);
	}
	if items.is_empty() { None } else { Some(items) }
}

pub(super) fn parse_nav_list(
	list_node: Node<'_, '_>,
	current_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Vec<TocItem> {
	let mut items = Vec::new();
	for child in list_node.children() {
		if child.node_type() != NodeType::Element || child.tag_name().name() != "li" {
			continue;
		}
		if let Some(item) = parse_nav_item(child, current_path, sections, id_positions) {
			items.push(item);
		}
	}
	items
}

pub(super) fn parse_nav_item(
	item_node: Node<'_, '_>,
	current_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<TocItem> {
	let link_node = item_node
		.children()
		.find(|child| child.node_type() == NodeType::Element && child.tag_name().name() == "a")
		.or_else(|| {
			item_node.descendants().find(|desc| desc.node_type() == NodeType::Element && desc.tag_name().name() == "a")
		})?;
	let href = link_node.attribute("href").or_else(|| link_node.attribute(("http://www.w3.org/1999/xlink", "href")))?;
	let text = extract_link_text(link_node);
	if text.is_empty() {
		return None;
	}
	let reference = resolve_href(current_path, href);
	let offset = compute_nav_offset(&reference, sections, id_positions);
	let mut item = TocItem::new(text, reference, offset);
	for child in item_node.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		if child.tag_name().name() == "ol" || child.tag_name().name() == "ul" {
			item.children.extend(parse_nav_list(child, current_path, sections, id_positions));
		}
	}
	Some(item)
}

fn extract_link_text(link: Node<'_, '_>) -> String {
	let text = collect_element_text(link);
	trim_string(&collapse_whitespace(&text))
}

pub(super) fn compute_nav_offset(
	reference: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> usize {
	let (path_part, fragment) = split_href(reference);
	if let Some(section) = sections.iter().find(|section| section.path == path_part) {
		if let Some(frag) = fragment.as_deref()
			&& let Some(offset) = id_positions.get(&format!("{path_part}#{frag}")).or_else(|| id_positions.get(frag))
			&& *offset >= section.start
			&& *offset < section.end
		{
			return *offset;
		}
		return section.start;
	}
	if let Some(frag) = fragment
		&& let Some(offset) = id_positions.get(&frag)
	{
		return *offset;
	}
	// Fallback: match by file name if full path didn't resolve.
	if let Some(name) = Path::new(&path_part).file_name().and_then(|n| n.to_str())
		&& let Some(section) = sections.iter().find(|section| {
			Path::new(&section.path)
				.file_name()
				.and_then(|n| n.to_str())
				.is_some_and(|base| base.eq_ignore_ascii_case(name))
		}) {
		return section.start;
	}
	0
}

fn build_toc_from_ncx<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	ncx_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let ncx_content = read_zip_entry_by_name(archive, ncx_path).ok()?;
	let ncx_doc =
		XmlDocument::parse_with_options(&ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_map =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navMap")?;
	let mut items = Vec::new();
	for navpoint in nav_map.children() {
		if navpoint.node_type() == NodeType::Element
			&& navpoint.tag_name().name() == "navPoint"
			&& let Some(item) = convert_navpoint(navpoint, sections, id_positions, ncx_path)
		{
			items.push(item);
		}
	}
	if items.is_empty() { None } else { Some(items) }
}

pub(super) fn convert_navpoint(
	nav: Node,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
	base_path: &str,
) -> Option<TocItem> {
	let label = nav
		.children()
		.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navLabel")
		.and_then(|label| {
			label
				.children()
				.find(|t| t.node_type() == NodeType::Element && t.tag_name().name() == "text")
				.and_then(|t| t.text())
		})
		.unwrap_or("")
		.to_string();
	let content_src = nav
		.children()
		.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "content")
		.and_then(|c| c.attribute("src"))?;
	if label.trim().is_empty() {
		return None;
	}
	let reference = resolve_href(base_path, content_src);
	let offset = compute_nav_offset(&reference, sections, id_positions);
	let mut item = TocItem::new(label, reference, offset);
	for child in nav.children() {
		if child.node_type() == NodeType::Element
			&& child.tag_name().name() == "navPoint"
			&& let Some(child_item) = convert_navpoint(child, sections, id_positions, base_path)
		{
			item.children.push(child_item);
		}
	}
	Some(item)
}

#[cfg(test)]
mod tests;
