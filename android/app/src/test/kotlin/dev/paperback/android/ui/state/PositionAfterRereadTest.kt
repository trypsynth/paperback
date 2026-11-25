package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Test

class PositionAfterRereadTest {
	@Test
	fun `the place keeps its share of the text`() {
		assertEquals(500L, positionAfterReread(250L, 1_000L, 2_000L))
		assertEquals(100L, positionAfterReread(200L, 1_000L, 500L))
	}

	@Test
	fun `the same length keeps the same position`() {
		assertEquals(4_321L, positionAfterReread(4_321L, 10_000L, 10_000L))
	}

	// Multiplying first in whole numbers would overflow on a book this long.
	@Test
	fun `a very long document does not overflow`() {
		assertEquals(3_000_000_000L, positionAfterReread(3_000_000_000L, 6_000_000_000L, 6_000_000_000L))
	}

	@Test
	fun `an empty document before or after lands at the start`() {
		assertEquals(0L, positionAfterReread(10L, 0L, 500L))
		assertEquals(0L, positionAfterReread(10L, 100L, 0L))
	}

	@Test
	fun `a position past the old end stays inside the new text`() {
		assertEquals(50L, positionAfterReread(200L, 100L, 50L))
	}
}
