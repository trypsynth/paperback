package dev.paperback.android.tts

import android.content.Context
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

class DaisyAudioPlayer(private val context: Context) : RecordedPlayer {
	private val scope = CoroutineScope(Dispatchers.Main)
	private val cacheDir = File(context.cacheDir, "paperback_daisy_audio_cache")

	private var session: DocumentSession? = null
	private var docKey: String? = null

	private var mediaPlayer: MediaPlayer? = null
	private var currentSource: Int? = null
	private var playing = false
	private var loadGeneration = 0

	private var pendingTargetMs: Long? = null

	/** So a jump resolving to the same spot doesn't restart audio. */
	private var lastSeekTarget: Pair<Int, Long>? = null

	private var lastReportedClip: Int? = null
	private var pollJob: Job? = null

	var onClipChanged: ((position: Long) -> Unit)? = null

	var onPlaybackStateChanged: ((playing: Boolean) -> Unit)? = null

	/** Fires after the async load a cross-file seek needs, so an announcement can't just use the value that was asked for. */
	var onRelativeSeekLanded: ((elapsedMs: Long) -> Unit)? = null

	/** Only relative seeks report: those following the caret or restoring a saved position have nothing to announce. */
	private var reportNextSeek = false

	/** One speed for every recording; it survives loading the next file. */
	var speed: Float = 1f
		set(value) {
			field = value
			// Only while it plays, since setting a speed on a paused MediaPlayer starts it.
			mediaPlayer?.takeIf { playing && it.isPlaying }?.let { applySpeed(it) }
		}

	private fun applySpeed(player: MediaPlayer) {
		try {
			player.playbackParams = player.playbackParams.setSpeed(speed)
		} catch (_: Exception) {
		}
	}

	/** The speed is set as it starts, which is the one moment setting it can't start a paused player by surprise. */
	private fun startAtSpeed(player: MediaPlayer) {
		applySpeed(player)
		player.start()
	}

	private val audioFocus =
		AudioFocusHolder(context, isPlaying = { playing }, onPause = { pause() }, onResume = { play() })

	/** `newDocKey` scopes the extracted-source cache so it doesn't collide with another document's. */
	fun attach(newSession: DocumentSession, newDocKey: String) {
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

	override fun isPlaying(): Boolean = playing

	override fun play() {
		if (session == null) return
		audioFocus.request()
		playing = true
		val pending = pendingTargetMs
		pendingTargetMs = null
		when {
			pending != null -> seekToMs(pending)
			currentSource != null -> {
				try {
					mediaPlayer?.let { startAtSpeed(it) }
				} catch (_: Exception) {
				}
				onPlaybackStateChanged?.invoke(true)
				startPolling()
			}
			else -> seekToMs(0)
		}
	}

	override fun pause() {
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

	/** Notifies only when something was active: this runs on every tab switch, and firing there would force TtsManager into a paused state it was never in. */
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
		audioFocus.abandon()
		if (wasActive) onPlaybackStateChanged?.invoke(false)
	}

	override fun seekToPosition(position: Long): Boolean {
		val session = session ?: return false
		val point = session.audioPointForPositionFfi(position)
		if (!point.found) return false
		return seekToMs(point.timeMs)
	}

	/**
	 * A paused target in another file is only recorded and applied on resume, so browsing a paused book doesn't load a file per keystroke.
	 * One in the open file is applied even while paused, keeping the decoder's position honest for `seekRelativeMs`, which measures from it.
	 */
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
			// A play() that routed through here still has to get the transport moving.
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

	private fun spillAcrossFileBoundary(session: DocumentSession, deltaMs: Long): Boolean {
		val source = currentSource ?: return false
		val player = mediaPlayer ?: return false
		// With a target recorded but unapplied, the decoder's position is the wrong place to measure from.
		if (pendingTargetMs != null) return false
		val rawMs: Long
		val lengthMs: Long
		try {
			rawMs = player.currentPosition.toLong()
			lengthMs = player.duration.toLong()
		} catch (_: Exception) {
			return false
		}
		return when (val spill = spillOf(rawMs, lengthMs, deltaMs)) {
			is SeekSpill.PastEnd -> {
				val next = session.audioNextSourceAfterFfi(source)
				if (next < 0) return false
				loadSource(next, SourceSeek.FromStart(spill.overflowMs))
				true
			}
			is SeekSpill.BeforeStart -> {
				val previous = session.audioPreviousSourceBeforeFfi(source)
				if (previous < 0) return false
				loadSource(previous, SourceSeek.FromEnd(spill.underflowMs))
				true
			}
			SeekSpill.WithinFile -> false
		}
	}

	private fun resumeLoadedPlayer() {
		try {
			mediaPlayer?.let { startAtSpeed(it) }
		} catch (_: Exception) {
		}
		onPlaybackStateChanged?.invoke(true)
		startPolling()
	}

	private fun applySeek(source: Int, seekMs: Long) {
		val player = mediaPlayer ?: return
		val landedMs = seekWithinPlayer(player, seekMs)
		lastSeekTarget = source to landedMs
		try {
			if (playing) {
				startAtSpeed(player)
				startPolling()
			} else {
				player.pause()
			}
		} catch (_: Exception) {
		}
		reportSeeked(source, landedMs)
	}

	/** Clamped to the file's real length, since a bundle of narration files declares placeholder clip durations hours longer than the recording. */
	private fun seekWithinPlayer(player: MediaPlayer, seekMs: Long): Long {
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

	private fun reportSeeked(source: Int, rawMs: Long) {
		if (!reportNextSeek) return
		reportNextSeek = false
		val session = session ?: return
		val elapsed = session.audioElapsedForSourcePositionFfi(source, rawMs)
		if (elapsed >= 0) onRelativeSeekLanded?.invoke(elapsed)
	}

	/** [FromEnd] is resolved only once the decoder is prepared, since only it knows the real length. */
	private sealed interface SourceSeek {
		data class FromStart(val ms: Long) : SourceSeek

		data class FromEnd(val ms: Long) : SourceSeek
	}

	private fun loadSource(sourceIndex: Int, seek: SourceSeek) {
		val session = session ?: return
		lastSeekTarget = null
		currentSource = sourceIndex
		// So a second seek arriving before the decoder is ready still has something to measure from.
		pendingTargetMs = when (seek) {
			is SourceSeek.FromStart ->
				session.audioElapsedForSourcePositionFfi(sourceIndex, seek.ms).takeIf { it >= 0 }
			is SourceSeek.FromEnd -> null
		}
		stopPolling()
		// The generation check discards a superseded load's prepare callback.
		val myGeneration = ++loadGeneration
		val oldPlayer = mediaPlayer
		mediaPlayer = null
		try {
			oldPlayer?.release()
		} catch (_: Exception) {
		}
		// Off the main thread because a zip-embedded source is extracted to disk.
		scope.launch {
			val path = withContext(Dispatchers.IO) { resolveSourcePath(session, sourceIndex) }
			if (path == null || myGeneration != loadGeneration) return@launch
			startPlayer(path, sourceIndex, seek, myGeneration)
		}
	}

	/** Left pointing at the failed clip, a retry there would be a no-op under `seekToMs`'s same-spot check, and playback could never recover. */
	private fun resetAfterLoadFailure() {
		currentSource = null
		lastSeekTarget = null
		playing = false
		stopPolling()
		onPlaybackStateChanged?.invoke(false)
	}

	private fun startPlayer(path: String, sourceIndex: Int, seek: SourceSeek, myGeneration: Int) {
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
						startAtSpeed(mp)
					} catch (_: Exception) {
					}
					onPlaybackStateChanged?.invoke(true)
					startPolling()
				}
				// The decoder is the authority on the resume point again (see `resumePointMs`).
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

	private fun resolveSourcePath(session: DocumentSession, index: Int): String? {
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

	/** For a seek landing where the poll loop won't visit, a paused one most of all. */
	private fun reportClipAtSourcePosition(source: Int, rawMs: Long) {
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

	/** Null means no position yet; treating it as the start would overwrite a stored position. */
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

internal sealed interface SeekSpill {
	object WithinFile : SeekSpill

	data class PastEnd(val overflowMs: Long) : SeekSpill

	data class BeforeStart(val underflowMs: Long) : SeekSpill
}

/** In the file's own time, not elapsed time: a bundle of narration files gives every clip a placeholder duration far past the recording, so elapsed arithmetic would land back in the same file past its end. */
internal fun spillOf(rawMs: Long, lengthMs: Long, deltaMs: Long): SeekSpill {
	if (lengthMs <= 0) return SeekSpill.WithinFile
	val naiveMs = rawMs + deltaMs
	return when {
		naiveMs > lengthMs -> SeekSpill.PastEnd(naiveMs - lengthMs)
		naiveMs < 0 -> SeekSpill.BeforeStart(-naiveMs)
		else -> SeekSpill.WithinFile
	}
}
