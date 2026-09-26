//! Extracting text from a PDF page via its tagged structure tree, when one is present and
//! trustworthy: walking the tree's `P`/`H1`-`H6`/`L`/`Table`/etc. elements and resolving each
//! leaf's marked-content id to the text pdfium associated with it. This is the preferred
//! extraction path: it recovers real paragraph/heading/list/table structure that the
//! plain-text fallback ([`super::text`]) can only guess at from font size and line shape.
//!
//! This module is the walk. Reading the page's glyphs into the ids the tree names them by is
//! [`marked_content`], and turning a `Table` element into something the reader can open is
//! [`tables`].
//!
//! Some PDFs advertise a structure tree while leaving most of their text untagged, though, so
//! [`extract_tagged_page_text`] first checks how much of the page's text is actually covered
//! by a marked-content id and bails out to the caller's plain-text fallback below
//! [`MIN_MCID_COVERAGE`]. It bails out the same way when the tree it is handed leads to no text
//! at all, which is what a file pdfium cannot read the structure of is left looking like once
//! [`super::repair`] has had its chance at it.

use std::collections::HashMap;

use self::{
	marked_content::{MIN_MCID_COVERAGE, PageText, TreeFacts, collect_text, first_marked_content_id},
	tables::{append_pdf_table_to_buffer, build_html_table},
};
use super::images::{UnclaimedImages, append_image};
use crate::{
	document::{DocumentBuffer, Marker, MarkerType, TocItem},
	pdfium::{PdfPage, PdfTextPage, Tag},
	util::text::{collapse_whitespace, display_len, trim_string},
};

mod marked_content;
mod tables;

/// Attempts tagged extraction for one page: builds the marked-content-id → text map, checks
/// its coverage against [`MIN_MCID_COVERAGE`], and if trusted walks the structure tree into
/// `buffer`/`page_display_text`/`current_lines_info`/`flat_toc_items`. Returns whether tagged
/// extraction was actually used; the caller falls back to plain-text extraction when it isn't.
///
/// `image_tops` is where the page draws its images, used only when the tree claims none of them;
/// see [`UnclaimedImages`].
#[allow(clippy::too_many_arguments)]
pub(super) fn extract_tagged_page_text(
	page: &PdfPage,
	text_page: &PdfTextPage,
	page_index: i32,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	flat_toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
	image_tops: &[f64],
) -> bool {
	let Some(struct_tree) = page.tags() else { return false };
	let child_count = struct_tree.child_count();
	if child_count == 0 {
		return false;
	}
	let facts = TreeFacts::of(&struct_tree);
	// A tree that names its figures is trusted with them, and the page's own image objects are
	// then left alone: a `Figure` is not always one drawn image, so counting both would announce
	// some of them twice.
	let unclaimed = UnclaimedImages::new(if facts.claims_figures { &[] } else { image_tops });
	let content = marked_content::read(text_page, &facts, !unclaimed.is_empty());
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
	// pdfium hands back a null child for a top-level element it cannot load, so a page whose only
	// element is one of those walks to nothing at all. That is what a missing parent tree looks
	// like: every glyph carries a marked-content id, the tree reports one child, and that child
	// cannot be reached. [`super::repair`] gets such a file its tree back where it can; where it
	// cannot, reporting the tagged path as used would hand the caller an empty page and stop
	// plain extraction from ever running over text that is there.
	if content.text.is_empty() || (0..child_count).all(|i| struct_tree.child(i).is_none()) {
		tracing::warn!(page_index, "page structure tree leads to no text, falling back to plain extraction");
		return false;
	}
	let mut current_block = String::new();
	let mut pending_label = String::new();
	let mut page_text = PageText::new(&content.text);
	let mut images = ImagePlacement { mcid_tops: &content.tops, unclaimed, block_top: None };
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

/// Where a page's own images go when its structure tree claims none of them.
///
/// The walk writes one block at a time and each block's height comes from the first piece of text
/// in it, so [`Self::note`] is called as text joins a block and [`Self::before_line`] just before
/// the block goes out. For a page whose tree does claim its figures there is nothing here to do
/// and every call returns at once.
struct ImagePlacement<'a> {
	mcid_tops: &'a HashMap<i32, f64>,
	unclaimed: UnclaimedImages,
	/// The top edge of the block being built, from the first piece of text in it.
	block_top: Option<f64>,
}

impl ImagePlacement<'_> {
	/// Note a piece of text joining the block being built. The first one sets the block's height;
	/// everything after it is lower down the same block.
	fn note(&mut self, mcid: i32) {
		if self.block_top.is_none() {
			self.block_top = self.mcid_tops.get(&mcid).copied();
		}
	}

	/// Note the height of a whole element, which is where its own first piece of text sits. Unlike
	/// [`Self::note`] this speaks for the element that is about to be written rather than for text
	/// joining a block already under way, so it sets the height rather than deferring to one
	/// already held.
	fn note_element(&mut self, elem: &Tag) {
		if self.unclaimed.is_empty() {
			return;
		}
		if let Some(mcid) = first_marked_content_id(elem) {
			self.block_top = self.mcid_tops.get(&mcid).copied();
		}
	}

	/// Write every image drawn above the line that is about to go out, and forget the height ready
	/// for the next block.
	fn before_line(
		&mut self,
		buffer: &mut DocumentBuffer,
		page_display_text: &mut String,
		current_lines_info: &mut Vec<(usize, String)>,
	) {
		if self.unclaimed.is_empty() {
			return;
		}
		let top = self.block_top.take().unwrap_or(f64::NEG_INFINITY);
		self.unclaimed.place_above(top, buffer, page_display_text, current_lines_info);
	}

	/// Write the images left below everything the page wrote.
	fn after_page(
		&mut self,
		buffer: &mut DocumentBuffer,
		page_display_text: &mut String,
		current_lines_info: &mut Vec<(usize, String)>,
	) {
		self.unclaimed.place_above(f64::NEG_INFINITY, buffer, page_display_text, current_lines_info);
	}
}

/// Replaces a list label made only of private-use characters with a plain bullet.
///
/// The Symbol and Wingdings bullets reach the text layer as private-use code points (U+F0B7 for
/// the Symbol font's, to name the common one). They mean nothing outside the font that drew them,
/// and a screen reader says nothing at all when it meets one. A label that is only those stands
/// in for a bullet, so it is written as one.
pub(super) fn normalize_list_label(label: &str) -> String {
	let is_private_use = |c: char| ('\u{E000}'..='\u{F8FF}').contains(&c);
	if !label.is_empty() && label.chars().all(is_private_use) { "\u{2022}".to_string() } else { label.to_string() }
}

fn flush_block(
	pending_label: &mut String,
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	images: &mut ImagePlacement,
) {
	let trimmed = trim_string(&collapse_whitespace(current_block));
	current_block.clear();
	if trimmed.is_empty() {
		// A label waits here for the block that carries its item's text. An item that never gets
		// any text writes the label out itself, at the end of the item.
		return;
	}
	let line = if pending_label.is_empty() { trimmed } else { format!("{pending_label} {trimmed}") };
	pending_label.clear();
	images.before_line(buffer, page_display_text, current_lines_info);
	let offset = buffer.current_position();
	current_lines_info.push((offset, line.clone()));
	buffer.append(&line);
	buffer.append("\n");
	page_display_text.push_str(&line);
	page_display_text.push('\n');
}

/// Like `flush_block`, but preserves line breaks within the content instead of
/// collapsing them. Used for preformatted elements like `Code`.
fn flush_block_lines(
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	images: &mut ImagePlacement,
) {
	let text = current_block.clone();
	current_block.clear();
	for line in text.split('\n') {
		let trimmed = trim_string(&collapse_whitespace(line));
		if !trimmed.is_empty() {
			images.before_line(buffer, page_display_text, current_lines_info);
			let offset = buffer.current_position();
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
	current_lines_info: &mut Vec<(usize, String)>,
	toc_items: &mut Vec<(u32, TocItem)>,
	render_tables_inline: bool,
	images: &mut ImagePlacement,
) {
	let elem_type = elem.kind().unwrap_or_default();
	if elem_type == "Lbl" {
		// A list item's bullet or number. It is held back rather than added to the block being
		// built, because the paragraph inside the LBody beside it starts by flushing that block:
		// the label would go out as a line of its own, which arrowing through the item meets as a
		// stop with nothing to say.
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
		let pos = buffer.current_position();
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
		append_image(buffer, MarkerType::Figure, &description, page_display_text, current_lines_info);
		// The description goes in beside what the figure draws rather than in place of it, which
		// is what a PDF reader following /Alt to the letter would do. A figure is not always only
		// a picture: a travel guide measured for this wraps its page numbers, its offers and the
		// addresses that go with them in figures whose /Alt describes the artwork behind the
		// words, and letting the description stand in for them dropped 39 pieces of its own
		// text, links included. Nothing is returned here, so the walk below reads the figure's
		// content like any other container's.
	}
	let is_block = matches!(
		elem_type.as_str(),
		"P" | "H"
			| "H1" | "H2"
			| "H3" | "H4"
			| "H5" | "H6"
			| "L" | "LI"
			| "Div" | "Sect"
			| "Part" | "Art"
			| "TOC" | "TOCI"
			| "Code"
	);
	let preserve_lines = elem_type == "Code";
	if is_block {
		flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		// Whatever the page drew above this element goes in before its position is taken, or the
		// heading, list and table markers below would point at an image line instead of at the
		// text they name.
		images.note_element(elem);
		images.before_line(buffer, page_display_text, current_lines_info);
	}
	let block_start_pos = buffer.current_position() + display_len(current_block);
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
			current_block.push_str(text);
		}
	}
	if is_block {
		if preserve_lines {
			flush_block_lines(current_block, buffer, page_display_text, current_lines_info, images);
		} else {
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		}
		// An item with a label and no text of its own: the label is all there is to show, so it
		// goes out on its own here rather than onto the front of whatever comes next. Writing it
		// at the end of whichever block encloses it also stops a stray label, in a document that
		// puts one somewhere no list item follows, from travelling any further than that.
		if !pending_label.is_empty() {
			current_block.push_str(pending_label);
			pending_label.clear();
			flush_block(pending_label, current_block, buffer, page_display_text, current_lines_info, images);
		}
		let heading_level = match elem_type.as_str() {
			"H1" | "H" => Some(1), // "H" is a fallback generic heading, treated as H1
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
				toc_items.push((level as u32, TocItem::new(title, String::new(), block_start_pos)));
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
