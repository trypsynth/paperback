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
	rootfiles(archive)?
		.into_iter()
		.next()
		.map(|rootfile| rootfile.path)
		// TRANSLATORS: Error shown when an EPUB's container.xml is missing its rootfile reference
		.ok_or_else(|| anyhow::anyhow!(t("rootfile not found in container.xml")))
}

/// A package document `container.xml` names, and how it is read when it names several renditions of one book: `tactile` for braille, `textual` for print.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rootfile {
	pub path: String,
	pub access_mode: Option<String>,
}

pub(crate) fn rootfiles<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<Rootfile>> {
	const RENDITION: &str = "http://www.idpf.org/2013/rendition";
	let container_xml = read_zip_entry_by_name(archive, "META-INF/container.xml")
		.context("Failed to read META-INF/container.xml in EPUB")?;
	let doc = XmlDocument::parse_with_options(
		&container_xml,
		ParsingOptions { allow_dtd: true, ..ParsingOptions::default() },
	)
	.context("Failed to parse container.xml")?;
	Ok(doc
		.descendants()
		.filter(|node| node.node_type() == NodeType::Element && node.tag_name().name() == "rootfile")
		.filter_map(|node| {
			let path = node.attribute("full-path")?.to_string();
			let access_mode = node.attribute((RENDITION, "accessMode")).map(str::to_string);
			Some(Rootfile { path, access_mode })
		})
		.collect())
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
						ncx_path = Some(resolved);
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

#[cfg(test)]
mod tests;
