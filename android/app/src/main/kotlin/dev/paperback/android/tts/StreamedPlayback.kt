package dev.paperback.android.tts

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.os.Handler
import android.os.Looper
import android.util.Log

private const val TAG = "PaperbackTts"

/**
 * One streaming track so speech starts on the engine's first chunk, paragraphs follow with no gap, and a pause resumes on the exact sample.
 * Engine callbacks arrive on binder threads and a feeder thread writes, hence the lock; [onSegmentPlayed] runs on the main thread.
 */
internal class StreamedPlayback(
	private val audioAttributes: AudioAttributes,
	private val onSegmentPlayed: (String) -> Unit
) {
	private data class Format(val sampleRateHz: Int, val encoding: Int, val channels: Int) {
		val bytesPerFrame: Int
			get() = channels * when (encoding) {
				AudioFormat.ENCODING_PCM_8BIT -> 1
				AudioFormat.ENCODING_PCM_FLOAT -> 4
				else -> 2
			}

		fun bytesFor(ms: Long): Long = if (ms <= 0) 0 else ms * sampleRateHz / 1000 * bytesPerFrame
	}

	private class Segment(val id: String, val skipMs: Long, val pauseMs: () -> Int) {
		var format: Format? = null
		val chunks = ArrayDeque<ByteArray>()
		var done = false
		var skipBytes = -1L
		var silenceWritten = false
	}

	private class End(val id: String, val frame: Long)

	// wait() and notifyAll() are only on java.lang.Object, which Kotlin's Any does not expose.
	@Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN")
	private val lock = Object()
	private val segments = ArrayDeque<Segment>()
	private val ends = ArrayDeque<End>()
	private var track: AudioTrack? = null
	private var trackFormat: Format? = null
	private var framesWritten = 0L
	private var lastHeardEnd = 0L

	// Bumped whenever the track is dropped, so a write already under way for the old one stops.
	private var generation = 0
	private var wantPlaying = true
	private var closed = false
	private val main = Handler(Looper.getMainLooper())
	private var polling = false
	private val feeder = Thread(::feed, "PaperbackTtsFeeder").apply {
		isDaemon = true
		start()
	}

	/** [skipMs] is for carrying on part-way through a paragraph made again at a new rate. */
	fun enqueue(id: String, pauseMs: () -> Int, skipMs: Long = 0) {
		synchronized(lock) {
			segments.addLast(Segment(id, skipMs, pauseMs))
			lock.notifyAll()
		}
	}

	fun remove(id: String) {
		synchronized(lock) {
			val segment = segments.firstOrNull { it.id == id } ?: return
			if (segment !== segments.first()) segments.remove(segment)
		}
	}

	fun begin(id: String, sampleRateHz: Int, encoding: Int, channels: Int) {
		synchronized(lock) {
			segments.firstOrNull { it.id == id }?.format = Format(sampleRateHz, encoding, channels)
			lock.notifyAll()
		}
	}

	fun audio(id: String, bytes: ByteArray) {
		synchronized(lock) {
			segments.firstOrNull { it.id == id }?.chunks?.addLast(bytes) ?: return
			lock.notifyAll()
		}
	}

	/** Called for failed segments too, so the queue never waits on one. */
	fun done(id: String) {
		synchronized(lock) {
			segments.firstOrNull { it.id == id }?.done = true
			lock.notifyAll()
		}
	}

	fun playedMsOfCurrent(): Long? {
		synchronized(lock) {
			val format = trackFormat ?: return null
			val head = headFrame() ?: return null
			val playing = ends.indexOfFirst { it.frame > head }
			val start = when {
				playing > 0 -> ends[playing - 1].frame
				playing == 0 -> lastHeardEnd
				else -> ends.lastOrNull()?.frame ?: lastHeardEnd
			}
			return (head - start).coerceAtLeast(0) * 1000 / format.sampleRateHz
		}
	}

	fun isIdle(): Boolean = synchronized(lock) { segments.isEmpty() && ends.isEmpty() }

	fun pause() {
		synchronized(lock) {
			wantPlaying = false
			track?.pause()
		}
	}

	fun play() {
		synchronized(lock) {
			wantPlaying = true
			track?.play()
			lock.notifyAll()
		}
		startPolling()
	}

	fun clear() {
		synchronized(lock) {
			segments.clear()
			ends.clear()
			releaseTrack()
			lock.notifyAll()
		}
	}

	fun close() {
		synchronized(lock) {
			closed = true
			segments.clear()
			ends.clear()
			releaseTrack()
			lock.notifyAll()
		}
		feeder.interrupt()
	}

	// Called with the lock held.
	private fun releaseTrack() {
		try {
			track?.pause()
			track?.flush()
			track?.release()
		} catch (_: Exception) {
		}
		track = null
		trackFormat = null
		framesWritten = 0
		lastHeardEnd = 0
		generation++
	}

	// The head counts frames as an unsigned 32-bit number.
	private fun headFrame(): Long? = track?.let { it.playbackHeadPosition.toLong() and 0xFFFFFFFFL }

	private fun feed() {
		while (true) {
			val next = synchronized(lock) {
				while (!closed && nothingToWrite()) {
					try {
						lock.wait()
					} catch (_: InterruptedException) {
					}
				}
				if (closed) return
				takeNextBytes()
			}
			if (next == null) {
				if (!rest()) return
			} else if (next.isNotEmpty()) {
				writeFully(next)
			}
		}
	}

	private fun rest(): Boolean = try {
		synchronized(lock) {
			// A paused track takes nothing, so retrying it would only wake the CPU until play.
			val before = generation
			while (!wantPlaying && !closed && generation == before) lock.wait()
		}
		Thread.sleep(WAIT_MS)
		true
	} catch (_: InterruptedException) {
		!closed
	}

	private fun writeFully(bytes: ByteArray) {
		var offset = 0
		while (offset < bytes.size) {
			val written = synchronized(lock) {
				val target = track ?: return
				val count = target.write(bytes, offset, bytes.size - offset, AudioTrack.WRITE_NON_BLOCKING)
				if (count > 0) framesWritten += count / (trackFormat?.bytesPerFrame ?: 1)
				count
			}
			if (written < 0) return
			offset += written
			if (offset < bytes.size) {
				val before = synchronized(lock) { generation }
				if (!rest()) return
				if (synchronized(lock) { generation } != before) return
			}
		}
	}

	// Called with the lock held.
	private fun nothingToWrite(): Boolean {
		val segment = segments.firstOrNull() ?: return true
		return segment.chunks.isEmpty() && !segment.done
	}

	// Called with the lock held. Empty means the segment just finished; null means wait and ask again.
	private fun takeNextBytes(): ByteArray? {
		val segment = segments.first()
		val format = segment.format
		if (format != null && !openTrackFor(format)) return null
		if (segment.chunks.isNotEmpty() && format != null) {
			var chunk = segment.chunks.removeFirst()
			if (segment.skipBytes < 0) segment.skipBytes = format.bytesFor(segment.skipMs)
			if (segment.skipBytes > 0) {
				val dropped = minOf(segment.skipBytes, chunk.size.toLong()).toInt()
				segment.skipBytes -= dropped
				chunk = chunk.copyOfRange(dropped, chunk.size)
			}
			return chunk
		}
		if (!segment.done) return null
		if (!segment.silenceWritten && format != null) {
			segment.silenceWritten = true
			val silence = ByteArray(format.bytesFor(segment.pauseMs().toLong()).toInt())
			if (format.encoding == AudioFormat.ENCODING_PCM_8BIT) silence.fill(0x80.toByte())
			if (silence.isNotEmpty()) return silence
		}
		segments.removeFirst()
		ends.addLast(End(segment.id, framesWritten))
		main.post(::startPolling)
		return ByteArray(0)
	}

	// Called with the lock held. A new format after a voice change waits for the old track to play out before getting its own.
	private fun openTrackFor(format: Format): Boolean {
		val current = track
		if (current != null && trackFormat == format) return true
		if (current != null) {
			if ((headFrame() ?: 0) < framesWritten || ends.isNotEmpty()) return false
			releaseTrack()
		}
		val channelMask = if (format.channels == 2) AudioFormat.CHANNEL_OUT_STEREO else AudioFormat.CHANNEL_OUT_MONO
		val minBuffer = AudioTrack.getMinBufferSize(format.sampleRateHz, channelMask, format.encoding)
		return try {
			val made = AudioTrack
				.Builder()
				.setAudioAttributes(audioAttributes)
				.setAudioFormat(
					AudioFormat
						.Builder()
						.setSampleRate(format.sampleRateHz)
						.setEncoding(format.encoding)
						.setChannelMask(channelMask)
						.build()
				).setTransferMode(AudioTrack.MODE_STREAM)
				.setBufferSizeInBytes(maxOf(minBuffer, format.bytesFor(BUFFER_MS).toInt()))
				.build()
			track = made
			trackFormat = format
			if (wantPlaying) made.play()
			true
		} catch (e: Exception) {
			Log.e(TAG, "could not open an audio track for $format", e)
			false
		}
	}

	private fun startPolling() {
		if (polling || closed) return
		polling = true
		main.post(::poll)
	}

	private fun poll() {
		val heard = ArrayList<String>()
		val keepGoing = synchronized(lock) {
			// A segment that never opened a track, one that failed before any audio, has nothing to wait for.
			val head = headFrame()
			while (ends.isNotEmpty() && (head == null || head >= ends.first().frame)) {
				val end = ends.removeFirst()
				lastHeardEnd = end.frame
				heard.add(end.id)
			}
			!closed && wantPlaying && (ends.isNotEmpty() || segments.isNotEmpty())
		}
		heard.forEach(onSegmentPlayed)
		if (keepGoing) main.postDelayed(::poll, POLL_MS) else polling = false
	}

	private companion object {
		const val POLL_MS = 40L
		const val WAIT_MS = 10L

		/** Rides out a busy moment on the device, short enough that a pause or stop isn't waiting on it. */
		const val BUFFER_MS = 250L
	}
}
