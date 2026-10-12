package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.NoHandle

private class FindPlaybackConfig : ConfigManagerFfi(NoHandle) {
	val booleans = mutableMapOf<String, Boolean>()
	var flushes = 0

	override fun getAppBool(
		key: String,
		defaultValue: Boolean
	): Boolean = booleans[key] ?: defaultValue

	override fun getAppInt(
		key: String,
		defaultValue: Int
	): Int = defaultValue

	override fun setAppBool(
		key: String,
		value: Boolean
	) {
		booleans[key] = value
	}

	override fun flush() {
		flushes++
	}
}

class FindPlaybackSettingTest {
	@Test
	fun `new readers stop after search matches by default`() {
		assertTrue(ReaderSettings(FindPlaybackConfig()).stopAfterFindMatch.state.value)
	}

	@Test
	fun `continuous reading is saved immediately and restored on the next launch`() {
		val config = FindPlaybackConfig()
		val settings = ReaderSettings(config)
		settings.stopAfterFindMatch.set(false)
		assertFalse(settings.stopAfterFindMatch.state.value)
		assertEquals(false, config.booleans["tts_stop_after_find_match"])
		assertEquals(1, config.flushes)
		assertFalse(ReaderSettings(config).stopAfterFindMatch.state.value)

		settings.stopAfterFindMatch.set(true)
		assertEquals(2, config.flushes)
		assertTrue(ReaderSettings(config).stopAfterFindMatch.state.value)
	}
}
