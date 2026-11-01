package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import uniffi.paperback.SegmentTypeFfi

class FormatDurationTest {
	@Test
	fun `under a minute reads as minutes and seconds`() {
		assertEquals("0:00", formatDuration(0))
		assertEquals("0:01", formatDuration(1_000))
		assertEquals("0:59", formatDuration(59_000))
	}

	@Test
	fun `an hour or more grows an hours field`() {
		assertEquals("59:59", formatDuration(3_599_000))
		assertEquals("1:00:00", formatDuration(3_600_000))
		assertEquals("1:01:01", formatDuration(3_661_000))
	}

	// Truncating would make the announced time trail the audio.
	@Test
	fun `milliseconds round to the nearest second`() {
		assertEquals("0:00", formatDuration(499))
		assertEquals("0:01", formatDuration(500))
		assertEquals("1:00", formatDuration(59_500))
	}

	// A relative seek can compute a target before the start of the book before it is clamped.
	@Test
	fun `a negative position reads as zero rather than going backwards`() {
		assertEquals("0:00", formatDuration(-1))
		assertEquals("0:00", formatDuration(-100_000))
	}
}

class NavUnitNameTest {
	@Test
	fun `a seek amount with a preset name uses it`() {
		assertEquals("30 seconds", getSeekAmountName(30))
		assertEquals("1 minute", getSeekAmountName(60))
		assertEquals("1 hour", getSeekAmountName(3600))
	}

	// The saved amount is a plain integer in the shared config, so it can be one no preset covers.
	@Test
	fun `a seek amount with no preset falls back to counting seconds`() {
		assertEquals("45 seconds", getSeekAmountName(45))
	}

	@Test
	fun `each kind of navigation unit names itself`() {
		assertEquals("Section", getNavUnitName(NavUnit.Segment(SegmentTypeFfi.SECTION)))
		assertEquals("30 seconds", getNavUnitName(NavUnit.Time(30)))
		assertEquals("Find", getNavUnitName(NavUnit.Find))
	}

	// The unit chip shows nothing but this name, so a shared one would be indistinguishable.
	@Test
	fun `no two segment types share a name`() {
		val names = SegmentTypeFfi.entries.map { getSegmentTypeName(it) }
		assertEquals(names.size, names.toSet().size)
		assertTrue(names.none { it.isBlank() })
	}
}

class SeekAmountsTest {
	// TalkBack swipes cycle this list, so disorder or a repeat reads as the selector jumping or sticking.
	@Test
	fun `the offered seek amounts are ordered and free of duplicates`() {
		assertEquals(AUDIO_SEEK_AMOUNTS_SECONDS.sorted(), AUDIO_SEEK_AMOUNTS_SECONDS)
		assertEquals(AUDIO_SEEK_AMOUNTS_SECONDS.size, AUDIO_SEEK_AMOUNTS_SECONDS.toSet().size)
		assertTrue(AUDIO_SEEK_AMOUNTS_SECONDS.all { it > 0 })
	}
}
