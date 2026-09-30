package dev.paperback.android.tts

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test
import java.io.File
import java.nio.ByteBuffer
import java.nio.ByteOrder

private const val SAMPLE_RATE = 22050
private const val BLOCK_ALIGN = 2

private fun wav(
	samples: ByteArray,
	trailingChunk: Boolean = false
): File {
	val extra = if (trailingChunk) 12 else 0
	val buffer = ByteBuffer.allocate(44 + samples.size + extra).order(ByteOrder.LITTLE_ENDIAN)
	buffer.put("RIFF".toByteArray()).putInt(36 + samples.size + extra).put("WAVE".toByteArray())
	buffer
		.put("fmt ".toByteArray())
		.putInt(16)
		.putShort(1)
		.putShort(1)
	buffer
		.putInt(SAMPLE_RATE)
		.putInt(SAMPLE_RATE * BLOCK_ALIGN)
		.putShort(BLOCK_ALIGN.toShort())
		.putShort(16)
	buffer.put("data".toByteArray()).putInt(samples.size).put(samples)
	if (trailingChunk) buffer.put("LIST".toByteArray()).putInt(4).put("INFO".toByteArray())
	return File.createTempFile("pause", ".wav").apply {
		deleteOnExit()
		writeBytes(buffer.array())
	}
}

private fun intAt(
	bytes: ByteArray,
	offset: Int
): Int =
	ByteBuffer
		.wrap(bytes, offset, 4)
		.order(ByteOrder.LITTLE_ENDIAN)
		.int

class WavSilenceTest {
	@Test
	fun `silence is added after the samples and both sizes grow to cover it`() {
		val samples = byteArrayOf(1, 2, 3, 4)
		val file = wav(samples)
		appendSilence(file, 100)
		val bytes = file.readBytes()
		val silence = SAMPLE_RATE / 10 * BLOCK_ALIGN
		assertEquals(44 + samples.size + silence, bytes.size)
		assertEquals(bytes.size - 8, intAt(bytes, 4))
		assertEquals(samples.size + silence, intAt(bytes, 40))
		assertArrayEquals(samples, bytes.copyOfRange(44, 48))
		assertEquals(0, bytes.drop(48).count { it != 0.toByte() })
	}

	@Test
	fun `no pause leaves the file alone`() {
		val file = wav(byteArrayOf(1, 2))
		val before = file.readBytes()
		appendSilence(file, 0)
		assertArrayEquals(before, file.readBytes())
	}

	@Test
	fun `a chunk after the samples leaves the file alone`() {
		val file = wav(byteArrayOf(1, 2), trailingChunk = true)
		val before = file.readBytes()
		appendSilence(file, 100)
		assertArrayEquals(before, file.readBytes())
	}

	@Test
	fun `a file that is not WAV is left alone`() {
		val file = File.createTempFile("pause", ".wav").apply {
			deleteOnExit()
			writeBytes("not audio at all".toByteArray())
		}
		appendSilence(file, 100)
		assertEquals("not audio at all", file.readText())
	}

	@Test
	fun `a stored pause is brought onto the setting's steps and inside its range`() {
		assertEquals(0, paragraphPauseFor(-50))
		assertEquals(250, paragraphPauseFor(247))
		assertEquals(1000, paragraphPauseFor(5000))
	}
}
