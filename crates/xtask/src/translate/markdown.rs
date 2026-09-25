//! Handling Markdown around a translation request: splitting a document into sections small
//! enough to send, and putting the code spans back exactly as they were.

/// Puts the source's inline code spans back, so a `` `Alt+Left` `` stays `Alt+Left`.
///
/// The prompt says code spans are verbatim, and the model mostly obeys, but the exceptions are
/// the ones that matter: `Alt+Left` came back as `Alt+Gauche` in French, `Alt+Links` in Dutch
/// and `Alt+Влево` in Russian. Those read as translations and are not keys anyone can press.
/// The rule is absolute, so it is enforced here rather than left to the prompt.
///
/// Only runs when the span counts match. A different count means the model added or dropped
/// one, so the nth span in the translation isn't the nth in the source and positional
/// restoration would put text in the wrong place; the translation is then left exactly as it
/// came back, for a human to look at.
pub(super) fn restore_code_spans(source: &str, translated: &str) -> String {
	let source_spans = code_spans(source);
	let translated_spans = code_spans(translated);
	if source_spans.len() != translated_spans.len() {
		return translated.to_string();
	}
	let mut out = String::with_capacity(translated.len());
	let mut last = 0;
	for ((range, _), (_, original)) in translated_spans.into_iter().zip(source_spans) {
		out.push_str(&translated[last..range.start]);
		out.push_str(original);
		last = range.end;
	}
	out.push_str(&translated[last..]);
	out
}

/// The inner text of every single-backtick inline code span, with its byte range.
///
/// Deliberately single-line: a span never spans a newline in Markdown, and stopping at one
/// keeps a stray unmatched backtick from swallowing the rest of the document.
fn code_spans(text: &str) -> Vec<(std::ops::Range<usize>, &str)> {
	let bytes = text.as_bytes();
	let mut out = Vec::new();
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] != b'`' {
			i += 1;
			continue;
		}
		let start = i + 1;
		let mut j = start;
		while j < bytes.len() && bytes[j] != b'`' && bytes[j] != b'\n' {
			j += 1;
		}
		if j < bytes.len() && bytes[j] == b'`' && j > start {
			out.push((start..j, &text[start..j]));
			i = j + 1;
		} else {
			i += 1;
		}
	}
	out
}

/// Splits Markdown at `##` headings, so every piece is a whole section.
///
/// The readme sync hashes and re-translates these, and [`split_markdown`] packs these into
/// request-sized chunks. They have to be the same unit: a section whose text has not changed
/// must hash the same as the one translated last time, which only holds if both are cut the
/// same way.
pub(super) fn split_sections(markdown: &str) -> Vec<String> {
	split_at_headings(markdown, 2)
}

/// Splits Markdown at the headings of one level, each piece starting at one of them.
fn split_at_headings(markdown: &str, level: usize) -> Vec<String> {
	let marker = format!("{} ", "#".repeat(level));
	let mut pieces: Vec<String> = Vec::new();
	let mut current = String::new();
	for line in markdown.lines() {
		if line.starts_with(&marker) && !current.trim().is_empty() {
			pieces.push(current.trim_end().to_string());
			current = String::new();
		}
		current.push_str(line);
		current.push('\n');
	}
	if !current.trim().is_empty() {
		pieces.push(current.trim_end().to_string());
	}
	pieces
}

/// The deepest heading level Markdown has.
const MAX_HEADING_LEVEL: usize = 6;

/// What a translated section has to keep from its English source: the same headings, at the same
/// levels, in the same order, and the same number of list items.
///
/// The model is asked for one document and returns another, and nothing downstream can tell the
/// difference between prose it rendered differently and prose it left out. It does leave things
/// out: a German run dropped the whole "Currently supported file types" section and the
/// "Supported languages" one, and cut a paragraph off mid-sentence, and the result was written
/// to `doc/readme-de.md` and opened as a pull request with nothing complaining. Structure is the
/// part of a translation that must not change, so it is the part worth checking.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Outline {
	/// One entry per heading, holding its level. Not the text: that is translated.
	headings: Vec<usize>,
	list_items: usize,
	fences: usize,
}

/// Why a translation does not match its source, or `None` when it does.
pub(super) fn structure_mismatch(source: &str, translated: &str) -> Option<String> {
	if translated.trim().is_empty() {
		return Some("the translation is empty".to_string());
	}
	let want = outline(source);
	let got = outline(translated);
	if want == got {
		return None;
	}
	if want.headings != got.headings {
		return Some(format!("headings are {:?} in English and {:?} in the translation", want.headings, got.headings));
	}
	if want.list_items != got.list_items {
		return Some(format!("{} list items in English and {} in the translation", want.list_items, got.list_items));
	}
	Some(format!("{} code fences in English and {} in the translation", want.fences, got.fences))
}

fn outline(markdown: &str) -> Outline {
	let mut headings = Vec::new();
	let mut list_items = 0;
	let mut fences = 0;
	let mut in_fence = false;
	for line in markdown.lines() {
		let trimmed = line.trim_start();
		if trimmed.starts_with("```") {
			fences += 1;
			in_fence = !in_fence;
			continue;
		}
		if in_fence {
			continue;
		}
		if let Some(level) = heading_level(trimmed) {
			headings.push(level);
		} else if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
			list_items += 1;
		}
	}
	Outline { headings, list_items, fences }
}

/// A chunk named by its first heading, so an error says which part of the readme failed.
pub(super) fn chunk_name(markdown: &str) -> String {
	for line in markdown.lines() {
		let trimmed = line.trim_start();
		if heading_level(trimmed).is_some() {
			return format!("\"{}\"", trimmed.trim_start_matches('#').trim());
		}
	}
	"a readme section with no heading".to_string()
}

/// The level of an ATX heading, or `None` for any other line. A run of `#` counts only when a
/// space follows it, which is what Markdown requires and what keeps a `#` inside prose out.
fn heading_level(line: &str) -> Option<usize> {
	let hashes = line.bytes().take_while(|b| *b == b'#').count();
	(hashes > 0 && hashes <= MAX_HEADING_LEVEL && line.as_bytes().get(hashes) == Some(&b' ')).then_some(hashes)
}

/// A section cut at ever deeper headings until each piece fits in `limit`, or there are no
/// deeper headings left to cut at.
fn split_oversized(section: String, limit: usize, level: usize) -> Vec<String> {
	if section.len() <= limit || level > MAX_HEADING_LEVEL {
		return vec![section];
	}
	split_at_headings(&section, level).into_iter().flat_map(|piece| split_oversized(piece, limit, level + 1)).collect()
}

/// Splits Markdown into chunks of at most `limit` characters, breaking only at headings so the
/// model never sees a half-open construct. A section longer than the limit is cut at its deeper
/// headings: the changelog is one `##` section, and whole it is more than one response can hold.
/// A piece with no heading left to cut at is left whole rather than cut mid-paragraph.
pub(super) fn split_markdown(markdown: &str, limit: usize) -> Vec<String> {
	let sections = split_sections(markdown).into_iter().flat_map(|section| split_oversized(section, limit, 3));
	let mut chunks: Vec<String> = Vec::new();
	for section in sections {
		match chunks.last_mut() {
			Some(last) if last.len() + section.len() + 2 <= limit => {
				last.push_str("\n\n");
				last.push_str(&section);
			}
			_ => chunks.push(section),
		}
	}
	chunks
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_translation_that_kept_every_heading_and_bullet_passes() {
		let source = "## One\n\n* a\n* b\n\n### Two\n\nText.\n";
		let translated = "## Eins\n\n* a\n* b\n\n### Zwei\n\nText.\n";
		assert_eq!(structure_mismatch(source, translated), None);
	}

	/// What actually happened to German: two sections went missing from the response.
	#[test]
	fn a_translation_missing_a_section_is_rejected() {
		let source = "## One\n\nText.\n\n## Two\n\nText.\n\n## Three\n\nText.\n";
		let translated = "## Eins\n\nText.\n\n## Drei\n\nText.\n";
		assert!(structure_mismatch(source, translated).is_some_and(|why| why.contains("headings")));
	}

	#[test]
	fn a_translation_missing_list_items_is_rejected() {
		let source = "## Formats\n\n* EPUB\n* PDF\n* RTF\n";
		let translated = "## Formate\n\n* EPUB\n* PDF\n";
		assert!(structure_mismatch(source, translated).is_some_and(|why| why.contains("list items")));
	}

	#[test]
	fn an_empty_translation_is_rejected() {
		assert!(structure_mismatch("## One\n\nText.\n", "   ").is_some());
	}

	/// A `#` and a `*` inside a fenced block are content, not structure.
	#[test]
	fn fenced_blocks_do_not_count_as_headings_or_bullets() {
		let source = "## One\n\n```\n# not a heading\n* not a bullet\n```\n";
		let translated = "## Eins\n\n```\n# not a heading\n* not a bullet\n```\n";
		assert_eq!(structure_mismatch(source, translated), None);
	}

	#[test]
	fn a_hash_without_a_space_is_not_a_heading() {
		let source = "## One\n\nIssue #42 is fixed.\n";
		let translated = "## Eins\n\nProblem #42 ist behoben.\n";
		assert_eq!(structure_mismatch(source, translated), None);
	}

	#[test]
	fn markdown_splits_on_section_headings() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.\n\n## Two\n\nBody two.\n";
		let chunks = split_markdown(doc, 30);
		assert_eq!(chunks.len(), 3, "each section should be its own chunk at this limit");
		assert!(chunks[1].starts_with("## One"));
		assert!(chunks[2].starts_with("## Two"));
	}

	#[test]
	fn markdown_sections_pack_together_under_the_limit() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.\n\n## Two\n\nBody two.\n";
		let chunks = split_markdown(doc, 10_000);
		assert_eq!(chunks.len(), 1, "the whole document fits in one chunk");
		assert_eq!(chunks[0].trim(), doc.trim());
	}

	#[test]
	fn an_oversized_section_is_cut_at_its_deeper_headings() {
		let doc = "## Changelog\n\n### Version 2\n\n* Two.\n\n### Version 1\n\n#### Added\n\n* One.\n\n#### Fixed\n\n* Won.\n";
		let chunks = split_markdown(doc, 30);
		assert_eq!(chunks.len(), 5, "got {chunks:?}");
		assert!(chunks[0].starts_with("## Changelog"));
		assert!(chunks[1].starts_with("### Version 2"));
		assert!(chunks[2].starts_with("### Version 1"));
		assert!(chunks[3].starts_with("#### Added"));
		assert!(chunks[4].starts_with("#### Fixed"));
		assert_eq!(chunks.join("\n\n"), doc.trim_end());
	}

	// The real case: French came back with `Alt+Gauche` where the source said `Alt+Left`.
	#[test]
	fn a_translated_key_name_is_put_back() {
		let source = "Press `Alt+Left` to go back, or `Ctrl+Space` to play.";
		let translated = "Appuyez sur `Alt+Gauche` pour revenir, ou `Ctrl+Espace` pour lire.";
		assert_eq!(
			restore_code_spans(source, translated),
			"Appuyez sur `Alt+Left` pour revenir, ou `Ctrl+Space` pour lire."
		);
	}

	#[test]
	fn prose_around_a_restored_span_is_left_alone() {
		let source = "The `readme.md` file.";
		let translated = "Het `readme.md` bestand.";
		assert_eq!(restore_code_spans(source, translated), translated);
	}

	// A changed count means the nth span in the translation is no longer the nth in the
	// source, so restoring by position would drop text into the wrong place.
	#[test]
	fn a_mismatched_span_count_leaves_the_translation_untouched() {
		let source = "Press `Alt+Left` then `Ctrl+C`.";
		let translated = "Appuyez sur `Alt+Gauche`.";
		assert_eq!(restore_code_spans(source, translated), translated);
	}

	#[test]
	fn code_spans_do_not_run_past_a_newline() {
		// A stray unmatched backtick must not swallow the rest of the document.
		let spans = code_spans("a ` stray\nand `real` one");
		assert_eq!(spans.len(), 1);
		assert_eq!(spans[0].1, "real");
	}

	#[test]
	fn an_empty_span_is_not_a_span() {
		assert!(code_spans("nothing `` here").is_empty());
	}

	#[test]
	fn restoring_handles_multibyte_text_around_the_spans() {
		let source = "Press `Alt+Left` now.";
		let translated = "Нажмите `Alt+Влево` сейчас.";
		assert_eq!(restore_code_spans(source, translated), "Нажмите `Alt+Left` сейчас.");
	}

	#[test]
	fn markdown_round_trips_when_nothing_needs_splitting() {
		let doc = "# Title\n\nIntro.\n\n## One\n\nBody one.";
		assert_eq!(split_markdown(doc, 10_000).join("\n\n"), doc);
	}
}
