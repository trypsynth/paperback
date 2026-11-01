use std::collections::HashMap;

use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};

use crate::{
	document::{DocPos, TocItem},
	parser::util::{path::resolve_relative_path, xml::find_child_element},
	types::HeadingInfo,
};

/// Case-insensitive, since some producers write `dc:Title`.
pub(super) fn parse_daisy2_ncc_metadata(ncc_content: &str) -> (Option<String>, Option<String>) {
	let doc = scraper::Html::parse_document(ncc_content);
	let selector = scraper::Selector::parse("meta[name][content]").unwrap();
	let mut title = None;
	let mut author = None;
	for element in doc.select(&selector) {
		let Some(name) = element.value().attr("name") else { continue };
		let Some(content) = element.value().attr("content") else { continue };
		let content = content.trim();
		if content.is_empty() {
			continue;
		}
		if title.is_none() && name.eq_ignore_ascii_case("dc:title") {
			title = Some(content.to_string());
		} else if author.is_none() && name.eq_ignore_ascii_case("dc:creator") {
			author = Some(content.to_string());
		}
	}
	(title, author)
}

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

/// Each heading links into a SMIL file, so `smil_anchors` resolves it; `id_positions` is for a text-only book linking straight at content HTML. Unresolved headings are dropped rather than stranded at position 0.
pub(super) fn parse_daisy2_ncc_headings(
	ncc_content: &str,
	id_positions: &HashMap<String, DocPos>,
	smil_anchors: &HashMap<String, DocPos>,
) -> Vec<HeadingInfo> {
	let doc = scraper::Html::parse_document(ncc_content);
	let heading_selector = scraper::Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
	let link_selector = scraper::Selector::parse("a[href]").unwrap();
	let mut headings = Vec::new();
	for element in doc.select(&heading_selector) {
		let level = element.value().name()[1..].parse::<i32>().unwrap_or(1);
		let text = element.text().collect::<String>().trim().to_string();
		if text.is_empty() {
			continue;
		}
		let Some(href) = element.select(&link_selector).next().and_then(|a| a.value().attr("href")) else { continue };
		let fragment = href.split_once('#').map_or(href, |(_, frag)| frag);
		let Some(&offset) =
			smil_anchors.get(href).or_else(|| smil_anchors.get(fragment)).or_else(|| id_positions.get(fragment))
		else {
			continue;
		};
		headings.push(HeadingInfo { offset, level, text });
	}
	headings
}

/// In an audio DAISY 3 book a navPoint points into the SMIL, not the `DTBook`, so `id_positions` alone would strand every entry at the start; text-only books pass an empty `smil_anchors`.
/// `None` when nothing resolved, so the caller builds a TOC from the text rather than serve a uniformly wrong one.
pub(super) fn parse_daisy_ncx(
	ncx_content: &str,
	ncx_dir: &str,
	id_positions: &HashMap<String, DocPos>,
	smil_anchors: &HashMap<String, DocPos>,
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

/// Every labelled navPoint as (level, label, content src), in reading order.
pub(super) fn ncx_entries(ncx_content: &str) -> Vec<(i32, String, String)> {
	let Ok(ncx_doc) =
		XmlDocument::parse_with_options(ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return Vec::new();
	};
	ncx_doc
		.descendants()
		.filter(|n| n.is_element() && n.tag_name().name() == "navPoint")
		.filter_map(|nav| {
			let label = find_child_element(nav, "navLabel")
				.and_then(|label| find_child_element(label, "text"))
				.and_then(|text| text.text())?
				.trim();
			let src = find_child_element(nav, "content").and_then(|content| content.attribute("src"))?;
			let level = nav.ancestors().filter(|n| n.tag_name().name() == "navPoint").count();
			(!label.is_empty()).then(|| (i32::try_from(level).unwrap_or(6).min(6), label.to_string(), src.to_string()))
		})
		.collect()
}

/// Path-qualified first, since that can't collide across files, then bare.
fn resolve_navpoint_offset(
	content_src: &str,
	ncx_dir: &str,
	id_positions: &HashMap<String, DocPos>,
	smil_anchors: &HashMap<String, DocPos>,
) -> Option<DocPos> {
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
	id_positions: &HashMap<String, DocPos>,
	smil_anchors: &HashMap<String, DocPos>,
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
	let mut item = TocItem::new(label, target_id.to_string(), offset.unwrap_or(DocPos::ZERO));
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
