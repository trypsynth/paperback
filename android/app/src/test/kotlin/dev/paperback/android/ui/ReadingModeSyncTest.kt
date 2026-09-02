package dev.paperback.android.ui

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ReadingModeSyncTest {
	@Test
	fun `leaving text mode takes the position from the list`() {
		assertTrue(shouldSyncPositionFromList(previousMode = true, isTextMode = false))
	}

	// The case behind issue #743: popping the table of contents composes MainScreen again, which
	// runs the effect with no previous mode. Syncing there would replace the section the reader
	// just chose with the index the document was opened at.
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
