//! Building a PDF's table of contents from its bookmark outline, snapping each bookmark to
//! the nearest matching line on its target page (bookmark destinations only carry a page,
//! not an in-page offset), plus the shared flat-list-to-tree builder used both here and by
//! [`super::structure`]'s tagged heading extraction.

use std::collections::HashSet;

use pdfium::PdfiumDocument;

use super::text::sanitize_pdf_text;
use crate::{
	document::{DocumentBuffer, Marker, MarkerType, TocItem},
	util::text::{collapse_whitespace, trim_string},
};

pub(super) fn add_heading_markers(buffer: &mut DocumentBuffer, items: &[TocItem], level: i32) {
	for item in items {
		let marker_type = match level {
			1 => MarkerType::Heading1,
			2 => MarkerType::Heading2,
			3 => MarkerType::Heading3,
			4 => MarkerType::Heading4,
			5 => MarkerType::Heading5,
			_ => MarkerType::Heading6,
		};
		buffer.add_marker(Marker::new(marker_type, item.offset).with_text(item.name.clone()).with_level(level));
		add_heading_markers(buffer, &item.children, level + 1);
	}
}

pub(super) fn extract_toc(
	document: &PdfiumDocument,
	page_offsets: &[usize],
	page_lines_info: &[Vec<(usize, String)>],
) -> Vec<TocItem> {
	let Ok(bookmarks) = document.toc(16) else {
		return Vec::new();
	};
	if bookmarks.is_empty() {
		return Vec::new();
	}
	let mut items = Vec::<(u32, TocItem)>::new();
	let mut used_offsets = HashSet::new();
	let bookmark_count = bookmarks.len();
	let mut skipped_count = 0usize;
	for bookmark in &bookmarks {
		let Some(level) = bookmark.level() else {
			skipped_count += 1;
			continue;
		};
		let Ok(raw_title) = bookmark.title() else {
			skipped_count += 1;
			continue;
		};
		let title = trim_string(&collapse_whitespace(&sanitize_pdf_text(&raw_title)));
		if title.is_empty() {
			skipped_count += 1;
			continue;
		}
		let Ok(dest) = bookmark.dest(document) else {
			skipped_count += 1;
			continue;
		};
		let Some(page_index) = dest.index(document) else {
			skipped_count += 1;
			continue;
		};
		let Ok(page_index) = usize::try_from(page_index) else {
			skipped_count += 1;
			continue;
		};
		let Some(&page_start_offset) = page_offsets.get(page_index) else {
			skipped_count += 1;
			continue;
		};
		let mut actual_offset = page_start_offset;
		let mut actual_title = title.clone();
		if let Some(lines) = page_lines_info.get(page_index) {
			let title_alpha: String = title.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
			for (line_offset, line) in lines {
				let line_alpha: String = line.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
				let ends_with_number = line.chars().last().unwrap_or(' ').is_ascii_digit();
				let is_all_caps = line.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase);
				let is_page_header = ends_with_number && is_all_caps;
				if (line_alpha == title_alpha
					|| line_alpha.starts_with(&title_alpha)
					|| line_alpha.ends_with(&title_alpha))
					&& !title_alpha.is_empty()
					&& !is_page_header
				{
					actual_offset = *line_offset;
					if line.len() < 150 {
						actual_title.clone_from(line);
					}
					break;
				}
			}
		}
		while used_offsets.contains(&actual_offset) {
			actual_offset += 1;
		}
		used_offsets.insert(actual_offset);
		items.push((level, TocItem::new(actual_title, String::new(), actual_offset)));
	}
	if skipped_count > 0 {
		tracing::warn!(skipped_count, bookmark_count, "skipped some bookmarks while building pdf toc");
	}
	build_toc_tree(items)
}

pub(super) fn build_toc_tree(flat_items: Vec<(u32, TocItem)>) -> Vec<TocItem> {
	let mut root = Vec::<TocItem>::new();
	let mut path = Vec::<usize>::new();
	let mut level_stack = Vec::<u32>::new();
	for (level, item) in flat_items {
		while let Some(&last_level) = level_stack.last() {
			if last_level < level {
				break;
			}
			level_stack.pop();
			path.pop();
		}
		let siblings = children_at_path_mut(&mut root, &path);
		siblings.push(item);
		path.push(siblings.len() - 1);
		level_stack.push(level);
	}
	root
}

fn children_at_path_mut<'a>(nodes: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = nodes;
	for &index in path {
		current = &mut current[index].children;
	}
	current
}
