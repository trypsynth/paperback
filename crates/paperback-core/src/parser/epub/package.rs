//! Locating and parsing an EPUB's OPF package document: `META-INF/container.xml` (to find
//! the OPF itself), then the OPF's `<manifest>`/`<spine>`/`<metadata>` into the pieces the
//! rest of the parser needs (manifest items, reading order, nav/NCX paths, title/author).

use std::{
	collections::HashMap,
	io::{Read, Seek},
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	parser::util::path::resolve_relative_path,
	t,
	util::{text::url_decode, zip::read_zip_entry_by_name},
};

pub(super) struct ManifestItem {
	pub(super) id: String,
	pub(super) path: String,
	pub(super) media_type: String,
	pub(super) properties: Vec<String>,
}

pub(super) struct PackageMetadata {
	pub(super) title: Option<String>,
	pub(super) author: Option<String>,
}

pub(super) type PackageParts =
	(HashMap<String, ManifestItem>, Vec<String>, Option<String>, Option<String>, PackageMetadata);

pub(super) fn find_container_path<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<String> {
	let container_xml = read_zip_entry_by_name(archive, "META-INF/container.xml")
		.context("Failed to read META-INF/container.xml in EPUB")?;
	let doc = XmlDocument::parse_with_options(
		&container_xml,
		ParsingOptions { allow_dtd: true, ..ParsingOptions::default() },
	)
	.context("Failed to parse container.xml")?;
	for node in doc.descendants() {
		if node.node_type() == NodeType::Element
			&& node.tag_name().name() == "rootfile"
			&& let Some(path) = node.attribute("full-path")
		{
			return Ok(path.to_string());
		}
	}
	// TRANSLATORS: Error shown when an EPUB's container.xml is missing its rootfile reference
	anyhow::bail!(t("rootfile not found in container.xml"))
}

pub(super) fn parse_package(package: Node<'_, '_>, opf_dir: &str) -> PackageParts {
	let mut manifest = HashMap::new();
	let mut spine = Vec::new();
	let mut nav_path = None;
	let mut ncx_path = None;
	let mut title = None;
	let mut author = None;
	for child in package.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		match child.tag_name().name() {
			"metadata" => {
				title = child
					.descendants()
					.find(|n| n.node_type() == NodeType::Element && n.tag_name().name().eq_ignore_ascii_case("title"))
					.and_then(|n| n.text().map(str::to_string));
				author = child
					.descendants()
					.find(|n| n.node_type() == NodeType::Element && n.tag_name().name().eq_ignore_ascii_case("creator"))
					.and_then(|n| n.text().map(str::to_string));
			}
			"manifest" => {
				for item in
					child.children().filter(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "item")
				{
					let Some(id) = item.attribute("id") else {
						tracing::warn!("skipping epub manifest item with no id attribute");
						continue;
					};
					let Some(href) = item.attribute("href") else {
						tracing::warn!(id = %id, "skipping epub manifest item with no href attribute");
						continue;
					};
					let media_type = item.attribute("media-type").unwrap_or("").to_string();
					let properties = item
						.attribute("properties")
						.map_or_else(Vec::new, |v| v.split_whitespace().map(ToString::to_string).collect());
					let resolved = resolve_relative_path(opf_dir, &url_decode(href));
					let manifest_item =
						ManifestItem { id: id.to_string(), path: resolved.clone(), media_type, properties };
					if manifest_item.properties.iter().any(|p| p == "nav") {
						nav_path = Some(resolved.clone());
					}
					if manifest_item.media_type == "application/x-dtbncx+xml" {
						ncx_path = Some(resolved.clone());
					}
					manifest.insert(id.to_string(), manifest_item);
				}
			}
			"spine" => {
				if ncx_path.is_none()
					&& let Some(id) = child.attribute("toc")
				{
					ncx_path = manifest.get(id).map(|m| m.path.clone());
				}
				for itemref in
					child.children().filter(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "itemref")
				{
					if let Some(idref) = itemref.attribute("idref") {
						spine.push(idref.to_string());
					}
				}
			}
			_ => {}
		}
	}
	(manifest, spine, nav_path, ncx_path, PackageMetadata { title, author })
}
