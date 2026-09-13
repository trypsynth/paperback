//! MOBI/KF8 NCX table-of-contents parsing: reads the book's NCX index through
//! [`super::index`], which walks Mobipocket's generic tagged-index format, and turns its
//! entries into a hierarchical table of contents.

use std::collections::HashMap;

use super::index::{IndexEntry, read_index};
use crate::document::TocItem;

/// Tag giving an entry's position when it has no fragment of its own.
const TAG_POSITION: u8 = 1;

/// Tag giving the offset of an entry's title in the `CNCX` blob.
const TAG_TITLE: u8 = 3;

/// Tag giving an entry's depth in the contents tree.
const TAG_LEVEL: u8 = 4;

/// Tag giving a fragment number and a position within it, in that order.
const TAG_FRAGMENT: u8 = 6;

/// An entry's first value for `tag`, when it carries one.
fn value(entry: &IndexEntry, tag: u8) -> Option<usize> {
	entry.tags.get(&tag)?.first().copied()
}

pub(super) fn parse_ncx(
	data: &[u8],
	records: &[usize],
	mobi_header: &[u8],
	exth: &HashMap<u32, Vec<u8>>,
	is_kf8: bool,
	frag_offsets: &HashMap<usize, usize>,
) -> Vec<TocItem> {
	let mut ncx_index = 0xFFFF_FFFF;
	if is_kf8 && mobi_header.len() >= 232 {
		ncx_index = u32::from_be_bytes(mobi_header[228..232].try_into().unwrap_or([0; 4])) as usize;
	} else if !is_kf8 && mobi_header.len() >= 248 {
		ncx_index = u32::from_be_bytes(mobi_header[244..248].try_into().unwrap_or([0; 4])) as usize;
	}
	if (ncx_index == 0xFFFF_FFFF || ncx_index == 0)
		&& let Some(ext) = exth.get(&253)
		&& ext.len() >= 4
	{
		ncx_index = u32::from_be_bytes([ext[0], ext[1], ext[2], ext[3]]) as usize;
	}
	if ncx_index == 0xFFFF_FFFF || ncx_index == 0 || ncx_index >= records.len() - 1 {
		return Vec::new();
	}
	let Some(index) = read_index(data, records, ncx_index) else { return Vec::new() };
	let mut entries = Vec::new();
	for entry in &index.entries {
		// Tag 1 gives a position on its own; tag 6 gives a fragment and a position together
		// and wins where both are present, which is how KF8 books point into a fragment.
		let mut position = value(entry, TAG_POSITION);
		let mut fragment = None;
		if let Some(values) = entry.tags.get(&TAG_FRAGMENT) {
			fragment = values.first().copied();
			if let Some(&within) = values.get(1) {
				position = Some(within);
			}
		}
		let (Some(title_offset), Some(position)) = (value(entry, TAG_TITLE), position) else { continue };
		let Some(title) = index.string_at(title_offset) else { continue };
		// A fragment's own start has to be added back, since the position is measured from
		// inside it rather than from the top of the book.
		let filepos = frag_offsets.get(&fragment.unwrap_or(0)).copied().unwrap_or(0) + position;
		// An entry with no depth of its own sits at the top level.
		let level = match value(entry, TAG_LEVEL) {
			Some(0) | None => 1,
			Some(level) => u32::try_from(level).unwrap_or(1),
		};
		entries.push((title, level, format!("#fp{filepos:010}")));
	}
	let mut toc: Vec<TocItem> = Vec::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut levels: Vec<u32> = Vec::new();
	for (title, level, reference) in entries {
		if level == 0 {
			continue;
		}
		while let Some(&last_level) = levels.last() {
			if last_level < level {
				break;
			}
			stack.pop();
			levels.pop();
		}
		let item = TocItem::new(title, reference, 0);
		let mut current = &mut toc;
		for &idx in &stack {
			current = &mut current[idx].children;
		}
		current.push(item);
		stack.push(current.len() - 1);
		levels.push(level);
	}
	toc
}
