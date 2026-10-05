//! Turning the paths on the command line into the files to convert.
//!
//! A Unix shell expands `*.pdf` into the list of matching files before pb ever sees it. The
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
	io::BufRead,
	path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, bail};

/// One input as the reader gave it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Given {
	/// An argument on the command line, which may be a pattern.
	Argument(PathBuf),
	/// A line of the list on stdin, taken as written.
	Listed(PathBuf),
}

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
	arg.as_os_str().to_string_lossy().contains(['*', '?']) && !arg.exists()
}

/// Every file the inputs name, and every pattern among the arguments that matched nothing.
///
/// A listed input, and an argument that is not a pattern, is passed through as it is, whether or
/// not it exists: `input::check` says why a missing file is a problem in terms the reader can act
/// on, which is a better place for that than here.
#[must_use]
pub fn collect(given: &[Given]) -> Inputs {
	let mut collected = Inputs::default();
	for item in given {
		let arg = match item {
			Given::Listed(path) => {
				collected.files.push(path.clone());
				continue;
			}
			Given::Argument(arg) => arg,
		};
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

/// The arguments, with a `-` replaced by the inputs listed on stdin, one per line.
///
/// A listed line is taken as written, never as a pattern. Blank lines are skipped, and so are
/// lines starting with `#` unless a file of that name exists. Surrounding whitespace, Windows line
/// ends and a byte order mark are taken off. stdin is read only when a `-` is among the arguments.
///
/// # Errors
///
/// Returns an error if `-` is given more than once, if a line of the list is not UTF-8, or if `-`
/// is the only argument and the list names nothing.
pub fn with_stdin_list(args: &[PathBuf], stdin: impl BufRead) -> Result<Vec<Given>> {
	let dashes = args.iter().filter(|arg| is_stdin(arg)).count();
	if dashes == 0 {
		return Ok(args.iter().cloned().map(Given::Argument).collect());
	}
	if dashes > 1 {
		bail!("- was given {dashes} times, but the list on stdin can be read only once");
	}
	let listed = read_list(stdin, |entry| Path::new(entry).exists())?;
	if listed.is_empty() && args.len() == 1 {
		bail!("the list on stdin names no documents");
	}
	let mut given = Vec::with_capacity(args.len() - 1 + listed.len());
	for arg in args {
		if is_stdin(arg) {
			given.extend(listed.iter().map(|entry| Given::Listed(PathBuf::from(entry))));
		} else {
			given.push(Given::Argument(arg.clone()));
		}
	}
	Ok(given)
}

fn is_stdin(arg: &Path) -> bool {
	arg.as_os_str() == "-"
}

/// The entries of the list; a `#` line is a comment unless `exists` says a file of that name does.
fn read_list(stdin: impl BufRead, exists: impl Fn(&str) -> bool) -> Result<Vec<String>> {
	let mut listed = Vec::new();
	for (index, line) in stdin.lines().enumerate() {
		let line = line.with_context(|| format!("line {} of the list on stdin is not UTF-8 text", index + 1))?;
		let entry = line.trim_start_matches('\u{feff}').trim();
		if entry.is_empty() || (entry.starts_with('#') && !exists(entry)) {
			continue;
		}
		listed.push(entry.to_string());
	}
	Ok(listed)
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

/// The pattern as `glob` should read it, with every bracket escaped as a class of one (`[[]`, `[]]`), since a backslash is the path separator on Windows.
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
	use std::{env, fs, io, process, sync::atomic::AtomicU64};

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
		let collected = collect_args(&[dir.pattern("*.epub")]);
		assert_eq!(names(&collected.files), ["a.epub", "b.epub"], "not sorted, or the .txt slipped in");
		assert!(collected.unmatched.is_empty());
	}

	/// `?` stands for exactly one character, so it separates `a1.txt` from `a12.txt`.
	#[test]
	fn a_question_mark_stands_for_one_character() {
		let dir = TempDir::new("one-char");
		for name in ["a1.txt", "a12.txt"] {
			fs::write(dir.join(name), b"x").expect("write");
		}
		let collected = collect_args(&[dir.pattern("a?.txt")]);
		assert_eq!(names(&collected.files), ["a1.txt"]);
	}

	#[test]
	fn folders_matching_the_pattern_are_left_out() {
		let dir = TempDir::new("folders");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*")]);
		assert_eq!(names(&collected.files), ["a.epub"]);
	}

	/// A folder of books is as likely to be named with brackets in it as anything else, and
	/// `Books [2024]/*.pdf` matching nothing is the reader being told their own folder is empty.
	#[test]
	fn brackets_in_a_pattern_are_read_as_brackets() {
		let dir = TempDir::new("brackets");
		fs::create_dir(dir.join("Books [2024]")).expect("mkdir");
		fs::write(dir.join("Books [2024]").join("a.pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("Books [2024]/*.pdf")]);
		assert_eq!(names(&collected.files), ["a.pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	/// Brackets are as ordinary in a file's own name.
	#[test]
	fn brackets_in_a_file_name_are_read_as_brackets() {
		let dir = TempDir::new("brackets-file");
		fs::write(dir.join("notes[final].pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*s[final].pdf")]);
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
			let collected = collect_args(&[dir.pattern(name)]);
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
		let collected = collect_args(&[dir.pattern("*/*.pdf")]);
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
		let collected = collect_args(&[dir.pattern("*.pdf")]);
		assert_eq!(names(&collected.files), ["a.pdf"]);
	}

	/// `SCAN.PDF` and `scan.pdf` are one file on Windows and two elsewhere, so a pattern follows the
	/// platform. `*.pdf` reaches `SCAN.PDF` on Windows because there it is the file the reader
	/// named, and misses it on macOS and Linux because there it is a different file.
	#[test]
	fn a_pattern_matches_the_case_the_platform_considers_the_same_file() {
		let dir = TempDir::new("case");
		fs::write(dir.join("SCAN.PDF"), b"x").expect("write");
		let lower = collect_args(&[dir.pattern("*.pdf")]);
		let upper = collect_args(&[dir.pattern("*.PDF")]);
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
		let collected = collect_args(&[dir.pattern("*.pdf"), dir.pattern("*.mobi")]);
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
		let collected = collect_args(&[dir.pattern("*.pdf"), dir.pattern("*.epub")]);
		assert_eq!(names(&collected.files), ["a.pdf", "c.pdf", "b.epub"]);
	}

	/// Converting a file twice writes its output over itself and reports its failure twice.
	#[test]
	fn a_file_named_twice_is_converted_once() {
		let dir = TempDir::new("dupe");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*.epub"), dir.join("a.epub")]);
		assert_eq!(names(&collected.files), ["a.epub", "b.epub"]);
	}

	#[test]
	fn a_missing_path_is_passed_through_rather_than_refused_here() {
		assert_eq!(collect_args(&[PathBuf::from("nowhere.epub")]).files, [PathBuf::from("nowhere.epub")]);
	}

	#[test]
	fn no_arguments_collect_to_no_files() {
		assert!(collect_args(&[]).files.is_empty());
	}

	/// A shell that expands `*.txt` hands over `./book.txt`; a reader who types the name gets
	/// `book.txt`. Those are one file, and a run comparing them must agree.
	#[test]
	fn a_leading_dot_slash_does_not_make_a_different_file() {
		assert_eq!(same_file(Path::new("./book.txt")), same_file(Path::new("book.txt")));
		assert_eq!(same_file(Path::new("a/./b.txt")), same_file(Path::new("a/b.txt")));
	}

	fn collect_args(args: &[PathBuf]) -> Inputs {
		let given: Vec<Given> = args.iter().cloned().map(Given::Argument).collect();
		collect(&given)
	}

	fn listed(args: &[&str], stdin: &str) -> anyhow::Result<Vec<Given>> {
		let args: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
		with_stdin_list(&args, io::Cursor::new(stdin.as_bytes()))
	}

	fn on_stdin(names: &[&str]) -> Vec<Given> {
		names.iter().map(|name| Given::Listed(PathBuf::from(name))).collect()
	}

	#[test]
	fn a_dash_is_replaced_by_the_inputs_listed_on_stdin() {
		let inputs = listed(&["a.epub", "-", "d.pdf"], "b.epub\nc.docx\n").expect("a list");
		assert_eq!(
			inputs,
			[
				Given::Argument("a.epub".into()),
				Given::Listed("b.epub".into()),
				Given::Listed("c.docx".into()),
				Given::Argument("d.pdf".into())
			]
		);
	}

	#[test]
	fn blank_lines_and_comments_in_the_list_are_skipped() {
		let inputs = listed(&["-"], "# books\n\nb.epub\n   \n  # more\nc.pdf").expect("a list");
		assert_eq!(inputs, on_stdin(&["b.epub", "c.pdf"]));
	}

	#[test]
	fn a_hash_line_naming_a_file_that_exists_is_that_file() {
		let listed =
			read_list(io::Cursor::new("#1 Hit.epub\n# a comment\n"), |entry| entry == "#1 Hit.epub").expect("a list");
		assert_eq!(listed, ["#1 Hit.epub"]);
	}

	#[test]
	fn a_listed_line_is_taken_as_written_and_not_as_a_pattern() {
		let dir = TempDir::new("listed-literal");
		fs::write(dir.join("ab.epub"), b"x").expect("write");
		let mangled = dir.pattern("??.epub");
		let given = listed(&["-"], &format!("{}\n", mangled.display())).expect("a list");
		let collected = collect(&given);
		assert_eq!(collected.files, [mangled]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	#[test]
	fn a_pattern_typed_beside_a_list_still_expands() {
		let dir = TempDir::new("typed-pattern");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let given = listed(&[dir.pattern("*.epub").to_str().expect("a UTF-8 path"), "-"], "b.pdf\n").expect("a list");
		assert_eq!(names(&collect(&given).files), ["a.epub", "b.pdf"]);
	}

	#[test]
	fn windows_line_ends_and_a_byte_order_mark_are_taken_off() {
		let inputs = listed(&["-"], "\u{feff}b.epub\r\nc.pdf\r\n").expect("a list");
		assert_eq!(inputs, on_stdin(&["b.epub", "c.pdf"]));
	}

	#[test]
	fn surrounding_spaces_are_taken_off_and_inner_ones_kept() {
		let inputs = listed(&["-"], "  my book.epub  \n").expect("a list");
		assert_eq!(inputs, on_stdin(&["my book.epub"]));
	}

	#[test]
	fn a_second_dash_is_refused() {
		let error = listed(&["-", "-"], "b.epub\n").expect_err("two dashes").to_string();
		assert!(error.contains("once"), "{error}");
	}

	#[test]
	fn without_a_dash_stdin_is_left_unread() {
		struct Untouched;
		impl io::Read for Untouched {
			fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
				panic!("stdin was read without a - in the arguments")
			}
		}
		let args = [PathBuf::from("a.epub")];
		assert_eq!(
			with_stdin_list(&args, io::BufReader::new(Untouched)).expect("no list"),
			[Given::Argument("a.epub".into())]
		);
	}

	#[test]
	fn a_dash_alone_with_an_empty_list_is_refused() {
		let error = listed(&["-"], "# nothing yet\n\n").expect_err("no inputs at all").to_string();
		assert!(error.contains("stdin"), "{error}");
	}

	#[test]
	fn a_list_that_is_not_utf8_names_the_line() {
		let args = [PathBuf::from("-")];
		let error = with_stdin_list(&args, io::Cursor::new(&b"b.epub\n\xff\xfe.pdf\n"[..])).expect_err("not UTF-8");
		assert!(error.to_string().contains("line 2"), "{error}");
	}
}
