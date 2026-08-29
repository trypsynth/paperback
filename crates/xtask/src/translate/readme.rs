use std::{
	collections::hash_map::DefaultHasher,
	error::Error,
	fs,
	hash::{Hash, Hasher},
	path::Path,
};

use super::claude::{ClaudeClient, language_name};

const MARKER_PREFIX: &str = "<!-- machine-translated from doc/readme.md (source-hash: ";
const MARKER_SUFFIX: &str = "); please review and edit as needed -->";

/// Machine-translates `doc/readme.md` into `doc/readme-<lang>.md` for every language in
/// `langs`. Every language goes through the same pipeline, including ones that already have
/// a translated file, machine-generated or not, since there's no reliable way to tell how
/// stale a hand-written one has become. It is gated only on whether the English source has
/// changed since the last time that language's file was (re)written (see
/// [`needs_translation`]).
///
/// The gate is still all-or-nothing per language: one edited sentence re-translates the whole
/// document. Hashing per section instead would cut what a typical edit costs by roughly an
/// order of magnitude, and the chunking in [`ClaudeClient::translate_markdown`] is where that
/// would hook in.
pub fn sync_readmes(
	root: &Path,
	langs: &[String],
	client: Option<&ClaudeClient>,
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
		let Some(client) = client else { unreachable!("client is always Some outside --dry-run") };
		let Some(language) = language_name(lang) else {
			continue;
		};
		let translated_md = client.translate_markdown(&source_md, language)?;
		fs::write(&target_path, format!("{}\n\n{}\n", marker_line(&hash), translated_md.trim_end()))?;
		println!("readme-{lang}.md ({language}): translated");
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
