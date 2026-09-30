package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.paperback.TocEntry

class SectionTitleTest {
	private val toc = listOf(
		TocEntry("Preface", 100, 0),
		TocEntry("Chapter One", 500, 0),
		TocEntry("A Digression", 700, 1),
		TocEntry("Chapter Two", 900, 0)
	)

	@Test
	fun `a position inside a section is named by it`() {
		assertEquals("Chapter One", sectionTitleAt(toc, 600))
		assertEquals("A Digression", sectionTitleAt(toc, 800))
		assertEquals("Chapter Two", sectionTitleAt(toc, 5000))
	}

	@Test
	fun `a position exactly on a section start is inside it`() {
		assertEquals("Chapter One", sectionTitleAt(toc, 500))
	}

	// Front matter before the first heading belongs to no section. Naming the first one there
	// would announce a chapter the reader has not reached.
	@Test
	fun `a position before every section is named by none`() {
		assertEquals("", sectionTitleAt(toc, 0))
		assertEquals("", sectionTitleAt(toc, 99))
	}

	@Test
	fun `a document with no contents names no section`() {
		assertEquals("", sectionTitleAt(emptyList(), 400))
	}

	// Nested entries are still in reading order, so the deepest one the reader has passed is the
	// one they are in, not the chapter above it.
	@Test
	fun `a nested section wins over the chapter it sits in`() {
		assertEquals("A Digression", sectionTitleAt(toc, 750))
	}
}
