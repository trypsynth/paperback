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

fn document_to_html(doc: &Document) -> String {
	let content = &doc.buffer.content;
	let mut html = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
	let mut events: Vec<(usize, &str)> = Vec::new();
	for marker in &doc.buffer.markers {
		let pos = marker.position;
		let end = pos + marker.length;
		let (open, close) = match marker.mtype {
			MarkerType::Heading1 => ("<h1>", "</h1>"),
			MarkerType::Heading2 => ("<h2>", "</h2>"),
			MarkerType::Heading3 => ("<h3>", "</h3>"),
			MarkerType::Heading4 => ("<h4>", "</h4>"),
			MarkerType::Heading5 => ("<h5>", "</h5>"),
			MarkerType::Heading6 => ("<h6>", "</h6>"),
			MarkerType::Link => ("<a>", "</a>"),
			MarkerType::List => ("<ul>", "</ul>"),
			MarkerType::ListItem => ("<li>", "</li>"),
			MarkerType::Table => ("<table>", "</table>"),
			MarkerType::PageBreak | MarkerType::SectionBreak | MarkerType::Separator => {
				events.push((pos, "<hr>"));
				continue;
			}
			_ => continue,
		};
		events.push((pos, open));
		events.push((end, close));
	}
	events.sort_by_key(|e| e.0);
	let mut event_idx = 0;
	for (i, ch) in content.chars().enumerate() {
		while event_idx < events.len() && events[event_idx].0 <= i {
			html.push_str(events[event_idx].1);
			event_idx += 1;
		}
		match ch {
			'&' => html.push_str("&amp;"),
			'<' => html.push_str("&lt;"),
			'>' => html.push_str("&gt;"),
			'\n' => html.push_str("<br>\n"),
			c => html.push(c),
		}
	}
	while event_idx < events.len() {
		html.push_str(events[event_idx].1);
		event_idx += 1;
	}
	html.push_str("\n</body>\n</html>\n");
	html
}
