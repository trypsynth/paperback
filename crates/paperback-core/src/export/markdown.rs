use crate::{
	document::{Document, MarkerType},
	util::text::{ch_width, display_len},
};

#[must_use]
pub fn render(doc: &Document) -> String {
	let content = &doc.buffer.content;
	enum Mk {
		Prefix(&'static str),
		LinkOpen,
		LinkClose(String),
		BoldOpen,
		BoldClose,
		ItalicOpen,
		ItalicClose,
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
			MarkerType::Bold => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Mk::BoldOpen });
				events.push(Ev { pos: end, kind: Mk::BoldClose });
			}
			MarkerType::Italic => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Mk::ItalicOpen });
				events.push(Ev { pos: end, kind: Mk::ItalicClose });
			}
			MarkerType::Separator => {
				// Replace the dash line written into the content by html_to_text with "---"
				events.push(Ev { pos, kind: Mk::Replace { until: pos + marker.length } });
			}
			// MarkerType::Underline intentionally has no arm: CommonMark has no native
			// underline construct; falls through to the `_` arm below and is silently
			// dropped from markdown output rather than emitting raw HTML `<u>`.
			_ => {}
		}
	}
	// At equal positions: close before replace before prefix before open
	events.sort_by(|a, b| {
		a.pos.cmp(&b.pos).then_with(|| {
			let p = |k: &Mk| match k {
				Mk::LinkClose(_) | Mk::BoldClose | Mk::ItalicClose => 0u8,
				Mk::Replace { .. } => 1,
				Mk::Prefix(_) => 2,
				Mk::LinkOpen | Mk::BoldOpen | Mk::ItalicOpen => 3,
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
				Mk::BoldOpen | Mk::BoldClose => md.push_str("**"),
				Mk::ItalicOpen | Mk::ItalicClose => md.push('*'),
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
	// Flush any closes that fall at or past end of content
	while event_idx < events.len() {
		match &events[event_idx].kind {
			Mk::LinkClose(url) => {
				md.push(']');
				if !url.is_empty() {
					md.push('(');
					md.push_str(url);
					md.push(')');
				}
			}
			Mk::BoldClose => md.push_str("**"),
			Mk::ItalicClose => md.push('*'),
			_ => {}
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker, MarkerType};

	fn simple_doc(content: &str, markers: Vec<Marker>) -> Document {
		let mut buffer = DocumentBuffer::with_content(content.to_string());
		for marker in markers {
			buffer.add_marker(marker);
		}
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc
	}

	#[test]
	fn test_bold_basic() {
		let doc = simple_doc("bold text", vec![Marker::new(MarkerType::Bold, 0).with_length(4)]);
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown: {}", md);
	}

	#[test]
	fn test_italic_basic() {
		let doc = simple_doc("italic text", vec![Marker::new(MarkerType::Italic, 0).with_length(6)]);
		let md = render(&doc);
		assert!(md.contains("*italic*"), "Expected *italic* in markdown: {}", md);
	}

	#[test]
	fn test_underline_produces_no_syntax() {
		let doc = simple_doc("underline text", vec![Marker::new(MarkerType::Underline, 0).with_length(9)]);
		let md = render(&doc);
		// The text should still appear, but without any markdown syntax
		assert!(md.contains("underline"), "Expected text 'underline' in markdown: {}", md);
		// No underline syntax should be present
		assert!(!md.contains("__"), "Expected no __ syntax for underline");
		assert!(!md.contains("_underline_"), "Expected no _underline_ italic-like syntax");
		// No HTML-like underline either
		assert!(!md.contains("<u>"), "Expected no <u> HTML in markdown output");
	}

	#[test]
	fn test_nested_bold_italic() {
		// "bold italic" where 0-4 is bold, 5-11 is italic
		let doc = simple_doc(
			"bold italic",
			vec![Marker::new(MarkerType::Bold, 0).with_length(4), Marker::new(MarkerType::Italic, 5).with_length(6)],
		);
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown: {}", md);
		assert!(md.contains("*italic*"), "Expected *italic* in markdown: {}", md);
	}

	#[test]
	fn test_bold_close_at_end_of_content() {
		// Test that a bold span ending exactly at end-of-content doesn't lose its closing **
		let doc = simple_doc("bold", vec![Marker::new(MarkerType::Bold, 0).with_length(4)]);
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown with closing **: {}", md);
		// Count ** to verify both open and close are present
		let count = md.matches("**").count();
		assert_eq!(count, 2, "Expected exactly 2 occurrences of ** (open and close)");
	}

	#[test]
	fn test_italic_close_at_end_of_content() {
		// Test that an italic span ending exactly at end-of-content doesn't lose its closing *
		let doc = simple_doc("italic", vec![Marker::new(MarkerType::Italic, 0).with_length(6)]);
		let md = render(&doc);
		assert!(md.contains("*italic*"), "Expected *italic* in markdown with closing *: {}", md);
		// Count * to verify both open and close are present (should be 2)
		let count = md.matches("*").count();
		assert_eq!(count, 2, "Expected exactly 2 occurrences of * (open and close)");
	}
}
