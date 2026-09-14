package dev.paperback.android.tts

import android.content.Context
import dev.paperback.android.ui.state.DocumentTabState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi

/**
 * A DAISY audiobook's own recorded narration, moved from tab to tab as the reader switches
 * documents. One player serves every open document: only one of them can be narrating at a time,
 * and holding a decoder open per tab would keep a file handle and a chunk of memory per book.
 *
 * Where playback has reached is written back to the config on every clip change and every pause,
 * not at some later checkpoint, because Android can kill this process without a lifecycle
 * callback of any kind, and a book resumed from the wrong place is a book the reader has to find
 * their way back through.
 */
class RecordedNarration(
	context: Context,
	private val config: ConfigManagerFfi,
	private val scope: CoroutineScope
) : RecordedPlayer {
	private val player = DaisyAudioPlayer(context)

	/** The document the player is currently narrating, if any. */
	private var attachedDocumentUri: String? = null

	/** Invoked whenever playback starts or stops, including at the end of the book. */
	var onPlayingChanged: ((playing: Boolean) -> Unit)? = null

	/** Invoked with where a relative seek actually landed, in document elapsed time. */
	var onSeekLanded: ((elapsedMs: Long) -> Unit)? = null

	/** Invoked with the text position of the clip now narrating, whenever it changes. */
	var onClipChanged: ((position: Long) -> Unit)? = null

	/** Invoked when the player moves off a document, so a caller tracking what it last said
	 * about this one can forget it. */
	var onDetached: (() -> Unit)? = null

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
	}

	/**
	 * Switches to narrating [tab], picking up from its saved audio position, or from its saved
	 * reading position when it has never been listened to. Does nothing for a tab with no
	 * recording, or one that is already attached.
	 */
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

	/** Saves where playback had reached and lets the document go, ahead of switching to another
	 * one or the app going away. */
	fun detach() {
		persistPosition()
		player.detach()
		attachedDocumentUri = null
		onDetached?.invoke()
	}

	/** Whether the recording currently loaded belongs to [documentUri]. */
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
