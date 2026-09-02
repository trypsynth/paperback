//! Whether a translation came back sound: the placeholders, the accelerator and the shortcut
//! suffix the English had, and every form a plural needs.

use super::claude::PluralPhrase;

/// Whether an existing translation is provably damaged: it dropped a placeholder, an
/// accelerator, or a shortcut suffix that the English has.
///
/// The same rules [`check`] applies to fresh output, pointed at what is already in the catalog.
/// Entries that fail this were never translated as such - they are what `msgmerge` copied in
/// from a similar string before anything checked - and since nothing in the normal flow will
/// ever revisit them, this is what marks them for re-translation.
#[must_use]
pub fn is_damaged(source: &str, translated: &str) -> bool {
	!translated.is_empty() && check(source, translated).is_none()
}

/// The plural counterpart of [`check`]: every form has to be present and carry the English's
/// placeholders, or the whole set is rejected.
///
/// All-or-nothing on purpose. A partly-good set written into the file leaves some `msgstr[N]`
/// filled and others blank, which reads as translated to every tool that looks at it while
/// gettext quietly falls back to the English for the missing counts.
pub(super) fn check_plural(phrase: &PluralPhrase, forms: &[String], nplurals: usize) -> Option<Vec<String>> {
	if forms.len() != nplurals || forms.iter().any(|f| f.trim().is_empty()) {
		return None;
	}
	// Checked against the English plural rather than the singular: the placeholder belongs to
	// the countable message as a whole, and `1 document.` / `%d documents.` legitimately
	// differ in whether the singular spells the number out.
	let expected = placeholder_counts(&phrase.plural);
	forms.iter().all(|f| placeholder_counts(f) == expected).then(|| forms.to_vec())
}

/// Accepts a translation only if it kept the parts that aren't prose, returning `None` when it
/// didn't so the caller leaves the entry for the next run instead of writing damage into the
/// catalog.
pub(super) fn check(source: &str, translated: &str) -> Option<String> {
	if translated.trim().is_empty() {
		return None;
	}
	if placeholder_counts(source) != placeholder_counts(translated) {
		return None;
	}
	if accelerator_count(source) != accelerator_count(translated) {
		return None;
	}
	if shortcut_suffix(source) != shortcut_suffix(translated) {
		return None;
	}
	Some(translated.to_string())
}

/// Counts of `%s`/`%d`/`{}`, the placeholder styles this project's strings use.
fn placeholder_counts(s: &str) -> (usize, usize, usize) {
	(s.matches("%s").count(), s.matches("%d").count(), s.matches("{}").count())
}

/// How many accelerator markers a string has: an `&` directly before an alphanumeric.
///
/// `&&` is an escaped literal ampersand rather than a marker, and is skipped along with the
/// character after it so `A && B` counts as none rather than one.
fn accelerator_count(s: &str) -> usize {
	let chars: Vec<char> = s.chars().collect();
	let mut count = 0;
	let mut i = 0;
	while i < chars.len() {
		if chars[i] == '&' {
			match chars.get(i + 1) {
				Some('&') => i += 2,
				Some(c) if c.is_alphanumeric() => {
					count += 1;
					i += 2;
				}
				_ => i += 1,
			}
			continue;
		}
		i += 1;
	}
	count
}

/// The `\tCtrl+O` shortcut suffix, which has to come back byte for byte. `None` when the string
/// has no tab, so strings without a shortcut compare equal to each other.
fn shortcut_suffix(s: &str) -> Option<&str> {
	s.split_once('\t').map(|(_, suffix)| suffix)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn placeholder_counts_catch_a_dropped_token() {
		assert_eq!(placeholder_counts("%s Heading level %d"), placeholder_counts("Ebene %d von %s"));
		assert_ne!(
			placeholder_counts("Remove the {} selected documents?"),
			placeholder_counts("Sollen die gewahlten Dokumente entfernt werden?")
		);
	}

	// The failure this backend exists to stop: `E&xit` came back as a Russian word with no
	// accelerator at all, 216 times in one locale.
	#[test]
	fn a_dropped_accelerator_is_rejected() {
		assert_eq!(check("E&xit", "Выход"), None);
		assert_eq!(check("E&xit", "В&ыход"), Some("В&ыход".to_string()));
	}

	#[test]
	fn an_invented_accelerator_is_rejected() {
		assert_eq!(check("Ready", "&Listo"), None);
		assert_eq!(check("Ready", "Listo"), Some("Listo".to_string()));
	}

	#[test]
	fn an_escaped_ampersand_is_not_an_accelerator() {
		assert_eq!(accelerator_count("Search && Replace"), 0);
		assert_eq!(accelerator_count("&Search && Replace"), 1);
		assert_eq!(check("Search && Replace", "Buscar && Reemplazar"), Some("Buscar && Reemplazar".to_string()));
	}

	#[test]
	fn the_accelerator_may_move_to_a_different_letter() {
		// Which letter it lands on is the model's call, since it has to be a letter the
		// translation actually contains. Only the count is enforced.
		assert_eq!(check("&Open", "&Abrir"), Some("&Abrir".to_string()));
		assert_eq!(check("&Open", "A&brir"), Some("A&brir".to_string()));
	}

	#[test]
	fn a_shortcut_suffix_must_come_back_untouched() {
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar\tCtrl+C"), Some("&Copiar\tCtrl+C".to_string()));
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar\tCtrl+D"), None, "a rewritten key combination");
		assert_eq!(check("&Copy\tCtrl+C", "&Copiar"), None, "a dropped shortcut");
	}

	#[test]
	fn an_empty_translation_is_rejected() {
		assert_eq!(check("Ready", "   "), None);
	}

	// Real entries from the catalog, each one something msgmerge copied in from a similar
	// string before anything checked.
	#[test]
	fn damage_already_in_the_catalog_is_recognised() {
		assert!(is_damaged("&Status:", "Estado"), "dropped accelerator");
		assert!(is_damaged("Choose &Background Color...", "Seleccionar color de fondo..."), "dropped accelerator");
		assert!(is_damaged("{} minutes", "minutos"), "dropped placeholder");
		// An *extra* placeholder is the dangerous direction: nothing fills it.
		assert!(is_damaged("Page %d", "Página %d: %s"), "invented placeholder");
		assert!(is_damaged("&Copy\tCtrl+C", "&Copiar\tCtrl+D"), "rewritten shortcut");
	}

	#[test]
	fn a_sound_translation_is_not_flagged_as_damaged() {
		assert!(!is_damaged("E&xit", "В&ыход"));
		assert!(!is_damaged("Ready", "Listo"));
		assert!(!is_damaged("Page %d", "Página %d"));
		assert!(!is_damaged("&Copy\tCtrl+C", "&Copiar\tCtrl+C"));
		// An untranslated entry is the normal flow's job, not the repair pass's.
		assert!(!is_damaged("Ready", ""));
	}

	fn plural(singular: &str, plural: &str) -> PluralPhrase {
		PluralPhrase { singular: singular.to_string(), plural: plural.to_string(), context: None }
	}

	#[test]
	fn a_complete_plural_set_is_accepted() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string(), "%d документов.".to_string()];
		assert_eq!(check_plural(&p, &forms, 3), Some(forms.to_vec()));
	}

	// Russian needs three. Two written into a three-form entry leaves msgstr[2] blank, which
	// looks translated to every tool that inspects the file while gettext falls back to the
	// English for those counts.
	#[test]
	fn a_short_plural_set_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string()];
		assert_eq!(check_plural(&p, &forms, 3), None);
	}

	#[test]
	fn a_plural_set_with_a_blank_form_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["a".to_string(), "  ".to_string()];
		assert_eq!(check_plural(&p, &forms, 2), None);
	}

	// The count is substituted into the placeholder, so a form without it renders a sentence
	// with no number in it.
	#[test]
	fn a_plural_form_that_dropped_the_placeholder_is_rejected() {
		let p = plural("%d document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "документа.".to_string()];
		assert_eq!(check_plural(&p, &forms, 2), None);
	}

	// Checked against the English plural, not the singular: `1 document.` spells the number
	// out and has no placeholder, which would reject every correct translation.
	#[test]
	fn the_placeholder_check_uses_the_english_plural() {
		let p = plural("1 document.", "%d documents.");
		let forms = ["%d документ.".to_string(), "%d документа.".to_string()];
		assert!(check_plural(&p, &forms, 2).is_some());
	}
}
