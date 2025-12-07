use regex::RegexBuilder;

use crate::{
	bridge::ffi,
	document::{DocumentHandle, MarkerType},
	html_to_text::HeadingInfo,
};

fn heading_info(doc: &DocumentHandle, idx: i32) -> Option<HeadingInfo> {
	doc.heading_info(idx)
}

fn select_marker_index(
	doc: &DocumentHandle,
	position: i64,
	wrap: bool,
	direction: ffi::NavDirection,
	kind: MarkerType,
) -> (Option<usize>, bool) {
	let initial = match direction {
		ffi::NavDirection::Next => doc.next_marker_index(position, kind),
		ffi::NavDirection::Previous => doc.previous_marker_index(position, kind),
		_ => None,
	};
	if initial.is_some() {
		return (initial, false);
	}
	if !wrap {
		return (None, false);
	}
	let alt_pos = match direction {
		ffi::NavDirection::Next => -1,
		ffi::NavDirection::Previous => i64::try_from(doc.document().buffer.content.len()).unwrap_or(0) + 1,
		_ => -1,
	};
	(
		match direction {
			ffi::NavDirection::Next => doc.next_marker_index(alt_pos, kind),
			ffi::NavDirection::Previous => doc.previous_marker_index(alt_pos, kind),
			_ => None,
		},
		true,
	)
}

const fn build_nav_result(found: bool, wrapped: bool, offset: usize, level: i32, text: String) -> ffi::NavResult {
	ffi::NavResult { found, wrapped, offset, marker_level: level, marker_text: text }
}

pub fn reader_navigate(doc: &DocumentHandle, req: &ffi::NavRequest) -> ffi::NavResult {
	use ffi::NavTarget;
	match req.target {
		NavTarget::Section => {
			let (idx_opt, wrapped) =
				select_marker_index(doc, req.position, req.wrap, req.direction, MarkerType::SectionBreak);
			if let Some(idx) = idx_opt {
				let offset = doc.marker_position(idx as i32).unwrap_or(0);
				return build_nav_result(true, wrapped, offset, 0, String::new());
			}
			build_nav_result(false, wrapped, 0, 0, String::new())
		}
		NavTarget::Page => {
			let (idx_opt, wrapped) =
				select_marker_index(doc, req.position, req.wrap, req.direction, MarkerType::PageBreak);
			if let Some(idx) = idx_opt {
				let offset = doc.marker_position(idx as i32).unwrap_or(0);
				return build_nav_result(true, wrapped, offset, 0, String::new());
			}
			build_nav_result(false, wrapped, 0, 0, String::new())
		}
		NavTarget::Heading => {
			let level_filter = if req.level_filter > 0 { Some(req.level_filter) } else { None };
			let (idx_opt, wrapped) = match req.direction {
				ffi::NavDirection::Next => doc.next_heading_index(req.position, level_filter),
				ffi::NavDirection::Previous => doc.previous_heading_index(req.position, level_filter),
				_ => None,
			}
			.map_or((None, false), |idx| (Some(idx as usize), false));
			let (idx_final, wrapped_final) = if idx_opt.is_none() && req.wrap {
				let alt_pos = match req.direction {
					ffi::NavDirection::Next => -1,
					ffi::NavDirection::Previous => i64::try_from(doc.document().buffer.content.len()).unwrap_or(0) + 1,
					_ => -1,
				};
				let retry = match req.direction {
					ffi::NavDirection::Next => doc.next_heading_index(alt_pos, level_filter),
					ffi::NavDirection::Previous => doc.previous_heading_index(alt_pos, level_filter),
					_ => None,
				};
				(retry.map(|i| i as usize), retry.is_some())
			} else {
				(idx_opt, wrapped)
			};
			if let Some(idx) = idx_final {
				let offset = doc.marker_position(idx as i32).unwrap_or(0);
				let (level, text) = doc
					.document()
					.buffer
					.markers
					.get(idx)
					.map(|m| (m.level, m.text.clone()))
					.unwrap_or_else(|| {
						heading_info(doc, idx as i32)
							.map(|h| (h.level, h.text))
							.unwrap_or((0, String::new()))
					});
				return build_nav_result(true, wrapped_final, offset, level, text);
			}
			build_nav_result(false, wrapped_final, 0, 0, String::new())
		}
		NavTarget::List | NavTarget::ListItem => {
			let kind = if req.target == NavTarget::List { MarkerType::List } else { MarkerType::ListItem };
			let (idx_opt, wrapped) = select_marker_index(doc, req.position, req.wrap, req.direction, kind);
			if let Some(idx) = idx_opt {
				let marker = doc.document().buffer.markers.get(idx);
				let offset = marker.map_or(0, |m| m.position);
				let level = marker.map_or(0, |m| m.level);
				let text = marker.map(|m| m.text.clone()).unwrap_or_default();
				return build_nav_result(true, wrapped, offset, level, text);
			}
			build_nav_result(false, wrapped, 0, 0, String::new())
		}
		_ => build_nav_result(false, false, 0, 0, String::new()),
	}
}

pub fn reader_search(
	haystack: &str,
	needle: &str,
	start: i64,
	forward: bool,
	match_case: bool,
	whole_word: bool,
	regex: bool,
) -> i64 {
	if needle.is_empty() {
		return -1;
	}
	let start_utf16 = start.clamp(0, i64::MAX) as usize;
	let haystack = if match_case { haystack.to_string() } else { haystack.to_lowercase() };
	let needle = if match_case { needle.to_string() } else { needle.to_lowercase() };
	let utf16_to_byte_index = |s: &str, utf16_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (byte_idx, ch) in s.char_indices() {
			let len16 = ch.len_utf16();
			if utf16_count >= utf16_idx {
				return byte_idx;
			}
			utf16_count += len16;
		}
		s.len()
	};
	let byte_to_utf16_index = |s: &str, byte_idx: usize| -> usize {
		let mut utf16_count = 0usize;
		for (idx, ch) in s.char_indices() {
			if idx >= byte_idx {
				break;
			}
			utf16_count += ch.len_utf16();
		}
		utf16_count
	};
	let start_byte = utf16_to_byte_index(&haystack, start_utf16);
	if regex {
		let mut pattern = needle;
		if whole_word {
			pattern = format!(r"\b{pattern}\b");
		}
		let mut builder = RegexBuilder::new(&pattern);
		if !match_case {
			builder.case_insensitive(true);
		}
		let re = match builder.build() {
			Ok(r) => r,
			Err(_) => return -1,
		};
		if forward {
			if let Some(m) = re.find(&haystack[start_byte..]) {
				let byte_pos = start_byte + m.start();
				let utf16_pos = byte_to_utf16_index(&haystack, byte_pos);
				return utf16_pos as i64;
			}
		} else {
			let mut last: Option<usize> = None;
			let end_byte = start_byte.min(haystack.len());
			for m in re.find_iter(&haystack[..end_byte]) {
				last = Some(m.start());
			}
			if let Some(pos) = last {
				let utf16_pos = byte_to_utf16_index(&haystack, pos);
				return utf16_pos as i64;
			}
		}
		return -1;
	}
	let check_whole_word = |base: &str, pos: usize, len: usize| -> bool {
		let prev = if pos == 0 { None } else { base[..pos].chars().last() };
		let next = base[pos + len..].chars().next();
		let boundary_before = prev.is_none_or(|c| !c.is_alphanumeric());
		let boundary_after = next.is_none_or(|c| !c.is_alphanumeric());
		boundary_before && boundary_after
	};
	if forward {
		let slice = &haystack[start_byte..];
		if let Some(idx) = slice.find(&needle) {
			let global = start_byte + idx;
			if !whole_word || check_whole_word(&haystack, global, needle.len()) {
				let utf16_pos = byte_to_utf16_index(&haystack, global);
				return utf16_pos as i64;
			}
			// Keep searching forward for the next occurrence that matches whole word
			let mut cursor = global + 1;
			while cursor <= haystack.len() {
				if let Some(next_idx) = haystack[cursor..].find(&needle) {
					let candidate = cursor + next_idx;
					if !whole_word || check_whole_word(&haystack, candidate, needle.len()) {
						let utf16_pos = byte_to_utf16_index(&haystack, candidate);
						return utf16_pos as i64;
					}
					cursor = candidate + 1;
				} else {
					break;
				}
			}
		}
	} else {
		let slice = &haystack[..start_byte];
		let mut last = None;
		let mut search_start = 0;
		while let Some(idx) = slice[search_start..].find(&needle) {
			let candidate = search_start + idx;
			if !whole_word || check_whole_word(&haystack, candidate, needle.len()) {
				last = Some(candidate);
			}
			search_start = candidate + 1;
		}
		if let Some(pos) = last {
			let utf16_pos = byte_to_utf16_index(&haystack, pos);
			return utf16_pos as i64;
		}
	}
	-1
}
