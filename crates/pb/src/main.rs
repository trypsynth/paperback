use std::{
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
	// A Windows shell hands `*.pdf` over as one literal argument, so the patterns are expanded
	// here rather than being left for a shell that may not expand them.
	let files = inputs::collect(&cli.input)?;
	// Read up front so a bad specification is refused before a long parse rather than after it.
	// The pages themselves are chosen below, once the document has been read and its length known.
	let selection = cli.pages.as_deref().map(PageSelection::parse).transpose()?;

	let destination = destination(&cli, files.len())?;
	// One document into one named file is the whole of pb until now, and it says nothing when it
	// works. Several documents is a run over a folder, which is worth a report: a hundred files
	// converting silently and two failing quietly is a reader who has to go and look.
	if files.len() == 1 && matches!(destination, Destination::File(_)) {
		let file = &files[0];
		let converted = Converter::new(&cli).convert(file, selection.as_ref())?;
		return write_converted(&cli, &destination, file, selection.as_ref(), &converted);
	}
	run(&cli, &files, selection.as_ref(), &destination)
}

/// Where the output of this run goes, and whether the instruction to put it there is one that can
/// be carried out.
fn destination(cli: &Cli, count: usize) -> Result<Destination<'_>> {
	if let Some(dir) = &cli.output_dir {
		return Ok(Destination::Directory(dir));
	}
	// One named file, several documents: there is nowhere to put the second one that is not a lie
	// about what the first is. Naming `--output-dir` says what was meant.
	if let Some(path) = &cli.output {
		if count > 1 {
			bail!(
				"-o names one file, but {count} documents were given to convert\n\
				 Write them into a folder with --output-dir instead"
			);
		}
		return Ok(Destination::File(path));
	}
	Ok(Destination::Stdout)
}

/// Converts each document in turn, carrying on past the ones that fail and saying so at the end.
///
/// A document that cannot be read is that document's problem, not the run's: over a folder of a
/// hundred books, stopping at the first EPUB with no password leaves the reader with nothing and
/// no way of telling which of the hundred was the problem. Each failure is named as it happens
/// and the run carries on, and the exit code says afterwards that not everything converted.
fn run(cli: &Cli, files: &[PathBuf], selection: Option<&PageSelection>, destination: &Destination<'_>) -> Result<()> {
	if let Destination::Directory(dir) = destination {
		// Made up front rather than per document, so a folder pb cannot create is reported once
		// before anything has been read rather than as the same failure for every file.
		fs::create_dir_all(dir).with_context(|| format!("failed to create the folder {}", dir.display()))?;
	}
	let reporting = files.len() > 1 || matches!(destination, Destination::Directory(_));
	let mut converter = Converter::new(cli);
	let mut tally = Tally::default();
	for file in files {
		// Before the document is touched, not after it is written. A run over a folder is a
		// long silence between the first prompt and the summary, and a reader watching it has
		// nothing to tell whether pb has stopped or is chewing through a nine-hundred-page PDF.
		// It also says which document a password prompt belongs to, which matters once the run
		// has asked only once and is reusing that answer for every book after.
		if reporting {
			eprintln!("pb: converting {}...", file.display());
		}
		let outcome = converter
			.convert(file, selection)
			.and_then(|converted| write_converted(cli, destination, file, selection, &converted));
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
				// Asked at most once in a run. A document that will not open for the password already
				// in hand -- the one `-p` gave, or the one a reader typed for an earlier document --
				// is not asked about again: they have answered, and the answer was wrong. Asking
				// again looks like a second attempt at a document already dealt with, and where
				// there is no terminal to type into -- a script, a pipeline, a run over a folder --
				// it waits there for good.
				//
				// The refusal is then reported exactly as the first document's would be, because it
				// is the same failure. A run that answered "1 failed, 1 needing a password" for two
				// documents that both turned on the password was describing one problem twice, in
				// two vocabularies.
				//
				// The parser cannot tell a wrong password from a missing one:
				// `PdfError::PasswordRequired` covers both, which is why this is asked from what the
				// reader supplied rather than from what came back.
				if self.password().is_some() {
					return Err(e.context(format!("failed to parse {}", input.display())));
				}
				let password = rpassword::prompt_password("Password: ").context("failed to read password")?;
				self.remembered = Some(password.clone());
				context.password = Some(password);
				// A password the reader gets wrong lands here too, and is reported the way any
				// other document that would not parse is.
				parse_document(&context).with_context(|| format!("failed to parse {}", input.display()))?
			}
			Err(e) => return Err(e.context(format!("failed to parse {}", input.display()))),
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
fn write_converted(
	cli: &Cli,
	destination: &Destination<'_>,
	input: &Path,
	selection: Option<&PageSelection>,
	converted: &str,
) -> Result<()> {
	let path = match destination {
		Destination::Stdout => return write_stdout(converted),
		Destination::File(path) => path.to_path_buf(),
		Destination::Directory(dir) => dir.join(target_for(cli, input, selection)),
	};
	// Asked to be left alone rather than replaced. A run over a folder writes names nobody chose
	// individually, and a reader who runs the same command twice should be told which files the
	// second run would replace rather than find out afterwards.
	//
	// This is checked for a named file as well as a folder, which pb did not used to do: `fs::write`
	// replaces whatever is at the path, and `-o out.txt` on a file that already holds something is
	// the reader losing work they never said they were replacing. --force is how they say it.
	if path.exists() && !cli.force {
		bail!("{} is already there; pass --force to replace it", path.display());
	}
	write_file(&path, converted, !cli.metadata && matches!(cli.format, Format::Markdown))
}

/// The name one document's output gets inside `--output-dir`.
fn target_for(cli: &Cli, input: &Path, selection: Option<&PageSelection>) -> PathBuf {
	let stem = input.file_stem().map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
	output_name(&stem, cli.format, selection)
}

/// A document's output file named after the document, the pages asked for, and the format.
///
/// The page range is in the name because two runs of the same command over the same folder ask
/// for different extracts, and without it the second silently replaces the first -- which, of
/// everything this could get wrong, is the one a reader would only notice by having lost the
/// pages they wanted.
///
/// Taken from the document's stem rather than its whole name, so that `book.epub` and `book.pdf`
/// sitting in one folder both come out as `book.md` and collide, which is worth a reader's
/// knowing rather than papering over: two documents of the same name are two different books.
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

	/// The line printed as a document is picked up. It names the document, because that is the
	/// whole of it: a run over a folder is otherwise a silence between the first prompt and the
	/// summary, and a password prompt with nothing above it belongs to no document in particular.
	#[test]
	fn the_starting_line_says_which_document_is_being_read() {
		let line = format!("pb: converting {}...", Path::new("books/py_enc.pdf").display());
		assert!(line.contains("py_enc.pdf"), "{line}");
		assert!(line.starts_with("pb: "), "{line} would not sit with the rest of pb's output");
		assert!(line.ends_with("..."), "{line} does not read as work still in progress");
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
		assert!(matches!(destination(&cli, 5).expect("stdout"), Destination::Stdout));
	}

	/// Nothing already on disk is replaced without `--force`, wherever the output was headed.
	/// This one used to replace a named file silently -- `fs::write` does, and `-o out.txt` on a
	/// file holding something was the reader losing work they never said they were replacing.
	#[test]
	fn an_output_file_that_is_already_there_is_left_alone_without_force() {
		let dir = TempDir::new("overwrite");
		let existing = dir.join("out.txt");
		fs::write(&existing, b"what was there before").expect("write");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");

		let refuse = |args: &[&str]| {
			let cli = cli::Cli::try_parse_from(args).expect("parse");
			write_converted(&cli, &destination(&cli, 1).expect("destination"), &input, None, "new text")
		};

		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap()])
			.expect_err("an existing file must not be replaced without --force");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "what was there before");

		refuse(&["pb", "book.epub", "-o", existing.to_str().unwrap(), "--force"]).expect("--force replaces it");
		assert_eq!(fs::read_to_string(&existing).expect("read"), "new text");
	}

	/// The same rule in a folder, where the name is pb's rather than the reader's.
	#[test]
	fn a_file_in_the_output_directory_that_is_already_there_is_left_alone_without_force() {
		let dir = TempDir::new("dir-overwrite");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");
		let out = dir.join("out");
		fs::create_dir_all(&out).expect("mkdir");
		fs::write(out.join("book.md"), b"what was there before").expect("write");

		let attempt = |force: bool| {
			// The absolute path, because `run` is what makes the folder and this goes straight to
			// the writer; a relative one would resolve against the crate directory.
			let mut args = vec!["pb", "book.epub", "--output-dir", out.to_str().unwrap(), "-f", "md"];
			if force {
				args.push("--force");
			}
			let cli = cli::Cli::try_parse_from(&args).expect("parse");
			let destination = destination(&cli, 1).expect("destination");
			write_converted(&cli, &destination, &input, None, "new text")
		};

		attempt(false).expect_err("an existing file must not be replaced without --force");
		assert_eq!(fs::read_to_string(out.join("book.md")).expect("read"), "what was there before");
		attempt(true).expect("--force replaces it");
		// Markdown written to a file still carries its BOM, so the replacement is checked for the
		// text rather than for an exact match on the bytes.
		let replaced = fs::read_to_string(out.join("book.md")).expect("read");
		assert!(replaced.contains("new text"), "{replaced:?}");
		assert!(replaced.starts_with('\u{feff}'), "the BOM went missing in the rewrite");
	}

	/// A file that is not there is written without ceremony, which is the ordinary case and the
	/// one that must not grow a prompt or a flag.
	#[test]
	fn an_output_path_that_is_free_is_written_without_asking() {
		let dir = TempDir::new("fresh");
		let input = dir.join("book.epub");
		fs::write(&input, b"x").expect("write");
		let fresh = dir.join("out.txt");
		let cli = cli::Cli::try_parse_from(["pb", "book.epub", "-o", fresh.to_str().unwrap()]).expect("parse");
		write_converted(&cli, &destination(&cli, 1).expect("destination"), &input, None, "new text")
			.expect("writing to a path that is free");
		assert_eq!(fs::read_to_string(&fresh).expect("read"), "new text");
	}

	/// A password refused under `--no-prompt` is counted apart from a failure, because the reader
	/// can do something about it and because exit code 2 has meant that since long before this ran
	/// over more than one file.
	#[test]
	fn a_document_held_by_a_password_is_recognised_when_it_reaches_the_tally() {
		let cli = cli::Cli::try_parse_from(["pb", "a.epub", "--no-prompt"]).expect("parse");
		let error = anyhow::Error::new(NeedsPassword).context("a.epub");
		assert!(error.chain().any(is_needs_password), "the run would count this as a plain failure");
		assert!(cli.no_prompt);
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
