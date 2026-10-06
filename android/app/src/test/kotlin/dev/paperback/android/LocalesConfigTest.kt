package dev.paperback.android

import org.junit.Assert.assertEquals
import org.junit.Assume.assumeTrue
import org.junit.Test
import java.io.File

class LocalesConfigTest {
	// A language translated in po/ but missing here is one Android never offers on Paperback's language page.
	@Test
	fun `the language page offers English and every translated language`() {
		val poDir = File("../../po")
		assumeTrue("run from android/app, beside the repository's po/", poDir.isDirectory)
		val translated = poDir.listFiles { file -> file.extension == "po" }.orEmpty().map { file ->
			val parts = file.nameWithoutExtension.split('_')
			if (parts.size == 2) "${parts[0]}-${parts[1].uppercase()}" else parts[0]
		}
		val config = File("src/main/res/xml/locales_config.xml").readText()
		val offered = Regex("""android:name="([^"]+)"""").findAll(config).map { it.groupValues[1] }.toList()
		assertEquals((translated + "en").sorted(), offered.sorted())
	}
}
