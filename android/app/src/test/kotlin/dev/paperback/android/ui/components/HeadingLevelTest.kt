package dev.paperback.android.ui.components

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.paperback.MarkerType

class HeadingLevelTest {
	// The level sizes the line and tells a screen reader how deep in the book it is, so an
	// off-by-one here reads a chapter title out as a subsection.
	@Test
	fun `each heading marker reports its own level`() {
		assertEquals(1, headingLevelOf(MarkerType.HEADING1))
		assertEquals(2, headingLevelOf(MarkerType.HEADING2))
		assertEquals(3, headingLevelOf(MarkerType.HEADING3))
		assertEquals(4, headingLevelOf(MarkerType.HEADING4))
		assertEquals(5, headingLevelOf(MarkerType.HEADING5))
		assertEquals(6, headingLevelOf(MarkerType.HEADING6))
	}

	@Test
	fun `a marker that is not a heading has no level`() {
		assertNull(headingLevelOf(MarkerType.LINK))
		assertNull(headingLevelOf(MarkerType.IMAGE))
		assertNull(headingLevelOf(MarkerType.TABLE))
		assertNull(headingLevelOf(MarkerType.FIGURE))
	}

	@Test
	fun `exactly six markers are headings and they cover one to six`() {
		val levels = MarkerType.entries.mapNotNull { headingLevelOf(it) }
		assertEquals(listOf(1, 2, 3, 4, 5, 6), levels.sorted())
	}
}
