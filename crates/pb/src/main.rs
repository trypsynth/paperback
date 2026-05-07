use std::{fs, path::PathBuf, process};

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use paperback_core::{
	document::{Document, MarkerType, ParserContext},
	parser::{self, parse_document},
};

#[derive(Parser)]
#[command(name = "pb", about = "Convert a document to text or HTML")]
struct Cli {
	/// Input document file
	input: PathBuf,
	/// Output format
	#[arg(short, long, default_value = "text")]
	format: Format,
	/// Write output to a file instead of stdout
	#[arg(short, long)]
	output: Option<PathBuf>,
}

#[derive(Clone, ValueEnum)]
enum Format {
	Text,
	Html,
}

fn main() {
	if let Err(e) = run() {
		eprintln!("pb: {e}");
		process::exit(1);
	}
}

fn run() -> Result<()> {
	let cli = Cli::parse();
	let ext = cli.input.extension().and_then(|e| e.to_str()).unwrap_or("");
	if !parser::parser_supports_extension(ext) {
		bail!("unsupported file format: .{ext}");
	}
	let context =
		ParserContext { file_path: cli.input.to_string_lossy().into_owned(), password: None, forced_extension: None };
	let doc = parse_document(&context).with_context(|| format!("failed to parse {}", cli.input.display()))?;
	let result = match cli.format {
		Format::Text => doc.buffer.content.clone(),
		Format::Html => document_to_html(&doc),
	};
	match cli.output {
		Some(path) => {
			fs::write(&path, &result).with_context(|| format!("failed to write {}", path.display()))?;
		}
		None => print!("{result}"),
	}
	Ok(())
}

fn push_escaped(ch: char, out: &mut String) {
	match ch {
		'&' => out.push_str("&amp;"),
		'<' => out.push_str("&lt;"),
		'>' => out.push_str("&gt;"),
		c => out.push(c),
	}
}

fn html_escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for ch in s.chars() {
		push_escaped(ch, &mut out);
	}
	out
}

fn html_escape_attr(s: &str) -> String {
	s.replace('&', "&amp;").replace('"', "&quot;")
}

#[cfg(windows)]
fn ch_width(ch: char) -> usize {
	ch.len_utf16()
}
#[cfg(not(windows))]
fn ch_width(_ch: char) -> usize {
	1
}

#[cfg(windows)]
fn str_display_len(s: &str) -> usize {
	s.encode_utf16().count()
}
#[cfg(not(windows))]
fn str_display_len(s: &str) -> usize {
	s.chars().count()
}

// Return the display position of the '\n' that ends the line beginning at `start`,
// or the content end position if there is no such newline.
fn line_end_pos(content: &str, start: usize) -> usize {
	let mut pos = 0usize;
	for ch in content.chars() {
		if pos >= start && ch == '\n' {
			return pos;
		}
		pos += ch_width(ch);
	}
	pos
}

fn document_to_html(doc: &Document) -> String {
	let content = &doc.buffer.content;
	let mut html = format!(
		"<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n</head>\n<body>\n",
		html_escape(&doc.title)
	);
	enum Ek {
		BlockOpen(&'static str),
		BlockClose(&'static str),
		InlineOpen(String),
		InlineClose(&'static str),
		Hr,
		Replace { until: usize, content: String },
	}
	struct Ev {
		pos: usize,
		kind: Ek,
	}
	let mut events: Vec<Ev> = Vec::new();
	for marker in &doc.buffer.markers {
		let pos = marker.position;
		// Markers from html_to_text carry length=0 for headings, links, and list items
		// because those types store their span only implicitly in the content.
		// Recover the span: for block elements scan to the next '\n'; for inline links
		// use the display length of the link text that was written into the content.
		let effective_end =
			|explicit: usize| -> usize { if explicit > 0 { pos + explicit } else { line_end_pos(content, pos) } };
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
				let implied_len = if marker.length > 0 { marker.length } else { str_display_len(&text) };
				if implied_len == 0 {
					continue;
				}
				let end = pos + implied_len;
				let open = if marker.reference.is_empty() {
					"<a>".to_string()
				} else {
					format!("<a href=\"{}\">", html_escape_attr(&marker.reference))
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
	// Closes before opens at the same position to avoid empty elements
	events.sort_by(|a, b| {
		a.pos.cmp(&b.pos).then_with(|| {
			let p = |k: &Ek| match k {
				Ek::BlockClose(_) | Ek::InlineClose(_) => 0u8,
				Ek::Hr | Ek::Replace { .. } => 1,
				Ek::InlineOpen(_) => 2,
				Ek::BlockOpen(_) => 3,
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
						if pending_newlines >= 1 {
							if in_para {
								html.push_str("</p>\n");
								in_para = false;
							}
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
			}
			event_idx += 1;
		}
		// Skip chars that belong to a replaced range (e.g. table inline text)
		if let Some(until) = skip_until {
			if display_pos < until {
				#[cfg(windows)]
				{
					display_pos += ch.len_utf16();
				}
				#[cfg(not(windows))]
				{
					display_pos += 1;
				}
				continue;
			}
			skip_until = None;
		}
		if block_depth == 0 {
			// Free-text mode: detect paragraph boundaries on double newlines
			if ch == '\n' {
				pending_newlines += 1;
			} else {
				if pending_newlines >= 1 {
					if in_para {
						html.push_str("</p>\n");
						in_para = false;
					}
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
		#[cfg(windows)]
		{
			display_pos += ch.len_utf16();
		}
		#[cfg(not(windows))]
		{
			display_pos += 1;
		}
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
