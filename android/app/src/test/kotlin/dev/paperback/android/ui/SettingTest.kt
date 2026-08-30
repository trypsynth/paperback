package dev.paperback.android.ui

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SettingTest {
	@Test
	fun `the stored value is what the setting starts on`() {
		val setting = Setting(7) { }
		assertEquals(7, setting.state.value)
	}

	@Test
	fun `setting a value both publishes and persists it`() {
		val persisted = mutableListOf<Int>()
		val setting = Setting(1) { persisted.add(it) }
		setting.set(2)
		assertEquals(2, setting.state.value)
		assertEquals(listOf(2), persisted)
	}

	// The whole reason persist runs per write rather than at some checkpoint: Android can kill
	// the process between two changes, and the second one must not take the first down with it.
	@Test
	fun `every write is persisted, not just the last`() {
		val persisted = mutableListOf<Int>()
		val setting = Setting(0) { persisted.add(it) }
		setting.set(1)
		setting.set(2)
		setting.set(3)
		assertEquals(listOf(1, 2, 3), persisted)
	}

	@Test
	fun `a write outside the accepted range is clamped before it is stored`() {
		val persisted = mutableListOf<Int>()
		val setting = Setting(100, { it.coerceIn(70, 300) }) { persisted.add(it) }
		setting.set(1000)
		assertEquals(300, setting.state.value)
		assertEquals(listOf(300), persisted)
	}

	// A config file written by a different version (or hand-edited) can hold anything at all.
	// Sanitizing only on write would hand that raw value straight to the UI, which for the text
	// scale means a slider pinned outside its own range.
	@Test
	fun `a stored value outside the accepted range is clamped too`() {
		val setting = Setting(9000, { it.coerceIn(70, 300) }) { }
		assertEquals(300, setting.state.value)
	}

	// SettingsScreen builds its slider as MIN..MAX in whole steps, so a MAX the steps can't
	// land on exactly would leave the largest text size unreachable by the slider.
	@Test
	fun `the text scale range divides evenly into steps`() {
		val span = ReaderSettings.MAX_TEXT_SCALE_PERCENT - ReaderSettings.MIN_TEXT_SCALE_PERCENT
		assertEquals(0, span % ReaderSettings.TEXT_SCALE_PERCENT_STEP)
		assertTrue(ReaderSettings.MIN_TEXT_SCALE_PERCENT < ReaderSettings.MAX_TEXT_SCALE_PERCENT)
	}
}
