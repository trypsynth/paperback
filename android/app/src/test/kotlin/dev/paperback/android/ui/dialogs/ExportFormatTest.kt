package dev.paperback.android.ui.dialogs

import org.junit.Assert.assertEquals
import org.junit.Test
import uniffi.paperback.ExportFormat

class ExportFormatTest {
	// A mismatched extension hands the reader a file their system opens with the wrong app.
	@Test
	fun `every format has the extension its content is written in`() {
		assertEquals("txt", ExportFormat.TEXT.fileExtension())
		assertEquals("html", ExportFormat.HTML.fileExtension())
		assertEquals("md", ExportFormat.MARKDOWN.fileExtension())
	}

	@Test
	fun `every format known to the core has an extension of its own`() {
		val extensions = ExportFormat.entries.map { it.fileExtension() }
		assertEquals(ExportFormat.entries.size, extensions.distinct().size)
	}
}
