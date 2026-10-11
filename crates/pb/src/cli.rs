use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pb", about = "Convert any document to text, HTML, or Markdown")]
pub struct Cli {
	/// Input document files, patterns or http(s) links. `*` and `?` are expanded, so the same command line works on Windows. Quote a link that holds ? or &
	#[arg(required_unless_present_any = ["list_formats", "files_from"], num_args = 1..)]
	pub input: Vec<PathBuf>,
	/// Also convert the documents listed in this file, one path per line; `-` reads the list from stdin. Spaces around a name are removed, blank lines and lines starting with # are skipped, and patterns are not expanded
	#[arg(long, value_name = "FILE")]
	pub files_from: Option<PathBuf>,
	/// List the formats pb can read, and the extensions it knows them by
	#[arg(long)]
	pub list_formats: bool,
	/// Output format
	#[arg(short, long, default_value = "text")]
	pub format: Format,
	/// Write output to a file instead of stdout. Only for a single input; use --output-dir for several
	#[arg(short, long, conflicts_with = "output_dir")]
	pub output: Option<PathBuf>,
	/// Write one file per input into this folder, named after each document
	#[arg(long)]
	pub output_dir: Option<PathBuf>,
	/// Replace an output file that is already there, rather than leaving it alone
	#[arg(long)]
	pub force: bool,
	/// Password for encrypted documents (omit to be prompted interactively)
	#[arg(short, long)]
	pub password: Option<String>,
	/// Convert only these pages of every input, as ranges: 5-10, 55, 80-end (comma or semicolon separated)
	#[arg(long)]
	pub pages: Option<String>,
	/// Keep the headers and footers a document repeats from page to page, which are taken out by default as the app does
	#[arg(long)]
	pub keep_repeated: bool,
	/// Read the text off pages that are a picture and nothing else, as Batch OCR does
	#[arg(long)]
	pub ocr_image_pages: bool,
	/// Read the text off pages that already have text, for a text layer worth replacing
	#[arg(long)]
	pub ocr_text_pages: bool,
	/// Print document metadata instead of content
	#[arg(short, long)]
	pub metadata: bool,
	/// Never ask: exit with code 2 instead of prompting for a password, and refuse a link that gets a security warning instead of asking whether to download it (useful for batch processing)
	#[arg(long)]
	pub no_prompt: bool,
	/// The largest document to download from a link, in MB
	#[arg(long, value_name = "MB", default_value_t = 512)]
	pub max_download_size: u64,
	/// Keep every line of an untagged PDF page separate instead of joining wrapped lines back
	/// into paragraphs (for code listings, poetry and transcripts)
	#[arg(long)]
	pub no_join_paragraphs: bool,
	/// Print paperback-core's parser log output to stderr (set `RUST_LOG` for finer control)
	#[arg(short, long)]
	pub verbose: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
	#[value(alias = "txt")]
	Text,
	#[value(alias = "htm")]
	Html,
	#[value(alias = "md")]
	Markdown,
}

impl Format {
	/// The extension a file of this format is given, which is what names it in `--output-dir`.
	#[must_use]
	pub const fn extension(self) -> &'static str {
		match self {
			Self::Text => "txt",
			Self::Html => "html",
			Self::Markdown => "md",
		}
	}
}

#[cfg(test)]
mod tests {
	use clap::CommandFactory;

	use super::*;

	fn parse(args: &[&str]) -> Cli {
		Cli::try_parse_from(args).expect("parse args")
	}

	/// Catches conflicting flags, duplicate short options and other definition mistakes that
	/// clap only reports at runtime.
	#[test]
	fn the_command_definition_is_valid() {
		Cli::command().debug_assert();
	}

	#[test]
	fn input_is_required() {
		assert!(Cli::try_parse_from(["pb"]).is_err(), "input path must be required");
	}

	/// Asking what pb reads is a question about pb, not about a file, so it takes none.
	#[test]
	fn listing_the_formats_needs_no_input() {
		let cli = parse(&["pb", "--list-formats"]);
		assert!(cli.list_formats);
		assert_eq!(cli.input, Vec::<PathBuf>::new());
	}

	#[test]
	fn defaults_to_text_output_on_stdout() {
		let cli = parse(&["pb", "book.epub"]);
		assert_eq!(cli.input, [PathBuf::from("book.epub")]);
		assert!(matches!(cli.format, Format::Text));
		assert!(cli.output.is_none());
		assert!(cli.output_dir.is_none());
		assert!(!cli.force);
		assert!(cli.password.is_none());
		assert!(!cli.metadata);
		assert!(!cli.no_prompt);
		assert!(!cli.no_join_paragraphs);
		assert!(!cli.verbose);
	}

	/// One file per argument, in the order given. A Windows shell hands `*.pdf` over as one
	/// literal argument, so several can arrive having been named one at a time as well as several
	/// having been expanded before pb saw them.
	#[test]
	fn takes_several_inputs() {
		let cli = parse(&["pb", "a.epub", "b.pdf", "c.docx"]);
		assert_eq!(cli.input, [PathBuf::from("a.epub"), PathBuf::from("b.pdf"), PathBuf::from("c.docx")]);
	}

	/// A pattern reaches the parser whole rather than being rejected as an option, so `inputs`
	/// gets to expand it and say why when it matches nothing.
	#[test]
	fn a_pattern_reaches_the_parser_whole() {
		assert_eq!(parse(&["pb", "*.pdf"]).input, [PathBuf::from("*.pdf")]);
		assert_eq!(parse(&["pb", "books/*/ch?.epub"]).input, [PathBuf::from("books/*/ch?.epub")]);
	}

	#[test]
	fn reads_the_output_directory_and_the_force_flag() {
		let cli = parse(&["pb", "a.epub", "--output-dir", "out", "--force"]);
		assert_eq!(cli.output_dir, Some(PathBuf::from("out")));
		assert!(cli.force);
		assert!(!parse(&["pb", "a.epub", "--output-dir", "out"]).force);
	}

	/// `--force` applies wherever the output lands: refusing to replace a named file as well as a
	/// folder is the whole of it, and a flag that only guarded one of the two would leave the
	/// other silently overwriting.
	#[test]
	fn the_force_flag_is_accepted_wherever_the_output_goes() {
		assert!(parse(&["pb", "a.epub", "-o", "out.txt", "--force"]).force);
		assert!(parse(&["pb", "a.epub", "-o", "out.txt", "-f", "md", "--force"]).force);
		assert!(parse(&["pb", "a.epub", "--output-dir", "out", "--force"]).force);
		assert!(!parse(&["pb", "a.epub", "-o", "out.txt"]).force);
	}

	/// Naming one file to write to and a folder to write into are two different instructions, and
	/// taking both means guessing which one was meant.
	#[test]
	fn a_named_file_and_an_output_directory_are_mutually_exclusive() {
		assert!(Cli::try_parse_from(["pb", "a.epub", "-o", "out.txt", "--output-dir", "out"]).is_err());
	}

	#[test]
	fn accepts_the_verbose_flag() {
		assert!(parse(&["pb", "b.epub", "-v"]).verbose);
		assert!(parse(&["pb", "b.epub", "--verbose"]).verbose);
	}

	#[test]
	fn accepts_format_names_and_their_aliases() {
		assert!(matches!(parse(&["pb", "b.epub", "--format", "html"]).format, Format::Html));
		assert!(matches!(parse(&["pb", "b.epub", "--format", "htm"]).format, Format::Html));
		assert!(matches!(parse(&["pb", "b.epub", "-f", "markdown"]).format, Format::Markdown));
		assert!(matches!(parse(&["pb", "b.epub", "-f", "md"]).format, Format::Markdown));
		assert!(matches!(parse(&["pb", "b.epub", "-f", "txt"]).format, Format::Text));
	}

	#[test]
	fn rejects_an_unknown_format() {
		assert!(Cli::try_parse_from(["pb", "b.epub", "--format", "pdf"]).is_err());
	}

	#[test]
	fn reads_the_remaining_options() {
		let cli = parse(&["pb", "b.docx", "-o", "out.txt", "-p", "hunter2", "--metadata", "--no-prompt"]);
		assert_eq!(cli.output, Some(PathBuf::from("out.txt")));
		assert_eq!(cli.password.as_deref(), Some("hunter2"));
		assert!(cli.metadata);
		assert!(cli.no_prompt);
	}

	#[test]
	fn accepts_the_no_join_paragraphs_flag() {
		assert!(parse(&["pb", "b.pdf", "--no-join-paragraphs"]).no_join_paragraphs);
	}

	/// The specification is left as typed rather than parsed here, so that the error a reader gets
	/// for a bad one comes from the code that knows what a page is.
	#[test]
	fn the_pages_flag_is_handed_over_whole() {
		assert_eq!(parse(&["pb", "b.pdf", "--pages", "5-10,80-end"]).pages.as_deref(), Some("5-10,80-end"));
		assert_eq!(parse(&["pb", "b.pdf"]).pages, None);
	}

	/// pb has always read a document the way the app does, running heads taken out, so a script written against it keeps getting the same text. Keeping them is the request.
	#[test]
	fn repeated_lines_are_taken_out_unless_asked_to_keep_them() {
		assert!(!parse(&["pb", "b.pdf"]).keep_repeated);
		assert!(!parse(&["pb", "b.pdf", "--pages", "1-5"]).keep_repeated);
		assert!(parse(&["pb", "b.pdf", "--keep-repeated"]).keep_repeated);
	}

	/// No flag turns the OCR instruction back on. --ocr-image-pages is not such a flag: it reads the
	/// pages the instruction marks and takes the instruction out of the file, so a conversion that
	/// prints it is a fault in the pass rather than something the reader asked for. Asserted against
	/// the command's own argument list, since parsing sample command lines would pass just as well
	/// against a flag that existed but was never spelled out in them.
	#[test]
	fn no_flag_offers_the_ocr_placeholder() {
		let flags: Vec<String> =
			Cli::command().get_arguments().filter_map(|arg| arg.get_long().map(str::to_string)).collect();
		assert!(
			!flags.iter().any(|flag| flag.contains("placeholder")),
			"pb offers a flag for the OCR instruction itself: {flags:?}"
		);
		assert!(flags.contains(&"ocr-image-pages".to_string()), "the flag that reads scanned pages is gone: {flags:?}");
		assert!(flags.contains(&"ocr-text-pages".to_string()), "the flag that re-reads text pages is gone: {flags:?}");
	}

	#[test]
	fn accepts_the_ocr_image_pages_flag() {
		assert!(!parse(&["pb", "b.pdf"]).ocr_image_pages);
		assert!(parse(&["pb", "b.pdf", "--ocr-image-pages"]).ocr_image_pages);
	}

	/// The two flags ask for disjoint kinds of page, so each stands on its own. Neither requires the
	/// other, since a reader who wants scanned pages re-read and a reader who wants text pages
	/// re-read are asking for different work.
	#[test]
	fn either_page_flag_can_be_given_on_its_own() {
		let images = parse(&["pb", "b.pdf", "--ocr-image-pages"]);
		assert!(images.ocr_image_pages);
		assert!(!images.ocr_text_pages);

		let text = parse(&["pb", "b.pdf", "--ocr-text-pages"]);
		assert!(text.ocr_text_pages);
		assert!(!text.ocr_image_pages);

		let both = parse(&["pb", "b.pdf", "--ocr-image-pages", "--ocr-text-pages"]);
		assert!(both.ocr_image_pages);
		assert!(both.ocr_text_pages);

		assert!(!parse(&["pb", "b.pdf"]).ocr_image_pages);
		assert!(!parse(&["pb", "b.pdf"]).ocr_text_pages);
	}

	#[test]
	fn downloads_are_limited_to_512_mb_unless_told_otherwise() {
		assert_eq!(parse(&["pb", "https://example.org/a.pdf"]).max_download_size, 512);
		assert_eq!(parse(&["pb", "https://example.org/a.pdf", "--max-download-size", "2048"]).max_download_size, 2048);
	}

	#[test]
	fn a_list_of_inputs_can_be_named_with_or_without_typed_ones() {
		let cli = parse(&["pb", "--files-from", "-", "--output-dir", "out"]);
		assert_eq!(cli.files_from, Some(PathBuf::from("-")));
		assert_eq!(cli.input, Vec::<PathBuf>::new());
		let cli = parse(&["pb", "a.epub", "--files-from", "list.txt"]);
		assert_eq!(cli.files_from, Some(PathBuf::from("list.txt")));
		assert_eq!(cli.input, [PathBuf::from("a.epub")]);
		assert!(parse(&["pb", "a.epub"]).files_from.is_none());
	}

	/// Paths that start with a dash or contain spaces reach the parser intact rather than being
	/// read as flags.
	#[test]
	fn treats_awkward_paths_as_input() {
		let cli = parse(&["pb", "--", "-weird name.txt"]);
		assert_eq!(cli.input, [PathBuf::from("-weird name.txt")]);
	}

	/// Each format names its files differently, and a reader who asked for Markdown should not
	/// find `.txt` beside their book wondering what happened to the request.
	#[test]
	fn every_format_names_its_files() {
		assert_eq!(Format::Text.extension(), "txt");
		assert_eq!(Format::Html.extension(), "html");
		assert_eq!(Format::Markdown.extension(), "md");
		let mut extensions = [Format::Text, Format::Html, Format::Markdown].map(Format::extension).to_vec();
		extensions.sort_unstable();
		extensions.dedup();
		assert_eq!(extensions.len(), 3, "two formats would write over each other's names");
	}
}
