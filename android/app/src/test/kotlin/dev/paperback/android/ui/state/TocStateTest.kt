package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.paperback.TocEntry

class TocStateTest {
	private fun toc(vararg entries: Pair<Long, Int>): List<TocEntry> =
		entries.mapIndexed { index, (position, level) -> TocEntry("Entry $index", position, level) }

	@Test
	fun `an empty table of contents has no active entry`() {
		assertNull(activeEntryFor(emptyList(), 100))
	}

	@Test
	fun `the active entry is the last one starting at or before the position`() {
		val toc = toc(0L to 0, 100L to 0, 200L to 0)
		assertEquals(0, activeEntryFor(toc, 50))
		assertEquals(1, activeEntryFor(toc, 100))
		assertEquals(1, activeEntryFor(toc, 199))
		assertEquals(2, activeEntryFor(toc, 5000))
	}

	// A document whose first heading is some way in still has to report a section for the text
	// above it, or opening the contents from the front matter would point at nothing.
	@Test
	fun `a position ahead of every entry counts as the first one`() {
		assertEquals(0, activeEntryFor(toc(500L to 0, 900L to 0), 10))
	}

	@Test
	fun `a top level entry has no ancestors`() {
		assertEquals(emptySet<Int>(), ancestorsOf(toc(0L to 0, 10L to 0), 1))
	}

	@Test
	fun `ancestors are every shallower entry up the branch`() {
		// 0: part (level 0), 1: chapter (1), 2: section (2), 3: subsection (3)
		val toc = toc(0L to 0, 10L to 1, 20L to 2, 30L to 3)
		assertEquals(setOf(0, 1, 2), ancestorsOf(toc, 3))
	}

	// Walking back up must skip the siblings between an entry and its real parent, or opening a
	// deep entry would expand half the book.
	@Test
	fun `siblings on the way up are not ancestors`() {
		val toc = toc(0L to 0, 10L to 1, 20L to 1, 30L to 2)
		assertEquals(setOf(0, 2), ancestorsOf(toc, 3))
	}

	@Test
	fun `pointing at a position expands the active entry's branch`() {
		val state = TocState()
		state.pointAt(toc(0L to 0, 10L to 1, 20L to 2), 25)
		assertEquals(2, state.state.value.activeIndex)
		assertEquals(setOf(0, 1), state.state.value.expandedIndices)
	}

	// Reopening the contents screen re-reads where the reader is, but must not collapse a branch
	// they opened by hand on the way past.
	@Test
	fun `pointing again keeps rows the reader expanded themselves`() {
		val state = TocState()
		state.toggleExpanded(5)
		state.pointAt(toc(0L to 0, 10L to 1), 15)
		assertEquals(setOf(0, 5), state.state.value.expandedIndices)
	}

	@Test
	fun `an empty table of contents clears the active entry and leaves expansion alone`() {
		val state = TocState()
		state.toggleExpanded(3)
		state.pointAt(emptyList(), 0)
		assertNull(state.state.value.activeIndex)
		assertEquals(setOf(3), state.state.value.expandedIndices)
	}

	@Test
	fun `toggling an expanded row collapses it again`() {
		val state = TocState()
		state.toggleExpanded(2)
		state.toggleExpanded(2)
		assertEquals(emptySet<Int>(), state.state.value.expandedIndices)
	}
}
