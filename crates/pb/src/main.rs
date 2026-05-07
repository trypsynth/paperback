use std::fs;

use anyhow::{Context, Result, bail};
use clap::Parser;
use paperback_core::{
	document::{Document, ParserContext},
	parser::{self, PASSWORD_REQUIRED_ERROR_PREFIX, parse_document},
};

mod cli;
mod html;

use cli::{Cli, Format};

fn main() -> Result<()> {
	let cli = Cli::parse();
	let ext = cli.input.extension().and_then(|e| e.to_str()).unwrap_or("");
	if !parser::parser_supports_extension(ext) {
		bail!("unsupported file format: .{ext}");
	}
	let file_path = cli.input.to_string_lossy().into_owned();
	let mut context = ParserContext { file_path, password: cli.password, forced_extension: None };
	let doc = match parse_document(&context) {
		Ok(doc) => doc,
		Err(e) if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
			let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
			context.password = Some(password);
			parse_document(&context).with_context(|| format!("failed to parse {}", cli.input.display()))?
		}
		Err(e) => return Err(e.context(format!("failed to parse {}", cli.input.display()))),
	};
	let result = if cli.metadata {
		metadata(&doc)
	} else {
		match cli.format {
			Format::Text => doc.buffer.content.clone(),
			Format::Html => html::document_to_html(&doc),
		}
	};
	match cli.output {
		Some(path) => fs::write(&path, &result).with_context(|| format!("failed to write {}", path.display())),
		None => Ok(print!("{result}")),
	}
}

fn metadata(doc: &Document) -> String {
	let mut out = String::new();
	if !doc.title.is_empty() {
		out.push_str(&format!("Title: {}\n", doc.title));
	}
	if !doc.author.is_empty() {
		out.push_str(&format!("Author: {}\n", doc.author));
	}
	out.push_str(&format!("Words: {}\n", doc.stats.word_count));
	out.push_str(&format!("Characters: {}\n", doc.stats.char_count));
	out.push_str(&format!("Lines: {}\n", doc.stats.line_count));
	out
}
