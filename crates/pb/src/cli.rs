use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pb", about = "Convert any document to text, HTML, or Markdown")]
pub struct Cli {
	/// Input document file
	pub input: PathBuf,
	/// Output format
	#[arg(short, long, default_value = "text")]
	pub format: Format,
	/// Write output to a file instead of stdout
	#[arg(short, long)]
	pub output: Option<PathBuf>,
	/// Password for encrypted documents (omit to be prompted interactively)
	#[arg(short, long)]
	pub password: Option<String>,
	/// Print document metadata instead of content
	#[arg(short, long)]
	pub metadata: bool,
	/// Exit with code 2 instead of prompting for a password (useful for batch processing)
	#[arg(long)]
	pub no_prompt: bool,
}

#[derive(Clone, ValueEnum)]
pub enum Format {
	#[value(alias = "txt")]
	Text,
	#[value(alias = "htm")]
	Html,
	#[value(alias = "md")]
	Markdown,
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

	#[test]
	fn defaults_to_text_output_on_stdout() {
		let cli = parse(&["pb", "book.epub"]);
		assert_eq!(cli.input, PathBuf::from("book.epub"));
		assert!(matches!(cli.format, Format::Text));
		assert!(cli.output.is_none());
		assert!(cli.password.is_none());
		assert!(!cli.metadata);
		assert!(!cli.no_prompt);
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

	/// Paths that start with a dash or contain spaces reach the parser intact rather than being
	/// read as flags.
	#[test]
	fn treats_awkward_paths_as_input() {
		let cli = parse(&["pb", "--", "-weird name.txt"]);
		assert_eq!(cli.input, PathBuf::from("-weird name.txt"));
	}
}
