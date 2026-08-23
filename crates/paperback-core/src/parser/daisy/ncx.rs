use std::collections::HashMap;

use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};

use crate::{document::TocItem, parser::util::path::resolve_relative_path};

pub(super) fn extract_daisy2_links(ncc_content: &str) -> Vec<String> {
	let mut links = Vec::new();
	let scraper = scraper::Html::parse_document(ncc_content);
	let selector = scraper::Selector::parse("a[href]").unwrap();
	for element in scraper.select(&selector) {
		if let Some(href) = element.value().attr("href") {
			let file_path = href.split('#').next().unwrap_or(href);
			if !file_path.is_empty() && !links.contains(&file_path.to_string()) {
				links.push(file_path.to_string());
			}
		}
	}
	links
}

/// Parses an NCX `navMap` into TOC items. `smil_anchors` maps ids *within the SMIL files* to
/// text positions: in an audio DAISY 3 book a navPoint's `content/@src` points into the SMIL,
/// not into the `DTBook`, so resolving it against `id_positions` alone would silently strand
/// every entry at the start of the book. Text-only books, whose navPoints do point at
/// `DTBook` ids, pass an empty map. Returns `None` when nothing resolved at all, letting the
/// caller fall back to a TOC built from the text itself rather than serve a uniformly wrong one.
pub(super) fn parse_daisy_ncx(
	ncx_content: &str,
	ncx_dir: &str,
	id_positions: &HashMap<String, usize>,
	smil_anchors: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let ncx_doc =
		XmlDocument::parse_with_options(ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_map =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navMap")?;
	let mut items = Vec::new();
	let mut resolved_any = false;
	for navpoint in nav_map.children() {
		if navpoint.node_type() == NodeType::Element
			&& navpoint.tag_name().name() == "navPoint"
			&& let Some(item) = convert_daisy_navpoint(navpoint, ncx_dir, id_positions, smil_anchors, &mut resolved_any)
		{
			items.push(item);
		}
	}
	if items.is_empty() || !resolved_any { None } else { Some(items) }
}

/// The text position a navPoint's `content/@src` names, looked up path-qualified first (which
/// can't collide across files) and then bare. `resolved_any` records whether any navPoint in
/// the whole map found a real target.
fn resolve_navpoint_offset(
	content_src: &str,
	ncx_dir: &str,
	id_positions: &HashMap<String, usize>,
	smil_anchors: &HashMap<String, usize>,
) -> Option<usize> {
	let (file_part, fragment) = content_src.split_once('#')?;
	let qualified =
		(!file_part.is_empty()).then(|| format!("{}#{fragment}", resolve_relative_path(ncx_dir, file_part)));
	qualified
		.as_ref()
		.and_then(|key| smil_anchors.get(key).or_else(|| id_positions.get(key)))
		.or_else(|| smil_anchors.get(fragment))
		.or_else(|| id_positions.get(fragment))
		.copied()
}

fn convert_daisy_navpoint(
	nav: Node,
	ncx_dir: &str,
	id_positions: &HashMap<String, usize>,
	smil_anchors: &HashMap<String, usize>,
	resolved_any: &mut bool,
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
	let target_id =
		content_src.find('#').map_or_else(|| nav.attribute("id").unwrap_or(content_src), |idx| &content_src[idx + 1..]);
	let offset = resolve_navpoint_offset(content_src, ncx_dir, id_positions, smil_anchors)
		.or_else(|| id_positions.get(target_id).copied())
		.or_else(|| nav.attribute("id").and_then(|id| id_positions.get(id)).copied());
	*resolved_any |= offset.is_some();
	let mut item = TocItem::new(label, target_id.to_string(), offset.unwrap_or(0));
	for child in nav.children() {
		if child.node_type() == NodeType::Element
			&& child.tag_name().name() == "navPoint"
			&& let Some(child_item) = convert_daisy_navpoint(child, ncx_dir, id_positions, smil_anchors, resolved_any)
		{
			item.children.push(child_item);
		}
	}
	Some(item)
}
