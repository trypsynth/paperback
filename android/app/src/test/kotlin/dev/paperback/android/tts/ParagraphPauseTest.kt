package dev.paperback.android.tts

import org.junit.Assert.assertEquals
import org.junit.Test

class ParagraphPauseTest {
	@Test
	fun `a stored pause is brought onto the setting's steps and inside its range`() {
		assertEquals(0, paragraphPauseFor(-50))
		assertEquals(250, paragraphPauseFor(247))
		assertEquals(1000, paragraphPauseFor(5000))
	}
}
