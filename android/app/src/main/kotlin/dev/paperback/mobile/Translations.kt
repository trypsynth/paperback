package dev.paperback.mobile

import android.content.Context
import org.json.JSONObject
import java.util.Locale

object Translations {
	internal var map: HashMap<String, String> = HashMap()

	fun load(context: Context) {
		val lang = Locale.getDefault().language
		try {
			val json = context.assets.open("translations/$lang.json").bufferedReader().use { it.readText() }
			val obj = JSONObject(json)
			val loaded = HashMap<String, String>(obj.length())
			for (key in obj.keys()) {
				loaded[key] = obj.getString(key)
			}
			map = loaded
		} catch (_: Exception) {
			// Language not available or asset not yet generated — fall back to English
		}
	}
}

fun t(str: String): String = Translations.map[str] ?: str
