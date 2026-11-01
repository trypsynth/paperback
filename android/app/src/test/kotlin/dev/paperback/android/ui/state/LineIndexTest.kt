package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Test

class LineIndexTest {
	@Test
	fun `line numbers count from one and indices from zero`() {
		assertEquals(0, lineIndexFor(1))
		assertEquals(1, lineIndexFor(2))
		assertEquals(41, lineIndexFor(42))
	}

	// Line zero would index off the front of the list, a crash rather than a scroll.
	@Test
	fun `a line before the first one lands on the first`() {
		assertEquals(0, lineIndexFor(0))
		assertEquals(0, lineIndexFor(-5))
	}

	@Test
	fun `a long document keeps its line numbers`() {
		assertEquals(999_999, lineIndexFor(1_000_000))
	}
}
