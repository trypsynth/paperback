//! Snapshots how Paperback reads every PDF in a folder, so a change to the PDF parser or to the library under it shows up as a diff rather than as a reader's bug report.
//!
//! `cargo run -p paperback-core --release --example pdf_corpus -- <folder> [--bless]`
//!
//! Each PDF's snapshot is its metadata, its table of contents, and its text with every marker written inline where it falls. The first run writes them under `<folder>/snapshots`; later runs compare against them, write a `.new` file beside each one that changed, and exit nonzero. `--bless` accepts the new output. An encrypted file reads its password from a `<file>.password` beside it.
//!
//! The PDFs themselves are never committed: most of them are other people's documents.

use std::{
	env,
	fmt::Write as _,
	fs,
	path::{Path, PathBuf},
	process::ExitCode,
	time::Instant,
};

use paperback_core::{
	document::{Document, MarkerType, ParserContext, TocItem},
	parser::parse_document,
};

fn main() -> ExitCode {
	let args: Vec<String> = env::args().skip(1).collect();
	let bless = args.iter().any(|arg| arg == "--bless");
	let Some(folder) = args.iter().find(|arg| !arg.starts_with("--")).map(PathBuf::from) else {
		eprintln!("usage: pdf_corpus <folder> [--bless]");
		return ExitCode::FAILURE;
	};
	let snapshots = folder.join("snapshots");
	let mut pdfs = Vec::new();
	collect_pdfs(&folder, &snapshots, &mut pdfs);
	pdfs.sort();
	let (mut same, mut changed, mut new, mut failed) = (0, 0, 0, 0);
	for pdf in &pdfs {
		let relative = pdf.strip_prefix(&folder).unwrap_or(pdf);
		let snapshot = snapshots.join(relative).with_extension("txt");
		let started = Instant::now();
		let dump = match read(pdf) {
			Ok(document) => dump(&document),
			Err(error) => format!("error: {error}\n"),
		};
		let elapsed = started.elapsed().as_millis();
		if dump.starts_with("error: ") {
			failed += 1;
		}
		if let Some(parent) = snapshot.parent() {
			let _ = fs::create_dir_all(parent);
		}
		let status = match fs::read_to_string(&snapshot) {
			Ok(previous) if previous == dump => {
				same += 1;
				"same".to_string()
			}
			Ok(previous) if !bless => {
				changed += 1;
				let _ = fs::write(snapshot.with_extension("txt.new"), &dump);
				format!("CHANGED at line {}", first_difference(&previous, &dump))
			}
			Ok(_) => {
				changed += 1;
				let _ = fs::write(&snapshot, &dump);
				let _ = fs::remove_file(snapshot.with_extension("txt.new"));
				"blessed".to_string()
			}
			Err(_) => {
				new += 1;
				let _ = fs::write(&snapshot, &dump);
				"new".to_string()
			}
		};
		println!("{elapsed:>6} ms  {status:<24} {}", relative.display());
	}
	println!("{} pdfs: {same} same, {changed} changed, {new} new, {failed} failed to open", pdfs.len());
	if changed > 0 && !bless { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

fn collect_pdfs(folder: &Path, snapshots: &Path, out: &mut Vec<PathBuf>) {
	let Ok(entries) = fs::read_dir(folder) else { return };
	for entry in entries.flatten() {
		let path = entry.path();
		if path == snapshots {
			continue;
		}
		if path.is_dir() {
			collect_pdfs(&path, snapshots, out);
		} else if path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("pdf")) {
			out.push(path);
		}
	}
}

fn read(pdf: &Path) -> anyhow::Result<Document> {
	let mut context = ParserContext::new(pdf.to_string_lossy().into_owned());
	let mut password_file = pdf.as_os_str().to_owned();
	password_file.push(".password");
	if let Ok(password) = fs::read_to_string(password_file) {
		context = context.with_password(password.trim_end_matches(['\r', '\n']).to_string());
	}
	parse_document(&context)
}

fn dump(document: &Document) -> String {
	let mut out = format!("title: {}\nauthor: {}\ntoc:\n", document.title, document.author);
	dump_toc(&document.toc_items, 1, &mut out);
	out.push_str("text:\n");
	out.push_str(&annotated_text(document));
	if !out.ends_with('\n') {
		out.push('\n');
	}
	out
}

fn dump_toc(items: &[TocItem], depth: usize, out: &mut String) {
	for item in items {
		let _ = writeln!(out, "{}{} @{}", "  ".repeat(depth), item.name, item.offset);
		dump_toc(&item.children, depth + 1, out);
	}
}

/// The text with each marker written as `⟦kind⟧` where it starts and `⟦/kind⟧` where a marker with a length ends.
fn annotated_text(document: &Document) -> String {
	let buffer = &document.buffer;
	let mut tags = Vec::new();
	for (order, marker) in buffer.markers.iter().enumerate() {
		let kind = kind(marker.mtype);
		let mut open = format!("⟦{kind}");
		if marker.level != 0 {
			let _ = write!(open, " level={}", marker.level);
		}
		if !marker.reference.is_empty() {
			let _ = write!(open, " ref={}", marker.reference);
		}
		open.push('⟧');
		tags.push(Tag { at: marker.position.get(), opens: true, order, text: open });
		if !marker.length.is_zero() {
			let at = (marker.position + marker.length).get();
			tags.push(Tag { at, opens: false, order, text: format!("⟦/{kind}⟧") });
		}
	}
	// A marker that ends where the next one starts closes first, so the two never read as nested.
	tags.sort_by_key(|tag| (tag.at, tag.opens, tag.order));
	let mut out = String::with_capacity(buffer.content.len() + tags.len() * 12);
	let mut pending = tags.iter().peekable();
	for (index, ch) in buffer.content.chars().enumerate() {
		while let Some(tag) = pending.next_if(|tag| tag.at <= index) {
			out.push_str(&tag.text);
		}
		out.push(ch);
	}
	for tag in pending {
		out.push_str(&tag.text);
	}
	out
}

struct Tag {
	at: usize,
	opens: bool,
	order: usize,
	text: String,
}

fn kind(mtype: MarkerType) -> String {
	format!("{mtype:?}").to_lowercase()
}

fn first_difference(previous: &str, current: &str) -> usize {
	previous
		.lines()
		.zip(current.lines())
		.position(|(a, b)| a != b)
		.unwrap_or_else(|| previous.lines().count().min(current.lines().count()))
		+ 1
}
