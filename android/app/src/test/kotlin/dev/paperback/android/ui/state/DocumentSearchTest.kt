package dev.paperback.android.ui.state

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runCurrent
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import uniffi.paperback.SearchOptionsFfi

@OptIn(ExperimentalCoroutinesApi::class)
class DocumentSearchTest {
	private fun options(matchCase: Boolean = false) = SearchOptionsFfi(matchCase, false, false, true)

	@Test
	fun `nothing is being searched for until a search starts`() {
		val search = DocumentSearch()
		assertNull(search.query.value)
		assertNull(search.options.value)
	}

	@Test
	fun `starting a search publishes the query and its options`() {
		val search = DocumentSearch()
		search.start("needle", options(matchCase = true))
		assertEquals("needle", search.query.value)
		assertEquals(true, search.options.value?.matchCase)
	}

	@Test
	fun `starting a second search replaces the first`() {
		val search = DocumentSearch()
		search.start("first", options())
		search.start("second", options())
		assertEquals("second", search.query.value)
	}

	// Both go together: a query with no options (or the reverse) would leave every caller that
	// checks one and uses the other reading a search that is only half gone.
	@Test
	fun `clearing drops the query and the options together`() {
		val search = DocumentSearch()
		search.start("needle", options())
		search.clear()
		assertNull(search.query.value)
		assertNull(search.options.value)
	}

	@Test
	fun `nothing matches the same search when none is running`() {
		assertFalse(DocumentSearch().isSameAs("needle", options()))
	}

	@Test
	fun `the running search matches itself`() {
		val search = DocumentSearch()
		search.start("needle", options())
		assertTrue(search.isSameAs("needle", options()))
	}

	@Test
	fun `a different query or option is not the same search`() {
		val search = DocumentSearch()
		search.start("needle", options())
		assertFalse(search.isSameAs("haystack", options()))
		assertFalse(search.isSameAs("needle", options(matchCase = true)))
	}

	// Find Previous runs the same search backward. Counting that as a new search would start it
	// from the match the reader is on rather than stepping off it.
	@Test
	fun `searching the other way is still the same search`() {
		val search = DocumentSearch()
		search.start("needle", SearchOptionsFfi(false, false, false, true))
		assertTrue(search.isSameAs("needle", SearchOptionsFfi(false, false, false, false)))
	}

	// One step is carried out before the next is asked for, which is also why the flow needs no
	// more buffer than it has: MainScreen collects each request as it arrives.
	@Test
	fun `stepping reports the direction it was asked for`() =
		runTest {
			val search = DocumentSearch()
			val seen = mutableListOf<Boolean>()
			val collecting = backgroundScope.launch { search.stepRequests.collect { seen += it } }
			runCurrent()
			search.findNext()
			runCurrent()
			search.findPrevious()
			runCurrent()
			collecting.cancel()
			assertEquals(listOf(true, false), seen)
		}
}
