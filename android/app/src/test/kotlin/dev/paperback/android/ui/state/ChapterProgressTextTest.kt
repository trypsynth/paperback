package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.paperback.ChapterProgressFfi

class ChapterProgressTextTest {
	private fun byWords(title: String, percent: Int, wordsLeft: Long) = ChapterProgressFfi(
		found = true,
		title = title,
		percent = percent,
		wordsLeft = wordsLeft,
		elapsedMs = -1,
		totalMs = -1
	)

	@Test
	fun `a text chapter gives its percentage and the words left`() {
		assertEquals("Chapter 3, 42%, 850 words left", chapterProgressText(byWords("Chapter 3", 42, 850)))
	}

	@Test
	fun `one word left is singular`() {
		assertEquals("Chapter 3, 99%, 1 word left", chapterProgressText(byWords("Chapter 3", 99, 1)))
	}

	@Test
	fun `a chapter without a title starts at its percentage`() {
		assertEquals("39%, 6 words left", chapterProgressText(byWords(" ", 39, 6)))
	}

	@Test
	fun `a recorded chapter gives its times`() {
		val progress = ChapterProgressFfi(
			found = true,
			title = "Chapter 3",
			percent = 34,
			wordsLeft = -1,
			elapsedMs = 250_000,
			totalMs = 720_000
		)
		assertEquals("Chapter 3, 4:10 of 12:00, 7:50 left", chapterProgressText(progress))
	}
}
