use std::path::Path;

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, NodeType, ParsingOptions};

use crate::parser::util::path::resolve_relative_path;

pub(super) struct ManifestItem {
	pub(super) href: String,
	pub(super) media_type: String,
}

/// A parsed OPF package: manifest items and spine in document order, plus the metadata the
/// parser cares about. Hrefs are resolved against the archive/directory root at parse time.
pub(super) struct OpfPackage {
	pub(super) items: Vec<(String, ManifestItem)>,
	pub(super) spine: Vec<String>,
	pub(super) title: Option<String>,
	pub(super) author: Option<String>,
}

impl OpfPackage {
	pub(super) fn item(&self, id: &str) -> Option<&ManifestItem> {
		self.items.iter().find(|(item_id, _)| item_id == id).map(|(_, item)| item)
	}
}

pub(super) fn parse_opf_package(opf_content: &str, opf_dir: &str) -> Result<OpfPackage> {
	let doc =
		XmlDocument::parse_with_options(opf_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.context("Failed to parse OPF XML")?;
	let mut items = Vec::new();
	let mut spine = Vec::new();
	let mut title = None;
	let mut author = None;
	if let Some(package) =
		doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "package")
	{
		for child in package.children() {
			if !child.is_element() {
				continue;
			}
			match child.tag_name().name() {
				"metadata" => {
					for meta_child in child.children() {
						if meta_child.is_element() {
							let name = meta_child.tag_name().name();
							if name == "Title" || name == "title" {
								title = meta_child.text().map(|s| s.trim().to_string());
							} else if name == "Creator" || name == "creator" {
								author = meta_child.text().map(|s| s.trim().to_string());
							}
						}
					}
					for meta_child in child.descendants() {
						if meta_child.is_element() {
							let name = meta_child.tag_name().name();
							if name == "Title" || name == "title" {
								if title.is_none() {
									title = meta_child.text().map(|s| s.trim().to_string());
								}
							} else if (name == "Creator" || name == "creator") && author.is_none() {
								author = meta_child.text().map(|s| s.trim().to_string());
							}
						}
					}
				}
				"manifest" => {
					for item in child.children().filter(|n| n.is_element() && n.tag_name().name() == "item") {
						let Some(href) = item.attribute("href") else { continue };
						let id = item.attribute("id").unwrap_or("").to_string();
						let media_type = item.attribute("media-type").unwrap_or("").to_string();
						items.push((id, ManifestItem { href: resolve_relative_path(opf_dir, href), media_type }));
					}
				}
				"spine" => {
					for itemref in child.children().filter(|n| n.is_element() && n.tag_name().name() == "itemref") {
						if let Some(idref) = itemref.attribute("idref") {
							spine.push(idref.to_string());
						}
					}
				}
				_ => {}
			}
		}
	}
	Ok(OpfPackage { items, spine, title, author })
}

pub(super) fn is_dtbook_item(item: &ManifestItem) -> bool {
	item.media_type == "application/x-dtbook+xml"
}

/// Whether `item` is plausibly DTBook content convertible directly, without a SMIL layer:
/// the manifest's declared type, or the same untyped-`.xml` fallback `find_single_dtbook_href`
/// uses for a book whose OPF mislabels its DTBook items as generic `text/xml`. Unlike
/// `is_dtbook_item`, this doesn't rank one match over the other, since the spine walk in
/// `build_daisy_document` decides per item rather than picking a single best match package-wide.
pub(super) fn is_dtbook_like_item(item: &ManifestItem) -> bool {
	is_dtbook_item(item)
		|| (item.media_type == "text/xml"
			&& Path::new(&item.href).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("xml")))
}

/// Finds the single `DTBook` XML file for a legacy single-file DAISY book: the manifest's
/// declared `DTBook` item, or failing that, the first plain `.xml` item.
pub(super) fn find_single_dtbook_href(package: &OpfPackage) -> Option<String> {
	let mut fallback = None;
	for (_, item) in &package.items {
		if is_dtbook_item(item) {
			return Some(item.href.clone());
		}
		if item.media_type == "text/xml"
			&& fallback.is_none()
			&& Path::new(&item.href).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
		{
			fallback = Some(item.href.clone());
		}
	}
	fallback
}

pub(super) fn dir_of(resolved_path: &str) -> String {
	resolved_path.rsplit_once('/').map_or_else(String::new, |(dir, _)| dir.to_string())
}

#[cfg(test)]
mod tests;
