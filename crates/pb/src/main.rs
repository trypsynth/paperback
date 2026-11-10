use std::{
	collections::HashMap,
	env,
	fmt::Write as _,
	fs,
	io::{self, IsTerminal as _, Read as _, Write as _},
	path::{Path, PathBuf},
	process,
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use paperback_core::{
	document::{Document, DocumentHandle, ParserContext},
	export::{self, ExportFormat},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, parse_document},
};

mod cli;
mod console;
mod formats;
mod input;
mod inputs;
mod ocr;
mod pages;
mod remote;

use cli::{Cli, Format};
use inputs::Source;
use pages::PageSelection;

#[derive(Debug)]
enum Destination<'a> {
	Stdout,
	File(&'a Path),
	Directory(&'a Path),
}

#[derive(Default)]
struct Tally {
	converted: usize,
	failed: usize,
	/// Kept apart from `failed` because exit code 2 has meant this since before pb took several inputs.
	locked: usize,
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	init_logging(cli.verbose);
	if cli.list_formats {
		return write_stdout(&formats::listing());
	}
	// Parsed up front so a bad specification is refused before a long parse rather than after it.
	let selection = cli.pages.as_deref().map(PageSelection::parse).transpose()?;
	let listed = cli.files_from.as_deref().map(read_files_from).transpose()?;
	let given = inputs::gather(&cli.input, listed)?;
	let inputs = inputs::collect(&given);
	let destination = destination(&cli, inputs.sources.len())?;
	run(&cli, &inputs, selection.as_ref(), &destination)
}

fn read_files_from(path: &Path) -> Result<Vec<inputs::Given>> {
	let from_stdin = path.as_os_str() == "-";
	let bytes = if from_stdin {
		stdin_list_allowed(io::stdin().is_terminal())?;
		let mut bytes = Vec::new();
		io::stdin().lock().read_to_end(&mut bytes).context("failed to read the list from stdin")?;
		bytes
	} else {
		fs::read(path).with_context(|| format!("failed to read the list {}", path.display()))?
	};
	// A pipe carries what cmd's own commands write, in the console's code page, and a file what a Windows program saved, in the ANSI one.
	let fallback = if from_stdin { console::text } else { console::ansi_text };
	Ok(inputs::read_list(&bytes, |entry| Path::new(entry).exists(), fallback))
}

const fn megabytes(size: u64) -> u64 {
	size.saturating_mul(1024 * 1024)
}

/// Otherwise pb would wait in silence on the console for a list to be typed.
fn stdin_list_allowed(stdin_is_terminal: bool) -> Result<()> {
	if stdin_is_terminal {
		let example = if cfg!(windows) {
			"dir /b *.pdf | pb --files-from - --output-dir out (cmd), or Get-ChildItem -Name *.pdf | pb --files-from - --output-dir out (PowerShell)"
		} else {
			"ls *.pdf | pb --files-from - --output-dir out"
		};
		bail!(
			"--files-from - reads the list from a pipe or a redirected file, but stdin is the console\nPipe a list in, for example: {example}"
		);
	}
	Ok(())
}

fn destination(cli: &Cli, count: usize) -> Result<Destination<'_>> {
	if let Some(dir) = &cli.output_dir {
		return Ok(Destination::Directory(dir));
	}
	if let Some(path) = &cli.output {
		if count > 1 {
			bail!(
				"-o names one file, but {count} documents were given to convert\nWrite them into a folder with --output-dir instead"
			);
		}
		return Ok(Destination::File(path));
	}
	// Several documents on stdout would run into each other mid-sentence, and in HTML one `<html>` would start inside another.
	if count > 1 {
		bail!(
			"{count} documents were given with nowhere to put them\nWrite them into a folder with --output-dir instead"
		);
	}
	Ok(Destination::Stdout)
}

/// Carries on past failures: stopping at the first locked EPUB in a folder of a hundred leaves the reader nothing and no clue which one it was.
fn run(
	cli: &Cli,
	inputs: &inputs::Inputs,
	selection: Option<&PageSelection>,
	destination: &Destination<'_>,
) -> Result<()> {
	if let Destination::Directory(dir) = destination {
		// Up front, so a folder pb can't create is reported once rather than as every file's failure.
		fs::create_dir_all(dir).with_context(|| format!("failed to create the folder {}", dir.display()))?;
	}
	let sources = &inputs.sources;
	let reporting = sources.len() > 1 || matches!(destination, Destination::Directory(_));
	let mut tally = Tally::default();
	// Before anything is read, so a mistyped pattern isn't learned of only after a long run.
	for pattern in &inputs.unmatched {
		eprintln!("pb: no file matches {pattern:?}");
		tally.failed += 1;
	}
	for problem in &inputs.problems {
		eprintln!("pb: {problem}");
		tally.failed += 1;
	}
	let mut written = HashMap::new();
	let one_at_a_time: Vec<&Source> = if at_once(cli, sources.len()) {
		let files: Vec<&Source> = sources.iter().filter(|source| matches!(source, Source::File(_))).collect();
		let mut later = Vec::new();
		for (source, result) in convert_at_once(cli, destination, selection, &files, &mut written)? {
			// Asked for after the batch, one at a time, since a prompt can't be shared between documents converting together.
			if !cli.no_prompt && result.as_ref().is_err_and(|err| err.chain().any(is_needs_password)) {
				written.retain(|_, claimant| *claimant != source.shown());
				later.push(source);
			} else {
				count(&mut tally, &result);
			}
		}
		later.extend(sources.iter().filter(|source| matches!(source, Source::Link(_))));
		later
	} else {
		sources.iter().collect()
	};
	let mut job = Job {
		cli,
		destination,
		selection,
		converter: Converter::new(cli),
		downloads: {
			let downloads = remote::Downloads::default().with_max_size(megabytes(cli.max_download_size));
			if cli.no_prompt { downloads } else { downloads.with_prompt(remote::ask_console) }
		},
		written,
	};
	for source in one_at_a_time {
		let shown = source.shown();
		// Before converting, so a password prompt says which document it belongs to.
		if reporting {
			eprintln!("pb: converting {shown}...");
		}
		let result = job.convert(source, &shown);
		if let Some(line) = outcome(&shown, &result, reporting, destination) {
			eprintln!("{line}");
		}
		count(&mut tally, &result);
	}
	job.downloads.remove();
	report(&tally, reporting);
	Ok(())
}

/// OCR is left out because it drives the system's own engine, which hasn't been tried from several threads at once.
fn at_once(cli: &Cli, count: usize) -> bool {
	cli.jobs != 1 && count > 1 && !cli.ocr_image_pages && !cli.ocr_text_pages
}

/// Converted on a pool of `--jobs` threads, which the parsers' own parallel work shares, so a batch doesn't start a second set of threads for every document. Names are claimed in input order first, so which of two documents wanting one name gets it doesn't depend on which finishes first.
fn convert_at_once<'s>(
	cli: &Cli,
	destination: &Destination<'_>,
	selection: Option<&PageSelection>,
	files: &[&'s Source],
	written: &mut HashMap<PathBuf, String>,
) -> Result<Vec<(&'s Source, Result<PathBuf>)>> {
	let mut results = Vec::new();
	let mut claimed = Vec::new();
	for &source in files {
		let Source::File(input) = source else { continue };
		let shown = source.shown();
		let path = match destination {
			Destination::Directory(dir) => dir.join(target_for(cli, input, selection)),
			_ => target_for(cli, input, selection),
		};
		let target = resolved(&path);
		if let Some(earlier) = written.get(&target) {
			results.push((source, Err(anyhow::anyhow!("{earlier} and {shown} are both writing {}", path.display()))));
		} else {
			written.insert(target, shown);
			claimed.push(source);
		}
	}
	for (source, result) in &results {
		if let Some(line) = outcome(&source.shown(), result, true, destination) {
			eprintln!("{line}");
		}
	}
	let pool = rayon::ThreadPoolBuilder::new()
		.num_threads(cli.jobs)
		.build()
		.context("failed to start the threads to convert on")?;
	let converted: Vec<(&Source, Result<PathBuf>)> = pool.install(|| {
		use rayon::prelude::*;
		claimed
			.par_iter()
			.with_max_len(1)
			.map(|&source| {
				let shown = source.shown();
				let result = match source {
					Source::File(input) => convert_file(cli, destination, selection, input, &shown),
					Source::Link(_) => unreachable!("only files are converted at once"),
				};
				let deferred = !cli.no_prompt && result.as_ref().is_err_and(|err| err.chain().any(is_needs_password));
				if !deferred && let Some(line) = outcome(&shown, &result, true, destination) {
					eprintln!("{line}");
				}
				(source, result)
			})
			.collect()
	});
	results.extend(converted);
	Ok(results)
}

/// Its name was claimed before the batch began, so the write is checked against nothing else in the run.
fn convert_file(
	cli: &Cli,
	destination: &Destination<'_>,
	selection: Option<&PageSelection>,
	input: &Path,
	shown: &str,
) -> Result<PathBuf> {
	let converted = Converter { cli, remembered: None, prompt: false }.convert(input, shown, selection)?;
	write_converted(cli, destination, input, shown, selection, &converted, &mut HashMap::new())?;
	Ok(target_for(cli, input, selection))
}

/// Only `--no-prompt`'s refusal counts as needing a password; a password given and refused is an ordinary failure, or one problem gets reported twice.
fn count(tally: &mut Tally, result: &Result<PathBuf>) {
	match result {
		Ok(_) => tally.converted += 1,
		Err(err) if err.chain().any(is_needs_password) => tally.locked += 1,
		Err(_) => tally.failed += 1,
	}
}

fn outcome(shown: &str, result: &Result<PathBuf>, reporting: bool, destination: &Destination<'_>) -> Option<String> {
	match result {
		Ok(target) => (reporting && matches!(destination, Destination::Directory(_)))
			.then(|| format!("pb: {shown} -> {}", target.display())),
		Err(err) => Some(format!("pb: {err:#}")),
	}
}

struct Job<'a> {
	cli: &'a Cli,
	destination: &'a Destination<'a>,
	selection: Option<&'a PageSelection>,
	converter: Converter<'a>,
	downloads: remote::Downloads,
	written: HashMap<PathBuf, String>,
}

impl Job<'_> {
	fn convert(&mut self, source: &Source, shown: &str) -> Result<PathBuf> {
		let downloaded;
		let input = match source {
			Source::File(path) => path.as_path(),
			Source::Link(url) => {
				downloaded = self.downloads.fetch(url)?;
				downloaded.path.as_path()
			}
		};
		let converted = self.converter.convert(input, shown, self.selection)?;
		write_converted(self.cli, self.destination, input, shown, self.selection, &converted, &mut self.written)?;
		Ok(target_for(self.cli, input, self.selection))
	}
}

#[derive(Debug)]
struct NeedsPassword;

impl std::fmt::Display for NeedsPassword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("needs a password; skipping (use -p to supply one)")
	}
}

impl std::error::Error for NeedsPassword {}

/// The password prefix is a sentinel between the parsers and pb, not something to show the reader.
fn parse_failed(shown: &str, error: &anyhow::Error) -> anyhow::Error {
	let message = error.to_string();
	let message = message.strip_prefix(PASSWORD_REQUIRED_ERROR_PREFIX).unwrap_or(&message);
	anyhow::anyhow!("failed to parse {shown}: {message}")
}

/// The marker is carried as a context, so callers search the chain rather than downcasting the root.
fn is_needs_password(cause: &(dyn std::error::Error + 'static)) -> bool {
	cause.is::<NeedsPassword>()
}

/// Exits the process on any problem rather than returning.
fn report(tally: &Tally, reporting: bool) {
	if reporting {
		let total = tally.converted + tally.failed + tally.locked;
		let mut line = format!("pb: converted {} of {}", tally.converted, total);
		if tally.failed > 0 {
			let _ = write!(line, ", {} failed", tally.failed);
		}
		if tally.locked > 0 {
			let _ = write!(line, ", {} needing a password", tally.locked);
		}
		eprintln!("{line}");
	}
	// Passwords first, so a run with a locked document exits 2 whatever else went wrong.
	if tally.locked > 0 {
		process::exit(2);
	}
	if tally.failed > 0 {
		process::exit(1);
	}
}

/// The first typed password is offered to every later document, since a folder of books often shares one; books under different passwords need a run each.
struct Converter<'a> {
	cli: &'a Cli,
	remembered: Option<String>,
	/// Off for documents converted at once: one that would be asked for a password comes back to be asked one at a time.
	prompt: bool,
}

impl<'a> Converter<'a> {
	const fn new(cli: &'a Cli) -> Self {
		Self { cli, remembered: None, prompt: true }
	}

	fn password(&self) -> Option<&String> {
		self.cli.password.as_ref().or(self.remembered.as_ref())
	}

	fn convert(&mut self, input: &Path, shown: &str, selection: Option<&PageSelection>) -> Result<String> {
		let cli = self.cli;
		input::check(input)?;
		let file_path = input.to_string_lossy().into_owned();
		let mut context = ParserContext::new(file_path)
			.with_render_tables_inline(true)
			.with_join_pdf_paragraphs(!cli.no_join_paragraphs);
		// Stripping repeated page-edge lines can cost real sentences (see `ParserContext::strip_running_text`), hence `--keep-repeated`.
		context = context.with_strip_running_text(!cli.keep_repeated);
		if let Some(password) = self.password() {
			context = context.with_password(password.clone());
		}
		let doc = match parse_document(&context) {
			Ok(doc) => doc,
			Err(e) if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
				if cli.no_prompt {
					return Err(anyhow::Error::new(NeedsPassword).context(shown.to_string()));
				}
				// The parser can't tell a wrong password from a missing one, and without a terminal asking again would wait forever.
				if self.password().is_some() {
					return Err(parse_failed(shown, &e));
				}
				if !self.prompt {
					return Err(anyhow::Error::new(NeedsPassword).context(shown.to_string()));
				}
				let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
				context.password = Some(password.clone());
				// Remembered only once it opens something, so a mistyped password costs one document, not the run.
				let doc = parse_document(&context).map_err(|e| parse_failed(shown, &e))?;
				self.remembered = Some(password);
				doc
			}
			Err(e) => return Err(parse_failed(shown, &e)),
		};
		// Chosen from the parsed document, since an encrypted PDF's range can't be resolved before the password and repeated lines are judged over the whole document.
		let mut doc = match selection {
			Some(selection) => pages::apply(selection, &doc)?,
			None => doc,
		};
		let ocr = ocr::Selection { image_pages: cli.ocr_image_pages, text_pages: cli.ocr_text_pages };
		if ocr.any() {
			// After the slice so only kept pages are read, and after the password so an encrypted document opens once.
			ocr::pages(&mut doc, &context.file_path, context.password.as_deref(), ocr, !cli.no_join_paragraphs)?;
		}
		ocr::reword_placeholders(&mut doc, ocr.image_pages);
		Ok(match (cli.metadata, cli.format) {
			(true, _) => metadata(&doc),
			(false, Format::Text) => doc.buffer.content,
			(false, Format::Html) => export::render(&DocumentHandle::new(doc), ExportFormat::Html).into_owned(),
			(false, Format::Markdown) => export::render(&DocumentHandle::new(doc), ExportFormat::Markdown).into_owned(),
		})
	}
}

/// `--force` covers files there before pb started; two documents in one run wanting one name is pb's own naming, so `written` refuses it regardless.
fn write_converted(
	cli: &Cli,
	destination: &Destination<'_>,
	input: &Path,
	shown: &str,
	selection: Option<&PageSelection>,
	converted: &str,
	written: &mut HashMap<PathBuf, String>,
) -> Result<()> {
	let path = match destination {
		Destination::Stdout => return write_stdout(converted),
		Destination::File(path) => path.to_path_buf(),
		Destination::Directory(dir) => dir.join(target_for(cli, input, selection)),
	};
	// `--output-dir .` lands each output on the next document's name, so this is refused whatever --force says.
	let target = resolved(&path);
	if target == resolved(input) {
		bail!("{} is the document being converted; write somewhere else", path.display());
	}
	if let Some(earlier) = written.get(&target) {
		bail!("{earlier} and {shown} are both writing {}", path.display());
	}
	if path.exists() && !cli.force {
		bail!("{} is already there; pass --force to replace it", path.display());
	}
	written.insert(target, shown.to_string());
	write_file(&path, converted, !cli.metadata && matches!(cli.format, Format::Markdown))
}

/// So `.`, an absolute path and a different case all reach one folder; an output that doesn't exist yet has its folder resolved instead.
fn resolved(path: &Path) -> PathBuf {
	let full = fs::canonicalize(path).unwrap_or_else(|_| {
		let parent = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
		match (fs::canonicalize(parent), path.file_name()) {
			(Ok(dir), Some(name)) => dir.join(name),
			_ => path.to_path_buf(),
		}
	});
	inputs::same_file(&full)
}

fn target_for(cli: &Cli, input: &Path, selection: Option<&PageSelection>) -> PathBuf {
	let stem = input.file_stem().map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
	output_name(&stem, cli.format, selection)
}

/// The page range is in the name so two runs asking for different extracts don't replace each other.
fn output_name(stem: &str, format: Format, selection: Option<&PageSelection>) -> PathBuf {
	let mut name = PathBuf::from(stem);
	if let Some(selection) = selection {
		let _ = write!(name.as_mut_os_string(), ".p{}", selection.label());
	}
	let _ = write!(name.as_mut_os_string(), ".{}", format.extension());
	name
}

/// Markdown files get a BOM so editors like `EdSharp` detect the encoding; stdout is left alone, since there the reader is a pipe.
fn write_file(path: &Path, text: &str, markdown: bool) -> Result<()> {
	let written = if markdown {
		let mut bytes = vec![0xEF_u8, 0xBB, 0xBF];
		bytes.extend_from_slice(text.as_bytes());
		fs::write(path, bytes)
	} else {
		fs::write(path, text)
	};
	written.with_context(|| format!("failed to write {}", path.display()))
}

/// `pb book.pdf | head` closes the pipe early, and `print!` would panic on that as if pb crashed.
fn write_stdout(text: &str) -> Result<()> {
	let mut out = io::stdout().lock();
	match out.write_all(text.as_bytes()).and_then(|()| out.flush()) {
		Err(e) if is_closed_pipe(&e) => Ok(()),
		result => result.context("failed to write to stdout"),
	}
}

/// Windows also reports a closed pipe as `ERROR_NO_DATA` (232).
fn is_closed_pipe(error: &io::Error) -> bool {
	error.kind() == io::ErrorKind::BrokenPipe || (cfg!(windows) && error.raw_os_error() == Some(232))
}

/// Off by default so conversions aren't flooded with parser logging; `RUST_LOG` always wins.
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
	let _ = writeln!(out, "Words: {}", doc.stats().word_count);
	let _ = writeln!(out, "Characters: {}", doc.stats().char_count);
	let _ = writeln!(out, "Lines: {}", doc.stats().line_count);
	out
}

#[cfg(test)]
mod tests {
	use std::{
		env, fs,
		path::PathBuf,
		process,
		sync::atomic::{AtomicU64, Ordering},
	};

	use super::*;

	/// paperback-core's own is behind `cfg(test)`, out of another crate's reach.
	struct TempDir {
		path: PathBuf,
	}

	impl TempDir {
		fn new(label: &str) -> Self {
			static COUNTER: AtomicU64 = AtomicU64::new(0);
			let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
			let path = env::temp_dir().join(format!("pb_main_test_{label}_{}_{unique}", process::id()));
			fs::create_dir_all(&path).expect("create the temp dir");
			Self { path }
		}

		fn join(&self, name: &str) -> PathBuf {
			self.path.join(name)
		}
	}

	impl Drop for TempDir {
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.path);
		}
	}

	fn named(stem: &str, format: Format, pages: Option<&str>) -> String {
		let selection = pages.map(|spec| PageSelection::parse(spec).expect("a page specification"));
		output_name(stem, format, selection.as_ref()).to_string_lossy().into_owned()
	}

	#[test]
	fn an_output_is_named_after_the_document_and_the_format() {
		assert_eq!(named("book", Format::Text, None), "book.txt");
		assert_eq!(named("book", Format::Html, None), "book.html");
		assert_eq!(named("book", Format::Markdown, None), "book.md");
	}

	#[test]
	fn different_page_ranges_get_different_names() {
		assert_ne!(named("book", Format::Markdown, Some("1-2")), named("book", Format::Markdown, Some("3-4")));
	}

	#[test]
	fn the_page_range_is_in_the_name_when_there_is_one() {
		assert_eq!(named("book", Format::Markdown, Some("5-10")), "book.p5-10.md");
		assert_eq!(named("book", Format::Text, Some("5-10")), "book.p5-10.txt");
		assert_eq!(named("book", Format::Html, Some("80-end")), "book.p80-end.html");
		assert_eq!(named("book", Format::Html, Some("5-10,80-end")), "book.p5-10_80-end.html");
	}

	#[test]
	fn disjoint_ranges_are_not_run_together_into_one_range() {
		let name = named("book", Format::Text, Some("1-5;20"));
		assert_eq!(name, "book.p1-5_20.txt");
		assert!(!name.contains("1-5-20"), "{name} reads as a single range from 1 to 520");
	}

	#[test]
	fn several_disjoint_ranges_each_keep_their_own_bounds() {
		assert_eq!(named("book", Format::Text, Some("1-5;20;80-end")), "book.p1-5_20_80-end.txt");
		assert_eq!(named("book", Format::Text, Some("7;20;33")), "book.p7_20_33.txt");
	}

	#[test]
	fn a_single_page_is_not_written_as_a_range() {
		assert_eq!(named("book", Format::Markdown, Some("7")), "book.p7.md");
	}

	#[test]
	fn ranges_are_named_after_what_they_merge_to() {
		assert_eq!(named("book", Format::Markdown, Some("5-10,7-12")), named("book", Format::Markdown, Some("5-12")));
	}

	#[test]
	fn the_extension_of_the_input_is_replaced_not_kept() {
		assert_eq!(output_name("book", Format::Markdown, None), PathBuf::from("book.md"));
		assert!(!named("book", Format::Markdown, None).contains(".epub"));
	}

	#[test]
	fn one_named_file_cannot_hold_several_documents() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub", "b.epub", "-o", "out.txt"]).expect("parse");
		let error = destination(&cli, 2).expect_err("two documents, one file").to_string();
		assert!(error.contains("--output-dir"), "{error} does not say what to use instead");
	}

	#[test]
	fn one_named_file_holds_one_document() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub", "-o", "out.txt"]).expect("parse");
		assert!(matches!(destination(&cli, 1).expect("one document, one file"), Destination::File(_)));
	}

	#[test]
	fn an_output_directory_holds_any_number_of_documents() {
		for count in [1, 2, 40] {
			let cli = cli::Cli::try_parse_from(["pb", "a.epub", "--output-dir", "out"]).expect("parse");
			assert!(matches!(destination(&cli, count).expect("a folder holds any number"), Destination::Directory(_)));
		}
	}

	#[test]
	fn with_nowhere_to_put_it_the_output_goes_to_stdout() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub"]).expect("parse");
		assert!(matches!(destination(&cli, 1).expect("stdout"), Destination::Stdout));
	}

	#[test]
	fn several_documents_cannot_be_sent_to_stdout() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub", "b.epub"]).expect("parse");
		let error = destination(&cli, 2).expect_err("two documents, one stdout").to_string();
		assert!(error.contains("--output-dir"), "{error} does not say what to use instead");
	}

	#[test]
	fn an_output_file_that_is_already_there_is_left_alone_without_force() {
		let dir = TempDir::new("overwrite");
		let existing = dir.join("out.txt");
		fs::write(&existing, b"what was there before").expect("write");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");
		let refuse = |args: &[&str]| {
			let cli = cli::Cli::try_parse_from(args).expect("parse");
			let mut written = HashMap::new();
			write_converted(
				&cli,
				&destination(&cli, 1).expect("destination"),
				&input,
				"book.epub",
				None,
				"new text",
				&mut written,
			)
		};
		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap()])
			.expect_err("an existing file must not be replaced without --force");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "what was there before");
		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap(), "--force"]).expect("--force replaces it");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "new text");
	}

	#[test]
	fn two_documents_wanting_one_name_do_not_overwrite_each_other() {
		let dir = TempDir::new("collision");
		let first = dir.join("a");
		let second = dir.join("b");
		fs::create_dir_all(&first).expect("mkdir");
		fs::create_dir_all(&second).expect("mkdir");
		let first = first.join("book.txt");
		let second = second.join("book.txt");
		fs::write(&first, b"the first book").expect("write");
		fs::write(&second, b"the second book").expect("write");
		let out = dir.join("out");
		fs::create_dir_all(&out).expect("mkdir");
		let cli = cli::Cli::try_parse_from(["pb", "book.txt", "--output-dir", out.to_str().unwrap(), "--force"])
			.expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &first, "a/book.txt", None, "the first book", &mut written)
			.expect("the first book");
		let error = write_converted(&cli, &destination, &second, "b/book.txt", None, "the second book", &mut written)
			.expect_err("--force must not let the second book overwrite the first");
		assert!(error.to_string().contains("both writing"), "{error}");
		assert_eq!(fs::read_to_string(out.join("book.txt")).expect("read"), "the first book");
	}

	#[test]
	fn a_document_is_never_written_over_itself() {
		let dir = TempDir::new("self");
		let input = dir.join("book.md");
		fs::write(&input, b"the book").expect("write");
		let cli = cli::Cli::try_parse_from(["pb", "book.md", "--output-dir", dir.path.to_str().unwrap(), "-f", "md"])
			.expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &input, "book.md", None, "converted", &mut written)
			.expect_err("a document must not be replaced by its own conversion");
		assert_eq!(fs::read_to_string(&input).expect("read"), "the book");
	}

	#[test]
	fn a_document_spelled_another_way_is_still_not_written_over_itself() {
		let dir = TempDir::new("self-spelled");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("book.md"), b"the book").expect("write");
		let input = dir.join("sub").join("..").join("book.md");
		let out = dir.path.to_str().unwrap();
		let cli =
			cli::Cli::try_parse_from(["pb", "book.md", "--output-dir", out, "-f", "md", "--force"]).expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &input, "book.md", None, "converted", &mut written)
			.expect_err("--force must not let a document be replaced by its own conversion");
		assert_eq!(fs::read_to_string(dir.join("book.md")).expect("read"), "the book");
	}

	#[test]
	fn an_output_path_that_is_free_is_written_without_asking() {
		let dir = TempDir::new("fresh");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");
		let fresh = dir.join("out.txt");
		let cli = cli::Cli::try_parse_from(["pb", "book.epub", "-o", fresh.to_str().unwrap()]).expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &input, "book.epub", None, "new text", &mut written)
			.expect("writing to a path that is free");
		assert_eq!(fs::read_to_string(&fresh).expect("read"), "new text");
	}

	#[test]
	fn the_password_marker_is_not_shown_to_the_reader() {
		let error = parse_failed(
			"alpha.pdf",
			&anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect"),
		);
		let printed = error.to_string();
		assert!(!printed.contains(PASSWORD_REQUIRED_ERROR_PREFIX), "{printed}");
		assert!(printed.contains("alpha.pdf"), "{printed} does not name the document");
		assert!(printed.contains("Password required or incorrect"), "{printed}");
	}

	#[test]
	fn an_ordinary_parse_error_is_left_alone() {
		let printed = parse_failed("x.pdf", &anyhow::anyhow!("Failed to open PDF document")).to_string();
		assert!(printed.contains("Failed to open PDF document"), "{printed}");
	}

	/// Counted apart once, a run said "1 failed, 1 needing a password" about two documents that both failed on the password.
	#[test]
	fn a_refused_password_is_an_ordinary_failure() {
		let error = anyhow::Error::msg("failed to parse locked.pdf")
			.context("[password_required]Password required or incorrect");
		let counted_as_a_password_problem = error.chain().any(is_needs_password);
		assert!(!counted_as_a_password_problem, "a refused password would be counted apart from the failures again");

		let printed = format!("{error:#}");
		assert!(printed.contains("locked.pdf"), "{printed}");
		assert!(printed.contains("password_required"), "{printed} lost the parser's own wording");
	}

	#[test]
	fn a_password_message_names_its_document_and_says_what_to_do() {
		let needed = format!("{:#}", anyhow::Error::new(NeedsPassword).context("locked.pdf"));
		assert!(needed.contains("locked.pdf"), "{needed}");
		assert!(needed.contains("-p"), "{needed} does not say how to supply one");
	}

	#[test]
	fn a_pattern_is_only_a_pattern_when_no_such_file_is_there() {
		let dir = TempDir::new("is-pattern");
		let real = dir.join("book.pdf");
		fs::write(&real, b"x").expect("write");
		assert!(!inputs::is_pattern(&real), "a file that exists is a path whatever else it holds");
		assert!(inputs::is_pattern(&dir.join("*.pdf")));
	}

	#[test]
	fn the_download_limit_is_given_in_megabytes() {
		assert_eq!(megabytes(512), 512 * 1024 * 1024);
		assert_eq!(megabytes(u64::MAX), u64::MAX);
	}

	#[test]
	fn a_list_from_stdin_is_refused_when_stdin_is_the_console() {
		let error = stdin_list_allowed(true).expect_err("the console").to_string();
		assert!(error.contains("console"), "{error}");
		stdin_list_allowed(false).expect("a pipe or a redirected file");
	}

	#[test]
	fn a_run_keeps_the_password_it_was_given() {
		let cli = cli::Cli::try_parse_from(["pb", "a.pdf"]).expect("parse");
		let mut converter = Converter::new(&cli);
		assert!(converter.password().is_none(), "nothing has been asked for yet");

		converter.remembered = Some("hunter2".to_string());
		assert_eq!(converter.password().map(String::as_str), Some("hunter2"));

		let given = cli::Cli::try_parse_from(["pb", "a.pdf", "-p", "abc"]).expect("parse");
		let mut converter = Converter::new(&given);
		converter.remembered = Some("hunter2".to_string());
		assert_eq!(converter.password().map(String::as_str), Some("abc"), "-p must win over a remembered one");
	}

	#[test]
	fn two_links_wanting_one_name_are_reported_by_their_links() {
		let dir = TempDir::new("link-collision");
		let (one, two) = (dir.join("1"), dir.join("2"));
		fs::create_dir_all(&one).expect("mkdir");
		fs::create_dir_all(&two).expect("mkdir");
		let (one, two) = (one.join("download.pdf"), two.join("download.pdf"));
		fs::write(&one, b"x").expect("write");
		fs::write(&two, b"x").expect("write");
		let out = dir.join("out");
		let cli = cli::Cli::try_parse_from(["pb", "x", "--output-dir", out.to_str().unwrap()]).expect("parse");
		fs::create_dir_all(&out).expect("mkdir");
		let destination = destination(&cli, 2).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &one, "https://example.org/download?id=1", None, "one", &mut written)
			.expect("the first link");
		let error =
			write_converted(&cli, &destination, &two, "https://example.org/download?id=2", None, "two", &mut written)
				.expect_err("two links, one name")
				.to_string();
		assert!(error.contains("download?id=1") && error.contains("download?id=2"), "{error}");
	}

	fn text_files(dir: &TempDir, names: &[&str]) -> Vec<Source> {
		names
			.iter()
			.map(|name| {
				let path = dir.join(name);
				fs::create_dir_all(path.parent().expect("a folder")).expect("mkdir");
				fs::write(&path, format!("the text of {name}\n")).expect("write");
				Source::File(path)
			})
			.collect()
	}

	#[test]
	fn documents_converted_at_once_each_get_their_own_output() {
		let dir = TempDir::new("at-once");
		let names: Vec<String> = (0..12).map(|i| format!("book{i}.txt")).collect();
		let sources = text_files(&dir, &names.iter().map(String::as_str).collect::<Vec<_>>());
		let out = dir.join("out");
		fs::create_dir_all(&out).expect("mkdir");
		let cli =
			cli::Cli::try_parse_from(["pb", "x", "--output-dir", out.to_str().unwrap(), "-j", "4"]).expect("parse");
		let destination = destination(&cli, sources.len()).expect("destination");
		let files: Vec<&Source> = sources.iter().collect();
		let results = convert_at_once(&cli, &destination, None, &files, &mut HashMap::new()).expect("a pool");
		assert!(
			results.iter().all(|(_, result)| result.is_ok()),
			"{:?}",
			results.iter().map(|(_, r)| r.as_ref().err().map(ToString::to_string)).collect::<Vec<_>>()
		);
		for name in &names {
			let written = fs::read_to_string(out.join(name)).expect("an output for every document");
			assert!(written.contains(&format!("the text of {name}")), "{written}");
		}
	}

	#[test]
	fn the_first_listed_of_two_documents_wanting_one_name_gets_it_whichever_finishes_first() {
		let dir = TempDir::new("at-once-collision");
		let sources = text_files(&dir, &["a/book.txt", "b/book.txt"]);
		let out = dir.join("out");
		fs::create_dir_all(&out).expect("mkdir");
		let cli =
			cli::Cli::try_parse_from(["pb", "x", "--output-dir", out.to_str().unwrap(), "-j", "2"]).expect("parse");
		let destination = destination(&cli, 2).expect("destination");
		let files: Vec<&Source> = sources.iter().collect();
		let results = convert_at_once(&cli, &destination, None, &files, &mut HashMap::new()).expect("a pool");
		let failed: Vec<String> =
			results.iter().filter(|(_, result)| result.is_err()).map(|(source, _)| source.shown()).collect();
		assert_eq!(failed.len(), 1, "{failed:?}");
		assert!(failed[0].contains('b'), "the second listed should be the one refused: {failed:?}");
		assert!(fs::read_to_string(out.join("book.txt")).expect("read").contains("a/book.txt"));
	}

	#[test]
	fn ocr_and_a_single_document_are_converted_one_at_a_time() {
		let parse = |args: &[&str]| cli::Cli::try_parse_from(args).expect("parse");
		assert!(at_once(&parse(&["pb", "x", "-j", "4"]), 2));
		assert!(at_once(&parse(&["pb", "x", "-j", "0"]), 2));
		assert!(at_once(&parse(&["pb", "x"]), 2), "at once is the default");
		assert!(!at_once(&parse(&["pb", "x", "-j", "1"]), 2));
		assert!(!at_once(&parse(&["pb", "x", "-j", "4"]), 1));
		assert!(!at_once(&parse(&["pb", "x", "-j", "4", "--ocr-image-pages"]), 2));
	}

	#[test]
	fn a_parse_failure_names_what_the_reader_gave() {
		let printed =
			parse_failed("https://example.org/a.pdf", &anyhow::anyhow!("Failed to open PDF document")).to_string();
		assert!(printed.contains("https://example.org/a.pdf"), "{printed}");
	}
}
