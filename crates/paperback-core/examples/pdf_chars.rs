//! Prints every character the PDF engine reads off one page, followed by the page's objects and tag tree, for chasing down how a PDF's text layer was read: `cargo run -p paperback-core --release --example pdf_chars -- <file> <page>`.

use std::{env, process::ExitCode};

use paperback_core::dump_pdf_page_chars;

fn main() -> ExitCode {
	let args: Vec<String> = env::args().skip(1).collect();
	let (Some(path), Some(page)) = (args.first(), args.get(1).and_then(|page| page.parse().ok())) else {
		eprintln!("usage: pdf_chars <file> <page, zero-based>");
		return ExitCode::FAILURE;
	};
	match dump_pdf_page_chars(path, None, page) {
		Ok(dump) => {
			print!("{dump}");
			ExitCode::SUCCESS
		}
		Err(error) => {
			eprintln!("{error}");
			ExitCode::FAILURE
		}
	}
}
