package dev.paperback.android.tts

import kotlinx.coroutines.flow.StateFlow

interface Narrator {
	fun isPlaying(): Boolean

	fun play()

	fun pause()

	fun moveTo(position: Long)
}

/** Separate from TtsManager so [SpokenNarrator] can be tested without an engine. */
interface SpeechEngine {
	val isSpeaking: StateFlow<Boolean>
	val isPaused: StateFlow<Boolean>

	fun pause()

	fun resume()

	fun stop()
}

/** Separate from DaisyAudioPlayer so [RecordedNarrator] can be tested without a player. */
interface RecordedPlayer {
	fun isPlaying(): Boolean

	fun play()

	fun pause()

	fun seekToPosition(position: Long): Boolean
}

class RecordedNarrator(private val player: RecordedPlayer) : Narrator {
	override fun isPlaying(): Boolean = player.isPlaying()

	override fun play() = player.play()

	override fun pause() = player.pause()

	override fun moveTo(position: Long) {
		player.seekToPosition(position)
	}
}

class SpokenNarrator(private val speech: SpeechEngine, private val speakCurrentSegment: () -> Unit) : Narrator {
	override fun isPlaying(): Boolean = speech.isSpeaking.value

	override fun play() {
		// Resuming carries on mid sentence, where a fresh segment would start the paragraph again.
		if (speech.isPaused.value) {
			speech.resume()
		} else {
			speakCurrentSegment()
		}
	}

	override fun pause() = speech.pause()

	override fun moveTo(position: Long) {
		if (speech.isSpeaking.value) {
			// Speaking on past a jump would read out where the reader used to be.
			speakCurrentSegment()
		} else if (speech.isPaused.value) {
			// Kept over a jump, the pause would resume a sentence that is no longer where the reader is.
			speech.stop()
		}
	}
}
