//! Turning the paths on the command line into the files to convert.
//!
//! A Unix shell expands `*.pdf` into the list of matching files before pb ever see it. The
//! Windows shells do not, so a pattern reaches pb as one literal argument, and a reader who
//! learned the habit on one platform is left typing `pb *.pdf` and being told the file `*.pdf`
//! does not exist. Expanding it here means one command line works the same everywhere.
//!
//! Only `*` and `?` are patterns, and `require_literal_separator` keeps a `*` from crossing a
//! folder: `*.pdf` is the PDFs beside you. Brackets are escaped rather than left to the globbing
//! library, because a `[` is a legal character in a path on every platform pb runs on and a
//! character class would make `Books [2024]/*.pdf` match nothing.

use std::{
	collections::HashSet,
	path::{Component, Path, PathBuf},
};

/// What the arguments on the command line named.
#[derive(Debug, Default)]
pub struct Inputs {
	/// The files to convert, in the order the arguments named them.
	pub files: Vec<PathBuf>,
	/// Patterns that matched no file. Carried rather than refused, so that one mistyped pattern
	/// among several does not stop the run before the patterns that were right have converted.
	pub unmatched: Vec<String>,
}

/// Whether an argument is a pattern to expand rather than a path to convert.
///
/// An argument that names a file which exists is always taken as that file, even if it holds a
/// `*` or a `?`, so a document whose real name contains one of those still converts.
#[must_use]
pub fn is_pattern(arg: &Path) -> bool {
	// `to_string_lossy` rather than a component walk: a pattern is a string the reader typed, and
	// asking the filesystem whether it happens to exist is the only way to tell one apart from a
	// file that really is called that.
	arg.as_os_str().to_string_lossy().contains(['*', '?']) && !arg.exists()
}

/// Every file the arguments name, and every pattern among them that matched nothing.
///
/// An argument that is not a pattern is passed through as it is, whether or not it exists:
/// `input::check` says why a missing file is a problem in terms the reader can act on, which is
/// a better place for that than here.
#[must_use]
pub fn collect(args: &[PathBuf]) -> Inputs {
	let mut collected = Inputs::default();
	for arg in args {
		if !is_pattern(arg) {
			collected.files.push(arg.clone());
			continue;
		}
		let (files, matched) = expand(arg);
		if matched {
			collected.files.extend(files);
		} else {
			collected.unmatched.push(arg.to_string_lossy().into_owned());
		}
	}
	dedupe(&mut collected.files);
	collected
}

/// The files one pattern matches, sorted, and whether it matched anything.
///
/// Folders are dropped rather than passed on. `pb *` in a folder of books sitting beside some
/// subfolders should convert the books, and `input::check` already refuses a folder with a
/// message saying so; naming every subfolder as a failure would bury the one real problem.
fn expand(pattern: &Path) -> (Vec<PathBuf>, bool) {
	let options = glob::MatchOptions {
		// `SCAN.PDF` is the same file as `scan.pdf` on Windows and a different one on Unix, so a
		// pattern typed for the platform should match the platform. Matching case-blind on Unix
		// instead would match a file the reader can see is not the one they named.
		case_sensitive: !cfg!(windows),
		// `*` stops at a folder. `**` still reaches into folders, and is not what pb documents.
		require_literal_separator: true,
		..glob::MatchOptions::default()
	};
	// A pattern pb cannot read at all matches nothing, which the caller reports by name rather
	// than as an error it would have to word twice.
	let Ok(matches) = glob::glob_with(&escaped(pattern), options) else { return (Vec::new(), false) };
	let mut files: Vec<PathBuf> = matches.flatten().filter(|path| path.is_file()).collect();
	if files.is_empty() {
		return (files, false);
	}
	// Sorted, so that two runs over the same folder convert in the same order and report their
	// failures in the same order.
	files.sort();
	(files, true)
}

/// The pattern as `glob` should read it, with the meaning taken off every bracket.
///
/// The escape for a bracket is a character class of one: `[[]` is a `[`, and `[]]` is a `]`. A
/// backslash would read as an escape to a Unix reader and as the path separator to a Windows one,
/// which is why the class form is the one that means the same thing everywhere. Escaping every
/// bracket rather than only unmatched ones is also what stops `[2024]` from being read as a class
/// and matching a single character out of 2024.
///
/// Escaping is applied to the whole string rather than to the file name alone, because a pattern
/// can name a folder on the way: `Books [2024]/*.pdf` is a folder of PDFs in a folder with
/// brackets, and it is the same mistake to read it as a class either way.
fn escaped(pattern: &Path) -> String {
	let mut out = String::new();
	for character in pattern.to_string_lossy().chars() {
		match character {
			'[' => out.push_str("[[]"),
			']' => out.push_str("[]]"),
			_ => out.push(character),
		}
	}
	out
}

/// Removes paths already converted, keeping the first mention of each.
///
/// Two arguments can name one file -- `pb *.pdf book.epub`, or the same glob twice -- and
/// converting it twice writes the same output over itself and reports the same failure twice.
/// The reader asked for it once by the time they see one line.
fn dedupe(files: &mut Vec<PathBuf>) {
	let mut seen = HashSet::new();
	files.retain(|path| seen.insert(same_file(path)));
}

/// The path as a key for "is this the same file", without the spellings that mean one file.
///
/// Windows is case-insensitive and treats `a/b` and `a\b` as one path; Unix is not, and a reader
/// who passes both `Book.epub` and `book.epub` on Linux means two files that do not both exist.
/// Normalizing only as far as each platform agrees keeps the comparison honest.
#[must_use]
pub fn same_file(path: &Path) -> PathBuf {
	#[cfg(windows)]
	let path = PathBuf::from(path.to_string_lossy().to_lowercase());
	#[cfg(not(windows))]
	let path = path.to_path_buf();
	// Components rather than the whole string, so `a/../b` and `b` meet as the same file. A leading
	// `./` is dropped too: `components()` keeps one, and `book.txt` is the file a shell means when
	// it hands over `./book.txt`.
	path.components().filter(|component| *component != Component::CurDir).collect()
}

#[cfg(test)]
mod tests {
	use std::{env, fs, process, sync::atomic::AtomicU64};

	use super::*;

	/// A directory of its own for each test, removed with it. paperback-core has one of these but
	/// keeps it behind `cfg(test)`, where another crate cannot reach it.
	struct TempDir {
		path: PathBuf,
	}

	impl TempDir {
		fn new(label: &str) -> Self {
			static COUNTER: AtomicU64 = AtomicU64::new(0);
			let unique = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
			let path = env::temp_dir().join(format!("pb_inputs_test_{label}_{}_{unique}", process::id()));
			fs::create_dir_all(&path).expect("create the temp dir");
			Self { path }
		}

		fn join(&self, name: &str) -> PathBuf {
			self.path.join(name)
		}

		/// A pattern the reader would type to reach these files.
		fn pattern(&self, tail: &str) -> PathBuf {
			self.join(tail)
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
	#[cfg(not(windows))]
	#[test]
	fn a_file_that_really_exists_wins_over_being_read_as_a_pattern() {
		let dir = TempDir::new("exists");
		let path = dir.join("we*rd.epub");
		fs::write(&path, b"x").expect("write");
		assert!(!is_pattern(&path), "a file that exists is a path whatever its name holds");
	}

	#[test]
	fn a_pattern_expands_to_the_files_it_matches() {
		let dir = TempDir::new("expand");
		for name in ["b.epub", "a.epub", "c.txt"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		let collected = collect(&[dir.pattern("*.epub")]);
		assert_eq!(names(&collected.files), ["a.epub", "b.epub"], "not sorted, or the .txt slipped in");
		assert!(collected.unmatched.is_empty());
	}

	/// `*.pdf` with no folder in front of it is the commonest pattern there is.
	#[test]
	fn a_pattern_with_no_folder_in_front_of_it_reads_the_folder_the_reader_is_in() {
		let dir = TempDir::new("bare");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("b.txt"), b"x").expect("write");
		let saved = env::current_dir().expect("a current directory");
		env::set_current_dir(&dir.path).expect("change directory");
		let collected = collect(&[PathBuf::from("*.pdf")]);
		env::set_current_dir(saved).expect("change back");
		assert_eq!(names(&collected.files), ["a.pdf"]);
	}

	/// `?` stands for exactly one character, so it separates `a1.txt` from `a12.txt`.
	#[test]
	fn a_question_mark_stands_for_one_character() {
		let dir = TempDir::new("one-char");
		for name in ["a1.txt", "a12.txt"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		let collected = collect(&[dir.pattern("a?.txt")]);
		assert_eq!(names(&collected.files), ["a1.txt"]);
	}

	#[test]
	fn folders_matching_the_pattern_are_left_out() {
		let dir = TempDir::new("folders");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*")]);
		assert_eq!(names(&collected.files), ["a.epub"]);
	}

	/// A folder of books is as likely to be named with brackets in it as anything else, and
	/// `Books [2024]/*.pdf` matching nothing is the reader being told their own folder is empty.
	#[test]
	fn brackets_in_a_pattern_are_read_as_brackets() {
		let dir = TempDir::new("brackets");
		fs::create_dir(dir.join("Books [2024]")).expect("mkdir");
		fs::write(dir.join("Books [2024]").join("a.pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("Books [2024]/*.pdf")]);
		assert_eq!(names(&collected.files), ["a.pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	/// Brackets are as ordinary in a file's own name.
	#[test]
	fn brackets_in_a_file_name_are_read_as_brackets() {
		let dir = TempDir::new("brackets-file");
		fs::write(dir.join("notes[final].pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*s[final].pdf")]);
		assert_eq!(names(&collected.files), ["notes[final].pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	/// A bracket on its own, either side of a name.
	#[test]
	fn a_lone_bracket_is_read_as_a_bracket() {
		let dir = TempDir::new("lone-bracket");
		for name in ["[book].pdf", "book].pdf", "book[.pdf"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		for name in ["[book].pdf", "book].pdf", "book[.pdf"] {
			let collected = collect(&[dir.pattern(name)]);
			assert_eq!(names(&collected.files), [name], "{name}");
		}
	}

	/// A pattern in the middle is followed into the folders it names.
	#[test]
	fn a_pattern_names_a_folder_to_look_inside() {
		let dir = TempDir::new("nested");
		fs::create_dir_all(dir.join("one")).expect("mkdir");
		fs::create_dir_all(dir.join("two")).expect("mkdir");
		fs::write(dir.join("one").join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("two").join("b.pdf"), b"x").expect("write");
		fs::write(dir.join("c.pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*/*.pdf")]);
		assert_eq!(
			names(&collected.files),
			["a.pdf", "b.pdf"],
			"the folders were not descended, or c.pdf was not left alone"
		);
	}

	/// `*` stops at a folder, so `*.pdf` is the PDFs beside the reader and not every PDF on disk.
	#[test]
	fn a_star_does_not_reach_into_folders() {
		let dir = TempDir::new("no-descend");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("sub").join("b.pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*.pdf")]);
		assert_eq!(names(&collected.files), ["a.pdf"]);
	}

	/// `SCAN.PDF` and `scan.pdf` are one file on Windows and two elsewhere, so a pattern follows the
	/// platform. `*.pdf` reaches `SCAN.PDF` on Windows because there it is the file the reader
	/// named, and misses it on macOS and Linux because there it is a different file.
	#[test]
	fn a_pattern_matches_the_case_the_platform_considers_the_same_file() {
		let dir = TempDir::new("case");
		fs::write(dir.join("SCAN.PDF"), b"x").expect("write");
		let lower = collect(&[dir.pattern("*.pdf")]);
		let upper = collect(&[dir.pattern("*.PDF")]);
		assert_eq!(upper.files.len(), 1, "`*.PDF` reaches SCAN.PDF everywhere");
		if cfg!(windows) {
			assert_eq!(lower.files.len(), 1, "`*.pdf` reaches SCAN.PDF on Windows");
		} else {
			assert_eq!(lower.files.len(), 0, "`*.pdf` reaches nothing on {}", env::consts::OS);
			assert_eq!(lower.unmatched.len(), 1, "and says so rather than matching the wrong file");
		}
	}

	/// One mistyped pattern among several should not cost the reader the ones that were right.
	#[test]
	fn a_pattern_matching_nothing_is_carried_rather_than_fatal() {
		let dir = TempDir::new("no-match");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*.pdf"), dir.pattern("*.mobi")]);
		assert_eq!(
			names(&collected.files),
			["a.pdf"],
			"the pattern that did match was thrown away with the one that did not"
		);
		assert_eq!(collected.unmatched.len(), 1, "the pattern that matched nothing was not reported");
		assert!(collected.unmatched[0].contains("*.mobi"));
	}

	#[test]
	fn several_patterns_are_all_expanded_in_the_order_they_were_given() {
		let dir = TempDir::new("several");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		fs::write(dir.join("c.pdf"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*.pdf"), dir.pattern("*.epub")]);
		assert_eq!(names(&collected.files), ["a.pdf", "c.pdf", "b.epub"]);
	}

	/// Converting a file twice writes its output over itself and reports its failure twice.
	#[test]
	fn a_file_named_twice_is_converted_once() {
		let dir = TempDir::new("dupe");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		let collected = collect(&[dir.pattern("*.epub"), dir.join("a.epub")]);
		assert_eq!(names(&collected.files), ["a.epub", "b.epub"]);
	}

	#[test]
	fn a_missing_path_is_passed_through_rather_than_refused_here() {
		assert_eq!(collect(&[PathBuf::from("nowhere.epub")]).files, [PathBuf::from("nowhere.epub")]);
	}

	#[test]
	fn no_arguments_collect_to_no_files() {
		assert!(collect(&[]).files.is_empty());
	}

	/// A shell that expands `*.txt` hands over `./book.txt`; a reader who types the name gets
	/// `book.txt`. Those are one file, and a run comparing them must agree.
	#[test]
	fn a_leading_dot_slash_does_not_make_a_different_file() {
		assert_eq!(same_file(Path::new("./book.txt")), same_file(Path::new("book.txt")));
		assert_eq!(same_file(Path::new("a/./b.txt")), same_file(Path::new("a/b.txt")));
	}
}
