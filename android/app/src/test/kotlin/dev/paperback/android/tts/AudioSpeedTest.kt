package dev.paperback.android.tts

import org.junit.Assert.assertEquals
import org.junit.Test

class AudioSpeedTest {
	@Test
	fun `the speed stays between half and four times and settles on hundredths`() {
		assertEquals(4f, clampAudioSpeed(9f))
		assertEquals(0.5f, clampAudioSpeed(0.1f))
		assertEquals(2.6f, clampAudioSpeed(2.5f + AUDIO_SPEED_STEP))
	}

	@Test
	fun `speeds read as desktop says them`() {
		assertEquals("1x", audioSpeedLabel(1f))
		assertEquals("2.5x", audioSpeedLabel(2.5f))
		assertEquals("1.25x", audioSpeedLabel(1.25f))
	}
}
