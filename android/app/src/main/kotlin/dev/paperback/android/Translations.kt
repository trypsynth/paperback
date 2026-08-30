package dev.paperback.android

import android.content.Context
import org.json.JSONObject
import java.util.Locale

object Translations {
	internal var map: HashMap<String, String> = HashMap()

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

fun t(str: String): String = Translations.map[str] ?: str

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
