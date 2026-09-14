package dev.paperback.android.tts

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.os.Build

/**
 * The attributes every stream this app plays goes out with: media usage, so it rides the volume
 * the user expects a book to, and speech content, so the system knows it is talking rather than
 * playing music.
 */
fun speechAudioAttributes(): AudioAttributes =
	AudioAttributes
		.Builder()
		.setUsage(AudioAttributes.USAGE_MEDIA)
		.setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
		.build()

/**
 * Audio focus for one player. Something else taking focus (a call, another app starting up)
 * pauses playback through [onPause], and focus coming back resumes it through [onResume], but
 * only when this player is the one that was interrupted: [isPlaying] is read at the moment focus
 * is lost, so a player that was already paused stays paused.
 *
 * Ducking is deliberately not handled. The system ducks on API 26 and up, and on older releases
 * speech carries on at full volume rather than being quietly talked over.
 */
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
