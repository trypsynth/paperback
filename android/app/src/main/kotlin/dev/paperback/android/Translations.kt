package dev.paperback.android

import android.content.Context
import org.json.JSONObject
import java.util.Locale

object Translations {
	internal var map: HashMap<String, String> = HashMap()

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
			// The po header is the entry with the empty msgid.
			pluralRule = PluralRule.parse(loaded[""])
		} catch (_: Exception) {
			// Asset unreadable or malformed — fall back to English
		}
	}
}

fun assetLocaleTags(context: Context, dir: String, prefix: String, suffix: String): List<String> = try {
	context.assets
		.list(dir)
		.orEmpty()
		.filter { it.startsWith(prefix) && it.endsWith(suffix) && it.length > prefix.length + suffix.length }
		.map { it.substring(prefix.length, it.length - suffix.length) }
} catch (_: Exception) {
	emptyList()
}

/** Falls back to any tag of the same language, since shipped `zh_CN` and `pt_br` never equal the bare "zh" or "pt" a device reports, which left those readers in English. */
fun bestLocaleMatch(available: List<String>, locale: Locale): String? {
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
 * U+2063, silent to screen readers, appended where [nt]'s "many" English text equals its "few" text so the flat catalogue can hold both (Bosnian "sekunde" vs "sekundi").
 * Deliberate, not stray whitespace: don't strip it from translatable strings.
 */
private const val PLURAL_MANY_MARKER = "⁣"

fun t(str: String): String = Translations.map[str] ?: str.removeSuffix(PLURAL_MANY_MARKER)

private const val PLACEHOLDER = "{}"

/** One left-to-right scan, not `replaceFirst` per argument, so a "{}" inside an argument (a file name, a search term) never swallows the next argument. */
fun t(str: String, vararg args: String): String {
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

/** Read from the catalogue header rather than hardcoded: Bosnian gives 21 the form of 1, Polish gives it to 1 alone, though both want three forms. */
internal class PluralRule private constructor(private val nplurals: Int, private val expression: String) {
	fun formIndex(count: Long): Int? {
		val value = evalTernary(expression, count) ?: return null
		return if (value in 0 until nplurals) value else null
	}

	val forms: Int
		get() = nplurals

	companion object {
		fun parse(header: String?): PluralRule? {
			val line = header
				?.lineSequence()
				?.firstOrNull { it.startsWith("Plural-Forms:") }
				?: return null
			val nplurals = line
				.substringAfter("nplurals=", "")
				.takeWhile { it.isDigit() }
				.toIntOrNull()
				?.takeIf { it > 0 }
				?: return null
			val expression = line
				.substringAfter("plural=", "")
				.substringBefore(';')
				.trim()
				.takeIf { it.isNotEmpty() }
				?: return null
			return PluralRule(nplurals, expression)
		}

		/** Null on anything unrecognized, so a malformed header can't silently pick form 0 for every count. */
		private fun evalTernary(expr: String, n: Long): Int? {
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

		private fun evalBoolean(expr: String, n: Long): Boolean? {
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

		private fun evalArithmetic(expr: String, n: Long): Long? {
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

		/** False for `(a) && (b)`, whose outer parentheses look enclosing but are two separate groups. */
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

		private fun String.indexOfTopLevel(char: Char, from: Int = 0): Int {
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
 * Callers pass forms already wrapped in [t] (`nt(t("1 book"), t("{} books"), ...)`): the pot scanner only extracts plain `t("...")` calls.
 * The hardcoded fallback is the Bosnian/Serbian rule, wrong for Polish, so the catalogue's header wins whenever it parses.
 */
fun nt(one: String, few: String, many: String, count: Long): String {
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
