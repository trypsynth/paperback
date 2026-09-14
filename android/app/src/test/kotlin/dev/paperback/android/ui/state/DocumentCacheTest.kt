package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder
import java.io.File

class DocumentCacheTest {
	@get:Rule
	val folder = TemporaryFolder()

	// Reopening a document has to land on the directory its last opening used, or every opening
	// leaves another copy of the file behind.
	@Test
	fun `the same document always gets the same directory`() {
		val cache = DocumentCache(folder.root)
		val uri = "content://com.android.providers.downloads/document/42"
		assertEquals(cache.directoryFor(uri), cache.directoryFor(uri))
	}

	@Test
	fun `different documents get different directories`() {
		val cache = DocumentCache(folder.root)
		assertNotEquals(
			cache.directoryFor("content://media/document/1"),
			cache.directoryFor("content://media/document/2")
		)
	}

	// A content URI can hold anything a provider cares to put in it, including characters no file
	// name can carry, so the directory name is a hash rather than the URI itself.
	@Test
	fun `the directory name is plain hex whatever the uri holds`() {
		val name = cacheKeyFor("content://provider/a b/../%20?q=/x:y")
		assertTrue(name.matches(Regex("[0-9a-f]{32}")))
	}

	@Test
	fun `documents sit under one directory of their own`() {
		val cache = DocumentCache(folder.root)
		assertEquals("documents", cache.directoryFor("content://x").parentFile?.name)
	}

	@Test
	fun `purging removes the per-opening directories an older release left`() {
		val legacy = folder.newFolder("2f1c8d4e-9a3b-4c5d-8e7f-0a1b2c3d4e5f")
		File(legacy, "book.epub").writeText("stale")
		val kept = folder.newFolder("documents")
		val other = folder.newFolder("translations")
		DocumentCache(folder.root).purgeLegacy()
		assertFalse(legacy.exists())
		assertTrue(kept.exists())
		assertTrue(other.exists())
	}

	@Test
	fun `purging a cache directory that does not exist is not an error`() {
		DocumentCache(File(folder.root, "missing")).purgeLegacy()
	}
}
