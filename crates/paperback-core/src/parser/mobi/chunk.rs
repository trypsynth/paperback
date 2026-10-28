//! MOBI decodes the whole book into one HTML string, and one `HtmlToText::convert` over it built a DOM the size of the book, running huge books out of memory (#781).

use crate::parser::convert::block_elements::is_block_element;

/// Large enough that ordinary books stay one or a few chunks, small enough that a multi-hundred-megabyte book's peak DOM stays bounded.
const TARGET_CHUNK_BYTES: usize = 4 * 1024 * 1024;

const VOID_ELEMENTS: &[&str] =
	&["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];

pub(super) fn split_html_chunks(html: &str) -> Vec<&str> {
	split_html_chunks_with_target(html, TARGET_CHUNK_BYTES)
}

/// Cuts only after a block element closes back at the chunk's starting depth, a point already a line break in the output, so each fresh `HtmlToText` matches what one uninterrupted conversion would do there.
/// With no safe split point it falls back to larger chunks, down to one; chunking only bounds memory, so that is never wrong.
fn split_html_chunks_with_target(html: &str, target_bytes: usize) -> Vec<&str> {
	if html.len() <= target_bytes {
		return vec![html];
	}
	let bytes = html.as_bytes();
	let mut chunks = Vec::new();
	let mut chunk_start = 0usize;
	let mut depth_at_chunk_start = 0i32;
	let mut depth = 0i32;
	let mut in_tag = false;
	let mut tag_start = 0usize;
	let mut quote: Option<u8> = None;
	// `None` once consumed by a split, until the next tag closes.
	let mut last_safe_split: Option<(usize, i32)> = None;
	let mut i = 0usize;
	while i < bytes.len() {
		let b = bytes[i];
		if in_tag {
			if let Some(q) = quote {
				if b == q {
					quote = None;
				}
			} else if b == b'"' || b == b'\'' {
				quote = Some(b);
			} else if b == b'>' {
				in_tag = false;
				let tag_str = &html[tag_start + 1..i];
				if !tag_str.starts_with('!') && !tag_str.starts_with('?') {
					let (is_close, name_str) = tag_str.strip_prefix('/').map_or((false, tag_str), |rest| (true, rest));
					let name_end = name_str.find(|c: char| c.is_whitespace() || c == '/').unwrap_or(name_str.len());
					let name = name_str[..name_end].to_ascii_lowercase();
					if is_close {
						depth = (depth - 1).max(0);
					} else {
						let self_closing = tag_str.trim_end().ends_with('/') || VOID_ELEMENTS.contains(&name.as_str());
						if !self_closing {
							depth += 1;
						}
					}
					if depth <= depth_at_chunk_start && is_block_element(&name) {
						last_safe_split = Some((i + 1, depth));
					}
				}
			}
		} else if b == b'<' {
			in_tag = true;
			tag_start = i;
		}
		i += 1;
		if i - chunk_start >= target_bytes
			&& let Some((split_at, depth_at_split)) = last_safe_split
			&& split_at > chunk_start
		{
			chunks.push(&html[chunk_start..split_at]);
			chunk_start = split_at;
			depth_at_chunk_start = depth_at_split;
			last_safe_split = None;
		}
	}
	if chunk_start < html.len() {
		chunks.push(&html[chunk_start..]);
	}
	chunks
}

#[cfg(test)]
mod tests {
	use super::split_html_chunks_with_target;

	#[test]
	fn short_content_is_a_single_chunk() {
		let html = "<p>Hello</p>";
		assert_eq!(split_html_chunks_with_target(html, 1000), vec![html]);
	}

	#[test]
	fn splits_between_sibling_paragraphs_and_preserves_all_content() {
		let html = "<p>one</p><p>two</p><p>three</p><p>four</p>";
		let chunks = split_html_chunks_with_target(html, 12);
		assert!(chunks.len() > 1, "expected more than one chunk, got {chunks:?}");
		assert_eq!(chunks.concat(), html);
		for chunk in &chunks {
			assert!(!chunk.is_empty());
		}
	}

	#[test]
	fn never_splits_inside_a_tag_or_a_quoted_attribute() {
		let html = r#"<p data-x="a>b<c">one</p><p>two</p><p>three</p>"#;
		let chunks = split_html_chunks_with_target(html, 10);
		assert_eq!(chunks.concat(), html);
		for chunk in &chunks {
			assert_eq!(chunk.matches('<').count(), chunk.matches('>').count());
		}
	}

	#[test]
	fn falls_back_to_one_chunk_when_no_safe_split_exists() {
		let html = "<p>one two three four five six seven eight nine ten</p>";
		let chunks = split_html_chunks_with_target(html, 10);
		assert_eq!(chunks, vec![html]);
	}

	#[test]
	fn void_and_self_closing_elements_do_not_change_nesting_depth() {
		let html = "<p>a<br>b<img src=\"x\"/>c</p><p>two</p><p>three</p>";
		let chunks = split_html_chunks_with_target(html, 15);
		assert_eq!(chunks.concat(), html);
	}
}
