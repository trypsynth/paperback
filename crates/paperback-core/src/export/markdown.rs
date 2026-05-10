use crate::{
	document::{Document, MarkerType},
	util::text::{ch_width, display_len},
};

pub fn render(doc: &Document) -> String {
	let content = &doc.buffer.content;
	enum Mk {
		Prefix(&'static str),
		LinkOpen,
		LinkClose(String),
		// Replace `until` display-units of content with "\n---\n"
		Replace { until: usize },
	}
	struct Ev {
		pos: usize,
		kind: Mk,
	}
	let mut events: Vec<Ev> = Vec::new();
	for marker in &doc.buffer.markers {
		let pos = marker.position;
		match marker.mtype {
			MarkerType::Heading1 => events.push(Ev { pos, kind: Mk::Prefix("# ") }),
			MarkerType::Heading2 => events.push(Ev { pos, kind: Mk::Prefix("## ") }),
			MarkerType::Heading3 => events.push(Ev { pos, kind: Mk::Prefix("### ") }),
			MarkerType::Heading4 => events.push(Ev { pos, kind: Mk::Prefix("#### ") }),
			MarkerType::Heading5 => events.push(Ev { pos, kind: Mk::Prefix("##### ") }),
			MarkerType::Heading6 => events.push(Ev { pos, kind: Mk::Prefix("###### ") }),
			MarkerType::Link => {
				let text: String = marker.text.split_whitespace().collect::<Vec<_>>().join(" ");
				let implied_len = if marker.length > 0 { marker.length } else { display_len(&text) };
				if implied_len == 0 {
					continue;
				}
				let end = pos + implied_len;
				events.push(Ev { pos, kind: Mk::LinkOpen });
				events.push(Ev { pos: end, kind: Mk::LinkClose(marker.reference.clone()) });
			}
			MarkerType::Separator => {
				// Replace the dash line written into the content by html_to_text with "---"
				events.push(Ev { pos, kind: Mk::Replace { until: pos + marker.length } });
			}
			_ => {}
		}
	}
	// At equal positions: close before replace before prefix before open
	events.sort_by(|a, b| {
		a.pos.cmp(&b.pos).then_with(|| {
			let p = |k: &Mk| match k {
				Mk::LinkClose(_) => 0u8,
				Mk::Replace { .. } => 1,
				Mk::Prefix(_) => 2,
				Mk::LinkOpen => 3,
			};
			p(&a.kind).cmp(&p(&b.kind))
		})
	});
	let mut md = String::with_capacity(content.len() + events.len() * 4);
	let mut event_idx = 0usize;
	let mut display_pos = 0usize;
	let mut skip_until: Option<usize> = None;
	for ch in content.chars() {
		while event_idx < events.len() && events[event_idx].pos <= display_pos {
			if skip_until.is_some_and(|u| events[event_idx].pos < u) {
				event_idx += 1;
				continue;
			}
			match &events[event_idx].kind {
				Mk::Prefix(p) => md.push_str(p),
				Mk::LinkOpen => md.push('['),
				Mk::LinkClose(url) => {
					md.push(']');
					if !url.is_empty() {
						md.push('(');
						md.push_str(url);
						md.push(')');
					}
				}
				Mk::Replace { until } => {
					// Leading newline ensures "---" is never parsed as a setext heading underline
					md.push_str("\n---\n");
					skip_until = Some(*until);
				}
			}
			event_idx += 1;
		}
		if let Some(until) = skip_until {
			if display_pos < until {
				display_pos += ch_width(ch);
				continue;
			}
			skip_until = None;
		}
		md.push(ch);
		display_pos += ch_width(ch);
	}
	// Flush any link closes that fall at or past end of content
	while event_idx < events.len() {
		if let Mk::LinkClose(url) = &events[event_idx].kind {
			md.push(']');
			if !url.is_empty() {
				md.push('(');
				md.push_str(url);
				md.push(')');
			}
		}
		event_idx += 1;
	}
	normalize_newlines(md)
}

// Expand single newlines to blank lines and collapse runs of 3+ newlines to 2.
// Markdown ignores single newlines; a blank line is required for paragraph breaks.
fn normalize_newlines(s: String) -> String {
	let mut out = String::with_capacity(s.len() + s.len() / 4);
	let mut nl_run = 0usize;
	for ch in s.chars() {
		if ch == '\n' {
			nl_run += 1;
		} else {
			if nl_run > 0 {
				out.push_str("\n\n");
			}
			nl_run = 0;
			out.push(ch);
		}
	}
	if nl_run > 0 {
		out.push('\n');
	}
	out
}
