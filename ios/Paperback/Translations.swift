import Foundation

/// U+2063 INVISIBLE SEPARATOR: renders nothing and isn't spoken by VoiceOver, so it's silent in
/// the UI either way. Appended to an English source string only where `nt`'s "many" form would
/// otherwise be byte-identical to its "few" form (e.g. both are "{} seconds" in English, but
/// Bosnian needs distinct "sekunde" vs "sekundi") — since translations are keyed by the English
/// text itself with no separate context field, two entries with the same English text can only
/// be told apart by making that text itself differ, invisibly. `t(_:)` strips it from the
/// untranslated fallback so it never leaks into English UI; a provided translation is used as-is
/// since translators write the translated text only, without the marker.
///
/// IMPORTANT for anyone editing translatable strings: don't strip this character if you see it —
/// it's deliberate, not stray whitespace.
private let pluralManyMarker = "⁣"

func t(_ key: String) -> String {
	let value = NSLocalizedString(key, comment: "")
	guard value == key, key.hasSuffix(pluralManyMarker) else { return value }
	return String(key.dropLast(pluralManyMarker.count))
}

/// Selects among three already-translated forms for languages (e.g. Bosnian, Serbian, Croatian)
/// whose grammar needs three: `one` for a count ending in 1, except one ending in 11 (1, 21, 31,
/// ...); `few` for a count ending in 2-4, except one ending in 12-14 (2, 3, 4, 22, 23, 24, ...);
/// and `many` for everything else (0, 5-20, 25-30, ...). Desktop gets this for free from the
/// target language's own `Plural-Forms` rule via patois; mobile has no such runtime, so this
/// hardcodes the one three-form rule Paperback ships a mobile translation for.
///
/// Callers translate each form themselves with `t(_:)` *before* calling this — `nt(t("1 book"),
/// t("{} books"), t("{} books"), count)`, never `nt("1 book", "{} books", "{} books", count)` —
/// because the pot scanner only recognizes plain `t("...")` calls; a bare string literal handed
/// straight to `nt(_:_:_:_:)` would never be extracted and could never be translated.
func nt(_ one: String, _ few: String, _ many: String, _ count: Int) -> String {
	let mod10 = count % 10
	let mod100 = count % 100
	if mod10 == 1 && mod100 != 11 {
		return one
	}
	if (2...4).contains(mod10) && !(12...14).contains(mod100) {
		return few
	}
	return many
}
