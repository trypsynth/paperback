use std::{
	collections::{HashMap, HashSet},
	env,
	error::Error,
	fs,
	path::{Path, PathBuf},
	process::{self, Command},
};

mod claude;
mod readme;

use patois_build::po::PoDocument;

use crate::project_root;

/// Regenerates `po/paperback.pot`, syncs every `po/<lang>.po` against it via `msgmerge`
/// (adds blank entries for new strings, flags changed-but-similar entries `#, fuzzy`),
/// then fills any blank/fuzzy entry via the Claude API (see `claude`), passing each string's
/// `#. TRANSLATORS:` note along with it.
/// Writes back only when something genuinely changed: `msgmerge` regenerates the
/// `POT-Creation-Date`/`PO-Revision-Date` header lines on every run regardless of real
/// content changes, so those are ignored when deciding whether to touch a file, to avoid
/// committing pure timestamp churn. Finally does the same for `doc/readme.md`, machine-
/// translating it into `doc/readme-<lang>.md` for the same set of languages (see
/// `readme::sync_readmes`).
///
/// Locales listed in `po/human-maintained-locales.txt` are skipped entirely, for both the
/// po-string sync and the README sync. See that file and
/// <https://github.com/trypsynth/paperback/issues/638>.
pub fn translate() -> Result<(), Box<dyn Error>> {
	let mut dry_run = false;
	let mut repair = false;
	for arg in env::args().skip(2) {
		match arg.as_str() {
			"--dry-run" => dry_run = true,
			"--repair" => repair = true,
			_ => {
				crate::print_help();
				return Err(format!("Unknown argument for translate: {arg}").into());
			}
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
	let client = if dry_run {
		None
	} else {
		let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| "ANTHROPIC_API_KEY environment variable is not set")?;
		// An empty value is its own case, and worth naming. A CI secret that exists but holds
		// an empty string satisfies `env::var`, so without this the run gets all the way to the
		// API and comes back "x-api-key header is required", which reads like a broken key
		// rather than a missing one and sends you looking in the wrong place.
		if api_key.trim().is_empty() {
			return Err("ANTHROPIC_API_KEY is set but empty".into());
		}
		let client = claude::ClaudeClient::new(api_key);
		println!("translating with {}", client.model());
		Some(client)
	};
	// The `#. TRANSLATORS:` comments, keyed by the string they annotate. They live in the pot
	// rather than in any po file, and the po parser doesn't carry them, so they're collected
	// once here and handed to the translator alongside each string.
	let context = translator_comments(&fs::read_to_string(&pot_path)?);
	let mut po_files: Vec<PathBuf> = fs::read_dir(&po_dir)?
		.filter_map(Result::ok)
		.map(|e| e.path())
		.filter(|p| p.extension().and_then(|e| e.to_str()) == Some("po"))
		.collect();
	po_files.sort();
	let langs: Vec<String> =
		po_files.iter().filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string)).collect();
	let human_maintained = load_human_maintained_locales(&root)?;
	for po_path in &po_files {
		let lang = po_path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
		if human_maintained.contains(lang) {
			println!("{lang}: human-maintained, skipping");
			continue;
		}
		translate_one(po_path, &pot_path, dry_run, repair, client.as_ref(), &context)?;
	}
	let auto_langs: Vec<String> = langs.into_iter().filter(|l| !human_maintained.contains(l.as_str())).collect();
	readme::sync_readmes(&root, &auto_langs, client.as_ref(), dry_run)?;
	Ok(())
}

/// Parses `#. TRANSLATORS:` comments out of a pot, keyed by the msgid each one sits above.
///
/// Comment lines accumulate until a `msgid` line claims them, which is how gettext associates
/// them; a run of them belonging to one entry is joined into a single note.
///
/// The keys have to match what `PoDocument` reports as an entry's msgid, so they are unescaped
/// and joined across continuation lines the same way it does. Keying on the raw quoted text
/// instead would silently miss exactly the long or quote-containing strings that need a
/// translator note most.
fn translator_comments(pot: &str) -> HashMap<String, String> {
	let mut out = HashMap::new();
	let mut pending: Vec<String> = Vec::new();
	let lines: Vec<&str> = pot.lines().map(str::trim).collect();
	let mut i = 0;
	while i < lines.len() {
		let line = lines[i];
		if let Some(rest) = line.strip_prefix("#.") {
			let rest = rest.trim();
			// "TRANSLATORS:" is a convention for whoever reads the pot; the note after it is
			// what carries the meaning, so the marker itself is dropped.
			pending.push(rest.strip_prefix("TRANSLATORS:").unwrap_or(rest).trim().to_string());
			i += 1;
			continue;
		}
		if let Some(rest) = line.strip_prefix("msgid ") {
			let mut msgid = po_unquote(rest);
			i += 1;
			while i < lines.len() && lines[i].starts_with('"') {
				msgid.push_str(&po_unquote(lines[i]));
				i += 1;
			}
			if !pending.is_empty() && !msgid.is_empty() {
				out.insert(msgid, pending.join(" "));
			}
			pending.clear();
			continue;
		}
		// Any other line (a blank separator, a #, flag, a msgstr) ends the comment run: a
		// note only ever belongs to the entry directly below it.
		if !line.starts_with('#') {
			pending.clear();
		}
		i += 1;
	}
	out
}

/// Decodes one quoted po/pot string, matching the unescaping `PoDocument` applies.
fn po_unquote(s: &str) -> String {
	let s = s.trim();
	if s.len() < 2 || !s.starts_with('"') || !s.ends_with('"') {
		return String::new();
	}
	let mut out = String::new();
	let mut chars = s[1..s.len() - 1].chars();
	while let Some(c) = chars.next() {
		if c != '\\' {
			out.push(c);
			continue;
		}
		match chars.next() {
			Some('n') => out.push('\n'),
			Some('t') => out.push('\t'),
			Some('"') => out.push('"'),
			// An escaped backslash, and a stray one at the very end, both yield one backslash.
			Some('\\') | None => out.push('\\'),
			Some(other) => {
				out.push('\\');
				out.push(other);
			}
		}
	}
	out
}

/// Locale codes listed in `po/human-maintained-locales.txt`, one per line (`#` starts a
/// comment; blank lines ignored). These are skipped entirely by both the po-string sync
/// and the README sync. See that file for why.
fn load_human_maintained_locales(root: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
	let path = root.join("po").join("human-maintained-locales.txt");
	let Ok(content) = fs::read_to_string(&path) else {
		return Ok(HashSet::new());
	};
	Ok(parse_human_maintained_locales(&content))
}

fn parse_human_maintained_locales(content: &str) -> HashSet<String> {
	content
		.lines()
		.map(|line| line.split('#').next().unwrap_or("").trim())
		.filter(|line| !line.is_empty())
		.map(str::to_string)
		.collect()
}

/// Adds entries whose existing translation is provably damaged to `candidates`, returning how
/// many were added.
///
/// Needed because `--previous` only helps from here on. `msgmerge` writes the `#| msgid` line
/// that marks an entry for re-translation at the moment it fuzzy-matches, so entries stranded
/// before the flag was added are already `#, fuzzy` with no `#|`, their msgids still match the
/// pot exactly, and no future merge will ever touch them again. Nothing in the normal flow can
/// reach them; this is what does.
///
/// Only entries that fail a mechanical check are added - a dropped placeholder, accelerator or
/// shortcut suffix. A translation that merely looks doubtful is left alone: re-translating on
/// suspicion would churn thousands of entries that are perfectly fine, and the checks are the
/// only part of this that can be right or wrong on its own.
fn add_damaged_entries(doc: &PoDocument, candidates: &mut Vec<(usize, String)>) -> usize {
	let already: HashSet<usize> = candidates.iter().map(|(i, _)| *i).collect();
	let damaged: Vec<(usize, String)> = doc
		.entries
		.iter()
		.enumerate()
		.filter(|(i, e)| !already.contains(i) && claude::is_damaged(&e.msgid, &e.msgstr))
		.map(|(i, e)| (i, e.msgid.clone()))
		.collect();
	let count = damaged.len();
	candidates.extend(damaged);
	count
}

fn translate_one(
	po_path: &Path,
	pot_path: &Path,
	dry_run: bool,
	repair: bool,
	client: Option<&claude::ClaudeClient>,
	context: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
	let lang = po_path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
	let original = fs::read_to_string(po_path)?;
	// Work on a scratch copy so po_path is never touched unless there's a real change to
	// write back (checked at the end) — true for --dry-run and for the "msgmerge only
	// bumped a timestamp" case alike.
	let tmp = env::temp_dir().join(format!("paperback-translate-{lang}-{}.po", process::id()));
	fs::write(&tmp, &original)?;
	// `--previous` is load-bearing, not cosmetic. When msgmerge fuzzy-matches a changed string
	// against a similar old one, it copies that old translation across and marks the entry
	// `#, fuzzy`; only `--previous` also records the string it matched against, as a `#| msgid`
	// line. That line is the sole thing distinguishing "msgmerge just guessed at this, it needs
	// translating" from "already machine-translated, flagged for a human, leave it alone", and
	// `PoDocument::needs_translation` selects on exactly that.
	//
	// Without the flag every fuzzy entry looked like the second kind, so a changed string
	// silently kept whatever translation msgmerge had guessed and was never offered for
	// translation again. That is how `&Settings` ended up showing the old translation of
	// `Settings` and `&Close` the old translation of `Close`: same words, accelerator quietly
	// gone, and nothing would ever have revisited them.
	let msgmerge_ok = Command::new("msgmerge")
		.args(["--update", "--backup=none", "--no-wrap", "--previous"])
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
	let mut doc = PoDocument::parse(&merged);
	let mut candidates: Vec<(usize, String)> = doc.needs_translation().map(|(i, m)| (i, m.to_string())).collect();
	let repaired = if repair { add_damaged_entries(&doc, &mut candidates) } else { 0 };
	if dry_run {
		if candidates.is_empty() {
			println!("{lang}: fully translated, nothing to do");
		} else if repaired > 0 {
			println!("{lang}: {} entries would be translated ({repaired} of them damaged)", candidates.len());
		} else {
			println!("{lang}: {} entries would be translated", candidates.len());
		}
		return Ok(());
	}
	let final_content = if candidates.is_empty() {
		merged
	} else {
		let Some(client) = client else { unreachable!("client is always Some outside --dry-run") };
		match claude::language_name(&lang) {
			None => {
				println!("{lang}: no language name mapped, skipping ({} entries need one)", candidates.len());
				merged
			}
			Some(language) => {
				let phrases: Vec<claude::Phrase> = candidates
					.iter()
					.map(|(_, text)| claude::Phrase { source: text.clone(), context: context.get(text).cloned() })
					.collect();
				let annotated = phrases.iter().filter(|p| p.context.is_some()).count();
				let results = client.translate_phrases(&phrases, language)?;
				let translations: Vec<(usize, String)> = candidates
					.iter()
					.map(|(i, _)| *i)
					.zip(results)
					.filter_map(|(i, result)| result.map(|text| (i, text)))
					.collect();
				let count = translations.len();
				let skipped = candidates.len() - count;
				doc.apply_all(&translations);
				print!("{lang} ({language}): translated {count} entries");
				if repaired > 0 {
					print!(", {repaired} of them repaired");
				}
				if annotated > 0 {
					print!(", {annotated} with translator notes");
				}
				if skipped > 0 {
					print!(", skipped {skipped} (failed a placeholder/accelerator check, will retry next run)");
				}
				println!();
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

	#[test]
	fn human_maintained_locales_parses_one_per_line() {
		let content = "bs\nfi\nsr\n";
		let locales = parse_human_maintained_locales(content);
		assert_eq!(locales, HashSet::from(["bs".to_string(), "fi".to_string(), "sr".to_string()]));
	}

	#[test]
	fn human_maintained_locales_ignores_comments_and_blank_lines() {
		let content = "# comment\n\nfi   # Jani Kinnunen\n\n  sr  \n";
		let locales = parse_human_maintained_locales(content);
		assert_eq!(locales, HashSet::from(["fi".to_string(), "sr".to_string()]));
	}

	#[test]
	fn human_maintained_locales_empty_when_only_comments() {
		let content = "# nothing here yet\n";
		assert!(parse_human_maintained_locales(content).is_empty());
	}

	#[test]
	fn a_translator_note_attaches_to_the_msgid_below_it() {
		let pot = "#. TRANSLATORS: Default status bar text when no document is open\nmsgid \"Ready\"\nmsgstr \"\"\n";
		let notes = translator_comments(pot);
		assert_eq!(notes.get("Ready").map(String::as_str), Some("Default status bar text when no document is open"));
	}

	#[test]
	fn a_msgid_with_no_note_gets_no_entry() {
		let pot = "msgid \"Ready\"\nmsgstr \"\"\n\nmsgid \"Cancel\"\nmsgstr \"\"\n";
		assert!(translator_comments(pot).is_empty());
	}

	// A note belongs only to the entry directly below it. Without the reset, a blank line
	// after an entry would let its note drift onto the next, unrelated string.
	#[test]
	fn a_note_does_not_drift_onto_the_following_entry() {
		let pot = "#. TRANSLATORS: about Ready\nmsgid \"Ready\"\nmsgstr \"\"\n\nmsgid \"Cancel\"\nmsgstr \"\"\n";
		let notes = translator_comments(pot);
		assert!(notes.contains_key("Ready"));
		assert!(!notes.contains_key("Cancel"));
	}

	// The keys have to match PoDocument's unescaped, joined msgid or the lookup silently
	// misses, which would hit exactly the long strings that most need their note.
	#[test]
	fn keys_are_unescaped_and_joined_across_continuation_lines() {
		let pot =
			"#. TRANSLATORS: a two-line prompt\nmsgid \"\"\n\"No parser for {}.\\n\"\n\"Open it how?\"\nmsgstr \"\"\n";
		let notes = translator_comments(pot);
		assert_eq!(notes.get("No parser for {}.\nOpen it how?").map(String::as_str), Some("a two-line prompt"));
	}

	#[test]
	fn an_escaped_quote_in_a_msgid_survives_the_key() {
		let pot = "#. TRANSLATORS: note\nmsgid \"Say \\\"hi\\\"\"\nmsgstr \"\"\n";
		assert!(translator_comments(pot).contains_key("Say \"hi\""));
	}

	#[test]
	fn several_note_lines_join_into_one() {
		let pot = "#. TRANSLATORS: first line\n#. second line\nmsgid \"Ready\"\nmsgstr \"\"\n";
		let notes = translator_comments(pot);
		assert_eq!(notes.get("Ready").map(String::as_str), Some("first line second line"));
	}

	// The header's empty msgid is not a translatable string, so a note above it has nothing
	// to attach to and must not leak onto the first real entry.
	#[test]
	fn the_header_entry_never_takes_a_note() {
		let pot = "#. TRANSLATORS: stray\nmsgid \"\"\nmsgstr \"\"\n\nmsgid \"Ready\"\nmsgstr \"\"\n";
		assert!(translator_comments(pot).is_empty());
	}
}
