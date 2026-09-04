use std::{
	collections::hash_map::DefaultHasher,
	error::Error,
	fs,
	hash::{Hash, Hasher},
	path::Path,
};

use super::{
	claude::{ClaudeClient, language_name},
	markdown::split_sections,
};

const MARKER_PREFIX: &str = "<!-- machine-translated from doc/readme.md (source-hash: ";
const MARKER_SUFFIX: &str = "); please review and edit as needed -->";
/// Separates the whole-document hash from the per-section ones inside the marker.
const SECTIONS_TAG: &str = "; sections: ";

/// Machine-translates `doc/readme.md` into `doc/readme-<lang>.md` for every language in
/// `langs`. Every language goes through the same pipeline, including ones that already have
/// a translated file, machine-generated or not, since there's no reliable way to tell how
/// stale a hand-written one has become. It is gated only on whether the English source has
/// changed since the last time that language's file was (re)written (see
/// [`needs_translation`]).
///
/// Only the sections that changed are sent to the model. The rest are carried over from the
/// existing translation untouched, so a one-line edit costs one request and leaves the other
/// sections byte-identical instead of rewriting the whole document in fresh wording. Where the
/// sections cannot be lined up, the whole document is translated as before: see
/// [`reusable_sections`].
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
	let sections = split_sections(&source_md);
	let hashes: Vec<String> = sections.iter().map(|section| section_hash(section)).collect();
	for lang in langs {
		let target_path = doc_dir.join(format!("readme-{lang}.md"));
		let existing = fs::read_to_string(&target_path).ok();
		if !needs_translation(existing.as_deref(), &hash) {
			continue;
		}
		let reusable = existing.as_deref().and_then(|content| reusable_sections(content, &hashes));
		let to_translate = reusable.as_ref().map_or(sections.len(), |r| r.iter().filter(|s| s.is_none()).count());
		if dry_run {
			match &reusable {
				Some(_) => println!("readme-{lang}.md: would translate {to_translate} of {} sections", sections.len()),
				None => println!("readme-{lang}.md: would be translated in full"),
			}
			continue;
		}
		let Some(client) = client else { unreachable!("client is always Some outside --dry-run") };
		let Some(language) = language_name(lang) else {
			continue;
		};
		let translated_md = match reusable {
			Some(reusable) => {
				let mut out: Vec<String> = Vec::with_capacity(sections.len());
				for (section, existing_section) in sections.iter().zip(reusable) {
					match existing_section {
						Some(kept) => out.push(kept),
						None => out.push(client.translate_markdown(section, language)?),
					}
				}
				out.join("\n\n")
			}
			None => client.translate_markdown(&source_md, language)?,
		};
		fs::write(&target_path, format!("{}\n\n{}\n", marker_line(&hash, &hashes), translated_md.trim_end()))?;
		println!("readme-{lang}.md ({language}): translated {to_translate} of {} sections", sections.len());
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

/// The existing translation of each source section, where it can be carried over unchanged.
///
/// `Some(v)` with `v[i]` holding the translated text of source section `i` when that section's
/// English text is unchanged, and `None` in that slot when it has to be translated again.
///
/// `None` for the whole thing when the sections cannot be lined up with confidence: the file
/// carries no per-section hashes (written before this existed, or by hand), or the number of
/// sections in the source, in the marker, or in the translation disagree. A section added or
/// removed shifts every section after it, and a hand-edited file may not be split the same way
/// at all, so pairing by position would carry the wrong text across. Translating the whole
/// document is slower but cannot silently reattach a heading to the wrong body.
fn reusable_sections(existing_content: &str, source_hashes: &[String]) -> Option<Vec<Option<String>>> {
	let stored = existing_marker_section_hashes(existing_content)?;
	if stored.len() != source_hashes.len() {
		return None;
	}
	let body = existing_content.split_once('\n').map_or("", |(_, rest)| rest).trim_start();
	let translated = split_sections(body);
	if translated.len() != stored.len() {
		return None;
	}
	Some(
		stored
			.iter()
			.zip(source_hashes)
			.zip(translated)
			.map(|((was, now), text)| (was == now).then_some(text))
			.collect(),
	)
}

fn source_hash(source_md: &str) -> String {
	let mut hasher = DefaultHasher::new();
	source_md.hash(&mut hasher);
	format!("{:016x}", hasher.finish())
}

/// Shorter than the document hash: one of these is written per section, and the marker stays a
/// single line. Half the bits is ample for telling a couple of dozen sections apart, and a
/// collision costs a section that was not re-translated, not a wrong one.
fn section_hash(section: &str) -> String {
	let mut hasher = DefaultHasher::new();
	section.hash(&mut hasher);
	format!("{:08x}", hasher.finish() & 0xffff_ffff)
}

fn marker_line(hash: &str, section_hashes: &[String]) -> String {
	format!("{MARKER_PREFIX}{hash}{SECTIONS_TAG}{}{MARKER_SUFFIX}", section_hashes.join(","))
}

fn existing_marker_hash(content: &str) -> Option<&str> {
	let marker = content.lines().next()?.trim().strip_prefix(MARKER_PREFIX)?.strip_suffix(MARKER_SUFFIX)?;
	Some(marker.split_once(SECTIONS_TAG).map_or(marker, |(hash, _)| hash))
}

fn existing_marker_section_hashes(content: &str) -> Option<Vec<&str>> {
	let marker = content.lines().next()?.trim().strip_prefix(MARKER_PREFIX)?.strip_suffix(MARKER_SUFFIX)?;
	let (_, sections) = marker.split_once(SECTIONS_TAG)?;
	Some(sections.split(',').collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	fn marker(hash: &str, sections: &[&str]) -> String {
		marker_line(hash, &sections.iter().map(|s| (*s).to_string()).collect::<Vec<_>>())
	}

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
		let content = format!("{}\n\nTranslated text", marker(&hash, &["aaaaaaaa"]));
		assert!(!needs_translation(Some(&content), &hash));
	}

	#[test]
	fn file_with_stale_marker_needs_translation() {
		let content = format!("{}\n\nTranslated text", marker("oldhash0000000", &["aaaaaaaa"]));
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
		let content = format!("{}\nrest of the file", marker(&hash, &["aaaaaaaa", "bbbbbbbb"]));
		assert_eq!(existing_marker_hash(&content), Some(hash.as_str()));
		assert_eq!(existing_marker_section_hashes(&content), Some(vec!["aaaaaaaa", "bbbbbbbb"]));
	}

	/// The point of the whole thing: an edit to one section leaves the others alone.
	#[test]
	fn only_the_changed_section_is_marked_for_translation() {
		let old_hashes = ["11111111".to_string(), "22222222".to_string(), "33333333".to_string()];
		let content =
			format!("{}\n\nIntro text\n\n## One\n\nFirst\n\n## Two\n\nSecond", marker_line("doc", &old_hashes));
		let now = ["11111111".to_string(), "99999999".to_string(), "33333333".to_string()];
		let reusable = reusable_sections(&content, &now).expect("sections line up");
		assert!(reusable[0].is_some(), "unchanged first section should carry over");
		assert!(reusable[1].is_none(), "changed section should be re-translated");
		assert!(reusable[2].is_some(), "unchanged last section should carry over");
	}

	/// The same journey a real file makes: translated once, the English edited in one place,
	/// then asked what needs doing. Uses the real hashes rather than stand-ins, so a change to
	/// how sections are cut or hashed shows up here.
	#[test]
	fn a_real_edit_reuses_every_untouched_section() {
		let before = "# Title

Intro.

## Features

One.

## Changelog

Old entry.";
		let hashes: Vec<String> = split_sections(before).iter().map(|s| section_hash(s)).collect();
		assert_eq!(hashes.len(), 3, "title, features, changelog");
		// What the pipeline would have written after translating it.
		let translated = format!(
			"{}

# Titre

Intro.

## Fonctions

Une.

## Journal

Ancienne entree.",
			marker_line(&source_hash(before), &hashes)
		);
		// One line added to the changelog, which is the common case.
		let after = "# Title

Intro.

## Features

One.

## Changelog

New entry.

Old entry.";
		let now: Vec<String> = split_sections(after).iter().map(|s| section_hash(s)).collect();
		let reusable = reusable_sections(&translated, &now).expect("same shape, so the sections pair up");
		assert_eq!(reusable.iter().filter(|s| s.is_none()).count(), 1, "only the changelog changed");
		assert!(reusable[2].is_none(), "the changelog is the section that changed");
		assert!(reusable[0].as_deref().unwrap().contains("Titre"), "the title section carries over as translated");
		assert!(reusable[1].as_deref().unwrap().contains("Fonctions"), "features carries over as translated");
	}

	/// A file written before per-section hashes existed, or edited by hand into a different
	/// shape, cannot be paired up; translating it whole is the safe answer.
	#[test]
	fn a_file_without_section_hashes_cannot_be_reused() {
		let content = format!("{}{}{}\n\nIntro\n\n## One\n\nFirst", MARKER_PREFIX, "abc", MARKER_SUFFIX);
		assert_eq!(reusable_sections(&content, &["1".to_string(), "2".to_string()]), None);
	}

	#[test]
	fn a_source_that_gained_a_section_cannot_be_reused() {
		let content =
			format!("{}\n\nIntro\n\n## One\n\nFirst", marker_line("doc", &["1".to_string(), "2".to_string()]));
		let now = ["1".to_string(), "2".to_string(), "3".to_string()];
		assert_eq!(reusable_sections(&content, &now), None);
	}

	/// The marker says two sections but the body holds three, so the file was edited under the
	/// pipeline's feet and positions can no longer be trusted.
	#[test]
	fn a_translation_that_does_not_match_its_own_marker_cannot_be_reused() {
		let content = format!(
			"{}\n\nIntro\n\n## One\n\nFirst\n\n## Two\n\nSecond",
			marker_line("doc", &["1".to_string(), "2".to_string()])
		);
		assert_eq!(reusable_sections(&content, &["1".to_string(), "2".to_string()]), None);
	}
}
