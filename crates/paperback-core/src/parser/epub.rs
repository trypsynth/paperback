use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
	path::Path,
};

use anyhow::{Context, Result};
use rayon::prelude::*;
use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		ConverterOutput, Parser, add_converter_markers_excluding_links,
		convert::{
			html_to_text::{HtmlSourceMode, HtmlToText},
			xml_to_text::XmlToText,
		},
		is_external_url,
		util::{
			path::{extract_title_from_path, resolve_relative_path},
			xml::collect_element_text,
		},
	},
	t,
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
	util::{
		text::{collapse_whitespace, trim_string, url_decode},
		zip::read_zip_entry_by_name,
	},
};

struct SectionContent {
	text: String,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
	id_positions: HashMap<String, usize>,
}

impl ConverterOutput for SectionContent {
	fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}
	fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}
	fn get_images(&self) -> &[ImageInfo] {
		&self.images
	}
	fn get_figures(&self) -> &[ImageInfo] {
		&self.figures
	}
	fn get_tables(&self) -> &[TableInfo] {
		&self.tables
	}
	fn get_separators(&self) -> &[SeparatorInfo] {
		&self.separators
	}
	fn get_lists(&self) -> &[ListInfo] {
		&self.lists
	}
	fn get_list_items(&self) -> &[ListItemInfo] {
		&self.list_items
	}
	fn get_bolds(&self) -> &[FormatInfo] {
		&self.bolds
	}
	fn get_italics(&self) -> &[FormatInfo] {
		&self.italics
	}
	fn get_underlines(&self) -> &[FormatInfo] {
		&self.underlines
	}
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

struct SpineConversionResult {
	buffer: DocumentBuffer,
	id_positions: HashMap<String, usize>,
	sections: Vec<SectionMeta>,
	conversion_errors: Vec<String>,
}

pub struct EpubParser;

impl Parser for EpubParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing epub");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open EPUB file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read EPUB as zip '{}'", context.file_path))?;
		let container_path = find_container_path(&mut archive)?;
		let opf_content = read_zip_entry_by_name(&mut archive, &container_path)?;
		let opf_dir = container_path.rfind('/').map_or(String::new(), |i| container_path[..i].to_string());
		let opf_doc = XmlDocument::parse_with_options(
			&opf_content,
			ParsingOptions { allow_dtd: true, ..ParsingOptions::default() },
		)
		.context("Failed to parse OPF document")?;
		let package_node = opf_doc
			.descendants()
			.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "package")
			// TRANSLATORS: Error shown when an EPUB's OPF document has no <package> element
			.ok_or_else(|| anyhow::anyhow!(t("OPF package element missing")))?;
		let (manifest, spine, nav_path, ncx_path, metadata) = parse_package(package_node, &opf_dir);
		let mut conversion = convert_spine_items(&context.file_path, &manifest, &spine, context.render_tables_inline);
		if conversion.sections.is_empty() {
			let reason = if conversion.conversion_errors.is_empty() {
				// TRANSLATORS: Reason given when an EPUB has no spine items that could be read
				t("no readable spine items")
			} else {
				// TRANSLATORS: Reason given when EPUB spine items failed to convert; {} is a comma-separated list of underlying errors
				t("failed to convert spine items: {}").replace("{}", &conversion.conversion_errors.join(", "))
			};
			tracing::warn!(path = %context.file_path, reason = %reason, "epub has no readable content");
			// TRANSLATORS: Error shown when an EPUB has no readable content; {} is the specific reason (see the two messages above)
			anyhow::bail!(t("EPUB has no readable content ({})").replace("{}", &reason));
		}
		let title = metadata
			.title
			.filter(|t| !t.trim().is_empty())
			.unwrap_or_else(|| {
				let fallback = extract_title_from_path(&context.file_path);
				tracing::debug!(path = %context.file_path, title = %fallback, "epub metadata title missing, using filename derived title");
				fallback
			});
		let author = metadata.author.unwrap_or_default();
		let toc_items = build_epub_toc(
			&mut archive,
			nav_path.as_deref(),
			ncx_path.as_deref(),
			&conversion.sections,
			&conversion.id_positions,
		);
		let page_items = build_epub_pages(
			&mut archive,
			nav_path.as_deref(),
			ncx_path.as_deref(),
			&conversion.sections,
			&conversion.id_positions,
		);
		let section_count = conversion.sections.len();
		let toc_count = toc_items.len();
		let page_count = page_items.len();
		for page in page_items {
			conversion.buffer.add_marker(Marker::new(MarkerType::PageBreak, page.offset).with_text(page.name));
		}
		let manifest_items: HashMap<String, String> =
			manifest.values().map(|item| (item.id.clone(), item.path.clone())).collect();
		let mut document = Document::new().with_title(title).with_author(author);
		document.set_buffer(conversion.buffer);
		document.id_positions = conversion.id_positions;
		document.spine_items = spine;
		document.manifest_items = manifest_items;
		document.toc_items = toc_items;
		tracing::debug!(
			path = %context.file_path,
			sections = section_count,
			toc_items = toc_count,
			page_items = page_count,
			"epub parsed successfully"
		);
		Ok(document)
	}
}

/// Reads and converts every spine item to text. Each rayon worker opens its own independent
/// `ZipArchive` (via `map_init`, so this happens once per task rather than once per item) so the
/// zip-read I/O and the HTML-to-text conversion both run across cores instead of the read being
/// serialized through one shared archive handle before conversion can start.
fn convert_spine_items(
	file_path: &str,
	manifest: &HashMap<String, ManifestItem>,
	spine: &[String],
	render_tables_inline: bool,
) -> SpineConversionResult {
	let converted: Vec<Result<(&ManifestItem, SectionContent), String>> = spine
		.par_iter()
		.map_init(
			|| {
				File::open(file_path)
					.map_err(|err| err.to_string())
					.and_then(|file| ZipArchive::new(BufReader::new(file)).map_err(|err| err.to_string()))
			},
			|archive_result, idref| {
				let item = manifest.get(idref).ok_or_else(|| format!("missing manifest item for {idref}"))?;
				let archive = archive_result.as_mut().map_err(|err| format!("{} ({err})", item.path))?;
				let data =
					read_zip_entry_by_name(archive, &item.path).map_err(|err| format!("{} ({err})", item.path))?;
				let section =
					convert_section(&data, render_tables_inline).map_err(|err| format!("{} ({err})", item.path))?;
				Ok((item, section))
			},
		)
		.collect();

	// Keep each spine item's original index (for the "Section N" label, which reflects spine
	// position even across skipped items) alongside its manifest item and converted section.
	let mut ok_entries: Vec<(usize, &ManifestItem, SectionContent)> = Vec::with_capacity(converted.len());
	let mut conversion_errors = Vec::new();
	for (idx, slot) in converted.into_iter().enumerate() {
		match slot {
			Ok((item, section)) => ok_entries.push((idx, item, section)),
			Err(err) => {
				tracing::warn!(error = %err, "skipping epub spine item that could not be read or converted");
				conversion_errors.push(err);
			}
		}
	}

	// `DocumentBuffer::from_parts` builds the buffer's content and per-char indices for every
	// section in parallel, in one pass, instead of appending them one at a time; it hands back
	// each section's `[start, end)` span so markers and id positions (below) can still be placed
	// relative to where each section landed.
	let texts: Vec<String> = ok_entries.iter_mut().map(|(_, _, section)| std::mem::take(&mut section.text)).collect();
	let (mut buffer, spans) = DocumentBuffer::from_parts(texts);

	let mut id_positions = HashMap::new();
	let mut sections = Vec::new();
	for (entry, span) in ok_entries.iter().zip(&spans) {
		let (idx, item, section) = entry;
		let section_start = span.start;
		let section_label = format!("Section {}", idx + 1);
		buffer.add_marker(
			Marker::new(MarkerType::SectionBreak, section_start)
				.with_text(section_label)
				.with_reference(item.path.clone()),
		);
		for (id, relative) in &section.id_positions {
			let absolute = section_start + relative;
			// Keep the first occurrence for bare ids to avoid later sections overwriting earlier ones.
			id_positions.entry(id.clone()).or_insert(absolute);
			id_positions.insert(format!("{}#{id}", item.path), absolute);
		}
		add_converter_markers_excluding_links(&mut buffer, section, section_start);
		for link in &section.links {
			let resolved = resolve_href(&item.path, &link.reference);
			buffer.add_marker(
				Marker::new(MarkerType::Link, section_start + link.offset)
					.with_text(link.text.clone())
					.with_reference(resolved),
			);
		}
		sections.push(SectionMeta { path: item.path.clone(), start: section_start, end: span.end });
	}
	SpineConversionResult { buffer, id_positions, sections, conversion_errors }
}

fn build_epub_toc<R: Read + Seek>(
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

fn find_container_path<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<String> {
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

struct PackageMetadata {
	title: Option<String>,
	author: Option<String>,
}

type PackageParts = (HashMap<String, ManifestItem>, Vec<String>, Option<String>, Option<String>, PackageMetadata);

fn parse_package(package: Node<'_, '_>, opf_dir: &str) -> PackageParts {
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

fn convert_section(content: &str, render_tables_inline: bool) -> Result<SectionContent> {
	let mut xml_converter = XmlToText::with_render_tables_inline(render_tables_inline);
	if xml_converter.convert(content) {
		return Ok(SectionContent {
			text: xml_converter.get_text(),
			headings: xml_converter.get_headings().to_vec(),
			links: xml_converter.get_links().to_vec(),
			images: xml_converter.get_images().to_vec(),
			figures: xml_converter.get_figures().to_vec(),
			tables: xml_converter.get_tables().to_vec(),
			separators: xml_converter.get_separators().to_vec(),
			lists: xml_converter.get_lists().to_vec(),
			list_items: xml_converter.get_list_items().to_vec(),
			bolds: xml_converter.get_bolds().to_vec(),
			italics: xml_converter.get_italics().to_vec(),
			underlines: xml_converter.get_underlines().to_vec(),
			id_positions: xml_converter.get_id_positions().clone(),
		});
	}
	tracing::warn!("epub section xml conversion failed, falling back to html converter");
	let mut html_converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	if html_converter.convert(content, HtmlSourceMode::NativeHtml) {
		return Ok(SectionContent {
			text: html_converter.get_text(),
			headings: html_converter.get_headings().to_vec(),
			links: html_converter.get_links().to_vec(),
			images: html_converter.get_images().to_vec(),
			figures: html_converter.get_figures().to_vec(),
			tables: html_converter.get_tables().to_vec(),
			separators: html_converter.get_separators().to_vec(),
			lists: html_converter.get_lists().to_vec(),
			list_items: html_converter.get_list_items().to_vec(),
			bolds: html_converter.get_bolds().to_vec(),
			italics: html_converter.get_italics().to_vec(),
			underlines: html_converter.get_underlines().to_vec(),
			id_positions: html_converter.get_id_positions().clone(),
		});
	}
	// currently unreachable, HtmlToText::convert always returns true today
	tracing::warn!("epub section content unsupported by both xml and html converters");
	// TRANSLATORS: Error shown when an EPUB spine item's content type cannot be converted
	anyhow::bail!(t("unsupported content"))
}

fn resolve_href(current_path: &str, target: &str) -> String {
	if is_external_url(target) {
		return target.to_string();
	}
	if target.starts_with('#') {
		return target.to_string();
	}
	let (path_part, fragment) = split_href(target);
	let resolved = if path_part.is_empty() {
		current_path.to_string()
	} else {
		let current_dir = current_path.rfind('/').map_or("", |i| &current_path[..i]);
		resolve_relative_path(current_dir, &path_part)
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
	let text = collect_element_text(link);
	trim_string(&collapse_whitespace(&text))
}

fn compute_nav_offset(reference: &str, sections: &[SectionMeta], id_positions: &HashMap<String, usize>) -> usize {
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

fn convert_navpoint(
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

fn build_epub_pages<R: Read + Seek>(
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
