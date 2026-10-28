//! Some PDFs advertise a structure tree yet leave most of their text untagged, so a page below [`MIN_MCID_COVERAGE`] falls back to plain extraction.

use std::collections::HashMap;

use self::{
	marked_content::{MIN_MCID_COVERAGE, PageText, TreeFacts, collect_text, first_marked_content_id},
	tables::{append_pdf_table_to_buffer, build_html_table},
};
use super::{
	images::{UnclaimedImages, append_image},
	text::PageEvidence,
};
use crate::{
	document::{DocLen, DocPos, DocumentBuffer, Marker, MarkerType, TocItem},
	pdf_engine::{PdfPage, PdfTextPage, Tag},
	util::text::{collapse_leaders, collapse_whitespace, trim_string},
};

mod marked_content;
mod tables;

#[allow(clippy::too_many_arguments)]
pub(super) fn extract_tagged_page_text(
	page: &PdfPage,
	text_page: &PdfTextPage,
	page_index: i32,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(DocPos, String)>,
	flat_toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
	image_tops: &[f64],
	image_paths: &[String],
	evidence: &mut PageEvidence,
) -> bool {
	let Some(struct_tree) = page.tags() else { return false };
	let child_count = struct_tree.child_count();
	if child_count == 0 {
		return false;
	}
	let facts = TreeFacts::of(&struct_tree);
	// A `Figure` is not always one drawn image, so counting both the tree's figures and the page's image objects would announce some twice.
	let unclaimed = UnclaimedImages::new(
		if facts.claims_figures { &[] } else { image_tops },
		if facts.claims_figures { &[] } else { image_paths },
		page_index,
	);
	let content = marked_content::read(text_page, &facts, !unclaimed.is_empty(), evidence);
	let coverage = content.coverage;
	let tagged_trusted = coverage >= MIN_MCID_COVERAGE;
	tracing::debug!(page_index, coverage, tagged_trusted, "computed mcid coverage for page structure tree");
	if !tagged_trusted {
		tracing::warn!(
			page_index,
			coverage,
			"page advertises a structure tree but mcid coverage is too low, falling back to plain extraction"
		);
		return false;
	}
	// A top-level element with nothing on this page comes back as no child, and reporting such a page as tagged would stop plain extraction from reading text that is there.
	if content.text.is_empty() || (0..child_count).all(|i| struct_tree.child(i).is_none()) {
		tracing::warn!(page_index, "page structure tree leads to no text, falling back to plain extraction");
		return false;
	}
	let mut current_block = String::new();
	let mut pending_label = String::new();
	let mut page_text = PageText::new(&content.text);
	// Only a tree that names figures can ask for one's picture, so the walk over the page's objects that answers where each image sits is skipped for the rest.
	let figure_images = if facts.claims_figures { page.image_paths_by_mcid() } else { HashMap::new() };
	let mut images = ImagePlacement {
		mcid_tops: &content.tops,
		unclaimed,
		page_index,
		figure_images: &figure_images,
		block_top: None,
	};
	for i in 0..child_count {
		if let Some(child) = struct_tree.child(i) {
			process_struct_element(
				&child,
				&mut page_text,
				buffer,
				page_display_text,
				&mut current_block,
				&mut pending_label,
				current_lines_info,
				flat_toc_items,
				render_tables_inline,
				&mut images,
			);
		}
	}
	flush_block(&mut pending_label, &mut current_block, buffer, page_display_text, current_lines_info, &mut images);
	images.after_page(buffer, page_display_text, current_lines_info);
	true
}

struct ImagePlacement<'a> {
	mcid_tops: &'a HashMap<i32, f64>,
	unclaimed: UnclaimedImages,
	page_index: i32,
	figure_images: &'a HashMap<i32, String>,
	block_top: Option<f64>,
}

impl ImagePlacement<'_> {
	fn note(&mut self, mcid: i32) {
		if self.block_top.is_none() {
			self.block_top = self.mcid_tops.get(&mcid).copied();
		}
	}

	/// Overrides a height already held, unlike [`Self::note`], because it speaks for the element about to be written.
	fn note_element(&mut self, elem: &Tag) {
		if self.unclaimed.is_empty() {
			return;
		}
		if let Some(mcid) = first_marked_content_id(elem) {
			self.block_top = self.mcid_tops.get(&mcid).copied();
		}
	}

	fn before_line(
		&mut self,
		buffer: &mut DocumentBuffer,
		page_display_text: &mut String,
		current_lines_info: &mut Vec<(DocPos, String)>,
	) {
		if self.unclaimed.is_empty() {
			return;
		}
		let top = self.block_top.take().unwrap_or(f64::NEG_INFINITY);
		self.unclaimed.place_above(top, buffer, page_display_text, current_lines_info);
	}

	fn figure_reference(&self, elem: &Tag) -> String {
		find_image_path(elem, self.figure_images)
			.map_or_else(String::new, |path| format!("pdf:{}:{path}", self.page_index))
	}

	fn after_page(
		&mut self,
		buffer: &mut DocumentBuffer,
		page_display_text: &mut String,
		current_lines_info: &mut Vec<(DocPos, String)>,
	) {
		self.unclaimed.place_above(f64::NEG_INFINITY, buffer, page_display_text, current_lines_info);
	}
}

/// The whole subtree is searched because some makers nest a figure's picture a level or two down, in a `Div` or beside a `Caption`.
fn find_image_path(elem: &Tag, image_paths: &HashMap<i32, String>) -> Option<String> {
	let count = elem.child_count();
	for i in 0..count {
		if let Some(child) = elem.child(i) {
			if let Some(path) = find_image_path(&child, image_paths) {
				return Some(path);
			}
		} else if let Some(mcid) = elem.child_mcid(i)
			&& let Some(path) = image_paths.get(&mcid)
		{
			return Some(path.clone());
		}
	}
	None
}

/// Symbol and Wingdings bullets reach the text layer as private-use code points (U+F0B7 is the common one), which a screen reader says nothing for.
pub(super) fn normalize_list_label(label: &str) -> String {
	let is_private_use = |c: char| ('\u{E000}'..='\u{F8FF}').contains(&c);
	if !label.is_empty() && label.chars().all(is_private_use) { "\u{2022}".to_string() } else { label.to_string() }
}

pub(super) fn is_bare_list_label(text: &str) -> bool {
	let label = text.trim();
	if label.is_empty() || label.chars().count() > 10 {
		return false;
	}
	let is_bullet = |c: char| {
		matches!(c, '\u{2022}' | '\u{25E6}' | '\u{25AA}' | '\u{25CF}' | '\u{2013}' | '-' | '*')
			|| ('\u{E000}'..='\u{F8FF}').contains(&c)
	};
	if label.chars().all(is_bullet) {
		return true;
	}
	let Some(body) = label.strip_suffix('.').or_else(|| label.strip_suffix(')')) else { return false };
	let body = body.strip_prefix('(').unwrap_or(body);
	let numeric = !body.is_empty() && body.chars().all(|c| c.is_ascii_digit() || c == '.');
	let letter = body.chars().count() == 1 && body.chars().all(char::is_alphabetic);
	let roman = !body.is_empty() && body.len() <= 5 && body.chars().all(|c| "ivxlIVXL".contains(c));
	numeric || letter || roman
}

fn flush_block(
	pending_label: &mut String,
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(DocPos, String)>,
	images: &mut ImagePlacement,
) {
	let trimmed = collapse_leaders(&trim_string(&collapse_whitespace(current_block)));
	current_block.clear();
	if trimmed.is_empty() {
		// A label waits for the block carrying its item's text; an item with none writes it out at its end.
		return;
	}
	let line = if pending_label.is_empty() { trimmed } else { format!("{pending_label} {trimmed}") };
	pending_label.clear();
	images.before_line(buffer, page_display_text, current_lines_info);
	let offset = buffer.end();
	current_lines_info.push((offset, line.clone()));
	buffer.append(&line);
	buffer.append("\n");
	page_display_text.push_str(&line);
	page_display_text.push('\n');
}

fn flush_block_lines(
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(DocPos, String)>,
	images: &mut ImagePlacement,
) {
	let text = current_block.clone();
	current_block.clear();
	for line in text.split('\n') {
		let trimmed = trim_string(&collapse_whitespace(line));
		if !trimmed.is_empty() {
			images.before_line(buffer, page_display_text, current_lines_info);
			let offset = buffer.end();
			current_lines_info.push((offset, trimmed.clone()));
			buffer.append(&trimmed);
			buffer.append("\n");
			page_display_text.push_str(&trimmed);
			page_display_text.push('\n');
		}
	}
}

#[allow(clippy::too_many_arguments)]
fn process_struct_element(
	elem: &Tag,
	page_text: &mut PageText,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_block: &mut String,
	pending_label: &mut String,
	current_lines_info: &mut Vec<(DocPos, String)>,
	toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
	images: &mut ImagePlacement,
) {
	let elem_type = elem.kind().unwrap_or_default();
	if elem_type == "Lbl" {
		// Held back because the paragraph in the LBody beside it starts by flushing the block, which would leave the label as a line of its own with nothing to say.
		let mut label = String::new();
		collect_text(elem, page_text.text, &mut label);
		let label = trim_string(&collapse_whitespace(&label));
		if !label.is_empty() {
			images.note_element(elem);
			*pending_label = normalize_list_label(&label);
		}
		return;
	}
	if elem_type == "Table" {
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		let html = build_html_table(elem, page_text.text);
		images.note_element(elem);
		images.before_line(buffer, page_display_text, current_lines_info);
		let pos = buffer.end();
		append_pdf_table_to_buffer(buffer, html, pos, current_lines_info, page_display_text, render_tables_inline);
		return;
	}
	if elem_type == "Figure" {
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		let description = elem
			.alt_text()
			.or_else(|| elem.actual_text())
			.map(|text| trim_string(&collapse_whitespace(&text)))
			.unwrap_or_default();
		images.note_element(elem);
		images.before_line(buffer, page_display_text, current_lines_info);
		let reference = images.figure_reference(elem);
		append_image(buffer, MarkerType::Figure, &description, page_display_text, current_lines_info, &reference);
		// No return, so the figure's content is read too: a travel guide wraps page numbers, offers and addresses in figures whose /Alt describes the artwork, and letting /Alt stand in dropped 39 pieces of its text, links included.
	}
	let is_block = matches!(
		elem_type.as_str(),
		"P" | "H"
			| "H1"
			| "H2"
			| "H3"
			| "H4"
			| "H5"
			| "H6"
			| "L"
			| "LI"
			| "Div"
			| "Sect"
			| "Part"
			| "Art"
			| "TOC"
			| "TOCI"
			| "Code"
	);
	let preserve_lines = elem_type == "Code";
	if is_block {
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		// Images go in before the position is taken, or the heading and list markers below would point at an image line.
		images.note_element(elem);
		images.before_line(buffer, page_display_text, current_lines_info);
	}
	let block_start_pos = buffer.end() + DocLen::of(current_block);
	let count = elem.child_count();
	for i in 0..count {
		if let Some(child) = elem.child(i) {
			process_struct_element(
				&child,
				page_text,
				buffer,
				page_display_text,
				current_block,
				pending_label,
				current_lines_info,
				toc_items,
				render_tables_inline,
				images,
			);
		} else if let Some(mcid) = elem.child_mcid(i)
			&& let Some(text) = page_text.take(mcid)
		{
			images.note(mcid);
			// Word tags a list's number inside the item's body rather than as an Lbl, separated only by a tab stop, so "1." and "The input voltage" ran together.
			if is_bare_list_label(current_block) && text.chars().next().is_some_and(|c| !c.is_whitespace()) {
				current_block.push(' ');
			}
			current_block.push_str(text);
		}
	}
	if is_block {
		if preserve_lines {
			flush_block_lines(current_block, buffer, page_display_text, current_lines_info, images);
		} else {
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		}
		// A label with no text of its own goes out here, which also stops a stray label travelling past its enclosing block onto whatever comes next.
		if !pending_label.is_empty() {
			current_block.push_str(pending_label);
			pending_label.clear();
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		}
		let heading_level = match elem_type.as_str() {
			"H1" | "H" => Some(1),
			"H2" => Some(2),
			"H3" => Some(3),
			"H4" => Some(4),
			"H5" => Some(5),
			"H6" => Some(6),
			_ => None,
		};
		if let Some(level) = heading_level {
			let mut title = String::new();
			collect_text(elem, page_text.text, &mut title);
			let title = trim_string(&collapse_whitespace(&title));
			if !title.is_empty() {
				let marker_type = match level {
					1 => MarkerType::Heading1,
					2 => MarkerType::Heading2,
					3 => MarkerType::Heading3,
					4 => MarkerType::Heading4,
					5 => MarkerType::Heading5,
					_ => MarkerType::Heading6,
				};
				buffer.add_marker(Marker::new(marker_type, block_start_pos).with_text(title.clone()).with_level(level));
				toc_items
					.push((u32::try_from(level).unwrap_or(1), TocItem::new(title, String::new(), block_start_pos)));
			}
		}
		if elem_type == "L" || elem_type == "TOC" {
			let child_count = elem.child_count();
			buffer.add_marker(Marker::new(MarkerType::List, block_start_pos).with_level(child_count));
		}
		if elem_type == "LI" || elem_type == "TOCI" {
			let mut li_text = String::new();
			collect_text(elem, page_text.text, &mut li_text);
			let li_text = trim_string(&collapse_whitespace(&li_text));
			buffer.add_marker(Marker::new(MarkerType::ListItem, block_start_pos).with_text(li_text));
		}
	}
}

#[cfg(test)]
mod tests;
