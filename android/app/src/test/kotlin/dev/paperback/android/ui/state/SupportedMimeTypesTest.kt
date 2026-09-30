package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SupportedMimeTypesTest {
	// The picker filters on type, so a format missing from this table is a book the reader cannot
	// pick at all, however readable it is once opened.
	@Test
	fun `the formats Android does not know still have a type`() {
		assertTrue(extraMimeTypesFor("epub").contains("application/epub+zip"))
		assertTrue(extraMimeTypesFor("mobi").contains("application/x-mobipocket-ebook"))
		assertTrue(extraMimeTypesFor("chm").contains("application/vnd.ms-htmlhelp"))
	}

	@Test
	fun `an extension can carry more than one type`() {
		assertEquals(setOf("application/xml", "text/xml"), extraMimeTypesFor("xml"))
	}

	// Extensions reach this from the config file and from file names, which can be in any case.
	@Test
	fun `the case of the extension does not matter`() {
		assertEquals(extraMimeTypesFor("epub"), extraMimeTypesFor("EPUB"))
		assertEquals(extraMimeTypesFor("epub"), extraMimeTypesFor("ePub"))
	}

	@Test
	fun `an extension with nothing to add is empty rather than missing`() {
		assertEquals(emptySet<String>(), extraMimeTypesFor("jpg"))
		assertEquals(emptySet<String>(), extraMimeTypesFor(""))
	}

	@Test
	fun `every type in the table is well formed`() {
		val extensions = listOf("epub", "fb2", "md", "chm", "opf", "fodt", "zip", "rtf", "pdf", "xml")
		val types = extensions.flatMap { extraMimeTypesFor(it) }
		assertTrue(types.isNotEmpty())
		assertTrue(types.all { it.contains("/") && !it.contains(" ") })
	}
}
