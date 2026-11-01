package dev.paperback.android.ui.state

import android.content.Context
import android.net.Uri
import androidx.core.net.toUri
import uniffi.paperback.ConfigManagerFfi
import java.io.File

private const val SIDECAR_EXTENSION = "paperback"

/** Writes go through a cache file first: the config layer needs a real path, and a picked destination is a content URI only the resolver can open. */
class SettingsTransfer(private val config: ConfigManagerFfi, private val cacheDir: File) {
	fun exportToSidecar(documentUri: String): Boolean {
		if (documentUri.startsWith(CONTENT_SCHEME)) return false
		val path = settingsPathFor(documentUri)
		return runCatching { config.exportDocumentSettings(path, sidecarPathFor(path)) }.isSuccess
	}

	fun exportTo(context: Context, documentUri: String, destination: Uri): Boolean {
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

	fun importFrom(context: Context, documentUri: String, source: Uri): Boolean {
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

internal fun settingsPathFor(documentUri: String): String =
	if (documentUri.startsWith("content://")) documentUri else documentUri.toUri().path ?: documentUri

internal fun sidecarPathFor(documentPath: String): String {
	val file = File(documentPath)
	return File(file.parentFile, "${file.nameWithoutExtension}.$SIDECAR_EXTENSION").absolutePath
}
