use std::ops::Range;

/// A single `msgid`/`msgstr` entry, plus the exact line ranges in the source file needed
/// to patch it in place without disturbing anything else (header metadata, TRANSLATORS
/// comments, spacing, other entries).
pub struct PoEntryLoc {
	pub msgid: String,
	pub msgstr: String,
	pub is_fuzzy: bool,
	/// Range of `#,`/`#|` lines immediately preceding `msgid` (empty range at the msgid
	/// line itself when there's no existing flag/prev-msgid block to replace — inserting
	/// into an empty range is just an insert, not a removal).
	flag_block: Range<usize>,
	/// Range of the full `msgstr "..."` statement, including any continuation lines.
	msgstr_range: Range<usize>,
}

/// A parsed `.po` file, kept as its original lines so entries that aren't touched are
/// re-rendered byte-for-byte identical to the source.
pub struct PoDocument {
	lines: Vec<String>,
	pub entries: Vec<PoEntryLoc>,
}

impl PoDocument {
	#[must_use]
	pub fn parse(content: &str) -> Self {
		let lines: Vec<String> = content.lines().map(str::to_string).collect();
		let mut entries = Vec::new();
		let mut flag_line: Option<usize> = None;
		let mut prev_msgid_range: Option<(usize, usize)> = None;
		let mut i = 0;
		while i < lines.len() {
			let trimmed = lines[i].trim_start();
			if trimmed.starts_with('#') {
				if trimmed.starts_with("#,") {
					flag_line = Some(i);
				} else if trimmed.starts_with("#|") {
					prev_msgid_range = Some(prev_msgid_range.map_or((i, i + 1), |(start, _)| (start, i + 1)));
				}
				i += 1;
				continue;
			}
			if trimmed.is_empty() {
				flag_line = None;
				prev_msgid_range = None;
				i += 1;
				continue;
			}
			let Some(rest) = trimmed.strip_prefix("msgid ") else {
				i += 1;
				continue;
			};
			let msgid_start = i;
			let mut msgid = po_unescape(rest);
			i += 1;
			while i < lines.len() && lines[i].trim_start().starts_with('"') {
				msgid.push_str(&po_unescape(lines[i].trim_start()));
				i += 1;
			}
			if i < lines.len() && lines[i].trim_start().starts_with("msgstr ") {
				let msgstr_start = i;
				let msgstr_rest = lines[i].trim_start().strip_prefix("msgstr ").unwrap().to_string();
				let mut msgstr = po_unescape(&msgstr_rest);
				i += 1;
				while i < lines.len() && lines[i].trim_start().starts_with('"') {
					msgstr.push_str(&po_unescape(lines[i].trim_start()));
					i += 1;
				}
				let is_fuzzy = flag_line.is_some_and(|fl| lines[fl].contains("fuzzy"));
				let flag_block_start = match (flag_line, prev_msgid_range) {
					(Some(f), Some((p, _))) => f.min(p),
					(Some(f), None) => f,
					(None, Some((p, _))) => p,
					(None, None) => msgid_start,
				};
				entries.push(PoEntryLoc {
					msgid,
					msgstr,
					is_fuzzy,
					flag_block: flag_block_start..msgid_start,
					msgstr_range: msgstr_start..i,
				});
			}
			flag_line = None;
			prev_msgid_range = None;
		}
		Self { lines, entries }
	}

	/// Entries that still need a translation: blank `msgstr`, or flagged `#, fuzzy`
	/// (msgmerge marks an entry fuzzy when the source string changed but a similar old
	/// translation exists — that translation is stale, not just missing). The header
	/// entry (`msgid ""`) is never a candidate.
	pub fn needs_translation(&self) -> impl Iterator<Item = (usize, &str)> {
		self.entries
			.iter()
			.enumerate()
			.filter(|(_, e)| !e.msgid.is_empty() && (e.msgstr.is_empty() || e.is_fuzzy))
			.map(|(i, e)| (i, e.msgid.as_str()))
	}

	/// Applies a batch of `(entry index, translated text)` results: for each entry, the
	/// existing `#,`/`#|` block (if any) collapses to a plain `#, fuzzy` line and the
	/// `msgstr` is replaced. Everything else in the file is untouched. All ranges are
	/// computed against the original parse, so patches are applied bottom-up in one pass
	/// to keep earlier ranges valid.
	pub fn apply_all(&mut self, translations: &[(usize, String)]) {
		let mut ops: Vec<(Range<usize>, Vec<String>)> = Vec::new();
		for (idx, translated) in translations {
			let entry = &self.entries[*idx];
			ops.push((entry.msgstr_range.clone(), vec![format!("msgstr \"{}\"", po_escape(translated))]));
			ops.push((entry.flag_block.clone(), vec!["#, fuzzy".to_string()]));
		}
		ops.sort_by_key(|op| std::cmp::Reverse(op.0.start));
		for (range, replacement) in ops {
			self.lines.splice(range, replacement);
		}
	}

	#[must_use]
	pub fn render(&self) -> String {
		let mut out = self.lines.join("\n");
		out.push('\n');
		out
	}
}

fn po_unescape(s: &str) -> String {
	let s = s.trim();
	let Some(inner) = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')) else {
		return String::new();
	};
	let mut out = String::with_capacity(inner.len());
	let mut chars = inner.chars();
	while let Some(c) = chars.next() {
		if c == '\\' {
			match chars.next() {
				Some('n') => out.push('\n'),
				Some('t') => out.push('\t'),
				Some('"') => out.push('"'),
				Some('\\') | None => out.push('\\'),
				Some(other) => {
					out.push('\\');
					out.push(other);
				}
			}
		} else {
			out.push(c);
		}
	}
	out
}

fn po_escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for c in s.chars() {
		match c {
			'"' => out.push_str("\\\""),
			'\\' => out.push_str("\\\\"),
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			c => out.push(c),
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	fn find<'a>(doc: &'a PoDocument, msgid: &str) -> &'a PoEntryLoc {
		doc.entries.iter().find(|e| e.msgid == msgid).unwrap_or_else(|| panic!("no entry for {msgid:?}"))
	}

	#[test]
	fn header_entry_is_never_a_translation_candidate() {
		let doc = PoDocument::parse("msgid \"\"\nmsgstr \"\"\n\"Language: de\\n\"\n");
		assert_eq!(doc.needs_translation().count(), 0);
	}

	#[test]
	fn already_translated_entry_is_left_untouched() {
		let src = "msgid \"Cancel\"\nmsgstr \"Abbrechen\"\n";
		let doc = PoDocument::parse(src);
		assert_eq!(doc.needs_translation().count(), 0);
		assert_eq!(doc.render(), src);
	}

	#[test]
	fn blank_entry_with_no_flag_gets_a_fuzzy_line_inserted() {
		let src = "msgid \"Warning\"\nmsgstr \"\"\n";
		let mut doc = PoDocument::parse(src);
		let candidates: Vec<_> = doc.needs_translation().map(|(i, m)| (i, m.to_string())).collect();
		assert_eq!(candidates, vec![(0, "Warning".to_string())]);
		doc.apply_all(&[(0, "Warnung".to_string())]);
		assert_eq!(doc.render(), "#, fuzzy\nmsgid \"Warning\"\nmsgstr \"Warnung\"\n");
	}

	#[test]
	fn translators_comment_is_preserved_when_inserting_a_fuzzy_line() {
		let src = "#. TRANSLATORS: shown on hover\nmsgid \"Warning\"\nmsgstr \"\"\n";
		let mut doc = PoDocument::parse(src);
		doc.apply_all(&[(0, "Warnung".to_string())]);
		assert_eq!(doc.render(), "#. TRANSLATORS: shown on hover\n#, fuzzy\nmsgid \"Warning\"\nmsgstr \"Warnung\"\n");
	}

	#[test]
	fn fuzzy_entry_with_prev_msgid_comment_is_retranslated_and_comment_dropped() {
		let src = "#, fuzzy\n#| msgid \"No pages.\"\nmsgid \"No images.\"\nmsgstr \"Sem pagina.\"\n";
		let mut doc = PoDocument::parse(src);
		let entry = find(&doc, "No images.");
		assert!(entry.is_fuzzy);
		let idx = doc.entries.iter().position(|e| e.msgid == "No images.").unwrap();
		doc.apply_all(&[(idx, "Sem imagens.".to_string())]);
		assert_eq!(doc.render(), "#, fuzzy\nmsgid \"No images.\"\nmsgstr \"Sem imagens.\"\n");
	}

	#[test]
	fn multiline_msgid_is_decoded_and_can_be_translated() {
		let src = "msgid \"\"\n\"Are you sure you want to remove the selected document? This will also remove \"\n\"its reading position and bookmarks.\"\nmsgstr \"\"\n";
		let mut doc = PoDocument::parse(src);
		let entry = find(
			&doc,
			"Are you sure you want to remove the selected document? This will also remove its reading position and bookmarks.",
		);
		assert!(entry.msgstr.is_empty());
		let idx = 0;
		doc.apply_all(&[(idx, "Translated.".to_string())]);
		assert!(doc.render().contains("msgstr \"Translated.\""));
	}

	#[test]
	fn multiple_entries_patch_independently_in_one_pass() {
		let src = "msgid \"A\"\nmsgstr \"\"\n\nmsgid \"B\"\nmsgstr \"already\"\n\nmsgid \"C\"\nmsgstr \"\"\n";
		let mut doc = PoDocument::parse(src);
		let candidates: Vec<_> = doc.needs_translation().map(|(i, m)| (i, m.to_string())).collect();
		assert_eq!(candidates, vec![(0, "A".to_string()), (2, "C".to_string())]);
		doc.apply_all(&[(0, "A-translated".to_string()), (2, "C-translated".to_string())]);
		let rendered = doc.render();
		assert!(rendered.contains("msgid \"A\"\nmsgstr \"A-translated\""));
		assert!(rendered.contains("msgid \"B\"\nmsgstr \"already\""));
		assert!(rendered.contains("msgid \"C\"\nmsgstr \"C-translated\""));
	}

	#[test]
	fn round_trip_escapes_quotes_and_newlines_in_translated_text() {
		let src = "msgid \"Quote\"\nmsgstr \"\"\n";
		let mut doc = PoDocument::parse(src);
		doc.apply_all(&[(0, "She said \"hi\"\nline two".to_string())]);
		let rendered = doc.render();
		assert!(rendered.contains("msgstr \"She said \\\"hi\\\"\\nline two\""));
	}
}
