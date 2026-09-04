use std::{
	collections::{HashMap, HashSet},
	mem,
};

use anyhow::{Context, Result};
use libchm::{ChmFile, Entry, EntryCategory, EntrySel};
use rayon::prelude::*;
use scraper::{ElementRef, Html, Selector};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		ConverterOutput, Parser, add_converter_markers_excluding_links,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		is_external_url,
		util::path::{extract_title_from_path, resolve_relative_path},
	},
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
	util::encoding::convert_to_utf8,
};

pub struct ChmParser;

impl Parser for ChmParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing chm file");
		let mut chm = ChmFile::open(&context.file_path)
			.with_context(|| format!("Failed to open CHM file: {}", context.file_path))?;
		let mut html_files = Vec::new();
		// Keep the directory entry for each HTML file. Enumeration has already decoded it, so
		// holding on to it saves a full directory B-tree descent per file when we read them below.
		let mut entries_by_path: HashMap<String, Entry> = HashMap::new();
		let mut hhc_file = String::new();
		for entry in chm.entries(EntrySel::ALL)? {
			let lower_path = entry.path.to_lowercase();
			if lower_path.contains(".hhc") && (hhc_file.is_empty() || lower_path.contains("index.hhc")) {
				hhc_file.clone_from(&entry.path);
			}
			if (lower_path.contains(".htm") || lower_path.contains(".html")) && entry.category != EntryCategory::Special
			{
				let path = entry.path.clone();
				entries_by_path.insert(path.clone(), entry);
				html_files.push(path);
			}
		}
		html_files.sort();
		tracing::debug!(html_file_count = html_files.len(), hhc_found = !hhc_file.is_empty(), "chm structure detected");
		let title = parse_system_file(&mut chm).unwrap_or_else(|| {
			tracing::debug!(path = %context.file_path, "no title found in #SYSTEM record, falling back to filename");
			extract_title_from_path(&context.file_path)
		});
		if hhc_file.is_empty() {
			tracing::debug!(path = %context.file_path, "chm has no hhc file, table of contents will be empty");
		}
		let mut toc_items = if hhc_file.is_empty() { Vec::new() } else { parse_hhc_file(&mut chm, &hhc_file)? };
		let ordered_files = build_ordered_file_list(&html_files, &toc_items);
		let converted =
			convert_sections(&context.file_path, &ordered_files, &entries_by_path, context.render_tables_inline);
		// Keep each file's original index so the "Section N" label still reflects its position in
		// the document even when earlier files were skipped.
		let mut sections: Vec<(usize, &String, SectionContent)> = Vec::with_capacity(converted.len());
		for (idx, slot) in converted.into_iter().enumerate() {
			match slot {
				Ok(section) => sections.push((idx, &ordered_files[idx], section)),
				Err(err) => {
					tracing::warn!(error = %err, "skipping chm html entry that could not be read or converted");
				}
			}
		}
		// Sections are separated exactly as the one-at-a-time build separated them: a section that
		// does not already end with a newline gets one appended. An empty section after the first
		// adds nothing, because the text so far already ends with a newline.
		let texts: Vec<String> = sections
			.iter_mut()
			.enumerate()
			.map(|(pos, (_, _, section))| {
				let text = mem::take(&mut section.text);
				if text.ends_with('\n') || (text.is_empty() && pos > 0) { text } else { text + "\n" }
			})
			.collect();
		// `from_parts` builds the content and its per-character indices for every section in one
		// parallel pass, handing back where each section landed so markers can be placed after.
		let (mut buffer, spans) = DocumentBuffer::from_parts(texts);
		let mut id_positions = HashMap::new();
		let mut file_positions = HashMap::new();
		for ((idx, file_path, section), span) in sections.iter().zip(&spans) {
			let section_start = span.start;
			let normalized_path = normalize_path(file_path);
			file_positions.insert(normalized_path.clone(), section_start);
			// Store file-level position so fragment-less internal links can be resolved.
			id_positions.insert(normalized_path.clone(), section_start);
			for (id, relative_pos) in &section.id_positions {
				id_positions.insert(format!("{normalized_path}#{id}"), section_start + relative_pos);
			}
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, section_start)
					.with_text(format!("Section {}", idx + 1))
					.with_reference((*file_path).clone()),
			);
			add_converter_markers_excluding_links(&mut buffer, section, section_start);
			for link in &section.links {
				let resolved_href = resolve_chm_href(file_path, &link.reference);
				buffer.add_marker(
					Marker::new(MarkerType::Link, section_start + link.offset)
						.with_text(link.text.clone())
						.with_reference(resolved_href),
				);
			}
		}
		calculate_toc_offsets(&mut toc_items, &file_positions, &id_positions);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed chm file successfully");
		Ok(document)
	}
}

/// One converted HTML file: its text plus everything the converter recorded about it.
///
/// This is what crosses back from a rayon worker to the sequential assembly step, so it owns
/// its data rather than borrowing the converter that produced it.
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

/// Reads and converts every HTML file to text, returning one result per file in order.
///
/// Conversion dominates the cost of parsing a CHM (around 90% of the time on a large one) and
/// each file is independent, so this runs across cores. Each rayon worker opens its own
/// [`ChmFile`] via `map_init` (once per task rather than once per file), so the reads and the
/// LZX decompression parallelise too instead of being serialised through one shared handle.
/// Failures are returned rather than dropped, so the caller can log each one and carry on.
fn convert_sections(
	source_path: &str,
	ordered_files: &[String],
	entries_by_path: &HashMap<String, Entry>,
	render_tables_inline: bool,
) -> Vec<Result<SectionContent, String>> {
	ordered_files
		.par_iter()
		.map_init(
			|| ChmFile::open(source_path).map_err(|err| err.to_string()),
			|chm_result, file_path| {
				let chm = chm_result.as_mut().map_err(|err| format!("{file_path} ({err})"))?;
				// Enumeration already handed us the entry; look it up again only if this path
				// somehow was not one of the files we enumerated.
				let content_bytes = match entries_by_path.get(file_path) {
					Some(entry) => chm.read(entry),
					None => chm.find(file_path).and_then(|entry| chm.read(&entry)),
				}
				.map_err(|err| format!("{file_path} ({err})"))?;
				if content_bytes.is_empty() {
					return Err(format!("{file_path} (entry is empty)"));
				}
				convert_section(&content_bytes, render_tables_inline)
					.ok_or_else(|| format!("{file_path} (html conversion failed)"))
			},
		)
		.collect()
}

/// Convert one file's raw bytes to text, transcoding to UTF-8 first.
fn convert_section(content_bytes: &[u8], render_tables_inline: bool) -> Option<SectionContent> {
	let utf8_content = convert_to_utf8(content_bytes);
	let mut converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	// currently always true, HtmlToText::convert has no failure path today
	if !converter.convert(&utf8_content, HtmlSourceMode::NativeHtml) {
		return None;
	}
	Some(SectionContent {
		text: converter.get_text(),
		headings: converter.get_headings().to_vec(),
		links: converter.get_links().to_vec(),
		images: ConverterOutput::get_images(&converter).to_vec(),
		figures: ConverterOutput::get_figures(&converter).to_vec(),
		tables: converter.get_tables().to_vec(),
		separators: converter.get_separators().to_vec(),
		lists: converter.get_lists().to_vec(),
		list_items: converter.get_list_items().to_vec(),
		bolds: converter.get_bolds().to_vec(),
		italics: converter.get_italics().to_vec(),
		underlines: converter.get_underlines().to_vec(),
		id_positions: converter.get_id_positions().clone(),
	})
}

fn parse_system_file(chm: &mut ChmFile) -> Option<String> {
	let content = chm.find("/#SYSTEM").and_then(|e| chm.read(&e)).ok()?;
	if content.len() < 4 {
		return None;
	}
	let read_le16 = |data: &[u8], offset: usize| -> u16 { u16::from_le_bytes([data[offset], data[offset + 1]]) };
	let mut index = 4;
	while index + 4 <= content.len() {
		let code = read_le16(&content, index);
		let length = read_le16(&content, index + 2) as usize;
		if index + 4 + length > content.len() {
			break;
		}
		// Code 3 is the title.
		if code == 3 && length > 0 {
			let title_bytes = &content[index + 4..index + 4 + length];
			let title_bytes =
				if title_bytes.last() == Some(&0) { &title_bytes[..title_bytes.len() - 1] } else { title_bytes };
			let title = String::from_utf8_lossy(title_bytes).to_string();
			if !title.trim().is_empty() {
				return Some(title);
			}
		}
		index += 4 + length;
	}
	None
}

fn parse_hhc_file(chm: &mut ChmFile, hhc_path: &str) -> Result<Vec<TocItem>> {
	let content_bytes = chm
		.find(hhc_path)
		.and_then(|e| chm.read(&e))
		.with_context(|| format!("Failed to read .hhc file: {hhc_path}"))?;
	if content_bytes.is_empty() {
		tracing::debug!(path = %hhc_path, "hhc file is empty, table of contents will be empty");
		return Ok(Vec::new());
	}
	let content = convert_to_utf8(&content_bytes);
	let document = Html::parse_document(&content);
	let body_selector = Selector::parse("body").unwrap();
	let Some(body) = document.select(&body_selector).next() else {
		tracing::debug!(path = %hhc_path, "hhc file has no body element, table of contents will be empty");
		return Ok(Vec::new());
	};
	let mut toc_items = Vec::new();
	parse_hhc_node(body, &mut toc_items);
	Ok(toc_items)
}

fn parse_hhc_node(node: ElementRef, items: &mut Vec<TocItem>) {
	let param_selector = Selector::parse("param").unwrap();
	let children: Vec<_> = node.children().collect();
	let mut consumed_indices = HashSet::new();
	for (index, child) in children.iter().enumerate() {
		if consumed_indices.contains(&index) {
			continue;
		}
		let Some(child_element) = child.value().as_element() else {
			continue;
		};
		let Some(child_ref) = ElementRef::wrap(*child) else {
			continue;
		};
		match child_element.name() {
			"ul" => {
				parse_hhc_node(child_ref, items);
			}
			"li" => {
				let mut name = String::new();
				let mut local = String::new();
				for obj_child in child_ref.children() {
					if let Some(obj_element) = obj_child.value().as_element()
						&& obj_element.name() == "object"
						&& let Some(object_ref) = ElementRef::wrap(obj_child)
					{
						for param in object_ref.select(&param_selector) {
							let param_name = param.value().attr("name").unwrap_or("").to_lowercase();
							let param_value = param.value().attr("value").unwrap_or("");
							match param_name.as_str() {
								"name" => name = param_value.to_string(),
								"local" => local = param_value.to_string(),
								_ => {}
							}
						}
					}
				}
				if !name.is_empty() {
					let mut item = TocItem::new(name, local, usize::MAX);
					let mut found_child_ul = false;
					// PATTERN 1: Check for child UL (standard CHM pattern)
					for nested_child in child_ref.children() {
						if let Some(nested_element) = nested_child.value().as_element()
							&& nested_element.name() == "ul"
							&& let Some(nested_ref) = ElementRef::wrap(nested_child)
						{
							parse_hhc_node(nested_ref, &mut item.children);
							found_child_ul = true;
						}
					}
					// PATTERN 2: Check for sibling UL elements, as seen in nvgt.chm.
					if !found_child_ul {
						let mut next_element = None;
						for (next_idx, child) in children.iter().enumerate().skip(index + 1) {
							if let Some(next_el) = child.value().as_element() {
								if next_el.name() == "ul" {
									next_element = Some((next_idx, *child));
									break;
								} else if next_el.name() == "li" {
									break;
								}
							}
						}
						if let Some((ul_index, sibling_node)) = next_element
							&& let Some(sibling_ref) = ElementRef::wrap(sibling_node)
						{
							parse_hhc_node(sibling_ref, &mut item.children);
							consumed_indices.insert(ul_index); // Mark as consumed
						}
					}
					items.push(item);
				}
			}
			_ => {}
		}
	}
}

fn build_ordered_file_list(html_files: &[String], toc_items: &[TocItem]) -> Vec<String> {
	if toc_items.is_empty() {
		return html_files.to_vec();
	}
	let mut ordered = Vec::new();
	let mut seen = HashSet::new();
	let mut path_map = HashMap::new();
	for file in html_files {
		let normalized = normalize_path(file);
		path_map.insert(normalized, file.clone());
	}
	let mut toc_files = Vec::new();
	collect_toc_files(toc_items, &mut toc_files);
	for toc_file in toc_files {
		let normalized = normalize_path(&toc_file);
		if let Some(actual_path) = path_map.get(&normalized)
			&& seen.insert(normalized)
		{
			ordered.push(actual_path.clone());
		}
	}
	for file in html_files {
		let normalized = normalize_path(file);
		if !seen.contains(&normalized) {
			ordered.push(file.clone());
		}
	}
	ordered
}

fn collect_toc_files(items: &[TocItem], files: &mut Vec<String>) {
	for item in items {
		if !item.reference.is_empty() {
			let file_path = item.reference.split_once('#').map_or(item.reference.as_str(), |(path, _)| path);
			if !file_path.is_empty() && !files.contains(&file_path.to_string()) {
				files.push(file_path.to_string());
			}
		}
		collect_toc_files(&item.children, files);
	}
}

fn normalize_path(path: &str) -> String {
	let mut result = path.replace('\\', "/").to_lowercase();
	if !result.starts_with('/') {
		result.insert(0, '/');
	}
	result
}

fn resolve_chm_href(current_file: &str, href: &str) -> String {
	if is_external_url(href) {
		return href.to_string();
	}
	let (path_part, fragment) = href.split_once('#').map_or((href, None), |(p, f)| (p, Some(f)));
	let resolved_path = if path_part.is_empty() {
		normalize_path(current_file)
	} else {
		let current_normalized = normalize_path(current_file);
		let current_dir = current_normalized.rfind('/').map_or("", |i| &current_normalized[..i]);
		let path_normalized = path_part.replace('\\', "/");
		format!("/{}", resolve_relative_path(current_dir, &path_normalized)).to_lowercase()
	};
	match fragment {
		Some(frag) if !frag.is_empty() => format!("{resolved_path}#{frag}"),
		_ => resolved_path,
	}
}

fn calculate_toc_offsets(
	items: &mut [TocItem],
	file_positions: &HashMap<String, usize>,
	id_positions: &HashMap<String, usize>,
) {
	for item in items {
		if !item.reference.is_empty() {
			item.offset = calculate_offset_from_reference(&item.reference, file_positions, id_positions);
		}
		calculate_toc_offsets(&mut item.children, file_positions, id_positions);
	}
}

fn calculate_offset_from_reference(
	reference: &str,
	file_positions: &HashMap<String, usize>,
	id_positions: &HashMap<String, usize>,
) -> usize {
	let (file_path, fragment) =
		reference.find('#').map_or((reference, None), |pos| (&reference[..pos], Some(&reference[pos + 1..])));
	let normalized_path = normalize_path(file_path);
	if let Some(fragment_id) = fragment {
		let id_key = format!("{normalized_path}#{fragment_id}");
		if let Some(&offset) = id_positions.get(&id_key) {
			return offset;
		}
	}
	file_positions.get(&normalized_path).copied().unwrap_or(usize::MAX)
}
