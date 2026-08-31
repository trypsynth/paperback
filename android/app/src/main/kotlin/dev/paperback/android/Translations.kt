package dev.paperback.android

import android.content.Context
import org.json.JSONObject
import java.util.Locale

object Translations {
	internal var map: HashMap<String, String> = HashMap()

	/**
	 * How [nt] picks a plural form for the loaded catalogue, from its own `Plural-Forms` header.
	 * Null until a catalogue is loaded, and for one whose header can't be read.
	 */
	internal var pluralRule: PluralRule? = null

	fun load(context: Context) {
		val match = bestLocaleMatch(assetLocaleTags(context, "translations", "", ".json"), Locale.getDefault())
			?: return
		try {
			val json = context.assets
				.open("translations/$match.json")
				.bufferedReader()
				.use { it.readText() }
			val obj = JSONObject(json)
			val loaded = HashMap<String, String>(obj.length())
			for (key in obj.keys()) {
				loaded[key] = obj.getString(key)
			}
			map = loaded
			// The po header comes through as the entry with the empty msgid, the same as it does
			// for desktop, so the target language's own rule ships with its translations.
			pluralRule = PluralRule.parse(loaded[""])
		} catch (_: Exception) {
			// Asset unreadable or malformed — fall back to English
		}
	}
}

/**
 * The locale tags of the assets in [dir] named [prefix] + tag + [suffix], e.g. "zh_CN" for
 * `translations/zh_CN.json`. Names not fitting that shape (`readmes/readme.html`, the English
 * original) are left out.
 */
fun assetLocaleTags(
	context: Context,
	dir: String,
	prefix: String,
	suffix: String
): List<String> =
	try {
		context.assets
			.list(dir)
			.orEmpty()
			.filter { it.startsWith(prefix) && it.endsWith(suffix) && it.length > prefix.length + suffix.length }
			.map { it.substring(prefix.length, it.length - suffix.length) }
	} catch (_: Exception) {
		emptyList()
	}

/**
 * The tag in [available] that best fits [locale], or null when none does and English should stand.
 *
 * Matched against the tags actually shipped rather than a list kept alongside them, because those
 * names come from the po files and neither reduce to the bare language code [Locale.getLanguage]
 * reports nor agree on country-code casing: the catalogue holds `zh_CN` and `pt_br`. A Chinese
 * device asks for "zh" and a Brazilian one for "pt", so both matched nothing and every reader in
 * those two languages saw English no matter how complete their translation was.
 *
 * A tag for the device's own country wins; otherwise any tag for the same language does, which is
 * what lets a zh_TW or pt_PT device fall to the one Chinese or Portuguese catalogue there is
 * instead of all the way back to English.
 */
fun bestLocaleMatch(
	available: List<String>,
	locale: Locale
): String? {
	val language = locale.language.lowercase()
	if (language.isEmpty()) {
		return null
	}
	val country = locale.country.lowercase()
	if (country.isNotEmpty()) {
		available.firstOrNull { it.equals("${language}_$country", ignoreCase = true) }?.let { return it }
	}
	return available.firstOrNull { it.substringBefore('_').equals(language, ignoreCase = true) }
}

/**
 * U+2063 INVISIBLE SEPARATOR: renders nothing and isn't spoken by screen readers, so it's silent
 * in the UI either way. Appended to an English source string only where [nt]'s "many" form would
 * otherwise be byte-identical to its "few" form (e.g. both are "{} seconds" in English, but
 * Bosnian needs distinct "sekunde" vs "sekundi") — since [Translations.map] is a flat
 * string-to-string table with no separate context field, two entries with the same English text
 * can only be told apart by making that text itself differ, invisibly. [t] strips it from the
 * untranslated fallback so it never leaks into English UI; a provided translation is used as-is
 * since translators write the translated text only, without the marker.
 *
 * IMPORTANT for anyone editing translatable strings: don't strip this character if you see it —
 * it's deliberate, not stray whitespace.
 */
private const val PLURAL_MANY_MARKER = "⁣"

fun t(str: String): String = Translations.map[str] ?: str.removeSuffix(PLURAL_MANY_MARKER)

/** The marker a translatable string uses where an argument goes. */
private const val PLACEHOLDER = "{}"

/**
 * Translates [str], then substitutes each "{}" placeholder in order with the given [args].
 *
 * Done as one left-to-right scan rather than a `replaceFirst` per argument, so that text an
 * argument brings with it is never itself substituted into: a file name or search term
 * containing "{}" would otherwise stay the leftmost placeholder and swallow the argument
 * meant for the one after it. Spare placeholders and spare arguments are both left alone.
 */
fun t(
	str: String,
	vararg args: String
): String {
	val translated = t(str)
	if (args.isEmpty()) {
		return translated
	}
	val result = StringBuilder(translated.length)
	var index = 0
	var next = 0
	while (next < args.size) {
		val placeholder = translated.indexOf(PLACEHOLDER, index)
		if (placeholder == -1) {
			break
		}
		result.append(translated, index, placeholder).append(args[next])
		index = placeholder + PLACEHOLDER.length
		next++
	}
	result.append(translated, index, translated.length)
	return result.toString()
}

/**
 * A language's gettext plural rule: how many forms it has, and which one a count takes.
 *
 * Read from the catalogue's own `Plural-Forms` header rather than hardcoded, because the rule
 * differs between languages that all want three forms: Bosnian, Serbian and Croatian give 21 and
 * 31 the same form as 1, while Polish gives the singular to 1 alone, so a single hardcoded rule
 * cannot serve both. Desktop gets this from patois at runtime; this is the mobile equivalent.
 *
 * Only the shapes gettext's own catalogues use are understood — a chain of `n`-comparisons and
 * `?:` — which covers every language Paperback ships. Anything else parses to null and [nt]
 * falls back to its previous behaviour rather than guessing.
 */
internal class PluralRule private constructor(
	private val nplurals: Int,
	private val expression: String
) {
	/** The 0-based form index for [count], or null when the expression doesn't evaluate. */
	fun formIndex(count: Long): Int? {
		val value = evalTernary(expression, count) ?: return null
		return if (value in 0 until nplurals) value else null
	}

	val forms: Int
		get() = nplurals

	companion object {
		/**
		 * Parses the `Plural-Forms: nplurals=N; plural=EXPR;` line out of a po header, or null
		 * when the header is missing, has no such line, or states a form count below one.
		 */
		fun parse(header: String?): PluralRule? {
			val line = header
				?.lineSequence()
				?.firstOrNull { it.startsWith("Plural-Forms:") }
				?: return null
			val nplurals = line.substringAfter("nplurals=", "")
				.takeWhile { it.isDigit() }
				.toIntOrNull()
				?.takeIf { it > 0 }
				?: return null
			val expression = line.substringAfter("plural=", "")
				.substringBefore(';')
				.trim()
				.takeIf { it.isNotEmpty() }
				?: return null
			return PluralRule(nplurals, expression)
		}

		/**
		 * Evaluates a gettext plural expression for [n], or null on anything unrecognized.
		 *
		 * Deliberately narrow: `?:`, `||`, `&&`, the comparisons, `%` and integer literals, which
		 * is the whole of what gettext plural rules use. Returning null on the unexpected keeps a
		 * malformed header from silently selecting form 0 for every count.
		 */
		private fun evalTernary(
			expr: String,
			n: Long
		): Int? {
			val text = expr.trim().removeSurroundingParens()
			val question = text.indexOfTopLevel('?')
			if (question == -1) {
				return text.toIntOrNull() ?: evalBoolean(text, n)?.let { if (it) 1 else 0 }
			}
			val colon = text.indexOfTopLevel(':', from = question + 1).takeIf { it != -1 } ?: return null
			val condition = evalBoolean(text.substring(0, question), n) ?: return null
			val branch = if (condition) {
				text.substring(question + 1, colon)
			} else {
				text.substring(colon + 1)
			}
			return evalTernary(branch, n)
		}

		private fun evalBoolean(
			expr: String,
			n: Long
		): Boolean? {
			val text = expr.trim().removeSurroundingParens()
			text.splitTopLevel("||")?.let { (left, right) ->
				val l = evalBoolean(left, n) ?: return null
				if (l) {
					return true
				}
				return evalBoolean(right, n)
			}
			text.splitTopLevel("&&")?.let { (left, right) ->
				val l = evalBoolean(left, n) ?: return null
				if (!l) {
					return false
				}
				return evalBoolean(right, n)
			}
			for (op in listOf("==", "!=", ">=", "<=", ">", "<")) {
				val at = text.indexOfTopLevel(op) ?: continue
				val left = evalArithmetic(text.substring(0, at), n) ?: return null
				val right = evalArithmetic(text.substring(at + op.length), n) ?: return null
				return when (op) {
					"==" -> left == right
					"!=" -> left != right
					">=" -> left >= right
					"<=" -> left <= right
					">" -> left > right
					else -> left < right
				}
			}
			return null
		}

		private fun evalArithmetic(
			expr: String,
			n: Long
		): Long? {
			val text = expr.trim().removeSurroundingParens()
			val modulo = text.indexOfTopLevel('%')
			if (modulo != -1) {
				val left = evalArithmetic(text.substring(0, modulo), n) ?: return null
				val right = evalArithmetic(text.substring(modulo + 1), n) ?: return null
				if (right == 0L) {
					return null
				}
				return left % right
			}
			if (text == "n") {
				return n
			}
			return text.toLongOrNull()
		}

		private fun String.removeSurroundingParens(): String {
			var text = trim()
			while (text.length > 1 && text.startsWith("(") && text.endsWith(")") && text.outerParenSpansAll()) {
				text = text.substring(1, text.length - 1).trim()
			}
			return text
		}

		/**
		 * Whether the parenthesis opening this string is the one the final character closes, so
		 * that stripping both keeps the expression intact. False for `(a) && (b)`, whose outer
		 * parentheses look enclosing but are two separate groups.
		 */
		private fun String.outerParenSpansAll(): Boolean {
			var depth = 0
			for (i in indices) {
				when (this[i]) {
					'(' -> depth++
					')' -> {
						depth--
						if (depth == 0) {
							return i == length - 1
						}
					}
				}
			}
			return false
		}

		/** The index of [char] outside any parentheses, or -1. */
		private fun String.indexOfTopLevel(
			char: Char,
			from: Int = 0
		): Int {
			var depth = 0
			for (i in from until length) {
				val c = this[i]
				if (c == char && depth == 0) {
					return i
				}
				when (c) {
					'(' -> depth++
					')' -> depth--
				}
			}
			return -1
		}

		private fun String.indexOfTopLevel(token: String): Int? {
			var depth = 0
			var i = 0
			while (i <= length - token.length) {
				when (this[i]) {
					'(' -> depth++
					')' -> depth--
				}
				if (depth == 0 && startsWith(token, i)) {
					// "<" must not match inside "<=", nor "=" inside "==".
					val nextIsEquals = token.length == 1 && i + 1 < length && this[i + 1] == '='
					if (!nextIsEquals) {
						return i
					}
				}
				i++
			}
			return null
		}

		private fun String.splitTopLevel(token: String): Pair<String, String>? {
			val at = indexOfTopLevel(token) ?: return null
			return substring(0, at) to substring(at + token.length)
		}
	}
}

/**
 * Selects among three already-translated forms for languages (e.g. Bosnian, Serbian, Croatian)
 * whose grammar needs three: [one] for a count ending in 1, except one ending in 11 (1, 21, 31,
 * ...); [few] for a count ending in 2-4, except one ending in 12-14 (2, 3, 4, 22, 23, 24, ...);
 * and [many] for everything else (0, 5-20, 25-30, ...). Desktop gets this for free from the
 * target language's own `Plural-Forms` rule via patois; mobile has no such runtime, so the rule
 * is read from the loaded catalogue's header, which ships alongside its translations.
 *
 * The hardcoded fallback below is that same Bosnian/Serbian rule, used only when no catalogue is
 * loaded or its header couldn't be parsed. It is wrong for some languages — Polish gives the
 * singular to 1 alone, so it would read "21 sekunda" where the language wants "21 sekund" — which
 * is why the header is preferred whenever it is there.
 *
 * A two-form language selects [one] or [few], and a one-form language always [one], matching how
 * `msgstr[0]` and `msgstr[1]` are filled in its catalogue.
 *
 * Callers translate each form themselves with [t] *before* calling this — `nt(t("1 book"),
 * t("{} books"), t("{} books"), count)`, never `nt("1 book", "{} books", "{} books", count)` —
 * because the pot scanner only recognizes plain `t("...")` calls; a bare string literal handed
 * straight to `nt()` would never be extracted and could never be translated.
 */
fun nt(
	one: String,
	few: String,
	many: String,
	count: Long
): String {
	val forms = listOf(one, few, many)
	Translations.pluralRule?.let { rule ->
		rule.formIndex(count)?.let { index ->
			return forms[index.coerceAtMost(forms.size - 1)]
		}
	}
	val mod10 = count % 10
	val mod100 = count % 100
	return when {
		mod10 == 1L && mod100 != 11L -> one
		mod10 in 2L..4L && mod100 !in 12L..14L -> few
		else -> many
	}
}
