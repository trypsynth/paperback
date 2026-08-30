package dev.paperback.android

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import java.util.Locale

/** The tags actually shipped in assets/translations, as `cargo xtask android` names them. */
private val SHIPPED = listOf(
	"bs", "cs", "de", "es", "fi", "fr", "ja", "nl", "pl", "pt_br", "ru", "sr", "vi", "zh_CN"
)

class BestLocaleMatchTest {
	@Test
	fun `a plain language code matches its own catalogue`() {
		assertEquals("de", bestLocaleMatch(SHIPPED, Locale("de", "DE")))
		assertEquals("fr", bestLocaleMatch(SHIPPED, Locale("fr", "FR")))
	}

	// The bug this all exists for: a Chinese device reports "zh", the catalogue is "zh_CN", and
	// matching on the bare language code found nothing, so a complete translation read English.
	@Test
	fun `a country-tagged catalogue is found from the device's country`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale("zh", "CN")))
	}

	// Same bug, and a reminder that the two country tags disagree on case: pt_br vs zh_CN.
	@Test
	fun `matching ignores the case of the country tag`() {
		assertEquals("pt_br", bestLocaleMatch(SHIPPED, Locale("pt", "BR")))
	}

	// Only one Chinese and one Portuguese catalogue exists, so a reader in Taiwan or Portugal is
	// better served by it than by falling all the way back to English.
	@Test
	fun `another country for the same language still finds that language`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale("zh", "TW")))
		assertEquals("pt_br", bestLocaleMatch(SHIPPED, Locale("pt", "PT")))
	}

	@Test
	fun `a language with no country still matches`() {
		assertEquals("zh_CN", bestLocaleMatch(SHIPPED, Locale("zh")))
		assertEquals("de", bestLocaleMatch(SHIPPED, Locale("de")))
	}

	// English ships no catalogue: the msgids are already English, so no match is the right answer.
	@Test
	fun `an untranslated language matches nothing`() {
		assertNull(bestLocaleMatch(SHIPPED, Locale("en", "US")))
		assertNull(bestLocaleMatch(SHIPPED, Locale("is", "IS")))
	}

	@Test
	fun `no catalogues at all matches nothing`() {
		assertNull(bestLocaleMatch(emptyList(), Locale("de", "DE")))
	}

	// A country-specific catalogue must win over a bare one for the same language, so shipping
	// pt alongside pt_br would still send a Brazilian reader to pt_br.
	@Test
	fun `the device's own country wins over a bare language catalogue`() {
		val both = listOf("pt", "pt_br")
		assertEquals("pt_br", bestLocaleMatch(both, Locale("pt", "BR")))
		assertEquals("pt", bestLocaleMatch(both, Locale("pt", "PT")))
	}
}
