package dev.paperback.android.ui.state

import android.content.Context
import android.net.Uri
import androidx.core.net.toUri
import uniffi.paperback.ConfigManagerFfi
import java.io.File

private const val SIDECAR_EXTENSION = "paperback"

/**
 * Moves a document's saved reading data (its position, bookmarks and notes) in and out of a
 * `.paperback` file, so it can travel with the document to another device or another platform.
 *
 * Writing goes through a file in the cache first, then copies that into whatever the system
 * picker handed back: the config layer writes to a real path, and a picked destination is a
 * content URI that only the resolver can open.
 */
class SettingsTransfer(
	private val config: ConfigManagerFfi,
	private val cacheDir: File
) {
	/**
	 * Writes the document's data to the `.paperback` file beside it. Only for a document opened
	 * from a real path: one opened through a content URI has no "beside it" to write to.
	 */
	fun exportToSidecar(documentUri: String): Boolean {
		if (documentUri.startsWith(CONTENT_SCHEME)) return false
		val path = settingsPathFor(documentUri)
		return runCatching { config.exportDocumentSettings(path, sidecarPathFor(path)) }.isSuccess
	}

	fun exportTo(
		context: Context,
		documentUri: String,
		destination: Uri
	): Boolean {
		val temporary = File(cacheDir, "temp_export.$SIDECAR_EXTENSION")
		return try {
			config.exportDocumentSettings(settingsPathFor(documentUri), temporary.absolutePath)
			context.contentResolver.openOutputStream(destination)?.use { out ->
				temporary.inputStream().use { it.copyTo(out) }
			}
			true
		} catch (_: Exception) {
			false
		} finally {
			temporary.delete()
		}
	}

	fun importFrom(
		context: Context,
		documentUri: String,
		source: Uri
	): Boolean {
		val temporary = File(cacheDir, "temp_import.$SIDECAR_EXTENSION")
		return try {
			context.contentResolver.openInputStream(source)?.use { input ->
				temporary.outputStream().use { input.copyTo(it) }
			}
			config.importSettingsFromFile(settingsPathFor(documentUri), temporary.absolutePath)
			true
		} catch (_: Exception) {
			false
		} finally {
			temporary.delete()
		}
	}

	private companion object {
		const val CONTENT_SCHEME = "content://"
	}
}

/**
 * How the config layer identifies a document. A content URI is its own identity, since there is
 * no path behind it to speak of; anything else is identified by the path it points at.
 */
internal fun settingsPathFor(documentUri: String): String =
	if (documentUri.startsWith("content://")) documentUri else documentUri.toUri().path ?: documentUri

/** The `.paperback` file that sits beside a document, carrying the reading data for it. */
internal fun sidecarPathFor(documentPath: String): String {
	val file = File(documentPath)
	return File(file.parentFile, "${file.nameWithoutExtension}.$SIDECAR_EXTENSION").absolutePath
}
