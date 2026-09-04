//! Splitting decoded MOBI HTML into bounded chunks before DOM parsing.
//!
//! `HtmlToText::convert` builds a full DOM tree (`scraper`/`html5ever`) for whatever it's
//! handed. MOBI decodes its *entire* book into one HTML string before converting, so a
//! single `convert` call means a DOM proportional to the whole book — which runs the
//! process out of memory on genuinely huge books (#781). Splitting into bounded chunks,
//! each parsed and converted independently and then stitched back together (the way EPUB
//! already does per spine item), keeps peak DOM size bounded by chunk size instead of book
//! size.

use crate::parser::convert::block_elements::is_block_element;

/// Target size, in bytes of still-encoded HTML, of each chunk handed to a single
/// `HtmlToText::convert` call. Large enough that ordinary books produce only one or a
/// handful of chunks (no behavior change from before chunking existed); small enough that
/// even a multi-hundred-megabyte book's peak DOM size stays bounded.
const TARGET_CHUNK_BYTES: usize = 4 * 1024 * 1024;

const VOID_ELEMENTS: &[&str] =
	&["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];

/// Splits `html` into chunks of roughly [`TARGET_CHUNK_BYTES`] each. See
/// [`split_html_chunks_with_target`] for how split points are chosen.
pub(super) fn split_html_chunks(html: &str) -> Vec<&str> {
	split_html_chunks_with_target(html, TARGET_CHUNK_BYTES)
}

/// Splits `html` into chunks of roughly `target_bytes` each, cutting only right after a
/// block-level element's closing (or self-closing) tag at a nesting depth back to (at most) what
/// it was at the start of the current chunk — i.e. a point that was already a line break in the
/// rendered output, not partway through one. This keeps every chunk independently well-formed on
/// its own (an element straddling a chunk boundary just loses its formatting at the seam, never
/// its text) and keeps each `HtmlToText`'s fresh internal line-building state consistent with
/// what a single, uninterrupted conversion would have done at that same point.
///
/// Falls back to fewer, larger chunks (down to a single one covering all of `html`) whenever no
/// safe split point is found before `target_bytes` is reached — content with no tags at all, or
/// one element wrapping everything, being the extreme cases. Chunking is a memory-bound
/// optimization, not a correctness requirement, so there is never a wrong answer to fall back to.
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
	// The most recent position (and the nesting depth there) that is safe to split at, since
	// the last chunk boundary. `None` once consumed by a split, until the next tag closes.
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
