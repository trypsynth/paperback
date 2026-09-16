use std::{
	collections::{HashMap, HashSet},
	fmt::Write as _,
	ops::Range,
};

use crate::{
	document::{DocumentHandle, MarkerType},
	parser::is_external_url,
	t,
	util::{
		html::{escape, escape_attr, push_escaped},
		text::ch_width,
	},
};

/// The id of the anchor [`render_with_anchor`] writes at a reading position.
#[must_use]
pub fn anchor_id(offset: usize) -> String {
	format!("pos-{offset}")
}

#[must_use]
pub fn render(doc: &DocumentHandle) -> String {
	render_with_anchor(doc, None)
}

/// Renders the document, with an anchor named by [`anchor_id`] at `anchor`.
///
/// A format that keeps no markup of its own has nothing for a web view to open but this
/// rendering, so the reading position has to travel in the rendering itself.
#[must_use]
pub fn render_with_anchor(doc: &DocumentHandle, anchor: Option<usize>) -> String {
	let total = doc.document().buffer.total_display_len();
	render_range(doc, 0..total, anchor)
}

/// Renders the display-unit span `range` of the document, with an anchor named by
/// [`anchor_id`] at `anchor`.
///
/// Both ends of `range` must already sit on a line boundary; [`render_with_anchor`]'s
/// whole-document range trivially does, and every other caller snaps first. A marker whose
/// own span lies wholly outside `range` is skipped, so rendering a window of a huge book
/// costs the window rather than the book. A range short of the whole document opens the
/// body with a line saying so, since a reader given a slice with no such line has no way to
/// tell it apart from a book that ends there.
#[must_use]
pub fn render_range(doc: &DocumentHandle, range: Range<usize>, anchor: Option<usize>) -> String {
	let document = doc.document();
	let content = &document.buffer.content;
	let doc_display_len = document.buffer.total_display_len();
	let range_start = range.start.min(doc_display_len);
	let range_end = range.end.clamp(range_start, doc_display_len);
	let is_partial = range_start > 0 || range_end < doc_display_len;
	let content = {
		let byte_start = document.buffer.byte_index_for_display(range_start);
		let byte_end = document.buffer.byte_index_for_display(range_end);
		&content[byte_start..byte_end]
	};
	// Precompute section boundaries once so link resolution is O(log S) per link
	// instead of O(M) per link (where M = total marker count).
	let section_break_positions: Vec<usize> =
		document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::SectionBreak).map(|m| m.position).collect();
	// Single scan of the rendered range: collect newline positions in display coordinates.
	// Turns each line-end lookup into an O(log lines) binary search instead of a fresh scan.
	let newline_display_positions: Vec<usize> = {
		let mut positions = Vec::new();
		let mut dpos = range_start;
		for ch in content.chars() {
			if ch == '\n' {
				positions.push(dpos);
			}
			dpos += ch_width(ch);
		}
		positions
	};
	// A line running past the end of the range ends there, which is a line boundary of the
	// rendered slice even when it is not one of the document.
	let newline_from = |start: usize| -> usize {
		let idx = newline_display_positions.partition_point(|&p| p < start);
		newline_display_positions.get(idx).copied().unwrap_or(range_end)
	};
	// path → (section_start, section_end)
	let path_to_bounds: HashMap<&str, (usize, usize)> = document
		.spine_items
		.iter()
		.enumerate()
		.filter_map(|(i, manifest_id)| {
			let path = document.manifest_items.get(manifest_id)?;
			let start = section_break_positions.get(i).copied().unwrap_or(0);
			let end = section_break_positions.get(i + 1).copied().unwrap_or(doc_display_len);
			Some((path.as_str(), (start, end)))
		})
		.collect();
	// Returns the file path of the spine item that contains `pos`.
	let section_path_at = |pos: usize| -> Option<&str> {
		let count = section_break_positions.partition_point(|&bp| bp <= pos);
		if count == 0 {
			return None;
		}
		let manifest_id = document.spine_items.get(count - 1)?;
		document.manifest_items.get(manifest_id).map(String::as_str)
	};
	let mut html = format!(
		"<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n</head>\n<body>\n",
		escape(&document.title)
	);
	if is_partial {
		// TRANSLATORS: Note at the top of a web view showing only the part of a very long document around the reading position
		let _ = writeln!(html, "<p>{}</p>", escape(&t("This view shows the part of the document you are reading.")));
	}
	enum Ek {
		BlockOpen(&'static str),
		BlockClose(&'static str),
		InlineOpen(String),
		InlineClose(&'static str),
		Hr,
		Replace { until: usize, content: String },
		Anchor(usize),
	}
	struct Ev {
		pos: usize,
		kind: Ek,
	}
	let mut events: Vec<Ev> = Vec::new();
	let mut target_offsets = HashSet::new();
	target_offsets.extend(anchor);
	for marker in &document.buffer.markers {
		let pos = marker.position;
		// Outside the rendered range this marker has nothing to open or close, and resolving
		// its link target would cost the whole book's worth of lookups for nothing. A length of
		// zero means the span runs to the end of its line, which cannot reach a range that
		// starts on a line boundary of its own.
		if pos >= range_end || pos.saturating_add(marker.length.max(1)) <= range_start {
			continue;
		}
		// Markers from html_to_text carry length=0 for headings, links, and list items
		// because those types store their span only implicitly in the content.
		// Recover the span: for block elements scan to the next '\n'; for inline links
		// use the display length of the link text that was written into the content.
		let effective_end =
			|explicit: usize| -> usize { if explicit > 0 { pos + explicit } else { newline_from(pos) } };
		match marker.mtype {
			MarkerType::Heading1 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h1>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h1>") });
			}
			MarkerType::Heading2 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h2>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h2>") });
			}
			MarkerType::Heading3 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h3>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h3>") });
			}
			MarkerType::Heading4 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h4>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h4>") });
			}
			MarkerType::Heading5 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h5>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h5>") });
			}
			MarkerType::Heading6 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<h6>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</h6>") });
			}
			MarkerType::Link => {
				let Some(end) = super::link_span_end(marker) else { continue };
				let open = if marker.reference.is_empty() {
					"<a>".to_string()
				} else {
					let href = marker.reference.trim();
					if is_external_url(href) {
						format!("<a href=\"{}\">", escape_attr(href))
					} else if let Some(fragment) = href.strip_prefix('#') {
						let current_path = section_path_at(pos);
						resolve_fragment(&document.id_positions, fragment, current_path).map_or_else(
							|| format!("<a href=\"{}\">", escape_attr(href)),
							|off| {
								target_offsets.insert(off);
								format!("<a href=\"#pos-{off}\">")
							},
						)
					} else {
						let mut parts = href.splitn(2, '#');
						let file_part = parts.next().unwrap_or_default();
						let frag_part = parts.next().unwrap_or_default();
						if let Some(&(section_start, section_end)) = path_to_bounds.get(file_part) {
							let off = if frag_part.is_empty() {
								section_start
							} else {
								resolve_fragment(&document.id_positions, frag_part, Some(file_part))
									.filter(|&f| f >= section_start && f < section_end)
									.unwrap_or(section_start)
							};
							target_offsets.insert(off);
							format!("<a href=\"#pos-{off}\">")
						} else {
							// CHM / fallback: try fragment, then bare file-path key
							let current_path = section_path_at(pos);
							let off = if frag_part.is_empty() {
								document.id_positions.get(file_part).copied()
							} else {
								resolve_fragment(&document.id_positions, frag_part, Some(file_part))
									.or_else(|| document.id_positions.get(file_part).copied())
									.or_else(|| resolve_fragment(&document.id_positions, frag_part, current_path))
							};
							off.map_or_else(
								|| format!("<a href=\"{}\">", escape_attr(href)),
								|off| {
									target_offsets.insert(off);
									format!("<a href=\"#pos-{off}\">")
								},
							)
						}
					}
				};
				events.push(Ev { pos, kind: Ek::InlineOpen(open) });
				events.push(Ev { pos: end, kind: Ek::InlineClose("</a>") });
			}
			MarkerType::Bold => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Ek::InlineOpen("<b>".to_string()) });
				events.push(Ev { pos: end, kind: Ek::InlineClose("</b>") });
			}
			MarkerType::Italic => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Ek::InlineOpen("<i>".to_string()) });
				events.push(Ev { pos: end, kind: Ek::InlineClose("</i>") });
			}
			MarkerType::Underline => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Ek::InlineOpen("<u>".to_string()) });
				events.push(Ev { pos: end, kind: Ek::InlineClose("</u>") });
			}
			MarkerType::List if marker.length > 0 => {
				// Only emit a <ul> wrapper when an explicit length is available; without it
				// we cannot determine where the list ends and bare <li> items are cleaner.
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Ek::BlockOpen("<ul>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</ul>") });
			}
			MarkerType::ListItem => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, kind: Ek::BlockOpen("<li>") });
				events.push(Ev { pos: end, kind: Ek::BlockClose("</li>") });
			}
			MarkerType::Table if !marker.reference.is_empty() => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Ek::Replace { until: end, content: marker.reference.clone() } });
			}
			MarkerType::PageBreak | MarkerType::Separator => {
				events.push(Ev { pos, kind: Ek::Hr });
			}
			_ => {}
		}
	}
	// An anchor outside the range has no text to sit next to, and the tail flush below would
	// otherwise pile every one of them up at the end of the page.
	for offset in target_offsets.into_iter().filter(|&off| off >= range_start && off <= range_end) {
		events.push(Ev { pos: offset, kind: Ek::Anchor(offset) });
	}
	// Closes before opens at the same position to avoid empty elements
	events.sort_by(|a, b| {
		a.pos.cmp(&b.pos).then_with(|| {
			let p = |k: &Ek| match k {
				Ek::BlockClose(_) | Ek::InlineClose(_) => 0u8,
				Ek::Hr | Ek::Replace { .. } => 1,
				Ek::BlockOpen(_) => 2,
				Ek::InlineOpen(_) => 3,
				Ek::Anchor(_) => 4,
			};
			p(&a.kind).cmp(&p(&b.kind))
		})
	});
	let mut event_idx = 0usize;
	let mut block_depth: usize = 0;
	let mut in_para = false;
	let mut pending_newlines: usize = 0;
	let mut display_pos: usize = range_start;
	let mut skip_until: Option<usize> = None;
	for ch in content.chars() {
		// Fire events whose position has been reached
		while event_idx < events.len() && events[event_idx].pos <= display_pos {
			// Suppress events that fall inside an active replace range
			if skip_until.is_some_and(|u| events[event_idx].pos < u) && !matches!(events[event_idx].kind, Ek::Anchor(_))
			{
				event_idx += 1;
				continue;
			}
			match &events[event_idx].kind {
				Ek::BlockOpen(tag) => {
					if block_depth == 0 {
						if in_para {
							html.push_str("</p>\n");
							in_para = false;
						}
						pending_newlines = 0;
					}
					html.push_str(tag);
					html.push('\n');
					block_depth += 1;
				}
				Ek::BlockClose(tag) => {
					block_depth = block_depth.saturating_sub(1);
					html.push_str(tag);
					html.push('\n');
					if block_depth == 0 {
						pending_newlines = 0;
						in_para = false;
					}
				}
				Ek::InlineOpen(tag) => {
					if block_depth == 0 {
						if pending_newlines >= 1 && in_para {
							html.push_str("</p>\n");
							in_para = false;
						}
						pending_newlines = 0;
						if !in_para {
							html.push_str("<p>");
							in_para = true;
						}
					}
					html.push_str(tag);
				}
				Ek::InlineClose(tag) => {
					html.push_str(tag);
				}
				Ek::Hr => {
					if block_depth == 0 && in_para {
						html.push_str("</p>\n");
						in_para = false;
					}
					html.push_str("<hr>\n");
					pending_newlines = 0;
				}
				Ek::Replace { until, content: ref_html } => {
					if block_depth == 0 && in_para {
						html.push_str("</p>\n");
						in_para = false;
					}
					html.push_str(ref_html);
					html.push('\n');
					skip_until = Some(*until);
					pending_newlines = 0;
				}
				Ek::Anchor(offset) => {
					let _ = write!(html, "<a id=\"pos-{offset}\"></a>");
				}
			}
			event_idx += 1;
		}
		// Skip chars that belong to a replaced range (e.g. table inline text)
		if let Some(until) = skip_until {
			if display_pos < until {
				display_pos += ch_width(ch);
				continue;
			}
			skip_until = None;
		}
		if block_depth == 0 {
			// Free-text mode: detect paragraph boundaries on double newlines
			if ch == '\n' {
				pending_newlines += 1;
			} else {
				if pending_newlines >= 1 && in_para {
					html.push_str("</p>\n");
					in_para = false;
				}
				pending_newlines = 0;
				if !in_para {
					html.push_str("<p>");
					in_para = true;
				}
				push_escaped(ch, &mut html);
			}
		} else if ch != '\n' {
			// Inside a block element: escape and emit, skip bare newlines
			push_escaped(ch, &mut html);
		}
		display_pos += ch_width(ch);
	}
	// Flush any closing tags that extend to or past end of content
	while event_idx < events.len() {
		match &events[event_idx].kind {
			Ek::BlockClose(tag) => {
				html.push_str(tag);
				html.push('\n');
			}
			Ek::InlineClose(tag) => {
				html.push_str(tag);
			}
			Ek::Hr => {
				if in_para {
					html.push_str("</p>\n");
					in_para = false;
				}
				html.push_str("<hr>\n");
			}
			Ek::Anchor(offset) => {
				let _ = write!(html, "<a id=\"pos-{offset}\"></a>");
			}
			_ => {}
		}
		event_idx += 1;
	}
	if in_para {
		html.push_str("</p>\n");
	}
	html.push_str("</body>\n</html>\n");
	html
}

fn resolve_fragment(id_positions: &HashMap<String, usize>, fragment: &str, scoped_path: Option<&str>) -> Option<usize> {
	let fragment = fragment.trim_start_matches('#');
	if fragment.is_empty() {
		return None;
	}
	if let Some(path) = scoped_path {
		let key = format!("{path}#{fragment}");
		if let Some(&offset) = id_positions.get(&key) {
			return Some(offset);
		}
	}
	id_positions.get(fragment).copied()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, DocumentHandle, Marker, MarkerType};

	fn simple_doc(content: &str, markers: Vec<Marker>) -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content(content.to_string());
		for marker in markers {
			buffer.add_marker(marker);
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		DocumentHandle::new(doc)
	}

	// A range covering the whole document is what `render` itself asks for, so the two must not
	// be able to drift apart.
	#[test]
	fn render_range_over_the_whole_document_matches_render() {
		let doc = simple_doc(
			"alpha\nbeta\ngamma",
			vec![Marker::new(MarkerType::Heading1, 0).with_length(5), Marker::new(MarkerType::Bold, 6).with_length(4)],
		);
		let total = doc.document().buffer.total_display_len();
		assert_eq!(render_range(&doc, 0..total, None), render(&doc));
	}

	#[test]
	fn render_range_renders_only_the_range_and_says_it_is_partial() {
		let doc = simple_doc("alpha\nbeta\ngamma\n", vec![]);
		let html = render_range(&doc, 6..11, None);
		assert!(html.contains("beta"), "{html}");
		assert!(!html.contains("alpha"), "{html}");
		assert!(!html.contains("gamma"), "{html}");
		assert!(html.contains("part of the document"), "a partial view says so: {html}");
	}

	// An anchor for a link target outside the range used to be flushed out at the end of the page,
	// which on a book with a link per paragraph meant every anchor in the book.
	#[test]
	fn render_range_leaves_out_anchors_for_targets_outside_the_range() {
		let doc = simple_doc("alpha\nbeta\ngamma\n", vec![Marker::new(MarkerType::Bold, 0).with_length(5)]);
		let html = render_range(&doc, 6..11, Some(6));
		assert_eq!(html.matches(r#"<a id=""#).count(), 1, "only the reading position is anchored: {html}");
	}

	#[test]
	fn test_bold_basic() {
		let doc = simple_doc("bold text", vec![Marker::new(MarkerType::Bold, 0).with_length(4)]);
		let html = render(&doc);
		assert!(html.contains("<b>bold</b>"), "Expected <b>bold</b> in HTML: {html}");
	}

	#[test]
	fn test_italic_basic() {
		let doc = simple_doc("italic text", vec![Marker::new(MarkerType::Italic, 0).with_length(6)]);
		let html = render(&doc);
		assert!(html.contains("<i>italic</i>"), "Expected <i>italic</i> in HTML: {html}");
	}

	#[test]
	fn test_underline_basic() {
		let doc = simple_doc("underline text", vec![Marker::new(MarkerType::Underline, 0).with_length(9)]);
		let html = render(&doc);
		assert!(html.contains("<u>underline</u>"), "Expected <u>underline</u> in HTML: {html}");
	}

	#[test]
	fn test_nested_bold_italic() {
		// "bold italic" where 0-4 is bold, 5-11 is italic (nested/overlapping)
		let doc = simple_doc(
			"bold italic",
			vec![Marker::new(MarkerType::Bold, 0).with_length(4), Marker::new(MarkerType::Italic, 5).with_length(6)],
		);
		let html = render(&doc);
		// Both tags should be present and properly ordered
		assert!(html.contains("<b>bold</b>"), "Expected <b>bold</b> in HTML: {html}");
		assert!(html.contains("<i>italic</i>"), "Expected <i>italic</i> in HTML: {html}");
	}

	#[test]
	fn test_nested_same_start_different_end() {
		// "bold italic" where 0-11 is bold, 0-6 is italic (nested: italic entirely inside bold)
		let doc = simple_doc(
			"bold italic",
			vec![Marker::new(MarkerType::Bold, 0).with_length(11), Marker::new(MarkerType::Italic, 0).with_length(6)],
		);
		let html = render(&doc);
		assert!(html.contains("<b>"), "Expected <b> in HTML: {html}");
		assert!(html.contains("</b>"), "Expected </b> in HTML: {html}");
		assert!(html.contains("<i>"), "Expected <i> in HTML: {html}");
		assert!(html.contains("</i>"), "Expected </i> in HTML: {html}");
	}

	#[test]
	fn test_coincident_end_edge_case() {
		// "text" where both bold and italic end at position 4
		// This tests the pre-existing characteristic where non-perfectly-nested
		// tags can occur based on insertion order in buffer.markers
		let doc = simple_doc(
			"text",
			vec![Marker::new(MarkerType::Bold, 0).with_length(4), Marker::new(MarkerType::Italic, 0).with_length(4)],
		);
		let html = render(&doc);
		// The order of closes may vary based on insertion order (pre-existing behavior)
		assert!(html.contains("<b>"), "Expected <b> in HTML: {html}");
		assert!(html.contains("</b>"), "Expected </b> in HTML: {html}");
		assert!(html.contains("<i>"), "Expected <i> in HTML: {html}");
		assert!(html.contains("</i>"), "Expected </i> in HTML: {html}");
		let b_open = html.matches("<b>").count();
		let b_close = html.matches("</b>").count();
		let i_open = html.matches("<i>").count();
		let i_close = html.matches("</i>").count();
		assert_eq!(b_open, b_close, "Bold tags should be paired");
		assert_eq!(i_open, i_close, "Italic tags should be paired");
		// Note: the tag nesting order depends on marker iteration order.
		// The output might be <b><i>text</i></b> or <i><b>text</b></i> or
		// </b></i> before </i></b> depending on which marker is processed first.
		// This is acceptable per the task brief.
	}

	#[test]
	fn test_multiple_separate_bold_spans() {
		// "bold text normal more bold"
		// Bold at 0-4 and 22-26
		let doc = simple_doc(
			"bold text normal more bold",
			vec![Marker::new(MarkerType::Bold, 0).with_length(4), Marker::new(MarkerType::Bold, 22).with_length(4)],
		);
		let html = render(&doc);
		let b_open = html.matches("<b>").count();
		let b_close = html.matches("</b>").count();
		assert_eq!(b_open, 2, "Expected 2 <b> opens");
		assert_eq!(b_close, 2, "Expected 2 </b> closes");
	}

	#[test]
	fn test_bold_italic_underline_combined() {
		// "text" with all three marker types
		let doc = simple_doc(
			"text",
			vec![
				Marker::new(MarkerType::Bold, 0).with_length(4),
				Marker::new(MarkerType::Italic, 1).with_length(2),
				Marker::new(MarkerType::Underline, 2).with_length(2),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<b>"), "Expected <b> in HTML: {html}");
		assert!(html.contains("</b>"), "Expected </b> in HTML: {html}");
		assert!(html.contains("<i>"), "Expected <i> in HTML: {html}");
		assert!(html.contains("</i>"), "Expected </i> in HTML: {html}");
		assert!(html.contains("<u>"), "Expected <u> in HTML: {html}");
		assert!(html.contains("</u>"), "Expected </u> in HTML: {html}");
	}
}
