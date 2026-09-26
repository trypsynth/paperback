use std::{env, fmt::Write as _, fs, io, process};

use anyhow::{Context, Result};
use clap::Parser;
use paperback_core::{
	document::{Document, ParserContext},
	export::{self, ExportFormat},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, parse_document, pdf},
	set_pdfium_library_path,
};

mod cli;
mod formats;
mod input;
mod pages;

use cli::{Cli, Format};

fn main() -> Result<()> {
	let cli = Cli::parse();
	init_logging(cli.verbose);
	point_at_pdfium();
	if cli.list_formats {
		print!("{}", formats::listing());
		return Ok(());
	}
	// clap holds the input to being there unless the formats are all that was asked for.
	let input = cli.input.expect("an input file");
	let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
	input::check(&input)?;
	let file_path = input.to_string_lossy().into_owned();
	// Read up front so a bad specification is refused before a long parse rather than after it.
	// The pages themselves are chosen below, once the document has been read and its length known.
	let selection = cli.pages.as_deref().map(pages::PageSelection::parse).transpose()?;
	// This path bypasses the text buffer entirely, so there are no page-break markers in it to
	// select from and --pages would be silently dropped. The normal route below is taken instead.
	if !cli.metadata && matches!(cli.format, Format::Html) && ext == "epub" && selection.is_none() {
		let html = export::epub_direct::render(&file_path)
			.with_context(|| format!("failed to convert {}", input.display()))?;
		// map_or_else reads worse here than the plain if/else.
		#[allow(clippy::option_if_let_else)]
		return if let Some(path) = cli.output {
			fs::write(&path, &html).with_context(|| format!("failed to write {}", path.display()))
		} else {
			print!("{html}");
			Ok(())
		};
	}
	let mut context =
		ParserContext::new(file_path).with_render_tables_inline(true).with_join_pdf_paragraphs(!cli.no_join_paragraphs);
	// Off unless asked for, and the reason is in `ParserContext::strip_running_text`: taking the
	// repeated page-edge lines out is a judgement, and a caller converting a page range out of a
	// long document is better served by every word than by a tidy edge. It costs the whole
	// document to get right, which is why it is not the default.
	context = context.with_strip_running_text(cli.strip_repeated);
	if let Some(password) = cli.password {
		context = context.with_password(password);
	}
	// Handed to the parser rather than worked out here, because it is the parser that knows what a
	// page costs to read. A PDF is asked for just the pages wanted, which is nearly all of the
	// time saved on a long document; the cheaper formats read the whole thing and are sliced
	// afterwards by `pages::apply` below. Both routes end up with the same extract.
	let mut parsed_the_pages_asked_for = false;
	if let Some(selection) = &selection
		&& ext.eq_ignore_ascii_case("pdf")
	{
		// The page count comes from opening the document, which is cheap, rather than from
		// parsing it, which is not.
		let count = usize::try_from(pdf::page_count(&context)?).unwrap_or(0);
		context = context.with_only_pages(pages::wanted_pages(selection, count)?);
		parsed_the_pages_asked_for = true;
	}
	let doc = match parse_document(&context) {
		Ok(doc) => doc,
		Err(e) if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
			if cli.no_prompt {
				eprintln!("pb: {} needs a password; skipping (use -p to supply one)", input.display());
				process::exit(2);
			}
			let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
			context.password = Some(password);
			parse_document(&context).with_context(|| format!("failed to parse {}", input.display()))?
		}
		Err(e) => return Err(e.context(format!("failed to parse {}", input.display()))),
	};
	// Cut down to the pages asked for, unless the parse already returned only those. Which route
	// was taken is the parser's business: a PDF that was asked for a few pages never built the
	// rest, while a cheaper format read everything and is sliced here.
	let doc = match selection {
		Some(selection) if !parsed_the_pages_asked_for => pages::apply(&selection, &doc)?,
		_ => doc,
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

/// Points the PDF reader at the Pdfium library shipped beside this executable, the same way the
/// desktop app does. Without this the loader falls back to the operating system's own search,
/// which finds the library next to the binary on some platforms and not others.
fn point_at_pdfium() {
	if let Ok(exe) = env::current_exe()
		&& let Some(dir) = exe.parent()
	{
		set_pdfium_library_path(dir.to_string_lossy().into_owned());
	}
}

/// Prints paperback-core's tracing output to stderr, off by default so normal conversions
/// aren't flooded with parser debug logging. Pass `-v`/`--verbose` to turn it on, or set
/// `RUST_LOG` directly for finer-grained control (which always takes precedence).
fn init_logging(verbose: bool) {
	let default_filter = if verbose { "paperback_core=debug" } else { "off" };
	let filter = env::var("RUST_LOG").unwrap_or_else(|_| default_filter.to_string());
	tracing_subscriber::fmt().with_env_filter(filter).with_writer(io::stderr).with_target(false).init();
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
