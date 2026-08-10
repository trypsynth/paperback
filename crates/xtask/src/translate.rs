use std::{collections::HashSet, env, error::Error, fs, path::PathBuf, process::Command};

mod deepl;
mod po;
mod readme;

use crate::project_root;

/// Regenerates `po/paperback.pot`, syncs every `po/<lang>.po` against it via `msgmerge`
/// (adds blank entries for new strings, flags changed-but-similar entries `#, fuzzy`),
/// then fills any blank/fuzzy entry via the `DeepL` API for languages `DeepL` supports.
/// Writes back only when something genuinely changed — `msgmerge` regenerates the
/// `POT-Creation-Date`/`PO-Revision-Date` header lines on every run regardless of real
/// content changes, so those are ignored when deciding whether to touch a file, to avoid
/// committing pure timestamp churn. Finally does the same for `doc/readme.md`, machine-
/// translating it into `doc/readme-<lang>.md` for the same set of languages (see
/// `readme::sync_readmes`).
pub fn translate() -> Result<(), Box<dyn Error>> {
	let mut dry_run = false;
	for arg in env::args().skip(2) {
		if arg == "--dry-run" {
			dry_run = true;
		} else {
			crate::print_help();
			return Err(format!("Unknown argument for translate: {arg}").into());
		}
	}

	let root = project_root();
	let po_dir = root.join("po");
	let pot_path = po_dir.join("paperback.pot");

	// gen_pot() unconditionally rewrites paperback.pot with a fresh POT-Creation-Date
	// even when no translatable string actually changed. Left as-is, that alone would
	// make every run "dirty" and every --dry-run leave the working tree modified. Restore
	// the pre-run content when nothing but that timestamp moved (or always, for
	// --dry-run, which must touch nothing on disk).
	let original_pot = fs::read_to_string(&pot_path).ok();
	crate::gen_pot()?;
	if let Some(original) = &original_pot {
		if dry_run {
			fs::write(&pot_path, original)?;
		} else {
			let regenerated = fs::read_to_string(&pot_path)?;
			if content_without_volatile_headers(original) == content_without_volatile_headers(&regenerated) {
				fs::write(&pot_path, original)?;
			}
		}
	}

	// --dry-run makes no API calls and needs no key, so it can be run by anyone locally
	// to preview what a real run would do.
	let client_and_supported = if dry_run {
		None
	} else {
		let api_key = env::var("DEEPL_API_KEY").map_err(|_| "DEEPL_API_KEY environment variable is not set")?;
		let client = deepl::DeepLClient::new(api_key);
		let supported = client.supported_target_languages()?;
		Some((client, supported))
	};

	let mut po_files: Vec<PathBuf> = fs::read_dir(&po_dir)?
		.filter_map(Result::ok)
		.map(|e| e.path())
		.filter(|p| p.extension().and_then(|e| e.to_str()) == Some("po"))
		.collect();
	po_files.sort();
	let langs: Vec<String> =
		po_files.iter().filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string)).collect();

	for po_path in &po_files {
		translate_one(po_path, &pot_path, dry_run, client_and_supported.as_ref())?;
	}

	readme::sync_readmes(&root, &langs, client_and_supported.as_ref(), dry_run)?;

	Ok(())
}

fn translate_one(
	po_path: &std::path::Path,
	pot_path: &std::path::Path,
	dry_run: bool,
	client_and_supported: Option<&(deepl::DeepLClient, HashSet<String>)>,
) -> Result<(), Box<dyn Error>> {
	let lang = po_path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
	let original = fs::read_to_string(po_path)?;

	// Work on a scratch copy so po_path is never touched unless there's a real change to
	// write back (checked at the end) — true for --dry-run and for the "msgmerge only
	// bumped a timestamp" case alike.
	let tmp = env::temp_dir().join(format!("paperback-translate-{lang}-{}.po", std::process::id()));
	fs::write(&tmp, &original)?;
	let msgmerge_ok = Command::new("msgmerge")
		.args(["--update", "--backup=none", "--no-wrap"])
		.arg(&tmp)
		.arg(pot_path)
		.status()
		.is_ok_and(|s| s.success());
	if !msgmerge_ok {
		eprintln!("warning: msgmerge failed for {lang}, leaving it untouched this run");
		let _ = fs::remove_file(&tmp);
		return Ok(());
	}
	let merged = fs::read_to_string(&tmp)?;
	let _ = fs::remove_file(&tmp);

	let mut doc = po::PoDocument::parse(&merged);
	let candidates: Vec<(usize, String)> = doc.needs_translation().map(|(i, m)| (i, m.to_string())).collect();

	if dry_run {
		if candidates.is_empty() {
			println!("{lang}: fully translated, nothing to do");
		} else {
			println!("{lang}: {} entries would be translated", candidates.len());
		}
		return Ok(());
	}

	let final_content = if candidates.is_empty() {
		merged
	} else {
		let Some((client, supported)) = client_and_supported else {
			unreachable!("client_and_supported is always Some outside --dry-run")
		};
		match deepl::resolve_target_lang(&lang, supported) {
			None => {
				println!(
					"{lang}: not supported by DeepL, skipping ({} entries need a human translator)",
					candidates.len()
				);
				merged
			}
			Some(target_lang) => {
				let texts: Vec<String> = candidates.iter().map(|(_, t)| t.clone()).collect();
				let results = client.translate_batch(&texts, &target_lang)?;
				let translations: Vec<(usize, String)> = candidates
					.iter()
					.map(|(i, _)| *i)
					.zip(results)
					.filter_map(|(i, result)| result.map(|text| (i, text)))
					.collect();
				let count = translations.len();
				let skipped = candidates.len() - count;
				doc.apply_all(&translations);
				if skipped > 0 {
					println!(
						"{lang} ({target_lang}): translated {count} entries, skipped {skipped} (placeholder mismatch, will retry next run)"
					);
				} else {
					println!("{lang} ({target_lang}): translated {count} entries");
				}
				doc.render()
			}
		}
	};

	if content_without_volatile_headers(&original) == content_without_volatile_headers(&final_content) {
		return Ok(());
	}
	fs::write(po_path, final_content)?;
	Ok(())
}

/// Drops the `msgmerge`-regenerated `POT-Creation-Date`/`PO-Revision-Date` header lines
/// so an otherwise-identical file doesn't look changed just because those timestamps
/// moved.
fn content_without_volatile_headers(content: &str) -> String {
	content
		.lines()
		.filter(|line| {
			let t = line.trim();
			!(t.starts_with("\"POT-Creation-Date:") || t.starts_with("\"PO-Revision-Date:"))
		})
		.collect::<Vec<_>>()
		.join("\n")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn timestamp_only_changes_are_ignored() {
		let a = "msgid \"\"\nmsgstr \"\"\n\"POT-Creation-Date: 2026-01-01 00:00+0000\\n\"\n\"PO-Revision-Date: 2026-01-01 00:00+0000\\n\"\n\"Language: de\\n\"\n";
		let b = "msgid \"\"\nmsgstr \"\"\n\"POT-Creation-Date: 2026-06-01 12:00+0000\\n\"\n\"PO-Revision-Date: 2026-06-01 12:00+0000\\n\"\n\"Language: de\\n\"\n";
		assert_eq!(content_without_volatile_headers(a), content_without_volatile_headers(b));
	}

	#[test]
	fn real_content_changes_are_detected() {
		let a = "msgid \"Cancel\"\nmsgstr \"\"\n";
		let b = "msgid \"Cancel\"\nmsgstr \"Abbrechen\"\n";
		assert_ne!(content_without_volatile_headers(a), content_without_volatile_headers(b));
	}
}
