package dev.paperback.android.ui

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class ListScrollPositionTest {
	// The case behind issue #794: the reader picks a chapter in the table of contents, the list
	// is rebuilt at the index the document was opened at, and saving that opening index sends
	// read-aloud back to the start of the book.
	@Test
	fun `the index the list starts at is not saved`() =
		runTest {
			assertEquals(emptyList<Int>(), flowOf(120).scrollsWorthSaving().toList())
		}

	@Test
	fun `a scroll after the starting index is saved`() =
		runTest {
			assertEquals(listOf(140), flowOf(120, 140).scrollsWorthSaving().toList())
		}

	@Test
	fun `only where a scroll came to rest is saved`() =
		runTest {
			val scroll = flow {
				emit(120)
				emit(121)
				emit(122)
				delay(1000)
				emit(200)
				emit(201)
			}
			assertEquals(listOf(122, 201), scroll.scrollsWorthSaving().toList())
		}

	@Test
	fun `a repeated index is reported once`() =
		runTest {
			assertEquals(listOf(140), flowOf(120, 140, 140).scrollsWorthSaving().toList())
		}
}
