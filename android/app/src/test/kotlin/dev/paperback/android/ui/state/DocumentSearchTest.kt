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

	// Callers check one and use the other, so a half-cleared search breaks them.
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

	// As a new search, Find Previous would start from the current match rather than stepping off it.
	@Test
	fun `searching the other way is still the same search`() {
		val search = DocumentSearch()
		search.start("needle", SearchOptionsFfi(false, false, false, true))
		assertTrue(search.isSameAs("needle", SearchOptionsFfi(false, false, false, false)))
	}

	// MainScreen collects each request as it arrives, so the flow needs no more buffer than this.
	@Test
	fun `stepping reports the direction it was asked for`() = runTest {
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
