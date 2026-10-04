use std::{
	collections::HashMap,
	env,
	fmt::Write as _,
	fs,
	io::{self, Write as _},
	path::{Path, PathBuf},
	process,
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use paperback_core::{
	document::{Document, ParserContext},
	export::{self, ExportFormat},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, parse_document},
	set_pdfium_library_path,
};

mod cli;
mod formats;
mod input;
mod inputs;
mod ocr;
mod pages;

use cli::{Cli, Format};
use pages::PageSelection;

/// Where one document's output goes.
#[derive(Debug)]
enum Destination<'a> {
	/// Straight to stdout, which is what pb has always done without `-o`.
	Stdout,
	/// One named file, which only makes sense for a single input.
	File(&'a Path),
	/// One file per input in a folder, named after each document.
	Directory(&'a Path),
}

/// What a run over several documents came to, and what the exit code should say about it.
#[derive(Default)]
struct Tally {
	converted: usize,
	failed: usize,
	/// Documents refused for want of a password under `--no-prompt`. Kept apart from `failed`
	/// because it has had an exit code of its own since before there was more than one input.
	locked: usize,
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	init_logging(cli.verbose);
	point_at_pdfium();
	if cli.list_formats {
		return write_stdout(&formats::listing());
	}
	// Read up front so a bad specification is refused before a long parse rather than after it.
	// The pages themselves are chosen below, once the document has been read and its length known.
	let selection = cli.pages.as_deref().map(PageSelection::parse).transpose()?;
	// A Windows shell hands `*.pdf` over as one literal argument, so the patterns are expanded here
	// rather than being left for a shell that may not expand them. A pattern that matched nothing
	// is carried rather than raised, so one mistyped pattern does not cost the reader the rest.
	let inputs = inputs::collect(&cli.input);
	let destination = destination(&cli, inputs.files.len())?;
	run(&cli, &inputs, selection.as_ref(), &destination)
}

/// Where the output of this run goes, and whether the instruction to put it there is one that can
/// be carried out.
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
	// Several documents to stdout would arrive one after another with nothing between them: two
	// books run into each other mid-sentence, and in HTML one `<html>` document starts inside
	// another. One document on stdout is a pipe a reader can take; several is a file nobody asked
	// to be assembled.
	if count > 1 {
		bail!(
			"{count} documents were given with nowhere to put them\nWrite them into a folder with --output-dir instead"
		);
	}
	Ok(Destination::Stdout)
}

/// Converts each document in turn, carrying on past the ones that fail and saying so at the end.
///
/// A document that cannot be read is that document's problem, not the run's: over a folder of a
/// hundred books, stopping at the first EPUB with no password leaves the reader with nothing and
/// no way of telling which of the hundred was the problem.
fn run(
	cli: &Cli,
	inputs: &inputs::Inputs,
	selection: Option<&PageSelection>,
	destination: &Destination<'_>,
) -> Result<()> {
	if let Destination::Directory(dir) = destination {
		// Made up front rather than per document, so a folder pb cannot create is reported once
		// before anything has been read rather than as the same failure for every file.
		fs::create_dir_all(dir).with_context(|| format!("failed to create the folder {}", dir.display()))?;
	}
	let files = &inputs.files;
	let reporting = files.len() > 1 || matches!(destination, Destination::Directory(_));
	let mut converter = Converter::new(cli);
	let mut tally = Tally::default();
	let mut written: HashMap<PathBuf, PathBuf> = HashMap::new();
	// Reported before anything is read, so the reader learns of a mistyped pattern without waiting
	// for the run it was meant to be part of.
	for pattern in &inputs.unmatched {
		eprintln!("pb: no file matches {pattern:?}");
		tally.failed += 1;
	}
	for file in files {
		// Before the document is touched, not after it is written. A run over a folder is a long
		// silence between the first prompt and the summary, and it is what tells a reader which
		// document a password prompt belongs to.
		if reporting {
			eprintln!("pb: converting {}...", file.display());
		}
		let outcome = converter
			.convert(file, selection)
			.and_then(|converted| write_converted(cli, destination, file, selection, &converted, &mut written));
		match outcome {
			Ok(()) => {
				tally.converted += 1;
				if reporting && matches!(destination, Destination::Directory(_)) {
					eprintln!("pb: {} -> {}", file.display(), target_for(cli, file, selection).display());
				}
			}
			// A document held by a password under `--no-prompt` is counted apart from a failure,
			// because the reader can do something about it -- supply one -- and because exit code
			// 2 has meant that since before this could run over more than one file. A password that
			// was given and refused is an ordinary failure instead: it is the same error the first
			// document of the run reports, and counting it here too described one problem twice.
			Err(err) if err.chain().any(is_needs_password) => {
				eprintln!("pb: {err:#}");
				tally.locked += 1;
			}
			Err(err) => {
				eprintln!("pb: {err:#}");
				tally.failed += 1;
			}
		}
	}
	report(&tally, reporting);
	Ok(())
}

/// Marks a document that `--no-prompt` would not ask a password for, so that carrying on past it
/// is a decision rather than an accident. The prompt is what turns this into a refusal: without
/// `--no-prompt` the reader is asked, and the run carries on with the answer.
#[derive(Debug)]
struct NeedsPassword;

impl std::fmt::Display for NeedsPassword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("needs a password; skipping (use -p to supply one)")
	}
}

impl std::error::Error for NeedsPassword {}

/// A document that would not parse, said the way a reader is meant to hear it.
///
/// The parsers mark a document that wants a password with a prefix so that pb can tell it from
/// every other way a document will not open. That prefix is a sentinel between pieces of this
/// code, not something to read: a wrong password should say the password was not accepted, not
/// spell out the name of the field it was recognised by.
fn parse_failed(input: &Path, error: &anyhow::Error) -> anyhow::Error {
	let message = error.to_string();
	let message = message.strip_prefix(PASSWORD_REQUIRED_ERROR_PREFIX).unwrap_or(&message);
	anyhow::anyhow!("failed to parse {}: {message}", input.display())
}

/// Whether this error, or anything it was raised on top of, is [`NeedsPassword`].
///
/// The marker is carried as a context rather than returned bare, so it is somewhere in the chain
/// rather than at the root, and which is why this walks the chain instead of downcasting the error
/// itself.
fn is_needs_password(cause: &(dyn std::error::Error + 'static)) -> bool {
	cause.is::<NeedsPassword>()
}

/// What happened, and what the exit code should be.
///
/// Returns nothing because it does not return at all unless the run went well: a run with a
/// problem exits from here rather than unwinding, so the caller has nothing to do either way.
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
	// Passwords first, so that a run whose only problem was an encrypted document still exits 2
	// whatever else went wrong with it.
	if tally.locked > 0 {
		process::exit(2);
	}
	if tally.failed > 0 {
		process::exit(1);
	}
}

/// Converts the documents of one run, and remembers what it learned from the last of them.
///
/// Only one thing is worth remembering: the password a reader typed for an encrypted document. A
/// folder of books commonly holds many under the same password, and asking again for each is the
/// reader answering the same question thirty times over. So the answer to the first is offered to
/// every document after it, and a document that will not take it is reported rather than asked
/// about again. Books under different passwords need one run apiece.
struct Converter<'a> {
	cli: &'a Cli,
	/// What a reader typed when the first encrypted document asked for one, reused after that.
	remembered: Option<String>,
}

impl<'a> Converter<'a> {
	const fn new(cli: &'a Cli) -> Self {
		Self { cli, remembered: None }
	}

	/// The password to open this document with: the one given with `-p`, or the one a reader gave
	/// for an earlier document.
	fn password(&self) -> Option<&String> {
		self.cli.password.as_ref().or(self.remembered.as_ref())
	}

	/// Reads one document and renders it in the format asked for.
	fn convert(&mut self, input: &Path, selection: Option<&PageSelection>) -> Result<String> {
		let cli = self.cli;
		let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
		input::check(input)?;
		let file_path = input.to_string_lossy().into_owned();
		// This path bypasses the text buffer entirely, so there are no page-break markers in it to
		// select from and --pages would be silently dropped. The normal route below is taken instead.
		if !cli.metadata && matches!(cli.format, Format::Html) && ext == "epub" && selection.is_none() {
			return export::epub_direct::render(&file_path)
				.with_context(|| format!("failed to convert {}", input.display()));
		}
		let mut context = ParserContext::new(file_path)
			.with_render_tables_inline(true)
			.with_join_pdf_paragraphs(!cli.no_join_paragraphs);
		// On by default, so pb gives the app's reading of a file as it always has. Taking the repeated page-edge lines out is a judgement, though, and on some documents it costs real sentences (see `ParserContext::strip_running_text`), so `--keep-repeated` is there for a caller that would rather keep every line.
		context = context.with_strip_running_text(!cli.keep_repeated);
		if let Some(password) = self.password() {
			context = context.with_password(password.clone());
		}
		let doc = match parse_document(&context) {
			Ok(doc) => doc,
			Err(e) if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
				if cli.no_prompt {
					return Err(anyhow::Error::new(NeedsPassword).context(input.display().to_string()));
				}
				// Asked at most once in a run. A document that will not open for the password
				// already in hand is not asked about again: the reader has answered, and the
				// answer was wrong. Where there is no terminal to type into, asking again
				// waits there for good rather than saying the password did not fit.
				//
				// The parser cannot tell a wrong password from a missing one, which is why
				// this is asked from what the reader supplied rather than what came back.
				if self.password().is_some() {
					return Err(parse_failed(input, &e));
				}
				let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
				context.password = Some(password.clone());
				// Remembered only once the password has actually opened something. A reader who
				// mistypes it is asked again for the next document rather than having one wrong
				// answer silently applied to every book after, so a mistyped password costs one
				// document and not the whole run.
				let doc = parse_document(&context).map_err(|e| parse_failed(input, &e))?;
				self.remembered = Some(password);
				doc
			}
			Err(e) => return Err(parse_failed(input, &e)),
		};
		// Cut down to the pages asked for. Every format goes this way: the pages are chosen from the
		// parsed document rather than asked of the parser, because a page range out of an encrypted
		// PDF cannot be resolved until the password is in hand, and because the survey that judges a
		// repeated line a fact about the whole document rather than about the extract.
		let mut doc = match selection {
			Some(selection) => pages::apply(selection, &doc)?,
			None => doc,
		};
		let ocr = ocr::Selection { image_pages: cli.ocr_image_pages, text_pages: cli.ocr_text_pages };
		if ocr.any() {
			// After the slice, so that a page range is read from the pages it kept rather than from
			// the whole book, and after the password is settled, so an encrypted document is opened
			// once with the password rather than twice without it.
			//
			// The pass replaces what each page had with the words read off its picture, so what is
			// left of a page that was read is its text and nothing else. A page the engine read
			// nothing from keeps what it had, which is why this is not the same as stripping every
			// instruction afterwards.
			ocr::pages(&mut doc, &context.file_path, context.password.as_deref(), ocr)?;
		}
		let handle = paperback_core::document::DocumentHandle::new(doc);
		Ok(if cli.metadata {
			metadata(handle.document())
		} else {
			let format = match cli.format {
				Format::Text => ExportFormat::Text,
				Format::Html => ExportFormat::Html,
				Format::Markdown => ExportFormat::Markdown,
			};
			export::render(&handle, format)
		})
	}
}

/// Puts one document's output where this run was told to put it.
///
/// `written` is what this run has already put there, so that two documents wanting one name is
/// caught. `--force` is about files that were there before pb started; two documents in one run
/// arriving at one name is pb's own naming, and overwriting the first with the second would lose a
/// book on the strength of a flag the reader meant for something else entirely.
fn write_converted(
	cli: &Cli,
	destination: &Destination<'_>,
	input: &Path,
	selection: Option<&PageSelection>,
	converted: &str,
	written: &mut HashMap<PathBuf, PathBuf>,
) -> Result<()> {
	let path = match destination {
		Destination::Stdout => return write_stdout(converted),
		Destination::File(path) => path.to_path_buf(),
		Destination::Directory(dir) => dir.join(target_for(cli, input, selection)),
	};
	// `--output-dir .` on a folder of books is a natural thing to type and destroys the folder on
	// the second run, because each document's output lands on the next document's name. Refused
	// whatever --force says: the reader is not replacing a file, they are replacing their inputs.
	if inputs::same_file(&path) == inputs::same_file(input) {
		bail!("{} is the document being converted; write somewhere else", path.display());
	}
	if let Some(earlier) = written.get(&inputs::same_file(&path)) {
		bail!("{} and {} are both writing {}", earlier.display(), input.display(), path.display());
	}
	// Asked to be left alone rather than replaced. A run over a folder writes names nobody chose
	// individually, and a reader who runs the same command twice should be told which files the
	// second run would replace rather than find out afterwards.
	if path.exists() && !cli.force {
		bail!("{} is already there; pass --force to replace it", path.display());
	}
	written.insert(inputs::same_file(&path), input.to_path_buf());
	write_file(&path, converted, !cli.metadata && matches!(cli.format, Format::Markdown))
}

/// The name one document's output gets inside `--output-dir`.
fn target_for(cli: &Cli, input: &Path, selection: Option<&PageSelection>) -> PathBuf {
	let stem = input.file_stem().map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
	output_name(&stem, cli.format, selection)
}

/// A document's output file named after the document, the pages asked for, and the format.
///
/// The page range is in the name because two runs over one folder ask for different extracts, and
/// without it the second silently replaces the first. Taken from the stem rather than the whole
/// name, so that `book.epub` and `book.pdf` in one folder both want `book.md` -- which is why a
/// run tracks what it has written rather than letting the second take the first's place.
fn output_name(stem: &str, format: Format, selection: Option<&PageSelection>) -> PathBuf {
	let mut name = PathBuf::from(stem);
	if let Some(selection) = selection {
		let _ = write!(name.as_mut_os_string(), ".p{}", selection.label());
	}
	let _ = write!(name.as_mut_os_string(), ".{}", format.extension());
	name
}

/// Writes one document's output to a file.
///
/// A UTF-8 BOM goes in front of Markdown written to a file, so that editors like `EdSharp` detect
/// the encoding. It goes there rather than in the bytes themselves because a Markdown file with
/// one is what those editors expect, and stdout is left alone: there the reader is a pipe.
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

/// Writes the output to stdout, treating a reader that stopped early as a normal end.
///
/// `pb book.pdf | head` closes the pipe once it has its lines; `print!` panics on the failed
/// write, which reads as a crash in a script that only wanted the start of a book.
fn write_stdout(text: &str) -> Result<()> {
	let mut out = io::stdout().lock();
	match out.write_all(text.as_bytes()).and_then(|()| out.flush()) {
		Err(e) if is_closed_pipe(&e) => Ok(()),
		result => result.context("failed to write to stdout"),
	}
}

/// Whether a write failed because the other end of the pipe has gone away. Windows reports a
/// closed pipe as `ERROR_NO_DATA` (232) as well as the `ERROR_BROKEN_PIPE` that maps to
/// [`io::ErrorKind::BrokenPipe`].
fn is_closed_pipe(error: &io::Error) -> bool {
	error.kind() == io::ErrorKind::BrokenPipe || (cfg!(windows) && error.raw_os_error() == Some(232))
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

#[cfg(test)]
mod tests {
	use std::{
		env, fs,
		path::PathBuf,
		process,
		sync::atomic::{AtomicU64, Ordering},
	};

	use super::*;

	/// A directory of its own for each test, removed with it. paperback-core has one of these but
	/// keeps it behind `cfg(test)`, where another crate cannot reach it.
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

	/// Two runs over one folder asking for different extracts must not land on the same file. This
	/// is the whole reason the range is in the name, and the test that says so.
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

	/// `--pages 1-5;20` must not be named `p1-5-20`, which reads as one range from 1 to 520. The
	/// character holding one range together is not the character holding several apart.
	#[test]
	fn disjoint_ranges_are_not_run_together_into_one_range() {
		let name = named("book", Format::Text, Some("1-5;20"));
		assert_eq!(name, "book.p1-5_20.txt");
		assert!(!name.contains("1-5-20"), "{name} reads as a single range from 1 to 520");
	}

	/// Three disjoint ranges stay three, however they were typed.
	#[test]
	fn several_disjoint_ranges_each_keep_their_own_bounds() {
		assert_eq!(named("book", Format::Text, Some("1-5;20;80-end")), "book.p1-5_20_80-end.txt");
		assert_eq!(named("book", Format::Text, Some("7;20;33")), "book.p7_20_33.txt");
	}

	/// A single page is one number, not a range that happens to have equal ends: `book.p7.md` reads
	/// as the page that was asked for, where `book.p7-7.md` reads as a range typed the long way.
	#[test]
	fn a_single_page_is_not_written_as_a_range() {
		assert_eq!(named("book", Format::Markdown, Some("7")), "book.p7.md");
	}

	/// Two spellings of the same request name the same file, because they are the same extract.
	/// `--pages 5-10,7-12` merges to 5-12, and a reader who typed the overlap must not end up with
	/// a second file holding what the first already holds.
	#[test]
	fn ranges_are_named_after_what_they_merge_to() {
		assert_eq!(named("book", Format::Markdown, Some("5-10,7-12")), named("book", Format::Markdown, Some("5-12")));
	}

	/// The stem rather than the whole name, so `book.epub` becomes `book.md` and not
	/// `book.epub.md`.
	#[test]
	fn the_extension_of_the_input_is_replaced_not_kept() {
		assert_eq!(output_name("book", Format::Markdown, None), PathBuf::from("book.md"));
		assert!(!named("book", Format::Markdown, None).contains(".epub"));
	}

	/// `-o` names one file. Given several documents there is nowhere to put the second that is not
	/// a lie about what the first is, so the reader is told about `--output-dir` instead.
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

	/// Several documents to stdout would run into each other with nothing between them, and in
	/// HTML one document would start inside another. There is no honest way to put them there.
	#[test]
	fn several_documents_cannot_be_sent_to_stdout() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub", "b.epub"]).expect("parse");
		let error = destination(&cli, 2).expect_err("two documents, one stdout").to_string();
		assert!(error.contains("--output-dir"), "{error} does not say what to use instead");
	}

	/// Nothing already on disk is replaced without `--force`, wherever the output was headed.
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
			write_converted(&cli, &destination(&cli, 1).expect("destination"), &input, None, "new text", &mut written)
		};
		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap()])
			.expect_err("an existing file must not be replaced without --force");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "what was there before");
		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap(), "--force"]).expect("--force replaces it");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "new text");
	}

	/// `a/book.txt` and `b/book.txt` both want `out/book.txt`. `--force` is about files that were
	/// there before pb started, so it cannot be what lets the second book overwrite the first.
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
		write_converted(&cli, &destination, &first, None, "the first book", &mut written).expect("the first book");
		let error = write_converted(&cli, &destination, &second, None, "the second book", &mut written)
			.expect_err("--force must not let the second book overwrite the first");
		assert!(error.to_string().contains("both writing"), "{error}");
		assert_eq!(fs::read_to_string(out.join("book.txt")).expect("read"), "the first book");
	}

	/// `--output-dir .` is a natural thing to type, and it replaces each document with the next.
	#[test]
	fn a_document_is_never_written_over_itself() {
		let dir = TempDir::new("self");
		let input = dir.join("book.md");
		fs::write(&input, b"the book").expect("write");
		let cli = cli::Cli::try_parse_from(["pb", "book.md", "--output-dir", dir.path.to_str().unwrap(), "-f", "md"])
			.expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &input, None, "converted", &mut written)
			.expect_err("a document must not be replaced by its own conversion");
		assert_eq!(fs::read_to_string(&input).expect("read"), "the book");
	}

	/// A file that is not there is written without ceremony.
	#[test]
	fn an_output_path_that_is_free_is_written_without_asking() {
		let dir = TempDir::new("fresh");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");
		let fresh = dir.join("out.txt");
		let cli = cli::Cli::try_parse_from(["pb", "book.epub", "-o", fresh.to_str().unwrap()]).expect("parse");
		let destination = destination(&cli, 1).expect("destination");
		let mut written = HashMap::new();
		write_converted(&cli, &destination, &input, None, "new text", &mut written)
			.expect("writing to a path that is free");
		assert_eq!(fs::read_to_string(&fresh).expect("read"), "new text");
	}

	/// The prefix pb recognises a password by is a sentinel between pieces of this code, and a
	/// reader whose password was refused should be told that rather than shown the field name.
	#[test]
	fn the_password_marker_is_not_shown_to_the_reader() {
		let error = parse_failed(
			Path::new("alpha.pdf"),
			&anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect"),
		);
		let printed = error.to_string();
		assert!(!printed.contains(PASSWORD_REQUIRED_ERROR_PREFIX), "{printed}");
		assert!(printed.contains("alpha.pdf"), "{printed} does not name the document");
		assert!(printed.contains("Password required or incorrect"), "{printed}");
	}

	/// An error that is not about a password keeps its own wording.
	#[test]
	fn an_ordinary_parse_error_is_left_alone() {
		let printed = parse_failed(Path::new("x.pdf"), &anyhow::anyhow!("Failed to open PDF document")).to_string();
		assert!(printed.contains("Failed to open PDF document"), "{printed}");
	}

	/// A password that was given and refused is an ordinary failure, reported the way the parser
	/// reported it. It used to have a message of its own and be counted apart from the failures,
	/// which made a run whose first document was prompted for and whose second was not say
	/// "1 failed, 1 needing a password" about two documents that had both turned on the password.
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

	/// Only `--no-prompt` produces a password problem of its own, and its message still says which
	/// document it is about and how to supply one.
	#[test]
	fn a_password_message_names_its_document_and_says_what_to_do() {
		let needed = format!("{:#}", anyhow::Error::new(NeedsPassword).context("locked.pdf"));
		assert!(needed.contains("locked.pdf"), "{needed}");
		assert!(needed.contains("-p"), "{needed} does not say how to supply one");
	}

	/// `is_pattern` answers the one question it is asked: is this something to expand. A file that
	/// happens to be named with a star in it is that file, and the distinction is the whole reason
	/// the check exists.
	#[test]
	fn a_pattern_is_only_a_pattern_when_no_such_file_is_there() {
		let dir = TempDir::new("is-pattern");
		let real = dir.join("book.pdf");
		fs::write(&real, b"x").expect("write");
		assert!(!inputs::is_pattern(&real), "a file that exists is a path whatever else it holds");
		assert!(inputs::is_pattern(&dir.join("*.pdf")));
	}

	/// A run remembers one password and offers it to every document after the first, rather than
	/// asking again: a folder of books under the same password is otherwise the same question
	/// answered once per book.
	#[test]
	fn a_run_keeps_the_password_it_was_given() {
		let cli = cli::Cli::try_parse_from(["pb", "a.pdf"]).expect("parse");
		let mut converter = Converter::new(&cli);
		assert!(converter.password().is_none(), "nothing has been asked for yet");

		converter.remembered = Some("hunter2".to_string());
		assert_eq!(converter.password().map(String::as_str), Some("hunter2"));

		// `-p` is the reader's own instruction for every document, so it is what is used.
		let given = cli::Cli::try_parse_from(["pb", "a.pdf", "-p", "abc"]).expect("parse");
		let mut converter = Converter::new(&given);
		converter.remembered = Some("hunter2".to_string());
		assert_eq!(converter.password().map(String::as_str), Some("abc"), "-p must win over a remembered one");
	}
}
