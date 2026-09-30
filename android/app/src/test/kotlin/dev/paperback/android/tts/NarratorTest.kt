package dev.paperback.android.tts

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

private class FakeSpeech : SpeechEngine {
	val calls = mutableListOf<String>()
	private val speaking = MutableStateFlow(false)
	private val paused = MutableStateFlow(false)

	override val isSpeaking: StateFlow<Boolean> = speaking
	override val isPaused: StateFlow<Boolean> = paused

	fun beSpeaking() {
		speaking.value = true
		paused.value = false
	}

	fun bePaused() {
		speaking.value = false
		paused.value = true
	}

	override fun pause() {
		calls += "pause"
	}

	override fun resume() {
		calls += "resume"
	}

	override fun stop() {
		calls += "stop"
	}
}

private class FakePlayer : RecordedPlayer {
	val calls = mutableListOf<String>()
	var playing = false

	override fun isPlaying(): Boolean = playing

	override fun play() {
		calls += "play"
	}

	override fun pause() {
		calls += "pause"
	}

	override fun seekToPosition(position: Long): Boolean {
		calls += "seek:$position"
		return true
	}
}

class RecordedNarratorTest {
	@Test
	fun `playing follows the player`() {
		val player = FakePlayer()
		val narrator = RecordedNarrator(player)
		assertFalse(narrator.isPlaying())
		player.playing = true
		assertTrue(narrator.isPlaying())
	}

	// A recording is anchored to the text it narrates, so following the reader is a seek. There
	// is no equivalent of "say this bit again".
	@Test
	fun `moving the reading position seeks the recording`() {
		val player = FakePlayer()
		RecordedNarrator(player).moveTo(4200)
		assertEquals(listOf("seek:4200"), player.calls)
	}

	@Test
	fun `play and pause reach the player unchanged`() {
		val player = FakePlayer()
		val narrator = RecordedNarrator(player)
		narrator.play()
		narrator.pause()
		assertEquals(listOf("play", "pause"), player.calls)
	}
}

class SpokenNarratorTest {
	private fun narrator(
		speech: FakeSpeech,
		spoken: MutableList<String>
	) = SpokenNarrator(speech) { spoken += "speak" }

	@Test
	fun `playing from a pause carries on rather than starting the paragraph again`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		speech.bePaused()
		narrator(speech, spoken).play()
		assertEquals(listOf("resume"), speech.calls)
		assertEquals(emptyList<String>(), spoken)
	}

	@Test
	fun `playing from a standing start speaks the current segment`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		narrator(speech, spoken).play()
		assertEquals(emptyList<String>(), speech.calls)
		assertEquals(listOf("speak"), spoken)
	}

	@Test
	fun `pausing reaches the engine`() {
		val speech = FakeSpeech()
		narrator(speech, mutableListOf()).pause()
		assertEquals(listOf("pause"), speech.calls)
	}

	// Carrying on through a jump would read out where the reader used to be.
	@Test
	fun `moving while speaking speaks the new segment instead`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		speech.beSpeaking()
		narrator(speech, spoken).moveTo(99)
		assertEquals(listOf("speak"), spoken)
	}

	// A pause belongs to the segment it happened in: resuming after a jump would carry on from a
	// sentence that is no longer where the reader is.
	@Test
	fun `moving while paused drops the pause`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		speech.bePaused()
		narrator(speech, spoken).moveTo(99)
		assertEquals(listOf("stop"), speech.calls)
		assertEquals(emptyList<String>(), spoken)
	}

	// Browsing a document nobody is listening to must stay silent, or every arrow key would
	// start reading.
	@Test
	fun `moving while silent says nothing`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		narrator(speech, spoken).moveTo(99)
		assertEquals(emptyList<String>(), speech.calls)
		assertEquals(emptyList<String>(), spoken)
	}

	@Test
	fun `playing reports what the engine is doing`() {
		val speech = FakeSpeech()
		val narrator = narrator(speech, mutableListOf())
		assertFalse(narrator.isPlaying())
		speech.beSpeaking()
		assertTrue(narrator.isPlaying())
		speech.bePaused()
		assertFalse(narrator.isPlaying())
	}
}

/** The play/pause control, which is the same gesture whichever narrator is reading. */
class PlayPauseTest {
	private fun toggle(narrator: Narrator) {
		if (narrator.isPlaying()) narrator.pause() else narrator.play()
	}

	@Test
	fun `toggling a playing recording pauses it`() {
		val player = FakePlayer()
		player.playing = true
		toggle(RecordedNarrator(player))
		assertEquals(listOf("pause"), player.calls)
	}

	@Test
	fun `toggling a paused recording plays it`() {
		val player = FakePlayer()
		toggle(RecordedNarrator(player))
		assertEquals(listOf("play"), player.calls)
	}

	@Test
	fun `toggling speech that is speaking pauses it`() {
		val speech = FakeSpeech()
		speech.beSpeaking()
		toggle(SpokenNarrator(speech) { })
		assertEquals(listOf("pause"), speech.calls)
	}

	@Test
	fun `toggling paused speech resumes rather than restarting`() {
		val speech = FakeSpeech()
		speech.bePaused()
		toggle(SpokenNarrator(speech) { })
		assertEquals(listOf("resume"), speech.calls)
	}

	@Test
	fun `toggling silent speech starts reading`() {
		val speech = FakeSpeech()
		val spoken = mutableListOf<String>()
		toggle(SpokenNarrator(speech) { spoken += "speak" })
		assertEquals(listOf("speak"), spoken)
	}
}
