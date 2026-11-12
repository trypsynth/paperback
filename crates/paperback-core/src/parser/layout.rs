//! What PDF and XPS share once a page is lines of text with their positions: reading order, running heads, paragraphs, headings and links.

mod line;
mod links;
mod paragraphs;
pub(in crate::parser) mod running;
mod toc;

pub(crate) use line::keeps_line_end_hyphen;
use line::median_line_font_size;
pub(in crate::parser) use line::{COORDINATE_EPSILON, Line, restore_reading_order, sorted_median};
pub(in crate::parser) use links::{PendingLink, place_links};
use paragraphs::join_paragraphs;
pub(in crate::parser) use paragraphs::split_lines;
pub(crate) use paragraphs::{join_wrapped_lines, join_wrapped_runs};
pub(in crate::parser) use running::RunningText;
use running::{EDGE_LINES, PageEdges};
pub(in crate::parser) use toc::build_toc_tree;

use crate::{
	document::{DocPos, DocumentBuffer, Marker, MarkerType, TocItem},
	parser::add_heading_markers,
};

/// Gives back the lines left once the running text is out, which the paragraphs' line indexes point into.
pub(in crate::parser) fn page_paragraphs(
	mut lines: Vec<Line>,
	running_text: &RunningText,
	join: bool,
) -> (Vec<Line>, Vec<(String, bool, usize)>) {
	// Measured before the running headers come out: they sit at or below body size, so dropping them would pull the median up past a heading only just above it.
	let body_size = median_line_font_size(&lines);
	strip_running_text(&mut lines, running_text);
	let paragraphs = if join { join_paragraphs(&lines, body_size) } else { split_lines(&lines, body_size) };
	(lines, paragraphs)
}

/// The survey always runs because it also measures body text size; `strip` only decides whether its answer is used.
pub(in crate::parser) fn survey_running_text<'a>(
	pages: impl Iterator<Item = &'a [Line]> + Clone,
	strip: bool,
) -> RunningText {
	let edges: Vec<PageEdges> = pages.clone().map(PageEdges::of).collect();
	// Taken over the whole document because a chapter opening is mostly heading.
	let body_font_size = median_line_font_size(pages.flatten());
	let running_text = running::detect(&edges, body_font_size);
	tracing::debug!(
		running_text_count = running_text.len(),
		body_font_size,
		"surveyed pages for running headers and footers"
	);
	if strip { running_text } else { running_text.recognizing_nothing() }
}

/// Only edge lines are considered, so a body line reading like a running head stays, and a page whose every line matches (a part title repeating the book's name) is never emptied.
fn strip_running_text(lines: &mut Vec<Line>, running_text: &RunningText) {
	let is_running = |index: usize, lines: &Vec<Line>| {
		let at_an_edge = index < EDGE_LINES || index + EDGE_LINES >= lines.len();
		at_an_edge && running_text.contains(&lines[index].text, lines[index].size)
	};
	let doomed: Vec<usize> = (0..lines.len()).filter(|index| is_running(*index, lines)).collect();
	if doomed.len() == lines.iter().filter(|line| !line.text.trim().is_empty()).count() {
		return;
	}
	for index in doomed.into_iter().rev() {
		lines.remove(index);
	}
}

/// Bookmarks win over structure-tree and font-size headings where they exist, because a person wrote them.
pub(in crate::parser) fn resolve_headings(
	buffer: &mut DocumentBuffer,
	toc_items: &mut Vec<TocItem>,
	flat_toc_items: Vec<(u32, TocItem)>,
	detected: Vec<(DocPos, String)>,
	any_tags_processed: bool,
) {
	if any_tags_processed {
		if toc_items.is_empty() {
			*toc_items = build_toc_tree(flat_toc_items);
		} else if flat_toc_items.is_empty() {
			add_heading_markers(buffer, toc_items, 1);
		}
	} else if toc_items.is_empty() && !detected.is_empty() {
		for (pos, text) in &detected {
			buffer.add_marker(Marker::new(MarkerType::Heading1, *pos).with_text(text.clone()).with_level(1));
		}
		*toc_items = detected.into_iter().map(|(pos, text)| TocItem::new(text, String::new(), pos)).collect();
	} else {
		add_heading_markers(buffer, toc_items, 1);
	}
}
