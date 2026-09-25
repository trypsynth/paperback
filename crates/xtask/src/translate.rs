use std::{
	collections::{HashMap, HashSet},
	env,
	error::Error,
	fs,
	path::{Path, PathBuf},
	process::{self, Command},
};

mod checks;
mod claude;
mod markdown;
mod prompts;
mod readme;

use patois_build::po::{PoDocument, Translation};

use crate::workspace::project_root;

/// Regenerates `po/paperback.pot`, syncs every `po/<lang>.po` against it via `msgmerge`
/// (adds a blank entry for every new or changed string),
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
/// <https://github.com/trypsynth/paperback/issues/638>. A locale that has `po/style/<lang>.md`
/// gets that file appended to the model's instructions (see `load_style_note`).
pub fn translate() -> Result<(), Box<dyn Error>> {
	let mut dry_run = false;
	let mut repair = false;
	let mut repair_copies = false;
	for arg in env::args().skip(2) {
		match arg.as_str() {
			"--dry-run" => dry_run = true,
			"--repair" => repair = true,
			"--repair-copies" => repair_copies = true,
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
	crate::pot::gen_pot()?;
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
		translate_one(&root, po_path, &pot_path, dry_run, repair, repair_copies, client.as_ref(), &context)?;
	}
	let auto_langs: Vec<String> = langs.into_iter().filter(|l| !human_maintained.contains(l.as_str())).collect();
	readme::sync_readmes(&root, &auto_langs, client.as_ref(), dry_run)?;
	Ok(())
}

/// Locale codes listed in `po/human-maintained-locales.txt`, one per line (`#` starts a
/// comment; blank lines ignored). These are skipped entirely by both the po-string sync
/// and the README sync. See that file for why.
/// The `TRANSLATORS:` note for each msgid that has one, read from `pot`.
///
/// Every entry the model is asked about gets its note as context, which is the difference
/// between translating "Open" as a verb and as an adjective.
fn translator_comments(pot: &str) -> HashMap<String, String> {
	patois_build::po::PoDocument::parse(pot)
		.entries
		.into_iter()
		.filter_map(|entry| Some((entry.msgid, entry.comment?)))
		.filter(|(msgid, _)| !msgid.is_empty())
		.collect()
}

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

/// The conventions a locale's translators wrote for the model in `po/style/<lang>.md`, or
/// `None` when there is no such file or it is blank.
pub fn load_style_note(root: &Path, lang: &str) -> Option<String> {
	let path = root.join("po").join("style").join(format!("{lang}.md"));
	parse_style_note(&fs::read_to_string(path).ok()?)
}

fn parse_style_note(content: &str) -> Option<String> {
	let note = content.trim();
	(!note.is_empty()).then(|| note.to_string())
}

pub fn style_note_suffix(lang: &str, present: bool) -> String {
	if present { format!(", with po/style/{lang}.md") } else { String::new() }
}

/// What to splice into a document: the entry index and its translated result.
type Applied = Vec<(usize, Translation)>;

/// Translates the ordinary entries, returning what to apply and how many carried a note.
fn translate_singulars(
	client: &claude::ClaudeClient,
	target: &claude::Target,
	candidates: &[(usize, String)],
	context: &HashMap<String, String>,
) -> Result<(Applied, usize), Box<dyn Error>> {
	if candidates.is_empty() {
		return Ok((Vec::new(), 0));
	}
	let phrases: Vec<claude::Phrase> = candidates
		.iter()
		.map(|(_, text)| claude::Phrase { source: text.clone(), context: context.get(text).cloned() })
		.collect();
	let annotated = phrases.iter().filter(|p| p.context.is_some()).count();
	let results = client.translate_phrases(&phrases, target)?;
	let applied = candidates
		.iter()
		.map(|(i, _)| *i)
		.zip(results)
		.filter_map(|(i, result)| result.map(|text| (i, Translation::Singular(text))))
		.collect();
	Ok((applied, annotated))
}

/// Translates the plural entries, asking for `nplurals` forms of each.
fn translate_plurals(
	client: &claude::ClaudeClient,
	target: &claude::Target,
	candidates: &[(usize, String, String)],
	context: &HashMap<String, String>,
	nplurals: usize,
	rule: &str,
) -> Result<Applied, Box<dyn Error>> {
	if candidates.is_empty() {
		return Ok(Vec::new());
	}
	let phrases: Vec<claude::PluralPhrase> = candidates
		.iter()
		.map(|(_, singular, plural)| claude::PluralPhrase {
			singular: singular.clone(),
			plural: plural.clone(),
			// The note is filed under the singular, which is the msgid the pot comment sits
			// above.
			context: context.get(singular).cloned(),
		})
		.collect();
	let results = client.translate_plurals(&phrases, target, nplurals, rule)?;
	Ok(candidates
		.iter()
		.map(|(i, _, _)| *i)
		.zip(results)
		.filter_map(|(i, result)| result.map(|forms| (i, Translation::Plural(forms))))
		.collect())
}

/// The `nplurals` count and the raw plural rule from a po file's `Plural-Forms` header.
///
/// The count comes from the parsed document, and the rule expression is taken verbatim from the
/// header text so the model is told the language's actual rule rather than a description of it.
fn plural_forms(content: &str) -> Option<(usize, String)> {
	let nplurals = PoDocument::parse(content).nplurals()?;
	let line = content.lines().map(str::trim).find(|l| l.contains("Plural-Forms:"))?;
	let rule = line.trim_start_matches('"').trim_end_matches("\\n\"").trim().to_string();
	Some((nplurals, rule))
}

/// Adds entries whose existing translation is provably damaged to `candidates`, returning how
/// many were added.
///
/// Reaches entries whose damage no merge will ever surface again: their msgid matches the pot
/// exactly and they are already fuzzy, so nothing in the normal flow selects them.
///
/// Only entries that fail a mechanical check are added - a dropped placeholder, accelerator or
/// shortcut suffix. A translation that merely looks doubtful is left alone: re-translating on
/// suspicion would churn thousands of entries that are perfectly fine, and the checks are the
/// only part of this that can be right or wrong on its own.
fn add_damaged_entries(
	doc: &PoDocument,
	candidates: &mut Vec<(usize, String)>,
	plurals: &mut Vec<(usize, String, String)>,
) -> usize {
	let already: HashSet<usize> = candidates.iter().map(|(i, _)| *i).collect();
	let already_plural: HashSet<usize> = plurals.iter().map(|(i, _, _)| *i).collect();
	let mut count = 0;
	for (i, entry) in doc.entries.iter().enumerate() {
		match entry.msgid_plural.as_deref() {
			// A plural entry is damaged when any one of its forms is: the whole set is
			// rewritten together, so one broken form condemns the entry.
			Some(plural) => {
				if already_plural.contains(&i) {
					continue;
				}
				if entry.msgstr_plural.iter().any(|form| checks::is_damaged(plural, form)) {
					plurals.push((i, entry.msgid.clone(), plural.to_string()));
					count += 1;
				}
			}
			None => {
				if !already.contains(&i) && checks::is_damaged(&entry.msgid, &entry.msgstr) {
					candidates.push((i, entry.msgid.clone()));
					count += 1;
				}
			}
		}
	}
	count
}

/// Adds machine-translated entries that are probably a msgmerge copy of some other string's
/// translation, returning how many were added.
///
/// A one-off repair, not something to leave switched on. It finds translations msgmerge copied
/// from a look-alike string (see the `--no-fuzzy-matching` note in `translate_one`), which are
/// recognisable by being shared with an entry whose English says something different. Some pairs share a translation legitimately ("Document Info" and
/// "Document Information"); re-translating those costs a call and changes nothing, which is
/// why this is safe to run once but would be wasteful to run on every push.
///
/// Only fuzzy entries are touched. A non-fuzzy entry was put there by a person, and a shared
/// translation is not evidence enough to overwrite their work.
fn add_copied_entries(doc: &PoDocument, candidates: &mut Vec<(usize, String)>) -> usize {
	let already: HashSet<usize> = candidates.iter().map(|(i, _)| *i).collect();
	let mut sources: HashMap<&str, HashSet<String>> = HashMap::new();
	for entry in &doc.entries {
		if entry.msgid_plural.is_none() && !entry.msgid.is_empty() && !entry.msgstr.is_empty() {
			sources.entry(entry.msgstr.as_str()).or_default().insert(same_meaning_key(&entry.msgid));
		}
	}
	let mut count = 0;
	for (i, entry) in doc.entries.iter().enumerate() {
		let shared = sources.get(entry.msgstr.as_str()).is_some_and(|keys| keys.len() > 1);
		if entry.is_fuzzy && entry.msgid_plural.is_none() && shared && !already.contains(&i) {
			candidates.push((i, entry.msgid.clone()));
			count += 1;
		}
	}
	count
}

/// What two English strings have to agree on to count as saying the same thing: the words,
/// ignoring the accelerator, case, spacing and trailing punctuation. `&Close` and `Close`, or
/// `Open...` and `Open`, sharing a translation is expected; `Batch OCR` and `Match Case` is not.
fn same_meaning_key(msgid: &str) -> String {
	msgid
		.chars()
		.filter(|c| !matches!(c, '&' | '.' | ':' | '…' | '!' | '?') && !c.is_whitespace())
		.flat_map(char::to_lowercase)
		.collect()
}

#[allow(clippy::too_many_arguments)]
fn translate_one(
	root: &Path,
	po_path: &Path,
	pot_path: &Path,
	dry_run: bool,
	repair: bool,
	repair_copies: bool,
	client: Option<&claude::ClaudeClient>,
	context: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
	let lang = po_path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
	let style = load_style_note(root, &lang);
	let original = fs::read_to_string(po_path)?;
	// Work on a scratch copy so po_path is never touched unless there's a real change to
	// write back (checked at the end), true for --dry-run and for the "msgmerge only
	// bumped a timestamp" case alike.
	let tmp = env::temp_dir().join(format!("paperback-translate-{lang}-{}.po", process::id()));
	fs::write(&tmp, &original)?;
	// `--no-fuzzy-matching` is load-bearing. Left to itself, msgmerge fills a new or changed
	// string with the translation of whichever old string it looks most like and flags it
	// `#, fuzzy`. Fuzzy entries are compiled in here (machine translations stay fuzzy on
	// purpose, see `patois_build`), so that guess would be what the reader sees: `{} ms` as
	// "{} minutes", `Batch OCR` as "Match Case". Nothing marks the guess reliably afterwards,
	// either: `--previous` only records the copied-from string when that string's own entry was
	// not fuzzy, which in a machine-translated catalog it almost never is.
	//
	// With matching off, a new or changed string arrives blank and is always translated, and a
	// failed call leaves English until the next run rather than a wrong string.
	let msgmerge_ok = Command::new("msgmerge")
		.args(["--update", "--backup=none", "--no-wrap", "--no-fuzzy-matching"])
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
	let mut plurals: Vec<(usize, String, String)> =
		doc.needs_plural_translation().map(|(i, s, p)| (i, s.to_string(), p.to_string())).collect();
	let mut repaired = if repair { add_damaged_entries(&doc, &mut candidates, &mut plurals) } else { 0 };
	if repair_copies {
		repaired += add_copied_entries(&doc, &mut candidates);
	}
	let total = candidates.len() + plurals.len();
	if dry_run {
		let style_note = style_note_suffix(&lang, style.is_some());
		if total == 0 {
			println!("{lang}: fully translated, nothing to do{style_note}");
		} else {
			let plural_note = if plurals.is_empty() { String::new() } else { format!(", {} plural", plurals.len()) };
			let repair_note = if repaired > 0 { format!(" ({repaired} damaged)") } else { String::new() };
			println!("{lang}: {total} entries would be translated{plural_note}{repair_note}{style_note}");
		}
		return Ok(());
	}
	let final_content = if total == 0 {
		merged
	} else {
		let Some(client) = client else { unreachable!("client is always Some outside --dry-run") };
		match claude::language_name(&lang) {
			None => {
				println!("{lang}: no language name mapped, skipping ({total} entries need one)");
				merged
			}
			Some(language) => {
				let target = claude::Target { language, style: style.as_deref() };
				let (mut applied, annotated) = translate_singulars(client, &target, &candidates, context)?;
				let plural_done = match plural_forms(&merged) {
					// Without a usable Plural-Forms header there is no way to know how many
					// forms to ask for, and guessing at two would write a Russian file that is
					// wrong in a way gettext accepts silently.
					None if !plurals.is_empty() => {
						eprintln!("warning: {lang} has no usable Plural-Forms header, leaving its plural entries");
						0
					}
					None => 0,
					Some((nplurals, rule)) => {
						let translated = translate_plurals(client, &target, &plurals, context, nplurals, &rule)?;
						let done = translated.len();
						applied.extend(translated);
						done
					}
				};
				let singular_done = applied.len() - plural_done;
				let skipped = total - applied.len();
				doc.apply(&applied);
				print!("{lang} ({language}): translated {singular_done} entries");
				if plural_done > 0 {
					print!(", {plural_done} plural");
				}
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
	fn a_style_note_is_trimmed() {
		let content = "\n\n# Dutch\n\nAddress the reader as \"je\".\n\n\n";
		assert_eq!(parse_style_note(content), Some("# Dutch\n\nAddress the reader as \"je\".".to_string()));
	}

	#[test]
	fn a_blank_style_note_counts_as_absent() {
		assert_eq!(parse_style_note("  \n\n"), None);
	}

	#[test]
	fn the_dry_run_line_names_the_style_note_it_would_use() {
		assert_eq!(style_note_suffix("nl", true), ", with po/style/nl.md");
		assert_eq!(style_note_suffix("nl", false), "");
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
	fn plural_forms_reads_the_count_and_the_rule() {
		let po = "msgid \"\"\nmsgstr \"\"\n\"Plural-Forms: nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : 1);\\n\"\n";
		let (nplurals, rule) = plural_forms(po).unwrap();
		assert_eq!(nplurals, 3);
		assert!(rule.starts_with("Plural-Forms: nplurals=3;"), "got: {rule}");
		assert!(!rule.ends_with("\\n\""), "the header's line ending should not reach the prompt: {rule}");
	}

	// Without the header there is no way to know how many forms to ask for, and guessing at
	// two writes a Russian file that is wrong in a way gettext accepts silently.
	#[test]
	fn plural_forms_is_none_without_the_header() {
		assert!(plural_forms("msgid \"\"\nmsgstr \"\"\n").is_none());
	}

	#[test]
	fn the_header_entry_never_takes_a_note() {
		let pot = "#. TRANSLATORS: stray\nmsgid \"\"\nmsgstr \"\"\n\nmsgid \"Ready\"\nmsgstr \"\"\n";
		assert!(translator_comments(pot).is_empty());
	}

	/// A catalog of `(msgid, msgstr, fuzzy)` entries, as msgmerge leaves them: no `#|` lines.
	fn catalog(entries: &[(&str, &str, bool)]) -> PoDocument {
		let mut text = String::from("msgid \"\"\nmsgstr \"Content-Type: text/plain; charset=UTF-8\\n\"\n");
		for (msgid, msgstr, fuzzy) in entries {
			text.push('\n');
			if *fuzzy {
				text.push_str("#, fuzzy\n");
			}
			text.push_str(&format!("msgid \"{msgid}\"\nmsgstr \"{msgstr}\"\n"));
		}
		PoDocument::parse(&text)
	}

	fn copied(doc: &PoDocument) -> Vec<String> {
		let mut candidates = Vec::new();
		add_copied_entries(doc, &mut candidates);
		candidates.into_iter().map(|(_, msgid)| msgid).collect()
	}

	/// The case that shipped: a sleep timer preset given another preset's translation.
	#[test]
	fn a_translation_shared_by_strings_that_say_different_things_is_a_copy() {
		let doc = catalog(&[("1 minute", "1 minuto", true), ("10 minutes", "1 minuto", true)]);
		assert_eq!(vec!["1 minute", "10 minutes"], copied(&doc));
	}

	/// An accelerator, case or trailing punctuation is not a different meaning.
	#[test]
	fn the_same_words_sharing_a_translation_are_not_a_copy() {
		let doc = catalog(&[
			("&Close", "Cerrar", true),
			("Close", "Cerrar", true),
			("Open...", "Abrir", true),
			("Open", "Abrir", true),
		]);
		assert!(copied(&doc).is_empty());
	}

	/// A person put a non-fuzzy translation there, and a shared translation is not reason enough
	/// to overwrite it. Its fuzzy partner is still retranslated.
	#[test]
	fn a_human_translation_is_never_selected() {
		let doc = catalog(&[("Match Case", "Mayusculas", false), ("Batch OCR", "Mayusculas", true)]);
		assert_eq!(vec!["Batch OCR"], copied(&doc));
	}

	#[test]
	fn a_translation_no_other_string_shares_is_left_alone() {
		let doc = catalog(&[("Batch OCR", "OCR por lotes", true), ("Match Case", "Mayusculas", true)]);
		assert!(copied(&doc).is_empty());
	}
}
