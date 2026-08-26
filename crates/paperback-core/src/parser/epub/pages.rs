//! Building an EPUB's page list (used for "go to page") from its EPUB3 nav document's
//! `page-list` nav (preferred) or NCX `pageList` (fallback). Mirrors [`super::toc`]'s
//! nav/NCX handling, reusing its item-parsing helpers since both walk the same node shapes.

use std::{
	collections::HashMap,
	io::{Read, Seek},
};

use roxmltree::{Document as XmlDocument, NodeType, ParsingOptions};
use zip::ZipArchive;

use super::{
	spine::SectionMeta,
	toc::{convert_navpoint, parse_nav_item, parse_nav_list},
};
use crate::{document::TocItem, util::zip::read_zip_entry_by_name};

pub(super) fn build_epub_pages<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	nav_path: Option<&str>,
	ncx_path: Option<&str>,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Vec<TocItem> {
	let (items, source) = if let Some(nav_path) = nav_path {
		if let Some(items) = build_pages_from_nav_document(archive, nav_path, sections, id_positions) {
			(items, "nav")
		} else if let Some(items) = ncx_path.and_then(|p| build_pages_from_ncx(archive, p, sections, id_positions)) {
			(items, "ncx")
		} else {
			(Vec::new(), "none")
		}
	} else if let Some(ncx) = ncx_path {
		match build_pages_from_ncx(archive, ncx, sections, id_positions) {
			Some(items) => (items, "ncx"),
			None => (Vec::new(), "none"),
		}
	} else {
		(Vec::new(), "none")
	};
	tracing::debug!(source, items = items.len(), "epub page list built");
	items
}

fn build_pages_from_nav_document<R: Read + Seek>(
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
					.any(|part| part.eq_ignore_ascii_case("page-list") || part.eq_ignore_ascii_case("doc-pagelist"))
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

fn build_pages_from_ncx<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	ncx_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let ncx_content = read_zip_entry_by_name(archive, ncx_path).ok()?;
	let ncx_doc =
		XmlDocument::parse_with_options(&ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let page_list =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "pageList")?;
	let mut items = Vec::new();
	for page_target in page_list.children() {
		if page_target.node_type() == NodeType::Element
			&& page_target.tag_name().name() == "pageTarget"
			&& let Some(item) = convert_navpoint(page_target, sections, id_positions, ncx_path)
		{
			items.push(item);
		}
	}
	if items.is_empty() { None } else { Some(items) }
}
