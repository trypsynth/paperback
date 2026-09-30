package dev.paperback.android.tts

import org.junit.Assert.assertEquals
import org.junit.Test

class SeekSpillTest {
	@Test
	fun `a seek landing inside the file stays in it`() {
		assertEquals(SeekSpill.WithinFile, spillOf(rawMs = 30_000, lengthMs = 45_000, deltaMs = 10_000))
	}

	@Test
	fun `landing exactly on the end still counts as inside`() {
		assertEquals(SeekSpill.WithinFile, spillOf(rawMs = 35_000, lengthMs = 45_000, deltaMs = 10_000))
	}

	@Test
	fun `the part of a forward seek past the end belongs to the next file`() {
		assertEquals(SeekSpill.PastEnd(5_000), spillOf(rawMs = 40_000, lengthMs = 45_000, deltaMs = 10_000))
	}

	@Test
	fun `the part of a backward seek before the start belongs to the previous file`() {
		assertEquals(SeekSpill.BeforeStart(7_000), spillOf(rawMs = 3_000, lengthMs = 45_000, deltaMs = -10_000))
	}

	@Test
	fun `landing exactly on the start stays in this file`() {
		assertEquals(SeekSpill.WithinFile, spillOf(rawMs = 10_000, lengthMs = 45_000, deltaMs = -10_000))
	}

	// The decoder reports no length until it has prepared. Spilling on that would send the reader
	// to another file over a length of zero, which every forward seek runs past.
	@Test
	fun `a file of unknown length can only be seeked within`() {
		assertEquals(SeekSpill.WithinFile, spillOf(rawMs = 0, lengthMs = 0, deltaMs = 10_000))
		assertEquals(SeekSpill.WithinFile, spillOf(rawMs = 0, lengthMs = -1, deltaMs = 10_000))
	}

	// A seek amount can be set as high as an hour, over narration files of a few minutes.
	@Test
	fun `a seek far past the end reports the whole overflow`() {
		assertEquals(SeekSpill.PastEnd(3_555_000), spillOf(rawMs = 0, lengthMs = 45_000, deltaMs = 3_600_000))
	}
}
