//! Prints every character the PDF library reads off one page, for comparing the pdfium and hayro backends: `cargo run -p paperback-core --release --example pdf_chars -- <file> <page>`, once with `--features hayro` and once without.

use std::{env, path::Path, process::ExitCode};

use paperback_core::{dump_pdf_page_chars, set_pdfium_library_path};

fn main() -> ExitCode {
	let args: Vec<String> = env::args().skip(1).collect();
	let (Some(path), Some(page)) = (args.first(), args.get(1).and_then(|page| page.parse().ok())) else {
		eprintln!("usage: pdf_chars <file> <page, zero-based>");
		return ExitCode::FAILURE;
	};
	// An example binary lands one folder below the pdfium library the desktop build puts in `target/<profile>`.
	if let Ok(exe) = env::current_exe()
		&& let Some(profile) = exe.parent().and_then(Path::parent)
	{
		set_pdfium_library_path(profile.to_string_lossy().into_owned());
	}
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
