//! The `.paperback` sidecar only moves bookmarks and notes between installs and cannot be read on its own, so this is the listing for reading them on a machine that has no Paperback or book.

use std::fmt::Write;

use crate::util::html::escape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationFormat {
	Text,
	Html,
}

#[derive(Debug, Clone, Copy)]
pub struct AnnotationOptions {
	pub include_passage: bool,
	pub numbered: bool,
}

#[derive(Debug, Clone)]
pub struct AnnotationEntry {
	pub note: String,
	pub passage: String,
	pub audio_ms: Option<u64>,
}

fn clock(ms: u64) -> String {
	let secs = ms / 1000;
	let (h, m, s) = (secs / 3600, secs / 60 % 60, secs % 60);
	if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
}

fn one_line(s: &str) -> String {
	s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A bookmark with no note would otherwise be an empty row, so its passage is always shown.
fn shown_passage(entry: &AnnotationEntry, options: AnnotationOptions) -> String {
	if options.include_passage || entry.note.is_empty() { one_line(&entry.passage) } else { String::new() }
}

#[must_use]
pub fn render_annotations(
	title: &str,
	entries: &[AnnotationEntry],
	options: AnnotationOptions,
	format: AnnotationFormat,
) -> String {
	match format {
		AnnotationFormat::Text => render_text(title, entries, options),
		AnnotationFormat::Html => render_html(title, entries, options),
	}
}

fn render_text(title: &str, entries: &[AnnotationEntry], options: AnnotationOptions) -> String {
	let mut out = format!("{title}\n");
	for (i, entry) in entries.iter().enumerate() {
		let mut lead = if options.numbered { format!("{}. ", i + 1) } else { String::new() };
		if let Some(ms) = entry.audio_ms {
			let _ = write!(lead, "[{}] ", clock(ms));
		}
		let indent = " ".repeat(if options.numbered { (i + 1).to_string().len() + 2 } else { 0 });
		let passage = shown_passage(entry, options);
		let quoted = (!passage.is_empty()).then(|| format!("\"{passage}\""));
		let lines: Vec<&str> = quoted.as_deref().into_iter().chain(entry.note.lines()).collect();
		out.push('\n');
		if lines.is_empty() {
			let _ = writeln!(out, "{}", lead.trim_end());
		}
		for (n, line) in lines.iter().enumerate() {
			let _ = writeln!(out, "{}{line}", if n == 0 { &lead } else { &indent });
		}
	}
	out
}

fn render_html(title: &str, entries: &[AnnotationEntry], options: AnnotationOptions) -> String {
	let title = escape(title);
	let list = if options.numbered { "ol" } else { "ul" };
	let mut out = format!(
		"<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>{title}</title>\n</head>\n<body>\n<h1>{title}</h1>\n<{list}>\n"
	);
	for entry in entries {
		out.push_str("<li>");
		if let Some(ms) = entry.audio_ms {
			let _ = write!(out, "<p>[{}]</p>", clock(ms));
		}
		let passage = shown_passage(entry, options);
		if !passage.is_empty() {
			let _ = write!(out, "<blockquote>{}</blockquote>", escape(&passage));
		}
		if !entry.note.is_empty() {
			let _ = write!(out, "<p style=\"white-space: pre-wrap\">{}</p>", escape(&entry.note));
		}
		out.push_str("</li>\n");
	}
	let _ = write!(out, "</{list}>\n</body>\n</html>\n");
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	fn entry(note: &str, passage: &str) -> AnnotationEntry {
		AnnotationEntry { note: note.into(), passage: passage.into(), audio_ms: None }
	}

	const PLAIN: AnnotationOptions = AnnotationOptions { include_passage: true, numbered: false };
	const NUMBERED: AnnotationOptions = AnnotationOptions { include_passage: true, numbered: true };

	#[test]
	fn text_numbers_and_shows_passage_with_note() {
		let out = render_annotations("Book", &[entry("my note", "the  passage")], NUMBERED, AnnotationFormat::Text);
		assert_eq!(out, "Book\n\n1. \"the passage\"\n   my note\n");
	}

	#[test]
	fn text_without_passage_puts_note_first() {
		let opts = AnnotationOptions { include_passage: false, numbered: false };
		let out = render_annotations("Book", &[entry("only note", "hidden")], opts, AnnotationFormat::Text);
		assert_eq!(out, "Book\n\nonly note\n");
	}

	#[test]
	fn bookmark_without_note_always_shows_passage() {
		let opts = AnnotationOptions { include_passage: false, numbered: false };
		let out = render_annotations("Book", &[entry("", "line")], opts, AnnotationFormat::Text);
		assert_eq!(out, "Book\n\n\"line\"\n");
	}

	#[test]
	fn audio_entry_shows_clock() {
		let e = AnnotationEntry { note: "n".into(), passage: String::new(), audio_ms: Some(3_725_000) };
		let out = render_annotations("Book", &[e], PLAIN, AnnotationFormat::Text);
		assert_eq!(out, "Book\n\n[1:02:05] n\n");
	}

	#[test]
	fn empty_list_is_title_only() {
		assert_eq!(render_annotations("Book", &[], PLAIN, AnnotationFormat::Text), "Book\n");
	}

	#[test]
	fn html_escapes_and_picks_list_type() {
		let out = render_annotations("A & B", &[entry("<b>", "x < y")], NUMBERED, AnnotationFormat::Html);
		assert!(out.contains("<title>A &amp; B</title>"));
		assert!(out.contains("<ol>") && out.contains("</ol>"));
		assert!(out.contains("<blockquote>x &lt; y</blockquote>"));
		assert!(out.contains("&lt;b&gt;"));
		let plain = render_annotations("T", &[entry("n", "p")], PLAIN, AnnotationFormat::Html);
		assert!(plain.contains("<ul>"));
	}
}
