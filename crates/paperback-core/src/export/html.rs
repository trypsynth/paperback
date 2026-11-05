use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
	fmt::Write as _,
	ops::Range,
};

use crate::{
	document::{DocLen, DocPos, DocumentHandle, MarkerType},
	parser::is_external_url,
	t,
	util::html::{escape, escape_attr, push_escaped_str},
};

const PRE_OPEN: &str = "<pre>";
const PRE_CLOSE: &str = "</pre>";

#[must_use]
pub fn anchor_id(offset: DocPos) -> String {
	format!("pos-{offset}")
}

#[must_use]
pub fn render(doc: &DocumentHandle) -> String {
	render_with_anchor(doc, None)
}

/// A format with no markup of its own has nothing for a web view to open but this, so the reading position travels in it as an anchor.
#[must_use]
pub fn render_with_anchor(doc: &DocumentHandle, anchor: Option<DocPos>) -> String {
	let total = doc.document().buffer.end();
	render_range(doc, DocPos::ZERO..total, anchor)
}

/// Both ends of `range` must sit on a line boundary. A partial range opens with a line saying so, since a slice is otherwise indistinguishable from a book that ends there.
#[must_use]
pub fn render_range(doc: &DocumentHandle, range: Range<DocPos>, anchor: Option<DocPos>) -> String {
	let document = doc.document();
	let doc_end = document.buffer.end();
	let range_start = range.start.min(doc_end);
	let range_end = range.end.clamp(range_start, doc_end);
	let is_partial = range_start > DocPos::ZERO || range_end < doc_end;
	let content = document.buffer.slice(document.buffer.byte_at(range_start)..document.buffer.byte_at(range_end));
	let section_break_positions: Vec<DocPos> =
		document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::SectionBreak).map(|m| m.position).collect();
	let newline_positions = document.buffer.newline_positions();
	// The range end is a line boundary of the rendered slice even when it is not one of the document.
	let newline_from = |start: DocPos| -> DocPos {
		let idx = newline_positions.partition_point(|&p| p < start);
		newline_positions.get(idx).copied().filter(|&p| p < range_end).unwrap_or(range_end)
	};
	let path_to_bounds: HashMap<&str, (DocPos, DocPos)> = document
		.spine_items
		.iter()
		.enumerate()
		.filter_map(|(i, manifest_id)| {
			let path = document.manifest_items.get(manifest_id)?;
			let start = section_break_positions.get(i).copied().unwrap_or_default();
			let end = section_break_positions.get(i + 1).copied().unwrap_or(doc_end);
			Some((path.as_str(), (start, end)))
		})
		.collect();
	let section_path_at = |pos: DocPos| -> Option<&str> {
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
	html.reserve(content.len() + content.len() / 8);
	if is_partial {
		// TRANSLATORS: Note at the top of a web view showing only the part of a very long document around the reading position
		let _ = writeln!(html, "<p>{}</p>", escape(&t("This view shows the part of the document you are reading.")));
	}
	enum Ek<'a> {
		BlockOpen(&'a str),
		/// The item's number, empty when it has none.
		ItemOpen(&'a str),
		SkipPrefix(DocPos),
		BlockClose(&'static str),
		InlineOpen(String, &'static str),
		InlineClose(&'static str),
		Hr,
		Replace {
			until: DocPos,
			content: String,
		},
		Anchor(DocPos),
	}
	struct Ev<'a> {
		pos: DocPos,
		/// Where the span began and its marker's index, so spans ending together can close in reverse.
		opened: (DocPos, usize),
		kind: Ek<'a>,
	}
	let mut events: Vec<Ev> = Vec::new();
	let mut target_offsets = HashSet::new();
	target_offsets.extend(anchor);
	for (span, marker) in document.buffer.markers.iter().enumerate() {
		let pos = marker.position;
		let opened = (pos, span);
		// Skipped so a window of a huge book costs the window; a zero length runs to its line's end, which can't reach a range starting on a line boundary.
		if pos >= range_end || pos + marker.length.max(DocLen::ONE) <= range_start {
			continue;
		}
		// html_to_text leaves headings and list items at length 0, so their span runs to the next newline.
		let effective_end =
			|explicit: DocLen| -> DocPos { if explicit.is_zero() { newline_from(pos) } else { pos + explicit } };
		match marker.mtype {
			MarkerType::Heading1 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h1>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h1>") });
			}
			MarkerType::Heading2 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h2>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h2>") });
			}
			MarkerType::Heading3 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h3>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h3>") });
			}
			MarkerType::Heading4 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h4>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h4>") });
			}
			MarkerType::Heading5 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h5>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h5>") });
			}
			MarkerType::Heading6 => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::BlockOpen("<h6>") });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</h6>") });
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
				events.push(Ev { pos, opened, kind: Ek::InlineOpen(open, "</a>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</a>") });
			}
			MarkerType::Bold if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<b>".to_string(), "</b>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</b>") });
			}
			MarkerType::Italic if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<i>".to_string(), "</i>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</i>") });
			}
			MarkerType::Underline if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<u>".to_string(), "</u>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</u>") });
			}
			MarkerType::List if !marker.length.is_zero() => {
				// Without a length the list's end is unknown, and bare <li> items beat a guessed <ul>.
				let end = pos + marker.length;
				let ordered = marker.reference.starts_with("<ol");
				let open = if marker.reference.is_empty() { "<ul>" } else { marker.reference.as_str() };
				events.push(Ev { pos, opened, kind: Ek::BlockOpen(open) });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose(if ordered { "</ol>" } else { "</ul>" }) });
			}
			MarkerType::ListItem => {
				let end = effective_end(marker.length);
				events.push(Ev { pos, opened, kind: Ek::ItemOpen(&marker.reference) });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose("</li>") });
				if !marker.prefix_length.is_zero() {
					// An empty item's bullet loses its trailing space to the line trim, so its prefix can reach past the line.
					let until = (pos + marker.prefix_length).min(end);
					events.push(Ev { pos, opened, kind: Ek::SkipPrefix(until) });
				}
			}
			MarkerType::Superscript if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<sup>".to_string(), "</sup>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</sup>") });
			}
			MarkerType::Subscript if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<sub>".to_string(), "</sub>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</sub>") });
			}
			MarkerType::Preformatted if !marker.length.is_zero() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::BlockOpen(PRE_OPEN) });
				events.push(Ev { pos: end, opened, kind: Ek::BlockClose(PRE_CLOSE) });
			}
			// Across lines it would straddle the paragraphs those lines become.
			MarkerType::Code if !marker.length.is_zero() && newline_from(pos) >= pos + marker.length => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::InlineOpen("<code>".to_string(), "</code>") });
				events.push(Ev { pos: end, opened, kind: Ek::InlineClose("</code>") });
			}
			MarkerType::Table if !marker.reference.is_empty() => {
				let end = pos + marker.length;
				events.push(Ev { pos, opened, kind: Ek::Replace { until: end, content: marker.reference.clone() } });
			}
			MarkerType::PageBreak | MarkerType::Separator => {
				events.push(Ev { pos, opened, kind: Ek::Hr });
			}
			_ => {}
		}
	}
	// Otherwise the tail flush below piles every anchor outside the range up at the end of the page.
	for offset in target_offsets.into_iter().filter(|&off| off >= range_start && off <= range_end) {
		events.push(Ev { pos: offset, opened: (offset, 0), kind: Ek::Anchor(offset) });
	}
	// Closes before opens at the same position to avoid empty elements. Whatever opened last closes first, so a heading ending in a link is `</a></h1>`, while a link around a heading is `</h1></a>`.
	events.sort_by(|a, b| {
		let class = |k: &Ek| match k {
			Ek::InlineClose(_) | Ek::BlockClose(_) => 0u8,
			Ek::Hr | Ek::Replace { .. } => 1,
			Ek::BlockOpen(_) | Ek::ItemOpen(_) => 2,
			Ek::InlineOpen(..) => 3,
			Ek::Anchor(_) => 4,
			// Last, since a skip drops whatever follows it inside the skipped span.
			Ek::SkipPrefix(_) => 5,
		};
		let inner = |k: &Ek| u8::from(matches!(k, Ek::BlockClose(_)));
		let (ca, cb) = (class(&a.kind), class(&b.kind));
		a.pos.cmp(&b.pos).then_with(|| ca.cmp(&cb)).then_with(|| {
			if ca == 0 {
				b.opened
					.0
					.cmp(&a.opened.0)
					.then_with(|| inner(&a.kind).cmp(&inner(&b.kind)))
					.then_with(|| b.opened.1.cmp(&a.opened.1))
			} else {
				Ordering::Equal
			}
		})
	});
	let mut event_idx = 0usize;
	let mut block_depth: usize = 0;
	let mut in_para = false;
	let mut pending_newlines: usize = 0;
	let mut position = range_start;
	let mut skip_until: Option<DocPos> = None;
	let mut pre_depth: usize = 0;
	let mut pre_inline_base: usize = 0;
	let mut ol_depth: usize = 0;
	let mut keep_prefix = false;
	let mut open_inline: Vec<(&str, &'static str)> = Vec::new();
	let mut rest = content;
	while !rest.is_empty() {
		while event_idx < events.len() && events[event_idx].pos <= position {
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
					if *tag == PRE_OPEN && pre_depth == 0 {
						pre_inline_base = open_inline.len();
					}
					pre_depth += usize::from(*tag == PRE_OPEN);
					ol_depth += usize::from(tag.starts_with("<ol"));
				}
				Ek::ItemOpen(number) => {
					if block_depth == 0 {
						if in_para {
							html.push_str("</p>\n");
							in_para = false;
						}
						pending_newlines = 0;
					}
					// A page range can cut off the list an item belongs to, and then the text is the only place its number survives.
					keep_prefix = !number.is_empty() && ol_depth == 0;
					if number.is_empty() || keep_prefix {
						html.push_str("<li>\n");
					} else {
						let _ = writeln!(html, "<li value=\"{number}\">");
					}
					block_depth += 1;
				}
				Ek::SkipPrefix(until) => {
					if !keep_prefix {
						skip_until = Some(*until);
					}
				}
				Ek::BlockClose(tag) => {
					block_depth = block_depth.saturating_sub(1);
					// A span around a whole block ends a newline after it, so it has to step outside the block to stay nested.
					let held: &[(&str, &str)] = if *tag == PRE_CLOSE && pre_depth == 1 {
						&open_inline[pre_inline_base.min(open_inline.len())..]
					} else {
						&[]
					};
					pre_depth = pre_depth.saturating_sub(usize::from(*tag == PRE_CLOSE));
					ol_depth = ol_depth.saturating_sub(usize::from(*tag == "</ol>"));
					for &(_, close) in held.iter().rev() {
						html.push_str(close);
					}
					html.push_str(tag);
					html.push('\n');
					for &(open, _) in held {
						html.push_str(open);
					}
					if block_depth == 0 {
						pending_newlines = 0;
						in_para = false;
					}
				}
				Ek::InlineOpen(tag, close) => {
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
					open_inline.push((tag, close));
				}
				Ek::InlineClose(tag) => close_inline(&mut html, &mut open_inline, tag),
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
		if skip_until.is_some_and(|until| position >= until) {
			skip_until = None;
		}
		let next_event = events.get(event_idx).map(|event| event.pos);
		let stop = skip_until.into_iter().chain(next_event).min();
		let (run, tail) = stop.map_or((rest, ""), |stop| super::split_after_chars(rest, stop.get() - position.get()));
		rest = tail;
		position = stop.unwrap_or(position);
		if skip_until.is_some() {
			continue;
		}
		for (index, line) in run.split('\n').enumerate() {
			if index > 0 {
				if block_depth == 0 {
					pending_newlines += 1;
				} else if pre_depth > 0 || !html.ends_with('\n') {
					// In a list this keeps an item's later paragraphs from running together.
					html.push('\n');
				}
			}
			if line.is_empty() {
				continue;
			}
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
			push_escaped_str(line, &mut html);
		}
	}
	while event_idx < events.len() {
		match &events[event_idx].kind {
			Ek::BlockClose(tag) => {
				html.push_str(tag);
				html.push('\n');
			}
			Ek::InlineClose(tag) => close_inline(&mut html, &mut open_inline, tag),
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

/// A parser can place spans so that they cross, so closing one closes whatever opened inside it and reopens those afterward.
fn close_inline(html: &mut String, open: &mut Vec<(&str, &'static str)>, close: &'static str) {
	let Some(depth) = open.iter().rposition(|&(_, tag)| tag == close) else { return };
	for &(_, tag) in open[depth + 1..].iter().rev() {
		html.push_str(tag);
	}
	let (opening, _) = open.remove(depth);
	if depth == open.len() && html.ends_with(opening) {
		// Just reopened, with nothing written since.
		html.truncate(html.len() - opening.len());
	} else {
		html.push_str(close);
	}
	for &(tag, _) in &open[depth..] {
		html.push_str(tag);
	}
}

fn resolve_fragment(
	id_positions: &HashMap<String, DocPos>,
	fragment: &str,
	scoped_path: Option<&str>,
) -> Option<DocPos> {
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

	#[test]
	fn a_heading_ending_in_a_link_closes_the_link_first() {
		let doc = simple_doc(
			"By Jane\nbody",
			vec![
				Marker::new(MarkerType::Heading1, 0.into()).with_length(7.into()),
				Marker::new(MarkerType::Link, 3.into())
					.with_text("Jane".to_string())
					.with_reference("https://a.org".to_string()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("Jane</a></h1>"), "{html}");
	}

	#[test]
	fn a_link_opening_before_a_heading_closes_after_it() {
		let doc = simple_doc(
			"see Title\nbody",
			vec![
				Marker::new(MarkerType::Heading1, 4.into()).with_length(5.into()),
				Marker::new(MarkerType::Link, 0.into())
					.with_text("see Title".to_string())
					.with_reference("https://a.org".to_string()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("</h1>\n</a>"), "{html}");
	}

	#[test]
	fn an_empty_span_writes_no_tags() {
		let doc = simple_doc(
			"text",
			vec![
				Marker::new(MarkerType::Bold, 2.into()).with_length(0.into()),
				Marker::new(MarkerType::Italic, 2.into()).with_length(0.into()),
				Marker::new(MarkerType::Underline, 2.into()).with_length(0.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<p>text</p>"), "{html}");
	}

	#[test]
	fn spans_over_the_same_text_close_in_reverse() {
		let doc = simple_doc(
			"text",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Italic, 0.into()).with_length(4.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<b><i>text</i></b>"), "{html}");
	}

	#[test]
	fn a_list_ending_with_its_item_closes_the_item_first() {
		let doc = simple_doc(
			"one\nafter",
			vec![
				Marker::new(MarkerType::List, 0.into()).with_length(3.into()),
				Marker::new(MarkerType::ListItem, 0.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("</li>\n</ul>"), "{html}");
	}

	fn list_doc(content: &str, tag: &str, items: &[(usize, &str, usize)]) -> DocumentHandle {
		let mut markers = vec![
			Marker::new(MarkerType::List, 0.into())
				.with_reference(tag.to_string())
				.with_length(content.chars().count().into()),
		];
		for &(position, number, prefix) in items {
			markers.push(
				Marker::new(MarkerType::ListItem, position.into())
					.with_reference(number.to_string())
					.with_prefix_length(prefix.into()),
			);
		}
		simple_doc(content, markers)
	}

	#[test]
	fn a_bulleted_list_item_leaves_its_bullet_to_the_list() {
		let doc = list_doc("• one\n• two\n", "<ul>", &[(0, "", 2), (6, "", 2)]);
		let html = render(&doc);
		assert!(html.contains("<ul>\n<li>\none</li>\n<li>\ntwo</li>\n</ul>"), "{html}");
		assert!(!html.contains('•'), "{html}");
	}

	#[test]
	fn a_numbered_list_keeps_its_numbers_and_style_as_markup() {
		let doc = list_doc("c. one\nd. two\n", "<ol type=\"a\">", &[(0, "3", 3), (7, "4", 3)]);
		let html = render(&doc);
		assert!(
			html.contains("<ol type=\"a\">\n<li value=\"3\">\none</li>\n<li value=\"4\">\ntwo</li>\n</ol>"),
			"{html}"
		);
	}

	#[test]
	fn a_numbered_item_cut_off_from_its_list_keeps_its_number_in_the_text() {
		let doc = simple_doc(
			"7. seven\n",
			vec![
				Marker::new(MarkerType::ListItem, 0.into())
					.with_reference("7".to_string())
					.with_prefix_length(3.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<li>\n7. seven</li>"), "{html}");
	}

	#[test]
	fn a_list_item_from_a_parser_without_a_prefix_loses_no_text() {
		let doc = list_doc("- one\n", "", &[(0, "", 0)]);
		let html = render(&doc);
		assert!(html.contains("<ul>\n<li>\n- one</li>"), "{html}");
	}

	#[test]
	fn an_empty_list_item_still_closes() {
		let doc = list_doc("•\n• two\n", "<ul>", &[(0, "", 2), (2, "", 2)]);
		let html = render(&doc);
		assert_eq!(html.matches("<li>").count(), html.matches("</li>").count(), "{html}");
		assert!(html.contains("two</li>"), "{html}");
	}

	#[test]
	fn a_formatted_span_at_an_item_start_survives_the_skipped_bullet() {
		let mut markers = vec![Marker::new(MarkerType::Bold, 0.into()).with_length(5.into())];
		markers.push(Marker::new(MarkerType::ListItem, 0.into()).with_prefix_length(2.into()));
		let html = render(&simple_doc("• one\n", markers));
		assert_eq!(html.matches("<b>").count(), html.matches("</b>").count(), "{html}");
	}

	#[test]
	fn later_lines_of_a_list_item_do_not_run_together() {
		let doc = list_doc("• one\ntwo\nthree\n", "<ul>", &[(0, "", 2)]);
		let html = render(&doc);
		assert!(html.contains("two\nthree"), "{html}");
	}

	#[test]
	fn a_preformatted_block_keeps_its_lines_and_indentation() {
		let content = "before\nif a < b:\n\n    go()\nafter";
		let doc = simple_doc(content, vec![Marker::new(MarkerType::Preformatted, 7.into()).with_length(19.into())]);
		let html = render(&doc);
		assert!(html.contains("<p>before</p>\n<pre>\nif a &lt; b:\n\n    go()</pre>\n<p>after</p>"), "{html}");
	}

	#[test]
	fn spans_that_cross_are_closed_and_reopened() {
		let doc = simple_doc(
			"abcdefgh",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Code, 2.into()).with_length(4.into()),
			],
		);
		assert!(render(&doc).contains("<b>ab<code>cd</code></b><code>ef</code>gh"));
	}

	#[test]
	fn a_span_reopened_with_nothing_left_to_cover_leaves_no_empty_tag() {
		let doc = simple_doc(
			"abcd\nefgh",
			vec![
				Marker::new(MarkerType::Code, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Italic, 2.into()).with_length(3.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<code>ab<i>cd</i></code>"), "{html}");
		assert!(!html.contains("<i></i>"), "{html}");
	}

	#[test]
	fn a_span_around_a_whole_block_stays_nested_in_it() {
		let doc = simple_doc(
			"abc\ndef",
			vec![
				Marker::new(MarkerType::Preformatted, 0.into()).with_length(3.into()),
				Marker::new(MarkerType::Italic, 0.into()).with_length(4.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<pre>\n<i>abc</i></pre>\n<p>def</p>"), "{html}");
	}

	#[test]
	fn superscripts_and_subscripts_get_their_tags() {
		let doc = simple_doc(
			"x2 and H2O",
			vec![
				Marker::new(MarkerType::Superscript, 1.into()).with_length(1.into()),
				Marker::new(MarkerType::Subscript, 8.into()).with_length(1.into()),
			],
		);
		assert!(render(&doc).contains("<p>x<sup>2</sup> and H<sub>2</sub>O</p>"));
	}

	#[test]
	fn inline_code_gets_its_tag() {
		let doc = simple_doc("call foo now", vec![Marker::new(MarkerType::Code, 5.into()).with_length(3.into())]);
		assert!(render(&doc).contains("<p>call <code>foo</code> now</p>"));
	}

	#[test]
	fn code_across_lines_writes_no_tag_that_would_straddle_paragraphs() {
		let doc = simple_doc("one\ntwo", vec![Marker::new(MarkerType::Code, 0.into()).with_length(7.into())]);
		let html = render(&doc);
		assert!(!html.contains("<code>"), "{html}");
		assert!(html.contains("<p>one</p>\n<p>two</p>"), "{html}");
	}

	// A whole-document range is what `render` itself asks for, so the two must not drift apart.
	#[test]
	fn render_range_over_the_whole_document_matches_render() {
		let doc = simple_doc(
			"alpha\nbeta\ngamma",
			vec![
				Marker::new(MarkerType::Heading1, 0.into()).with_length(5.into()),
				Marker::new(MarkerType::Bold, 6.into()).with_length(4.into()),
			],
		);
		let total = doc.document().buffer.end();
		assert_eq!(render_range(&doc, 0.into()..total, None), render(&doc));
	}

	#[test]
	fn render_range_renders_only_the_range_and_says_it_is_partial() {
		let doc = simple_doc("alpha\nbeta\ngamma\n", vec![]);
		let html = render_range(&doc, 6.into()..11.into(), None);
		assert!(html.contains("beta"), "{html}");
		assert!(!html.contains("alpha"), "{html}");
		assert!(!html.contains("gamma"), "{html}");
		assert!(html.contains("part of the document"), "a partial view says so: {html}");
	}

	// Anchors outside the range were once flushed at the end of the page, which on a book with a link per paragraph meant every anchor in the book.
	#[test]
	fn render_range_leaves_out_anchors_for_targets_outside_the_range() {
		let doc =
			simple_doc("alpha\nbeta\ngamma\n", vec![Marker::new(MarkerType::Bold, 0.into()).with_length(5.into())]);
		let html = render_range(&doc, 6.into()..11.into(), Some(6.into()));
		assert_eq!(html.matches(r#"<a id=""#).count(), 1, "only the reading position is anchored: {html}");
	}

	#[test]
	fn test_bold_basic() {
		let doc = simple_doc("bold text", vec![Marker::new(MarkerType::Bold, 0.into()).with_length(4.into())]);
		let html = render(&doc);
		assert!(html.contains("<b>bold</b>"), "Expected <b>bold</b> in HTML: {html}");
	}

	#[test]
	fn test_italic_basic() {
		let doc = simple_doc("italic text", vec![Marker::new(MarkerType::Italic, 0.into()).with_length(6.into())]);
		let html = render(&doc);
		assert!(html.contains("<i>italic</i>"), "Expected <i>italic</i> in HTML: {html}");
	}

	#[test]
	fn test_underline_basic() {
		let doc =
			simple_doc("underline text", vec![Marker::new(MarkerType::Underline, 0.into()).with_length(9.into())]);
		let html = render(&doc);
		assert!(html.contains("<u>underline</u>"), "Expected <u>underline</u> in HTML: {html}");
	}

	#[test]
	fn test_nested_bold_italic() {
		let doc = simple_doc(
			"bold italic",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Italic, 5.into()).with_length(6.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<b>bold</b>"), "Expected <b>bold</b> in HTML: {html}");
		assert!(html.contains("<i>italic</i>"), "Expected <i>italic</i> in HTML: {html}");
	}

	#[test]
	fn test_nested_same_start_different_end() {
		let doc = simple_doc(
			"bold italic",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(11.into()),
				Marker::new(MarkerType::Italic, 0.into()).with_length(6.into()),
			],
		);
		let html = render(&doc);
		assert!(html.contains("<b>"), "Expected <b> in HTML: {html}");
		assert!(html.contains("</b>"), "Expected </b> in HTML: {html}");
		assert!(html.contains("<i>"), "Expected <i> in HTML: {html}");
		assert!(html.contains("</i>"), "Expected </i> in HTML: {html}");
	}

	#[test]
	fn test_coincident_end_edge_case() {
		let doc = simple_doc(
			"text",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Italic, 0.into()).with_length(4.into()),
			],
		);
		let html = render(&doc);
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
	}

	#[test]
	fn test_multiple_separate_bold_spans() {
		let doc = simple_doc(
			"bold text normal more bold",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Bold, 22.into()).with_length(4.into()),
			],
		);
		let html = render(&doc);
		let b_open = html.matches("<b>").count();
		let b_close = html.matches("</b>").count();
		assert_eq!(b_open, 2, "Expected 2 <b> opens");
		assert_eq!(b_close, 2, "Expected 2 </b> closes");
	}

	#[test]
	fn test_bold_italic_underline_combined() {
		let doc = simple_doc(
			"text",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(4.into()),
				Marker::new(MarkerType::Italic, 1.into()).with_length(2.into()),
				Marker::new(MarkerType::Underline, 2.into()).with_length(2.into()),
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
