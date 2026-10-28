package dev.paperback.android.tts

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.os.Build

/** Media usage so a book rides the media volume the reader expects. */
fun speechAudioAttributes(): AudioAttributes = AudioAttributes
	.Builder()
	.setUsage(AudioAttributes.USAGE_MEDIA)
	.setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
	.build()

/** Ducking is deliberately not handled: the system ducks on API 26+, and below that speech carries on at full volume. */
class AudioFocusHolder(
	context: Context,
	private val isPlaying: () -> Boolean,
	private val onPause: () -> Unit,
	private val onResume: () -> Unit
) {
	private val audioManager = context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
	private var focusRequest: AudioFocusRequest? = null
	private var wasPlayingBeforeLoss = false

	private val listener =
		AudioManager.OnAudioFocusChangeListener { focusChange ->
			when (focusChange) {
				AudioManager.AUDIOFOCUS_LOSS,
				AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
					// Read at the moment of loss, so a player that was already paused stays paused when focus returns.
					wasPlayingBeforeLoss = isPlaying()
					if (wasPlayingBeforeLoss) onPause()
				}
				AudioManager.AUDIOFOCUS_GAIN -> {
					if (wasPlayingBeforeLoss) {
						wasPlayingBeforeLoss = false
						onResume()
					}
				}
			}
		}

	fun request() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			val request =
				AudioFocusRequest
					.Builder(AudioManager.AUDIOFOCUS_GAIN)
					.setAudioAttributes(speechAudioAttributes())
					.setOnAudioFocusChangeListener(listener)
					.build()
			focusRequest = request
			audioManager.requestAudioFocus(request)
		} else {
			@Suppress("DEPRECATION")
			audioManager.requestAudioFocus(listener, AudioManager.STREAM_MUSIC, AudioManager.AUDIOFOCUS_GAIN)
		}
	}

	fun abandon() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			focusRequest?.let { audioManager.abandonAudioFocusRequest(it) }
		} else {
			@Suppress("DEPRECATION")
			audioManager.abandonAudioFocus(listener)
		}
	}
}
