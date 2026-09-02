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
	let mut sections: Vec<String> = Vec::new();
	let mut current = String::new();
	for line in markdown.lines() {
		if line.starts_with("## ") && !current.trim().is_empty() {
			sections.push(current.trim_end().to_string());
			current = String::new();
		}
		current.push_str(line);
		current.push('\n');
	}
	if !current.trim().is_empty() {
		sections.push(current.trim_end().to_string());
	}
	sections
}

/// Splits Markdown into chunks of at most `limit` characters, breaking only at `##` headings so
/// a chunk is always a whole number of sections and the model never sees a half-open construct.
/// A single section longer than the limit is left whole rather than cut mid-paragraph.
pub(super) fn split_markdown(markdown: &str, limit: usize) -> Vec<String> {
	let sections = split_sections(markdown);
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
