use crate::{
	document::{DocumentBuffer, MarkerType, TocItem},
	types::HeadingInfo,
};

fn children_at_mut<'a>(toc: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = toc;
	for &idx in path {
		current = &mut current[idx].children;
	}
	current
}

pub fn build_toc_from_buffer(buffer: &DocumentBuffer) -> Vec<TocItem> {
	let headings: Vec<HeadingInfo> = buffer
		.markers
		.iter()
		.filter_map(|marker| {
			let level = marker_type_to_heading_level(marker.mtype);
			if level > 0 {
				Some(HeadingInfo { offset: marker.position, level, text: marker.text.clone() })
			} else {
				None
			}
		})
		.collect();
	build_toc_from_headings(&headings)
}

pub fn build_toc_from_headings(headings: &[HeadingInfo]) -> Vec<TocItem> {
	if headings.is_empty() {
		return Vec::new();
	}
	let mut toc = Vec::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut levels: Vec<i32> = Vec::new();
	for heading in headings {
		if heading.level <= 0 {
			continue;
		}
		while let Some(&last_level) = levels.last() {
			if last_level < heading.level {
				break;
			}
			stack.pop();
			levels.pop();
		}
		let item = TocItem::new(heading.text.clone(), String::new(), heading.offset);
		let siblings = children_at_mut(&mut toc, &stack);
		siblings.push(item);
		stack.push(siblings.len() - 1);
		levels.push(heading.level);
	}
	toc
}

pub const fn heading_level_to_marker_type(level: i32) -> MarkerType {
	match level {
		1 => MarkerType::Heading1,
		2 => MarkerType::Heading2,
		3 => MarkerType::Heading3,
		4 => MarkerType::Heading4,
		5 => MarkerType::Heading5,
		_ => MarkerType::Heading6,
	}
}

pub const fn marker_type_to_heading_level(marker_type: MarkerType) -> i32 {
	match marker_type {
		MarkerType::Heading1 => 1,
		MarkerType::Heading2 => 2,
		MarkerType::Heading3 => 3,
		MarkerType::Heading4 => 4,
		MarkerType::Heading5 => 5,
		MarkerType::Heading6 => 6,
		_ => 0,
	}
}
