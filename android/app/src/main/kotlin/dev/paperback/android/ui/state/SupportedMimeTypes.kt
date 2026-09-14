package dev.paperback.android.ui.state

import android.webkit.MimeTypeMap

/**
 * The MIME types Android's own table does not know for the formats Paperback reads. Without
 * these the system file picker greys out perfectly readable books, since it filters on the types
 * it is handed and takes no notice of the extension.
 *
 * Anything Android does know stays out of this table: [mimeTypesFor] asks it first.
 */
private val EXTRA_MIME_TYPES = mapOf(
	"chm" to setOf("application/vnd.ms-htmlhelp"),
	"doc" to setOf("application/msword"),
	"docm" to setOf("application/vnd.ms-word.document.macroEnabled.12"),
	"docx" to setOf("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
	"epub" to setOf("application/epub+zip"),
	"fb2" to setOf("application/x-fictionbook+xml"),
	"fodp" to setOf("application/vnd.oasis.opendocument.presentation"),
	"fodt" to setOf("application/vnd.oasis.opendocument.text"),
	"html" to setOf("text/html"),
	"md" to setOf("text/markdown"),
	"mobi" to setOf("application/x-mobipocket-ebook"),
	"odp" to setOf("application/vnd.oasis.opendocument.presentation"),
	"odt" to setOf("application/vnd.oasis.opendocument.text"),
	"opf" to setOf("application/oebps-package+xml"),
	"pdf" to setOf("application/pdf"),
	"pptx" to setOf("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
	"rtf" to setOf("application/rtf"),
	"txt" to setOf("text/plain"),
	"xml" to setOf("application/xml", "text/xml"),
	"zip" to setOf("application/zip")
)

/** Every MIME type a file with this extension might be offered under. */
internal fun extraMimeTypesFor(extension: String): Set<String> = EXTRA_MIME_TYPES[extension.lowercase()].orEmpty()

/**
 * The types to hand the system file picker so it offers every format this build can read.
 * Falls back to everything rather than nothing, since a picker filtered down to no type at all
 * would leave the reader unable to open anything.
 */
fun mimeTypesFor(extensions: List<String>): Array<String> {
	val mimeMap = MimeTypeMap.getSingleton()
	val mimes = mutableSetOf<String>()
	for (extension in extensions) {
		mimeMap.getMimeTypeFromExtension(extension)?.let { mimes.add(it) }
		mimes += extraMimeTypesFor(extension)
	}
	return if (mimes.isEmpty()) arrayOf("*/*") else mimes.toTypedArray()
}
