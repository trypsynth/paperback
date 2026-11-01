//! Windows shells don't expand `*.pdf` as Unix shells do, so pb expands it itself and one command line works everywhere.

use std::{
	collections::HashSet,
	path::{Component, Path, PathBuf},
};

use anyhow::{Result, bail};
use paperback_core::parser::is_remote_url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
	File(PathBuf),
	Link(String),
}

impl Source {
	#[must_use]
	pub fn shown(&self) -> String {
		match self {
			Self::File(path) => path.display().to_string(),
			Self::Link(url) => url.clone(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Given {
	Argument(PathBuf),
	/// Never expanded as a pattern.
	Listed(PathBuf),
	Unreadable(String),
}

#[derive(Debug, Default)]
pub struct Inputs {
	pub sources: Vec<Source>,
	/// Carried rather than refused, so one mistyped pattern doesn't stop the ones that were right from converting.
	pub unmatched: Vec<String>,
	pub problems: Vec<String>,
}

/// A file that exists is that file even if its real name holds `*` or `?`.
#[must_use]
pub fn is_pattern(arg: &Path) -> bool {
	arg.as_os_str().to_string_lossy().contains(['*', '?']) && !arg.exists()
}

/// Missing files pass through, since `input::check` explains that problem in terms the reader can act on.
#[must_use]
pub fn collect(given: &[Given]) -> Inputs {
	let mut collected = Inputs::default();
	for item in given {
		let (arg, listed) = match item {
			Given::Unreadable(problem) => {
				collected.problems.push(problem.clone());
				continue;
			}
			Given::Listed(path) => (path, true),
			Given::Argument(arg) => (arg, false),
		};
		if let Some(url) = arg.to_str().filter(|text| is_remote_url(text)) {
			collected.sources.push(Source::Link(url.to_string()));
			continue;
		}
		if listed && is_pattern(arg) {
			collected.problems.push(listed_pattern(arg));
			continue;
		}
		if listed || !is_pattern(arg) {
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

fn listed_pattern(path: &Path) -> String {
	format!(
		"{}: patterns are not expanded in a list given with --files-from\nIf the name had other letters in place of a ?, the program that wrote the list could not encode them; in Windows PowerShell 5.1, run $OutputEncoding = [Text.UTF8Encoding]::new() first",
		path.display()
	)
}

/// Folders are dropped, since `pb *` beside some subfolders means the books and `input::check` would refuse each folder, burying any real problem.
fn expand(pattern: &Path) -> (Vec<PathBuf>, bool) {
	let options = glob::MatchOptions {
		// `SCAN.PDF` and `scan.pdf` are one file on Windows and two on Unix, so the pattern follows the platform.
		case_sensitive: !cfg!(windows),
		// So `*.pdf` is the PDFs beside the reader, not every PDF below.
		require_literal_separator: true,
		..glob::MatchOptions::default()
	};
	// The caller reports an unreadable pattern by name, as one that matched nothing.
	let Ok(matches) = glob::glob_with(&escaped(pattern), options) else { return (Vec::new(), false) };
	let mut files: Vec<PathBuf> = matches.flatten().filter(|path| path.is_file()).collect();
	if files.is_empty() {
		return (files, false);
	}
	// So two runs over the same folder convert and report failures in the same order.
	files.sort();
	(files, true)
}

/// `[` is legal in paths, so `Books [2024]/*.pdf` must not be a character class; escaped as a class of one, since backslash is the Windows path separator.
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

/// Converting one file twice writes its output over itself and reports its failure twice.
fn dedupe(sources: &mut Vec<Source>) {
	let mut seen = HashSet::new();
	sources.retain(|source| {
		seen.insert(match source {
			Source::File(path) => Source::File(same_file(path)),
			Source::Link(url) => Source::Link(url.clone()),
		})
	});
}

/// Normalized only as far as the platform agrees: Windows ignores case and slash direction, Unix doesn't.
#[must_use]
pub fn same_file(path: &Path) -> PathBuf {
	#[cfg(windows)]
	let path = PathBuf::from(path.to_string_lossy().to_lowercase());
	#[cfg(not(windows))]
	let path = path.to_path_buf();
	// `components()` keeps a leading `./`, but a shell expanding `*.txt` hands over `./book.txt` for the reader's `book.txt`.
	path.components().filter(|component| *component != Component::CurDir).collect()
}

#[cfg(test)]
mod tests {
	use std::{env, fs, process, sync::atomic::AtomicU64};

	use super::*;

	/// paperback-core's own is behind `cfg(test)`, out of another crate's reach.
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

	#[test]
	fn brackets_in_a_pattern_are_read_as_brackets() {
		let dir = TempDir::new("brackets");
		fs::create_dir(dir.join("Books [2024]")).expect("mkdir");
		fs::write(dir.join("Books [2024]").join("a.pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("Books [2024]/*.pdf")]);
		assert_eq!(names(&collected.sources), ["a.pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

	#[test]
	fn brackets_in_a_file_name_are_read_as_brackets() {
		let dir = TempDir::new("brackets-file");
		fs::write(dir.join("notes[final].pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*s[final].pdf")]);
		assert_eq!(names(&collected.sources), ["notes[final].pdf"]);
		assert!(collected.unmatched.is_empty(), "{:?}", collected.unmatched);
	}

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

	#[test]
	fn a_star_does_not_reach_into_folders() {
		let dir = TempDir::new("no-descend");
		fs::create_dir(dir.join("sub")).expect("mkdir");
		fs::write(dir.join("a.pdf"), b"x").expect("write");
		fs::write(dir.join("sub").join("b.pdf"), b"x").expect("write");
		let collected = collect_args(&[dir.pattern("*.pdf")]);
		assert_eq!(names(&collected.sources), ["a.pdf"]);
	}

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
	#[test]
	fn a_link_in_a_list_is_a_link() {
		let collected = collect(&[Given::Listed(PathBuf::from("https://example.org/a.epub?x=1&y=2"))]);
		assert_eq!(collected.sources, [Source::Link("https://example.org/a.epub?x=1&y=2".to_string())]);
		assert!(collected.problems.is_empty(), "{:?}", collected.problems);
	}
}
