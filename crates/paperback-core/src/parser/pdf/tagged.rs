use std::collections::HashSet;

use super::running::{EDGE_LINES, RunningText};
use crate::document::{DocLen, DocPos, DocumentBuffer, TocItem};

pub(super) struct TaggedPage {
	pub buffer: DocumentBuffer,
	pub display_text: String,
	pub lines_info: Vec<(DocPos, String)>,
	pub toc_items: Vec<(u32, TocItem)>,
}

pub(super) fn append_tagged_page(buffer: &mut DocumentBuffer, page: DocumentBuffer, page_start_offset: DocPos) {
	buffer.append(&page.content);
	for mut marker in page.markers {
		marker.position += page_start_offset.from_start();
		buffer.add_marker(marker);
	}
}

/// A line carrying a marker is never a candidate: it is a heading, list item, table or figure, and a document opening a dozen pages with a figure would lose them all.
pub(super) fn tagged_running_lines(page: &TaggedPage, running_text: &RunningText) -> HashSet<usize> {
	let count = page.lines_info.len();
	let marked: HashSet<DocPos> = page.buffer.markers.iter().map(|marker| marker.position).collect();
	let at_an_edge = |index: usize| index < EDGE_LINES || index + EDGE_LINES >= count;
	let doomed: HashSet<usize> = (0..count)
		.filter(|index| at_an_edge(*index))
		.filter(|index| {
			let (position, text) = &page.lines_info[*index];
			// The tags say outright which lines are headings, so no size is needed.
			!marked.contains(position) && running_text.contains(text, 0.0)
		})
		.collect();
	if doomed.len() == count { HashSet::new() } else { doomed }
}

pub(super) fn without_lines(page: TaggedPage, doomed: &HashSet<usize>) -> TaggedPage {
	if doomed.is_empty() {
		return page;
	}
	let mut rebuilt = TaggedPage {
		buffer: DocumentBuffer::new(),
		display_text: String::new(),
		lines_info: Vec::new(),
		toc_items: Vec::new(),
	};
	let mut moved: Vec<(DocPos, DocLen, Option<DocPos>)> = Vec::with_capacity(page.lines_info.len());
	for (index, (position, text)) in page.lines_info.iter().enumerate() {
		let length = DocLen::of(text) + DocLen::ONE;
		if doomed.contains(&index) {
			moved.push((*position, length, None));
			continue;
		}
		let start = rebuilt.buffer.end();
		moved.push((*position, length, Some(start)));
		rebuilt.lines_info.push((start, text.clone()));
		rebuilt.buffer.append(text);
		rebuilt.buffer.append("\n");
		rebuilt.display_text.push_str(text);
		rebuilt.display_text.push('\n');
	}
	// A marker placed at the end of a page sits past the last line and keeps standing at the end.
	let end = rebuilt.buffer.end();
	let move_position = |position: DocPos| {
		moved
			.iter()
			.find(|(start, length, _)| position >= *start && position < *start + *length)
			.map_or(Some(end), |(start, _, moved_to)| moved_to.map(|moved_to| moved_to + (position - *start)))
	};
	for mut marker in page.buffer.markers {
		if let Some(position) = move_position(marker.position) {
			marker.position = position;
			rebuilt.buffer.add_marker(marker);
		}
	}
	rebuilt.toc_items = page
		.toc_items
		.into_iter()
		.filter_map(|(level, mut item)| {
			item.offset = move_position(item.offset)?;
			Some((level, item))
		})
		.collect();
	rebuilt
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use super::{TaggedPage, tagged_running_lines, without_lines};
	use crate::{
		document::{DocumentBuffer, Marker, MarkerType, TocItem},
		parser::pdf::running,
	};

	fn tagged_page(lines: &[&str], marked: &[usize]) -> TaggedPage {
		let mut page = TaggedPage {
			buffer: DocumentBuffer::new(),
			display_text: String::new(),
			lines_info: Vec::new(),
			toc_items: Vec::new(),
		};
		for (index, text) in lines.iter().enumerate() {
			let position = page.buffer.end();
			if marked.contains(&index) {
				page.buffer.add_marker(Marker::new(MarkerType::Heading1, position).with_text((*text).to_string()));
				page.toc_items.push((1, TocItem::new((*text).to_string(), String::new(), position)));
			}
			page.lines_info.push((position, (*text).to_string()));
			page.buffer.append(text);
			page.buffer.append("\n");
			page.display_text.push_str(text);
			page.display_text.push('\n');
		}
		page
	}

	fn running_text() -> running::RunningText {
		let bodies = ["Salt.", "Pepper.", "Thyme.", "Parsley.", "Sage."];
		let edges: Vec<running::PageEdges> = (1..=5)
			.map(|page| {
				running::PageEdges::of_texts(&[
					"The Book Of Tests".to_string(),
					bodies[page - 1].to_string(),
					format!("{page}"),
				])
			})
			.collect();
		running::detect_tagged(&edges)
	}

	#[test]
	fn a_tagged_page_loses_its_running_head_and_page_number() {
		let page = tagged_page(&["The Book Of Tests", "Body of page 6.", "6"], &[]);
		let doomed = tagged_running_lines(&page, &running_text());
		assert_eq!(doomed, HashSet::from([0, 2]));
		let rebuilt = without_lines(page, &doomed);
		assert_eq!(rebuilt.buffer.content, "Body of page 6.\n");
		assert_eq!(rebuilt.lines_info, vec![(0.into(), "Body of page 6.".to_string())]);
		assert_eq!(rebuilt.display_text, "Body of page 6.\n");
	}

	#[test]
	fn a_line_the_tags_speak_for_is_never_furniture() {
		let page = tagged_page(&["The Book Of Tests", "Rosemary.", "6"], &[0]);
		assert_eq!(tagged_running_lines(&page, &running_text()), HashSet::from([2]));
	}

	#[test]
	fn a_page_of_nothing_but_furniture_keeps_it() {
		let page = tagged_page(&["The Book Of Tests", "6"], &[]);
		assert!(tagged_running_lines(&page, &running_text()).is_empty(), "a page is never emptied");
	}

	#[test]
	fn a_line_repeated_on_a_few_pages_only_is_not_furniture() {
		// Four of twenty pages would be enough untagged; words rather than numbers because the signature blanks digits.
		let word = |page: usize| format!("{}", char::from(b'a' + u8::try_from(page).expect("a small page number")));
		let edges: Vec<running::PageEdges> = (0..20)
			.map(|page| {
				let opening = if page < 4 {
					"The following applies.".to_string()
				} else {
					format!("Opening {} here.", word(page))
				};
				running::PageEdges::of_texts(&[opening, format!("Closing {} here.", word(page))])
			})
			.collect();
		let detected = running::detect_tagged(&edges);
		let page = tagged_page(&["The following applies.", "Closing a here."], &[]);
		assert!(tagged_running_lines(&page, &detected).is_empty());
	}

	#[test]
	fn markers_and_toc_items_move_up_with_the_lines_they_sit_on() {
		let page = tagged_page(&["Furniture", "Chapter One", "Body."], &[1]);
		let rebuilt = without_lines(page, &HashSet::from([0]));
		assert_eq!(rebuilt.buffer.content, "Chapter One\nBody.\n");
		let heading = rebuilt.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Heading1);
		assert_eq!(heading.expect("the heading survives").position, 0, "it moved up by the dropped line");
		assert_eq!(rebuilt.toc_items[0].1.offset, 0);
	}

	#[test]
	fn a_marker_inside_a_dropped_line_goes_with_it() {
		let page = tagged_page(&["Furniture", "Body."], &[0]);
		let rebuilt = without_lines(page, &HashSet::from([0]));
		assert!(rebuilt.buffer.markers.is_empty(), "the dropped line's heading is dropped too");
		assert!(rebuilt.toc_items.is_empty());
	}
}
