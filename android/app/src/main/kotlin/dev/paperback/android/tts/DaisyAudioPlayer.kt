package dev.paperback.android.tts

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.media.MediaPlayer
import android.os.Build
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.DocumentSession
import java.io.File

private const val POLL_INTERVAL_MS = 250L

/**
 * Plays a DAISY 3 audiobook's recorded narration against its `AudioTimeline`, the Android
 * counterpart to desktop's wxMediaCtrl-backed AudioPlayer. Unlike desktop, there's no "sync
 * caret to audio" toggle: `onClipChanged` always fires, so the caller keeps the existing
 * currently-spoken-text display in step with playback unconditionally.
 */
class DaisyAudioPlayer(
	private val context: Context
) {
	private val scope = CoroutineScope(Dispatchers.Main)
	private val cacheDir = File(context.cacheDir, "paperback_daisy_audio_cache")

	private var session: DocumentSession? = null
	private var docKey: String? = null

	private var mediaPlayer: MediaPlayer? = null
	private var currentSource: Int? = null
	private var playing = false
	private var loadGeneration = 0

	/** A seek requested while paused, applied lazily on resume (see `play()`). */
	private var pendingTargetMs: Long? = null

	/** Most recent seek target, so a jump resolving to the same spot doesn't restart audio. */
	private var lastSeekTarget: Pair<Int, Long>? = null

	private var lastReportedClip: Int? = null
	private var pollJob: Job? = null

	/** Invoked with the start position of the clip currently narrating, whenever it changes. */
	var onClipChanged: ((position: Long) -> Unit)? = null

	/** Invoked whenever play/pause state changes, including auto-advance and end-of-book. */
	var onPlaybackStateChanged: ((playing: Boolean) -> Unit)? = null

	private val audioManager = context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
	private var audioFocusRequest: AudioFocusRequest? = null
	private var wasPlayingBeforeFocusLoss = false

	// Mirrors TtsManager's own focus handling.
	private val audioFocusChangeListener =
		AudioManager.OnAudioFocusChangeListener { focusChange ->
			when (focusChange) {
				AudioManager.AUDIOFOCUS_LOSS,
				AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
					wasPlayingBeforeFocusLoss = playing
					if (playing) pause()
				}
				AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK -> {
					// System handles ducking; nothing to do here.
				}
				AudioManager.AUDIOFOCUS_GAIN -> {
					if (wasPlayingBeforeFocusLoss) {
						wasPlayingBeforeFocusLoss = false
						play()
					}
				}
			}
		}

	private fun speechAudioAttributes(): AudioAttributes =
		AudioAttributes
			.Builder()
			.setUsage(AudioAttributes.USAGE_MEDIA)
			.setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
			.build()

	private fun requestAudioFocus() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			val request =
				AudioFocusRequest
					.Builder(AudioManager.AUDIOFOCUS_GAIN)
					.setAudioAttributes(speechAudioAttributes())
					.setOnAudioFocusChangeListener(audioFocusChangeListener)
					.build()
			audioFocusRequest = request
			audioManager.requestAudioFocus(request)
		} else {
			@Suppress("DEPRECATION")
			audioManager.requestAudioFocus(audioFocusChangeListener, AudioManager.STREAM_MUSIC, AudioManager.AUDIOFOCUS_GAIN)
		}
	}

	private fun abandonAudioFocus() {
		if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
			audioFocusRequest?.let { audioManager.abandonAudioFocusRequest(it) }
		} else {
			@Suppress("DEPRECATION")
			audioManager.abandonAudioFocus(audioFocusChangeListener)
		}
	}

	/** Switches to narrating `session`, stopping whatever this player was previously doing.
	 * `docKey` scopes the extracted-source cache so it doesn't collide with another document's. */
	fun attach(
		newSession: DocumentSession,
		newDocKey: String
	) {
		stop()
		session = newSession
		docKey = newDocKey
		lastSeekTarget = null
		lastReportedClip = null
	}

	fun detach() {
		stop()
		session = null
		docKey = null
	}

	val hasAudio: Boolean get() = session?.hasAudioFfi() == true

	fun isPlaying(): Boolean = playing

	fun play() {
		if (session == null) return
		requestAudioFocus()
		playing = true
		val pending = pendingTargetMs
		pendingTargetMs = null
		when {
			pending != null -> seekToMs(pending)
			currentSource != null -> {
				try {
					mediaPlayer?.start()
				} catch (_: Exception) {
				}
				onPlaybackStateChanged?.invoke(true)
				startPolling()
			}
			else -> seekToMs(0)
		}
	}

	fun pause() {
		playing = false
		stopPolling()
		mediaPlayer?.let {
			try {
				if (it.isPlaying) it.pause()
			} catch (_: Exception) {
			}
		}
		onPlaybackStateChanged?.invoke(false)
	}

	fun toggle() {
		if (playing) pause() else play()
	}

	/** Stops playback and releases the native player, ahead of switching documents or the
	 * app going away. Only notifies `onPlaybackStateChanged` when there was actually something
	 * to stop: `attach`/`detach` call this on every tab switch, including switches involving
	 * documents that never had DAISY audio attached, and firing unconditionally there would
	 * force TtsManager's paused state on even when it was never speaking. */
	fun stop() {
		val wasActive = playing || session != null
		playing = false
		loadGeneration++
		stopPolling()
		mediaPlayer?.let {
			try {
				it.stop()
			} catch (_: Exception) {
			}
			try {
				it.release()
			} catch (_: Exception) {
			}
		}
		mediaPlayer = null
		currentSource = null
		pendingTargetMs = null
		lastSeekTarget = null
		abandonAudioFocus()
		if (wasActive) onPlaybackStateChanged?.invoke(false)
	}

	/** Seeks playback to the point covering `position` in the text, if the timeline narrates
	 * it. Leaves the transport running or paused as it already was. */
	fun seekToPosition(position: Long): Boolean {
		val session = session ?: return false
		val point = session.audioPointForPositionFfi(position)
		if (!point.found) return false
		return seekToMs(point.timeMs)
	}

	/** Seeks playback to `elapsedMs` into the overall document timeline. While paused this
	 * only records the target, applying it lazily on resume (mirrors desktop), so browsing a
	 * paused book with real navigation doesn't drive a full load per keystroke. */
	fun seekToMs(elapsedMs: Long): Boolean {
		val session = session ?: return false
		if (!playing) {
			pendingTargetMs = elapsedMs
			return true
		}
		val cursor = session.audioCursorAtElapsedFfi(elapsedMs)
		if (!cursor.found) return false
		val clip = session.audioClipFfi(cursor.clipIndex)
		if (!clip.found) return false
		reportClip(cursor.clipIndex)
		val alreadyThere = lastSeekTarget == (clip.source to cursor.seekMs)
		if (alreadyThere) return true
		if (currentSource == clip.source && mediaPlayer != null) {
			applySeek(clip.source, cursor.seekMs)
		} else {
			loadSource(clip.source, cursor.seekMs)
		}
		return true
	}

	private fun applySeek(
		source: Int,
		seekMs: Long
	) {
		lastSeekTarget = source to seekMs
		val player = mediaPlayer ?: return
		try {
			val ms = seekMs.toInt()
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
				player.seekTo(ms.toLong(), MediaPlayer.SEEK_CLOSEST)
			} else {
				player.seekTo(ms)
			}
			if (playing) {
				player.start()
				startPolling()
			} else {
				player.pause()
			}
		} catch (_: Exception) {
		}
	}

	private fun loadSource(
		sourceIndex: Int,
		seekMs: Long
	) {
		val session = session ?: return
		lastSeekTarget = sourceIndex to seekMs
		currentSource = sourceIndex
		stopPolling()
		// A fresh MediaPlayer per source lets Android's async prepare run concurrently;
		// the generation check below discards a superseded load's callback.
		val myGeneration = ++loadGeneration
		val oldPlayer = mediaPlayer
		mediaPlayer = null
		try {
			oldPlayer?.release()
		} catch (_: Exception) {
		}
		// resolveSourcePath can extract a zip-embedded source to disk, so it runs off the
		// main thread; the generation check on return discards a superseded load.
		scope.launch {
			val path = withContext(Dispatchers.IO) { resolveSourcePath(session, sourceIndex) }
			if (path == null || myGeneration != loadGeneration) return@launch
			startPlayer(path, sourceIndex, seekMs, myGeneration)
		}
	}

	/** Clears load/seek state after a source fails to prepare or errors during playback.
	 * Leaving `currentSource`/`lastSeekTarget` pointing at the failed clip would make a retry
	 * to that same position a no-op under the `alreadyThere` check in `seekToMs`, so playback
	 * could never recover without seeking somewhere else first. */
	private fun resetAfterLoadFailure() {
		currentSource = null
		lastSeekTarget = null
		playing = false
		stopPolling()
		onPlaybackStateChanged?.invoke(false)
	}

	private fun startPlayer(
		path: String,
		sourceIndex: Int,
		seekMs: Long,
		myGeneration: Int
	) {
		val player = MediaPlayer()
		try {
			player.setAudioAttributes(speechAudioAttributes())
			player.setDataSource(path)
			player.setOnPreparedListener { mp ->
				if (myGeneration != loadGeneration) {
					try {
						mp.release()
					} catch (_: Exception) {
					}
					return@setOnPreparedListener
				}
				mediaPlayer = mp
				try {
					val ms = seekMs.toInt()
					if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
						mp.seekTo(ms.toLong(), MediaPlayer.SEEK_CLOSEST)
					} else {
						mp.seekTo(ms)
					}
				} catch (_: Exception) {
				}
				if (playing) {
					try {
						mp.start()
					} catch (_: Exception) {
					}
					onPlaybackStateChanged?.invoke(true)
					startPolling()
				}
			}
			player.setOnCompletionListener {
				if (myGeneration == loadGeneration) onSourceCompleted(sourceIndex)
			}
			player.setOnErrorListener { mp, _, _ ->
				if (myGeneration == loadGeneration) {
					if (mediaPlayer === mp) mediaPlayer = null
					resetAfterLoadFailure()
				}
				try {
					mp.release()
				} catch (_: Exception) {
				}
				true
			}
			player.prepareAsync()
		} catch (_: Exception) {
			if (myGeneration == loadGeneration) resetAfterLoadFailure()
			try {
				player.release()
			} catch (_: Exception) {
			}
		}
	}

	/** Auto-advances to whichever source continues the narration, or stops at the end of the
	 * book. Android's analog of desktop's `on_finished` handler. */
	private fun onSourceCompleted(sourceIndex: Int) {
		val session = session ?: return
		val next = session.audioNextSourceAfterFfi(sourceIndex)
		if (next >= 0) {
			loadSource(next, 0)
		} else {
			playing = false
			stopPolling()
			onPlaybackStateChanged?.invoke(false)
		}
	}

	/** Resolves source `index` to a real local file MediaPlayer can play: the document's own
	 * path directly when it isn't zip-embedded, otherwise extracted once and cached. */
	private fun resolveSourcePath(
		session: DocumentSession,
		index: Int
	): String? {
		val direct = session.audioSourceDirectPathFfi(index)
		if (direct.isNotEmpty()) return direct
		val key = docKey ?: return null
		val cacheFile = File(cacheDir, "${key}_$index")
		if (!cacheFile.exists()) {
			cacheDir.mkdirs()
			if (!session.audioExtractSourceFfi(index, cacheFile.absolutePath)) return null
		}
		return cacheFile.absolutePath
	}

	private fun startPolling() {
		pollJob?.cancel()
		pollJob =
			scope.launch {
				while (isActive) {
					delay(POLL_INTERVAL_MS)
					pollTick()
				}
			}
	}

	private fun stopPolling() {
		pollJob?.cancel()
		pollJob = null
	}

	/** Keeps the reported "currently narrating" clip in step with natural playback advance.
	 * The always-on equivalent of desktop's `pump_audio`. */
	private fun pollTick() {
		val session = session ?: return
		val source = currentSource ?: return
		val player = mediaPlayer ?: return
		if (!playing) return
		val rawMs =
			try {
				player.currentPosition.toLong()
			} catch (_: Exception) {
				return
			}
		val elapsed = session.audioElapsedForSourcePositionFfi(source, rawMs)
		if (elapsed < 0) return
		val cursor = session.audioCursorAtElapsedFfi(elapsed)
		if (cursor.found) reportClip(cursor.clipIndex)
	}

	private fun reportClip(clipIndex: Int) {
		if (clipIndex == lastReportedClip) return
		val session = session ?: return
		val clip = session.audioClipFfi(clipIndex)
		if (!clip.found) return
		lastReportedClip = clipIndex
		onClipChanged?.invoke(clip.start)
	}

	/** Where playback would resume right now. `null` means no position has been established
	 * yet; callers must not treat that as "the start", since it would overwrite a stored position. */
	fun resumePointMs(): Long? {
		val session = session
		val source = currentSource
		val player = mediaPlayer
		if (session != null && source != null && player != null) {
			val rawMs =
				try {
					player.currentPosition.toLong()
				} catch (_: Exception) {
					null
				}
			val elapsed = rawMs?.let { session.audioElapsedForSourcePositionFfi(source, it) }
			if (elapsed != null && elapsed >= 0) return elapsed
		}
		return pendingTargetMs
	}

	fun shutdown() {
		stop()
		session = null
		docKey = null
	}
}
