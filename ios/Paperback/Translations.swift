import Foundation

/// U+2063, invisible and unspoken by VoiceOver: translations are keyed by English text, so it makes a "many" form differ from an identical "few" form (Bosnian "sekunde" vs "sekundi").
/// Deliberate, not stray whitespace: don't strip it from translatable strings.
private let pluralManyMarker = "⁣"

func t(_ key: String) -> String {
	let value = NSLocalizedString(key, comment: "")
	guard value == key, key.hasSuffix(pluralManyMarker) else { return value }
	return String(key.dropLast(pluralManyMarker.count))
}

/// Hardcodes the one three-form plural rule mobile ships (Bosnian, Serbian, Croatian), since unlike desktop's patois there is no `Plural-Forms` runtime here.
/// Callers must wrap each form in `t(...)`: the pot scanner only extracts plain `t("...")` calls, so a bare literal here is never translated.
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
