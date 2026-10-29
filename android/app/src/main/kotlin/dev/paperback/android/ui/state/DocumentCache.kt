package dev.paperback.android.ui.state

import java.io.File
import java.security.MessageDigest

private const val DOCUMENT_DIR = "documents"
private const val KEY_BYTES = 16

private val UUID_DIR_REGEX = Regex("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")

/** Named after the URI rather than at random, so reopening a document reuses its directory instead of filling the cache with a copy per opening. */
class DocumentCache(private val cacheRoot: File) {
	fun directoryFor(uriString: String): File = File(cacheRoot, "$DOCUMENT_DIR/${cacheKeyFor(uriString)}")

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

/** Hashed rather than escaped: a content URI can run past what a file name can carry. */
internal fun cacheKeyFor(uriString: String): String = MessageDigest
	.getInstance("SHA-256")
	.digest(uriString.toByteArray())
	.take(KEY_BYTES)
	.joinToString("") { "%02x".format(it) }
