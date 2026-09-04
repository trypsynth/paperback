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

	/** Invoked once a `seekRelativeMs` has actually landed, with where it landed in document
	 * elapsed time. Fires after the async load a cross-file seek needs, so a caller that wants
	 * to announce the new position can't just use the value it asked for. Only relative seeks
	 * report: the ones that follow the caret or restore a saved position have nothing to say. */
	var onRelativeSeekLanded: ((elapsedMs: Long) -> Unit)? = null

	/** Set for the duration of one `seekRelativeMs`, so only that seek reports where it lands. */
	private var reportNextSeek = false

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
		reportNextSeek = false
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

	/** Seeks playback to `elapsedMs` into the overall document timeline. A paused target that
	 * lands in some other file is only recorded, and applied on resume (mirrors desktop), so
	 * browsing a paused book with real navigation doesn't drive a full load per keystroke. A
	 * target inside the file already open is applied straight away even while paused: it costs
	 * nothing, and it keeps the decoder's reported position honest for `seekRelativeMs`, which
	 * measures from it. */
	fun seekToMs(elapsedMs: Long): Boolean {
		val session = session ?: return false
		val cursor = session.audioCursorAtElapsedFfi(elapsedMs)
		if (!cursor.found) return false
		val clip = session.audioClipFfi(cursor.clipIndex)
		if (!clip.found) return false
		if (!playing && (currentSource != clip.source || mediaPlayer == null)) {
			pendingTargetMs = elapsedMs
			reportClip(cursor.clipIndex)
			if (reportNextSeek) {
				reportNextSeek = false
				onRelativeSeekLanded?.invoke(elapsedMs)
			}
			return true
		}
		pendingTargetMs = null
		reportClip(cursor.clipIndex)
		if (lastSeekTarget == (clip.source to cursor.seekMs)) {
			// Same spot: don't restart the audio, but a play() that routed through here still
			// has to get the transport moving.
			if (playing) resumeLoadedPlayer()
			return true
		}
		if (currentSource == clip.source && mediaPlayer != null) {
			applySeek(clip.source, cursor.seekMs)
		} else {
			loadSource(clip.source, SourceSeek.FromStart(cursor.seekMs))
		}
		return true
	}

	/** Moves playback `deltaMs` from wherever it is now, the time-unit equivalent of stepping
	 * by paragraph in a text document.
	 *
	 * A seek that runs off either end of the file now playing continues into its neighbour,
	 * measured against that file's own real length. It has to work that way rather than through
	 * elapsed time: an audiobook that is just a bundle of narration files gives every clip the
	 * same placeholder duration, far longer than the recording (see
	 * `build_plain_audio_zip_document`), so elapsed-time arithmetic would resolve back into the
	 * same file past its end, where the native seek only clamps. */
	fun seekRelativeMs(deltaMs: Long): Boolean {
		val session = session ?: return false
		reportNextSeek = true
		if (spillAcrossFileBoundary(session, deltaMs)) return true
		val current = resumePointMs() ?: 0L
		val target = if (deltaMs >= 0) {
			(current + deltaMs).coerceAtMost(session.audioTotalDurationMsFfi())
		} else {
			(current + deltaMs).coerceAtLeast(0L)
		}
		val seeked = seekToMs(target)
		if (!seeked) reportNextSeek = false
		return seeked
	}

	/** Handles the part of `seekRelativeMs` that leaves the current file, loading the
	 * neighbouring source at the leftover offset. False when the seek stays inside this file,
	 * when there's no neighbour to spill into, or when no decoder is loaded to measure against. */
	private fun spillAcrossFileBoundary(
		session: DocumentSession,
		deltaMs: Long
	): Boolean {
		val source = currentSource ?: return false
		val player = mediaPlayer ?: return false
		// A recorded-but-unapplied target means the loaded decoder isn't where we logically
		// are, so its reported position is the wrong thing to measure a relative seek from.
		if (pendingTargetMs != null) return false
		val rawMs: Long
		val lengthMs: Long
		try {
			rawMs = player.currentPosition.toLong()
			lengthMs = player.duration.toLong()
		} catch (_: Exception) {
			return false
		}
		if (lengthMs <= 0) return false
		val naiveMs = rawMs + deltaMs
		return when {
			naiveMs > lengthMs -> {
				val next = session.audioNextSourceAfterFfi(source)
				if (next < 0) return false
				loadSource(next, SourceSeek.FromStart(naiveMs - lengthMs))
				true
			}
			naiveMs < 0 -> {
				val previous = session.audioPreviousSourceBeforeFfi(source)
				if (previous < 0) return false
				loadSource(previous, SourceSeek.FromEnd(-naiveMs))
				true
			}
			else -> false
		}
	}

	private fun resumeLoadedPlayer() {
		try {
			mediaPlayer?.start()
		} catch (_: Exception) {
		}
		onPlaybackStateChanged?.invoke(true)
		startPolling()
	}

	private fun applySeek(
		source: Int,
		seekMs: Long
	) {
		val player = mediaPlayer ?: return
		val landedMs = seekWithinPlayer(player, seekMs)
		lastSeekTarget = source to landedMs
		try {
			if (playing) {
				player.start()
				startPolling()
			} else {
				player.pause()
			}
		} catch (_: Exception) {
		}
		reportSeeked(source, landedMs)
	}

	/** Seeks `player` to `seekMs`, clamped to the file's real length, and reports where it
	 * actually went. The clamp matters for a bundle of narration files, whose clips declare a
	 * placeholder duration hours longer than the recording. */
	private fun seekWithinPlayer(
		player: MediaPlayer,
		seekMs: Long
	): Long {
		val lengthMs = try {
			player.duration.toLong()
		} catch (_: Exception) {
			0L
		}
		val target = if (lengthMs > 0) seekMs.coerceIn(0L, lengthMs) else seekMs.coerceAtLeast(0L)
		try {
			if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
				player.seekTo(target, MediaPlayer.SEEK_CLOSEST)
			} else {
				player.seekTo(target.toInt())
			}
		} catch (_: Exception) {
		}
		return target
	}

	private fun reportSeeked(
		source: Int,
		rawMs: Long
	) {
		if (!reportNextSeek) return
		reportNextSeek = false
		val session = session ?: return
		val elapsed = session.audioElapsedForSourcePositionFfi(source, rawMs)
		if (elapsed >= 0) onRelativeSeekLanded?.invoke(elapsed)
	}

	/** Where in a source to start: an absolute offset, or a distance back from its real end,
	 * which only a prepared decoder knows. */
	private sealed interface SourceSeek {
		data class FromStart(
			val ms: Long
		) : SourceSeek

		data class FromEnd(
			val ms: Long
		) : SourceSeek
	}

	private fun loadSource(
		sourceIndex: Int,
		seek: SourceSeek
	) {
		val session = session ?: return
		lastSeekTarget = null
		currentSource = sourceIndex
		// Where we are between here and the decoder being ready, so a second seek arriving in
		// that window still has something to measure from. A distance back from the end has no
		// answer until the file's real length is known, so it stays unrecorded until then.
		pendingTargetMs = when (seek) {
			is SourceSeek.FromStart ->
				session.audioElapsedForSourcePositionFfi(sourceIndex, seek.ms).takeIf { it >= 0 }
			is SourceSeek.FromEnd -> null
		}
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
			startPlayer(path, sourceIndex, seek, myGeneration)
		}
	}

	/** Clears load/seek state after a source fails to prepare or errors during playback.
	 * Leaving `currentSource`/`lastSeekTarget` pointing at the failed clip would make a retry
	 * to that same position a no-op under the same-spot check in `seekToMs`, so playback could
	 * never recover without seeking somewhere else first. */
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
		seek: SourceSeek,
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
				val requestedMs = when (seek) {
					is SourceSeek.FromStart -> seek.ms
					is SourceSeek.FromEnd -> {
						val lengthMs = try {
							mp.duration.toLong()
						} catch (_: Exception) {
							0L
						}
						lengthMs - seek.ms
					}
				}
				val landedMs = seekWithinPlayer(mp, requestedMs)
				lastSeekTarget = sourceIndex to landedMs
				if (playing) {
					try {
						mp.start()
					} catch (_: Exception) {
					}
					onPlaybackStateChanged?.invoke(true)
					startPolling()
				}
				// The decoder is now sitting where it was asked to, so it is the authority on
				// the resume point again (see `resumePointMs`).
				pendingTargetMs = null
				reportClipAtSourcePosition(sourceIndex, landedMs)
				reportSeeked(sourceIndex, landedMs)
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
			loadSource(next, SourceSeek.FromStart(0))
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

	/** Reports whichever clip covers `rawMs` in `source`, for a seek that landed somewhere the
	 * poll loop won't visit on its own (a paused one, most of all). */
	private fun reportClipAtSourcePosition(
		source: Int,
		rawMs: Long
	) {
		val session = session ?: return
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
