package dev.paperback.android

import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import java.util.Locale

/** As `cargo xtask android` names them in assets/translations. */
private val SHIPPED = listOf(
	"bs", "cs", "de", "es", "fi", "fr", "ja", "nl", "pl", "pt_br", "ru", "sr", "uk", "vi", "zh_CN"
)

class BestLocaleMatchTest {
	@Test
	fun `a plain language code matches its own catalogue`() {
		assertEquals("de", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("de-DE")))
		assertEquals("fr", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("fr-FR")))
	}

	// The bug: a Chinese device reports "zh", the catalogue is "zh_CN", and a complete translation read English.
	@Test
	fun `a country-tagged catalogue is found from the device's country`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("zh-CN")))
	}

	// Same bug, and a reminder that the two country tags disagree on case: pt_br vs zh_CN.
	@Test
	fun `matching ignores the case of the country tag`() {
		assertEquals("pt_br", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("pt-BR")))
	}

	// The one Chinese or Portuguese catalogue serves Taiwan or Portugal better than English does.
	@Test
	fun `another country for the same language still finds that language`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("zh-TW")))
		assertEquals("pt_br", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("pt-PT")))
	}

	@Test
	fun `a language with no country still matches`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("zh")))
		assertEquals("de", bestLocaleMatch(SHIPPED, Locale.forLanguageTag("de")))
	}

	// English ships no catalogue: the msgids are already English, so no match is the right answer.
	@Test
	fun `an untranslated language matches nothing`() {
		assertNull(bestLocaleMatch(SHIPPED, Locale.forLanguageTag("en-US")))
		assertNull(bestLocaleMatch(SHIPPED, Locale.forLanguageTag("is-IS")))
	}

	@Test
	fun `no catalogues at all matches nothing`() {
		assertNull(bestLocaleMatch(emptyList(), Locale.forLanguageTag("de-DE")))
	}

	@Test
	fun `the device's own country wins over a bare language catalogue`() {
		val both = listOf("pt", "pt_br")
		assertEquals("pt_br", bestLocaleMatch(both, Locale.forLanguageTag("pt-BR")))
		assertEquals("pt", bestLocaleMatch(both, Locale.forLanguageTag("pt-PT")))
	}
}

class TranslateTest {
	@After
	fun clearCatalogue() {
		Translations.map = HashMap()
		Translations.pluralRule = null
	}

	// English ships no catalogue at all, so an untranslated build has to read as the msgids.
	@Test
	fun `a string with no catalogue entry is its own translation`() {
		assertEquals("Recent Documents", t("Recent Documents"))
	}

	@Test
	fun `a string in the catalogue is translated`() {
		Translations.map = hashMapOf("Recent Documents" to "Dokumente")
		assertEquals("Dokumente", t("Recent Documents"))
	}

	@Test
	fun `placeholders are filled in the order the arguments are given`() {
		Translations.map = hashMapOf("{} of {}" to "{} von {}")
		assertEquals("3 von 9", t("{} of {}", "3", "9"))
	}

	// Nav-unit labels translate before substituting, which is what lets a translation move the placeholder.
	@Test
	fun `a translation may put its placeholder anywhere`() {
		Translations.map = hashMapOf("Back {}" to "{} zurück")
		assertEquals("30 seconds zurück", t("Back {}", "30 seconds"))
	}

	// Otherwise a file name or search term containing "{}" would swallow the next argument.
	@Test
	fun `an argument containing a placeholder is not substituted into`() {
		assertEquals("{} and b", t("{} and {}", "{}", "b"))
	}

	// A translation may drop a placeholder or keep more than the call site fills, and neither may throw.
	@Test
	fun `spare placeholders and spare arguments are both left alone`() {
		assertEquals("a and {}", t("{} and {}", "a"))
		assertEquals("just a", t("just {}", "a", "b"))
		assertEquals("no placeholders", t("no placeholders", "a"))
	}
}

private const val POLISH =
	"Plural-Forms: nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<12 || n%100>14) ? 1 : 2);"
private const val SERBIAN =
	"Plural-Forms: nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && " +
		"(n%100<12 || n%100>14) ? 1 : 2);"
private const val GERMAN = "Plural-Forms: nplurals=2; plural=(n != 1);"
private const val CHINESE = "Plural-Forms: nplurals=1; plural=0;"

class PluralFormTest {
	@After
	fun clearCatalogue() {
		Translations.map = HashMap()
		Translations.pluralRule = null
	}

	private fun forCount(count: Long) = nt("one", "few", "many", count)

	// The bug: the hardcoded Serbian rule made Polish read "21 sekunda" for "21 sekund".
	@Test
	fun `Polish gives the singular to one alone`() {
		Translations.pluralRule = PluralRule.parse(POLISH)
		assertEquals("one", forCount(1))
		assertEquals("few", forCount(2))
		assertEquals("many", forCount(5))
		assertEquals("many", forCount(11))
		assertEquals("many", forCount(21))
		assertEquals("few", forCount(22))
		assertEquals("many", forCount(101))
		assertEquals("many", forCount(121))
	}

	// The language the hardcoded fallback and the plural marker were written for.
	@Test
	fun `Serbian keeps giving twenty-one the singular`() {
		Translations.pluralRule = PluralRule.parse(SERBIAN)
		assertEquals("one", forCount(1))
		assertEquals("one", forCount(21))
		assertEquals("few", forCount(22))
		assertEquals("many", forCount(11))
	}

	// For a two-form language the third slot duplicates the second, so it must never be reached.
	@Test
	fun `a two-form language uses only the first two forms`() {
		Translations.pluralRule = PluralRule.parse(GERMAN)
		assertEquals("one", forCount(1))
		assertEquals("few", forCount(0))
		assertEquals("few", forCount(2))
		assertEquals("few", forCount(21))
	}

	@Test
	fun `a one-form language always uses the first form`() {
		Translations.pluralRule = PluralRule.parse(CHINESE)
		assertEquals("one", forCount(0))
		assertEquals("one", forCount(1))
		assertEquals("one", forCount(5))
		assertEquals("one", forCount(21))
	}

	// Czech writes its rule this way.
	@Test
	fun `a rule written with parenthesized comparisons parses`() {
		Translations.pluralRule = PluralRule.parse("Plural-Forms: nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;")
		assertEquals("one", forCount(1))
		assertEquals("few", forCount(3))
		assertEquals("many", forCount(9))
	}

	// Arabic has six forms to nt()'s three slots.
	@Test
	fun `more forms than slots clamps to the last slot`() {
		Translations.pluralRule =
			PluralRule.parse(
				"Plural-Forms: nplurals=6; plural=(n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : " +
					"n%100>=3 && n%100<=10 ? 3 : n%100>=11 ? 4 : 5);"
			)
		assertEquals("one", forCount(0))
		assertEquals("few", forCount(1))
		assertEquals("many", forCount(2))
		assertEquals("many", forCount(5))
	}

	// From here down: an untrustworthy header must be no worse than the hardcoded rule.
	@Test
	fun `no catalogue keeps the previous behaviour`() {
		assertEquals("one", forCount(21))
		assertEquals("few", forCount(22))
		assertEquals("many", forCount(11))
	}

	@Test
	fun `a header without a plural rule parses to nothing`() {
		assertNull(PluralRule.parse("Language: pl\nMIME-Version: 1.0\n"))
		assertNull(PluralRule.parse(null))
	}

	@Test
	fun `a rule with no usable form count parses to nothing`() {
		assertNull(PluralRule.parse("Plural-Forms: nplurals=; plural=0;"))
		assertNull(PluralRule.parse("Plural-Forms: nplurals=0; plural=0;"))
		assertNull(PluralRule.parse("Plural-Forms: nplurals=2;"))
	}

	// The hardcoded rule at least keeps three-form Slavic languages readable.
	@Test
	fun `an unevaluatable expression falls back instead of picking form zero`() {
		Translations.pluralRule = PluralRule.parse("Plural-Forms: nplurals=3; plural=(n==1 ? 0 : ;")
		assertEquals("one", forCount(1))
		assertEquals("few", forCount(22))
		assertEquals("many", forCount(11))
	}

	// An out-of-range index is a malformed header, not a form to index with.
	@Test
	fun `an index beyond the declared form count is refused`() {
		val rule = PluralRule.parse("Plural-Forms: nplurals=2; plural=5;")
		assertNull(rule?.formIndex(1))
	}
}
