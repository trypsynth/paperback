use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, Marker, MarkerType, ParserContext},
	parser::{Parser, util::path::extract_title_from_path},
	t,
	util::zip::read_zip_entry_by_name,
};

mod href;
mod package;
mod pages;
mod spine;
mod toc;

use package::{find_container_path, parse_package};
use pages::build_epub_pages;
use spine::convert_spine_items;
use toc::build_epub_toc;

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
