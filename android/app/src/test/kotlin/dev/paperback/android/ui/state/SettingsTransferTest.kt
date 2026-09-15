package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Test
import java.io.File

class SettingsTransferTest {
	private fun path(vararg parts: String) = File(parts.joinToString(File.separator)).absolutePath

	@Test
	fun `the sidecar sits beside the document under its own name`() {
		assertEquals(path("books", "Dune.paperback"), sidecarPathFor(path("books", "Dune.epub")))
	}

	// Only the last extension is replaced. A name carrying dots of its own keeps them, or the
	// sidecar would be looked for under a name the export never wrote.
	@Test
	fun `only the final extension is replaced`() {
		assertEquals(path("books", "Vol.2.Dune.paperback"), sidecarPathFor(path("books", "Vol.2.Dune.epub")))
	}

	@Test
	fun `a document with no extension still gets a sidecar`() {
		assertEquals(path("books", "README.paperback"), sidecarPathFor(path("books", "README")))
	}

	@Test
	fun `a sidecar of a sidecar is itself`() {
		val sidecar = sidecarPathFor(path("books", "Dune.epub"))
		assertEquals(sidecar, sidecarPathFor(sidecar))
	}
}
