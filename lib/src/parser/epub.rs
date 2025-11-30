use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
	path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	html_to_text::{HeadingInfo, HtmlSourceMode, HtmlToText, LinkInfo, ListInfo, ListItemInfo},
	parser::{
		Parser,
		utils::{extract_title_from_path, heading_level_to_marker_type, read_zip_entry},
	},
	utils::text::{collapse_whitespace, trim_string, url_decode},
	xml_to_text::XmlToText,
};

struct SectionContent {
	text: String,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	id_positions: HashMap<String, usize>,
}

struct SectionMeta {
	path: String,
	start: usize,
	end: usize,
}

struct ManifestItem {
	id: String,
	path: String,
	media_type: String,
	properties: Vec<String>,
}

pub struct EpubParser;

impl Parser for EpubParser {
	fn name(&self) -> &'static str {
		"EPUB Books"
	}

	fn extensions(&self) -> &[&str] {
		&["epub"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_SECTIONS | ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	#[allow(clippy::too_many_lines)]
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open EPUB file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read EPUB as zip '{}'", context.file_path))?;
		let container_path = find_container_path(&mut archive)?;
		let opf_content = read_zip_entry(&mut archive, &container_path)?;
		let opf_dir = Path::new(&container_path).parent().unwrap_or_else(|| Path::new("")).to_path_buf();
		let opf_doc = XmlDocument::parse_with_options(
			&opf_content,
			ParsingOptions { allow_dtd: true, ..ParsingOptions::default() },
		)
		.context("Failed to parse OPF document")?;
		let package_node = opf_doc
			.descendants()
			.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "package")
			.ok_or_else(|| anyhow::anyhow!("OPF package element missing"))?;
		let (manifest, spine, nav_path, ncx_path, metadata) = parse_package(package_node, &opf_dir);
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		let mut sections = Vec::new();
		let mut conversion_errors = Vec::new();
		for (idx, idref) in spine.iter().enumerate() {
			let Some(item) = manifest.get(idref) else {
				conversion_errors.push(format!("missing manifest item for {idref}"));
				continue;
			};
			let section_data = match read_zip_entry(&mut archive, &item.path) {
				Ok(v) => v,
				Err(err) => {
					conversion_errors.push(format!("{} ({err})", item.path));
					continue;
				}
			};
			let section_start = buffer.current_position();
			let section_label = format!("Section {}", idx + 1);
			buffer.add_marker(Marker::new(MarkerType::SectionBreak, section_start).with_text(section_label));
			match convert_section(&section_data) {
				Ok(section) => {
					for (id, relative) in section.id_positions {
						let absolute = section_start + relative;
						// Keep the first occurrence for bare ids to avoid later sections overwriting earlier ones.
						id_positions.entry(id.clone()).or_insert(absolute);
						id_positions.insert(format!("{}#{id}", item.path), absolute);
					}
					for heading in section.headings {
						let marker_type = heading_level_to_marker_type(heading.level);
						buffer.add_marker(
							Marker::new(marker_type, section_start + heading.offset)
								.with_text(heading.text.clone())
								.with_level(heading.level),
						);
					}
					for link in section.links {
						let resolved = resolve_href(&item.path, &link.reference);
						buffer.add_marker(
							Marker::new(MarkerType::Link, section_start + link.offset)
								.with_text(link.text.clone())
								.with_reference(resolved),
						);
					}
					for list in section.lists {
						buffer.add_marker(
							Marker::new(MarkerType::List, section_start + list.offset).with_level(list.item_count),
						);
					}
					for list_item in section.list_items {
						buffer.add_marker(
							Marker::new(MarkerType::ListItem, section_start + list_item.offset)
								.with_text(list_item.text.clone())
								.with_level(list_item.level),
						);
					}
					if !section.text.is_empty() {
						buffer.append(&section.text);
						if !buffer.content.ends_with('\n') {
							buffer.append("\n");
						}
					}
					let section_end = buffer.current_position();
					sections.push(SectionMeta { path: item.path.clone(), start: section_start, end: section_end });
				}
				Err(err) => {
					conversion_errors.push(format!("{} ({err})", item.path));
				}
			}
		}
		if sections.is_empty() {
			let reason = if conversion_errors.is_empty() {
				String::from("no readable spine items")
			} else {
				format!("failed to convert spine items: {}", conversion_errors.join(", "))
			};
			anyhow::bail!("EPUB has no readable content ({reason})");
		}
		let title = metadata
			.title
			.filter(|t| !t.trim().is_empty())
			.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = metadata.author.unwrap_or_default();
		let toc_items = if let Some(nav_path) = nav_path {
			build_toc_from_nav_document(&mut archive, &nav_path, &sections, &id_positions)
				.or_else(|| {
					ncx_path.as_deref().and_then(|p| build_toc_from_ncx(&mut archive, p, &sections, &id_positions))
				})
				.unwrap_or_else(Vec::new)
		} else if let Some(ncx) = ncx_path {
			build_toc_from_ncx(&mut archive, &ncx, &sections, &id_positions).unwrap_or_default()
		} else {
			Vec::new()
		};
		let manifest_items: HashMap<String, String> =
			manifest.values().map(|item| (item.id.clone(), item.path.clone())).collect();
		let mut document = Document::new().with_title(title).with_author(author);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.spine_items = spine;
		document.manifest_items = manifest_items;
		document.toc_items = toc_items;
		Ok(document)
	}
}

fn find_container_path<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<String> {
	let container_xml =
		read_zip_entry(archive, "META-INF/container.xml").context("Failed to read META-INF/container.xml in EPUB")?;
	let doc = XmlDocument::parse_with_options(
		&container_xml,
		ParsingOptions { allow_dtd: true, ..ParsingOptions::default() },
	)
	.context("Failed to parse container.xml")?;
	for node in doc.descendants() {
		if node.node_type() == NodeType::Element && node.tag_name().name() == "rootfile" {
			if let Some(path) = node.attribute("full-path") {
				return Ok(path.to_string());
			}
		}
	}
	anyhow::bail!("rootfile not found in container.xml")
}

struct PackageMetadata {
	title: Option<String>,
	author: Option<String>,
}

type PackageParts = (HashMap<String, ManifestItem>, Vec<String>, Option<String>, Option<String>, PackageMetadata);

fn parse_package(package: Node<'_, '_>, opf_dir: &Path) -> PackageParts {
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
					let Some(id) = item.attribute("id") else { continue };
					let Some(href) = item.attribute("href") else { continue };
					let media_type = item.attribute("media-type").unwrap_or("").to_string();
					let properties = item
						.attribute("properties")
						.map_or_else(Vec::new, |v| v.split_whitespace().map(ToString::to_string).collect());
					let resolved = normalize_path(&opf_dir.join(url_decode(href)));
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
				if ncx_path.is_none() {
					if let Some(id) = child.attribute("toc") {
						ncx_path = manifest.get(id).map(|m| m.path.clone());
					}
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

fn convert_section(content: &str) -> Result<SectionContent> {
	let mut xml_converter = XmlToText::new();
	if xml_converter.convert(content) {
		return Ok(SectionContent {
			text: xml_converter.get_text(),
			headings: xml_converter.get_headings().to_vec(),
			links: xml_converter.get_links().to_vec(),
			lists: xml_converter.get_lists().to_vec(),
			list_items: xml_converter.get_list_items().to_vec(),
			id_positions: xml_converter.get_id_positions().clone(),
		});
	}
	let mut html_converter = HtmlToText::new();
	if html_converter.convert(content, HtmlSourceMode::NativeHtml) {
		return Ok(SectionContent {
			text: html_converter.get_text(),
			headings: html_converter.get_headings().to_vec(),
			links: html_converter.get_links().to_vec(),
			lists: html_converter.get_lists().to_vec(),
			list_items: html_converter.get_list_items().to_vec(),
			id_positions: html_converter.get_id_positions().clone(),
		});
	}
	anyhow::bail!("unsupported content")
}

fn resolve_href(current_path: &str, target: &str) -> String {
	let lower = target.to_ascii_lowercase();
	if lower.starts_with("http:") || lower.starts_with("https:") || lower.starts_with("mailto:") {
		return target.to_string();
	}
	if target.starts_with('#') {
		return target.to_string();
	}
	let (path_part, fragment) = split_href(target);
	let resolved = if path_part.is_empty() {
		current_path.to_string()
	} else {
		let mut base = PathBuf::from(current_path);
		base.pop();
		let joined = if path_part.starts_with('/') {
			PathBuf::from(path_part.trim_start_matches('/'))
		} else {
			base.join(path_part)
		};
		normalize_path(&joined)
	};
	if let Some(frag) = fragment {
		if frag.is_empty() { resolved } else { format!("{resolved}#{frag}") }
	} else {
		resolved
	}
}

fn split_href(input: &str) -> (String, Option<String>) {
	let decoded = url_decode(input);
	let trimmed = decoded.strip_prefix("epub://").unwrap_or(&decoded);
	if let Some((path, frag)) = trimmed.split_once('#') {
		(path.to_string(), Some(frag.to_string()))
	} else {
		(trimmed.to_string(), None)
	}
}

fn normalize_path(path: &Path) -> String {
	let mut components = Vec::new();
	for component in path.components() {
		match component {
			Component::ParentDir => {
				components.pop();
			}
			Component::Normal(part) => components.push(part.to_string_lossy().to_string()),
			_ => {}
		}
	}
	components.join("/")
}

fn build_toc_from_nav_document<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	nav_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let nav_content = read_zip_entry(archive, nav_path).ok()?;
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

fn parse_nav_list(
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

fn parse_nav_item(
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
	let mut text = String::new();
	collect_text(link, &mut text);
	trim_string(&collapse_whitespace(&text))
}

fn collect_text(node: Node<'_, '_>, buffer: &mut String) {
	match node.node_type() {
		NodeType::Text => {
			if let Some(value) = node.text() {
				buffer.push_str(value);
			}
		}
		NodeType::Element => {
			for child in node.children() {
				collect_text(child, buffer);
			}
		}
		_ => {}
	}
}

fn compute_nav_offset(reference: &str, sections: &[SectionMeta], id_positions: &HashMap<String, usize>) -> usize {
	let (path_part, fragment) = split_href(reference);
	if let Some(section) = sections.iter().find(|section| section.path == path_part) {
		if let Some(frag) = fragment.as_deref() {
			if let Some(offset) = id_positions.get(&format!("{path_part}#{frag}")).or_else(|| id_positions.get(frag)) {
				if *offset >= section.start && *offset < section.end {
					return *offset;
				}
			}
		}
		return section.start;
	}
	if let Some(frag) = fragment {
		if let Some(offset) = id_positions.get(&frag) {
			return *offset;
		}
	}
	// Fallback: match by file name if full path didn't resolve.
	if let Some(name) = Path::new(&path_part).file_name().and_then(|n| n.to_str()) {
		if let Some(section) = sections.iter().find(|section| {
			Path::new(&section.path)
				.file_name()
				.and_then(|n| n.to_str())
				.is_some_and(|base| base.eq_ignore_ascii_case(name))
		}) {
			return section.start;
		}
	}
	0
}

fn build_toc_from_ncx<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	ncx_path: &str,
	sections: &[SectionMeta],
	id_positions: &HashMap<String, usize>,
) -> Option<Vec<TocItem>> {
	let ncx_content = read_zip_entry(archive, ncx_path).ok()?;
	let ncx_doc =
		XmlDocument::parse_with_options(&ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_map =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navMap")?;
	let mut items = Vec::new();
	for navpoint in nav_map.children() {
		if navpoint.node_type() == NodeType::Element && navpoint.tag_name().name() == "navPoint" {
			if let Some(item) = convert_navpoint(navpoint, sections, id_positions) {
				items.push(item);
			}
		}
	}
	if items.is_empty() { None } else { Some(items) }
}

fn convert_navpoint(nav: Node, sections: &[SectionMeta], id_positions: &HashMap<String, usize>) -> Option<TocItem> {
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
	let reference = resolve_href("", content_src);
	let offset = compute_nav_offset(&reference, sections, id_positions);
	let mut item = TocItem::new(label, reference, offset);
	for child in nav.children() {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "navPoint" {
			if let Some(child_item) = convert_navpoint(child, sections, id_positions) {
				item.children.push(child_item);
			}
		}
	}
	Some(item)
}
