package dev.paperback.android.tts

import java.io.File
import java.io.RandomAccessFile

/** The ends and step of the pause setting, matching iOS. */
internal const val MIN_PARAGRAPH_PAUSE_MS = 0
internal const val MAX_PARAGRAPH_PAUSE_MS = 1000
internal const val PARAGRAPH_PAUSE_STEP_MS = 10

private const val RIFF_HEADER_BYTES = 12L
private const val SIZE_FIELD_OFFSET = 4L
private const val CHUNK_HEADER_BYTES = 8L
private const val FMT_SAMPLE_RATE_OFFSET = 4L
private const val FMT_BLOCK_ALIGN_OFFSET = 12L
private const val MS_PER_SECOND = 1000L

/** A stored pause brought inside the setting's range and onto its steps. */
internal fun paragraphPauseFor(ms: Int): Int {
	val clamped = ms.coerceIn(MIN_PARAGRAPH_PAUSE_MS, MAX_PARAGRAPH_PAUSE_MS)
	return (clamped + PARAGRAPH_PAUSE_STEP_MS / 2) / PARAGRAPH_PAUSE_STEP_MS * PARAGRAPH_PAUSE_STEP_MS
}

/**
 * Adds [ms] of silence to the end of a synthesized WAV file.
 *
 * The engine renders each paragraph to its own file and the players hand off between them with no
 * gap, so silence written into the file is the only place a pause can go. Some voices, Vocalizer
 * among them, end a paragraph so abruptly that the next one runs straight on without it.
 *
 * The file is left alone when it is not PCM WAV with the samples as its last chunk, since the
 * silence could then land somewhere that is not the end of the audio.
 */
internal fun appendSilence(
	file: File,
	ms: Int
) {
	if (ms <= 0) return
	RandomAccessFile(file, "rw").use { wav ->
		if (wav.length() < RIFF_HEADER_BYTES || wav.readTag() != "RIFF") return
		wav.readIntLe()
		if (wav.readTag() != "WAVE") return
		var sampleRate = 0L
		var blockAlign = 0L
		var position = RIFF_HEADER_BYTES
		while (position + CHUNK_HEADER_BYTES <= wav.length()) {
			wav.seek(position)
			val tag = wav.readTag()
			val size = wav.readIntLe().toLong() and 0xFFFFFFFFL
			val body = position + CHUNK_HEADER_BYTES
			if (tag == "fmt ") {
				wav.seek(body + FMT_SAMPLE_RATE_OFFSET)
				sampleRate = wav.readIntLe().toLong()
				wav.seek(body + FMT_BLOCK_ALIGN_OFFSET)
				blockAlign = wav.readShortLe().toLong()
			} else if (tag == "data") {
				if (sampleRate <= 0 || blockAlign <= 0 || body + size != wav.length()) return
				val silence = sampleRate * ms / MS_PER_SECOND * blockAlign
				wav.seek(wav.length())
				wav.write(ByteArray(silence.toInt()))
				wav.seek(position + SIZE_FIELD_OFFSET)
				wav.writeIntLe((size + silence).toInt())
				wav.seek(SIZE_FIELD_OFFSET)
				wav.writeIntLe((wav.length() - CHUNK_HEADER_BYTES).toInt())
				return
			}
			position = body + size + (size and 1)
		}
	}
}

private fun RandomAccessFile.readTag(): String {
	val bytes = ByteArray(4)
	readFully(bytes)
	return String(bytes, Charsets.US_ASCII)
}

private fun RandomAccessFile.readIntLe(): Int = Integer.reverseBytes(readInt())

private fun RandomAccessFile.readShortLe(): Int =
	java.lang.Short
		.reverseBytes(readShort())
		.toInt() and 0xFFFF

private fun RandomAccessFile.writeIntLe(value: Int) = writeInt(Integer.reverseBytes(value))
