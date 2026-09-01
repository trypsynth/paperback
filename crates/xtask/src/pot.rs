//! Generating `po/paperback.pot`: the Rust scan, the Kotlin and Swift scans layered on top,
//! and dropping entries whose call site is gone.

use std::{collections::HashMap, error::Error, fs, path::Path};

use walkdir::WalkDir;

use crate::{
	pot_lint,
	workspace::{crate_version, project_root, translatable_crate_src_dirs},
};

pub(crate) fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");
	// Step 1: generate from Rust crates tagged with translatable = true. patois-build
	// sanitizes the sources for `xgettext --language=C` itself, so the real source dirs go
	// straight in.
	let translatable_dirs = translatable_crate_src_dirs(&root)?;
	if translatable_dirs.is_empty() {
		return Err("no translatable crates found — check [package.metadata.patois] translatable = true".into());
	}
	// Before anything is written: a concatenated call would otherwise land in the pot as a
	// fragment that no lookup can ever match.
	pot_lint::check_sources(&root)?;
	let version = crate_version(&root, "paperback")?;
	patois_build::gen_pot_from_dirs(&translatable_dirs, &po_dir, "paperback", &version)?;
	// Steps 2 and 3: layer the mobile front-ends' own strings on top. Both directories are
	// required rather than skipped when absent: the Kotlin path spent a package rename
	// (dev.paperback.mobile -> dev.paperback.android) pointing at nothing, and because a
	// missing directory was silently fine, the only symptom was that Kotlin strings stopped
	// being extracted, which stayed invisible for as long as the pot carried the last good
	// copy of them forward.
	let ios_dir = root.join("ios/Paperback");
	// The uniffi-generated bindings live in the sibling `kotlin/uniffi` tree and have no
	// translatable strings, so the scan starts below them.
	let android_dir = root.join("android/app/src/main/kotlin/dev/paperback/android");
	extend_pot_from(&pot_file, &ios_dir, "swift")?;
	extend_pot_from(&pot_file, &android_dir, "kt")?;
	prune_dead_foreign_entries(&pot_file, &[("swift", &ios_dir), ("kt", &android_dir)])
}

/// Scan one mobile front-end's sources and add whatever they turn up to the pot, failing loudly
/// if the directory isn't where it's expected to be.
fn extend_pot_from(pot_file: &Path, src: &Path, extension: &str) -> Result<(), Box<dyn Error>> {
	if !src.is_dir() {
		return Err(format!("{extension} sources not found at {}", src.display()).into());
	}
	patois_build::extend_pot_from_source_dirs(&[src], extension, pot_file)?;
	Ok(())
}

/// Drop `#: swift`/`#: kt` entries whose exact source text no longer appears anywhere in that
/// platform's sources. `extend_pot_from_source_dirs` only ever adds entries — it has no way to
/// tell a removed or reworded string from one it just hasn't rescanned yet — so once a mobile
/// string changes or disappears from `t()`/`nt()` calls, its old pot entry stays forever unless
/// something prunes it back out. This never touches an entry with any other `#:` reference (the
/// Rust side gets a clean rebuild from `xgettext` every time, so it never needs this).
fn prune_dead_foreign_entries(pot_file: &Path, platforms: &[(&str, &Path)]) -> Result<(), Box<dyn Error>> {
	let mut sources: HashMap<&str, String> = HashMap::new();
	for (tag, dir) in platforms {
		let mut combined = String::new();
		for entry in WalkDir::new(dir) {
			let entry = entry?;
			if entry.path().extension().and_then(|e| e.to_str()) == Some(tag) {
				combined.push_str(&fs::read_to_string(entry.path())?);
				combined.push('\n');
			}
		}
		sources.insert(*tag, combined);
	}

	let pot = fs::read_to_string(pot_file)?;
	let (kept, removed) = prune_pot(&pot, &sources);
	if !removed.is_empty() {
		fs::write(pot_file, kept)?;
		println!("gen-pot: pruned {} dead mobile entry/entries:", removed.len());
		for literal in &removed {
			println!("  {literal}");
		}
	}
	Ok(())
}

/// The filtering itself, kept separate from the file/directory I/O around it so it can run
/// against fixture strings in a test: every `pot` record is kept unless [`foreign_tag`] and
/// [`quoted_msgid`] both find something and that exact text is missing from `sources[tag]`.
/// Returns the surviving pot text and the literal (quotes included) of each dropped entry.
fn prune_pot<'a>(pot: &'a str, sources: &HashMap<&str, String>) -> (String, Vec<&'a str>) {
	let mut removed: Vec<&str> = Vec::new();
	let kept: Vec<&str> = pot
		.split("\n\n")
		.filter(|record| {
			let Some(tag) = foreign_tag(record) else { return true };
			let Some(literal) = quoted_msgid(record) else { return true };
			let alive = sources.get(tag).is_some_and(|src| src.contains(literal));
			if !alive {
				removed.push(literal);
			}
			alive
		})
		.collect();
	(kept.join("\n\n"), removed)
}

/// The tag on a record's first line, if it's a bare `#: kt` or `#: swift` reference — the shape
/// only [`extend_pot_from`] ever writes, never `xgettext` (whose Rust references are always a
/// real `path:line`, never the bare platform tag by itself).
fn foreign_tag(record: &str) -> Option<&str> {
	match record.lines().next()?.strip_prefix("#: ")? {
		tag @ ("kt" | "swift") => Some(tag),
		_ => None,
	}
}

/// A record's `msgid "..."` line, from the opening quote to the end of the line, quotes
/// included — the same bytes `extend_pot_from_source_dirs` copied out of the source file, so
/// searching for this exact substring is equivalent to asking whether that call site is still
/// there.
fn quoted_msgid(record: &str) -> Option<&str> {
	let line = record.lines().find(|line| line.starts_with("msgid "))?;
	let start = line.find('"')?;
	Some(&line[start..])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn foreign_tag_matches_a_bare_kt_or_swift_reference() {
		assert_eq!(foreign_tag("#: kt\nmsgid \"Cancel\"\nmsgstr \"\""), Some("kt"));
		assert_eq!(foreign_tag("#: swift\nmsgid \"Cancel\"\nmsgstr \"\""), Some("swift"));
	}

	// A real xgettext reference is always a path with a line number, never the bare word "kt"
	// or "swift" — if one ever collided, this must keep treating it as a Rust entry.
	#[test]
	fn foreign_tag_ignores_a_rust_source_reference() {
		assert_eq!(foreign_tag("#: crates/paperback/src/kt.rs:12\nmsgid \"Cancel\"\nmsgstr \"\""), None);
	}

	#[test]
	fn foreign_tag_ignores_the_header_record() {
		assert_eq!(foreign_tag("# SOME DESCRIPTIVE TITLE.\nmsgid \"\"\nmsgstr \"\""), None);
	}

	#[test]
	fn quoted_msgid_is_the_literal_with_its_quotes() {
		assert_eq!(quoted_msgid("#: kt\nmsgid \"Go To...\"\nmsgstr \"\""), Some("\"Go To...\""));
	}

	#[test]
	fn quoted_msgid_keeps_the_plural_many_marker_byte_for_byte() {
		assert_eq!(quoted_msgid("#: kt\nmsgid \"{} seconds\u{2063}\"\nmsgstr \"\""), Some("\"{} seconds\u{2063}\""));
	}

	#[test]
	fn prune_drops_a_kt_entry_missing_from_its_source() {
		let pot = "#: kt\nmsgid \"Go To\u{2026}\"\nmsgstr \"\"\n\n#: kt\nmsgid \"Cancel\"\nmsgstr \"\"";
		let sources = HashMap::from([("kt", "t(\"Cancel\")".to_string())]);
		let (kept, removed) = prune_pot(pot, &sources);
		assert_eq!(removed, vec!["\"Go To\u{2026}\""]);
		assert!(kept.contains("Cancel"));
		assert!(!kept.contains("Go To\u{2026}"));
	}

	// The exact case this exists for: the same English text used to be one dead call site and
	// still is a live one elsewhere, distinguished only by the trailing marker byte.
	#[test]
	fn prune_tells_the_marked_form_apart_from_the_unmarked_one() {
		let pot = "#: kt\nmsgid \"{} seconds\"\nmsgstr \"\"\n\n#: kt\nmsgid \"{} seconds\u{2063}\"\nmsgstr \"\"";
		let sources = HashMap::from([(
			"kt",
			"nt(t(\"{} second\"), t(\"{} seconds\"), t(\"{} seconds\u{2063}\"), n)".to_string(),
		)]);
		let (kept, removed) = prune_pot(pot, &sources);
		assert!(removed.is_empty());
		assert!(kept.contains("\"{} seconds\"") && kept.contains("\"{} seconds\u{2063}\""));
	}

	// A record with no known tag (Rust, or the header) always survives, even when its own
	// "source" text is nowhere in the map — this function is never the thing that decides Rust
	// liveness.
	#[test]
	fn prune_leaves_non_foreign_records_untouched() {
		let pot = "#: crates/paperback/src/lib.rs:1\nmsgid \"Ready\"\nmsgstr \"\"";
		let (kept, removed) = prune_pot(pot, &HashMap::new());
		assert!(removed.is_empty());
		assert_eq!(kept, pot);
	}

	#[test]
	fn prune_joins_survivors_without_a_stray_blank_line() {
		let pot = "#: kt\nmsgid \"Dead\"\nmsgstr \"\"\n\n#: kt\nmsgid \"Alive\"\nmsgstr \"\"\n\n#: kt\nmsgid \"Also alive\"\nmsgstr \"\"";
		let sources = HashMap::from([("kt", "t(\"Alive\") t(\"Also alive\")".to_string())]);
		let (kept, _) = prune_pot(pot, &sources);
		assert_eq!(kept, "#: kt\nmsgid \"Alive\"\nmsgstr \"\"\n\n#: kt\nmsgid \"Also alive\"\nmsgstr \"\"");
	}
}
