//! A guard against translation calls whose message is built by concatenating literals.
//!
//! `t("first half " + "second half")` extracts as `"first half "` and nothing else, so the pot
//! carries a msgid that stops mid-sentence while the code looks up the whole thing. The two
//! never match, the string shows in English in every language, and nothing reports it: not the
//! compiler, not the extractor, not a translator, who sees only a msgid that looks odd.
//!
//! [`check_sources`] runs before the pot is generated, and again as a test over the whole tree,
//! so a new one is caught by CI rather than by a reader noticing their language never arrived.

use std::{
	fs,
	path::{Path, PathBuf},
};

/// The 1-based line of every `t(...)` or `nt(...)` call in `source` that joins string literals
/// with `+`.
pub fn concatenated_translation_calls(source: &str) -> Vec<usize> {
	let chars: Vec<char> = source.chars().collect();
	let mut hits = Vec::new();
	let mut i = 0;
	while i < chars.len() {
		let Some(open) = call_argument_start(&chars, i) else {
			i += 1;
			continue;
		};
		if let Some(end) = joins_literals(&chars, open) {
			hits.push(line_of(&chars, i) + 1);
			i = end;
			continue;
		}
		i += 1;
	}
	hits.sort_unstable();
	hits.dedup();
	hits
}

/// The index just past the `(` when `t(` or `nt(` starts at `i`, and it is a call rather than
/// the tail of a longer identifier such as `format(` or `stateDescription(`.
fn call_argument_start(chars: &[char], i: usize) -> Option<usize> {
	let name_len = if chars.get(i) == Some(&'n') && chars.get(i + 1) == Some(&'t') {
		2
	} else if chars.get(i) == Some(&'t') {
		1
	} else {
		return None;
	};
	if chars.get(i + name_len) != Some(&'(') {
		return None;
	}
	if i > 0 {
		let before = chars[i - 1];
		if before.is_alphanumeric() || before == '_' || before == '.' {
			return None;
		}
	}
	Some(i + name_len + 1)
}

/// Scans one call's arguments from `start`, returning the index past its closing paren when a
/// string literal is followed by `+` and another string literal.
fn joins_literals(chars: &[char], start: usize) -> Option<usize> {
	let mut i = start;
	let mut depth = 1usize;
	let mut just_closed_a_literal = false;
	while i < chars.len() {
		match chars[i] {
			'"' => {
				i = skip_literal(chars, i)?;
				just_closed_a_literal = true;
				continue;
			}
			'(' => {
				depth += 1;
				just_closed_a_literal = false;
			}
			')' => {
				depth -= 1;
				if depth == 0 {
					return None;
				}
				just_closed_a_literal = false;
			}
			'+' if just_closed_a_literal => {
				// A literal, a plus, and then another literal is the shape that breaks
				// extraction. A plus joining a literal to a variable does not: that message is
				// already unextractable, and this check is not the place to say so.
				let mut j = i + 1;
				while j < chars.len() && chars[j].is_whitespace() {
					j += 1;
				}
				if chars.get(j) == Some(&'"') {
					return Some(skip_to_call_end(chars, i, depth));
				}
				just_closed_a_literal = false;
			}
			c if c.is_whitespace() => {}
			_ => just_closed_a_literal = false,
		}
		i += 1;
	}
	None
}

/// Walks from `i` to just past the call's closing paren, so the caller resumes after it.
const fn skip_to_call_end(chars: &[char], i: usize, mut depth: usize) -> usize {
	let mut j = i;
	while j < chars.len() && depth > 0 {
		match chars[j] {
			'"' => {
				let Some(next) = skip_literal(chars, j) else {
					return chars.len();
				};
				j = next;
				continue;
			}
			'(' => depth += 1,
			')' => depth -= 1,
			_ => {}
		}
		j += 1;
	}
	j
}

/// The index just past the closing quote of the literal opening at `i`, honouring escapes.
const fn skip_literal(chars: &[char], i: usize) -> Option<usize> {
	let mut j = i + 1;
	while j < chars.len() {
		match chars[j] {
			'\\' => j += 2,
			'"' => return Some(j + 1),
			_ => j += 1,
		}
	}
	None
}

fn line_of(chars: &[char], index: usize) -> usize {
	chars[..index].iter().filter(|&&c| c == '\n').count()
}

/// The translatable sources under `dir`, skipping build output and this file's own examples.
fn source_files(dir: &Path, out: &mut Vec<PathBuf>) {
	let Ok(entries) = fs::read_dir(dir) else {
		return;
	};
	for entry in entries.flatten() {
		let path = entry.path();
		let name = entry.file_name();
		if path.is_dir() {
			if name == "target" || name == "build" || name == "generated" {
				continue;
			}
			source_files(&path, out);
		} else if path
			.extension()
			.and_then(|e| e.to_str())
			.is_some_and(|e| ["rs", "kt", "swift"].iter().any(|kind| e.eq_ignore_ascii_case(kind)))
		{
			// This file spells the pattern out on purpose, in its examples and its tests.
			if name != "pot_lint.rs" {
				out.push(path);
			}
		}
	}
}

/// Fails when any translatable source under `root` joins literals inside a translation call.
///
/// # Errors
///
/// Returns the offending `file:line` list, or an error when there are no sources to check,
/// which would mean this check is silently passing over a moved directory.
pub fn check_sources(root: &Path) -> Result<(), String> {
	let mut files = Vec::new();
	for dir in ["crates", "android/app/src/main/kotlin", "ios/Paperback"] {
		source_files(&root.join(dir), &mut files);
	}
	if files.is_empty() {
		return Err(format!("found no translatable sources under {}", root.display()));
	}
	let mut offenders = Vec::new();
	for file in files {
		let Ok(content) = fs::read_to_string(&file) else {
			continue;
		};
		for line in concatenated_translation_calls(&content) {
			offenders.push(format!("{}:{line}", file.display()));
		}
	}
	if offenders.is_empty() {
		return Ok(());
	}
	Err(format!(
		"these translation calls join string literals, so only the first fragment reaches the pot and the string \
		 can never be translated. Write each message as one literal:\n  {}",
		offenders.join("\n  ")
	))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_plain_call_is_fine() {
		assert!(concatenated_translation_calls(r#"t("Open Document")"#).is_empty());
	}

	#[test]
	fn a_call_joining_two_literals_is_reported() {
		assert_eq!(concatenated_translation_calls(r#"t("first " + "second")"#), vec![1]);
	}

	#[test]
	fn the_join_is_found_across_lines() {
		let src = "Text(\n\ttext = t(\n\t\t\"first \" +\n\t\t\t\"second\"\n\t),\n)";
		assert_eq!(concatenated_translation_calls(src), vec![2]);
	}

	#[test]
	fn plural_calls_are_checked_too() {
		assert_eq!(concatenated_translation_calls(r#"nt("a " + "b", "c", n)"#), vec![1]);
	}

	/// Joining strings outside a translation call is ordinary code.
	#[test]
	fn concatenation_outside_the_call_is_ignored() {
		assert!(concatenated_translation_calls(r#"val s = t("Open") + " " + name"#).is_empty());
	}

	/// `format(`, `stateDescription(` and friends end in the letter t but are not `t(`.
	#[test]
	fn a_longer_identifier_ending_in_t_is_not_a_translation_call() {
		assert!(concatenated_translation_calls(r#"format("a " + "b")"#).is_empty());
		assert!(concatenated_translation_calls(r#"obj.t("a " + "b")"#).is_empty());
	}

	/// A plus inside the text is text, not concatenation.
	#[test]
	fn a_plus_inside_a_literal_is_not_a_join() {
		assert!(concatenated_translation_calls("t(\"one \\\" + \\\" two\")").is_empty());
	}

	/// Every translatable source in the project, so a concatenated call cannot be merged.
	#[test]
	fn no_source_file_concatenates_a_translation_call() {
		if let Err(report) = check_sources(&crate::workspace::project_root()) {
			panic!("{report}");
		}
	}
}
