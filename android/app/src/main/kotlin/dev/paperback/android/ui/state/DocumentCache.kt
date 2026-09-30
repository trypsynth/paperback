package dev.paperback.android.ui.state

import java.io.File
import java.security.MessageDigest

private const val DOCUMENT_DIR = "documents"
private const val KEY_BYTES = 16

/** Directories left by an older release, which named them with a fresh UUID per opening. */
private val UUID_DIR_REGEX = Regex("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")

/**
 * Where a document opened from a content URI is unpacked to, so the parser has a real file to
 * read. One directory per document, named after the URI rather than at random, so reopening the
 * same document reuses its directory instead of filling the cache with a copy per opening.
 */
class DocumentCache(
	private val cacheRoot: File
) {
	fun directoryFor(uriString: String): File = File(cacheRoot, "$DOCUMENT_DIR/${cacheKeyFor(uriString)}")

	/**
	 * Deletes the per-opening directories an older release left behind. Those were named with a
	 * UUID, so nothing could ever find them again to clean them up, and a heavy reader could be
	 * carrying a copy of every document they had ever opened.
	 */
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

/**
 * A stable, filesystem-safe directory name for a document URI. Hashed rather than escaped: a
 * content URI can be long enough, and hold enough of anything, to run past what a file name can
 * carry on its own.
 */
internal fun cacheKeyFor(uriString: String): String =
	MessageDigest
		.getInstance("SHA-256")
		.digest(uriString.toByteArray())
		.take(KEY_BYTES)
		.joinToString("") { "%02x".format(it) }
