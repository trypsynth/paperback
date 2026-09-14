package dev.paperback.android.tts

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SpeechScaleTest {
	@Test
	fun `the slider ends map to the ends of the engine's range`() {
		assertEquals(0.1f, speechRateFor(0), 0.001f)
		assertEquals(3.0f, speechRateFor(100), 0.001f)
		assertEquals(0.1f, pitchFor(0), 0.001f)
		assertEquals(2.0f, pitchFor(100), 0.001f)
	}

	// 50 is where the slider sits by default, so it has to be an ordinary speaking rate rather
	// than the midpoint of a range that starts near silence.
	@Test
	fun `the default sits in the middle of the range`() {
		assertEquals(1.55f, speechRateFor(50), 0.001f)
		assertEquals(1.05f, pitchFor(50), 0.001f)
	}

	@Test
	fun `a stored value outside the slider's range is brought back inside it`() {
		assertEquals(speechRateFor(100), speechRateFor(500), 0.001f)
		assertEquals(speechRateFor(0), speechRateFor(-40), 0.001f)
		assertEquals(pitchFor(100), pitchFor(9000), 0.001f)
	}

	@Test
	fun `rate rises with the percentage`() {
		assertTrue(speechRateFor(10) < speechRateFor(50))
		assertTrue(speechRateFor(50) < speechRateFor(90))
		assertTrue(pitchFor(10) < pitchFor(90))
	}
}
