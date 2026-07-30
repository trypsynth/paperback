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
