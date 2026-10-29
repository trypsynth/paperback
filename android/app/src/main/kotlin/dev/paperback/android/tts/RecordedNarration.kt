package dev.paperback.android.tts

import android.content.Context
import dev.paperback.android.ui.state.DocumentTabState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi

/** One player shared by every tab, since only one book narrates at a time and a decoder per tab would hold a file and memory per book. */
class RecordedNarration(context: Context, private val config: ConfigManagerFfi, private val scope: CoroutineScope) :
	RecordedPlayer {
	private val player = DaisyAudioPlayer(context)

	private var attachedDocumentUri: String? = null

	var onPlayingChanged: ((playing: Boolean) -> Unit)? = null

	var onSeekLanded: ((elapsedMs: Long) -> Unit)? = null

	var onClipChanged: ((position: Long) -> Unit)? = null

	var onDetached: (() -> Unit)? = null

	private val _speed = MutableStateFlow(1f)
	val speed: StateFlow<Float> = _speed.asStateFlow()

	// Android can kill the process without any callback, so the position is saved on every pause and clip change rather than on the way out.
	init {
		player.onPlaybackStateChanged = { playing ->
			onPlayingChanged?.invoke(playing)
			if (!playing) persistPosition()
		}
		player.onRelativeSeekLanded = { elapsedMs -> onSeekLanded?.invoke(elapsedMs) }
		player.onClipChanged = { position ->
			onClipChanged?.invoke(position)
			persistPosition()
		}
		player.speed = config.getAppString(SPEED_KEY, "").toFloatOrNull()?.let(::clampAudioSpeed) ?: 1f
		_speed.value = player.speed
	}

	fun setSpeed(newSpeed: Float) {
		val clamped = clampAudioSpeed(newSpeed)
		if (clamped == player.speed) return
		player.speed = clamped
		_speed.value = clamped
		scope.launch(Dispatchers.IO) {
			config.setAppString(SPEED_KEY, clamped.toString())
			config.flush()
		}
	}

	private companion object {
		/** One speed for every recording, under the key desktop and iOS use too. */
		const val SPEED_KEY = "audio_speed"
	}

	fun attach(tab: DocumentTabState) {
		if (attachedDocumentUri == tab.documentUri) return
		detach()
		if (!tab.hasAudio) return
		player.attach(tab.session, tab.docKey)
		attachedDocumentUri = tab.documentUri
		val savedAudioMs = config.getDocumentAudioTimeFfi(tab.documentUri)
		if (savedAudioMs >= 0) {
			player.seekToMs(savedAudioMs)
		} else {
			player.seekToPosition(tab.savedPosition)
		}
	}

	fun detach() {
		persistPosition()
		player.detach()
		attachedDocumentUri = null
		onDetached?.invoke()
	}

	fun isNarrating(documentUri: String): Boolean = attachedDocumentUri == documentUri

	fun persistPosition() {
		val uri = attachedDocumentUri ?: return
		val ms = player.resumePointMs() ?: return
		scope.launch(Dispatchers.IO) {
			config.setDocumentAudioTimeFfi(uri, ms)
			config.flush()
		}
	}

	override fun isPlaying(): Boolean = player.isPlaying()

	override fun play() = player.play()

	override fun pause() = player.pause()

	override fun seekToPosition(position: Long): Boolean = player.seekToPosition(position)

	fun seekToMs(elapsedMs: Long): Boolean = player.seekToMs(elapsedMs)

	fun seekRelativeMs(deltaMs: Long): Boolean = player.seekRelativeMs(deltaMs)

	fun shutdown() = player.shutdown()
}
