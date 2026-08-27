use std::{
	collections::{HashSet, hash_map::DefaultHasher},
	env,
	error::Error,
	fs,
	hash::{Hash, Hasher},
	path::Path,
	process::{self, Command},
};

use super::deepl::{DeepLClient, resolve_target_lang};

const MARKER_PREFIX: &str = "<!-- machine-translated from doc/readme.md (source-hash: ";
const MARKER_SUFFIX: &str = "); please review and edit as needed -->";

/// Machine-translates `doc/readme.md` into `doc/readme-<lang>.md` for every language in
/// `langs` that `DeepL` supports. Every language goes through the same pipeline —
/// including ones that already have a translated file, machine-generated or not, since
/// there's no reliable way to tell how stale a hand-written one has become — gated only
/// on whether the English source has changed since the last time that language's file
/// was (re)written (see [`needs_translation`]).
pub fn sync_readmes(
	root: &Path,
	langs: &[String],
	client_and_supported: Option<&(DeepLClient, HashSet<String>)>,
	dry_run: bool,
) -> Result<(), Box<dyn Error>> {
	let doc_dir = root.join("doc");
	let source_path = doc_dir.join("readme.md");
	let Ok(source_md) = fs::read_to_string(&source_path) else {
		return Ok(());
	};
	let hash = source_hash(&source_md);
	for lang in langs {
		let target_path = doc_dir.join(format!("readme-{lang}.md"));
		let existing = fs::read_to_string(&target_path).ok();
		if !needs_translation(existing.as_deref(), &hash) {
			continue;
		}
		if dry_run {
			println!("readme-{lang}.md: would be translated");
			continue;
		}
		let Some((client, supported)) = client_and_supported else {
			unreachable!("client_and_supported is always Some outside --dry-run")
		};
		let Some(target_lang) = resolve_target_lang(lang, supported) else {
			continue;
		};
		let html = md_to_html_fragment(&source_path)?;
		let translated_html = client.translate_html(&html, &target_lang)?;
		let translated_md = html_to_md(&translated_html)?;
		fs::write(&target_path, format!("{}\n\n{}\n", marker_line(&hash), translated_md.trim_end()))?;
		println!("readme-{lang}.md ({target_lang}): translated");
	}
	Ok(())
}

/// Whether `doc/readme-<lang>.md` needs (re)translating: yes if it doesn't exist yet, or
/// exists without a marker comment (pre-dates this pipeline), or its marker's source hash
/// doesn't match the current English source. No only when a marker is present and its
/// hash matches — i.e. this exact English source was already translated into this file.
fn needs_translation(existing_content: Option<&str>, current_source_hash: &str) -> bool {
	existing_content.and_then(existing_marker_hash).is_none_or(|existing_hash| existing_hash != current_source_hash)
}

fn source_hash(source_md: &str) -> String {
	let mut hasher = DefaultHasher::new();
	source_md.hash(&mut hasher);
	format!("{:016x}", hasher.finish())
}

fn marker_line(hash: &str) -> String {
	format!("{MARKER_PREFIX}{hash}{MARKER_SUFFIX}")
}

fn existing_marker_hash(content: &str) -> Option<&str> {
	content.lines().next()?.trim().strip_prefix(MARKER_PREFIX)?.strip_suffix(MARKER_SUFFIX)
}

/// Converts `path` (Markdown) to an HTML fragment via `pandoc` (already a hard
/// requirement of this toolchain — see `crates/paperback/build.rs`,
/// `crates/xtask/src/{android,ios}.rs`), not a standalone document: no `<html>`/`<head>`
/// wrapper, since this is headed straight into a `DeepL` request, not rendered directly.
fn md_to_html_fragment(path: &Path) -> Result<String, Box<dyn Error>> {
	let tmp_html = env::temp_dir().join(format!("paperback-readme-{}.html", process::id()));
	let status = Command::new("pandoc")
		.arg(path)
		.arg("-f")
		.arg("markdown")
		.arg("-t")
		.arg("html")
		.arg("-o")
		.arg(&tmp_html)
		.status()?;
	if !status.success() {
		return Err("pandoc markdown->html conversion failed".into());
	}
	let html = fs::read_to_string(&tmp_html)?;
	let _ = fs::remove_file(&tmp_html);
	Ok(html)
}

/// Converts an HTML fragment back to Markdown via `pandoc`. The round trip isn't
/// byte-for-byte cosmetically identical to hand-written Markdown (e.g. indented code
/// blocks instead of fenced, `-` bullets instead of `*`) but is valid Markdown that
/// renders the same, and it's still `pandoc` doing the final `readme.html` render either
/// way, so that cosmetic difference never reaches an actual reader.
fn html_to_md(html: &str) -> Result<String, Box<dyn Error>> {
	let pid = process::id();
	let tmp_html = env::temp_dir().join(format!("paperback-readme-in-{pid}.html"));
	let tmp_md = env::temp_dir().join(format!("paperback-readme-out-{pid}.md"));
	fs::write(&tmp_html, html)?;
	let status = Command::new("pandoc")
		.arg(&tmp_html)
		.arg("-f")
		.arg("html")
		.arg("-t")
		.arg("markdown")
		.arg("-o")
		.arg(&tmp_md)
		.status();
	let _ = fs::remove_file(&tmp_html);
	if !status.is_ok_and(|s| s.success()) {
		let _ = fs::remove_file(&tmp_md);
		return Err("pandoc html->markdown conversion failed".into());
	}
	let md = fs::read_to_string(&tmp_md)?;
	let _ = fs::remove_file(&tmp_md);
	Ok(md)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_existing_file_needs_translation() {
		assert!(needs_translation(None, "abc123"));
	}

	#[test]
	fn file_without_marker_needs_translation() {
		assert!(needs_translation(Some("# Paperback\n\nSome text"), "abc123"));
	}

	#[test]
	fn file_with_matching_marker_is_skipped() {
		let hash = source_hash("some source content");
		let content = format!("{}\n\nTranslated text", marker_line(&hash));
		assert!(!needs_translation(Some(&content), &hash));
	}

	#[test]
	fn file_with_stale_marker_needs_translation() {
		let content = format!("{}\n\nTranslated text", marker_line("oldhash0000000"));
		assert!(needs_translation(Some(&content), "newhash0000000"));
	}

	#[test]
	fn source_hash_is_stable_and_content_sensitive() {
		assert_eq!(source_hash("hello"), source_hash("hello"));
		assert_ne!(source_hash("hello"), source_hash("world"));
	}

	#[test]
	fn marker_hash_round_trips_through_the_first_line() {
		let hash = source_hash("readme content here");
		let content = format!("{}\nrest of the file", marker_line(&hash));
		assert_eq!(existing_marker_hash(&content), Some(hash.as_str()));
	}
}
