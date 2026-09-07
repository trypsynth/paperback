use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use libchm::ChmFile;
use scraper::{ElementRef, Html, Selector};

use super::href::normalize_path;
use crate::{document::TocItem, util::encoding::convert_to_utf8};

pub(super) fn parse_system_file(chm: &mut ChmFile) -> Option<String> {
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

pub(super) fn parse_hhc_file(chm: &mut ChmFile, hhc_path: &str) -> Result<Vec<TocItem>> {
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

pub(super) fn build_ordered_file_list(html_files: &[String], toc_items: &[TocItem]) -> Vec<String> {
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

pub(super) fn calculate_toc_offsets(
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
