package dev.paperback.android.ui.state

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ReadingModeSyncTest {
	@Test
	fun `leaving text mode takes the position from the list`() {
		assertTrue(shouldSyncPositionFromList(previousMode = true, isTextMode = false))
	}

	// #743: popping the TOC recomposes MainScreen with no previous mode, and syncing there replaced the chosen section with the opening index.
	@Test
	fun `a first run with no previous mode does not touch the position`() {
		assertFalse(shouldSyncPositionFromList(previousMode = null, isTextMode = false))
	}

	@Test
	fun `staying in read aloud mode does not touch the position`() {
		assertFalse(shouldSyncPositionFromList(previousMode = false, isTextMode = false))
	}

	@Test
	fun `entering text mode does not take the position from the list`() {
		assertFalse(shouldSyncPositionFromList(previousMode = false, isTextMode = true))
		assertFalse(shouldSyncPositionFromList(previousMode = true, isTextMode = true))
	}
}
