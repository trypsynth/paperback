use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pb", about = "Convert a document to text or HTML")]
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
	/// Print document metadata (title, author, word count) instead of content
	#[arg(short, long)]
	pub metadata: bool,
}

#[derive(Clone, ValueEnum)]
pub enum Format {
	Text,
	Html,
	Markdown,
}
