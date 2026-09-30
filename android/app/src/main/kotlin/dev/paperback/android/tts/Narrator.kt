package dev.paperback.android.tts

import kotlinx.coroutines.flow.StateFlow

/**
 * Whatever is reading the document aloud right now. A DAISY audiobook has a recording of someone
 * reading it, and everything else gets the synthesizer, but the reader's controls mean the same
 * thing either way, so the screens and the view model work through this rather than asking which
 * kind of book is open at every turn.
 */
interface Narrator {
	/** Whether something is audible right now. Paused counts as not playing. */
	fun isPlaying(): Boolean

	/** Picks up from wherever reading has got to, whether that is resuming a pause or starting
	 * afresh. */
	fun play()

	fun pause()

	/** Follows the reading position to [position], which is a text offset in both cases: a
	 * recording is anchored to the text it narrates. */
	fun moveTo(position: Long)
}

/** What [SpokenNarrator] needs of the speech engine, so it can be exercised without one. */
interface SpeechEngine {
	val isSpeaking: StateFlow<Boolean>
	val isPaused: StateFlow<Boolean>

	fun pause()

	fun resume()

	fun stop()
}

/** What [RecordedNarrator] needs of the player, so it can be exercised without one. */
interface RecordedPlayer {
	fun isPlaying(): Boolean

	fun play()

	fun pause()

	fun seekToPosition(position: Long): Boolean
}

/**
 * Reads a DAISY audiobook through its own recording. Moving the reading position is a seek: the
 * recording knows which clip narrates which part of the text, so playback follows the caret
 * rather than the other way round.
 */
class RecordedNarrator(
	private val player: RecordedPlayer
) : Narrator {
	override fun isPlaying(): Boolean = player.isPlaying()

	override fun play() = player.play()

	override fun pause() = player.pause()

	override fun moveTo(position: Long) {
		player.seekToPosition(position)
	}
}

/**
 * Reads a document through the speech synthesizer. Unlike a recording there is nothing to seek:
 * moving means speaking the segment now under the reading position, which [speakCurrentSegment]
 * does, since only the caller knows how to pull a segment out of the document.
 *
 * Paused is a state of its own here rather than a stopped player: resuming carries on mid
 * sentence, while a fresh [speakCurrentSegment] would start the paragraph again.
 */
class SpokenNarrator(
	private val speech: SpeechEngine,
	private val speakCurrentSegment: () -> Unit
) : Narrator {
	override fun isPlaying(): Boolean = speech.isSpeaking.value

	override fun play() {
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
			// A pause belongs to the segment it happened in. Kept over a jump, resuming would
			// carry on from a sentence that is no longer where the reader is.
			speech.stop()
		}
	}
}
