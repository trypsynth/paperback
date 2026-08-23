use std::{fmt::Write as _, fs, process};

use anyhow::{Context, Result, bail};
use clap::Parser;
use paperback_core::{
	document::{Document, ParserContext},
	export::{self, ExportFormat},
	parser::{self, PASSWORD_REQUIRED_ERROR_PREFIX, parse_document},
};

mod cli;

use cli::{Cli, Format};

fn main() -> Result<()> {
	init_logging();
	let cli = Cli::parse();
	let ext = cli.input.extension().and_then(|e| e.to_str()).unwrap_or("");
	if !parser::parser_supports_extension(ext) {
		bail!("unsupported file format: .{ext}");
	}
	let file_path = cli.input.to_string_lossy().into_owned();
	if !cli.metadata && matches!(cli.format, Format::Html) && ext == "epub" {
		let html = export::epub_direct::render(&file_path)
			.with_context(|| format!("failed to convert {}", cli.input.display()))?;
		// map_or_else reads worse here than the plain if/else.
		#[allow(clippy::option_if_let_else)]
		return if let Some(path) = cli.output {
			fs::write(&path, &html).with_context(|| format!("failed to write {}", path.display()))
		} else {
			print!("{html}");
			Ok(())
		};
	}
	let mut context = ParserContext::new(file_path).with_render_tables_inline(true);
	if let Some(password) = cli.password {
		context = context.with_password(password);
	}
	let doc = match parse_document(&context) {
		Ok(doc) => doc,
		Err(e) if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
			if cli.no_prompt {
				eprintln!("pb: document requires a password; skipping (use -p to supply one)");
				process::exit(2);
			}
			let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
			context.password = Some(password);
			parse_document(&context).with_context(|| format!("failed to parse {}", cli.input.display()))?
		}
		Err(e) => return Err(e.context(format!("failed to parse {}", cli.input.display()))),
	};
	let handle = paperback_core::document::DocumentHandle::new(doc);
	let is_markdown = !cli.metadata && matches!(cli.format, Format::Markdown);
	let result = if cli.metadata {
		metadata(handle.document())
	} else {
		let format = match cli.format {
			Format::Text => ExportFormat::Text,
			Format::Html => ExportFormat::Html,
			Format::Markdown => ExportFormat::Markdown,
		};
		export::render(&handle, format)
	};
	// map_or_else reads worse here than the plain if/else.
	#[allow(clippy::option_if_let_else)]
	if let Some(path) = cli.output {
		if is_markdown {
			// Prepend UTF-8 BOM so editors like EdSharp detect the encoding correctly
			let mut bytes = vec![0xEF_u8, 0xBB, 0xBF];
			bytes.extend_from_slice(result.as_bytes());
			fs::write(&path, &bytes)
		} else {
			fs::write(&path, &result)
		}
		.with_context(|| format!("failed to write {}", path.display()))
	} else {
		print!("{result}");
		Ok(())
	}
}

/// Prints paperback-core's tracing output to stderr so parser logging is visible when testing
/// with this tool. Defaults to debug level for paperback-core; override with `RUST_LOG`.
fn init_logging() {
	let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "paperback_core=debug".to_string());
	tracing_subscriber::fmt().with_env_filter(filter).with_writer(std::io::stderr).with_target(false).init();
}

fn metadata(doc: &Document) -> String {
	let mut out = String::new();
	if !doc.title.is_empty() {
		let _ = writeln!(out, "Title: {}", doc.title);
	}
	if !doc.author.is_empty() {
		let _ = writeln!(out, "Author: {}", doc.author);
	}
	let _ = writeln!(out, "Words: {}", doc.stats.word_count);
	let _ = writeln!(out, "Characters: {}", doc.stats.char_count);
	let _ = writeln!(out, "Lines: {}", doc.stats.line_count);
	out
}
