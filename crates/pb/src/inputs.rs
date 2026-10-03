//! Turning the paths on the command line into the files to convert.
//!
//! A Unix shell expands `*.pdf` into the list of matching files before pb ever sees it. The
//! Windows shells do not, so a pattern reaches pb as one literal argument, and a reader who
//! learned the habit on one platform is left typing `pb *.pdf` and being told the file `*.pdf`
//! does not exist. Expanding it here means one command line works the same everywhere, and the
//! paths a Unix shell did expand pass through untouched.
//!
//! Only `*` and `?` make an argument a pattern. A `[` is left alone deliberately: it is a legal
//! character in a path on every platform pb runs on, and treating one as the start of a character
//! class would turn `pb notes[final].epub` into a pattern matching nothing. A pattern that also
//! matches nothing is an error rather than an empty run, since `pb *.pdf` in a folder of EPUBs
//! means the reader mistyped it, and converting nothing without a word says so less clearly than
//! saying which pattern found nothing.

use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

use anyhow::{Result, bail};

/// Whether an argument is a pattern to expand rather than a path to convert.
///
/// An argument that names a file which exists is always taken as that file, even if it holds a
/// `*` or a `?`, so a document whose real name contains one of those still converts.
#[must_use]
pub fn is_pattern(arg: &Path) -> bool {
	// `to_string_lossy` rather than a component walk: a pattern is a string the reader typed, and
	// asking the filesystem whether it happens to exist is the only way to tell one apart from a
	// file that really is called that.
	has_magic(arg) && !arg.exists()
}

/// Whether the argument holds a character pb expands.
///
/// Brackets are left out on purpose. `[` is legal in a path on every platform pb runs on, so
/// treating one as the start of a character class would make `pb notes[final].epub` a pattern
/// matching nothing. A doubled star is not a case either: `glob` reads what `**` means.
#[must_use]
fn has_magic(arg: &Path) -> bool {
	arg.as_os_str().to_string_lossy().contains(['*', '?'])
}

/// Every file to convert, in the order the arguments named them.
///
/// # Errors
///
/// Returns an error if an argument holds a pattern that matches no files, or if a pattern cannot
/// be read at all. An argument that is not a pattern is passed through as it is, whether or not
/// it exists: `input::check` says why a missing file is a problem in terms of the reader can act
/// on, which is a better place for that than here.
pub fn collect(args: &[PathBuf]) -> Result<Vec<PathBuf>> {
	let mut files = Vec::new();
	for arg in args {
		if !is_pattern(arg) {
			files.push(arg.clone());
			continue;
		}
		files.extend(expand(arg)?);
	}
	dedupe(&mut files);
	Ok(files)
}

/// The files one pattern matches, sorted.
///
/// Folders are dropped rather than passed on. `pb *` in a folder of books sitting beside some
/// subfolders should convert the books, and `input::check` already refuses a folder with a
/// message saying so; naming every subfolder as a failure would bury the one real problem.
fn expand(pattern: &Path) -> Result<Vec<PathBuf>> {
	let spec = pattern.to_string_lossy();
	let matches = glob::glob(&spec).map_err(|error| anyhow::anyhow!("cannot read the pattern \"{spec}\": {error}"))?;
	let mut files: Vec<PathBuf> = Vec::new();
	for entry in matches {
		let path = entry.map_err(|error| anyhow::anyhow!("cannot read the matches of \"{spec}\": {error}"))?;
		if path.is_file() {
			files.push(path);
		}
	}
	if files.is_empty() {
		bail!("no file matches \"{spec}\"");
	}
	// Sorted, so that two runs over the same folder convert in the same order and report their
	// failures in the same order. `glob` already returns each directory's entries in order, but
	// that is a property of the crate rather than a promise of this interface.
	files.sort();
	Ok(files)
}

/// Removes paths already converted, keeping the first mention of each.
///
/// Two arguments can name one file -- `pb *.pdf book.epub`, or the same glob twice -- and
/// converting it twice writes the same output over itself and reports the same failure twice.
/// The reader asked for it once by the time they see one line.
fn dedupe(files: &mut Vec<PathBuf>) {
	let mut seen = HashSet::new();
	files.retain(|path| seen.insert(normalized(path)));
}

/// The path as a key for "have I seen this one", without the spellings that mean the same file.
///
/// Windows is case-insensitive and treats `a/b` and `a\b` as one path; Unix is not, and a reader
/// who passes both `Book.epub` and `book.epub` on Linux means two files that do not both exist.
/// Normalizing only as far as each platform agrees keeps the comparison honest.
fn normalized(path: &Path) -> PathBuf {
	#[cfg(windows)]
	let path = PathBuf::from(path.to_string_lossy().to_lowercase());
	#[cfg(not(windows))]
	let path = path.to_path_buf();
	// Components rather than the whole string, so `a/../b` and `b` meet as the same file.
	path.components().collect()
}

#[cfg(test)]
mod tests {
	use std::{env, fs, process};

	use super::*;

	/// A directory of its own for each test, removed with it. paperback-core has one of these but
	/// keeps it behind `cfg(test)`, where another crate cannot reach it.
	struct TempDir {
		path: PathBuf,
	}

	impl TempDir {
		fn new(label: &str) -> Self {
			static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
			let unique = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
			let path = env::temp_dir().join(format!("pb_inputs_test_{label}_{}_{unique}", process::id()));
			fs::create_dir_all(&path).expect("create the temp dir");
			Self { path }
		}

		fn join(&self, name: &str) -> PathBuf {
			self.path.join(name)
		}

		/// The pattern a reader would type to reach these files, as a string.
		fn pattern(&self, tail: &str) -> String {
			self.join(tail).to_string_lossy().into_owned()
		}
	}

	impl Drop for TempDir {
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.path);
		}
	}

	fn names(paths: &[PathBuf]) -> Vec<String> {
		paths.iter().map(|path| path.file_name().unwrap_or_default().to_string_lossy().into_owned()).collect()
	}

	#[test]
	fn a_plain_path_is_not_a_pattern() {
		assert!(!is_pattern(Path::new("book.epub")));
		assert!(!is_pattern(Path::new("/books/2024/my book.epub")));
	}

	#[test]
	fn a_star_or_a_question_mark_makes_a_pattern() {
		assert!(is_pattern(Path::new("*.pdf")));
		assert!(is_pattern(Path::new("book?.epub")));
		assert!(is_pattern(Path::new("/books/*/chapters/*.pdf")));
	}

	/// A real file whose name holds a star is that file, not a pattern that matches everything.
	///
	/// Unix only: Windows will not create a file with `*` in its name, so on that platform there
	/// is no such file to be confused with a pattern and the rule has nothing to decide.
	#[cfg(not(windows))]
	#[test]
	fn a_file_that_really_exists_wins_over_being_read_as_a_pattern() {
		let dir = TempDir::new("exists");
		let path = dir.join("we*rd.epub");
		fs::write(&path, b"x").expect("write");
		assert!(!is_pattern(&path), "a file that exists is a path whatever its name holds");
	}

	/// `[` is a legal character in a path on every platform pb runs on, and treating it as the
	/// start of a character class would make this file unreachable.
	#[test]
	fn a_bracket_is_not_a_pattern() {
		assert!(!is_pattern(Path::new("notes[final].epub")));
		assert!(!is_pattern(Path::new("set[1-3].epub")));
	}

	#[test]
	fn a_pattern_expands_to_the_files_it_matches() {
		let dir = TempDir::new("expand");
		for name in ["b.epub", "a.epub", "c.txt"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		let files = collect(&[PathBuf::from(dir.pattern("*.epub"))]).expect("collect");
		assert_eq!(names(&files), ["a.epub", "b.epub"], "not sorted, or the .txt slipped in");
	}

	/// `?` stands for exactly one character, so it separates `a1.txt` from `a12.txt`.
	#[test]
	fn a_question_mark_stands_for_one_character() {
		let dir = TempDir::new("one-char");
		for name in ["a1.txt", "a12.txt"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		let files = collect(&[PathBuf::from(dir.pattern("a?.txt"))]).expect("collect");
		assert_eq!(names(&files), ["a1.txt"]);
	}

	/// A folder is not a document, and naming every subfolder as a failure would bury the one
	/// real problem among them.
	#[test]
	fn folders_matching_the_pattern_are_left_out() {
		let dir = TempDir::new("folders");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let files = collect(&[PathBuf::from(dir.pattern("*"))]).expect("collect");
		assert_eq!(names(&files), ["a.epub"]);
	}

	/// `pb *.pdf` in a folder of EPUBs means a mistyped pattern, and saying nothing at all about
	/// it leaves the reader to work out why their command did nothing.
	#[test]
	fn a_pattern_matching_nothing_is_an_error_naming_the_pattern() {
		let dir = TempDir::new("no-match");
		let pattern = dir.pattern("*.pdf");
		let error = collect(&[PathBuf::from(&pattern)]).expect_err("nothing matches").to_string();
		assert!(error.contains("*.pdf"), "{error} does not name the pattern");
	}

	#[test]
	fn several_patterns_are_all_expanded_in_the_order_they_were_given() {
		let dir = TempDir::new("several");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		fs::write(dir.join("c.pdf"), b"x").expect("write");
		let args = vec![PathBuf::from(dir.pattern("*.pdf")), PathBuf::from(dir.pattern("*.epub"))];
		let files = collect(&args).expect("collect");
		assert_eq!(names(&files), ["a.pdf", "c.pdf", "b.epub"]);
	}

	/// Converting a file twice writes its output over itself and reports its failure twice.
	#[test]
	fn a_file_named_twice_is_converted_once() {
		let dir = TempDir::new("dupe");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		let args = vec![PathBuf::from(dir.pattern("*.epub")), PathBuf::from(dir.join("a.epub"))];
		let files = collect(&args).expect("collect");
		assert_eq!(names(&files), ["a.epub", "b.epub"]);
	}

	/// A path that is not a pattern is passed through whatever is wrong with it, so that
	/// `input::check` can say why in terms the reader can act on.
	#[test]
	fn a_missing_path_is_passed_through_rather_than_refused_here() {
		let files = collect(&[PathBuf::from("nowhere.epub")]).expect("collect");
		assert_eq!(files, [PathBuf::from("nowhere.epub")]);
	}

	#[test]
	fn no_arguments_collect_to_no_files() {
		assert!(collect(&[]).expect("collect").is_empty());
	}
}
