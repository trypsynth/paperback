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
	path::{Component, Path, PathBuf},
};

use anyhow::{Result, bail};
use paperback_core::parser::is_remote_url;

/// One document to convert: a file on disk, or a link to download first.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
	File(PathBuf),
	Link(String),
}

impl Source {
	/// The source as the reader named it.
	#[must_use]
	pub fn shown(&self) -> String {
		match self {
			Self::File(path) => path.display().to_string(),
			Self::Link(url) => url.clone(),
		}
	}
}

/// One input as the reader gave it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Given {
	/// An argument on the command line, which may be a pattern.
	Argument(PathBuf),
	/// A line of the list read with `--files-from`, which is never a pattern.
	Listed(PathBuf),
	/// A line of that list that could not be read, described for the reader.
	Unreadable(String),
}

/// What the arguments on the command line named.
#[derive(Debug, Default)]
pub struct Inputs {
	/// The documents to convert, in the order the arguments named them.
	pub sources: Vec<Source>,
	/// Patterns that matched no file. Carried rather than refused, so that one mistyped pattern
	/// among several does not stop the run before the patterns that were right have converted.
	pub unmatched: Vec<String>,
	/// Listed inputs that cannot be converted, described for the reader.
	pub problems: Vec<String>,
}

/// Whether an argument is a pattern to expand rather than a path to convert.
///
/// An argument that names a file which exists is always taken as that file, even if it holds a
/// `*` or a `?`, so a document whose real name contains one of those still converts.
#[must_use]
pub fn is_pattern(arg: &Path) -> bool {
	arg.as_os_str().to_string_lossy().contains(['*', '?']) && !arg.exists()
}

/// Every file the inputs name, every pattern among the arguments that matched nothing, and every
/// listed input that cannot be converted.
///
/// An argument that is not a pattern, and a listed input without `*` or `?`, is passed through as
/// it is, whether or not it exists: `input::check` says why a missing file is a problem in terms
/// the reader can act on, which is a better place for that than here.
#[must_use]
pub fn collect(given: &[Given]) -> Inputs {
	let mut collected = Inputs::default();
	for item in given {
		let arg = match item {
			Given::Listed(path) if is_pattern(path) => {
				collected.problems.push(listed_pattern(path));
				continue;
			}
			Given::Listed(path) => {
				collected.sources.push(Source::File(path.clone()));
				continue;
			}
			Given::Unreadable(problem) => {
				collected.problems.push(problem.clone());
				continue;
			}
			Given::Argument(arg) => arg,
		};
		if let Some(url) = arg.to_str().filter(|text| is_remote_url(text)) {
			collected.sources.push(Source::Link(url.to_string()));
			continue;
		}
		if !is_pattern(arg) {
			collected.sources.push(Source::File(arg.clone()));
			continue;
		}
		let (files, matched) = expand(arg);
		if matched {
			collected.sources.extend(files.into_iter().map(Source::File));
		} else {
			collected.unmatched.push(arg.to_string_lossy().into_owned());
		}
	}
	dedupe(&mut collected.sources);
	collected
}

/// The arguments, followed by the entries of the list read with `--files-from`.
///
/// # Errors
///
/// Returns an error if there is a list, it names nothing, and no arguments were given either.
pub fn gather(args: &[PathBuf], listed: Option<Vec<Given>>) -> Result<Vec<Given>> {
	let mut given: Vec<Given> = args.iter().cloned().map(Given::Argument).collect();
	if let Some(listed) = listed {
		if listed.is_empty() && given.is_empty() {
			bail!("the list given with --files-from names no documents");
		}
		given.extend(listed);
	}
	Ok(given)
}

/// The entries of a list, one path per line.
///
/// Surrounding spaces, Windows line ends and byte order marks are taken off. Blank lines are
/// skipped, and so are lines starting with `#` unless `exists` says a file of that name does. A
/// list starting with a UTF-16 byte order mark is read as UTF-16. A line that is not UTF-8 is
/// decoded with `fallback`, and becomes [`Given::Unreadable`] when that gives nothing.
pub fn read_list(
	bytes: &[u8],
	exists: impl Fn(&str) -> bool,
	fallback: impl Fn(&[u8]) -> Option<String>,
) -> Vec<Given> {
	let lines: Vec<Option<String>> = utf16(bytes).map_or_else(
		|| bytes.split(|&byte| byte == b'\n').map(|line| decode_line(line, &fallback)).collect(),
		|text| text.lines().map(|line| Some(line.to_string())).collect(),
	);
	let mut listed = Vec::new();
	for (index, line) in lines.into_iter().enumerate() {
		let Some(line) = line else {
			listed.push(Given::Unreadable(format!("line {} of the list is not UTF-8 text", index + 1)));
			continue;
		};
		let entry = line.trim_start_matches('\u{feff}').trim();
		if entry.is_empty() || (entry.starts_with('#') && !exists(entry)) {
			continue;
		}
		listed.push(Given::Listed(PathBuf::from(entry)));
	}
	listed
}

fn decode_line(line: &[u8], fallback: &impl Fn(&[u8]) -> Option<String>) -> Option<String> {
	let line = line.strip_suffix(b"\r").unwrap_or(line);
	std::str::from_utf8(line).map(str::to_string).ok().or_else(|| fallback(line))
}

/// The text of a list that starts with a UTF-16 byte order mark, little- or big-endian.
fn utf16(bytes: &[u8]) -> Option<String> {
	let (little_endian, body) = match bytes {
		[0xFF, 0xFE, rest @ ..] => (true, rest),
		[0xFE, 0xFF, rest @ ..] => (false, rest),
		_ => return None,
	};
	let (pairs, _) = body.as_chunks::<2>();
	let units =
		pairs.iter().map(|&pair| if little_endian { u16::from_le_bytes(pair) } else { u16::from_be_bytes(pair) });
	Some(char::decode_utf16(units).map(|decoded| decoded.unwrap_or(char::REPLACEMENT_CHARACTER)).collect())
}

/// What a listed name holding `*` or `?` is reported as, since a list is not expanded.
fn listed_pattern(path: &Path) -> String {
	format!(
		"{}: patterns are not expanded in a list given with --files-from\nIf the name had other letters in place of a ?, the program that wrote the list could not encode them; in Windows PowerShell 5.1, run $OutputEncoding = [Text.UTF8Encoding]::new() first",
		path.display()
	)
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
fn dedupe(sources: &mut Vec<Source>) {
	let mut seen = HashSet::new();
	sources.retain(|source| {
		seen.insert(match source {
			Source::File(path) => Source::File(same_file(path)),
			Source::Link(url) => Source::Link(url.clone()),
		})
	});
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

	fn names(sources: &[Source]) -> Vec<String> {
		sources
			.iter()
			.map(|source| match source {
				Source::File(path) => path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
				Source::Link(url) => url.clone(),
			})
			.collect()
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
		assert_eq!(names(&collected.sources), ["a.epub", "b.epub"], "not sorted, or the .txt slipped in");
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
		assert_eq!(names(&collected.sources), ["a1.txt"]);
	}

	#[test]
	fn folders_matching_the_pattern_are_left_out() {
		let dir = TempDir::new("folders");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*")]);
		assert_eq!(names(&collected.sources), ["a.epub"]);
	}

	/// A folder of books is as likely to be named with brackets in it as anything else, and
	/// `Books [2024]/*.pdf` matching nothing is the reader being told their own folder is empty.
	#[test]
	fn brackets_in_a_pattern_are_read_as_brackets() {
		let dir = TempDir::new("brackets");
		fs::create_dir(dir.join("Books [2024]")).expect("mkdir");
		fs::write(dir.join("Books [2024]").join("a.pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("Books [2024]/*.pdf")]);
		assert_eq!(names(&collected.sources), ["a.pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	/// Brackets are as ordinary in a file's own name.
	#[test]
	fn brackets_in_a_file_name_are_read_as_brackets() {
		let dir = TempDir::new("brackets-file");
		fs::write(dir.join("notes[final].pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*s[final].pdf")]);
		assert_eq!(names(&collected.sources), ["notes[final].pdf"]);
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
			assert_eq!(names(&collected.sources), [name], "{name}");
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
			names(&collected.sources),
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
		assert_eq!(names(&collected.sources), ["a.pdf"]);
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
		assert_eq!(upper.sources.len(), 1, "`*.PDF` reaches SCAN.PDF everywhere");
		if cfg!(windows) {
			assert_eq!(lower.sources.len(), 1, "`*.pdf` reaches SCAN.PDF on Windows");
		} else {
			assert_eq!(lower.sources.len(), 0, "`*.pdf` reaches nothing on {}", env::consts::OS);
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
			names(&collected.sources),
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
		assert_eq!(names(&collected.sources), ["a.pdf", "c.pdf", "b.epub"]);
	}

	/// Converting a file twice writes its output over itself and reports its failure twice.
	#[test]
	fn a_file_named_twice_is_converted_once() {
		let dir = TempDir::new("dupe");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		fs::write(dir.join("b.epub"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*.epub"), dir.join("a.epub")]);
		assert_eq!(names(&collected.sources), ["a.epub", "b.epub"]);
	}

	#[test]
	fn a_missing_path_is_passed_through_rather_than_refused_here() {
		assert_eq!(
			collect_args(&[PathBuf::from("nowhere.epub")]).sources,
			[Source::File(PathBuf::from("nowhere.epub"))]
		);
	}

	#[test]
	fn no_arguments_collect_to_no_files() {
		assert!(collect_args(&[]).sources.is_empty());
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

	fn no_fallback(_: &[u8]) -> Option<String> {
		None
	}

	fn read(bytes: &[u8]) -> Vec<Given> {
		read_list(bytes, |_| false, no_fallback)
	}

	fn listed(names: &[&str]) -> Vec<Given> {
		names.iter().map(|name| Given::Listed(PathBuf::from(name))).collect()
	}

	#[test]
	fn the_listed_inputs_follow_the_typed_ones() {
		let given = gather(&[PathBuf::from("a.epub")], Some(listed(&["b.epub"]))).expect("inputs");
		assert_eq!(given, [Given::Argument("a.epub".into()), Given::Listed("b.epub".into())]);
	}

	#[test]
	fn without_a_list_the_arguments_are_the_inputs() {
		assert_eq!(gather(&[PathBuf::from("a.epub")], None).expect("inputs"), [Given::Argument("a.epub".into())]);
	}

	#[test]
	fn an_empty_list_with_nothing_else_is_refused() {
		let error = gather(&[], Some(Vec::new())).expect_err("no inputs at all").to_string();
		assert!(error.contains("--files-from"), "{error}");
	}

	#[test]
	fn an_empty_list_beside_typed_inputs_is_accepted() {
		assert_eq!(gather(&[PathBuf::from("a.epub")], Some(Vec::new())).expect("inputs").len(), 1);
	}

	#[test]
	fn blank_lines_and_comments_in_the_list_are_skipped() {
		assert_eq!(read(b"# books\n\nb.epub\n   \n  # more\nc.pdf"), listed(&["b.epub", "c.pdf"]));
	}

	#[test]
	fn a_hash_line_naming_a_file_that_exists_is_that_file() {
		let given = read_list(b"#1 Hit.epub\n# a comment\n", |entry| entry == "#1 Hit.epub", no_fallback);
		assert_eq!(given, listed(&["#1 Hit.epub"]));
	}

	#[test]
	fn windows_line_ends_and_a_byte_order_mark_are_taken_off() {
		assert_eq!(read("\u{feff}b.epub\r\nc.pdf\r\n".as_bytes()), listed(&["b.epub", "c.pdf"]));
	}

	#[test]
	fn surrounding_spaces_are_taken_off_and_inner_ones_kept() {
		assert_eq!(read(b"  my book.epub  \n"), listed(&["my book.epub"]));
	}

	#[test]
	fn a_utf16_list_is_read() {
		let mut bytes = vec![0xFF, 0xFE];
		bytes.extend("caf\u{e9}.epub\r\nb.pdf\r\n".encode_utf16().flat_map(u16::to_le_bytes));
		assert_eq!(read(&bytes), listed(&["caf\u{e9}.epub", "b.pdf"]));
	}

	#[test]
	fn a_line_that_is_not_utf8_is_decoded_with_the_fallback() {
		let latin1 = |bytes: &[u8]| Some(bytes.iter().map(|&byte| char::from(byte)).collect());
		assert_eq!(read_list(b"caf\xe9.txt\n", |_| false, latin1), listed(&["caf\u{e9}.txt"]));
	}

	#[test]
	fn a_line_nothing_can_decode_is_one_failed_input_and_the_rest_are_kept() {
		let given = read(b"b.epub\n\xff\xfe.pdf\nc.pdf\n");
		assert_eq!(given.len(), 3, "{given:?}");
		assert_eq!(given[0], Given::Listed("b.epub".into()));
		assert!(matches!(&given[1], Given::Unreadable(problem) if problem.contains("line 2")), "{:?}", given[1]);
		assert_eq!(given[2], Given::Listed("c.pdf".into()));
	}

	#[test]
	fn an_unreadable_line_is_reported_rather_than_converted() {
		let collected = collect(&[Given::Unreadable("line 2 of the list is not UTF-8 text".to_string())]);
		assert!(collected.sources.is_empty());
		assert_eq!(collected.problems, ["line 2 of the list is not UTF-8 text"]);
	}

	#[test]
	fn a_listed_name_with_a_star_or_a_question_mark_is_reported_rather_than_expanded() {
		let dir = TempDir::new("listed-literal");
		fs::write(dir.join("ab.epub"), b"x").expect("write");
		let collected = collect(&[Given::Listed(dir.pattern("??.epub"))]);
		assert!(collected.sources.is_empty(), "{:?}", collected.sources);
		assert_eq!(collected.problems.len(), 1, "{:?}", collected.problems);
		assert!(collected.problems[0].contains("not expanded"), "{}", collected.problems[0]);
		assert!(collected.problems[0].contains("$OutputEncoding"), "{}", collected.problems[0]);
	}

	#[test]
	fn a_pattern_typed_beside_a_list_still_expands() {
		let dir = TempDir::new("typed-pattern");
		fs::write(dir.join("a.epub"), b"x").expect("write");
		let given = gather(&[dir.pattern("*.epub")], Some(listed(&["b.pdf"]))).expect("inputs");
		assert_eq!(names(&collect(&given).sources), ["a.epub", "b.pdf"]);
	}
	#[test]
	fn a_link_with_a_question_mark_is_a_link_and_not_a_pattern() {
		let collected = collect_args(&[PathBuf::from("https://example.org/download?id=3&x=*")]);
		assert_eq!(collected.sources, [Source::Link("https://example.org/download?id=3&x=*".to_string())]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	#[test]
	fn links_and_files_keep_the_order_they_were_given() {
		let collected = collect_args(&[
			PathBuf::from("a.epub"),
			PathBuf::from("https://example.org/b.pdf"),
			PathBuf::from("c.docx"),
		]);
		assert_eq!(
			collected.sources,
			[
				Source::File("a.epub".into()),
				Source::Link("https://example.org/b.pdf".to_string()),
				Source::File("c.docx".into())
			]
		);
	}

	#[test]
	fn a_link_named_twice_is_converted_once() {
		let link = PathBuf::from("https://example.org/b.pdf");
		assert_eq!(collect_args(&[link.clone(), link]).sources.len(), 1);
	}

	#[test]
	fn links_differing_only_in_path_case_are_two_links() {
		let collected = collect_args(&[
			PathBuf::from("https://example.org/Book.pdf"),
			PathBuf::from("https://example.org/book.pdf"),
		]);
		assert_eq!(collected.sources.len(), 2);
	}

	#[test]
	fn a_link_is_shown_as_typed() {
		assert_eq!(
			Source::Link("https://example.org/a.epub?x=1".to_string()).shown(),
			"https://example.org/a.epub?x=1"
		);
	}
}
