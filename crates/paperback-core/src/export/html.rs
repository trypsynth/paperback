use std::collections::{HashMap, HashSet};

use crate::{
	document::{DocumentHandle, MarkerType},
	parser::is_external_url,
	util::text::{ch_width, display_len},
};

pub fn render(doc: &DocumentHandle) -> String {
	let document = doc.document();
	let content = &document.buffer.content;

	// Precompute section boundaries once so link resolution is O(log S) per link
	// instead of O(M) per link (where M = total marker count).
	let section_break_positions: Vec<usize> =
		document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::SectionBreak).map(|m| m.position).collect();
	// Single O(N) scan: collect newline positions in display coordinates and total display length.
	// Used to replace the O(N)-per-call line_end_pos() with an O(log lines) binary search.
	let (newline_display_positions, content_display_len): (Vec<usize>, usize) = {
		let mut positions = Vec::new();
		let mut dpos = 0usize;
		for ch in content.chars() {
			if ch == '\n' {
				positions.push(dpos);
			}
			dpos += ch_width(ch);
		}
		(positions, dpos)
	};
	let newline_from = |start: usize| -> usize {
		let idx = newline_display_positions.partition_point(|&p| p < start);
		newline_display_positions.get(idx).copied().unwrap_or(content_display_len)
	};
	// path → (section_start, section_end)
	let path_to_bounds: HashMap<&str, (usize, usize)> = document
		.spine_items
		.iter()
		.enumerate()
		.filter_map(|(i, manifest_id)| {
			let path = document.manifest_items.get(manifest_id)?;
			let start = section_break_positions.get(i).copied().unwrap_or(0);
			let end = section_break_positions.get(i + 1).copied().unwrap_or(content_display_len);
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
	for marker in &document.buffer.markers {
		let pos = marker.position;
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
				// Link length is not stored; recover it from the link text written into
				// the content (collapse_whitespace was applied when the text was stored).
				let text: String = marker.text.split_whitespace().collect::<Vec<_>>().join(" ");
				let implied_len = if marker.length > 0 { marker.length } else { display_len(&text) };
				if implied_len == 0 {
					continue;
				}
				let end = pos + implied_len;
				let open = if marker.reference.is_empty() {
					"<a>".to_string()
				} else {
					let href = marker.reference.trim();
					if is_external_url(href) {
						format!("<a href=\"{}\">", escape_attr(href))
					} else if let Some(fragment) = href.strip_prefix('#') {
						let current_path = section_path_at(pos);
						if let Some(off) = resolve_fragment(&document.id_positions, fragment, current_path) {
							target_offsets.insert(off);
							format!("<a href=\"#pos-{off}\">")
						} else {
							format!("<a href=\"{}\">", escape_attr(href))
						}
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
							let off = if !frag_part.is_empty() {
								resolve_fragment(&document.id_positions, frag_part, Some(file_part))
									.or_else(|| document.id_positions.get(file_part).copied())
									.or_else(|| resolve_fragment(&document.id_positions, frag_part, current_path))
							} else {
								document.id_positions.get(file_part).copied()
							};
							if let Some(off) = off {
								target_offsets.insert(off);
								format!("<a href=\"#pos-{off}\">")
							} else {
								format!("<a href=\"{}\">", escape_attr(href))
							}
						}
					}
				};
				events.push(Ev { pos, kind: Ek::InlineOpen(open) });
				events.push(Ev { pos: end, kind: Ek::InlineClose("</a>") });
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
	for offset in target_offsets {
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
	let mut display_pos: usize = 0;
	let mut skip_until: Option<usize> = None;
	for ch in content.chars() {
		// Fire events whose position has been reached
		while event_idx < events.len() && events[event_idx].pos <= display_pos {
			// Suppress events that fall inside an active replace range
			if skip_until.is_some_and(|u| events[event_idx].pos < u) {
				if !matches!(events[event_idx].kind, Ek::Anchor(_)) {
					event_idx += 1;
					continue;
				}
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
					html.push_str(&format!("<a id=\"pos-{offset}\"></a>"));
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
				html.push_str(&format!("<a id=\"pos-{offset}\"></a>"));
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

fn push_escaped(ch: char, out: &mut String) {
	match ch {
		'&' => out.push_str("&amp;"),
		'<' => out.push_str("&lt;"),
		'>' => out.push_str("&gt;"),
		c => out.push(c),
	}
}

fn escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for ch in s.chars() {
		push_escaped(ch, &mut out);
	}
	out
}

fn escape_attr(s: &str) -> String {
	s.replace('&', "&amp;").replace('"', "&quot;")
}
