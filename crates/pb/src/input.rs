//! What pb says when it cannot read the file it was given.
//!
//! The check runs before any parser is asked for the file, so that the reason is the one the
//! reader can act on: a path that is not there reads as a missing file rather than as a parser
//! failing to open it, and an unknown extension names the file it came from.

use std::{fs, io, path::Path};

use anyhow::{Result, bail};
use paperback_core::parser::{self, ParserRegistry, RegisteredParser};

/// How far apart two extensions can be before one stops being a plausible typing of the other.
/// One edit covers the slips people make: a dropped letter (`epb`), a doubled one, a
/// transposition (`pfd`), a neighbouring key.
const NEAREST_EXTENSION_EDITS: usize = 1;

/// Reports why `path` cannot be read, or `Ok` if there is nothing in the way of trying.
///
/// # Errors
///
/// Returns an error if the file is missing, is a folder, cannot be read at all, or is of a
/// format no parser is registered for.
pub fn check(path: &Path) -> Result<()> {
	let metadata = match fs::metadata(path) {
		Ok(metadata) => metadata,
		Err(error) if error.kind() == io::ErrorKind::NotFound => {
			bail!("no such file: {}", path.display());
		}
		Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
			bail!("cannot read {}: permission denied", path.display());
		}
		Err(error) => bail!("cannot read {}: {error}", path.display()),
	};
	if metadata.is_dir() {
		bail!("{} is a folder, not a document", path.display());
	}
	if parser::parser_supports_path(path) {
		return Ok(());
	}
	let extension = path.extension().and_then(|extension| extension.to_str()).unwrap_or_default();
	if extension.is_empty() {
		bail!("{} has no file extension, so pb cannot tell which format it is\n{}", path.display(), LIST_HINT);
	}
	if let Some(nearest) = nearest_extension(extension) {
		bail!("pb cannot read .{extension} files: {}\nDid you mean .{nearest}?", path.display());
	}
	bail!("pb cannot read .{extension} files: {}\n{}", path.display(), LIST_HINT)
}

/// Every extension a parser is registered for, in alphabetical order and without repeats.
fn supported_extensions() -> Vec<&'static str> {
	let mut extensions: Vec<&'static str> =
		ParserRegistry::global().all_parsers().iter().flat_map(RegisteredParser::extensions).copied().collect();
	extensions.sort_unstable();
	extensions.dedup();
	extensions
}

/// Where to find the formats pb reads. Forty extensions under every error is a wall to read
/// past, and the reader who wants them can ask.
const LIST_HINT: &str = "Run pb --list-formats to see what it can read.";

/// The one readable extension close enough to `extension` to be worth suggesting, or `None`
/// where nothing is close or more than one thing is.
fn nearest_extension(extension: &str) -> Option<&'static str> {
	let extension = extension.to_ascii_lowercase();
	let mut nearest: Vec<&'static str> = supported_extensions()
		.into_iter()
		.filter(|candidate| edits_between(&extension, candidate) <= NEAREST_EXTENSION_EDITS)
		.collect();
	// Two suggestions are worse than none: the reader has to guess which, and guessing is what
	// got them here.
	(nearest.len() == 1).then(|| nearest.remove(0))
}

/// How many single-character insertions, deletions, substitutions or swaps of two neighbours
/// turn `from` into `to`.
///
/// Swaps count as one edit rather than the two plain Levenshtein charges them, because
/// `.pfd` for `.pdf` is one slip of the fingers and reads to everyone as one mistake.
fn edits_between(from: &str, to: &str) -> usize {
	let from: Vec<char> = from.chars().collect();
	let to: Vec<char> = to.chars().collect();
	// Extensions are a few characters long, so the whole table is clearer than rolling rows.
	let mut table = vec![vec![0usize; to.len() + 1]; from.len() + 1];
	for (i, row) in table.iter_mut().enumerate() {
		row[0] = i;
	}
	for (j, cell) in table[0].iter_mut().enumerate() {
		*cell = j;
	}
	for i in 1..=from.len() {
		for j in 1..=to.len() {
			let cost = usize::from(from[i - 1] != to[j - 1]);
			let mut best = (table[i - 1][j] + 1).min(table[i][j - 1] + 1).min(table[i - 1][j - 1] + cost);
			if i > 1 && j > 1 && from[i - 1] == to[j - 2] && from[i - 2] == to[j - 1] {
				best = best.min(table[i - 2][j - 2] + 1);
			}
			table[i][j] = best;
		}
	}
	table[from.len()][to.len()]
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

	/// A directory of its own for each test, removed with it. paperback-core has one of these
	/// but keeps it behind `cfg(test)`, where another crate cannot reach it.
	struct TempDir {
		path: PathBuf,
	}

	impl TempDir {
		fn new(label: &str) -> Self {
			static COUNTER: AtomicU64 = AtomicU64::new(0);
			let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
			let path = env::temp_dir().join(format!("pb_test_{label}_{}_{unique}", process::id()));
			fs::create_dir_all(&path).expect("create the temp dir");
			Self { path }
		}

		fn path(&self) -> &Path {
			&self.path
		}

		fn join_str(&self, name: &str) -> String {
			self.path.join(name).to_string_lossy().into_owned()
		}
	}

	impl Drop for TempDir {
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.path);
		}
	}

	fn message(path: &Path) -> String {
		check(path).expect_err("the file cannot be read").to_string()
	}

	/// The reader's own complaint: an error that does not name the file leaves them guessing which
	/// one of a batch it was.
	#[test]
	fn every_refusal_names_the_file() {
		let dir = TempDir::new("pb-input");
		let missing = dir.join_str("missing.epub");
		let unreadable = dir.join_str("thing.xyz");
		fs::write(&unreadable, b"x").expect("write");
		let extensionless = dir.join_str("notes");
		fs::write(&extensionless, b"x").expect("write");
		for path in [&missing, &unreadable, &extensionless] {
			let message = message(Path::new(path));
			assert!(message.contains(path), "{message} does not name {path}");
		}
	}

	#[test]
	fn a_missing_file_reads_as_missing_rather_than_as_a_format() {
		let dir = TempDir::new("pb-missing");
		let path = dir.join_str("gone.epub");
		assert!(message(Path::new(&path)).starts_with("no such file:"));
	}

	/// A missing file with an extension nothing reads is still a missing file first: that is the
	/// thing to fix, and the format never came into it.
	#[test]
	fn a_missing_file_is_reported_before_its_extension() {
		let dir = TempDir::new("pb-missing-unknown");
		let path = dir.join_str("gone.xyz");
		assert!(message(Path::new(&path)).starts_with("no such file:"));
	}

	#[test]
	fn a_folder_is_told_apart_from_a_document() {
		let dir = TempDir::new("pb-folder");
		let message = message(dir.path());
		assert!(message.contains("is a folder, not a document"), "{message}");
	}

	#[test]
	fn a_file_with_no_extension_says_so_and_says_where_to_look() {
		let dir = TempDir::new("pb-bare");
		let path = dir.join_str("notes");
		fs::write(&path, b"x").expect("write");
		let message = message(Path::new(&path));
		assert!(message.contains("no file extension"), "{message}");
		assert!(message.contains("--list-formats"), "{message}");
	}

	#[test]
	fn an_unreadable_extension_says_where_the_readable_ones_are() {
		let dir = TempDir::new("pb-unknown");
		let path = dir.join_str("thing.xyz");
		fs::write(&path, b"x").expect("write");
		let message = message(Path::new(&path));
		assert!(message.contains("cannot read .xyz files"), "{message}");
		assert!(message.contains("--list-formats"), "{message}");
	}

	/// A near miss gets the near miss rather than the whole list, which is the answer the reader
	/// wants and is buried in forty other extensions otherwise.
	#[test]
	fn a_typed_extension_is_offered_the_one_it_meant() {
		let dir = TempDir::new("pb-typo");
		for (typed, meant) in [("book.epb", ".epub"), ("book.pfd", ".pdf"), ("book.moby", ".mobi")] {
			let path = dir.join_str(typed);
			fs::write(&path, b"x").expect("write");
			let message = message(Path::new(&path));
			assert!(message.contains(&format!("Did you mean {meant}?")), "{typed}: {message}");
			assert!(!message.contains("--list-formats"), "{typed}: {message}");
		}
	}

	/// The list of forty extensions used to sit under every one of these, which is a wall to
	/// read past for someone who mistyped a name.
	#[test]
	fn no_refusal_runs_past_two_lines() {
		let dir = TempDir::new("pb-short");
		let unreadable = dir.join_str("thing.xyz");
		fs::write(&unreadable, b"x").expect("write");
		let extensionless = dir.join_str("notes");
		fs::write(&extensionless, b"x").expect("write");
		for path in [&unreadable, &extensionless] {
			let message = message(Path::new(path));
			assert!(message.lines().count() <= 2, "{message}");
		}
	}

	#[test]
	fn a_readable_file_has_nothing_said_about_it() {
		let dir = TempDir::new("pb-fine");
		let path = dir.join_str("book.epub");
		fs::write(&path, b"x").expect("write");
		check(Path::new(&path)).expect("nothing in the way of trying");
	}

	#[test]
	fn a_swap_counts_as_one_edit_and_an_unrelated_word_as_many() {
		assert_eq!(edits_between("pdf", "pdf"), 0);
		assert_eq!(edits_between("pfd", "pdf"), 1);
		assert_eq!(edits_between("epb", "epub"), 1);
		assert!(edits_between("xyz", "epub") > NEAREST_EXTENSION_EDITS);
	}

	/// An extension a long way from everything gets no suggestion, and one that is a step from
	/// two things gets none either.
	#[test]
	fn a_suggestion_is_offered_only_when_it_is_the_only_one() {
		assert_eq!(nearest_extension("epb"), Some("epub"));
		assert_eq!(nearest_extension("qqqqq"), None);
		// One edit from cbr and one from cbz, so neither is the answer.
		assert_eq!(nearest_extension("cbx"), None);
	}
}
