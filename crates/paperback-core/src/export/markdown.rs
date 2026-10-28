use crate::document::{DocLen, DocPos, Document, MarkerType};

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
		Replace { until: DocPos },
	}
	struct Ev {
		pos: DocPos,
		kind: Mk,
		/// Closing in the reverse of opening order keeps two spans over the same text nested rather than interleaved.
		span: usize,
	}
	let mut events: Vec<Ev> = Vec::new();
	for (span, marker) in doc.buffer.markers.iter().enumerate() {
		let pos = marker.position;
		match marker.mtype {
			MarkerType::Heading1 => events.push(Ev { pos, kind: Mk::Prefix("# "), span }),
			MarkerType::Heading2 => events.push(Ev { pos, kind: Mk::Prefix("## "), span }),
			MarkerType::Heading3 => events.push(Ev { pos, kind: Mk::Prefix("### "), span }),
			MarkerType::Heading4 => events.push(Ev { pos, kind: Mk::Prefix("#### "), span }),
			MarkerType::Heading5 => events.push(Ev { pos, kind: Mk::Prefix("##### "), span }),
			MarkerType::Heading6 => events.push(Ev { pos, kind: Mk::Prefix("###### "), span }),
			MarkerType::Link => {
				let Some(end) = super::link_span_end(marker) else { continue };
				events.push(Ev { pos, kind: Mk::LinkOpen, span });
				events.push(Ev { pos: end, kind: Mk::LinkClose(marker.reference.clone()), span });
			}
			MarkerType::Bold => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Mk::BoldOpen, span });
				events.push(Ev { pos: end, kind: Mk::BoldClose, span });
			}
			MarkerType::Italic => {
				let end = pos + marker.length;
				events.push(Ev { pos, kind: Mk::ItalicOpen, span });
				events.push(Ev { pos: end, kind: Mk::ItalicClose, span });
			}
			MarkerType::Separator => {
				events.push(Ev { pos, kind: Mk::Replace { until: pos + marker.length }, span });
			}
			// Underline is dropped: CommonMark has no underline, and raw `<u>` is worse than none.
			_ => {}
		}
	}
	// Spans starting together close in reverse, so a bold link is `**[text](url)**`, not `[**text](url)**`.
	events.sort_by(|a, b| {
		let class = |k: &Mk| match k {
			Mk::LinkClose(_) | Mk::BoldClose | Mk::ItalicClose => 0u8,
			Mk::Replace { .. } => 1,
			Mk::Prefix(_) => 2,
			Mk::LinkOpen | Mk::BoldOpen | Mk::ItalicOpen => 3,
		};
		let (ca, cb) = (class(&a.kind), class(&b.kind));
		a.pos
			.cmp(&b.pos)
			.then_with(|| ca.cmp(&cb))
			.then_with(|| if ca == 0 { b.span.cmp(&a.span) } else { a.span.cmp(&b.span) })
	});
	let mut md = String::with_capacity(content.len() + events.len() * 4);
	let mut event_idx = 0usize;
	let mut position = DocPos::ZERO;
	let mut skip_until: Option<DocPos> = None;
	for ch in content.chars() {
		while event_idx < events.len() && events[event_idx].pos <= position {
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
					// The leading newline keeps "---" from being read as a setext heading underline.
					md.push_str("\n---\n");
					skip_until = Some(*until);
				}
			}
			event_idx += 1;
		}
		if let Some(until) = skip_until {
			if position < until {
				position += DocLen::ONE;
				continue;
			}
			skip_until = None;
		}
		md.push(ch);
		position += DocLen::ONE;
	}
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
	normalize_newlines(&md)
}

// Markdown ignores single newlines, so every break becomes a blank line.
fn normalize_newlines(s: &str) -> String {
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
		let doc = simple_doc("bold text", vec![Marker::new(MarkerType::Bold, 0.into()).with_length(4.into())]);
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown: {md}");
	}

	#[test]
	fn test_italic_basic() {
		let doc = simple_doc("italic text", vec![Marker::new(MarkerType::Italic, 0.into()).with_length(6.into())]);
		let md = render(&doc);
		assert!(md.contains("*italic*"), "Expected *italic* in markdown: {md}");
	}

	#[test]
	fn test_underline_produces_no_syntax() {
		let doc =
			simple_doc("underline text", vec![Marker::new(MarkerType::Underline, 0.into()).with_length(9.into())]);
		let md = render(&doc);
		assert!(md.contains("underline"), "Expected text 'underline' in markdown: {md}");
		assert!(!md.contains("__"), "Expected no __ syntax for underline");
		assert!(!md.contains("_underline_"), "Expected no _underline_ italic-like syntax");
		assert!(!md.contains("<u>"), "Expected no <u> HTML in markdown output");
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
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown: {md}");
		assert!(md.contains("*italic*"), "Expected *italic* in markdown: {md}");
	}

	// A bold hotspot gives two spans over the same characters, and interleaving them is no bold link to any renderer.
	#[test]
	fn a_bold_link_nests_rather_than_interleaving() {
		let doc = simple_doc(
			"define",
			vec![
				Marker::new(MarkerType::Bold, 0.into()).with_length(6.into()),
				Marker::new(MarkerType::Link, 0.into())
					.with_text("define".to_string())
					.with_reference("topic7".to_string()),
			],
		);
		assert_eq!(render(&doc).trim(), "**[define](topic7)**");
	}

	#[test]
	fn a_linked_bold_run_nests_the_other_way_round() {
		let doc = simple_doc(
			"define",
			vec![
				Marker::new(MarkerType::Link, 0.into())
					.with_text("define".to_string())
					.with_reference("topic7".to_string()),
				Marker::new(MarkerType::Bold, 0.into()).with_length(6.into()),
			],
		);
		assert_eq!(render(&doc).trim(), "[**define**](topic7)");
	}

	#[test]
	fn test_bold_close_at_end_of_content() {
		let doc = simple_doc("bold", vec![Marker::new(MarkerType::Bold, 0.into()).with_length(4.into())]);
		let md = render(&doc);
		assert!(md.contains("**bold**"), "Expected **bold** in markdown with closing **: {md}");
		let count = md.matches("**").count();
		assert_eq!(count, 2, "Expected exactly 2 occurrences of ** (open and close)");
	}

	#[test]
	fn test_italic_close_at_end_of_content() {
		let doc = simple_doc("italic", vec![Marker::new(MarkerType::Italic, 0.into()).with_length(6.into())]);
		let md = render(&doc);
		assert!(md.contains("*italic*"), "Expected *italic* in markdown with closing *: {md}");
		let count = md.matches('*').count();
		assert_eq!(count, 2, "Expected exactly 2 occurrences of * (open and close)");
	}
}
