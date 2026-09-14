package dev.paperback.android.ui

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.advanceTimeBy
import kotlinx.coroutines.test.runCurrent
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.minutes
import kotlin.time.Duration.Companion.seconds

@OptIn(ExperimentalCoroutinesApi::class)
class SleepTimerTest {
	@Test
	fun `a started timer counts down in seconds`() =
		runTest {
			val timer = SleepTimer(backgroundScope) {}
			timer.start(2)
			runCurrent()
			assertEquals(120, timer.remaining.value)
			advanceTimeBy(30.seconds + 500.milliseconds)
			assertEquals(90, timer.remaining.value)
		}

	@Test
	fun `running out stops playback and reports it`() =
		runTest {
			var stopped = 0
			val timer = SleepTimer(backgroundScope) { stopped++ }
			timer.start(1)
			runCurrent()
			advanceTimeBy(61.seconds)
			assertEquals(1, stopped)
			assertNull(timer.remaining.value)
		}

	@Test
	fun `cancelling stops the countdown without stopping playback`() =
		runTest {
			var stopped = 0
			val timer = SleepTimer(backgroundScope) { stopped++ }
			timer.start(5)
			runCurrent()
			advanceTimeBy(10.seconds)
			timer.cancel()
			advanceTimeBy(10.minutes)
			assertEquals(0, stopped)
			assertNull(timer.remaining.value)
		}

	// Setting a new length while one is running has to replace it. Left running, the first timer
	// would stop playback at its own deadline, well before the length just asked for.
	@Test
	fun `starting a second timer replaces the first`() =
		runTest {
			var stopped = 0
			val timer = SleepTimer(backgroundScope) { stopped++ }
			timer.start(1)
			runCurrent()
			advanceTimeBy(30.seconds + 500.milliseconds)
			timer.start(10)
			runCurrent()
			assertEquals(600, timer.remaining.value)
			advanceTimeBy(40.seconds + 500.milliseconds)
			assertEquals(0, stopped)
			assertEquals(560, timer.remaining.value)
		}
}
