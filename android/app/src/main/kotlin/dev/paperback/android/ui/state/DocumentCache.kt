package dev.paperback.android.ui.state

import android.content.ContentResolver
import android.net.Uri
import android.provider.OpenableColumns
import java.io.File
import java.security.MessageDigest

private const val DOCUMENT_DIR = "documents"
private const val KEY_BYTES = 16

private val UUID_DIR_REGEX = Regex("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")

/** Named after the URI rather than at random, so reopening a document reuses its directory instead of filling the cache with a copy per opening. */
class DocumentCache(private val cacheRoot: File) {
	fun directoryFor(uriString: String): File = File(cacheRoot, "$DOCUMENT_DIR/${cacheKeyFor(uriString)}")

	@Synchronized
	fun materialize(resolver: ContentResolver, uri: Uri, reuse: Boolean): File? {
		val directory = directoryFor(uri.toString())
		if (reuse) directory.listFiles()?.firstOrNull { it.isFile && it.name != ".incoming" }?.let { return it }
		val displayName = resolver
			.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME), null, null, null)
			?.use { cursor ->
				if (cursor.moveToFirst()) cursor.getString(0).orEmpty() else ""
			}.orEmpty()
		val name =
			File(displayName).name.takeUnless { it.isBlank() || it == "." || it == ".." || it == ".incoming" } ?: "document.epub"
		val input = resolver.openInputStream(uri) ?: return null
		directory.mkdirs()
		val temporary = File(directory, ".incoming")
		try {
			input.use { source -> temporary.outputStream().use { source.copyTo(it) } }
			val file = File(directory, name)
			check(temporary.renameTo(file))
			directory.listFiles()?.filter { it != file }?.forEach(File::delete)
			return file
		} finally {
			temporary.delete()
		}
	}

	@Synchronized
	fun forget(uriString: String) {
		directoryFor(uriString).deleteRecursively()
	}

	/** An older release named per-opening directories with a UUID, so nothing could find them again to clean them up. */
	fun purgeLegacy() {
		try {
			cacheRoot.listFiles()?.forEach {
				if (it.isDirectory && UUID_DIR_REGEX.matches(it.name)) {
					it.deleteRecursively()
				}
			}
		} catch (_: Exception) {
		}
	}
}

// These platform document providers read device storage. Other providers can download on open.
internal fun Uri.isLocalDocument(): Boolean = scheme == ContentResolver.SCHEME_CONTENT &&
	when (authority) {
		"com.android.externalstorage.documents",
		"com.android.providers.downloads.documents",
		"com.android.providers.media.documents" -> true
		else -> false
	}

/** Hashed rather than escaped: a content URI can run past what a file name can carry. */
internal fun cacheKeyFor(uriString: String): String = MessageDigest
	.getInstance("SHA-256")
	.digest(uriString.toByteArray())
	.take(KEY_BYTES)
	.joinToString("") { "%02x".format(it) }
